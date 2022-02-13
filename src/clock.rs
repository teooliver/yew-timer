use gloo::{
    console::{self, Timer},
    timers::callback::{Interval, Timeout},
};
use web_sys::HtmlInputElement as InputElement;

use crate::icons::play_circle::PlayCircle;
use crate::icons::stop_circle::StopCircle;
use crate::icons::x_circle::XCircle;
use crate::utils::format_time::calculate_timer;
use yew::{
    classes, events::KeyboardEvent, html, html::Scope, Component, Context, Html, Properties,
    TargetCast,
};

pub enum Msg {
    AddTask(String),
    StopInterval,
    StartInterval,
    RecordLap,
    Cancel,
    Done,
    Tick,
    StartClock,
    StopClock,
    UpdateTime,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub timer_array: Vec<i8>,
}

pub struct Clock {
    time: String,
    messages: Vec<&'static str>,
    standalone: Option<Interval>,
    interval: Option<Interval>,
    timeout: Option<Timeout>,
    console_timer: Option<Timer<'static>>,
    time_in_seconds: i16,
    laps: Vec<String>,
    tasks: Vec<String>,
    is_tracking: bool,
}

impl Clock {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }

    fn cancel(&mut self) {
        self.timeout = None;
        self.interval = None;
    }

    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::AddTask(value))
            } else {
                None
            }
        });
        html! {
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
        }
    }
}

impl Component for Clock {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        link.send_message(Msg::StartClock);

        Self {
            time: Clock::get_current_time(),
            messages: Vec::new(),
            standalone: None,
            interval: None,
            timeout: None,
            console_timer: None,
            time_in_seconds: 0,
            laps: vec![],
            tasks: vec![],
            is_tracking: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTask(description) => {
                if !description.is_empty() {
                    self.tasks.push(description.trim().to_string());
                }
            }

            Msg::StartClock => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1, move || link.send_message(Msg::UpdateTime))
                };

                self.standalone = Some(handle);

                console::log!("Start Clock");
            }

            Msg::UpdateTime => {
                self.time = Clock::get_current_time();
            }

            Msg::StopClock => {
                if let Some(timer) = self.standalone.take() {
                    drop(timer);
                }

                console::log!("Stop Clock");
            }

            Msg::StopInterval => {
                self.is_tracking = false;
                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(1, move || link.send_message(Msg::Done))
                };

                self.timeout = Some(handle);

                self.messages.clear();
                console::clear!();

                self.messages.push("Timer started!");
                self.console_timer = Some(Timer::new("Timer"));
            }

            Msg::StartInterval => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1000, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);

                self.messages.clear();
                console::clear!();

                self.is_tracking = true;
                self.messages.push("Interval started!");
            }

            Msg::RecordLap => {
                self.laps.push(self.time_in_seconds.to_string().clone());
            }

            Msg::Cancel => {
                self.cancel();
                self.messages.push("Canceled!");
                console::warn!("Canceled!");
                self.time_in_seconds = 0;
                self.laps = [].to_vec();
            }

            Msg::Done => {
                self.cancel();
                self.messages.push("Done!");

                console::info!("Done!");
                if let Some(timer) = self.console_timer.take() {
                    drop(timer);
                }
            }

            Msg::Tick => {
                self.messages.push("Tick...");
                self.time_in_seconds = self.time_in_seconds + 1;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let has_job = self.timeout.is_some() || self.interval.is_some();
        let has_job = false;
        let timer = calculate_timer(self.time_in_seconds as usize);
        html! {
            <>
            // <h1>{"Rust Clock Example"}</h1>
            <div class="stopwatch">
              { self.view_input(ctx.link()) }
              <div id="messages">
              { for self.tasks.iter().map(|lap| html! { <p>{ lap }</p> }) }
              </div>
              <div id="clock">
              <div id="time" class="time">
              { &self.time }
              </div>
              <div>
              <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartClock)} class="cancel-btn">
              { "Start Clock" }
              </button>
              <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StopClock)} class="cancel-btn">
              { "Stop Clock" }
              </button>
              </div>

              </div>
              <hr class="hr" />
            //   <div>
            //     <span class={classes!("counter")} >{self.time_in_seconds}</span>
            //   </div>
              <div class={classes!("counter")}>
                <span>{&timer[0]}</span>
                <span>{":"}</span>
                <span>{&timer[1]}</span>
                <span>{":"}</span>
                <span>{&timer[2]}</span>
              </div>
              <div>
                {
                    if self.is_tracking {
                        html! {
                            <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StopInterval)} class="stop-btn">
                            <StopCircle color="green" />
                            </button>
                        }
                    } else {
                        html! {
                        <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartInterval)} class="start-btn">
                            <PlayCircle color="green" />
                        </button>
                        }
                    }
                }
                    // <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::RecordLap)} class="lap-btn">
                    //     { "Lap" }
                    // </button>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::Cancel)} class="cancel-btn">
                        <XCircle color="green" />
                    </button>
                </div>
                <div id="messages">
                    { for self.laps.iter().map(|lap| html! { <p>{ lap }</p> }) }
                </div>
            </div>
            </>
        }
    }
}
