use std::io::Read;

type Result<T, E = String> = std::result::Result<T, E>;

mod range {
    use super::{parse_string_pair, Result};

    pub struct Range(i32, i32);

    impl Range {
        pub fn try_new(start: i32, end: i32) -> Result<Self> {
            if end < start {
                return Err(format!("end ({end}) is before start ({start})"));
            }
            Ok(Self(start, end))
        }

        pub fn try_parse(string: &str) -> Result<Range> {
            let (start, end) = parse_string_pair(string, '-')?;
            let start = start.parse().map_err(|err| format!("{err}"))?;
            let end = end.parse().map_err(|err| format!("{err}"))?;
            Range::try_new(start, end)
        }

        pub fn overlaps(&self, other: &Range) -> bool {
            let start = self.0.max(other.0);
            let end = self.1.min(other.1);
            start <= end
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
        .filter(|(first, second)| first.overlaps(second))
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
        let first = Range::try_new(2, 4)?;
        let second = Range::try_new(6, 8)?;
        assert_eq!(false, first.overlaps(&second));

        let first = Range::try_new(2, 3)?;
        let second = Range::try_new(4, 5)?;
        assert_eq!(false, first.overlaps(&second));

        let first = Range::try_new(5, 7)?;
        let second = Range::try_new(7, 9)?;
        assert_eq!(true, first.overlaps(&second));

        let first = Range::try_new(2, 8)?;
        let second = Range::try_new(3, 7)?;
        assert_eq!(true, first.overlaps(&second));

        let first = Range::try_new(4, 6)?;
        let second = Range::try_new(6, 6)?;
        assert_eq!(true, first.overlaps(&second));

        let first = Range::try_new(2, 6)?;
        let second = Range::try_new(4, 8)?;
        assert_eq!(true, first.overlaps(&second));

        let first = Range::try_new(68, 79)?;
        let second = Range::try_new(42, 67)?;
        assert_eq!(false, first.overlaps(&second));

        Ok(())
    }
}
