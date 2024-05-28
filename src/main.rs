/* FGEN
 * A file generator that reads a file with instructions and generates a new file based on those instructions.
 * Github: https://www.github.com/0x4248/FGEN
 * Licence: GNU General Public License v3.0
 * By: 0x4248
*/


use std::fs::File;
use std::env;

mod gen;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <instructions> <output>", args[0]);
        std::process::exit(1);
    }

    let instructions = std::fs::read_to_string(&args[1]).unwrap();
    let mut file = File::create(&args[2]).unwrap();
    let mut rng = rand::thread_rng();
    let result = gen::generate_file(&mut file, &instructions, &mut rng);
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
