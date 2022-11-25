use color_eyre::{eyre::WrapErr, owo_colors::OwoColorize, Result};
use edit::{edit_file, Builder};
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const TEMPLATE: &[u8; 2] = b"# ";

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<()> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create wip file")?
        .keep()
        .wrap_err("Failed to keep tempfile")?;
    file.write_all(TEMPLATE)?;
    // let the user write whatever they want in their favorite editor
    // before returning to the cli and finishing up
    edit_file(&filepath)?;
    // Read the user's changes back from th file into a string
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut contents)?;

    // use `title` if the user passed one in,
    // otherwise try to find a heading in the markdown
    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|v| v.starts_with("# "))
            // markdown headings are required to have "# " with
            // at least one space
            .map(|maybe_line| maybe_line.trim_start_matches("# ").to_string())
    });
    let filename = match document_title {
        Some(raw_title) => confirm_filename(&raw_title),
        None => ask_for_filename(),
    }?;
    let mut i: usize = 0;
    loop {
        let dest_filename = format!(
            "{}{}",
            filename,
            if i == 0 {
                "".to_string()
            } else {
                i.to_string()
            }
        );
        let mut dest = garden_path.join(dest_filename);
        dest.set_extension("md");
        if dest.exists() {
            i = i + 1;
        } else {
            fs::rename(filepath, &dest)?;
            break;
        }
    }
    Ok(())
}

fn ask_for_filename() -> Result<String> {
    rprompt::prompt_reply_stderr(&format!(
        "{}",
        "\
Enter filename
>"
        .bold()
        .blue(),
    ))
    .wrap_err("Failed to get filename")
    .map(|title| slug::slugify(title))
}

fn confirm_filename(raw_title: &str) -> Result<String> {
    loop {
        //prompt defaults to uppercase character in question
        // this is a convention, bot a requirement enforced by the code
        let result = rprompt::prompt_reply_stderr(&format!(
            "\
{} {}
Do you want a different title? (y/N):",
            "current title:".green().bold(),
            raw_title,
        ))
        .wrap_err("Failed to get input for y/n question")?;

        match result.as_str() {
            "y" | "Y" => break ask_for_filename(),
            "n" | "N" | "" => {
                // the capital N in the prompt is the default,
                // so no input gets treated as such
                break Ok(slug::slugify(raw_title));
            }
            _ => {
                // ask again because something went wrong
            }
        };
    }
}
