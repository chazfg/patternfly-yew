use crate::{AsClasses, Button, ExtendClasses, Icon, Variant};
use yew::prelude::*;

use strum_macros::{Display, EnumIter, EnumString};

#[derive(Copy, Clone, Display, Debug, PartialEq, Eq, EnumIter, EnumString)]
pub enum Color {
    Grey,
    Blue,
    Green,
    Orange,
    Red,
    Purple,
    Cyan,
}

impl Default for Color {
    fn default() -> Self {
        Self::Grey
    }
}

impl AsClasses for Color {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Color::Grey => {}
            Color::Blue => classes.push("pf-m-blue"),
            Color::Green => classes.push("pf-m-green"),
            Color::Orange => classes.push("pf-m-orange"),
            Color::Red => classes.push("pf-m-red"),
            Color::Purple => classes.push("pf-m-purple"),
            Color::Cyan => classes.push("pf-m-cyan"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LabelProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub outline: bool,
    #[prop_or_default]
    pub overflow: bool,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub href: String,
}

/// A Label component
///
/// > The **label** component allows users to add specific element captions for user clarity and convenience.
///
/// See: https://www.patternfly.org/v4/components/label/html
///
/// # Properties
///
/// Defined in [`LabelProps`].
#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let mut classes = Classes::from("pf-c-label");

    classes.extend_from(&props.color);

    if props.outline {
        classes.push("pf-m-outline");
    }

    if props.overflow {
        classes.push("pf-m-overflow");
    }

    let content = |content: Html| {
        if props.href.is_empty() {
            html! {<span class="pf-c-label__content">{content}</span>}
        } else {
            html! {<a class="pf-c-label__content" href={props.href.clone()}>{content}</a>}
        }
    };

    html! (
        <span class={classes}>
            { content (
                html!{
                    <>
                        if let Some(icon) = &props.icon {
                            <span class="pf-c-label__icon"> { icon.as_html() } </span>
                        }
                        { &props.label }
                    </>
                }
            )}
            if let Some(onclose) = &props.onclose {
                <Button variant={Variant::Plain} icon={Icon::Times} onclick={onclose.reform(|_| {})}/>
            }
        </span>
    )
}
