use std::{char, fs};

#[derive(Debug, Clone)]
struct NumberMeta {
    value: u32,
    len: usize,
    x: usize,
    y: usize,
    used: bool,
}

#[derive(Debug)]
struct SymbolMeta {
    x: usize,
    y: usize,
    symbol: Option<char>,
}

fn parse_data(input: &str, number_list: &mut Vec<NumberMeta>, symbol_list: &mut Vec<SymbolMeta>) {
    for (y, line) in input.lines().enumerate() {
        let mut x = 0;

        while x < line.len() {
            if line.chars().nth(x).is_some_and(|ch| ch == '.') {
                x += 1;
                continue;
            }

            if line.chars().nth(x).is_some_and(|ch| !ch.is_ascii_digit()) {
                symbol_list.push(SymbolMeta {
                    x,
                    y,
                    symbol: line.chars().nth(x),
                });
            }

            let mut rx = line.len();
            while rx > x {
                let subline = &line[x..rx];
                if let Ok(realised_number) = subline.parse::<u32>() {
                    number_list.push(NumberMeta {
                        value: realised_number,
                        len: subline.len(),
                        used: false,
                        x,
                        y,
                    });
                    x += subline.len() - 1;
                }
                rx -= 1;
            }

            x += 1;
        }
    }
}

fn main() {
    let contents =
        fs::read_to_string("./inputs/task.txt").expect("Should have been able to read the file");

    let mut parsed_number_list: Vec<NumberMeta> = Vec::new();
    let mut parsed_symbol_list: Vec<SymbolMeta> = Vec::new();
    parse_data(&contents, &mut parsed_number_list, &mut parsed_symbol_list);

    let mut parts_sum = 0;
    let mut gears_sum = 0;

    for symbol in parsed_symbol_list.iter() {
        let mut found_numbers: Vec<NumberMeta> = Vec::new();

        for dx in [0, 1, 2] {
            for dy in [0, 1, 2] {
                let cx = symbol.x + dx - 1;
                let cy = symbol.y + dy - 1;

                for num in parsed_number_list.iter_mut() {
                    if num.y == cy {
                        let possible_x = num.x..num.x + num.len;
                        if possible_x.contains(&cx) && !num.used {
                            num.used = true;
                            found_numbers.push(num.clone());
                        }
                    }
                }
            }
        }

        if symbol.symbol == Some('*') && found_numbers.len() == 2 {
            gears_sum += found_numbers[0].value * found_numbers[1].value;
        }

        for num in found_numbers {
            parts_sum += num.value;
        }
    }

    println!("Parts sum: {}", parts_sum);
    println!("Gears sum: {}", gears_sum);
}
