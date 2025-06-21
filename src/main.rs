/// pfru - an utility program to rename files using Lua string patterns.
/// It uses the `lua-patterns` crate to do the dirty work of matching the
/// patterns (and captures) the `strfmt` crate to do the in-text substitions.
///
/// The program is used as follows:
/// - the `pattern` argument specifies the Lua string pattern to be matched in every file's filename given (not the path).
/// - the `captures` argument is used to name the [captures](https://www.lua.org/pil/20.3.html) of the pattern.
/// - the `substitution` argument is the text that will replace each filename. It can do text substition using the
///   captures we made earlier (putting brackets around them, like `format!`)
use lua_patterns::LuaPattern;
use owo_colors::OwoColorize;
use std::{collections::HashMap, path::PathBuf};
use strfmt::strfmt;

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    files: Vec<PathBuf>,

    /// Assigns a name for each capture, in the order it was matched by the pattern
    #[arg(long = "cap")]
    captures: Vec<String>,

    /// The pattern to match the filename with
    #[arg(long = "pat")]
    pattern: String,

    /// The text that's going to replace the original filename. Enclose the captures
    /// you named with brackets in order to have them substitute into the text
    #[arg(long = "sub")]
    substitution: String,

    /// Shows all of the eventual changes should the replacement be done (without actually doing it)
    #[arg(long)]
    diff: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut pattern = LuaPattern::new(&args.pattern);

    for path in args.files.iter() {
        let file_name = match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => file_name, // it's the easiest way out: not deal with OsStr's.
                None => continue,
            },
            None => continue,
        };

        let captures = pattern.captures(file_name);
        let captures_names = &args.captures;

        if captures.len() != captures_names.len() + 1 {
            // if we get here, then the pattern clearly wasn't matched correctly.
            continue;
        }

        // the `strfmt` package only works with HashMaps. Thankfully it isn't that difficult to make one
        // with what we already had.
        let map = HashMap::from_iter(
            captures_names
                .iter()
                .cloned()
                .zip(captures.iter().skip(1).copied()),
        );

        let new_file_name = strfmt(&args.substitution, &map)?;

        // Make the replacement a little bit more visual.
        println!(
            "{}\n    {}",
            new_file_name.fg_rgb::<92, 241, 192>(),
            format!("replacing {}", file_name).fg_rgb::<158, 157, 159>()
        );

        if !args.diff {
            match inquire::Confirm::new("Proceed with changes?").prompt()? {
                true => {
                    let mut new_path = path.clone();
                    new_path.set_file_name(new_file_name);

                    std::fs::rename(path, new_path)?;
                }
                false => println!("no changes were made"),
            };
        }
    }

    Ok(())
}
