use std::{collections::BTreeMap, io::Read, slice::Iter, str::Split};

type Result<T, E = String> = std::result::Result<T, E>;

enum Command<'a> {
    ChangeDirectory { token: &'a str },
    List,
}

impl<'a> TryFrom<Split<'a, char>> for Command<'a> {
    type Error = String;

    fn try_from(mut tokens: Split<'a, char>) -> Result<Self, Self::Error> {
        match tokens.next() {
            Some("cd") => Ok(Self::ChangeDirectory {
                token: tokens
                    .next()
                    .ok_or_else(|| r#"'cd' was not followed by a directory name."#.to_owned())?,
            }),
            Some("ls") => Ok(Self::List),
            None => Err("Empty command ($ followed by nothing)".to_owned()),
            cmd => Err(format!(
                "Unrecognised command: {}",
                cmd.expect("match covers None case")
            )),
        }
    }
}

#[derive(Debug)]
enum Item<'a> {
    Directory(Directory<'a>),
    File { size: i32 },
}

impl<'a> Item<'a> {
    fn new_directory() -> Self {
        Self::Directory(Directory::default())
    }

    fn size(&self) -> i32 {
        match self {
            Item::Directory(directory) => directory.size(),
            Item::File { size } => *size,
        }
    }
}

type NamedItem<'a> = (&'a str, Item<'a>);

enum Line<'a> {
    Command(Command<'a>),
    Item(NamedItem<'a>),
}

impl<'a> TryFrom<Split<'a, char>> for Line<'a> {
    type Error = String;
    fn try_from(mut tokens: Split<'a, char>) -> Result<Self, Self::Error> {
        match tokens.next() {
            Some("$") => Ok(Self::Command(Command::try_from(tokens)?)),
            Some("dir") => Ok(Self::Item((
                tokens
                    .next()
                    .ok_or_else(|| "'dir' was not followed by a name".to_owned())?,
                Item::new_directory(),
            ))),
            Some(probably_size) => {
                let size = probably_size.parse::<i32>().map_err(|_| {
                    format!("Couldn't parse token ({probably_size}) as a file size")
                })?;
                let name = tokens
                    .next()
                    .ok_or_else(|| "A file size was not followed by a name".to_owned())?;
                Ok(Self::Item((name, Item::File { size })))
            }
            None => Err("Tried to make a Line from an empty token iter".to_owned()),
        }
    }
}

#[derive(Default, Debug)]
struct Directory<'a> {
    items: BTreeMap<&'a str, Item<'a>>,
}

impl<'a, 'b> TryFrom<&'b mut Item<'a>> for &'b mut Directory<'a> {
    type Error = String;

    fn try_from(item: &'b mut Item<'a>) -> Result<Self, Self::Error> {
        match item {
            Item::Directory(directory) => Ok(directory),
            _ => Err("Failed to convert Item to Directory".to_owned()),
        }
    }
}

enum Context {
    Commands,
    Ls,
}

impl<'a> Directory<'a> {
    fn size(&self) -> i32 {
        self.items.iter().map(|(_, item)| item.size()).sum()
    }

    fn all_directories(&self) -> Vec<&Directory> {
        let mut directories = vec![self];

        let child_directories = self
            .items
            .iter()
            .filter_map(|(_, item)| match item {
                Item::Directory(directory) => Some(directory.all_directories()),
                _ => None,
            })
            .flatten();

        directories.extend(child_directories);

        directories
    }

    fn insert_item(&mut self, name: &'a str, item: Item<'a>) {
        self.items.insert(name, item);
    }

    fn insert_item_at_path(&mut self, item: NamedItem<'a>, mut path: Iter<&'a str>) -> Result<()> {
        match path.next() {
            Some(dir_name) => {
                let directory: &mut Directory = self
                    .items
                    .get_mut(dir_name)
                    .ok_or_else(|| format!("Path led to a non-existent directory: {dir_name}"))?
                    .try_into()?;

                directory.insert_item_at_path(item, path)?;

                Ok(())
            }
            None => {
                let (name, item) = item;
                self.insert_item(name, item);
                Ok(())
            }
        }
    }

    fn reconstruct_from_cmdline_history(history: Split<'a, char>) -> Result<Self> {
        let mut root = Self::default();
        let mut ctx = Context::Commands;
        let mut path = Vec::<&'a str>::new();

        fn handle_command<'a>(
            command: Command<'a>,
            path: &mut Vec<&'a str>,
            ctx: &mut Context,
        ) -> Result<()> {
            match command {
                Command::ChangeDirectory { token: "/" } => path.clear(),
                Command::ChangeDirectory { token: ".." } => {
                    path.pop()
                        .ok_or_else(|| "Tried to 'cd ..' in root directory".to_owned())?;
                }
                Command::ChangeDirectory { token } => {
                    path.push(token);
                }
                Command::List => *ctx = Context::Ls,
            }

            Ok(())
        }

        for line in history {
            let line = Line::try_from(line.split(' '))?;
            match ctx {
                Context::Commands => match line {
                    Line::Command(command) => {
                        handle_command(command, &mut path, &mut ctx)?;
                    }
                    Line::Item(_) => {
                        return Err("Encountered an Item outside Context::Ls".to_owned())
                    }
                },
                Context::Ls => match line {
                    Line::Command(command) => {
                        ctx = Context::Commands;
                        handle_command(command, &mut path, &mut ctx)?;
                    }
                    Line::Item(item) => root.insert_item_at_path(item, path.iter())?,
                },
            }
        }

        Ok(root)
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let mut data = String::new();

    stdin
        .read_to_string(&mut data)
        .map_err(|err| format!("Couldn't read stdin: {err}"))?;

    let fs = Directory::reconstruct_from_cmdline_history(data.split('\n'))?;

    let mut dir_sizes = fs
        .all_directories()
        .into_iter()
        .map(|dir| dir.size())
        .collect::<Vec<_>>();

    dir_sizes.sort();

    let space_used = dir_sizes
        .last()
        .ok_or("Couldn't get total_space_used. dir_sizes is empty.")?;

    let unused_space = 70_000_000 - space_used;

    let space_needed = 30_000_000 - unused_space;

    let delete_this = dir_sizes
        .into_iter()
        .find(|size| *size > space_needed)
        .ok_or_else(|| "Didn't find a size".to_owned())?;

    println!("{delete_this}");

    Ok(())
}
