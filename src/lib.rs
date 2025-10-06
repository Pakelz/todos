use std::{fs, path::Path};

use crate::{
    convert::{convert_to_json, convert_to_struct, json_to_vec_string},
    todo::Todo,
    view::view_todo,
};

mod convert;
mod time;
mod todo;
mod view;

pub fn add(task: &str) {
    if task == "" {
        eprintln!("You input empty task!!!");
        return;
    }

    let value: String;
    if Path::new("todos.json").exists() {
        value = fs::read_to_string("todos.json").unwrap();
    } else {
        value = String::new();
    }

    let data_array;
    if value.is_empty() {
        data_array = Vec::new();
    } else {
        data_array = json_to_vec_string(value);
    }

    let mut todos = convert_to_struct(data_array);
    if todos.iter().find(|&x| x.description == task).is_some() {
        eprintln!("Task already exists");
        return;
    }

    let task = Todo::new(todos.len(), task.to_string());
    todos.push(task);

    let data_json = convert_to_json(todos);
    fs::write("todos.json", data_json).unwrap();
}

pub fn update(id: u32, task: &str) {
    let value = match fs::read_to_string("todos.json") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    if value.is_empty() {
        eprintln!("Task is empty");
        return;
    }

    let data_array = json_to_vec_string(value);
    let mut todos = convert_to_struct(data_array);

    if id > todos.len() as u32 {
        eprintln!("Out of task number");
        return;
    }

    todos.iter_mut().for_each(|f| {
        if f.id == id {
            f.description = task.to_owned();
            f.update_at = time::get_time().take_time().to_owned();
        }
    });

    let data_json = convert_to_json(todos);
    fs::write("todos.json", data_json).unwrap();
}

pub fn delete(id: u32) {
    let value = match fs::read_to_string("todos.json") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    if value.is_empty() {
        eprintln!("Task is empty");
        return;
    }

    let data_array = json_to_vec_string(value);
    let mut todos = convert_to_struct(data_array);

    if todos.len() < id as usize {
        eprintln!("Out of tas number");
        return;
    }

    todos.retain(|f| f.id != id);
    if todos.is_empty() {
        let new = String::new();
        fs::write("todos.json", new).unwrap();
    } else {
        let mut x = 1;
        todos.iter_mut().for_each(|f| {
            f.id = x;
            x += 1;
        });
        let data_json = convert_to_json(todos);
        fs::write("todos.json", data_json).unwrap();
    }
}

pub fn mark_in_progress(id: u32) {
    let value = match fs::read_to_string("todos.json") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    if value.is_empty() {
        eprintln!("Task is empty");
        return;
    }

    let data_array = json_to_vec_string(value);
    let mut todos = convert_to_struct(data_array);

    if todos.len() < id as usize {
        eprintln!("Out of task number");
        return;
    }

    todos.iter_mut().for_each(|f| {
        if f.id == id {
            f.status = "In Progress".to_string();
            f.update_at = time::get_time().take_time();
        }
    });

    let data_json = convert_to_json(todos);
    fs::write("todos.json", data_json).unwrap();
}

pub fn mark_done(id: u32) {
    let value = match fs::read_to_string("todos.json") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    if value.is_empty() {
        eprintln!("Task is emoty");
        return;
    }

    let data_array = json_to_vec_string(value);
    let mut todos = convert_to_struct(data_array);

    if todos.len() < id as usize {
        eprintln!("Out of task number");
        return;
    }

    todos.iter_mut().for_each(|f| {
        if f.id == id {
            f.status = "Done".to_owned();
            f.update_at = time::get_time().take_time();
        }
    });

    let data_json = convert_to_json(todos);
    fs::write("todos.json", data_json).unwrap();
}

pub fn list(state: &str) {
    let value = match fs::read_to_string("todos.json") {
        Ok(x) => x,
        Err(_) => String::new(),
    };

    if value.is_empty() {
        eprintln!("Task is empty");
        return;
    }

    let data_array = json_to_vec_string(value);
    let todos = convert_to_struct(data_array);

    view_todo(todos, state);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        add("Gym");
    }

    #[test]
    fn test_update() {
        update(1, "Gym");
    }

    #[test]
    fn test_delete() {
        delete(1);
    }

    #[test]
    fn test_mark_in_progress() {
        mark_in_progress(1);
    }

    #[test]
    fn test_mark_done() {
        mark_done(1);
    }

    #[test]
    fn test_list() {
        list("");
    }
}
