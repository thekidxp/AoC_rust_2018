use std::{fs::File, io::prelude::*};

fn main() {
    const FILE_PATH: &str = "./input_day_1";
    let mut file = File::open(FILE_PATH).expect("Failed to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file");

    let numbers: Vec<&str> = contents.split('\n').collect();

    let mut total = 0;

    let mut frequencies: Vec<i64> = Vec::new();

    for number in numbers {
        match number.parse() {
            Ok(num) => {
                frequencies.push(num);
                total += num;
            }
            _ => {}
        };
    }
    println!("total = {}", total);

    let second_number = second_part(frequencies);
    println!("total = {}", second_number);
}

fn second_part(frequencies: Vec<i64>) -> i64 {
    let mut running_total: Vec<i64> = Vec::new();
    let mut current_total: i64 = 0;
    let mut match_found = false;
    let mut count = 0;

    while !match_found {
        let frequency: i64 = *frequencies.get(count).unwrap();
        match running_total.last() {
            None => current_total = frequency,
            Some(number) => current_total = number + frequency,
        }

        if running_total.contains(&current_total) {
            match_found = true;
        } else {
            running_total.push(current_total);
        }

        if count == frequencies.len() - 1 {
            count = 0;
        } else {
            count += 1;
        }
    }

    current_total
}

#[cfg(test)]
mod tests {

    #[test]
    fn second_test() {}
}
