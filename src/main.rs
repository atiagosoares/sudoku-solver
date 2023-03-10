use std::str::FromStr;
use std::{fs, vec}; 
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use combinations::Combinations;


// The group is a collection of n cells that may contains n values
// For example: an empty or is a 9-sized group that contains the numbers 1 to 9
//              a cell with the number 4 is a 1-sized group that coitains the number 4
#[derive(Debug)]
struct Group{
    indexes: Vec<i16>,
    mask: i16
}

// The partition represents one way of dividing a board
// For example, the rows are one way to partition a board, the blocks are another one
struct Partition{
    groups: Vec<Group>,
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
                continue;
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
            if count_bits(&i) == 1i8{
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

    fn initialize_partitions(self: &mut Self) {
        // Creates the initial rows, columns and block partitions
        
        // Rows and columns
        let mut row_groups: Vec<Group> = vec![];
        let mut col_groups: Vec<Group> = vec![];
        let mut block_groups: Vec<Group> = vec![];
        for i in 0..9{
            let mut row_group_indexes: Vec<i16> = vec![];
            let mut col_group_indexes: Vec<i16> = vec![]; 
            let mut block_group_indexes: Vec<i16> = vec![];
            for j in 0..9{
                row_group_indexes.push(9*i + j); 
                col_group_indexes.push(i + 9*j);
                block_group_indexes.push(3*i + 18*(i/3) + 9*(j/3) + j%3);
            }
            row_groups.push(
                Group{indexes:row_group_indexes, mask: 0b111_111_111}
            );
            
            col_groups.push(
                Group{indexes:col_group_indexes, mask: 0b111_111_111}
            );

            block_groups.push(
                Group{indexes:block_group_indexes, mask: 0b111_111_111}
            );
        }
        self.partitions.push(Partition { groups: row_groups });
        self.partitions.push(Partition { groups: col_groups });
        self.partitions.push(Partition { groups: block_groups });
    }

    fn measure_entropy(self: &Self) -> i16{
        // Measures how many positive bits are still in the entire array
        // Game is solved when this value is 81

        let mut entropy = 0i16;
        for house in &self.values {
            entropy += count_bits(&house) as i16;
        };
        return entropy
    }
}



fn count_bits(number: &i16) -> i8{
    // Count non-zero bits in the last significant digits of a byte
    let mut count = 0i16;
    for house in 0..9{
        count += (number >> house) & 1i16; // Will add one if the bit at that house is 1
    };
    count as i8
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





fn main() {
    
    // Read the file contents
    let file_path = String::from_str("src/expert5.txt").expect("");
    let mut game = Game::load_file(file_path); 
    game.render();
    println!("");
    game.render_marks();

    // Define the partitions (rows, columns and squares) of the games as arrays of indexes
    game.initialize_partitions();

    for partition in &game.partitions {
        for group in &partition.groups {
            println!("{:?}", group.indexes);
        };
        print!("\n\n");
    }
    
    // Let's start solving...
    let now = Instant::now();
    let mut entropy = game.measure_entropy();
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
