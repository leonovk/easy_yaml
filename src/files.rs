use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn yaml_data<T: AsRef<Path>>(path: T) -> BufReader<File> {
    let file = File::open(path).expect("Failed to open file");
    BufReader::new(file)
}
