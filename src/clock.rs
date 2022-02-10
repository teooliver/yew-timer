use gloo::{
    console::{self, Timer},
    timers::callback::{Interval, Timeout},
};

use yew::{classes, html, Component, Context, Html, Properties};

pub enum Msg {
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
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
        html! {
            <>
            // <h1>{"Rust Clock Example"}</h1>
            <div class="stopwatch">
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
                <div>
                    <span class={classes!("counter")} >{self.time_in_seconds}</span>
                </div>
                <div>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StopInterval)} class="stop-btn">
                        { "Stop" }
                    </button>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartInterval)} class="start-btn">
                        { "Start" }
                    </button>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::RecordLap)} class="lap-btn">
                        { "Lap" }
                    </button>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::Cancel)} class="cancel-btn">
                        { "Cancel" }
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
