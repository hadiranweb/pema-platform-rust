use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub subtitle: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub class: String,
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let class = format!("card {}", props.class);
    let is_clickable = props.onclick.is_some();
    
    let card_class = if is_clickable {
        format!("{} card-clickable", class)
    } else {
        class
    };

    let onclick = props.onclick.clone().unwrap_or_default();

    html! {
        <div class={card_class} onclick={onclick}>
            if props.title.is_some() || props.subtitle.is_some() {
                <div class="card-header">
                    if let Some(title) = &props.title {
                        <h3 class="card-title">{title}</h3>
                    }
                    if let Some(subtitle) = &props.subtitle {
                        <p class="card-subtitle">{subtitle}</p>
                    }
                </div>
            }
            <div class="card-content">
                { for props.children.iter() }
            </div>
        </div>
    }
}
