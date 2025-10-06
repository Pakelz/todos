# 📝 Task Tracker CLI

A simple **Command Line Task Tracker** written in **Rust**, built from scratch **without using external libraries** such as `serde_json` or `clap`.  
This project is based on the [roadmap.sh CLI task tracker challenge](https://roadmap.sh/projects/task-tracker).

---

## 📦 Features

- Add new tasks  
- Update task description  
- Delete tasks  
- Mark tasks as **In Progress** or **Done**  
- List tasks filtered by status (All / Done / In Progress / Todo)  
- Store and read data manually from a JSON file (`todos.json`)  
- Display formatted output in a clean table-like view  
- Includes timestamps for created and updated time

---

## ⚙️ How It Works

This project manually handles JSON-like data:
- Tasks are stored in a plain `todos.json` file.

---

## 🧩 Installation

Clone the repository and build the project:
```bash
git clone https://github.com/Pakelz/todos.git
cd todos
cargo build
```

After building, the binary will be available in:
```bash
target/debug/todos
```

You can run the CLI directly from there or add it to your PATH.

---

## 🚀 Usage
# Adding a new task
todos add "Buy groceries"
# Output: Task added successfully (ID: 1)

# Updating and deleting tasks
todos update 1 "Buy groceries and cook dinner"
todos delete 1

# Marking a task as in progress or done
todos mark-in-progress 1
todos mark-done 1

# Listing all tasks
todos list

# Listing tasks by status
todos list done
todos list todo
todos list in-progress

---

## 📄 License

This project is open-source and available under the MIT License.


