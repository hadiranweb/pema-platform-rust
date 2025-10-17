use yew::prelude::*;

#[function_component(AnimatedSkyBackground)]
pub fn animated_sky_background() -> Html {
    html! {
        <div class="grok-dark-background">
            // Deep space gradient similar to Grok's dark theme
            <div class="deep-space-gradient"></div>
            
            // Static particles for simplicity
            <div class="particle-field">
                <div class="particle" style="left: 10%; top: 20%; width: 2px; height: 2px; animation-delay: 0s;"></div>
                <div class="particle" style="left: 25%; top: 15%; width: 3px; height: 3px; animation-delay: 1s;"></div>
                <div class="particle" style="left: 40%; top: 30%; width: 2px; height: 2px; animation-delay: 2s;"></div>
                <div class="particle" style="left: 60%; top: 10%; width: 4px; height: 4px; animation-delay: 3s;"></div>
                <div class="particle" style="left: 75%; top: 25%; width: 2px; height: 2px; animation-delay: 4s;"></div>
                <div class="particle" style="left: 85%; top: 40%; width: 3px; height: 3px; animation-delay: 5s;"></div>
                <div class="particle" style="left: 15%; top: 60%; width: 2px; height: 2px; animation-delay: 6s;"></div>
                <div class="particle" style="left: 35%; top: 70%; width: 3px; height: 3px; animation-delay: 7s;"></div>
                <div class="particle" style="left: 55%; top: 80%; width: 2px; height: 2px; animation-delay: 0.5s;"></div>
                <div class="particle" style="left: 70%; top: 65%; width: 4px; height: 4px; animation-delay: 1.5s;"></div>
                <div class="particle" style="left: 90%; top: 75%; width: 2px; height: 2px; animation-delay: 2.5s;"></div>
                <div class="particle" style="left: 5%; top: 45%; width: 3px; height: 3px; animation-delay: 3.5s;"></div>
                <div class="particle" style="left: 20%; top: 85%; width: 2px; height: 2px; animation-delay: 4.5s;"></div>
                <div class="particle" style="left: 45%; top: 5%; width: 3px; height: 3px; animation-delay: 5.5s;"></div>
                <div class="particle" style="left: 65%; top: 50%; width: 2px; height: 2px; animation-delay: 6.5s;"></div>
                <div class="particle" style="left: 80%; top: 90%; width: 4px; height: 4px; animation-delay: 7.5s;"></div>
            </div>
            
            // Ambient light effects
            <div class="ambient-glow">
                <div class="glow-orb glow-1"></div>
                <div class="glow-orb glow-2"></div>
                <div class="glow-orb glow-3"></div>
            </div>
            
            // Subtle grid pattern like Grok
            <div class="grid-overlay"></div>
        </div>
    }
}