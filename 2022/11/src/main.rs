use std::collections::VecDeque;

use common::*;

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Num(u32),
    Old,
}

impl TryFrom<&str> for Operand {
    type Error = Failure;

    fn try_from(operand_token: &str) -> Result<Self, Self::Error> {
        let operand = match operand_token {
            "old" => Self::Old,
            num_token => Self::Num(
                num_token
                    .parse()
                    .map_err(|err| e!("Couldn't parse num token: {err}"))?,
            ),
        };

        Ok(operand)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

impl TryFrom<&str> for Operation {
    type Error = Failure;

    fn try_from(string: &str) -> Result<Self> {
        let mut tokens = string.trim().split(' ').skip(4);
        let operator_token = tokens
            .next()
            .ok_or_else(|| e!("Did not find operator token"))?;
        let operand_token = tokens
            .next()
            .ok_or_else(|| e!("Did not find operand token"))?;
        let operand = Operand::try_from(operand_token)?;

        let operation = match operator_token {
            "+" => Self::Add(operand),
            "*" => Self::Multiply(operand),
            op => return Err(e!("Found unrecognised operator: {op}")),
        };

        Ok(operation)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    divisor: u32,
    catchers: (usize, usize),
    items_inspected: u32,
}

impl TryFrom<&str> for Monkey {
    type Error = Failure;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let mut lines = string.split('\n').skip(1);
        let starting_items_line = lines
            .next()
            .ok_or_else(|| e!("Expected a 'Starting items' line"))?;
        let items = starting_items_line
            .trim()
            .split(' ')
            .skip(2)
            .map(|num_token| {
                let num_token = num_token
                    .split(',')
                    .next()
                    .ok_or_else(|| e!("Expected number token"))?;
                let num_token = num_token
                    .parse::<u32>()
                    .map_err(|err| e!("Couldn't parse number token: {err}"))?;
                Ok(num_token)
            })
            .collect::<Result<_>>()?;

        let operation_line = lines
            .next()
            .ok_or_else(|| e!("Expected a 'Operation' line"))?;
        let operation = Operation::try_from(operation_line)?;

        let divisible_by_line = lines
            .next()
            .ok_or_else(|| e!("Expected a 'Test: divisible by' line"))?;
        let divisor_token = divisible_by_line.trim().split(' ').nth(3).ok_or_else(|| {
            e!(
                "Didn't find 4th token in 'Test: divisible by' line: {}",
                divisible_by_line.trim()
            )
        })?;
        let divisor: u32 = divisor_token
            .parse()
            .map_err(|err| e!("Couldn't parse divisor {err}"))?;

        let mut catchers = lines
            .take(2)
            .map(|line| {
                let catcher_token = line
                    .trim()
                    .split(' ')
                    .nth(5)
                    .ok_or_else(|| e!("Couldn't find 6th token on condition line"))?;
                catcher_token
                    .parse::<usize>()
                    .map_err(|err| e!("Couldn't parse catcher token: {err}"))
            })
            .collect::<Result<VecDeque<_>>>()?;

        let catchers = (
            catchers
                .pop_front()
                .ok_or_else(|| e!("Couldn't find catcher 0"))?,
            catchers
                .pop_front()
                .ok_or_else(|| e!("Couldn't find catcher 1"))?,
        );

        let monkey = Self {
            items,
            operation,
            divisor,
            catchers,
            items_inspected: 0,
        };

        Ok(monkey)
    }
}

impl Monkey {
    fn throw_item(&mut self) -> Option<(usize, u32)> {
        self.items.pop_front().map(|item| {
            let item = match self.operation {
                Operation::Add(Operand::Num(num)) => item + num,
                Operation::Add(Operand::Old) => item + item,
                Operation::Multiply(Operand::Num(num)) => item * num,
                Operation::Multiply(Operand::Old) => item * item,
            };
            let item = item / 3;
            let throw = if item % self.divisor == 0 {
                (self.catchers.0, item)
            } else {
                (self.catchers.1, item)
            };

            self.items_inspected += 1;
            throw
        })
    }

    fn catch_item(&mut self, item: u32) {
        self.items.push_front(item);
    }
}

struct Monkeys(Vec<Monkey>);

impl TryFrom<&str> for Monkeys {
    type Error = Failure;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let monkeys = string
            .split("\n\n")
            .map(Monkey::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(monkeys))
    }
}

impl Monkeys {
    fn snapshot(&self) -> Result<String> {
        use std::fmt::Write;

        let mut buf = String::new();
        for (i, monkey) in self.0.iter().enumerate() {
            write!(buf, "Monkey {i}: ").map_err(|err| e!("Couldn't write to buf: {err}"))?;
            for item in &monkey.items {
                write!(buf, "{item}, ").map_err(|err| e!("Couldn't write to buf: {err}"))?;
            }
            writeln!(buf).map_err(|err| e!("Couldn't write to buf: {err}"))?;
        }

        Ok(buf)
    }

