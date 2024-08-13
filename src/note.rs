#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Note {
    // MIDI note pitch (0 - 127)
    pub pitch: u8,
    // Note value in semiquavers (0 - )
    pub duration: u16,
    // Position of note head from score beginning, in semiquavers (0 - )
    pub position: u16,
    // MIDI channel to place the note in (0 - 15)
    // NB Channel 9 is reserved for percussion
    pub channel: u8,
}

impl Note {
    pub fn new(pitch: u8, duration: u16, position: u16, channel: u8) -> Self {
        Note { pitch, duration, position, channel }
    }
    pub fn checked_new(pitch: u8, duration: u16, position: u16, channel: u8) -> Option<Self> {
        Some(Note { pitch, duration, position, channel })
            .filter(|_| (0..128).contains(&pitch))
            .filter(|_| (0..16).contains(&channel))
    }
}

impl From<(u8, u16, u16, u8)> for Note {
    fn from(value: (u8, u16, u16, u8)) -> Self {
        Note {
            pitch: value.0,
            duration: value.1,
            position: value.2,
            channel: value.3,
        }
    }
}

impl From<(u8, u16, u16)> for Note {
    fn from(value: (u8, u16, u16)) -> Self {
        Note {
            pitch: value.0,
            duration: value.1,
            position: value.2,
            channel: 0,
        }
    }
}

impl From<Note> for (u8, u16, u16, u8) {
    fn from(value: Note) -> Self {
        (value.pitch, value.duration, value.position, value.channel)
    }
}

impl From<Note> for (u8, u16, u16) {
    fn from(value: Note) -> Self {
        (value.pitch, value.duration, value.position)
    }
}
