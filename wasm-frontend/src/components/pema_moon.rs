use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PemaMoonProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or(true)]
    pub animated: bool,
}

#[function_component(PemaMoon)]
pub fn pema_moon(props: &PemaMoonProps) -> Html {
    let class = format!("pema-moon {}", props.class);
    
    html! {
        <div class={class}>
            // Main moon container with glow effect
            <div class="moon-container">
                // Moon surface with Pema branding
                <div class="moon-surface">
                    // Pema logo/text on the moon
                    <div class="moon-logo">
                        <span class="pema-text">{"پما"}</span>
                    </div>
                    
                    // Moon craters for realism
                    <div class="moon-craters">
                        <div class="crater crater-1"></div>
                        <div class="crater crater-2"></div>
                        <div class="crater crater-3"></div>
                    </div>
                </div>
                
                // Glowing aura around the moon
                <div class="moon-glow"></div>
                
                // Subtle light rays
                <div class="moon-rays">
                    <div class="ray ray-1"></div>
                    <div class="ray ray-2"></div>
                    <div class="ray ray-3"></div>
                    <div class="ray ray-4"></div>
                </div>
            </div>
            
            // Floating particles around the moon
            if props.animated {
                <div class="moon-particles">
                    <div class="particle moon-particle-1"></div>
                    <div class="particle moon-particle-2"></div>
                    <div class="particle moon-particle-3"></div>
                    <div class="particle moon-particle-4"></div>
                    <div class="particle moon-particle-5"></div>
                </div>
            }
        </div>
    }
}