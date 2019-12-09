pub fn calculate_fuel_requirement(mass: i64) -> i64 {
    let result = mass / 3 - 2;
    if result > 0 { result } else { 0 }    
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