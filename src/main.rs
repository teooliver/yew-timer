mod app;
mod clock;
mod error;
mod event_row;
mod events_table;
mod icons;
mod test_component;
mod types;
mod utils;

fn main() {
    yew::start_app::<app::App>();
}
