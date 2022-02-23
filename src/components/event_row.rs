use crate::{
    types::tasks::TaskResponse,
    utils::format_time::{calculate_timer, fix_two_digits},
};
use gloo::console::{self};
use js_sys::Date;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub task_response: Vec<TaskResponse>,
}

#[function_component(TableRow)]
pub fn table_row(props: &Props) -> Html {
    // const [hours, minutes, seconds] = calculateTimer(timeInSeconds);

    html! {
      // <div class="EventsRow">
      {for props.task_response.iter().map(move |task| {
          let js_initial_time = Date::new(&JsValue::from_str(&task.initial_time));
          let js_end_time = Date::new(&JsValue::from_str(&task.end_time));
          let js_total_time_in_seconds = (js_end_time.get_time() - js_initial_time.get_time()) / 1000.0;
          let js_date = Date::new(&JsValue::from_str(&task.initial_time));
          console::log!("date: ", js_date.clone());

          let time_display = calculate_timer(js_total_time_in_seconds.clone() as usize);
          let hours: String = time_display[0].clone();
          let minutes: String = time_display[1].clone();
          let seconds: String = time_display[2].clone();
          html!{
              <li class="list-item">
                <div class="task">
                  <span class="task-name">{task.name.clone()}</span>
                  <span class="project">{task.project.clone()}</span>
                </div>
                <div class="right-side">
                  <div>
                    <span>{js_initial_time.get_hours()}</span>
                    <span>{":"}</span>
                    <span>{ fix_two_digits(js_initial_time.get_minutes())}</span>
                  </div>
                  <div>
                    <span class="separator">
                      {"->"}
                    </span>
                  </div>
                  <div>
                    <span>{js_end_time.get_hours()}</span>
                    <span>{":"}</span>
                    <span>{fix_two_digits(js_end_time.get_minutes())}</span>
                  </div>
                    <span class="total">
                    {hours}{":"}{minutes}{":"}{seconds}
                  </span>
                </div>
              </li>
          }
        })
      }
        // </div>
    }
}
