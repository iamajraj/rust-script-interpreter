use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Interpreter {
    registers: HashMap<char, i32>,
    memory: Vec<i32>,
    strings: HashMap<String, String>,
    labels: HashMap<String, usize>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            registers: HashMap::new(),
            memory: vec![0; 100], // 100 cells of memory
            strings: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    fn execute(&mut self, instructions: &str) {
        let mut program_counter = 0;
        let tokens: Vec<&str> = instructions.split_whitespace().collect();

        while program_counter < tokens.len() {
            let token = tokens[program_counter];

            match token {
                "INIT" => {
                    let var = tokens[program_counter + 1];
                    let value = tokens[program_counter + 2];
                    if value.starts_with('"') && value.ends_with('"') {
                        // Initialize a string variable
                        let stripped_value = value.trim_matches('"');
                        self.strings.insert(var.to_string(), stripped_value.to_string());
                    } else if value.contains('.') {
                        // Initialize a float variable
                        let float_value = value.parse::<f64>().unwrap();
                        self.registers.insert(var.chars().next().unwrap(), float_value as i32);
                    } else {
                        // Initialize an integer variable
                        let int_value = value.parse::<i32>().unwrap();
                        self.registers.insert(var.chars().next().unwrap(), int_value);
                    }
                    program_counter += 3;
                }
                "OUT" => {
                    let value = tokens[program_counter + 1];
                    if let Some(reg_val) = self.registers.get(&value.chars().next().unwrap()) {
                        // Print the value of a variable (integer or float)
                        println!("{}", reg_val);
                    } else if let Some(str_val) = self.strings.get(value) {
                        // Print the value of a string variable
                        println!("{}", str_val);
                    } else if value.contains('.') {
                        // Print a float
                        let float_value = value.parse::<f64>().unwrap();
                        println!("{}", float_value);
                    } else {
                        // Print a value directly
                        println!("{}", value);
                    }
                    program_counter += 2;
                }
                "STR" => {
                    let name = tokens[program_counter + 1];
                    let content_start = program_counter + 2;
                    let content_end = content_start + tokens[content_start..].iter().position(|&x| x == "OUTSTR").unwrap_or(tokens.len() - content_start);
                    let content = tokens[content_start..content_end].join(" ");
                    let content_with_newline = content.replace("\\n", "\n");
                    self.strings.insert(name.to_string(), content_with_newline);
                    program_counter += content_end - program_counter;
                }
                "OUTSTR" => {
                    let name = tokens[program_counter + 1];
                    if let Some(content) = self.strings.get(name) {
                        print!("{}", content);
                    }
                    program_counter += 2;
                }
                "LABEL" => {
                    let label_name = tokens[program_counter + 1];
                    self.labels.insert(label_name.to_string(), program_counter);
                    program_counter += 2;
                }
                "GOTO" => {
                        let label_name = tokens[program_counter + 1];
                        if let Some(&target_line) = self.labels.get(&label_name.to_string()) {
                            program_counter = target_line;
                        } else {
                            program_counter += 2;
                        }
                   
                }
                "EXIT" => {
                    break;
                }
                "INC" => {
                    let var = tokens[program_counter + 1];
                    let value = tokens[program_counter + 2];
                    if let Some(reg_val) = self.registers.get_mut(&var.chars().next().unwrap()) {
                        if let Ok(int_value) = value.parse::<i32>() {
                            *reg_val += int_value;
                        } else if let Some(str_val) = self.strings.get(value) {
                            let current_value = reg_val.to_string();
                            *reg_val = format!("{}{}", current_value, str_val).parse::<i32>().unwrap_or(*reg_val);
                        }
                    }
                    program_counter += 3;
                }
                _ => {
                    program_counter += 1;
                }
            }
        }
    }

    fn execute_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut instructions = String::new();
        for line in reader.lines() {
            instructions.push_str(&line?);
            instructions.push(' ');
        }

        self.execute(&instructions);

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut interpreter = Interpreter::new();

    interpreter.execute_from_file("instructions.txt")?;

    Ok(())
}

