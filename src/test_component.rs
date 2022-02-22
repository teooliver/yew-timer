use yew::prelude::*;
use yew_hooks::use_async;

#[function_component(Async)]
pub fn async_test() -> Html {
    let state = use_async(async move { fetch("/api/user/123".to_string()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            state.run();
        })
    };

    html! {
        <div>
            <button {onclick} disabled={state.loading}>{ "Start loading" }</button>
            {
                if state.loading {
                    html! { "Loading" }
                } else {
                    html! {}
                }
            }
            {
                if let Some(data) = &state.data {
                    html! { data }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &state.error {
                    html! { error }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

async fn fetch(url: String) -> Result<String, String> {
    // You can use reqwest to fetch your http api
    Ok(String::from("Jet Li"))
}
