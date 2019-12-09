mod day;
use std::fs::File;


fn main() { 
    let vec = day::read_integers_into_vector(File::open("./day1_input.txt").unwrap());
    println!("{:#?}", vec);
    
}