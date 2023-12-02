use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input/2.input");
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let line = line.split_once(": ").unwrap();
                let id: i32 = line.0.split_once(" ").unwrap().1.parse().unwrap();
                let thing: HashMap<&str, i32> = [("red", 12), ("green", 13), ("blue", 14)]
                    .into_iter()
                    .collect();

                for (amount, color) in
                    line.1
                        .split("; ")
                        .flat_map(|hand| hand.split(", "))
                        .map(|a| {
                            let (a, c) = a.split_once(" ").unwrap();
                            (a.parse::<i32>().unwrap(), c)
                        })
                {
                    if &amount > thing.get(color).unwrap() {
                        return 0;
                    }
                }
                return id;
            })
            .sum::<i32>()
    );
}

fn part2() {
    let input = include_str!("../input/2.input");
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let line = line.split_once(": ").unwrap();
                let mut thing: HashMap<&str, i32> = [("red", 0), ("green", 0), ("blue", 0)]
                    .into_iter()
                    .collect();

                for (amount, color) in
                    line.1
                        .split("; ")
                        .flat_map(|hand| hand.split(", "))
                        .map(|a| {
                            let (a, c) = a.split_once(" ").unwrap();
                            (a.parse::<i32>().unwrap(), c)
                        })
                {
                    *thing.get_mut(color).unwrap() = *thing.get(color).unwrap().max(&amount);
                }
                let mut aaaa = 1;
                for (_, a) in thing {
                    aaaa *= a;
                }
                return aaaa;
            })
            .sum::<i32>()
    );
}
