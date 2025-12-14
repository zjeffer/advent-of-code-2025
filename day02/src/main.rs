struct Id {
    value: i64,
}

impl Id {
    fn new(value: i64) -> Self {
        Id { value }
    }

    /// A valid ID:
    /// * Does not start with a 0
    /// * Is not a sequence repeated twice in a row
    fn is_valid(&self) -> bool {
        let string_representation = self.value.to_string();
        if string_representation.len() == 1 {
            return true;
        }

        if string_representation.len() % 2 != 0 {
            return true;
        }

        let middle_index = string_representation.len() / 2;
        let (first_half, second_half) = string_representation.split_at(middle_index);
        if first_half == second_half {
            return false;
        }
        return true;
    }
}

fn main() {
    // split on ,
    let id_ranges = include_str!("../data/input.txt")
        .trim()
        .split(',')
        .map(|range| {
            let mut bounds = range.split('-').map(|s| s.parse::<i64>().unwrap());
            let start = bounds.next().unwrap();
            let end = bounds.next().unwrap();
            (start, end)
        })
        .collect::<Vec<(i64, i64)>>();

    let ids_to_check = id_ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .map(Id::new);
    let invalid_ids: Vec<i64> = ids_to_check
        .filter(|id| !id.is_valid())
        .map(|id| id.value)
        .collect();
    let invalid_sum: i64 = invalid_ids.iter().sum();
    println!("Sum of invalid IDs: {}", invalid_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_id_validity() {
        let id = Id::new(1);
        assert!(id.is_valid());

        let id = Id::new(11);
        assert!(!id.is_valid());

        let id = Id::new(1234);
        assert!(id.is_valid());

        let id = Id::new(1212);
        assert!(!id.is_valid());

        let id = Id::new(100);
        assert!(id.is_valid());
    }

    #[test]
    fn test_example() {
        let example_ranges = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];

        let expected_invalid_ids = vec![
            11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859,
        ];

        let expected_sum = expected_invalid_ids.iter().sum::<i64>();

        let ids_to_check = example_ranges
            .iter()
            .flat_map(|(start, end)| *start..=*end)
            .map(Id::new);
        println!("Total IDs to check: {}", ids_to_check.clone().count());
        let invalid_ids: Vec<i64> = ids_to_check
            .filter(|id| !id.is_valid())
            .map(|id| id.value)
            .collect();
        println!("Invalid IDs: {:?}", invalid_ids);
        let invalid_sum: i64 = invalid_ids.iter().sum();

        assert_eq!(invalid_ids, expected_invalid_ids);
        assert_eq!(invalid_sum, expected_sum);
    }
}
