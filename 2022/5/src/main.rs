use std::io::Read;

type Result<T, E = String> = std::result::Result<T, E>;

struct Supply(Vec<Vec<char>>);

impl Supply {
    fn add_stack(&mut self) {
        let stack = Vec::new();
        self.0.push(stack);
    }

    fn get_or_add_stack_mut(&mut self, index: usize) -> Result<&mut Vec<char>> {
        if self.0.len() > index {
            Ok(self
                .0
                .get_mut(index)
                .expect("length guaranteed in condition above"))
        } else {
            self.add_stack();
            self.0
                .get_mut(index)
                .ok_or_else(|| format!("Couldn't get stack {index} after creating a new one"))
        }
    }

    fn move_crates(&mut self, quantity: usize, origin: usize, destination: usize) -> Result<()> {
        let crates_to_move = {
            let origin = self
                .0
                .get_mut(origin - 1)
                .ok_or_else(|| format!("Tried to move from non-existent stack {origin}"))?;

            let new_length = origin.len() - quantity;

            origin.drain(new_length..).collect::<Vec<_>>()
        };

        let destination = self
            .0
            .get_mut(destination - 1)
            .ok_or_else(|| format!("Tried to move to non-existent stack {destination}"))?;

        destination.extend(crates_to_move);

        Ok(())
    }

    fn try_deserialize(supply_diagram: &str) -> Result<Self> {
        let stack_layers = supply_diagram.split('\n').rev().skip(1);

        let mut supply = Self(Vec::<Vec<char>>::new());

        for stack_layer in stack_layers {
            let stack_layer = stack_layer.chars().skip(1).step_by(4);
            for (i, cr8) in stack_layer.enumerate() {
                let stack = supply.get_or_add_stack_mut(i)?;
                if cr8.is_uppercase() {
                    stack.push(cr8);
                }
            }
        }

        Ok(supply)
    }

    fn skim_top_crates(self) -> String {
        self.0
            .into_iter()
            .map(|stack| stack.last().cloned().unwrap_or(' '))
            .collect::<String>()
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| format!("Couldn't read stdin: {err}"))?;

    let sections = data.split("\n\n").collect::<Vec<_>>();

    let &[supply_diagram, instructions] = &sections[..] else {
        return Err(r#"Input didn't contain the expected two "\n\n" separated sections"#.to_owned());
    };

    let mut supply = Supply::try_deserialize(supply_diagram)?;

    let instructions = instructions
        .split('\n')
        .map(|instruction| {
            let instruction = instruction.split(' ').collect::<Vec<_>>();
            let &[_, quantity, _, origin, _, destination] = &instruction[..] else {
            return Err(format!("Move instruction without the expected 6 tokens: {instruction:?}"))
        };
            let quantity = quantity
                .parse::<usize>()
                .map_err(|err| format!("Couldn't parse quantity in move instruction: {err}"))?;
            let origin = origin
                .parse::<usize>()
                .map_err(|err| format!("Couldn't parse origin in move instruction: {err}"))?;
            let destination = destination
                .parse::<usize>()
                .map_err(|err| format!("Couldn't parse destination in move instruction: {err}"))?;

            Ok((quantity, origin, destination))
        })
        .collect::<Result<Vec<_>>>()?;

    for (quantity, origin, destination) in instructions {
        supply.move_crates(quantity, origin, destination)?;
    }

    let top_crates = supply.skim_top_crates();

    println!("{top_crates}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Result, Supply};

    #[test]
    fn supply() -> Result<()> {
        let mut supply = Supply::try_deserialize(
            r#"[A] [B] [C]
[D] [E] [F]
[G] [H] [I]
 "#,
        )?;

        assert_eq!(
            supply.0,
            vec![
                vec!['G', 'D', 'A'],
                vec!['H', 'E', 'B'],
                vec!['I', 'F', 'C']
            ]
        );

        supply.move_crates(2, 1, 3)?;

        assert_eq!(
            supply.0,
            vec![
                vec!['G'],
                vec!['H', 'E', 'B'],
                vec!['I', 'F', 'C', 'A', 'D']
            ]
        );

        let top_crates = supply.skim_top_crates();

        assert_eq!(top_crates, "GBD");

        Ok(())
    }
}
