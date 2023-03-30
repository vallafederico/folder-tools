mod utils;

use std::env;
use std::io::{self, Write};

use std::time::Instant;
use utils::list_files;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        writeln!(
            io::stderr(),
            "Usage: {} <folder_path>",
            args.get(0).unwrap_or(&String::from("list_files"))
        )
        .unwrap();
        std::process::exit(1);
    }

    let folder_path = &args[1];
    let start_time = Instant::now();
    list_files(folder_path);
    let elapsed = start_time.elapsed();
    println!("Finished in {:.2?}", elapsed);
}
