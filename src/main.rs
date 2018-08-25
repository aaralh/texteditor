extern crate termion;

// Import the color module.
use termion::{color, clear, cursor};
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;
use std::error::Error;
use termion::event::Key;
use termion::input::TermRead;


fn read_file(fileuri: String) -> String {

    let path = Path::new(&fileuri);

    // Try open file. If no file found return empty string.
    let mut f = match File::open(path) {
        Err(_why) => return String::new(),
        Ok(f) => f,
    };

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

// Editor mode handler.
fn editor_mode(content: String) {

    // Init raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(stdout,
            "{}{}",
            cursor::Goto(1, 1),
            content).unwrap();
    // Flush contents to screen.
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Print the key we type...
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('\n') => print!("\n\r"),
            Key::Char(c)   => print!("{}", c),
            Key::Alt(c)    => println!("Alt-{}", c),
            Key::Ctrl(c)   => println!("Ctrl-{}", c),
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => println!("<up>"),
            Key::Down      => println!("<down>"),
            _              => println!("Other"),
        }
        // Flush again.
        stdout.flush().unwrap();
    }
    // Move cursor to begin of next line.
    write!(stdout, "\n\r").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Clearing screen
    println!("{}", clear::All);

    // Check if file is passed via args.
    if args.len() == 2 {
        let fileuri = env::args().nth(1).expect("Missing argument");;
        let content = read_file(fileuri);
        editor_mode(content);
    } else {
        editor_mode("".to_string())
    }
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
