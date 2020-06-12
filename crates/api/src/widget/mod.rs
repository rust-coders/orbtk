use std::{fmt::Debug, rc::Rc};

use dces::prelude::{Component, Entity};

use crate::{css_engine::*, event::EventHandler, properties::AttachedProperty};

pub use self::build_context::*;
pub use self::context::*;
pub use self::registry::*;
pub use self::state::*;
pub use self::states_context::*;
pub use self::template::*;
pub use self::widget_container::*;

mod build_context;
mod context;
mod registry;
mod state;
mod states_context;
mod template;
mod widget_container;

/// Adds the given `pseudo_class` to the css selector of the given `widget`.
pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Some(selector) = widget.try_get_mut::<Selector>("selector") {
        selector.pseudo_classes.insert(String::from(pseudo_class));
        selector.set_dirty(true);
    }
}

/// Removes the given `pseudo_class` from the css selector of the given `widget`.
pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Some(selector) = widget.try_get_mut::<Selector>("selector") {
        selector.pseudo_classes.remove(pseudo_class);
        selector.set_dirty(true);
    }
}

/// Used to define the `parent_type`of a widget.
pub enum ParentType {
    /// No children could be added to the widget.
    None,

    /// Only one child could be added to the widget.
    Single,

    /// Multiple children could be added to the widget.
    Multi,
}

/// The `Widget` trait is used to define a new widget.
pub trait Widget: Template {
    /// Creates a new widget.
    fn new() -> Self;

    /// Creates a new widget.
    #[inline(always)]
    #[deprecated]
    fn create() -> Self {
        Self::new()
    }

    // This method will always be overwritten by the `widget!` macros.
    fn attach<P: Component + Debug>(self, property: AttachedProperty<P>) -> Self {
        self
    }

    /// Builds the widget and returns the template of the widget.
    fn build(self, ctx: &mut BuildContext) -> Entity;

    /// Inerts a new event handler.
    fn insert_handler(self, handler: impl Into<Rc<dyn EventHandler>>) -> Self;

    /// Appends a child to the widget.
    fn child(self, child: Entity) -> Self;
}
