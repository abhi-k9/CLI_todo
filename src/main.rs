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
}


fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    
    let todo_item = TodoItem::new("Say_hi".to_string());
    let todo_list = vec![todo_item];

    if command == "get" {
        for item in todo_list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}
