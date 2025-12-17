const MAX_BATTERIES_ENABLED: usize = 12;

struct Battery {
    digit: u8,
    enabled: bool,
}

impl Battery {
    fn new(digit: u8) -> Self {
        Battery {
            digit,
            enabled: false,
        }
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn get_value(&self) -> u8 {
        if self.enabled { self.digit as u8 } else { 0 }
    }
}

struct Bank {
    batteries: Vec<Battery>,
}

impl Bank {
    fn new(input: &str) -> Self {
        let batteries = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| Battery::new(d as u8))
            .collect();
        Bank { batteries }
    }

    /// From a bank of batteries, find the maximum joltage
    fn max_joltage(&mut self) -> u64 {
        let mut enabled_count = 0;
        for i in (0..10).rev() {
            // starting from the highest possible digit, enable all batteries with that digit
            for battery in &mut self.batteries {
                if battery.digit as usize == i && enabled_count < MAX_BATTERIES_ENABLED {
                    enabled_count += 1;
                    battery.enable();
                }
            }
            println!("Enabled {} batteries of digit {}", enabled_count, i);
        }
        self.print_batteries();
        // calculate the total joltage
        let enabled_batteries_string = self
            .batteries
            .iter()
            .filter(|b| b.enabled)
            .map(|b| b.digit.to_string())
            .collect::<String>();
        let joltage = enabled_batteries_string.parse::<u64>().unwrap();
        joltage
    }

    fn print_batteries(&self) {
        println!("Batteries status:");
        // first print a line with the digits
        for battery in &self.batteries {
            print!("{}", battery.digit);
        }
        println!();
        // then print a line with the enabled status
        for battery in &self.batteries {
            if battery.enabled {
                print!("^");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let mut max_joltage = 0;
    for line in input.lines() {
        let mut bank = Bank::new(line);
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

        let mut max_joltage: u64 = 0;
        for line in example_input.lines() {
            let mut bank = Bank::new(line);
            let joltage = bank.max_joltage();
            println!("Joltage: {}", joltage);
            max_joltage += joltage;
            println!("Accumulated joltage: {}", max_joltage);
        }
        println!("Total max joltage: {}", max_joltage);
        assert_eq!(max_joltage, 3121910778619);
    }
}
