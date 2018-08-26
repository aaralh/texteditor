extern crate termion;

use termion::*;
use termion::cursor::{self, DetectCursorPos};
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;
use std::error::Error;
use termion::event::Key;
use termion::input::TermRead;


fn read_file(fileuri: &String) -> String {

    let path = Path::new(fileuri);

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
fn editor_mode(content: String, fileuri: &String) {

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
            Key::Char('q')   => break,
            Key::Char('\n') => print!("\n\r"),
            Key::Char(c)   => print!("{}", c),
            Key::Alt(c)    => println!("Alt-{}", c),
            Key::Ctrl(c)   => println!("Ctrl-{}", c),
            Key::Left      => print!("{}", cursor::Left(1)),
            Key::Right     => print!("{}", cursor::Right(1)),
            Key::Up        => print!("{}", cursor::Up(1)),
            Key::Down      => print!("{}", cursor::Down(1)),
            Key::Esc       => god_mode(&mut stdout),
            _              => println!("Other"),
        }
        // Flush again.
        stdout.flush().unwrap();
    }
    // Move cursor to begin of next line.
    write!(stdout, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
}


// God mode is mode to change editor settings, save file and exit editor.
fn god_mode(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {

    let terminal_size = termion::terminal_size().ok();
    let size = terminal_size.unwrap();

    let cursor_position = stdout.cursor_pos().unwrap();

     write!(stdout,
            "{}:",
            cursor::Goto(1, size.1)
            ).unwrap();
    // Flush contents to screen.
    stdout.flush().unwrap();
    
    let stdin = stdin();
    for c in stdin.keys() {
        // Print the key we type...
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Char(c)   => print!("{}", c),
            Key::Left      => print!("{}", cursor::Left(1)),
            Key::Right     => print!("{}", cursor::Right(1)),
            Key::Up        => println!(""),
            Key::Down      => println!(""),
            Key::Esc       => break,
            _              => println!(""),
        }
        // Flush again.
        stdout.flush().unwrap();
    }

     write!(stdout,
            "{}",
            clear::CurrentLine
            ).unwrap();
    stdout.flush().unwrap();

    // Move cursor back where it was before was god mode activated.
    write!(stdout,
            "{}",
            cursor::Goto(cursor_position.0, cursor_position.1)
            ).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Clearing screen
    println!("{}", clear::All);

    // Check if file is passed via args.
    if args.len() == 2 {
        let fileuri = &env::args().nth(1).expect("Missing argument");
        let content = read_file(fileuri);
        editor_mode(content, fileuri);
    } else {
        editor_mode("".to_string(), &"".to_string());
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
