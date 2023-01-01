use std::fs; 
use std::collections::HashMap;

fn count_bits(number: i16) -> i8{
    // Count non-zero bits in the last significant digits of a byte
    let mut count = 0i16;
    for house in 0..9{
        count += (number >> house) & 1i16; // Will add one if the bit at that house is 1
    };
    count as i8
}

fn evolve(mut game: [i16; 81], group: [usize; 9]) -> [i16; 81]{

    // Count the values of each value in the group
    let mut value_counts: HashMap<i16, i8> = HashMap::new();
    for idx in group{
        value_counts.entry(game[idx])
            .and_modify(|counter| *counter += 1i8)
            .or_insert(1i8);
    };
    
    // Evaluate if the value count matches it's bitcount
    let mut x: Vec<i16> = vec![];
    for (k, v) in value_counts{
        if count_bits(k) == v{
            x.push(k);
        }
    };
    
    // Loop through the group one more time, removing x
    for idx in group{
        for i in &x{
            if game[idx] != *i{
                game[idx] &= !i;
            }
        }
    };
    return game
}

fn render(game: [i16; 81]){
    let mut counter = 0;
    let mut l_counter = 0;
    print!(" ");
    for i in game {
        counter += 1;
        if count_bits(i) == 1i8{
            for n in 1i16..10 {
               if 1i16 << (n - 1) == i{
                    print!("{n} ");
                }; 
            };
        }else {
            print!(". ");
        };

        if counter == 3 || counter == 6 {
            print!("| ");
        };

        if counter == 9 {
            print!("\n ");
            counter = 0;
            l_counter += 1;

            if l_counter == 3 || l_counter == 6 {
                print!("------|-------|-------\n ");
            }
        }
    }
}

fn measure_entropy(game: [i16; 81]) -> i16{
    // Measures how many positive bits are still in the entire array
    // Game is solved when this value is 81

    let mut entropy = 0i16;
    for house in game {
        entropy += count_bits(house) as i16;
    };
    return entropy
}


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

    // Loop through the file
    let mut pos = 0;
    for _char in file_content.chars() {
        // If it is an aesthetic character, ignore
        if _char == '|' || _char == '-' || _char == ' ' || _char == '\n'{
            continue;
        };
        // If it's the null charr (.), move the pos;
        if _char == '.' {
            pos += 1;
            continue;
        }
        // If it's a numeric character, encode it to binary
        game[pos] = 1; // Set to one
        game[pos] <<= _char.to_digit(10).unwrap() -1; // Shift to the left the correspondign number of bits
        // println!("It has {} bits.\n", count_bits(game[pos]));
        pos += 1;
    };
    
    println!("After loading the file:\n");
    render(game);

    // Define the partitions (rows, columns and squares) of the games as arrays of indexes
    let partitions: [[[usize; 9]; 9]; 2] = [
        [ // Rows
            [0, 1, 2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24, 25, 26],
            [27, 28, 29, 30, 31, 32, 33, 34, 35],
            [36, 37, 38, 39, 40, 41, 42, 43, 44],
            [45, 46, 47, 48, 49, 50, 51, 52, 53],
            [54, 55, 56, 57, 58, 59, 60, 61, 62],
            [63, 64, 65, 66, 67, 68, 69, 70, 71],
            [72, 73, 74, 75, 76, 77, 78, 79, 80]
        ],
        [ // Columns
            [0, 9, 18, 27, 36, 45, 54, 63, 72],
            [1, 10, 19, 28, 37, 46, 55, 64, 73],
            [2, 11, 20, 29, 38, 47, 56, 65, 74],
            [3, 12, 21, 30, 39, 48, 57, 66, 75],
            [4, 13, 22, 31, 40, 49, 58, 67, 76],
            [5, 14, 23, 32, 41, 50, 59, 68, 77],
            [6, 15, 24, 33, 42, 51, 60, 69, 78],
            [7, 16, 25, 34, 43, 52, 61, 70, 79],
            [8, 17, 26, 35, 44, 53, 62, 71, 80]
        ]
    ];
    
    // Let's start solving...
    let mut entropy = measure_entropy(game);
    let mut entropy_buffer: i16;
    println!("Initial entropy: {entropy}");

    let mut step_counter = 0;
    while entropy > 81i16 { // While it is still not solved...

        // Execute a calculation step
        step_counter += 1;
        for part in partitions {
            for slice in part{
                game = evolve(game, slice);
            };
        };
        // Measure the new entropy
        entropy_buffer = measure_entropy(game);
        println!("Entropy after step {step_counter}: {entropy_buffer}");
        // Check if algorythm isn't stuck
        if entropy_buffer == entropy {
            println!("No reduction in entropy. We're stuck.");
            break;
        };
        entropy = entropy_buffer;
    }

    println!("Final state:");
    render(game);
}
