use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StatsWidgetProps {
    pub title: String,
    pub value: String,
    pub icon: String,
    #[prop_or_default]
    pub color: String,
}

#[function_component(StatsWidget)]
pub fn stats_widget(props: &StatsWidgetProps) -> Html {
    let color_class = if props.color.is_empty() {
        "text-purple-600"
    } else {
        &props.color
    };
    
    html! {
        <div class="bg-white rounded-lg shadow-sm p-6 text-center">
            <div class="text-3xl mb-2">{&props.icon}</div>
            <div class={format!("text-2xl font-bold mb-1 {}", color_class)}>{&props.value}</div>
            <div class="text-gray-600">{&props.title}</div>
        </div>
    }
}
