use std::{
    fs::File, 
    io::prelude::*, 
    collections::HashSet,
};

fn main() {
    const FILE_PATH: &str = "./input_day_1";
    let mut file = File::open(FILE_PATH).expect("Failed to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file");

    let numbers: Vec<isize> = contents.lines().map(|l| l.parse().unwrap()).collect();

    let first_number = first_part(&numbers);
    
    println!("total = {}", first_number);

    let second_number = second_part(&numbers);
    println!("total = {}", second_number);
}

fn first_part(input: &[isize]) -> isize {
    input.iter().sum()
}

fn second_part(input: &[isize]) -> isize {
    let mut freq_repeated = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(freq_repeated);

    for num in input.iter().cycle() {
        freq_repeated += num;
        if frequencies.contains(&freq_repeated) {
            return freq_repeated;
        }

        frequencies.insert(freq_repeated);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_test() {
        assert_eq!(second_part(&vec![1, -1]), 0);
        assert_eq!(second_part(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(second_part(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(second_part(&vec![7, 7, -2, -7, -4]), 14);
    }
}
