use druid_widget_nursery::navigator::{View};


// Here you define your view. It can be any type that implements `Hash`. You can define an Enum
// instead and use that to define your views instead of a string
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UiView {
    name: String,
}

impl UiView {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

// implements the view trait for your view type
impl View for UiView {}