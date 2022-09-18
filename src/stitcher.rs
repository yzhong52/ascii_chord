use chords::Chord;
use chords::FRETBOARD;

pub fn row<'a>(chords: Vec<Chord<'a>>) -> String {
    // The 'PADDING' between chords horizontally
    const PADDING: usize = 4;

    let num_chords = chords.len();
    let board: Vec<&str> = FRETBOARD.split('\n').collect();
    let board_width = board[0].chars().count();
    let width = (board_width + PADDING) * num_chords;

    // +1 for the label - name of chord
    let height = board.len() + 1;

    let mut buffer = vec![vec![' '; width]; height];

    // Print the names of the chords
    for (i, chord) in chords.iter().enumerate() {
        let full_name = format!("{} ({})", chord.name, chord.short_name);
        for (char_id, char) in full_name.chars().enumerate() {
            buffer[0][char_id + i * (board_width + PADDING)] = char;
        }
    }

    // Print the chord diagram
    for (i, chord) in chords.iter().enumerate() {
        let diagram: Vec<String> = chord
            .fretboard()
            .split('\n')
            .map(|line| line.to_owned())
            .collect();

        for (line_id, line) in diagram.iter().enumerate() {
            for (char_id, char) in line.chars().enumerate() {
                buffer[line_id + 1][char_id + i * (board_width + PADDING)] = char;
            }
        }
    }

    let lines: Vec<String> = buffer
        .into_iter()
        .map(|line| line.into_iter().collect())
        .collect();
    let result: String = lines.join("\n");
    result
}
