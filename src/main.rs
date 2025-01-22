use rfd::FileDialog;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

/// Main entry point of the program.
///
/// # Purpose
/// This function prompts the user to select a text file, processes the file to
/// remove ANSI escape codes and replace Unicode box-drawing characters with ASCII equivalents,
/// and saves the cleaned output to a new file.
///
/// # Returns
/// - `Ok(())` if the process completes successfully.
/// - `Err(io::Error)` if an error occurs during file operations.
fn main() -> io::Result<()> {
    // Prompt the user to select an input file
    let input_file = FileDialog::new()
        .add_filter("Text Files", &["txt"])
        .set_title("Select Input File")
        .pick_file()
        .expect("No input file selected");

    // Convert input file path to PathBuf
    let input_path: PathBuf = input_file;

    // Generate output file name by appending "_output" to the input file name
    let output_file = {
        let output_file_name = input_path
            .file_stem()
            .map(|stem| format!("{}_output.txt", stem.to_string_lossy()))
            .unwrap_or_else(|| "output_output.txt".to_string());
        input_path.with_file_name(output_file_name)
    };

    println!(
        "Processing file: {}\nOutput will be saved to: {}",
        input_path.display(),
        output_file.display()
    );

    // Open the input file
    let file = File::open(&input_path)?;
    let reader = BufReader::new(file);

    // Open the output file
    let mut output = File::create(&output_file)?;

    // Process each line: Remove ANSI escape codes and replace Unicode box-drawing characters
    for line in reader.lines() {
        let line = line?;
        let cleaned_line = clean_text(&line);
        writeln!(output, "{}", cleaned_line)?;
    }

    println!(
        "Cleaning completed. Output saved to {}",
        output_file.display()
    );
    Ok(())
}

/// Cleans text by removing ANSI escape codes and replacing Unicode box-drawing characters.
///
/// # Parameters
/// - `input`: A string slice representing a single line of text to be cleaned.
///
/// # Returns
/// - A `String` containing the cleaned text.
///
/// # Details
/// - ANSI escape codes are removed using a regex.
/// - Unicode box-drawing characters are replaced with their ASCII equivalents.
fn clean_text(input: &str) -> String {
    // Remove ANSI escape codes using a regex
    let ansi_regex = regex::Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
    let no_ansi = ansi_regex.replace_all(input, "");

    // Replace Unicode box-drawing characters with ASCII equivalents
    no_ansi
        .replace('├', "+")
        .replace('─', "-")
        .replace('│', "|")
        .replace(['└', '┌', '┐', '┘', '┬', '┴', '┼', '╭', '╮', '╯', '╰'], "+")
        .replace('╱', "/")
        .replace('╲', "\\")
        .replace('╳', "X")
        .replace('╴', "-")
        .replace('╵', "|")
        .replace('╶', "-")
        .replace('╷', "|")
        .replace('╸', "-")
        .replace('╹', "|")
        .replace('╺', "-")
        .replace('╻', "|")
        .replace('╼', "-")
        .replace('╽', "|")
        .replace('╾', "-")
        .replace('╿', "|")
        .replace('═', "=")
        .replace('║', "|")
        .replace(
            [
                '╒', '╓', '╔', '╕', '╖', '╗', '╘', '╙', '╚', '╛', '╜', '╝', '╞', '╟', '╠', '╡',
                '╢', '╣', '╤', '╥', '╦', '╧', '╨', '╩', '╪', '╫', '╬', '╭', '╮', '╯', '╰',
            ],
            "+",
        )
        .replace('╱', "/")
        .replace('╲', "\\")
        .replace('╳', "X")
        .replace('╴', "-")
        .replace('╵', "|")
        .replace('╶', "-")
        .replace('╷', "|")
        .replace('╸', "-")
        .replace('╹', "|")
        .replace('╺', "-")
        .replace('╻', "|")
        .replace('╼', "-")
        .replace('╽', "|")
        .replace('╾', "-")
        .replace('╿', "|")
        .to_string()
}
