use std::env;
use std::io::Read;
use std::str::Split;

fn main() {
    // Accept args to get solutions
    let args: Vec<String> = env::args().collect();

    // First arg is the target, second should be the solution
    if args.len() != 2 {
        println!("Please define a solution to run");
    } else {
        // Call the correct solution function
        let solution = args[1].to_uppercase();

        match solution.as_str() {
            "1A" => solution_1a(),
            "1B" => solution_1b(),
            "2A" => solution_2a(),
            "2B" => solution_2b(),
            "3A" => solution_3a(),
            "3B" => solution_3b(),
            _ => println!("NOT SUPPORTED")
        };
    }
}

// Day 1 - part 1
fn solution_1a() {
    println!("Solving 1A");
    let file_location = "problem_1_input.txt";
    let problem_input_string= get_problem_input(file_location);

    let mut previous_value = -1;
    let mut increasing_measurement_counter = 0;
    for entry in problem_input_string.trim().split('\n') {
        let current_value = entry.parse::<i32>().unwrap();
        if previous_value != -1 {
            if current_value > previous_value {
                increasing_measurement_counter += 1;
            }
        }
        previous_value = current_value;
    }
    println!("Total number of increasing measurements: {}", increasing_measurement_counter);
}

// Day 1 - part 2
fn solution_1b() {
    println!("Solving 1B");
    //let file_location = "problem_1b_sample.txt";
    let file_location = "problem_1_input.txt";
    let problem_input_string = get_problem_input(file_location);

    let mut increasing_measurement_counter = 0;
    let split_input = problem_input_string.trim().split('\n').collect::<Vec<&str>>();

    if split_input.len() > 3 {
        for index in 3..(split_input.len()) {
            let fixed_index = index;
            let a = split_input.get(fixed_index - 3).expect("Missing A").parse::<i32>().unwrap();
            let b = split_input.get(fixed_index - 2).expect("Missing B").parse::<i32>().unwrap();
            let c = split_input.get(fixed_index - 1).expect("Missing C").parse::<i32>().unwrap();
            let d = split_input.get(fixed_index).expect("Missing D").parse::<i32>().unwrap();
            if (b+c+d) > (a+b+c) {
                increasing_measurement_counter += 1;
            }
        }
    } // Otherwise, not enough for a single window, answer would be 0

    println!("Total number of increasing windows: {}", increasing_measurement_counter);
}

// Day 2 - part 1
fn solution_2a() {
    println!("Solving 2A");

    //let file_location = "problem_2_sample.txt";
    let file_location = "problem_2_input.txt";
    let problem_input_string = get_problem_input(file_location);
    let split_input = problem_input_string.trim().split('\n').collect::<Vec<&str>>();

    let mut horizontal_position = 0;
    let mut vertical_position = 0;

    for command_str in split_input {
        let command_vec = command_str.split(' ').collect::<Vec<&str>>();
        let command = command_vec.get(0).expect("Couldn't parse command").to_string();
        let distance = command_vec.get(1).expect("Couldn't parse distance").parse::<i32>().unwrap();

        match command.as_str() {
            "forward" => horizontal_position += distance,
            "down" => vertical_position += distance,
            "up" => vertical_position -= distance,
            _ => ()
        };

        println!("{} {}", command, distance);
    }

    println!("Combined positions: {}", horizontal_position*vertical_position);
}

// Day 2 - part 2
fn solution_2b() {
    println!("Solving 2B");

    //let file_location = "problem_2_sample.txt";
    let file_location = "problem_2_input.txt";
    let problem_input_string = get_problem_input(file_location);

    let split_input = problem_input_string.trim().split('\n').collect::<Vec<&str>>();

    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    let mut aim = 0;

    for command_str in split_input {
        let command_vec = command_str.split(' ').collect::<Vec<&str>>();
        let command = command_vec.get(0).expect("Couldn't parse command").to_string();
        let distance = command_vec.get(1).expect("Couldn't parse distance").parse::<i32>().unwrap();

        match command.as_str() {
            "forward" => {
                horizontal_position += distance;
                vertical_position += distance * aim;
            },
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => ()
        };
    }

    println!("Combined positions: {}", horizontal_position*vertical_position);
}

