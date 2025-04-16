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
            let val = js_sys::Reflect::get(&win, &"getMarkdownValue".into()).unwrap();
            if val.is_function() {
                let func = val.dyn_into::<Function>().unwrap();
                let result = func.call0(&JsValue::NULL).unwrap();

                // Log result
                // console::log_1(&result);
                // เรียก API สำหรับ push ไฟล์ไป GitHub
                let path = format!("src/content/blog/{}.md", "your_filename"); // คุณต้องตั้งชื่อไฟล์
                let content = format!("{:?}", result);
                let display = format!("{} {}", path, content);
                console::log_1(&display.into());

                // เรียก API สำหรับส่งข้อมูลไป backend
                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(err) = push_to_github(&path, &content).await {
                        console::log_1(&format!("Error pushing to GitHub: {}", err).into());
                    }
                });
            } else {
                console::log_1(&"getMarkdownValue is not a function".into());
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
            "Click me: "
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
