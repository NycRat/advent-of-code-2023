fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input/3.input");
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut part_numbers: Vec<Vec<bool>> = vec![vec![false; chars[0].len() + 2]; chars.len() + 2];

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] != '.' && !chars[i][j].is_numeric() {
                // (-1,-1) => (0,0)
                part_numbers[i + 1 - 1][j + 1 - 1] = true;
                part_numbers[i + 1 + 1][j + 1 + 1] = true;
                part_numbers[i + 1 + 1][j + 1 - 1] = true;
                part_numbers[i + 1 - 1][j + 1 + 1] = true;
                part_numbers[i + 1][j + 1 - 1] = true;
                part_numbers[i + 1][j + 1 + 1] = true;
                part_numbers[i + 1 - 1][j + 1] = true;
                part_numbers[i + 1 + 1][j + 1] = true;
            }
        }
    }

    let mut cur_number = 0;
    let mut is_part = false;
    let mut ans = 0;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j].is_numeric() {
                if !is_part {
                    is_part = part_numbers[i + 1][j + 1];
                }
                cur_number = cur_number * 10 + chars[i][j].to_digit(10).unwrap();
            } else {
                if is_part {
                    ans += cur_number;
                }
                cur_number = 0;
                is_part = false;
            }
        }
        if is_part {
            ans += cur_number;
        }
        cur_number = 0;
        is_part = false;
    }

    println!("{ans:?}");
}

fn part2() {
    let input = include_str!("../input/3.input");
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut part_numbers: Vec<Vec<Option<i128>>> =
        vec![vec![None; chars[0].len() + 2]; chars.len() + 2];
    let mut ans = 0i128;

    let mut current_number = 0i128;
    let mut start_index: i32 = -1;
    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if let Some(digit) = chars[i][j].to_digit(10) {
                if start_index == -1 {
                    start_index = j as i32;
                }
                current_number = current_number * 10 + digit as i128;
            } else {
                if start_index != -1 {
                    for k in start_index as usize..j {
                        part_numbers[i + 1][k + 1] = Some(current_number);
                    }
                }
                current_number = 0;
                start_index = -1;
            }
        }
        if start_index != -1 {
            for k in start_index as usize..chars.len() {
                part_numbers[i + 1][k + 1] = Some(current_number);
            }
        }
    }

    // println!("{part_numbers:?}");

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] == '*' {
                let mut adjacent_parts: Vec<i128> = vec![];

                // top
                if part_numbers[i + 1 - 1][j + 1].is_none() {
                    if let Some(part) = part_numbers[i + 1 - 1][j + 1 - 1] {
                        adjacent_parts.push(part);
                    }
                    if let Some(part) = part_numbers[i + 1 - 1][j + 1 + 1] {
                        adjacent_parts.push(part);
                    }
                } else {
                    if let Some(part) = part_numbers[i + 1 - 1][j + 1 - 1] {
                        adjacent_parts.push(part);
                    } else if let Some(part) = part_numbers[i + 1 - 1][j + 1] {
                        adjacent_parts.push(part);
                    } else if let Some(part) = part_numbers[i + 1 - 1][j + 1 + 1] {
                        adjacent_parts.push(part);
                    }
                }

                // bottom
                if part_numbers[i + 1 + 1][j + 1].is_none() {
                    if let Some(part) = part_numbers[i + 1 + 1][j + 1 - 1] {
                        adjacent_parts.push(part);
                    }
                    if let Some(part) = part_numbers[i + 1 + 1][j + 1 + 1] {
                        adjacent_parts.push(part);
                    }
                } else {
                    if let Some(part) = part_numbers[i + 1 + 1][j + 1 - 1] {
                        adjacent_parts.push(part);
                    } else if let Some(part) = part_numbers[i + 1 + 1][j + 1] {
                        adjacent_parts.push(part);
                    } else if let Some(part) = part_numbers[i + 1 + 1][j + 1 + 1] {
                        adjacent_parts.push(part);
                    }
                }

                // left
                if let Some(part) = part_numbers[i + 1][j + 1 - 1] {
                    adjacent_parts.push(part);
                }
                // right
                if let Some(part) = part_numbers[i + 1][j + 1 + 1] {
                    adjacent_parts.push(part);
                }

                if adjacent_parts.len() == 2 {
                    // println!("{}, {}", adjacent_parts[0], adjacent_parts[1]);
                    ans += adjacent_parts[0] * adjacent_parts[1];
                }
            }
        }
    }

    println!("{ans:?}");
}
