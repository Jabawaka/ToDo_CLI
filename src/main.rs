extern crate console;

use console::Term;
use console::Style;
use console::Color;

use std::io::{self, Write, BufWriter};
use std::fs;

// ---- STRUCTURE DEFINTITION ----
struct Line {
    text: String,
    is_task: bool,
    is_done: bool,
}


// ---- MAIN FUNCTION ----
fn main() {
    // ---- SOME DECLARATION ----
    // Get the terminal handle
    let term = Term::stdout();

    // Style to apply to output
    let highlight = Style::from_dotted_str("white.on_green.bold");
    let normal = Style::from_dotted_str("white.on_black");

    // Variables to control logic
    let mut quit_flag = false;
    let mut curr_line = 0;

    // Load from provided file or fill with default
    let mut line_vec = Vec::<Line>::new();
    line_vec.push(Line {
        text: String::from("ToDo"),
        is_task: false,
        is_done: false
    });

    // ---- ENDLESS LOOP ----
    while quit_flag == false {
        
        let total_lines = line_vec.len();

        // Clear screen and display
        term.clear_screen().unwrap();

        let mut render_line = 0;
        for line in line_vec.iter() {
            let mut print_string;
            if line.is_task {
                print_string = format!("[{}] {}", if line.is_done { "X" } else { " " }, &line.text);
            } else {
                if render_line != 0 {
                    print_string = format!("\n# {}", &line.text);
                } else {
                    print_string = format!("# {}", &line.text);
                }
            }
    
            while print_string.chars().count() < term.size().1.into() {
                print_string.push(' ');
            }
    
            if render_line == curr_line {
                println!("{}", highlight.apply_to(&print_string));
            } else {
                println!("{}", normal.apply_to(&print_string));
            }
            render_line += 1;
        }
        println!("");

        // Read character and act accordingly
        let mut line = &mut line_vec[curr_line];

        let read_char = term.read_char().unwrap();

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
                line.text = term.read_line().unwrap();
            },
            's' => {
                print!("Enter file to save to: ");
                io::stdout().flush().unwrap();
                let path_to_file = term.read_line().unwrap();
                let save_file = fs::File::create(path_to_file).expect("Unable to create file");
                let mut save_file = BufWriter::new(save_file);

                let mut render_line = 0;
                for line in line_vec.iter() {
                    let mut print_string;

                    if line.is_task {
                        print_string = format!("[{}] {}\n", if line.is_done { "X" } else { " " }, &line.text);
                    } else {
                        if render_line != 0 {
                            print_string = format!("\n# {}\n", &line.text);
                        } else {
                            print_string = format!("# {}\n", &line.text);
                        }
                    }
                    render_line += 1;

                    save_file.write(&print_string.as_bytes());
                }
            }
            _ => println!("'{}' is not a valid command", read_char),
        }
    }
}
