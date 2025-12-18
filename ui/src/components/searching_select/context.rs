//! Context types and implementations for the select component.

use dioxus::prelude::*;

use std::{any::Any, rc::Rc};

use super::focus::FocusState;

trait DynPartialEq: Any {
    fn eq(&self, other: &dyn Any) -> bool;
}

impl<T: PartialEq + 'static> DynPartialEq for T {
    fn eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>() == Some(self)
    }
}

#[derive(Clone)]
pub(crate) struct RcPartialEqValue {
    value: Rc<dyn DynPartialEq>,
}

impl RcPartialEqValue {
    pub fn new<T: PartialEq + 'static>(value: T) -> Self {
        Self {
            value: Rc::new(value),
        }
    }

    pub fn as_any(&self) -> &dyn Any {
        (&*self.value) as &dyn Any
    }

    pub fn as_ref<T: PartialEq + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl PartialEq for RcPartialEqValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&*other.value)
    }
}

/// Main context for the select component containing all shared state
#[derive(Clone, Copy)]
pub(super) struct SelectContext {
    /// The typeahead buffer for searching options
    pub typeahead_buffer: Signal<String>,
    /// If the select is open
    pub open: Signal<bool>,
    /// Current value
    pub value: Memo<Option<RcPartialEqValue>>,
    /// Set the value callback
    pub set_value: Callback<Option<RcPartialEqValue>>,
    /// A list of options with their states
    pub options: Signal<Vec<OptionState>>,
    /// The ID of the list for ARIA attributes
    pub list_id: Signal<Option<String>>,
    /// The focus state for the select
    pub focus_state: FocusState,
    /// Whether the select is disabled
    pub disabled: ReadSignal<bool>,
    /// The placeholder text
    pub placeholder: ReadSignal<String>,
    /// The initial element to focus once the list is rendered
    pub initial_focus: Signal<Option<usize>>,
}

impl SelectContext {
    /// Select the currently focused item
    pub fn select_current_item(&mut self) {
        // If the select is open, select the focused item
        if self.open.cloned() {
            if let Some(focused_index) = self.focus_state.current_focus() {
                let options = self.options.read();
                if let Some(option) = options.iter().find(|opt| opt.tab_index == focused_index) {
                    self.set_value.call(Some(option.value.clone()));
                    self.open.set(false);
                }
            }
        }
    }
}

/// State for individual select options
pub(super) struct OptionState {
    /// Tab index for focus management
    pub tab_index: usize,
    /// The value of the option
    pub value: RcPartialEqValue,
    /// Display text for the option
    pub text_value: String,
    /// Unique ID for the option
    pub id: String,
}

/// Context for select option components to know if they're selected
#[derive(Clone, Copy)]
pub(super) struct SelectOptionContext {
    /// Whether this option is currently selected
    pub selected: ReadSignal<bool>,
}

/// Context for children of select list components to know if they should render
#[derive(Clone, Copy)]
pub(super) struct SelectListContext {
    /// Whether to render in the dom (or just run logic)
    pub render: ReadSignal<bool>,
}

/// Context for select group components
#[derive(Clone, Copy)]
pub(super) struct SelectGroupContext {
    /// ID of the element that labels this group
    pub labeled_by: Signal<Option<String>>,
}
