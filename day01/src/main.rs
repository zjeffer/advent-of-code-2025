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

    fn set_position(&mut self, position: i32) {
        if position < self.minimum || position > self.maximum {
            panic!(
                "Position {position} out of range [{},{}]",
                self.minimum, self.maximum
            );
        }
        self.position = position;
    }

    fn step_with_for_loop(&mut self, direction: Direction, steps: i32) {
        println!(
            "Current position: {}, moving {direction:?} by {steps} steps",
            self.position,
        );
        for _ in 1..=steps {
            match direction {
                Direction::Left => {
                    if self.position == self.minimum {
                        self.position = self.maximum;
                    } else {
                        self.position -= 1;
                    }
                }
                Direction::Right => {
                    if self.position == self.maximum {
                        self.position = self.minimum;
                    } else {
                        self.position += 1;
                    }
                }
            }
            if self.position == 0 {
                self.counter += 1;
            }
        }
    }

    fn step(&mut self, direction: Direction, steps: i32) {
        println!(
            "Current position: {}, moving {direction:?} by {steps} steps",
            self.position,
        );

        // every time we move, we need to check if we cross past 0 and increment the counter each time
        // or if we end at 0, increment the counter

        // to calculate where the position will land
        let absolute_steps = steps % self.get_range();

        let new_position: i32;
        match direction {
            Direction::Left => {
                // calculate new position
                new_position = self.position - absolute_steps;
                println!(
                    "    Calculated new position (before wrap): {}",
                    new_position
                );
            }
            Direction::Right => {
                new_position = self.position + absolute_steps;
                println!(
                    "    Calculated new position (before wrap): {}",
                    new_position
                );
            }
        }

        // calculate how many 0s are between the old position and the new position
        match direction {
            Direction::Left => {
                if new_position < self.minimum {
                    // wrapped around
                    let distance_to_min = self.position - self.minimum;
                    let distance_beyond_min = absolute_steps - distance_to_min;
                    let full_wraps = distance_beyond_min / self.get_range();
                    let remainder = distance_beyond_min % self.get_range();
                    self.counter += 1 + full_wraps;
                    if remainder == 0 {
                        self.counter -= 1; // ended exactly on 0
                    }
                } else if new_position == 0 {
                    self.counter += 1;
                }
            }
            Direction::Right => {
                if new_position > self.maximum {
                    // wrapped around
                    let distance_to_max = self.maximum - self.position;
                    let distance_beyond_max = absolute_steps - distance_to_max;
                    let full_wraps = distance_beyond_max / self.get_range();
                    let remainder = distance_beyond_max % self.get_range();
                    self.counter += 1 + full_wraps;
                    if remainder == 0 {
                        self.counter -= 1; // ended exactly on 0
                    }
                } else if new_position == 0 {
                    self.counter += 1;
                }
            }
        }

        self.set_position(
            ((new_position - self.minimum).rem_euclid(self.get_range())) + self.minimum,
        );

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
    let mut dial = Dial::new(DIAL_MIN, DIAL_MAX, DIAL_START);

    let input: &str = include_str!("../data/input.txt");
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (direction, steps_str) = Dial::parse_line(line);
        dial.step_with_for_loop(direction, steps_str);
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
    fn test_example_for_loop() {
        const EXAMPLE_INPUT: &str = include_str!("../data/example.txt");
        let mut dial = Dial::default();
        for line in EXAMPLE_INPUT.lines() {
            let (direction, steps_str) = Dial::parse_line(line);
            dial.step_with_for_loop(direction, steps_str);
        }
        assert_eq!(dial.position, 32);
        assert_eq!(dial.counter, 6);

        println!("Test final position (for loop): {}", dial.position);
        println!("Test counter (for loop): {}", dial.counter);
    }

    #[test]
    fn test_full() {
        let mut dial = Dial::default();

        let input: &str = include_str!("../data/input.txt");
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let (direction, steps_str) = Dial::parse_line(line);
            dial.step(direction, steps_str);
        }
        assert_eq!(dial.position, 68);
        assert_eq!(dial.counter, 6228);
    }

    #[test]
    fn test_full_for_loop() {
        let mut dial = Dial::default();

        let input: &str = include_str!("../data/input.txt");
        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let (direction, steps_str) = Dial::parse_line(line);
            dial.step_with_for_loop(direction, steps_str);
        }
        assert_eq!(dial.position, 68);
        assert_eq!(dial.counter, 6228);
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

        dial.step(Direction::Left, 1); // 0 - 1 = -1 -> wraps to 99
        assert_eq!(dial.position, 99);
        assert_eq!(dial.counter, 3); // no additional crossing

        dial.step(Direction::Right, 1); // 99 + 1 = 100 -> wraps to 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 4); // crossed 0 once more

        dial.step(Direction::Right, 1); // 0 + 1 = 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 4); // no additional crossing

        dial.step(Direction::Left, 1); // 1 - 1 = 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 5); // crossed 0 once more

        dial.step(Direction::Left, 100); // 0 - 100 = -100 -> wraps to 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 6); // crossed 0 once more

        dial.step(Direction::Right, 201); // 0 + 201 = 201 -> wraps to 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 8); // crossed 0 twice more (at 100 and 200)

        dial.step(Direction::Left, 202); // 1 - 202 = -201 -> wraps to 99
        assert_eq!(dial.position, 99);
        assert_eq!(dial.counter, 11); // crossed 0 3 times more (at 0, -100, -200)

        dial.step(Direction::Right, 398); // 99 + 398 = 497 -> wraps to 97
        assert_eq!(dial.position, 97);
        assert_eq!(dial.counter, 15); // crossed 0 4 times more (at 100, 200, 300, 400)
    }

    #[test]
    fn test_wrap_around_with_for_loop() {
        let mut dial = Dial::default();
        dial.step_with_for_loop(Direction::Left, 60); // 50 - 60 = -10 -> wraps to 90
        assert_eq!(dial.position, 90);
        assert_eq!(dial.counter, 1); // crossed 0 once

        dial.step_with_for_loop(Direction::Right, 150); // 90 + 150 = 240 -> wraps to 40 (=240 % DIAL_RANGE)
        assert_eq!(dial.position, 40);
        assert_eq!(dial.counter, 3); // crossed 0 twice more

        dial = Dial::default();
        dial.step_with_for_loop(Direction::Right, 50); // 50 + 50 = 100 -> wraps to 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 1);

        dial = Dial::default();
        dial.step_with_for_loop(Direction::Left, 49); // 50 - 49 = 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 0);
        dial.step_with_for_loop(Direction::Left, 2); // 1 - 2 = -1 -> wraps to 99
        assert_eq!(dial.position, 99);
        assert_eq!(dial.counter, 1); // crossed 0 once
        dial.step_with_for_loop(Direction::Right, 2); // 99 + 2 = 101 -> wraps to 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 2); // crossed 0 once more

        dial = Dial::default();
        dial.step_with_for_loop(Direction::Right, 250); // 50 + 250 = 300 -> wraps to 0 (=300 % DIAL_RANGE)
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 3); // crossed 0 twice + ended on 0

        dial.step_with_for_loop(Direction::Left, 1); // 0 - 1 = -1 -> wraps to 99
        assert_eq!(dial.position, 99);
        assert_eq!(dial.counter, 3); // no additional crossing

        dial.step_with_for_loop(Direction::Right, 1); // 99 + 1 = 100 -> wraps to 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 4); // crossed 0 once more

        dial.step_with_for_loop(Direction::Right, 1); // 0 + 1 = 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 4); // no additional crossing

        dial.step_with_for_loop(Direction::Left, 1); // 1 - 1 = 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 5); // crossed 0 once more

        dial.step_with_for_loop(Direction::Left, 100); // 0 - 100 = -100 -> wraps to 0
        assert_eq!(dial.position, 0);
        assert_eq!(dial.counter, 6); // crossed 0 once more

        dial.step_with_for_loop(Direction::Right, 201); // 0 + 201 = 201 -> wraps to 1
        assert_eq!(dial.position, 1);
        assert_eq!(dial.counter, 8); // crossed 0 twice more (at 100 and 200)

        dial.step_with_for_loop(Direction::Left, 202); // 1 - 202 = -201 -> wraps to 99
        assert_eq!(dial.position, 99);
        assert_eq!(dial.counter, 11); // crossed 0 3 times more (at 0, -100, -200)

        dial.step_with_for_loop(Direction::Right, 398); // 99 + 398 = 497 -> wraps to 97
        assert_eq!(dial.position, 97);
        assert_eq!(dial.counter, 15); // crossed 0 4 times more (at 100, 200, 300, 400)
    }
}
