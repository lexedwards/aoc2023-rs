use std::fs;

static NUMS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_and_last_digit(encoded_string: &str) -> Result<u32, &str> {
    if encoded_string.len() < 2 {
        return Err("invalid string length");
    };
    let mut replaced_encoded = encoded_string.to_owned();
    for (i, word) in NUMS.iter().enumerate() {
        replaced_encoded =
            replaced_encoded.replace(word, format!("{}{}{}", word, i, word).as_str());
    }
    let int_values: Vec<u32> = replaced_encoded
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|d| {
            d.to_string()
                .trim()
                .parse::<u32>()
                .unwrap_or_else(|u| panic!("tried to parse {} as int", u))
        })
        .collect();
    Ok(int_values[0] * 10 + int_values[int_values.len() - 1])
}

fn main() {
    let contents =
        fs::read_to_string("./inputs/task_01.txt").expect("Should have been able to read the file");
    let sum_total = contents
        .trim()
        .lines()
        .map(|line| first_and_last_digit(line).unwrap())
        .reduce(|acc, val| acc + val)
        .unwrap();
    println!("total: {}", sum_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_last_digit() {
        assert_eq!(first_and_last_digit("1twothree4"), Ok(14))
    }

    #[test]
    fn test_first_and_last_digit_error() {
        assert!(first_and_last_digit("").is_err())
    }

    #[test]
    fn test_first_and_last_digit_all_spelling() {
        assert_eq!(first_and_last_digit("onetwothree"), Ok(13))
    }

    #[test]
    fn test_first_and_last_digit_only_one_digit() {
        assert_eq!(first_and_last_digit("1twothree"), Ok(13))
    }
}
