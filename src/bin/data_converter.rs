use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};

const INPUT_FILE_PATH: &str = "resources/words_all.txt";
const OUTPUT_FILE_PATH: &str = "resources/words_5_chars.txt";

fn main() -> Result<(), io::Error> {
    let input_file: File = File::open(INPUT_FILE_PATH).expect("Failed to open input file");
    let input_lines: io::Lines<io::BufReader<File>> = io::BufReader::new(input_file).lines();

    let output_file: File =
        File::create(OUTPUT_FILE_PATH).expect("Failed to create and open output file");
    let mut output_writer: io::BufWriter<File> = io::BufWriter::new(output_file);

    for line in input_lines.flatten() {
        if line.len() == 5 {
            output_writer
                .write_fmt(format_args!("{}\n", line))
                .expect("Failed to write bytes to output file");
        }
    }

    println!("Successfully wrote all the 5-char words to the generated output file!");
    Ok(())
}
