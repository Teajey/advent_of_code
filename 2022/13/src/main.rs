mod recursive_number_list;

use std::cmp::Ordering;

use common::*;

#[derive(PartialEq, Debug, Clone)]
enum Oor<T> {
    One(T),
    RecursiveList(Vec<Oor<T>>),
}

fn main() -> Result<()> {
    let input = get_input()?;
    let packet_pairs = input
        .split("\n\n")
        .map(|section| {
            let packets = section
                .split('\n')
                .map(recursive_number_list::from_str)
                .collect::<Result<Vec<_>>>()?;

            let [packet1, packet2] = packets.as_slice() else {
                return Err(e!("Expected a pair of packets"));
            };

            Ok((packet1.to_vec(), packet2.to_vec()))
        })
        .collect::<Result<Vec<_>>>()?;

    let sum: usize = packet_pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, (a, b))| {
            if let Some(Ordering::Less) = Oor::RecursiveList(a).partial_cmp(&Oor::RecursiveList(b))
            {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();

    println!("{sum}");

    Ok(())
}
