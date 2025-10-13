use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub children: Children,
    #[prop_or_default]
    pub class: String,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let class = format!("data-table {}", props.class);

    html! {
        <div class="table-container">
            <table class={class}>
                { for props.children.iter() }
            </table>
        </div>
    }
}
