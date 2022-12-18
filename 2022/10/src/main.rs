use std::fmt::Display;

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

#[derive(Debug)]
struct CathodeRayTube([bool; 40 * 6]);

impl CathodeRayTube {
    fn update_pixel(&mut self, x: i32, cycle_index: i32) {
        let px = cycle_index % (40 * 6);
        let px_x = px % 40;

        let range = x - 1..=x + 1;

        if range.contains(&px_x) {
            self.0[px as usize] = true;
        }
    }
}

impl Display for CathodeRayTube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pixels = self
            .0
            .into_iter()
            .map(|on| if on { '#' } else { '.' })
            .collect::<String>();

        for i in 0..6 {
            let y_start = i * 40;
            writeln!(f, "{}", &pixels[y_start..y_start + 40])?;
        }

        Ok(())
    }
}

struct CentralProcessingUnit {
    x: i32,
    cycle_index: i32,
    tube: CathodeRayTube,
}

impl CentralProcessingUnit {
    fn new() -> Self {
        Self {
            x: 1,
            cycle_index: 0,
            tube: CathodeRayTube([false; 40 * 6]),
        }
    }

    fn cycle(&mut self) {
        self.tube.update_pixel(self.x, self.cycle_index);

        self.cycle_index += 1;
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddX(v) => {
                self.cycle();
                self.cycle();
                self.x += v;
            }
            Instruction::NoOp => {
                self.cycle();
            }
        }
    }

    fn execute_code(&mut self, code: &str) -> Result<()> {
        for line in code.split('\n') {
            let instruction = Instruction::try_from(line)?;

            self.run_instruction(instruction);
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let data = get_input()?;

    let mut cpu = CentralProcessingUnit::new();

    cpu.execute_code(&data)?;

    println!("{}", cpu.tube);

    Ok(())
}
