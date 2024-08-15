use crate::Note;
use midir::MidiOutputConnection;
use midly::{
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};
use std::{iter, thread, time::Duration};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Score(Vec<Note>);

impl Score {
    pub fn new() -> Self {
        Score(Vec::new())
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Score(Vec::with_capacity(capacity))
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn as_slice(&self) -> &[Note] {
        self.0.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [Note] {
        self.0.as_mut_slice()
    }
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity)
    }
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, note: impl Into<Note>) -> &mut Self {
        let note = note.into();
        self.0.push(note);
        self
    }
    pub fn add_note(&mut self, pitch: u8, duration: u16, position: u16, channel: u8) -> &mut Self {
        self.add(Note {
            pitch,
            duration,
            position,
            channel,
        })
    }
    pub fn extend(&mut self, notes: impl IntoIterator<Item = impl Into<Note>>) -> &mut Self {
        for note in notes {
            self.add(note);
        }
        self
    }
    pub fn remove_note(
        &mut self,
        pitch: u8,
        duration: u16,
        position: u16,
        channel: u8,
    ) -> &mut Self {
        self.remove(Note {
            pitch,
            duration,
            position,
            channel,
        })
    }
    pub fn remove(&mut self, note: impl Into<Note>) -> &mut Self {
        let note = note.into();
        if let Some(idx) = self.0.iter().position(|&n| n == note) {
            self.0.swap_remove(idx);
        }
        self
    }
    pub fn purge(&mut self, notes: impl IntoIterator<Item = impl Into<Note>>) -> &mut Self {
        for note in notes {
            self.remove(note);
        }
        self
    }

    const TPB: u16 = 24;
    const BPM: u16 = 120;
    pub fn to_track(&self) -> Track {
        let mut event_timings = self
            .0
            .iter()
            .flat_map(|n| {
                [
                    (
                        n.position,
                        TrackEventKind::Midi {
                            channel: n.channel.into(),
                            message: MidiMessage::NoteOn {
                                key: n.pitch.into(),
                                vel: 127.into(),
                            },
                        },
                    ),
                    (
                        n.position + n.duration,
                        TrackEventKind::Midi {
                            channel: n.channel.into(),
                            message: MidiMessage::NoteOff {
                                key: n.pitch.into(),
                                vel: 0.into(),
                            },
                        },
                    ),
                ]
            })
            .collect::<Vec<_>>();
        event_timings.sort_by_key(|e| e.0);
        let event_deltas = if let Some(first) = event_timings.first() {
            iter::once(*first)
                .chain(event_timings.windows(2).map(|w| (w[1].0 - w[0].0, w[1].1)))
                .collect()
        } else {
            vec![]
        };
        let track = event_deltas
            .iter()
            .map(|(d, k)| TrackEvent {
                delta: (u32::from(*d) * u32::from(Self::TPB) / 4).into(),
                kind: *k,
            })
            .chain(iter::once(TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            }))
            .collect();
        track
    }
    pub fn to_midi(&self) -> Smf {
        let header = Header {
            format: Format::SingleTrack,
            timing: Timing::Metrical(Self::TPB.into()),
        };
        let tracks = vec![self.to_track()];
        Smf { header, tracks }
    }
    pub fn play(&self, conn: &mut MidiOutputConnection) {
        const NOTE_ON: u8 = 0x90;
        const NOTE_OFF: u8 = 0x80;
        let track = self.to_track();
        for event in track {
            thread::sleep(Duration::from_millis(
                u64::from(u32::from(event.delta)) * 60000
                    / u64::from(Self::TPB)
                    / u64::from(Self::BPM),
            ));
            if let TrackEventKind::Midi { channel, message } = event.kind {
                let payload: &[u8] = match message {
                    MidiMessage::NoteOn { key, vel } => {
                        &[NOTE_ON | u8::from(channel), key.into(), vel.into()]
                    }
                    MidiMessage::NoteOff { key, vel } => {
                        &[NOTE_OFF | u8::from(channel), key.into(), vel.into()]
                    }
                    _ => &[],
                };
                conn.send(payload).unwrap();
            }
        }
    }
}

impl<I, T> From<I> for Score
where
    I: IntoIterator<Item = T>,
    T: Into<Note>,
{
    fn from(value: I) -> Self {
        Score(value.into_iter().map(Into::into).collect())
    }
}
