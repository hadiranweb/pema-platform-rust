use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub show: bool,
    #[prop_or_default]
    pub onclose: Callback<()>,
    #[prop_or_default]
    pub title: Option<String>,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_overlay_click = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| {
            onclose.emit(());
        })
    };

    let on_content_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    if !props.show {
        return html! {};
    }

    html! {
        <div class="modal-overlay" onclick={on_overlay_click}>
            <div class="modal-content" onclick={on_content_click}>
                if let Some(title) = &props.title {
                    <div class="modal-header">
                        <h3>{title}</h3>
                        <button class="modal-close" onclick={props.onclose.clone()}>
                            {"×"}
                        </button>
                    </div>
                }
                <div class="modal-body">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
