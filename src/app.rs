use super::clock::Clock;
use super::events_table::EventsTable;
use super::test_component::Async;
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div class="main">
            <Clock timer_array={vec![2, 20, 10]}/>
            <EventsTable />
            <Async />
          </div>
        }
    }
}