    fn round_of_monkey_business(mut self) -> Result<Self> {
        for i in 0..self.0.len() {
            let mut thrower = self.0[i].clone();
            while let Some((index, item)) = thrower.throw_item() {
                let mut catcher = self.0[index].clone();
                catcher.catch_item(item);
                let _ = std::mem::replace(&mut self.0[index], catcher);
            }
            let _ = std::mem::replace(&mut self.0[i], thrower);
        }

        Ok(self)
    }
}

fn monkey_business_from_input(input: &str) -> Result<u32> {
    let mut monkeys = Monkeys::try_from(input)?;

    for i in 0..20 {
        monkeys = monkeys.round_of_monkey_business()?;
    }

    let mut monkeys = monkeys
        .0
        .into_iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<_>>();

    monkeys.sort();

    let answer = monkeys.into_iter().rev().take(2).product::<u32>();

    Ok(answer)
}

fn main() -> Result<()> {
    let input = get_input()?;

    let answer = monkey_business_from_input(input.as_str())?;

    println!("{answer}");

    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::{Monkey, Operand, Operation, Result};

    #[test]
    fn parse_operation() -> Result<()> {
        let operation_string = "  Operation: new = old * 13";
        let operation = Operation::try_from(operation_string)?;

        assert_eq!(Operation::Multiply(Operand::Num(13)), operation);

        let operation_string = "  Operation: new = old + 4";
        let operation = Operation::try_from(operation_string)?;

        assert_eq!(Operation::Add(Operand::Num(4)), operation);

        let operation_string = "  Operation: new = old * old";
        let operation = Operation::try_from(operation_string)?;

        assert_eq!(Operation::Multiply(Operand::Old), operation);

        Ok(())
    }

    #[test]
    fn parse_monkeys() -> Result<()> {
        let string = "Monkey 1:
  Starting items: 73, 99, 55, 54, 88, 50, 55
  Operation: new = old + 4
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 6

Monkey 2:
  Starting items: 67, 98
  Operation: new = old * 11
  Test: divisible by 5
    If true: throw to monkey 6
    If false: throw to monkey 5";
        let monkeys = string
            .split("\n\n")
            .map(Monkey::try_from)
            .collect::<Result<Vec<_>>>()?;

        let expected = [
            Monkey {
                items: VecDeque::from_iter([73, 99, 55, 54, 88, 50, 55]),
                operation: Operation::Add(Operand::Num(4)),
                divisor: 17,
                catchers: (2, 6),
                items_inspected: 0,
            },
            Monkey {
                items: VecDeque::from_iter([67, 98]),
                operation: Operation::Multiply(Operand::Num(11)),
                divisor: 5,
                catchers: (6, 5),
                items_inspected: 0,
            },
        ];

        assert_eq!(monkeys[0], expected[0]);

        assert_eq!(monkeys[1], expected[1]);

        Ok(())
    }

    #[test]
    fn monkey_throw_items() -> Result<()> {
        let mut monkey = Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: Operation::Multiply(Operand::Old),
            divisor: 13,
            catchers: (1, 3),
            items_inspected: 0,
        };

        let items = [
            monkey.throw_item(),
            monkey.throw_item(),
            monkey.throw_item(),
        ];

        assert_eq!(
            &[Some((1, 2080)), Some((3, 1200)), Some((3, 3136))],
            &items[..]
        );

        Ok(())
    }

    #[test]
    fn monkey_business_from_input() -> Result<()> {
        let input = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let monkey_business = super::monkey_business_from_input(input)?;

        assert_eq!(10605, monkey_business);

        Ok(())
    }
}
