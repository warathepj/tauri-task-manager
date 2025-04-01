#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

// Define a Task struct to represent each task
#[derive(Clone, Debug)]
struct Task {
    id: usize,
    content: String,
    completed: bool,
}

pub fn App() -> Element {
    let mut name = use_signal(|| String::new());
    let mut greet_msg = use_signal(|| String::new());
    let mut new_input = use_signal(|| String::new());
    // Add a signal to store the list of tasks
    let mut tasks = use_signal(|| Vec::<Task>::new());
    // Counter for task IDs
    let mut next_id = use_signal(|| 0);

    let greet = move |_: FormEvent| async move {
        if name.read().is_empty() {
            return;
        }

        let name = name.read();
        let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        let new_msg = invoke("greet", args).await.as_string().unwrap();
        greet_msg.set(new_msg);
    };

    // Function to create a new task
    let create_task = move |_| {
        let input = {
            let input_binding = new_input.read();
            input_binding.trim().to_string()
        };
        
        if !input.is_empty() {
            // Read the ID first and store it
            let current_id = *next_id.read();
            
            // Create a new task and add it to the list
            tasks.write().push(Task {
                id: current_id,
                content: input,
                completed: false,
            });
            
            // Increment the ID counter
            next_id.set(current_id + 1);
            // Clear the input field
            new_input.set(String::new());
        }
    };

    // Create a mutable clone of tasks for delete_task
    let mut tasks_for_delete = tasks.clone();
    let mut delete_task = move |task_id: usize| {
        tasks_for_delete.write().retain(|task| task.id != task_id);
    };

    // Create a clone for the render function
    let tasks_for_render = tasks.clone();

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main {
            class: "container",
            h1 { "Welcome to Tauri + Dioxus" }

            div {
                class: "row",
                a {
                    href: "https://tauri.app",
                    target: "_blank",
                    img {
                        src: "/tauri.svg",
                        class: "logo tauri",
                        alt: "Tauri logo"
                    }
                }
                a {
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    img {
                        src: "/dioxus.png",
                        class: "logo dioxus",
                        alt: "Dioxus logo"
                    }
                }
            }
            p { "Click on the Tauri and Dioxus logos to learn more." }

            form {
                class: "row",
                onsubmit: greet,
                input {
                    id: "greet-input",
                    placeholder: "Enter a name...",
                    value: "{name}",
                    oninput: move |event| name.set(event.value())
                }
                button { r#type: "submit", "Greet" }
            }
            p { "{greet_msg}" }

            // Task creation section
            div {
                class: "row",
                input {
                    id: "new-input",
                    placeholder: "New task...",
                    value: "{new_input}",
                    oninput: move |event| new_input.set(event.value())
                }
                button {
                    r#type: "button",
                    onclick: create_task,
                    "Create Task"
                }
            }

            // Task list section
            div {
                class: "task-list",
                h2 { "Tasks" }
                ul {
                    style: "list-style-type: none; padding: 0;",
                    {tasks_for_render.read().iter().map(|task| {
                        let task = task.clone();
                        rsx! {
                            li {
                                key: "{task.id}",
                                style: "display: flex; align-items: center; margin-bottom: 8px; padding: 8px; border: 1px solid #ddd; border-radius: 4px;",
                                input {
                                    r#type: "checkbox",
                                    checked: task.completed,
                                }
                                span {
                                    style: "margin-left: 8px; flex-grow: 1;",
                                    "{task.content}"
                                }
                                button {
                                    style: "background-color: #ff4d4d; color: white; border: none; border-radius: 4px; padding: 4px 8px;",
                                    onclick: move |_| delete_task(task.id),
                                    "Delete"
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
