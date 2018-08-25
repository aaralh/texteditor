use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn read_file(fileuri: String) -> String {
    let mut f = File::open(fileuri).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

fn save_file(fileuri: String, content: String) -> bool {

    let path = Path::new(&fileuri);

    let mut file = match File::create(&path) {
        Err(_why) => return false,
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(_why) => {
            return false;
        },
        Ok(_) => return true,
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_file() {
        let fileuri = "./src/text.txt";
        let content = "This is test text.".to_string();
        assert_eq!(save_file(fileuri.to_string(), content), true);
    }

    #[test]
    fn test_read_file() {
        let fileuri = "./src/text.txt";
        let content = "This is test text.".to_string();
        assert_eq!(read_file(fileuri.to_string()), content);
    }

}
