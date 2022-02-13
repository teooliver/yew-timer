use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub size: Option<String>,
    pub color: Option<String>,
    pub class_name: Option<String>,
}

#[function_component(PlayCircle)]
pub fn play_circle(props: &Props) -> Html {
    // TODO: defaul value for Props
    html! {
       <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          fill={props.color.clone()}
          className="bi-play-circle"
          viewBox="0 0 16 16"
        >
          <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z" />
          <path d="M6.271 5.055a.5.5 0 0 1 .52.038l3.5 2.5a.5.5 0 0 1 0 .814l-3.5 2.5A.5.5 0 0 1 6 10.5v-5a.5.5 0 0 1 .271-.445z" />
        </svg>
    }
}
