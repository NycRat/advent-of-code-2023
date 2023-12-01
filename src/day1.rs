use std::collections::HashMap;

fn main() {
    // part1();
    part2();
}

fn part1() {
    println!(
        "{}",
        include_str!("../input/1.input")
            .lines()
            .map(|line| {
                let ha: Vec<u32> = line
                    .chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();
                ha.first().unwrap() * 10 + ha.last().unwrap()
            })
            .sum::<u32>()
    );
}

fn part2() {
    println!(
        "{}",
        include_str!("../input/1.input")
            .lines()
            .map(|line| {
                let nums: HashMap<&str, u32> = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .into_iter()
                .enumerate()
                .map(|(i, s)| (s, (i + 1) as u32))
                .collect();

                let digits: Vec<u32> = (0..line.len())
                    .map(|i| {
                        let c = line.chars().nth(i).unwrap();
                        if let Some(digit) = c.to_digit(10) {
                            return Some(digit);
                        }
                        for end in i + 3..=(usize::min(i + 5, line.len())) {
                            let ha = &line[i..end];
                            if let Some(digit) = nums.get(ha) {
                                return Some(*digit);
                            }
                        }
                        None
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect();

                // println!("{}", digits.first().unwrap() * 10 + digits.last().unwrap());
                // println!("{digits:?}");
                digits.first().unwrap() * 10 + digits.last().unwrap()
            })
            .sum::<u32>()
    );
}
