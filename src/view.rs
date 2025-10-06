use crate::todo::Todo;
pub fn view_todo(todos: Vec<Todo>, state: &str) {
    let mut max_len_description: usize = 0;
    let mut max_len_status = 0;
    let mut max_len_created_at = 0;
    for i in &todos {
        if i.description.len() > max_len_description {
            max_len_description = i.description.len();
        }
        if i.status.len() > max_len_status {
            max_len_status = i.status.len();
        }
        if i.created_at.len() > max_len_created_at {
            max_len_created_at = i.created_at.len();
        }
    }

    let header = format!(
        "ID | Description{} | Status{} | Created At{} | Update At{}",
        " ".repeat(max_len_description),
        " ".repeat(max_len_status - 6),
        " ".repeat(max_len_created_at - 10),
        " ".repeat(max_len_created_at)
    );

    let border_top = format!(
        "{}+{}+{}+{}+{}",
        "-".repeat(3),
        "-".repeat(max_len_description + 13),
        "-".repeat(max_len_status + 2),
        "-".repeat(max_len_created_at + 2),
        "-".repeat(max_len_created_at + 2)
    );

    println!("{header}");
    println!("{border_top}");

    for i in &todos {
        if state.to_lowercase() == i.status.to_lowercase() {
            let row = format!(
                "{}  | {}{} | {}{} | {} | {}",
                i.id,
                i.description,
                " ".repeat(max_len_description - i.description.len() + 11),
                i.status,
                " ".repeat(max_len_status - i.status.len()),
                i.created_at,
                i.update_at
            );
            println!("{row}");
        } else if state == "" {
            let row = format!(
                "{}  | {}{} | {}{} | {} | {}",
                i.id,
                i.description,
                " ".repeat(max_len_description - i.description.len() + 11),
                i.status,
                " ".repeat(max_len_status - i.status.len()),
                i.created_at,
                i.update_at
            );
            println!("{row}");
        }
    }
}
