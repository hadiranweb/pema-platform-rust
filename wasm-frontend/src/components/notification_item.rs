use yew::prelude::*;
use models::notification::Notification;

#[derive(Properties, PartialEq)]
pub struct NotificationItemProps {
    pub notification: Notification,
}

#[function_component(NotificationItem)]
pub fn notification_item(props: &NotificationItemProps) -> Html {
    let notification = &props.notification;
    
    html! {
        <div class="bg-white rounded-lg shadow-sm p-4 border-r-4 border-blue-500">
            <p class="font-medium">{&notification.message}</p>
            <p class="text-sm text-gray-500">{"نوع: "}{&notification.notification_type}</p>
        </div>
    }
}
