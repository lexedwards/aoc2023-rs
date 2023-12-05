use std::fs;
#[derive(Debug)]
struct GameData {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}

fn parse_str_data(line: &str) -> Result<GameData, &str> {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let mut game_data = GameData {
        id: parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap(),
        blue: 0,
        red: 0,
        green: 0,
    };

    let game_sets = parts[1].split("; ").collect::<Vec<&str>>();
    for set in game_sets {
        let num_colors = set.split(", ").collect::<Vec<&str>>();
        for color in num_colors {
            let cube_counters = color.split(" ").collect::<Vec<&str>>();
            let cube_count = cube_counters[0].parse::<u32>().unwrap();

            match cube_counters[1] {
                "blue" => {
                    if game_data.blue < cube_count {
                        game_data.blue = cube_count;
                    };
                }
                "red" => {
                    if game_data.red < cube_count {
                        game_data.red = cube_count;
                    }
                }
                "green" => {
                    if game_data.green < cube_count {
                        game_data.green = cube_count;
                    }
                }
                _ => {
                    panic!("Wasn't expecting this!")
                }
            };
        }
    }
    Ok(game_data)
}

const BOH_BLUE: u32 = 14;
const BOH_RED: u32 = 12;
const BOH_GREEN: u32 = 13;

fn main() {
    let contents =
        fs::read_to_string("./inputs/task.txt").expect("Should have been able to read the file");
    let parsed_data = contents
        .trim()
        .lines()
        .map(|lines| parse_str_data(lines).unwrap());
    let mut total_limited: u32 = 0;
    let mut total_power: u32 = 0;

    for data in parsed_data {
        let is_limited = data.blue <= BOH_BLUE && data.red <= BOH_RED && data.green <= BOH_GREEN;

        if is_limited {
            total_limited += data.id;
        };

        total_power += data.blue * data.red * data.green;
    }
    println!("total_limited: {}, power: {}", total_limited, total_power);
}
