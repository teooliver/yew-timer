use crate::types::tasks::GetAllTasks;
use gloo::console;
use reqwasm::http::Request;
use serde_json::Value;
use web_sys::MouseEvent;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};

#[function_component(EventTable)]
pub fn event_table() -> Html {
    let counter = use_state(|| GetAllTasks {
        results: vec![],
        pagination: None,
    });

    let use_name = use_state(|| "hello");

    {
        // let tasks = counter.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = reqwest::get("http://localhost:5000/tasks/group?page=1")
                        .await
                        .unwrap();

                    let text = response.text().await.unwrap();

                    let v: GetAllTasks = serde_json::from_str(&text).unwrap();
                    let name = v.results[0].tasks[0].name.as_str();

                    // untyped response option
                    // let v: Value = serde_json::from_str(&text).unwrap();
                    // let name = v["results"][0]["tasks"][0]["name"].as_str().unwrap();

                    console::log!("use_effect", name);
                    // tasks.set(v.clone());
                    // counter.set(v.clone());
                    // use_name.set(name.clone());
                });
                || ()
            },
            (),
        );
    }

    // console::log!("jflsdjfs", use_name.clone().to_string());

    let onclick = Callback::from(|mouse_event: MouseEvent| {
        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get("http://localhost:5000/tasks/group?page=1")
                .await
                .unwrap();

            let text = response.text().await.unwrap();

            let v: GetAllTasks = serde_json::from_str(&text).unwrap();
            let name = v.results[0].tasks[0].name.as_str();

            // untyped response option
            // let v: Value = serde_json::from_str(&text).unwrap();
            // let name = v["results"][0]["tasks"][0]["name"].as_str().unwrap();

            console::log!(name);
        });
    });

    html! {
       <>
           <h1>{ "RustConf Explorer" }</h1>
           <div>
              <h3>{"Fetch tasks"}</h3>
              <button  onclick={onclick}>{ "xxxxx" }</button>
              <p>
              { "hello" }
              //  {*counter.results[0].tasks[0].name.as_str().to_string()}
              </p>
            </div>
        </>
    }
}
