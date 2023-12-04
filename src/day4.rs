use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input/4.input");
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let (winning, have) = line.split_once(" | ").unwrap();
                let winning = winning.split_once(": ").unwrap().1;
                let winning_numbers: HashSet<&str> = winning.split(" ").filter(|x| x != &"").collect();
                let mut n_wins = 0;
                for number in have.split(" ") {
                    if winning_numbers.contains(number) {
                        n_wins = i32::max(1, n_wins * 2);
                    }
                }
                n_wins
            })
            .sum::<i32>()
    );
}

fn part2() {
    let input = include_str!("../input/4.input");
    let mut cards = [0; 300];
    for (i, line) in input.lines().enumerate() {
        let (winning, have) = line.split_once(" | ").unwrap();
        let winning = winning.split_once(": ").unwrap().1;
        let winning_cards: HashSet<&str> = winning.split(" ").filter(|x| x != &"").collect();

        let n_won_cards: usize = have
            .split(" ")
            .map(|x| if winning_cards.contains(x) { 1 } else { 0 })
            .sum();

        cards[i] += 1;
        for won_card in i..i + n_won_cards {
            cards[won_card + 1] += cards[i];
        }
    }
    println!("{}", cards.iter().sum::<i32>());
}
