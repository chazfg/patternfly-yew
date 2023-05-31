//! About modal
use crate::{use_backdrop, Button, ButtonVariant, Icon};
use yew::prelude::*;
use yew_hooks::{use_click_away, use_event_with_window};

/// Properties for [`About`]
#[derive(Clone, PartialEq, Properties)]
pub struct AboutModalProperties {
    /// Required Attributes
    pub brand_image_src: AttrValue,
    pub brand_image_alt: AttrValue,
    pub children: Children,

    #[prop_or(AttrValue::from("About Dialog"))]
    pub aria_label: AttrValue, // FIXME: This should be set if product_name is not used.
    #[prop_or_default]
    pub background_image_src: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(AttrValue::from("Close dialog"))]
    pub close_button_aria_label: AttrValue,
    #[prop_or_default]
    pub product_name: AttrValue,
    #[prop_or_default]
    pub trademark: AttrValue,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,

    /// Additional attributes not included in PF React
    /// Disable closing the modal when the escape key is pressed
    #[prop_or_default]
    pub disable_close_escape: bool,
    /// Disable closing the modal when the user clicks outside the modal
    #[prop_or_default]
    pub disable_close_click_outside: bool,
    /// Id of the outermost element
    #[prop_or_default]
    pub id: AttrValue,

    // TODO: Unimplemented PF React attributes:
    // * appendTo
    // * disableFocusTrap
    // * hasNoContentContainer
    // * isOpen
}

/// About modal component
///
/// > An **about modal** displays information about an application like product version number(s), as well as any appropriate legal text.
///
/// See: <https://www.patternfly.org/v4/components/about-modal>
///
/// For a complete example, see the PatternFly Yew quickstart.
///
/// ## Properties
///
/// Defined by [`AboutModalProperties`].
///
/// ## Contexts
///
/// If the modal dialog is wrapped by a [`crate::prelude::BackdropViewer`] component and no
/// `onclose` callback is set, then it will automatically close the backdrop when the modal dialog
/// gets closed.
///
#[function_component(AboutModal)]
pub fn about_modal(props: &AboutModalProperties) -> Html {
    // TODO: Focus is not trapped implemented.

    let backdrop = use_backdrop();

    let onclose = use_memo(
        |(onclose, backdrop)| {
            let onclose = onclose.clone();
            let backdrop = backdrop.clone();
            Callback::from(move |()| {
                if let Some(onclose) = &onclose {
                    onclose.emit(());
                } else if let Some(backdrop) = &backdrop {
                    backdrop.close();
                }
            })
        },
        (props.onclose.clone(), backdrop.clone()),
    );

    // escape key
    {
        let disabled = props.disable_close_escape.clone();
        let onclose = onclose.clone();
        use_event_with_window("keydown", move |e: KeyboardEvent| {
            if !disabled && e.key() == "Escape" {
                onclose.emit(());
            }
        });
    }

    // outside click
    let node_ref = use_node_ref();

    {
        let disabled = props.disable_close_click_outside.clone();
        let onclose = onclose.clone();
        use_click_away(node_ref.clone(), move |_: Event| {
            if !disabled {
                onclose.emit(());
            }
        });
    }

    let (aria_labeledby, aria_label, header) =
        if props.product_name.is_empty() {
            (
                props.id.clone(),
                props.aria_label.clone(),
                html!()
            )
        } else {
            (
                AttrValue::from("about-modal-title"),
                AttrValue::default(),
                html!(
                    <div class="pf-c-about-modal-box__header">
                        <h1 class="pf-c-title pf-m-4xl" id="about-modal-title">{ props.product_name.clone() }</h1>
                    </div>
                )
            )
        };

    let hero_style =
        if props.background_image_src.is_empty() {
            AttrValue::default()
        } else {
            AttrValue::from(format!("--pf-c-about-modal-box__hero--sm--BackgroundImage: url( {} );", props.background_image_src))
        };

    html!(
        <div
            id={props.id.clone()}
            class={classes!("pf-c-about-modal-box", props.class.clone())}
            role="dialog"
            aria-modal="true"
            aria-labeledby={aria_labeledby}
            aria-label={aria_label}
            ref={node_ref}
        >
            if !props.brand_image_src.is_empty() {
                <div class="pf-c-about-modal-box__brand">
                    <img
                      class="pf-c-about-modal-box__brand-image"
                      src={props.brand_image_src.clone()}
                      alt={props.brand_image_alt.clone()}
                    />
                </div>
            }

            <div class="pf-c-about-modal-box__close">
                <Button
                    variant={ButtonVariant::Plain}
                    aria_label={props.close_button_aria_label.clone().to_string()}  // TODO: `to_string` call can be removed once button aria_label is changed to AttValue type
                    onclick={onclose.reform(|_|())}
                >
                    { Icon::Times }
                </Button>
            </div>

            { header }

            <div
                class="pf-c-about-modal-box__hero"
                style={hero_style}
            />

            <div class="pf-c-about-modal-box__content">
                <div class="pf-c-about-modal-box__body">
                    { for props.children.clone() }
                </div>
                if !props.trademark.is_empty() {
                    <p class="pf-c-about-modal-box__strapline">{ props.trademark.clone() }</p>
                }
            </div>
        </div>
    )
}
