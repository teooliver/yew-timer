use super::clock::{Clock, Msg};
use crate::{
    types::tasks::TaskResponse,
    utils::format_time::{calculate_timer, convert_date_to_am_pm, fix_two_digits},
};
use gloo::console::{self};
use js_sys::Date;
use wasm_bindgen::JsValue;
use yew::{function_component, html, Callback, Context, Properties};

use crate::icons::stop_circle::StopCircle;
use crate::icons::x_circle::XCircle;
use crate::{icons::play_circle::PlayCircle, types::tasks::Task};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub start_time: Callback<Msg>,
    pub is_tracking: bool,
    pub has_job: bool,
}

#[function_component(ClockControls)]
pub fn clock_controls(props: &Props) -> Html {
    let onclick = {
        let start = props.start_time.clone();
        move |_| start.emit(Msg::StartInterval)
    };

    html! {
    // <div>


        // if props.is_tracking {
        //     html! {
        //         <button disabled={props.has_job} onclick={props.ctx.link().callback(|_| Msg::StopInterval)} class="stop-btn">
        //           <StopCircle color="green" />
        //         </button>
        //     }
        // } else {
            // html! {
              <button disabled={props.has_job} onclick={onclick} class="start-btn">
                  <PlayCircle color="green" />
              </button>
            // }
        // }

        // <button disabled={props.has_job} onclick={props.ctx.link().callback(|_| Msg::Cancel)} class="cancel-btn">
        //     <XCircle color="green" />
        // </button>
    // </div>
    }
}
