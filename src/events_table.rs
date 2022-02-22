use crate::event_row::TableRow;
use crate::types::tasks::GetAllTasks;
use crate::{error::Error, utils::format_time::calculate_timer};
use gloo::console;
use js_sys::Date;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use yew::{function_component, html, use_effect_with_deps};
use yew_hooks::use_async;

#[function_component(EventsTable)]
pub fn events_table() -> Html {
    let state = use_async(async move { fetch_all_tasks().await });

    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                state.run();
                || ()
            },
            (),
        );
    }

    if let Some(data) = &state.data {
        if !data.results.is_empty() {
            html! {
                <>
                {for data.results.iter().map(move |result| {
                    let js_total_time = Date::new(&JsValue::from_str(&result.total_time.to_string()));
                    let total_time_display = calculate_timer(js_total_time.clone().get_time() as usize);
                    let hours: String = total_time_display[0].clone();
                    let minutes: String = total_time_display[1].clone();
                    let seconds: String = total_time_display[2].clone();


                    html!{
                        <ul class="EventsTable">
                            <li class="date-header">
                                <span>{result.id.clone()}</span>
                                <span class="day-total">{hours}{":"}{minutes}{":"}{seconds}</span>
                           </li>
                            <TableRow  task_response={result.tasks.clone()}/>
                        </ul>
                    }
                    })

                }
                </>
            }
        } else {
            html! {
                <div class="task-preview">{ "No tasks..." }</div>
            }
        }
    } else {
        html! {
            <div class="task-preview">{ "Loading..." }</div>
        }
    }
}

// TODO: move this to utils
async fn fetch_all_tasks() -> Result<GetAllTasks, Error> {
    fetch::<GetAllTasks>(format!("http://localhost:5000/tasks/group?page=1")).await
}

// TODO: move this to utils
async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // Bad error handling and not prod ready, just a quick prototype
    let response = reqwest::get(url).await.unwrap();
    let text = response.text().await.unwrap();
    let v: T = serde_json::from_str(&text).unwrap();

    Ok(v)
}
