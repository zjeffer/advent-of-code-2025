const DIAL_START: i32 = 50;
const DIAL_MIN: i32 = 0;
const DIAL_MAX: i32 = 99;

struct Dial {
    position: i32,
    counter: i32,
    minimum: i32,
    maximum: i32,
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
        println!("Creating default Dial");
        Self {
            position: DIAL_START,
            counter: 0,
            minimum: DIAL_MIN,
            maximum: DIAL_MAX,
        }
    }
}

impl Dial {
    fn new(minimum: i32, maximum: i32, start: i32) -> Self {
        println!("Creating new Dial with start {start}, range: [{minimum},{maximum}]");
        Self {
            position: start,
            counter: 0,
            minimum,
            maximum,
        }
    }

    fn get_range(&self) -> i32 {
        self.maximum - self.minimum + 1
    }

    fn step(&mut self, direction: Direction, steps: i32) {
        println!(
            "Current position: {}, moving {direction:?} by {steps} steps",
            self.position,
        );

        // every time we move, we need to check if we cross past 0 and increment the counter each time
        // or if we end at 0, increment the counter

        // if the direction is left and we cross below DIAL_MIN, we wrap around to DIAL_MAX
        // if the direction is right and we cross above DIAL_MAX, we wrap around to DIAL_MIN
        match direction {
            Direction::Left => {
                if self.position - steps < self.minimum {
                    if self.position == 0 {
                        self.counter -= 1; // we start at 0, so don't double count
                    }
                    self.position = (self.position - steps + self.get_range()) % self.get_range();

                    // we crossed 0, increment counter
                    self.counter += steps / self.get_range() + 1;
                } else {
                    self.position -= steps;
                    if self.position == 0 {
                        self.counter += 1;
                    }
                }
            }
            Direction::Right => {
                if self.position + steps > DIAL_MAX {
                    self.position = (self.position + steps) % self.get_range();

                    // we crossed 0, increment counter
                    self.counter += steps / self.get_range() + 1;
                } else {
                    self.position += steps;
                    if self.position == 0 {
                        self.counter += 1;
                    }
                }
            }
        }
        println!(
            "    New position: {}, counter: {}",
            self.position, self.counter
        );
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

#[cfg(test)]
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
        assert_eq!(dial.counter, 6);

        println!("Test final position: {}", dial.position);
        println!("Test counter: {}", dial.counter);
    }

    #[test]
    fn test_basics() {
        let mut dial = Dial::default();
        dial.step(Direction::Right, 10); // 50 + 10 = 60
        assert_eq!(dial.position, 60);
        assert_eq!(dial.counter, 0);

        dial.step(Direction::Left, 20); // 60 - 20 = 40
        assert_eq!(dial.position, 40);
        assert_eq!(dial.counter, 0);

        dial = Dial::default();
        dial.step(Direction::Left, 50); // 50 - 50 = 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 1);
    }

    #[test]
    fn test_wrap_around() {
        let mut dial = Dial::default();
        dial.step(Direction::Left, 60); // 50 - 60 = -10 -> wraps to 90
        assert_eq!(dial.position, 90);
        assert_eq!(dial.counter, 1); // crossed 0 once

        dial.step(Direction::Right, 150); // 90 + 150 = 240 -> wraps to 40 (=240 % DIAL_RANGE)
        assert_eq!(dial.position, 40);
        assert_eq!(dial.counter, 3); // crossed 0 twice more

        dial = Dial::default();
        dial.step(Direction::Right, 50); // 50 + 50 = 100 -> wraps to 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 1);

        dial = Dial::default();
        dial.step(Direction::Left, 49); // 50 - 49 = 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 0);
        dial.step(Direction::Left, 2); // 1 - 2 = -1 -> wraps to 99
        assert_eq!(dial.position, 99);
        assert_eq!(dial.counter, 1); // crossed 0 once
        dial.step(Direction::Right, 2); // 99 + 2 = 101 -> wraps to 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 2); // crossed 0 once more

        dial = Dial::default();
        dial.step(Direction::Right, 250); // 50 + 250 = 300 -> wraps to 0 (=300 % DIAL_RANGE)
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 3); // crossed 0 twice + ended on 0
    }
}
