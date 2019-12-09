use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn calculate_fuel_requirement(mass: i64) -> i64 {
    let result = mass / 3 - 2;
    if result > 0 { result } else { 0 }    
}   



pub fn read_integers_into_vector(io: File) -> Vec<i64> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line.unwrap().parse::<i64>().unwrap());
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn calculate_fuel () {
        assert_eq!(0, calculate_fuel_requirement(1));
        assert_eq!(2, calculate_fuel_requirement(12));
        assert_eq!(2, calculate_fuel_requirement(14));
        assert_eq!(654, calculate_fuel_requirement(1969));
        assert_eq!(33583, calculate_fuel_requirement(100756));
    }
}