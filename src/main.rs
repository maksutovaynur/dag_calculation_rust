use std::env;
use std::fs;

mod dg;
mod parsing;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];
    let output_file_name = &args[2];
    let thread_count: i32 = args[3].parse().expect("Not a number!");
    println!("Args: {:?}", args);
    println!(
        "Input file: {}; output file: {}; thread_count: {}",
        input_file_name, output_file_name, thread_count);
    process_file(input_file_name, output_file_name, thread_count);
}


fn process_file(input_file_name: & String, output_file_name: & String, thread_count: i32) {

    let mut graph = dg::Graph::new();
    let data: String = fs::read_to_string(input_file_name).expect("Can't read file");
    for line in data.split("\n") {
        graph.add_cell(line);
    }
    println!("Graph size: {}", graph.size());
}
