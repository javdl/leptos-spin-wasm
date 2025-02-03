use leptos::*;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonVariant::Default => write!(f, "bg-primary text-primary-foreground hover:bg-primary/90"),
            ButtonVariant::Destructive => write!(f, "bg-destructive text-destructive-foreground hover:bg-destructive/90"),
            ButtonVariant::Outline => write!(f, "border border-input bg-background hover:bg-accent hover:text-accent-foreground"),
            ButtonVariant::Secondary => write!(f, "bg-secondary text-secondary-foreground hover:bg-secondary/80"),
            ButtonVariant::Ghost => write!(f, "hover:bg-accent hover:text-accent-foreground"),
            ButtonVariant::Link => write!(f, "text-primary underline-offset-4 hover:underline"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

impl fmt::Display for ButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonSize::Default => write!(f, "h-10 px-4 py-2"),
            ButtonSize::Sm => write!(f, "h-9 rounded-md px-3"),
            ButtonSize::Lg => write!(f, "h-11 rounded-md px-8"),
            ButtonSize::Icon => write!(f, "h-10 w-10"),
        }
    }
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(MouseEvent)>>,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Default);
    let size = size.unwrap_or(ButtonSize::Default);
    let disabled = disabled.unwrap_or(false);

    let base_classes = "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
    
    let combined_classes = format!(
        "{} {} {} {}",
        base_classes,
        variant.to_string(),
        size.to_string(),
        class.unwrap_or_default()
    );

    view! {
        <button
            class=combined_classes
            disabled=disabled
            on:click=move |e| {
                if let Some(callback) = &on_click {
                    callback(e);
                }
            }
        >
            {children()}
        </button>
    }
}
