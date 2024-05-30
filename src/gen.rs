/* FGEN
 * A file generator that reads a file with instructions and generates a new file based on those instructions.
 * Github: https://www.github.com/0x4248/FGEN
 * Licence: GNU General Public License v3.0
 * By: 0x4248
*/

use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::{self, Write};

pub fn generate_file(file: &mut File, instructions: &str, rng: &mut ThreadRng) -> io::Result<()> {
    for line in instructions.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "TEXT" => {
                let count: usize = parts[1].parse().unwrap();
                let text = parts[2..].join(" ");
                for _ in 0..count {
                    write!(file, "{}", text)?;
                }
            }
            "FILE" => {
                let count: usize = parts[1].parse().unwrap();
                let filename = parts[2];
                let file_contents = std::fs::read_to_string(filename).unwrap();
                for _ in 0..count {
                    write!(file, "{}", file_contents)?;
                }
            }
            "ESC" => match parts[1] {
                "\\n" => writeln!(file)?,
                "\\t" => write!(file, "\t")?,
                _ => {}
            },
            "HEX" => {
                let byte = u8::from_str_radix(parts[1].trim_start_matches("0x"), 16).unwrap();
                let count: usize = parts[2].parse().unwrap();
                file.write_all(&vec![byte; count])?;
            }
            "RAND" => {
                let count: usize = parts[1].parse().unwrap();
                let range: Vec<&str> = parts[2].split(',').collect();
                let min = u8::from_str_radix(range[0].trim_start_matches("0x"), 16).unwrap();
                let max = u8::from_str_radix(range[1].trim_start_matches("0x"), 16).unwrap();
                for _ in 0..count {
                    let byte = rng.gen_range(min..=max);
                    file.write_all(&[byte])?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
