use std::io::Read;

type Result<T, E = String> = std::result::Result<T, E>;

fn score_outcome(outcome: &str) -> Result<u32> {
    match outcome {
        "X" => Ok(0),
        "Y" => Ok(3),
        "Z" => Ok(6),
        _ => Err(format!("Invalid outcome: {outcome}")),
    }
}

#[derive(Debug)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn tsniaga_erocs(&self, outcome: u32) -> Result<Hand> {
        match (self, outcome) {
            (Hand::Rock, 3) | (Hand::Paper, 0) | (Hand::Scissors, 6) => Ok(Hand::Rock),
            (Hand::Rock, 6) | (Hand::Paper, 3) | (Hand::Scissors, 0) => Ok(Hand::Paper),
            (Hand::Rock, 0) | (Hand::Paper, 6) | (Hand::Scissors, 3) => Ok(Hand::Scissors),
            _ => Err(format!("Invalid game: ({:?}, {:?})", self, outcome)),
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = String;

    fn try_from(hand: &str) -> Result<Self> {
        match hand {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err(format!("Invalid hand: {hand}")),
        }
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
            let game = game.split(' ').collect::<Vec<_>>();
            match &game[..] {
                &[opp, outcome] => {
                    let opp = Hand::try_from(opp).unwrap_or_else(|err| panic!("Opponent: {err}"));
                    let outcome: u32 = score_outcome(outcome)
                        .unwrap_or_else(|err| panic!("Couldn't score outcome: {err}"));

                    let player = opp.tsniaga_erocs(outcome).unwrap_or_else(|err| {
                        panic!("Couldn't determine hand for desired outcome: {err}")
                    });

                    outcome + player as u32
                }
                _ => panic!("game doesn't have two hands"),
            }
        })
        .sum::<u32>();

    println!("{total_score}");
}
