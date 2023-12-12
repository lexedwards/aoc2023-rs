use std::fs;

fn parse_line(line: &str) -> Vec<Vec<u32>> {
    let parts: Vec<&str> = line.split(": ").collect();

    parts[1]
        .split(" | ")
        .map(|card| -> Vec<u32> {
            card.trim()
                .replace("  ", " ")
                .split(' ')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let contents =
        fs::read_to_string("./inputs/task.txt").expect("Should have been able to read the file");

    let num_of_games = contents.trim().lines().count();
    let parsed = contents.trim().lines().map(parse_line);

    let mut task_1_sum = 0;
    let mut task_2_cards: Vec<u32> = vec![1; num_of_games];
    for (idx, game) in parsed.enumerate() {
        let mut local_count = 0;
        let mut local_sum = 0;
        let winning_nums = &game[0];

        for num in &game[1] {
            let possible_idx = winning_nums.iter().position(|x| x == num);
            if possible_idx.is_some() {
                local_count += 1;
                local_sum = if local_sum == 0 { 1 } else { local_sum * 2 }
            }
        }
        task_1_sum += local_sum;
        if local_count > 0 {
            for n in 0..local_count {
                if idx + n < task_2_cards.len() {
                    task_2_cards[idx + 1 + n] += 1 * task_2_cards[idx];
                }
            }
        }
    }

    println!("total sum: {}", task_1_sum);

    let card_total = task_2_cards.into_iter().reduce(|acc, val| acc + val);

    println!("total cards: {}", card_total.unwrap());
}
