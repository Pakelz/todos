use todos::{add, delete, list, mark_done, mark_in_progress, update};

mod time;
mod todo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    if command == "add" {
        if args.len() != 3 {
            eprintln!("You must enter a task\nExample: todos add \"Play Dota\"");
            return;
        }
        add(&args[2]);
    } else if command == "update" {
        if args.len() != 4 {
            eprintln!("Please input the correct command\nExample: todos update 1 Swimming");
            return;
        }

        let num = args[2].clone().parse();
        match num {
            Ok(x) => update(x, &args[3]),
            Err(_) => eprintln!("You must enter a valid number"),
        }
    } else if command == "delete" {
        if args.len() != 3 {
            eprintln!("Please input the correct command\nExample: todos delete 1");
            return;
        }

        match args[2].clone().parse() {
            Ok(x) => delete(x),
            Err(_) => eprintln!("You must enter a valid number"),
        }
    } else if command == "mark-in-progress" {
        if args.len() != 3 {
            eprintln!("Please input the correct command\nExample: todos mark-in-progress 1");
            return;
        }

        match args[2].clone().parse() {
            Ok(x) => mark_in_progress(x),
            Err(_) => eprintln!("You must enter a valid number"),
        }
    } else if command == "mark-done" {
        if args.len() != 3 {
            eprintln!("Please input correct command\nExample: todos mark-done 1");
            return;
        }

        match args[2].clone().parse() {
            Ok(x) => mark_done(x),
            Err(_) => eprintln!("You must enter a valid number"),
        }
    } else if command == "list" {
        if args.len() < 2 || args.len() > 3 {
            eprintln!(
                "Please input correct command\nExample: todos list or todos list done or todos list in-progress"
            );
            return;
        }

        if args.len() == 3 {
            let state = args[2].clone();
            list(&state);
        } else if args.len() == 2 {
            list("");
        }
    } else {
        eprintln!("Please input valid command");
        return;
    }
}
