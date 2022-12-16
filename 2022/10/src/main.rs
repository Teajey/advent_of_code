use common::*;

enum Instruction {
    AddX(i32),
    NoOp,
}

impl TryFrom<&str> for Instruction {
    type Error = Failure;

    fn try_from(string: &str) -> Result<Self> {
        let mut tokens = string.split(' ');
        let instruction = match tokens
            .next()
            .ok_or_else(|| e!("expected at least one token"))?
        {
            "addx" => {
                let token = tokens
                    .next()
                    .ok_or_else(|| e!("'addx' was not followed by another token"))?;
                Instruction::AddX(token.parse().map_err(|err| {
                    e!("token following 'addx' couldn't be parsed as an integer: {err}")
                })?)
            }
            "noop" => Instruction::NoOp,
            unknown => return Err(e!("Unrecognised command: {unknown}")),
        };

        Ok(instruction)
    }
}

struct CentralProcessingUnit {
    x: i32,
    cycle_index: u32,
}

impl CentralProcessingUnit {
    fn new() -> Self {
        Self {
            x: 1,
            cycle_index: 0,
        }
    }

    fn cycle(&mut self, signal_strengths: &mut Vec<i32>) {
        let ord = (self.cycle_index + 1) as i32;

        if (ord - 20) % 40 == 0 {
            signal_strengths.push(ord * self.x);
        }

        self.cycle_index += 1;
    }

    fn run_instruction(&mut self, instruction: Instruction, signal_strengths: &mut Vec<i32>) {
        match instruction {
            Instruction::AddX(v) => {
                self.cycle(signal_strengths);
                self.cycle(signal_strengths);
                self.x += v;
            }
            Instruction::NoOp => {
                self.cycle(signal_strengths);
            }
        }
    }
}

fn get_signal_strengths_sum(raw_instructions: &str) -> Result<Vec<i32>> {
    let mut cpu = CentralProcessingUnit::new();
    let mut signal_strengths = vec![];

    for line in raw_instructions.split('\n') {
        let instruction = Instruction::try_from(line)?;

        cpu.run_instruction(instruction, &mut signal_strengths);
    }

    Ok(signal_strengths)
}

fn main() -> Result<()> {
    let data = get_input()?;

    let signal_strengths = get_signal_strengths_sum(&data)?;

    println!("{}", signal_strengths.into_iter().sum::<i32>());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Result;
    #[test]
    fn get_signal_strengths_sum() -> Result<()> {
        let raw_instructions = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let signal_strengths = super::get_signal_strengths_sum(raw_instructions)?;

        assert_eq!(signal_strengths.into_iter().sum::<i32>(), 13140);

        Ok(())
    }
}
