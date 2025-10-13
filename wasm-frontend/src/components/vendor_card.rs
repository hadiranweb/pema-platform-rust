use yew::prelude::*;
use models::vendor::Vendor;

#[derive(Properties, PartialEq)]
pub struct VendorCardProps {
    pub vendor: Vendor,
}

#[function_component(VendorCard)]
pub fn vendor_card(props: &VendorCardProps) -> Html {
    let vendor = &props.vendor;
    
    html! {
        <div class="bg-white rounded-lg shadow-sm p-6">
            <h3 class="text-lg font-semibold mb-2">{&vendor.name}</h3>
            <p class="text-gray-600 mb-2">{"مسئول: "}{&vendor.contact_person}</p>
            <p class="text-gray-600">{"ایمیل: "}{&vendor.email}</p>
        </div>
    }
}
