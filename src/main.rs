use midi_practice::Score;

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
    let smf = score.to_midi();
    smf.save("sample.mid").expect("failed to save file");
}
