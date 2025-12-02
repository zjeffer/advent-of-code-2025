const DIAL_START: i32 = 50;
const DIAL_MIN: i32 = 0;
const DIAL_MAX: i32 = 99;

struct Dial {
    position: i32,
    counter: i32,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {c}"),
        }
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            position: DIAL_START,
            counter: 0,
        }
    }
}

impl Dial {
    fn step(&mut self, direction: Direction, steps: i32) {
        println!(
            "Stepping {:?} by {} from {}",
            direction, steps, self.position
        );
		let steps = steps % (DIAL_MAX + 1);
        match direction {
            Direction::Left => {
                if self.position - steps < DIAL_MIN {
                    self.position = DIAL_MAX - (DIAL_MIN - (self.position - steps) - 1);
                } else {
                    self.position -= steps;
                }
            }
            Direction::Right => {
                if self.position + steps > DIAL_MAX {
                    self.position = DIAL_MIN + ((self.position + steps) - DIAL_MAX - 1);
                } else {
                    self.position += steps;
                }
            }
        }
        if self.position == 0 {
            self.counter += 1;
        }
    }

    fn parse_line(line: &str) -> (Direction, i32) {
        let (direction, steps_str) = line.split_at(1);
        let steps: i32 = steps_str.parse().unwrap();
        (Direction::from(direction.chars().next().unwrap()), steps)
    }
}

fn main() {
    let mut dial = Dial::default();

    let input: &str = include_str!("../data/input.txt");
    for line in input.lines() {
        let (direction, steps_str) = Dial::parse_line(line);
        dial.step(direction, steps_str);
    }

    println!("Final position: {}", dial.position);
    println!("Counter: {}", dial.counter);
}

mod tests {
    use super::*;

    #[test]
    fn test_example() {
        const EXAMPLE_INPUT: &str = include_str!("../data/example.txt");
        let mut dial = Dial::default();
        for line in EXAMPLE_INPUT.lines() {
            let (direction, steps_str) = Dial::parse_line(line);
            dial.step(direction, steps_str);
        }
        assert_eq!(dial.position, 32);
        assert_eq!(dial.counter, 3);

        println!("Test final position: {}", dial.position);
        println!("Test counter: {}", dial.counter);
    }
}
