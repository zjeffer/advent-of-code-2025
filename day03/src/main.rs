struct Bank {
    batteries: Vec<char>,
}

impl Bank {
    fn new(input: &str) -> Self {
        let batteries = input.chars().collect();
        Bank { batteries }
    }

    /// From a bank of batteries, find the maximum joltage
    fn max_joltage(&self) -> u32 {
        let mut current_max_number = 0;
        for i in 0..self.batteries.len() - 1 {
            for j in i + 1..self.batteries.len() {
                let number = self.batteries[i].to_digit(10).unwrap() * 10
                    + self.batteries[j].to_digit(10).unwrap();
                if number > current_max_number {
                    current_max_number = number;
                }
            }
        }
        current_max_number
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let mut max_joltage = 0;
    for line in input.lines() {
        let bank = Bank::new(line);
        let joltage = bank.max_joltage();
        max_joltage += joltage;
    }
    println!("Total max joltage: {}", max_joltage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example_input = include_str!("../data/example.txt");

        let mut max_joltage = 0;
        for line in example_input.lines() {
            let bank = Bank::new(line);
            let joltage = bank.max_joltage();
            max_joltage += joltage;
        }
        assert_eq!(max_joltage, 357);
    }
}
