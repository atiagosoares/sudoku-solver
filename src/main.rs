use std::fs;

fn main() {
    // Read the file contents
    let file_content = fs::read_to_string("src/test-sudoku.txt").expect("Reading...");
    println!("{file_content}");
    
    // Parse the file content into a sudoku struck
    /* The file will be encoded as follows:
    * Will be an array<i16> with 81 elements
    * Each element will represent one house of the sudoku game
    * Each bit will represent what number that house may be. Ex:
    * is 1 -> 000_000_001 ; is 4 > 000_001_000
    * is empty, may be any number: 111_111_111
    * may be 3 or 9 > 100_000_100
    * 
    * The game will be complete when:
    * All the houses have only one bit == 1
    */

    // Create the game array
    let mut game: [i16; 81] = [0b111_111_111; 81]; // Initially, assume all values are possible
    println!("Initialized game:\n{game:?}");

    // Loop through the file
    let mut pos = 0;
    let mut whitespace_counter = 0;
    for _char in file_content.chars() {
        // Three whitespaces in a row should move the pos
        if _char == ' ' || _char == '\n' {
            whitespace_counter += 1;
            if whitespace_counter == 3{
                pos += 1;
                whitespace_counter = 0;
            };
            continue;
        };
        whitespace_counter = 0;

        // If it is an aesthetic character, ignore
        if _char == '|' || _char == '-'{
            continue;
        };

        // If it's a numeric character, encode it to binary
        println!("Found number {} - encoding as {:0>9b}", _char, 1 << _char.to_digit(10).unwrap() - 1);
        game[pos] = 1; // Set to one
        game[pos] <<= _char.to_digit(10).unwrap() -1; // Shift to the left the correspondign number of bits
        pos += 1;
    };
    
    println!("After loading the file:\n{game:?}");
}
