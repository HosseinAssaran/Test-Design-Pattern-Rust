//! The code demonstrates that it doesn't depend on a concrete
//! factory implementation.
pub mod gui;

use gui::GuiFactory;

// Renders GUI. Factory object must be passed as a parameter to such the
// generic function with factory invocation to utilize static dispatch.
pub fn render(factory: impl GuiFactory) {
    let button1 = factory.create_button("button1");
    let button2 = factory.create_button("button2");
    let checkbox1 = factory.create_checkbox("chekcbox1");
    let checkbox2 = factory.create_checkbox("chekcbox2");

    use gui::{Button, Checkbox};

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}