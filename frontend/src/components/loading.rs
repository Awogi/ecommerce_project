use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loading-container">
            <div class="loading-spinner">
                <div class="spinner-ring"></div>
                <div class="spinner-ring"></div>
                <div class="spinner-ring"></div>
                <div class="spinner-ring"></div>
            </div>
            <p class="loading-text">{"Loading..."}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LoadingOverlayProps {
    pub message: Option<String>,
}

#[function_component(LoadingOverlay)]
pub fn loading_overlay(props: &LoadingOverlayProps) -> Html {
    let message = props.message.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Loading...");

    html! {
        <div class="loading-overlay">
            <div class="loading-content">
                <div class="loading-spinner">
                    <div class="spinner-ring"></div>
                    <div class="spinner-ring"></div>
                    <div class="spinner-ring"></div>
                    <div class="spinner-ring"></div>
                </div>
                <p class="loading-text">{message}</p>
            </div>
        </div>
    }
}