
pub mod factory {

    use crate::render::gui::{Button, Checkbox, GuiFactory};
    pub struct MacFactory {}

    impl GuiFactory for MacFactory {
        
        type B = MacButton; // Replace with your Button type definition
        type C = MacCheckbox;

        fn create_button(&self, label: &str) -> MacButton {
            // Implement logic to create a Macdows button with the given text
            MacButton { label: label.to_string() }
        }
        fn create_checkbox(&self, label: &str) -> MacCheckbox {
            // Implement logic to create a Macdows button with the given text
            MacCheckbox { label: label.to_string()}
        }
    }
    impl Button for MacButton {
        fn press(&self) {
            println!("{} in Mac Pressed.", self.label)  
        }
    }  

    impl Checkbox for MacCheckbox {
        fn switch(&self) {
            println!("{} in Mac Switched.", self.label)  
        }
    }  
        
    pub struct MacButton {
        pub label: String,
    }
    pub struct MacCheckbox {
        pub label: String,
    }
 }
