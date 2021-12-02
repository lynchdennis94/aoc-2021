use std::env;
use std::io::Read;

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
