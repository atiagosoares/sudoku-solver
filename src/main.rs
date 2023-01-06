use std::str::FromStr;
use std::{fs, vec}; 
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use combinations::Combinations;


// The group is a collection of n cells that may contains n values
// For example: an empty or is a 9-sized group that contains the numbers 1 to 9
//              a cell with the number 4 is a 1-sized group that coitains the number 4
struct Group{
    indexes: Vec<usize>,
    mask: i16,
    game: Game
}

// The partition represents one way of dividing a board
// For example, the rows are one way to partition a board, the blocks are another one
struct Partition{
    groups: Vec<Group>,
    game: Game
}

// The game object.
// Contais current state of cells and existing partitions
struct Game{
    values: Vec<i16>,
    partitions: Vec<Partition>
}

impl Game{
    fn initialize() -> Self{
        Game{values: vec![0b111_111_111; 81], partitions: vec![]}
    }

    fn load_file(file_path: String) -> Self{

        let file_content = fs::read_to_string(file_path).expect("Reading...");

        let mut game_values: Vec<i16> = vec![];
        // Loop through the file
        for _char in file_content.chars() {
            // If it is an aesthetic character, ignore
            if _char == '|' || _char == '-' || _char == ' ' || _char == '\n'{
                continue;
            };
            // If it's the null charr (.), move the pos;
            if _char == '.' {
                game_values.push(0b111_111_111);
            }
            // If it's a numeric character, encode it to binary
            game_values.push(1 << (_char.to_digit(10).unwrap() -1));
        };
        return Game{values: game_values, partitions: vec![]};
    }

    fn render(self: &Self){
        let mut counter = 0;
        let mut l_counter = 0;
        print!(" ");
        for &i in &self.values {
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

    fn render_marks(self: &Self){
        let mut cell_counter = 0;
        let mut line_counter = 0;
        for &cell in &self.values{
            cell_counter += 1;
            let mut bit_marks: String = "".to_owned();
            for number in 0..9 {
                let bit = 1 << number;
                if cell & bit > 0 {
                    bit_marks.push_str(&(number + 1).to_string());
                };
            };
            print!(" {bit_marks:^9} ");
            if cell_counter == 3 || cell_counter == 6 {print!("|")};
            if cell_counter == 9 {
                print!("\n ");
                cell_counter = 0;
                line_counter += 1;
                if line_counter == 3 || line_counter == 6{
                    print!("{:->99}\n ", ""); // Too hacky, maybe?
                }
            };
        }
    }
}


fn count_bits(number: i16) -> i8{
    // Count non-zero bits in the last significant digits of a byte
    let mut count = 0i16;
    for house in 0..9{
        count += (number >> house) & 1i16; // Will add one if the bit at that house is 1
    };
    count as i8
}

fn evolve(mut game: [i16; 81], group: [usize; 9]) -> [i16; 81]{

    // STRATEGY 1
    // Check if there is a number of n houses that can only be the same n values
    // -> The remaining houses can't any of those values

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

    // STRATEGY 2
    // Check for hidden singles and hidden pairs
    // Step 1: count how many times each bit appears
    let mut bit_counts: HashMap<i16, Vec<i16>> = HashMap::new();
    for b in 0i16..10{
        let bit = 1i16 << b;
        let mut count = 0i16;
        for cell in group{
            count += game[cell] & bit;
        };
        count /= bit;
        bit_counts.entry(count)
            .and_modify(|v| v.push(bit))
            .or_insert(vec![bit]); 
    };

    // Step 2:
    // bits with count = 1 are hidden singles
    if bit_counts.contains_key(&1i16) {
        for hidden_single in &bit_counts[&1i16] {
            for cell in group{
                if game[cell] & hidden_single == *hidden_single{
                    game[cell] = *hidden_single;
                };
            };
        };
    };
    
    // Step 3:
    // Bits with count = 2 are candidates for hidden pairs
    if bit_counts.contains_key(&2i16){
        for a in &bit_counts[&2i16]{
            for b in &bit_counts[&2i16]{

                if a == b {
                    continue;
                };
                let mask = a | b;
                // Check if the count of cells that include one of these elements is equal to 2
                let mut k = 0i16;
                for cell in group {
                    if game[cell] & mask == mask {
                        k += 1;
                    }; // spaghetti
                };
                    
                if k == 2i16 {
                    for cell in group {
                        if game[cell] & mask == mask {
                            game[cell] = mask;
                        };
                    };
                };
            };
        };
    };
    return game

}

fn proliferate_from_intersection(mut game: [i16; 81], group_a: [usize; 9], group_b: [usize; 9]) -> [i16; 81] {

    // Determine if a set of values must be in an intersection between two groups
    // most notably between the intersection between a square and a row or column
    
    // Find the intersection between the groups
    let ha = HashSet::from(group_a);
    let hb = HashSet::from(group_b);
    let a_diff = ha.difference(&hb);
    let b_diff = hb.difference(&ha);
    let diff = ha.symmetric_difference(&hb);

    // Determine which values must be in the intersection according to each group
    let mut not_in_a_diff = 0i16;
    for house in a_diff{
       not_in_a_diff |= game[*house]; 
    };

    let mut not_in_b_diff = 0i16;
    for house in b_diff {
        not_in_b_diff |= game[*house];
    };
    
    // Proliferate those restriction across the groups
    let in_intersec = not_in_b_diff & not_in_a_diff;
    for house in diff {
        game[*house] &= in_intersec;
    };
    return game
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
    let file_path = String::from_str("expoert5.txt").expect("");
    let game = Game::load_file(file_path); 
    game.render();
    println!("");
    game.render_marks();

    // Define the partitions (rows, columns and squares) of the games as arrays of indexes
    
    // // Let's start solving...
    // let now = Instant::now();
    // let mut entropy = measure_entropy(game);
    // let mut entropy_buffer: i16;
    // println!("Initial entropy: {entropy}");

    // let mut step_counter = 0;
    // while entropy > 81i16 { // While it is still not solved...

    //     // Execute a calculation step
    //     step_counter += 1;
    //     for part in partitions {
    //         for slice in part{
    //             game = evolve(game, slice);
    //         };
    //     };

    //     // Execute the intersection thingy...
    //     for square in partitions[2] {
    //         for row in partitions[0] {
    //             game = proliferate_from_intersection(game, square, row);
    //         };
    //         for column in partitions[1]{
    //             game = proliferate_from_intersection(game, square, column);
    //         };
    //     };

    //     // Measure the new entropy
    //     entropy_buffer = measure_entropy(game);
    //     println!("Entropy after step {step_counter}: {entropy_buffer}");
    //     // Check if algorythm isn't stuck
    //     if entropy_buffer == entropy {
    //         println!("No reduction in entropy. We're stuck.");
    //         break;
    //     };
    //     entropy = entropy_buffer;
    // }

    // println!("Final state:");
    // render(game);
    // print!("\n\n ");
    // render_marks(game);
    // println!("Elapsed {}us", now.elapsed().as_micros());
}
