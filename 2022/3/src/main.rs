use std::io::Read;

type Result<T, E = String> = std::result::Result<T, E>;

fn item_priority(item: char) -> Result<u32> {
    match item {
        'a'..='z' => Ok(item as u32 - 96),
        'A'..='Z' => Ok(item as u32 - 38),
        _ => Err(format!(
            "Couldn't prioritize item. Invalid character: {item}"
        )),
    }
}

fn sort_and_dedup_chars(string: &str) -> Vec<char> {
    let mut chars = string.chars().collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    chars
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| format!("Couldn't read stdin: {err}"))?;

    let dupes = data
        .split('\n')
        .map(|rucksack| {
            let mid = (f64::from(rucksack.len() as u32) / 2.).ceil() as usize;
            let (fore, aft) = rucksack.split_at(mid);

            let fore = sort_and_dedup_chars(fore);
            let aft = sort_and_dedup_chars(aft);

            fore.into_iter()
                .find(|c| aft.contains(c))
                .ok_or_else(|| "Found a rucksack without a duplicate".to_owned())
                .map(item_priority)
        })
        .collect::<Result<Result<Vec<_>>>>()??;

    println!("{}", dupes.into_iter().sum::<u32>());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Result;

    #[test]
    fn item_priority() -> Result<()> {
        assert_eq!(16, super::item_priority('p')?);
        assert_eq!(38, super::item_priority('L')?);
        assert_eq!(42, super::item_priority('P')?);
        assert_eq!(22, super::item_priority('v')?);
        assert_eq!(20, super::item_priority('t')?);
        assert_eq!(19, super::item_priority('s')?);

        Ok(())
    }
}
