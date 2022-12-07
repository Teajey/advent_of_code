use std::io::Read;

type Result<T, E = String> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| format!("Couldn't read stdin: {err}"))?;

    for (i, _) in data.chars().enumerate().skip(4) {
        let four_chars = &data[(i - 4)..i].as_bytes();
        if four_chars[0] != four_chars[1]
            && four_chars[0] != four_chars[2]
            && four_chars[0] != four_chars[3]
            && four_chars[1] != four_chars[2]
            && four_chars[1] != four_chars[3]
            && four_chars[2] != four_chars[3]
        {
            println!("{i}");
            break;
        }
    }

    Ok(())
}
