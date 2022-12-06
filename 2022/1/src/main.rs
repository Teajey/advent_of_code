use std::io::Read;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .expect("Couldn't read stdin");

    let max = data
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
                .sum::<u32>()
        })
        .max();

    println!("{max:?}");
}
