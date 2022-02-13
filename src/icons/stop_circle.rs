use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub size: Option<String>,
    pub color: Option<String>,
    pub class_name: Option<String>,
}

#[function_component(StopCircle)]
pub fn stop_circle(props: &Props) -> Html {
    // TODO: defaul value for Props
    html! {
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        fill={props.color.clone()}
        class="bi-stop-circle"
        viewBox="0 0 16 16"
      >
        <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z" />
        <path d="M5 6.5A1.5 1.5 0 0 1 6.5 5h3A1.5 1.5 0 0 1 11 6.5v3A1.5 1.5 0 0 1 9.5 11h-3A1.5 1.5 0 0 1 5 9.5v-3z" />
      </svg>
    }
}
