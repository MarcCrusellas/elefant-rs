#![allow(unused)]

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct EventPrototype {
    pub is_available: bool,
    pub event_script: Option<String>,
}
pub type EP = EventPrototype;


impl EventPrototype {
    pub fn new(event_script: &str) -> Self {
        EventPrototype {
            is_available: true,
            event_script: Some(event_script.to_string()),
        }
    }

    pub fn is_available(&self) -> bool {
        self.is_available
    }

    fn atach_script(&mut self, /* element */) {}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Events {
    pub click: EP,
    pub hover: EP,
    pub double_click: EP,
    pub long_press: EP,
    pub when_visible: EP,
    pub when_hidden: EP,
    pub when_focused: EP,
    pub when_unfocused: EP,
    pub when_apears: EP,
}

macro_rules! event_methods {
    ($($name:ident),*) => {
        $(
            pub fn $name(mut self, event_script: &str) -> Self {
                self.$name = EP::new(event_script);
                self
            }
        )*
    };
}

impl Events {
    pub fn is_available(&self) -> bool {
        self.click.is_available
            || self.hover.is_available
            || self.double_click.is_available
            || self.long_press.is_available
            || self.when_visible.is_available
            || self.when_hidden.is_available
            || self.when_focused.is_available
            || self.when_unfocused.is_available
            || self.when_apears.is_available
    }

    pub fn atach_script(&mut self, /* element */) {}

    event_methods!(
        click,
        hover,
        double_click,
        long_press,
        when_visible,
        when_hidden,
        when_focused,
        when_unfocused,
        when_apears
    );
}

