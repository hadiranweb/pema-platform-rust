use yew::prelude::*;
use models::user::User;

#[derive(Properties, PartialEq)]
pub struct UserProfileProps {
    pub user: User,
}

#[function_component(UserProfile)]
pub fn user_profile(props: &UserProfileProps) -> Html {
    let user = &props.user;
    
    html! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h3 class="text-lg font-semibold mb-4">{"پروفایل کاربر"}</h3>
            <p class="text-gray-600">{"نام کاربری: "}{&user.username}</p>
            <p class="text-gray-600">{"ایمیل: "}{&user.email}</p>
        </div>
    }
}
