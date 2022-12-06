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

fn chunk_as_threes<T>(data: &Vec<T>) -> Vec<(&T, &T, &T)> {
    let mut chunks = Vec::<(&T, &T, &T)>::new();

    for i in 0..(data.len() / 3) {
        let slice_start = i * 3;
        let chunk = (
            &data[slice_start],
            &data[slice_start + 1],
            &data[slice_start + 2],
        );
        chunks.push(chunk);
    }

    chunks
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| format!("Couldn't read stdin: {err}"))?;

    let data = data.split('\n').collect::<Vec<_>>();

    let priorities = chunk_as_threes(&data)
        .into_iter()
        .map(|chunk| {
            let first = sort_and_dedup_chars(chunk.0);
            let second = sort_and_dedup_chars(chunk.1);
            let third = sort_and_dedup_chars(chunk.2);

            first
                .into_iter()
                .find(|c| second.contains(c) && third.contains(c))
                .ok_or_else(|| "Found a group without a common type".to_owned())
                .map(item_priority)
        })
        .collect::<Result<Result<Vec<_>>>>()??;

    println!("{}", priorities.into_iter().sum::<u32>());

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

    #[test]
    fn chop_into_threes() {
        let list = vec![1, 2, 3, 4, 5, 6];

        let chunks = super::chunk_as_threes(&list);

        assert_eq!(chunks, vec![(&1, &2, &3), (&4, &5, &6)]);

        let list = vec![1, 2, 3, 4, 5];

        let chunks = super::chunk_as_threes(&list);

        assert_eq!(chunks, vec![(&1, &2, &3)]);
    }
}
