use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub is_open: bool,
    pub title: String,
    pub children: Children,
    pub on_close: Callback<()>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let ModalProps { is_open, title, children, on_close } = props;

    if !is_open {
        return html! {};
    }

    let backdrop_click = {
        let on_close = on_close.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_close.emit(());
        })
    };

    let modal_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    html! {
        <div class="modal-backdrop" onclick={backdrop_click}>
            <div class="modal-container" onclick={modal_click}>
                <div class="modal-header">
                    <h2 class="modal-title">{title}</h2>
                    <button class="modal-close-btn" onclick={on_close.reform(|_| ())}>
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M18 6L6 18M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>
                <div class="modal-content">
                    {for children.iter()}
                </div>
            </div>
        </div>
    }
}