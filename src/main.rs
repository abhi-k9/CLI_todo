use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name,
            completed: ' '
        };
    }
}

struct TodoList{
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList{list: Vec::new()}
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for (position, item) in self.list.iter().enumerate() {
            println!("({}) [{}] - {}", position, item.completed, item.name);
        }
    }

    fn toggle(& mut self, position: usize) {
        if self.list[position].completed == ' ' {
            self.list[position].completed = 'x';
        }
    }

    fn remove_task(& mut self, position: usize) {
        self.list.remove(position);
    }
}

enum Command {
    Get,
    Add(String),
    Toggle(usize),
    Remove(usize),
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].to_string()),
        "toggle" => Command::Toggle(arguments[2].parse().expect("Error converting to integer")),
        "remove" => Command::Remove(arguments[2].parse().expect("Error converting to integer")),
        _ => panic!("Must provide a proper command: `get`, `add`")
    }; 

    let mut todo_list = TodoList::new();

    todo_list.add_to_list("Say hi!".to_string());

    match command {
        Command::Add(task) => {
            todo_list.add_to_list(task)
        },
        Command::Get => {
            todo_list.print();
        },
        Command::Toggle(position) => {
            todo_list.toggle(position);
        },
        Command::Remove(position) => {
            todo_list.remove_task(position)
        },
    }

}
