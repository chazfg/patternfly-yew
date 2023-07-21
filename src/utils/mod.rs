//! Utilities

mod action;
mod context;
mod global_close;
mod html;
mod props;
mod raw;
mod styled;

pub use action::*;
pub use context::*;
pub use global_close::*;
pub use html::*;
pub use props::*;
pub use raw::*;
pub use styled::*;

use std::fmt::{Debug, Display, Formatter};
use web_sys::KeyboardEvent;
use yew::{html::IntoPropValue, AttrValue, Callback};

/// Create a random ID.
///
/// This is creating a random ID, using a v4 UUID.
pub fn random_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(uuid::Uuid);

impl Id {
    /// Get a new, random ID
    pub fn new() -> Self {
        Id(uuid::Uuid::new_v4())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl IntoPropValue<AttrValue> for Id {
    fn into_prop_value(self) -> AttrValue {
        self.to_string().into()
    }
}

impl IntoPropValue<Option<AttrValue>> for Id {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.to_string().into())
    }
}

/// Create a new callback handling only the case when the user pressed the enter key.
pub fn on_enter<F: Fn() + 'static>(f: F) -> Callback<KeyboardEvent> {
    Callback::from(move |evt: KeyboardEvent| {
        if evt.key_code() == 13 {
            f()
        }
    })
}
