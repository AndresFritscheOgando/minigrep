use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("Query: {}, Filename: {}", query, filename);

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text: \n{}", contents);
}
