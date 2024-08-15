use midi_practice::Score;
use midir::MidiOutput;
use std::env;

fn main() {
    let mut score = Score::from([
        (48, 4, 0),   // C3 crotchet (arp)
        (52, 4, 4),   // E3 crotchet (arp)
        (55, 4, 8),   // G3 crotchet (arp)
        (58, 4, 12),  // Bb3 crotchet (arp)
        (48, 16, 16), // C3 whole note (chord)
        (52, 16, 16), // E3 whole note (chord)
        (55, 16, 16), // G3 whole note (chord)
        (58, 16, 16), // Bb3 whole note (chord)
        (60, 16, 16), // C4 whole note (chord)
    ]);
    score.extend([
        (48, 16, 32, 9), // percussion (chord)
        (52, 16, 32, 9), // percussion (chord)
        (55, 16, 32, 9), // percussion (chord)
        (58, 16, 32, 9), // percussion (chord)
        (60, 16, 32, 9), // percussion (chord)
    ]);

    let dest = env::args().next().unwrap_or("sample.mid".to_owned());
    println!("[*] Saving score to {dest}...");
    let smf = score.to_midi();
    smf.save("sample.mid").expect("failed to save file");
    println!("[#] File saved.");

    println!("[*] Connecting to MIDI output...");
    let midi_out = MidiOutput::new("midi_out").expect("could not create MidiOutput");
    let ports = midi_out.ports();
    let port = ports.first().expect("no ports available");
    let port_name = midi_out.port_name(port).expect("port no longer available");
    let mut conn = midi_out
        .connect(port, "first")
        .expect("port no longer available");
    println!("[#] Connected to output port {port_name}.");
    println!("[*] Playing now! Listen...");
    score.play(&mut conn);
    println!("[#] Playback finished.");
    conn.close();
}