// Day 3 - part 1
fn solution_3a() {
    println!("Solving 3A");
    //let file_location = "problem_3_sample.txt";
    let file_location = "problem_3_input.txt";
    let problem_input_string = get_problem_input(file_location);

    let problem_input_split = problem_input_string.trim().split('\n');
    let problem_input_vec = problem_input_split.collect::<Vec<&str>>();
    let array_length = problem_input_vec[0].len();
    let (ones_array, zeros_array) = get_most_common_bit_for_positions(problem_input_vec, array_length);

    let mut answer_index = 0;
    let mut binary_gamma_rate = "".to_owned(); // most common bit
    let mut binary_epsilon_rate = "".to_owned(); // least common bit

    while answer_index < array_length {
        if ones_array[answer_index] >= zeros_array[answer_index] {
            binary_gamma_rate.push_str("1");
            binary_epsilon_rate.push_str("0");
        } else {
            binary_gamma_rate.push_str("0");
            binary_epsilon_rate.push_str("1");
        }
        answer_index += 1;
    }

    let gamma_rate = isize::from_str_radix(binary_gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(binary_epsilon_rate.as_str(), 2).unwrap();

    println!("Power level: {}", gamma_rate*epsilon_rate);
}

// Day 3 - part 2
fn solution_3b() {
    println!("Solving 3B");
    let file_location = "problem_3_sample.txt";
    //let file_location = "problem_3_input.txt";
    let problem_input_string = get_problem_input(file_location);

    let problem_input_split = problem_input_string.trim().split('\n');
    let problem_input_vec = problem_input_split.collect::<Vec<&str>>();
    let mut oxygen_problem_set = problem_input_vec.clone();
    let mut carbon_problem_set = problem_input_vec.clone();
    let array_length = problem_input_vec[0].len();
    let (ones_array, zeros_array) = get_most_common_bit_for_positions(problem_input_vec, array_length);

    println!("Ones array: {:?}", ones_array);
    println!("Zeroes array: {:?}", zeros_array);

    // Oxygen - filter out numbers that don't adhere to most common bits
    for filter_index in 0..array_length {
        let mut working_problem_set: Vec<&str> = Vec::new();
        let most_common_bit = if ones_array[filter_index] >= zeros_array[filter_index] {'1'} else {'0'};
        //println!("At filter index {}, the mcb is {}", filter_index, most_common_bit);
        for entry in oxygen_problem_set {
            if entry.chars().nth(filter_index).unwrap() == most_common_bit {
                // We've passed this filter, so continue
                working_problem_set.push(entry);
            }
        }
        oxygen_problem_set = working_problem_set.clone();
        if oxygen_problem_set.len() == 1 {
            break;
        }
    }

    // Carbon - filter out numbers that don't adhere to least common bits
    for filter_index in 0..array_length {
        let mut working_problem_set: Vec<&str> = Vec::new();
        let least_common_bit = if ones_array[filter_index] >= zeros_array[filter_index] {'0'} else {'1'};
        //println!("At filter index {}, the mcb is {}", filter_index, most_common_bit);
        for entry in carbon_problem_set {
            if entry.chars().nth(filter_index).unwrap() == least_common_bit {
                // We've passed this filter, so continue
                working_problem_set.push(entry);
            }
        }
        carbon_problem_set = working_problem_set.clone();
        if carbon_problem_set.len() == 1 {
            break;
        }
    }

    let oxygen_rate = isize::from_str_radix(oxygen_problem_set.get(0).unwrap(), 2).unwrap();
    let carbon_rate = isize::from_str_radix(carbon_problem_set.get(0).unwrap(), 2).unwrap();

    println!("Live support rating: {}", oxygen_rate*carbon_rate);
}

// Helper function for day 3 = determine most common bit in each position
fn get_most_common_bit_for_positions(problem_input_vec: Vec<&str>, array_length: usize) -> (Vec<i32>, Vec<i32>) {

    let mut ones_array = vec![0; array_length];
    let mut zeros_array = vec![0; array_length];

    // Calculate the total number of ones/zeros in each bit position for everything in input
    for entry in problem_input_vec {
        let mut entry_index = 0;
        while entry_index < array_length {
            if entry.chars().nth(entry_index).unwrap() == '1' {
                ones_array[entry_index] += 1;
            } else {
                zeros_array[entry_index] += 1;
            }
            entry_index += 1;
        }
    }

    return (ones_array, zeros_array);
}

// Helper function to get the problem input
fn get_problem_input(file_location: &str) -> String {
    // Set up relative file path
    let mut relative_file_path = "src/problem_inputs/".to_owned();
    relative_file_path.push_str(file_location);

    // Get file object
    let mut file = std::fs::File::open(relative_file_path).unwrap();

    // Read out contents and return
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Return the contents
    return contents;
}
