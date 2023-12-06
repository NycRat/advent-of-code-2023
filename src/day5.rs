use std::collections::VecDeque;

fn main() {
    part1();
    part2();
}

type Map = Vec<Mapping>;

#[derive(Debug)]
struct Mapping {
    from: i128,
    to: i128,
    range: i128,
}

fn part1() {
    let input = include_str!("../input/5.input");
    let thing: Vec<&str> = input.split("\n\n").collect();
    // println!("{:?}", thing[0]);

    let maps: Vec<Map> = thing
        .iter()
        .skip(1)
        .map(|a_map| {
            a_map
                .lines()
                .skip(1)
                .map(|mapping| {
                    let thing: Vec<i128> = mapping.split(" ").map(|x| x.parse().unwrap()).collect();
                    Mapping {
                        from: thing[1],
                        to: thing[0],
                        range: thing[2],
                    }
                })
                .collect()
        })
        .collect();

    // println!("{:#?}", maps);

    let seeds: Vec<i128> = thing[0]
        .split_once(" ")
        .unwrap()
        .1
        .split(" ")
        .map(|seed| seed.parse::<i128>().unwrap())
        .map(|seed| {
            let mut seed = seed;
            for map in &maps {
                for mapping in map {
                    // 79 >= 52 && 79 <= 52 + 48 - 1
                    // 79 -> 50 + 79 - 52
                    if seed >= mapping.from && seed <= mapping.from + mapping.range - 1 {
                        // seed = mapping.to + seed - mapping.from;
                        seed = mapping.to + seed - mapping.from;
                        break;
                    }
                }
            }
            seed
        })
        .collect();
    println!("{:?}", seeds.iter().min().unwrap());
}

fn part2() {
    let input = include_str!("../input/5.input");
    let thing: Vec<&str> = input.split("\n\n").collect();

    let maps: Vec<Map> = thing
        .iter()
        .skip(1)
        .map(|a_map| {
            a_map
                .lines()
                .skip(1)
                .map(|mapping| {
                    let thing: Vec<i128> = mapping.split(" ").map(|x| x.parse().unwrap()).collect();
                    Mapping {
                        from: thing[1],
                        to: thing[0],
                        range: thing[2],
                    }
                })
                .collect()
        })
        .collect();

    let seeds: Vec<i128> = thing[0]
        .split_once(" ")
        .unwrap()
        .1
        .split(" ")
        .map(|seed| seed.parse::<i128>().unwrap())
        .collect();

    let mut seeds = seeds.into_iter();
    let mut ans = i128::MAX;

    while seeds.clone().peekable().peek().is_some() {
        let start = seeds.next().unwrap();
        let len = seeds.next().unwrap();

        // if seed >= mapping.from && seed <= mapping.from + mapping.range - 1 {
        // 70 - 100
        // 90 - 120
        // 90 - 100
        // let mut queue: VecDeque<(i128, i128)> = VecDeque::new();
        // queue.push_front((start, len));
        fn check(seed: i128, len: i128, ans: &mut i128, map: &Map, next_map: Option<&Map>) {
            let mut seed = seed;
            for mapping in map {
                if seed >= mapping.from && seed <= mapping.from + mapping.range - 1 {
                    seed = mapping.to + seed - mapping.from;
                    // check(seed, len, ans, map)
                }
            }
            *ans = i128::min(*ans, seed);
        }
        for map in &maps {
            check(start, len, &mut ans, &map, None);
        }
    }
    println!("{ans}");
}
