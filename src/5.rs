use std::io;

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    // Yeni bir MinStack oluşturur
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
        let min_val = self.min_stack.last().cloned().unwrap_or(i32::MAX);
        self.min_stack.push(min_val.min(value));
    }

    fn pop(&mut self) {
        if self.stack.pop().is_some() {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> Option<i32> {
        self.stack.last().cloned()
    }

    fn min(&self) -> Option<i32> {
        self.min_stack.last().cloned()
    }
}

fn main() {
    let mut min_stack = MinStack::new();
    let mut input = String::new();

    loop {
        println!("Choose an option:");
        println!("1: Push element");
        println!("2: Pop element");
        println!("3: Get top element");
        println!("4: Get minimum element");
        println!("5: Exit");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                input.clear();
                println!("Enter the value to push:");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let value: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a valid integer.");
                        continue;
                    }
                };
                min_stack.push(value);
            },
            2 => {
                min_stack.pop();
                println!("Element popped.");
            },
            3 => {
                match min_stack.top() {
                    Some(top) => println!("Top element: {}", top),
                    None => println!("Stack is empty."),
                }
            },
            4 => {
                match min_stack.min() {
                    Some(min_val) => println!("Minimum element: {}", min_val),
                    None => println!("Stack is empty."),
                }
            },
            5 => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid choice, please enter a number between 1 and 5."),
        }
    }
}