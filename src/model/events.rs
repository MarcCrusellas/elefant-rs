#![allow(unused)]

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventPrototype {
    pub is_available: bool,
    pub event_script: Option<String>,
}
pub type EP = EventPrototype;

impl Default for EventPrototype {
    fn default() -> Self {
        EventPrototype {
            is_available: false,
            event_script: None,
        }
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl Default for Events {
    fn default() -> Self {
        Events {
            click: EP::default(),
            hover: EP::default(),
            double_click: EP::default(),
            long_press: EP::default(),
            when_visible: EP::default(),
            when_hidden: EP::default(),
            when_focused: EP::default(),
            when_unfocused: EP::default(),
            when_apears: EP::default(),
        }
    }
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
    pub fn new(
        click: &str,
        hover: &str,
        double_click: &str,
        long_press: &str,
        when_visible: &str,
        when_hidden: &str,
        when_focused: &str,
        when_unfocused: &str,
        when_apears: &str,
    ) -> Self {
        Events {
            click: EventPrototype::new(click),
            hover: EventPrototype::new(hover),
            double_click: EventPrototype::new(double_click),
            long_press: EventPrototype::new(long_press),
            when_visible: EventPrototype::new(when_visible),
            when_hidden: EventPrototype::new(when_hidden),
            when_focused: EventPrototype::new(when_focused),
            when_unfocused: EventPrototype::new(when_unfocused),
            when_apears: EventPrototype::new(when_apears),
        }
    }

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

