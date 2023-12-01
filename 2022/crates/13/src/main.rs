mod recursive_number_list;

use common::*;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Oor<T> {
    One(T),
    RecursiveList(Vec<Oor<T>>),
}

fn main() -> Result<()> {
    let input = get_input()?;
    let mut packets = input
        .split("\n\n")
        .map(|section| {
            section
                .split('\n')
                .map(recursive_number_list::from_str)
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .map(Oor::RecursiveList)
        .collect::<Vec<_>>();

    let divider_packets = (
        recursive_number_list::oor_from_str("[[2]]")?,
        recursive_number_list::oor_from_str("[[6]]")?,
    );

    packets.push(divider_packets.0.clone());
    packets.push(divider_packets.1.clone());

    packets.sort();

    let divider_packets = (
        packets
            .iter()
            .position(|oor| *oor == divider_packets.0)
            .expect("divider 1 must be in `packets`")
            + 1,
        packets
            .iter()
            .position(|oor| *oor == divider_packets.1)
            .expect("divider 2 must be in `packets`")
            + 1,
    );

    println!("{}", divider_packets.0 * divider_packets.1);

    Ok(())
}
