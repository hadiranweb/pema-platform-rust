use yew::prelude::*;
use yew_router::prelude::*;
use crate::AppRoute;
use crate::components::{Button, Card, AnimatedSkyBackground, PemaMoon};

#[derive(Clone, PartialEq)]
pub struct OnboardingStep {
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[function_component(Onboarding)]
pub fn onboarding() -> Html {
    let navigator = use_navigator().unwrap();
    let current_step = use_state(|| 0usize);
    let user_preferences = use_state(|| UserPreferences::default());

    let steps = vec![
        OnboardingStep {
            title: "به پلتفرم پما خوش آمدید".to_string(),
            description: "پلتفرم هوشمند سرمایه‌گذاری در نقره‌های گردآفرید".to_string(),
            icon: "🌙".to_string(),
        },
        OnboardingStep {
            title: "هدف سرمایه‌گذاری شما چیست؟".to_string(),
            description: "این اطلاعات به ما کمک می‌کند تا بهترین پیشنهادات را ارائه دهیم".to_string(),
            icon: "🎯".to_string(),
        },
        OnboardingStep {
            title: "میزان آشنایی شما با نقره چقدر است؟".to_string(),
            description: "بر اساس تجربه شما، محتوای مناسب ارائه خواهیم داد".to_string(),
            icon: "⭐".to_string(),
        },
        OnboardingStep {
            title: "آماده شروع هستید!".to_string(),
            description: "همه چیز آماده است. بیایید سفر سرمایه‌گذاری را شروع کنیم".to_string(),
            icon: "🚀".to_string(),
        },
    ];

    let steps_len = steps.len();
    let on_next = {
        let current_step = current_step.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let step = *current_step;
            if step < steps_len - 1 {
                current_step.set(step + 1);
            } else {
                // Navigate to register page after onboarding
                navigator.push(&AppRoute::Register);
            }
        })
    };

    let on_prev = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            let step = *current_step;
            if step > 0 {
                current_step.set(step - 1);
            }
        })
    };

    let on_skip = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Login);
        })
    };

    let current_step_data = &steps[*current_step];

    html! {
        <div class="onboarding-page">
            <AnimatedSkyBackground />
            <PemaMoon class="onboarding-moon" />
            
            <div class="onboarding-container">
                <div class="onboarding-progress">
                    <div class="progress-bar">
                        <div 
                            class="progress-fill" 
                            style={format!("width: {}%", (*current_step + 1) * 100 / steps.len())}
                        ></div>
                    </div>
                    <span class="progress-text">
                        {format!("{} از {}", *current_step + 1, steps.len())}
                    </span>
                </div>

                <Card class="onboarding-card">
                    <div class="onboarding-content">
                        <div class="step-icon">{&current_step_data.icon}</div>
                        <h1 class="step-title">{&current_step_data.title}</h1>
                        <p class="step-description">{&current_step_data.description}</p>
                        
                        // Step-specific content
                        {match *current_step {
                            1 => html! { <InvestmentGoalSelector preferences={user_preferences.clone()} /> },
                            2 => html! { <ExperienceSelector preferences={user_preferences.clone()} /> },
                            _ => html! {}
                        }}
                    </div>
                    
                    <div class="onboarding-actions">
                        if *current_step > 0 {
                            <Button onclick={on_prev} variant="secondary">
                                {"قبلی"}
                            </Button>
                        }
                        
                        <div class="action-group">
                            <Button onclick={on_skip} variant="secondary" class="skip-btn">
                                {"رد کردن"}
                            </Button>
                            <Button onclick={on_next} variant="primary">
                                {if *current_step == steps.len() - 1 { "شروع کنید" } else { "بعدی" }}
                            </Button>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct UserPreferences {
    pub investment_goal: Option<String>,
    pub experience_level: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct PreferenceSelectorProps {
    pub preferences: UseStateHandle<UserPreferences>,
}

#[function_component(InvestmentGoalSelector)]
pub fn investment_goal_selector(props: &PreferenceSelectorProps) -> Html {
    let goals = vec![
        ("short_term", "سرمایه‌گذاری کوتاه‌مدت"),
        ("long_term", "سرمایه‌گذاری بلندمدت"),
        ("diversification", "تنوع‌بخشی پورتفولیو"),
        ("passive_income", "درآمد منظم"),
    ];

    let on_goal_select = {
        let preferences = props.preferences.clone();
        Callback::from(move |goal: String| {
            let mut prefs = (*preferences).clone();
            prefs.investment_goal = Some(goal);
            preferences.set(prefs);
        })
    };

    html! {
        <div class="goal-selector">
            {for goals.iter().map(|(value, label)| {
                let value = value.to_string();
                let onclick = {
                    let on_goal_select = on_goal_select.clone();
                    let value = value.clone();
                    Callback::from(move |_| on_goal_select.emit(value.clone()))
                };
                
                let is_selected = props.preferences.investment_goal.as_ref() == Some(&value);
                let class = if is_selected { "goal-option selected" } else { "goal-option" };
                
                html! {
                    <div class={class} onclick={onclick}>
                        <span>{label}</span>
                    </div>
                }
            })}
        </div>
    }
}

#[function_component(ExperienceSelector)]
pub fn experience_selector(props: &PreferenceSelectorProps) -> Html {
    let levels = vec![
        ("beginner", "تازه‌کار"),
        ("intermediate", "متوسط"),
        ("advanced", "پیشرفته"),
        ("expert", "متخصص"),
    ];

    let on_level_select = {
        let preferences = props.preferences.clone();
        Callback::from(move |level: String| {
            let mut prefs = (*preferences).clone();
            prefs.experience_level = Some(level);
            preferences.set(prefs);
        })
    };

    html! {
        <div class="experience-selector">
            {for levels.iter().map(|(value, label)| {
                let value = value.to_string();
                let onclick = {
                    let on_level_select = on_level_select.clone();
                    let value = value.clone();
                    Callback::from(move |_| on_level_select.emit(value.clone()))
                };
                
                let is_selected = props.preferences.experience_level.as_ref() == Some(&value);
                let class = if is_selected { "experience-option selected" } else { "experience-option" };
                
                html! {
                    <div class={class} onclick={onclick}>
                        <span>{label}</span>
                    </div>
                }
            })}
        </div>
    }
}