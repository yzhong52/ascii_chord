use chord::Chord;
use chord::FRETBOARD;
use std::cmp::max;

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum NameStyle {
    ShortName,
    FullName,
    BothNames,
}

pub fn row<'a>(chords: Vec<Chord<'a>>, name_style: NameStyle, finger_notation: char) -> String {
    let display_names: Vec<String> = chords
        .iter()
        .map(|chord| match name_style {
            NameStyle::ShortName => chord.short_name.to_owned(),
            NameStyle::FullName => chord.name.to_owned(),
            NameStyle::BothNames => chord.both_names(),
        })
        .collect();

    let max_display_name_width = display_names.iter().map(|name| name.len()).max().unwrap();

    let num_chords = chords.len();
    let board: Vec<&str> = FRETBOARD.split('\n').collect();
    let board_width = board[0].chars().count();

    // The 'padding' between chords horizontally
    // Minimum 4 spaces between chords;
    // Minimum 2 spaces between titles;
    let padding: usize = max(4, max_display_name_width as i32 - board_width as i32 + 2) as usize;

    // We need to make sure the last one has enough additional padding for the title
    let last_padding = max(
        0,
        display_names.last().unwrap().len() as i32 - board_width as i32,
    ) as usize;

    let width = (board_width + padding) * num_chords - padding + last_padding;

    // +1 for the label - name of chord
    let height = board.len() + 1;

    let mut buffer = vec![vec![' '; width]; height];

    // Print the names of the chords
    for (i, display_name) in display_names.iter().enumerate() {
        for (char_id, char) in display_name.chars().enumerate() {
            buffer[0][char_id + i * (board_width + padding)] = char;
        }
    }

    // Print the chord diagram
    for (i, chord) in chords.iter().enumerate() {
        let diagram: Vec<String> = chord
            .fretboard(finger_notation)
            .split('\n')
            .map(|line| line.to_owned())
            .collect();

        for (line_id, line) in diagram.iter().enumerate() {
            for (char_id, char) in line.chars().enumerate() {
                buffer[line_id + 1][char_id + i * (board_width + padding)] = char;
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
