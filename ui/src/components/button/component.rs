use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
    Ghost,
}

impl ButtonVariant {
    pub fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Destructive => "destructive",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Ghost => "ghost",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=button)]
    attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        button {
            class: "button",
            "data-style": variant.class(),
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &onmouseup {
                    f.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn LinkButton(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] new_tab: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmounted: Option<EventHandler<MountedEvent>>,
    #[props(default)] onclick_only: bool,
    rel: Option<String>,
    #[props(into)] to: NavigationTarget,

    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        Link {
            class: "button",
            "data-style": variant.class(),
            to: to,
            onclick: onclick,
            onmounted: onmounted,
            onclick_only: onclick_only,
            rel: rel,
            attributes: attributes,
            {children}
        }
    }
}
