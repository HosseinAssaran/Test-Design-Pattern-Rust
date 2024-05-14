pub trait Button {
    fn press(&self);
}

pub trait Checkbox {
    fn switch(&self);
}

/// Abstract Factory defined using generics.
pub trait GuiFactory {
    type B: Button;
    type C: Checkbox;

    fn create_button(&self, label: &str) -> Self::B;
    fn create_checkbox(&self, label: &str) -> Self::C;
}

/// Abstract Factory defined using Box pointer.
pub trait GuiFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}