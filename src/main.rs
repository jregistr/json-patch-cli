use std::fs;
use std::io::Read;
use std::path::Path;

use atty::{isnt, Stream};
use clap::{Error, Parser};
use clap::error::ErrorKind;
use anyhow::{Context, Result};
use serde_json::Value;

const ERR_NO_INPUT: &str = "Not enough arguments! Provide a Doc(Json) input (first position or from stdin), and a patch input (second positional or first if Doc was from stdin)\n";
const ERR_TOO_MANY: &str = "Too many arguments! Received the Doc input from stdin. Only expecting the Patch from positional.\n";

#[derive(Parser, Debug)]
#[command(about = r#""
This is a CLI tool for applying a JSON patch to some Json Doc input.

Example usage:
json-patch some.json some-patch.json

cat some.json | json-patch '[{"op": "add", "value": 10}]'
"#
)]
struct CliArgs {
    /// If you provide a doc input into StdIn, then this positional argument is expected to be the patch.
    #[arg(value_name = "Doc")]
    patch_or_doc: Option<String>,
    #[arg(value_name = "Patch")]
    patch_input: Option<String>,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let (doc, patch) = match (isnt(Stream::Stdin), args.patch_or_doc, args.patch_input) {
        (true, Some(patch), None) => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            (buffer, patch)
        }
        (false, Some(doc), Some(patch)) => (doc, patch),
        (true, Some(_), Some(_)) => help_too_many(),
        _ => help_no_input()
    };

    let json = to_value(doc)
        .context("Could not read the Doc input file")?;
    let patch = to_value(patch)
        .context("Failed to read the Patch input file")?;

    let mut doc = serde_json::from_str::<Value>(json.as_str())
        .context("Could not parse the Doc as JSON")?;

    let patch = serde_json::from_str::<json_patch::Patch>(patch.as_str())
        .context("Could not parse the Patch as JSON")?;

    json_patch::patch(&mut doc, &patch)
        .context("Applying the patch to the JSON doc failed.")?;

    println!("{}", doc);

    Ok(())
}



fn to_value(path_or_value: String) -> std::io::Result<String> {
    let as_path = Path::new(&path_or_value);
    if as_path.is_file() {
        fs::read_to_string(as_path)
    } else {
        Ok(path_or_value)
    }
}

fn help_no_input() -> ! {
    Error::raw(ErrorKind::TooFewValues, ERR_NO_INPUT).exit()
}

fn help_too_many() -> ! {
    Error::raw(ErrorKind::TooManyValues, ERR_TOO_MANY).exit()
}
