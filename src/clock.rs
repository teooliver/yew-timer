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
    UpdateTime,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub timer_array: Vec<i8>,
}

pub struct Clock {
    time: String,
    messages: Vec<&'static str>,
    _standalone: (Interval, Interval),
    interval: Option<Interval>,
    timeout: Option<Timeout>,
    console_timer: Option<Timer<'static>>,
    timer_array: Vec<u8>,
    time_in_seconds: i16,
    laps: Vec<String>,
}

impl Clock {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        // console::log!("===> Date", date.clone());
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
        let standalone_handle =
            Interval::new(10, || console::debug!("Example of a standalone callback."));

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: Clock::get_current_time(),
            messages: Vec::new(),
            _standalone: (standalone_handle, clock_handle),
            interval: None,
            timeout: None,
            console_timer: None,
            timer_array: vec![0, 0, 0],
            time_in_seconds: 0,
            laps: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StopInterval => {
                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(3, move || link.send_message(Msg::Done))
                };

                self.timeout = Some(handle);

                self.messages.clear();
                console::clear!();

                self.messages.push("Timer started!");
                self.console_timer = Some(Timer::new("Timer"));
                true
            }
            Msg::StartInterval => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);

                self.messages.clear();
                console::clear!();

                self.messages.push("Interval started!");
                true
            }

            Msg::RecordLap => {
                self.laps.push(self.time.clone());
                true
            }

            Msg::Cancel => {
                self.cancel();
                self.messages.push("Canceled!");
                console::warn!("Canceled!");
                self.time_in_seconds = 0;
                self.laps = [].to_vec();
                true
            }
            Msg::Done => {
                self.cancel();
                self.messages.push("Done!");

                console::info!("Done!");
                if let Some(timer) = self.console_timer.take() {
                    drop(timer);
                }

                true
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                self.time_in_seconds = self.time_in_seconds + 1;

                true
            }
            Msg::UpdateTime => {
                self.time = Clock::get_current_time();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let has_job = self.timeout.is_some() || self.interval.is_some();
        let has_job = false;
        html! {
              <div class="stopwatch">
                <div id="clock">
                    <div id="time">
                        { &self.time }
                    </div>
                </div>
                <div>
                    <span class={classes!("clock")} >{self.time_in_seconds}</span>
                    // <span>{ Self::timer_in_sec() }</span>
                    // <span class={classes!("time-text")}>{ &ctx.props().timer_array[0] }</span>
                    // <span>{":"}</span>
                    // <span class={classes!("time-text")}>{ &ctx.props().timer_array[1] }</span>
                    // <span>{":"}</span>
                    // <span class={classes!("time-text")}>{ &ctx.props().timer_array[2] }</span>
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
                    { "Cancel!" }
                    </button>
                </div>
                <div id="messages">
                    { for self.laps.iter().map(|lap| html! { <p>{ lap }</p> }) }
                </div>

              </div>
              // <div className={styles.Controls}>
        //   {isPlaying ? (
        //     <button onClick={handleStopButton} data-testid='stop-button'>
        //       <StopCircle className={styles['stop-btn']} size='32' />
        //     </button>
        //   ) : (
        //     <button onClick={handlePlayButton} data-testid='play-button'>
        //       <PlayCircle className={styles['play-btn']} size='32' />
        //     </button>
        //   )}
        //   <button onClick={handleResetButton} data-testid='reset-button'>
        //     <XCircle className={styles['reset-btn']} size='32' />
        //   </button>
        // </div>
            }
    }
}
