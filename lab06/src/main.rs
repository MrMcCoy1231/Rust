use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[String]);
}

struct PingCommand {}
impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, _args: &[String]) {
        println!("pong!");
    }
}

struct CountCommand {}
impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &[String]) {
        println!("counted {} args", args.len());
    }
}

struct TimesCommand {
    count: u32,
}
impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, _args: &[String]) {
        self.count += 1;
        println!("called {} times", self.count);
    }
}



struct Terminal {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: HashMap::new(),
        }
    }

    fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.get_name().to_lowercase(), cmd);
    }

    fn suggest_command(&self, input: &str) -> Option<String> {
        let input_lower = input.to_lowercase();
        let mut suggestions = Vec::new();

        for key in self.commands.keys() {
            if key.starts_with(&input_lower) {
                suggestions.push(key.as_str());
            }
        }

        if !suggestions.is_empty() {
            Some(suggestions.join(", "))
        } else {
            None
        }
    }

    fn run(&mut self) {
        let filename = "commands.txt";
        let file = File::open(filename);

        let file = match file {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to open the file '{}': {}", filename, e);
                return;
            }
        };

        let reader = io::BufReader::new(file);

        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    let parts: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
                    let cmd_name = parts[0].to_lowercase();
                    let args = &parts[1..];

                    if cmd_name == "stop" {
                        println!("Stopping execution.");
                        break;
                    }

                    match self.commands.get_mut(&cmd_name) {
                        Some(cmd) => {
                            cmd.exec(args);
                        }
                        None => {
                            match self.suggest_command(&cmd_name) {
                                Some(suggestions) => {
                                    println!("Command '{}' not found. Did you mean: {}?", cmd_name, suggestions);
                                }
                                None => {
                                    println!("Unknown command: '{}'", cmd_name);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error reading a line: {}", e);
                    
                }
            }
        }
    }
}



fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run();
}
