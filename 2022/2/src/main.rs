use std::io::Read;

enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn score_against(&self, opp: Hand) -> u32 {
        match (self, opp) {
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => 6,
            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Scissors, Hand::Rock) => 0,
            (Hand::Rock, Hand::Rock)
            | (Hand::Paper, Hand::Paper)
            | (Hand::Scissors, Hand::Scissors) => 3,
        }
    }
}

fn read_hand(hand: &str) -> Result<Hand, String> {
    match hand {
        "A" | "X" => Ok(Hand::Rock),
        "B" | "Y" => Ok(Hand::Paper),
        "C" | "Z" => Ok(Hand::Scissors),
        _ => Err(format!("Invalid hand: {hand}")),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .expect("Couldn't read stdin");

    let total_score = data
        .split('\n')
        .rev()
        .skip(1)
        .map(|game| {
            println!("game: {game}");
            let game = game.split(' ').collect::<Vec<_>>();
            match &game[..] {
                &[opp, player] => {
                    let opp = read_hand(opp).unwrap_or_else(|err| panic!("Opponent: {err}"));
                    let player = read_hand(player).unwrap_or_else(|err| panic!("Player: {err}"));

                    let score = player.score_against(opp);
                    score + player as u32
                }
                _ => panic!("game doesn't have two hands"),
            }
        })
        .sum::<u32>();

    println!("{total_score}");
}
