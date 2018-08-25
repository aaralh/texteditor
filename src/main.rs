use std::fs::File;
use std::io::prelude::*;

fn read_file(fileuri: String) -> String {
    let mut f = File::open(fileuri).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

fn main() {
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_read_file() {
        let fileuri = "./src/text.txt";
        assert_eq!(read_file(fileuri.to_string()), "This is text.".to_string());
    }
}
