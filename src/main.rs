use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use cfg_if::cfg_if;

fn main() {
    launch(app);
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Filter {
    All,
    Active,
    Completed,
}

const STORAGE_FILE: &str = "todos.json";

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use web_sys::window;

        fn load_todos() -> (Vec<TodoItem>, u32) {
            let window = window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            if let Ok(Some(content)) = storage.get_item(STORAGE_FILE) {
                if let Ok(todos) = serde_json::from_str::<Vec<TodoItem>>(&content) {
                    let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                    return (todos, next_id);
                }
            }
            (Vec::new(), 0)
        }

        fn save_todos(todos: &[TodoItem]) {
            let window = window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            if let Ok(content) = serde_json::to_string(todos) {
                let _ = storage.set_item(STORAGE_FILE, &content);
            }
        }
    } else {
        use std::fs;

        fn load_todos() -> (Vec<TodoItem>, u32) {
            if let Ok(content) = fs::read_to_string(STORAGE_FILE) {
                if let Ok(todos) = serde_json::from_str::<Vec<TodoItem>>(&content) {
                    let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                    return (todos, next_id);
                }
            }
            (Vec::new(), 0)
        }

        fn save_todos(todos: &[TodoItem]) {
            if let Ok(content) = serde_json::to_string_pretty(todos) {
                let _ = fs::write(STORAGE_FILE, content);
            }
        }
    }
}

fn app() -> Element {
    let initial_data = load_todos();
    let mut todos = use_signal(|| initial_data.0);
    let mut next_id = use_signal(|| initial_data.1);
    let mut input_text = use_signal(|| String::new());
    let mut filter = use_signal(|| Filter::All);
    let mut editing_id = use_signal(|| None::<u32>);
    let mut edit_text = use_signal(|| String::new());

    let mut add_todo = move || {
        if !input_text.read().trim().is_empty() {
            let id = *next_id.read();
            todos.write().push(TodoItem {
                id,
                text: input_text.read().clone(),
                completed: false,
            });
            next_id.set(id + 1);
            input_text.set(String::new());
            save_todos(&todos.read());
        }
    };

    let mut toggle_todo = move |id: u32| {
        let mut list = todos.write();
        if let Some(todo) = list.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
        save_todos(&list);
    };

    let mut remove_todo = move |id: u32| {
        let mut list = todos.write();
        list.retain(|t| t.id != id);
        save_todos(&list);
    };

    let mut start_editing = move |id: u32, text: String| {
        editing_id.set(Some(id));
        edit_text.set(text);
    };

    let mut save_edit = move |id: u32| {
        let mut list = todos.write();
        if let Some(todo) = list.iter_mut().find(|t| t.id == id) {
            todo.text = edit_text.read().clone();
        }
        editing_id.set(None);
        save_todos(&list);
    };

    let filter_val = *filter.read();

    rsx! {
        style { {include_str!("../style.css")} }
        div { class: "container",
            div { class: "todo-card",
                h1 { "✨ Tasks" }
                
                div { class: "input-group",
                    input {
                        placeholder: "What needs to be done?",
                        value: "{input_text}",
                        oninput: move |evt| input_text.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                add_todo();
                            }
                        }
                    }
                    button { onclick: move |_| add_todo(), "Add" }
                }

                div { class: "filter-group",
                    button { 
                        class: if *filter.read() == Filter::All { "active" } else { "" },
                        onclick: move |_| filter.set(Filter::All), 
                        "All" 
                    }
                    button { 
                        class: if *filter.read() == Filter::Active { "active" } else { "" },
                        onclick: move |_| filter.set(Filter::Active), 
                        "Active" 
                    }
                    button { 
                        class: if *filter.read() == Filter::Completed { "active" } else { "" },
                        onclick: move |_| filter.set(Filter::Completed), 
                        "Completed" 
                    }
                }

                ul { class: "todo-list",
                    for todo in todos.read().iter().filter(|t| match filter_val {
                        Filter::All => true,
                        Filter::Active => !t.completed,
                        Filter::Completed => t.completed,
                    }).cloned() {
                        li { key: "{todo.id}", class: if todo.completed { "completed" } else { "" },
                            div { class: "todo-content",
                                input {
                                    type: "checkbox",
                                    checked: todo.completed,
                                    onchange: move |_| toggle_todo(todo.id)
                                }
                                if editing_id.read().is_some_and(|id| id == todo.id) {
                                    input {
                                        class: "edit-input",
                                        value: "{edit_text}",
                                        oninput: move |evt| edit_text.set(evt.value()),
                                        onkeydown: move |evt| {
                                            if evt.key() == Key::Enter {
                                                save_edit(todo.id);
                                            } else if evt.key() == Key::Escape {
                                                editing_id.set(None);
                                            }
                                        }
                                    }
                                } else {
                                    span { 
                                        ondoubleclick: move |_| start_editing(todo.id, todo.text.clone()),
                                        "{todo.text}" 
                                    }
                                }
                            }
                            button { class: "delete-btn", onclick: move |_| remove_todo(todo.id), "✕" }
                        }
                    }
                }

                if todos.read().is_empty() {
                    div { class: "empty-state", 
                        p { "Nothing here! ✨" }
                    }
                }
            }
        }
    }
}
