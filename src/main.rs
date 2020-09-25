extern crate console;

use console::Term;
use console::Style;
use console::Color;

use std::io::{self, Write};

// ---- STRUCTURE DEFINTITION ----
struct Line {
    text: String,
    is_task: bool,
    is_done: bool,
}


// ---- MAIN FUNCTION ----
fn main() {
    // ---- CONSOLE CONTROL ----
    // Get the terminal handle and clear the screen
    let term = Term::stdout();
    term.clear_screen().unwrap();

    // Style to highlight current cursor
    let green = Style::from_dotted_str("green.on_red()");

    // ---- ----
    // Variables to control logic
    let mut quit_flag = false;
    let mut curr_line = 0;

    // Vectors of
    let mut line_vec = Vec::<Line>::new();
    line_vec.push(Line {
        text: String::from("ToDo"),
        is_task: false,
        is_done: false
    });

    while quit_flag == false {
        // Clear screen and display
        term.clear_screen().unwrap();

        let mut render_line = 0;
        let total_lines = line_vec.len();

        for line in line_vec.iter() {
            let print_string;
            if line.is_task {
                print_string = format!("[{}] {}", if line.is_done { "X" } else { " " }, &line.text);
            } else {
                if render_line != 0 {
                    print_string = format!("\n# {}", &line.text);
                } else {
                    print_string = format!("# {}", &line.text);
                }
            }

            if render_line == curr_line {
                println!("{}", green.apply_to(&print_string));
            } else {
                println!("{}", &print_string);
            }
            render_line += 1;
        }
        println!("");

        // Read character and act accordingly
        let mut line = &mut line_vec[curr_line];

        match term.read_char() {
            Ok(read_char) => {
                match read_char {
                    'q' => quit_flag = true,
                    'j' => {
                        curr_line += 1;
                        if curr_line >= total_lines { curr_line = total_lines - 1; }
                        },
                    'k' => if curr_line > 0 { curr_line -= 1; },
                    ' ' => if line.is_task { line.is_done = !line.is_done; },
                    'd' => {
                        if line.is_task {
                            line_vec.remove(curr_line);
                            if curr_line == line_vec.len() { curr_line -= 1; }
                        } else {
                            print!("Delete whole section? (y/n)");
                            io::stdout().flush().unwrap();
                            match term.read_char() {
                                Ok(read_option) => {
                                    if read_option == 'y' || read_option == 'Y' {
                                        line_vec.remove(curr_line);
                                        if curr_line == line_vec.len() { curr_line -= 1; continue; }
                                        while line_vec[curr_line].is_task {
                                            line_vec.remove(curr_line);
                                            if curr_line == line_vec.len() { curr_line -= 1; break; }
                                        }
                                    }
                                },
                                Err(_e) => println!("Could not read char"),
                            }
                        }
                    }
                    'a' => match term.read_char() {
                        Ok(read_addition) => {
                            match read_addition {
                                's' => {
                                    print!("Enter new section: ");
                                    io::stdout().flush().unwrap();
                                    match term.read_line() {
                                        Ok(read_line) => {
                                            curr_line = line_vec.len();
                                            line_vec.push(Line {
                                                text: read_line,
                                                is_done: false,
                                                is_task: false,
                                            })
                                        },
                                        Err(_e) => println!("Could not read line"),
                                    }
                                },
                                't' => {
                                    print!("Enter new task: ");
                                    io::stdout().flush().unwrap();
                                    match term.read_line() {
                                        Ok(read_line) => {
                                            curr_line += 1;
                                            line_vec.insert(curr_line, Line {
                                                text: read_line,
                                                is_done: false,
                                                is_task: true,
                                            })
                                        },
                                        Err(_e) => println!("Could not read line"),
                                    }
                                },
                                _ => println!("'a{} is not a valid command'", read_addition),
                            }
                        },
                        Err(_e) => println!("Could not read character!"),
                    },
                    'm' => {
                        print!("Enter new text: ");
                        io::stdout().flush().unwrap();
                        match term.read_line() {
                            Ok(read_line) => {
                                line.text = read_line;
                                term.clear_last_lines(1).unwrap();
                            },
                            Err(_e) => println!("Could not read line"),
                        }
                    },
                    _ => println!("'{}' is not a valid command", read_char),
                }
            },
            Err(_e) => println!("Could not read char!"),
        }
    }
}


// Render all lines to the screen

