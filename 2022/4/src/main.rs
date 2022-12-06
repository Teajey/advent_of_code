use std::io::Read;

type Result<T, E = String> = std::result::Result<T, E>;

mod range {
    use super::{parse_string_pair, Result};

    pub struct Range(u32, u32);

    impl Range {
        pub fn try_new(start: u32, end: u32) -> Result<Self> {
            if end < start {
                return Err(format!("end ({end}) is before start ({start})"));
            }
            Ok(Self(start, end))
        }

        pub fn try_parse(string: &str) -> Result<Range> {
            let (start, end) = parse_string_pair(string, '-')?;
            let start = start.parse::<u32>().map_err(|err| format!("{err}"))?;
            let end = end.parse::<u32>().map_err(|err| format!("{err}"))?;
            Range::try_new(start, end)
        }

        pub fn contains(&self, other: &Range) -> bool {
            self.0 <= other.0 && other.1 <= self.1
        }
    }
}

use range::Range;

fn parse_string_pair(string: &str, separator: char) -> Result<(&str, &str)> {
    let strings = string.split(separator).collect::<Vec<_>>();
    let &[first, second] = &strings[..] else {
        return Err(format!("Could not use separator ({separator}) to split this string into two values: {string:?}"));
    };
    Ok((first, second))
}

fn parse_elf_assignment_pair(string: &str) -> Result<(Range, Range)> {
    let string_pair = parse_string_pair(string, ',')?;
    let first = Range::try_parse(string_pair.0)?;
    let second = Range::try_parse(string_pair.1)?;
    Ok((first, second))
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| format!("Couldn't read stdin: {err}"))?;

    let reconsiderable_assignments = data
        .split('\n')
        .map(parse_elf_assignment_pair)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|(first, second)| first.contains(second) || second.contains(first))
        .count();

    println!("{reconsiderable_assignments}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{Range, Result};

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn range_overlap() -> Result<()> {
        let outer = Range::try_new(2, 4)?;
        let inner = Range::try_new(6, 8)?;
        assert_eq!(false, outer.contains(&inner));

        let outer = Range::try_new(2, 3)?;
        let inner = Range::try_new(4, 5)?;
        assert_eq!(false, outer.contains(&inner));

        let outer = Range::try_new(5, 7)?;
        let inner = Range::try_new(7, 9)?;
        assert_eq!(false, outer.contains(&inner));

        let outer = Range::try_new(2, 8)?;
        let inner = Range::try_new(3, 7)?;
        assert_eq!(true, outer.contains(&inner));

        let outer = Range::try_new(4, 6)?;
        let inner = Range::try_new(6, 6)?;
        assert_eq!(true, outer.contains(&inner));

        let outer = Range::try_new(2, 6)?;
        let inner = Range::try_new(4, 8)?;
        assert_eq!(false, outer.contains(&inner));

        Ok(())
    }
}
