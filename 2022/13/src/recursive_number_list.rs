use std::cmp::Ordering;

use common::*;

use super::Oor;

impl PartialOrd for Oor<u8> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Oor::One(a), Oor::One(b)) => a.partial_cmp(b),
            (Oor::One(num), list) => Oor::RecursiveList(vec![Oor::One(*num)]).partial_cmp(list),
            (list, Oor::One(num)) => list.partial_cmp(&Oor::RecursiveList(vec![Oor::One(*num)])),
            (Oor::RecursiveList(a), Oor::RecursiveList(b)) => {
                for (a, b) in a.iter().zip(b) {
                    match a.partial_cmp(b) {
                        Some(Ordering::Equal) => (),
                        None => (),
                        ordering => return ordering,
                    }
                }

                a.len().partial_cmp(&b.len())
            }
        }
    }
}

fn oor_from_str(string: &str) -> Result<Oor<u8>> {
    let oor = match string
        .chars()
        .next()
        .ok_or_else(|| e!("Tried to parse an empty string as a recursive number list"))?
    {
        '[' => Oor::RecursiveList(from_str(string)?),
        _ => Oor::One(
            string
                .parse()
                .map_err(|err| e!("Couldn't parse string '{string}' as a u8: {err}"))?,
        ),
    };

    Ok(oor)
}

fn split_csv_outside_brackets(csv: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut brackets = 0;

    for (i, c) in csv.chars().enumerate() {
        match c {
            '[' => brackets += 1,
            ']' => brackets -= 1,
            ',' => {
                if brackets < 1 {
                    let slice = csv
                        .get(start..i)
                        .ok_or_else(|| e!("Couldn't slice csv at {start}..{i}"))?;
                    result.push(slice.to_owned());
                    start = i + 1;
                }
            }
            _ => (),
        }
    }

    if brackets != 0 {
        return Err(e!("Unbalanced brackets: {brackets}"));
    }

    let slice = csv
        .get(start..)
        .ok_or_else(|| e!("Couldn't slice csv at {start}.."))?;
    result.push(slice.to_owned());

    Ok(result)
}

pub(super) fn from_str(string: &str) -> Result<Vec<Oor<u8>>> {
    let inner_list_string = string
        .chars()
        .enumerate()
        .filter_map(|(i, ch)| {
            if (i == 0 && ch == '[') || (i == string.len() - 1 && ch == ']') {
                None
            } else {
                Some(ch)
            }
        })
        .collect::<String>();

    if inner_list_string.is_empty() {
        return Ok(vec![]);
    }

    split_csv_outside_brackets(&inner_list_string)?
        .into_iter()
        .map(|s| oor_from_str(&s))
        .collect()
}

#[cfg(test)]
mod test {
    use common::Result;

    use super::Oor;

    #[test]
    fn split_csv_outside_brackets() -> Result<()> {
        let example = "1,[2,[3,[4,[5,6,0]]]],8,9";
        let strings = super::split_csv_outside_brackets(example)?;

        assert_eq!(&["1", "[2,[3,[4,[5,6,0]]]]", "8", "9"], strings.as_slice());

        let example = "1,]2,]3,]4,]5,6,0[[[[,8,9";
        let strings = super::split_csv_outside_brackets(example)?;

        assert_eq!(
            &["1", "]2", "]3", "]4", "]5", "6", "0[[[[", "8", "9"],
            strings.as_slice()
        );

        let example = "2,[3,[4,[5,6,0]]]";
        let strings = super::split_csv_outside_brackets(example)?;

        assert_eq!(&["2", "[3,[4,[5,6,0]]]"], strings.as_slice());

        let example = "[]";
        let strings = super::split_csv_outside_brackets(example)?;

        assert_eq!(&["[]"], strings.as_slice());

        Ok(())
    }

    #[test]
    fn from_str() -> Result<()> {
        let example = "[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let oor = super::from_str(example)?;

        assert_eq!(
            &[
                Oor::One(1),
                Oor::RecursiveList(vec![
                    Oor::One(2),
                    Oor::RecursiveList(vec![
                        Oor::One(3),
                        Oor::RecursiveList(vec![
                            Oor::One(4),
                            Oor::RecursiveList(vec![Oor::One(5), Oor::One(6), Oor::One(0)])
                        ])
                    ])
                ]),
                Oor::One(8),
                Oor::One(9)
            ],
            oor.as_slice()
        );

        Ok(())
    }

    #[test]
    fn partial_cmp() -> Result<()> {
        let a = super::from_str("[1,1,3,1,1]")?;
        let b = super::from_str("[1,1,5,1,1]")?;

        assert_eq!(Some(std::cmp::Ordering::Less), a.partial_cmp(&b));

        let a = super::from_str("[[1],[2,3,4]]")?;
        let b = super::from_str("[[1],4]")?;

        assert_eq!(Some(std::cmp::Ordering::Less), a.partial_cmp(&b));

        let a = super::from_str("[9]")?;
        let b = super::from_str("[[8,7,6]]")?;

        assert_eq!(Some(std::cmp::Ordering::Greater), a.partial_cmp(&b));

        let a = super::from_str("[[4,4],4,4]")?;
        let b = super::from_str("[[4,4],4,4,4]")?;

        assert_eq!(Some(std::cmp::Ordering::Less), a.partial_cmp(&b));

        let a = super::from_str("[7,7,7,7]")?;
        let b = super::from_str("[7,7,7]")?;

        assert_eq!(Some(std::cmp::Ordering::Greater), a.partial_cmp(&b));

        let a = super::from_str("[[[]]]")?;
        let b = super::from_str("[[]]")?;

        assert_eq!(Some(std::cmp::Ordering::Greater), a.partial_cmp(&b));

        let a = super::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]")?;
        let b = super::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]")?;

        assert_eq!(Some(std::cmp::Ordering::Greater), a.partial_cmp(&b));

        Ok(())
    }
}
