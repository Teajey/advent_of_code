use std::io::Read;

type Result<T, E = String> = std::result::Result<T, E>;

#[allow(clippy::never_loop)]
fn find_head_of_unique_string_of_length(length: usize, data: &[u8]) -> Option<usize> {
    for i in length..(data.len()) {
        'chars: {
            let chars = &data[(i - length)..i];
            for j in 0..chars.len() - 1 {
                for k in j + 1..chars.len() {
                    if chars[j] == chars[k] {
                        break 'chars;
                    }
                }
            }
            return Some(i);
        }
    }

    None
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| format!("Couldn't read stdin: {err}"))?;

    let data = data.as_bytes();

    if let Some(head) = find_head_of_unique_string_of_length(14, data) {
        println!("{head}");
    }

    Ok(())
}
