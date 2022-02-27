use crate::{
    types::tasks::TaskResponse,
    utils::format_time::{calculate_timer, convert_date_to_am_pm, fix_two_digits},
};
use js_sys::Date;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub task_response: Vec<TaskResponse>,
}

#[function_component(TableRow)]
pub fn table_row(props: &Props) -> Html {
    html! {
      {for props.task_response.iter().map(move |task| {
          let js_initial_time = Date::new(&JsValue::from_str(&task.initial_time));
          let js_end_time = Date::new(&JsValue::from_str(&task.end_time));
          let js_total_time_in_seconds = (js_end_time.get_time() - js_initial_time.get_time()) / 1000.0;

          let [hours, minutes, seconds] = calculate_timer(js_total_time_in_seconds.clone() as usize);

          html!{
            <li class="list-item">
              <div class="task">
                <span class="task-name">{task.name.clone()}</span>
                <span class="project">{task.project.clone()}</span>
              </div>
              <div class="right-side">
                <div>
                  <span>{convert_date_to_am_pm(js_initial_time.get_hours(), js_initial_time.get_minutes())}</span>
                </div>
                <div>
                  <span class="separator">
                  {"-"}
                  </span>
                </div>
                <div>
                  <span>{convert_date_to_am_pm(js_end_time.get_hours(), js_end_time.get_minutes())}</span>
                </div>
                  <span class="total">
                  {hours}{":"}{minutes}{":"}{seconds}
                  </span>
              </div>
            </li>
          }
        })
      }
    }
}
