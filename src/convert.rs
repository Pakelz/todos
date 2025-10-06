#![allow(unused)]
use crate::{time, todo::Todo};

pub fn convert_to_struct(data: Vec<String>) -> Vec<Todo> {
    let mut converts: Vec<Todo> = Vec::new();

    for i in data {
        let test = i.trim_matches(&['{', '}']);
        let mut container: Vec<&str> = test.split(",").collect();
        let container: Vec<String> = container
            .iter_mut()
            .map(|f| {
                let parts: Vec<&str> = f.splitn(2, ':').collect();
                parts[1].trim().trim_matches('"').to_string()
            })
            .collect();

        let data = Todo {
            id: container[0].parse().unwrap(),
            description: container[1].to_owned(),
            status: container[2].to_owned(),
            created_at: container[3].to_owned(),
            update_at: container[4].to_owned(),
        };

        converts.push(data);
    }
    converts
}

pub fn convert_to_json(todos: Vec<Todo>) -> String {
    let data: Vec<String> = todos
        .into_iter()
        .map(|f| {
            format!(
                "{{\"id\": {}, \"description\": \"{}\", \"status\": \"{}\", \"createdAt\": \"{}\", \"updateAt\": \"{}\"}}",
                { f.id },
                { f.description.to_owned() },
                { f.status.to_owned() },
                { f.created_at.to_owned() },
                { f.update_at.to_owned() }
            )
        })
        .collect();

    format!("[{}]", data.join(","))
}

pub fn json_to_vec_string(value: String) -> Vec<String> {
    value
        .trim_matches(&['[', ']'])
        .split("},")
        .map(|f| {
            let mut s = f.to_string();
            if !s.ends_with("}") {
                s.push('}');
            }
            s
        })
        .collect()
}
