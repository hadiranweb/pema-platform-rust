use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    pub title: String,
    pub show: bool,
    pub on_close: Callback<()>
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_class = if props.show { "modal display-block" } else { "modal display-none" };

    let on_close = props.on_close.clone();
    let handle_close = Callback::from(move |_| {
        on_close.emit(());
    });

    html! {
        <div class={modal_class}>
            <section class="modal-main">
                <div class="modal-header">
                    <h2>{ &props.title }</h2>
                    <button onclick={handle_close}>{ "X" }</button>
                </div>
                <div class="modal-body">
                    { props.children.clone() }
                </div>
            </section>
        </div>
    }
}

