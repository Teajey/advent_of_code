use std::io::Read;

#[derive(Debug)]
pub struct Failure(String);

impl Failure {
    pub fn from(value: String) -> Self {
        Self(value)
    }
}

#[macro_export]
macro_rules! e {
    ($($t:tt)*) => {
        Failure::from(format_args!($($t)*).to_string())
    };
}

pub type Result<T, E = Failure> = std::result::Result<T, E>;

pub fn get_input() -> Result<String> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| e!("Couldn't read stdin: {err}"))?;

    Ok(data)
}
