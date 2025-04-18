pub mod github;
use github::push_to_github;
use js_sys::Function;
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, window}; // สำหรับ HTTP request


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let on_click = move |_| {
        if let Some(win) = window() {
            let val = js_sys::Reflect::get(&win, &"getMarkdownValue".into()).ok();
            let text_field = js_sys::Reflect::get(&win, &"getFilename".into()).ok();
            let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
            console::log_1(&format!("t {}" , token).into());
            if let (Some(val), Some(text_field)) = (val, text_field) {
                if val.is_function() && text_field.is_function() {
                    let func = val.dyn_into::<Function>().ok();
                    let func_file = text_field.dyn_into::<Function>().ok();
    
                    if let (Some(func), Some(func_file)) = (func, func_file) {
                        let result = func.call0(&JsValue::NULL).ok();
                        let result_file = func_file.call0(&JsValue::NULL).ok();
    
                        if let (Some(result), Some(result_file)) = (result, result_file) {
                            let content = result.as_string().unwrap_or_default();
                            let file_name = result_file.as_string().unwrap_or("untitled".to_string());
                            let path = format!("src/content/blog/{}.md", file_name);
                            console::log_1(&format!("Saving {}...", &path).into());
    
                            wasm_bindgen_futures::spawn_local(async move {
                                if let Err(err) = push_to_github(&path, &content).await {
                                    console::log_1(&format!("Error pushing to GitHub: {}", err).into());
                                }
                            });
                        }
                    }
                }
            }
        } else {
            console::log_1(&"No window object found".into());
        }
    };
    

    view! {
        <button class="bg-blue-500 text-white px-4 py-2 rounded mt-4" on:click=on_click>
            "Save Markdown"
        </button>
        <button
            class="text-mycolor-1"
            on:click=move |_| set_count.set(3)
        >
            "Click me!!!: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}

fn main() {
    mount_to_body(App);
}
