use std::io::Read;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .expect("Couldn't read stdin");

    let mut all_total_calories: Vec<u32> = data
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split('\n')
                .filter_map(|cal| {
                    if cal.is_empty() {
                        None
                    } else {
                        Some(
                            cal.parse::<u32>()
                                .unwrap_or_else(|_| panic!("Failed to parse as u32: {cal}")),
                        )
                    }
                })
                .sum()
        })
        .collect();

    all_total_calories.sort();

    let top_3_calories_total = all_total_calories.into_iter().rev().take(3).sum::<u32>();

    println!("{top_3_calories_total:?}");
}
