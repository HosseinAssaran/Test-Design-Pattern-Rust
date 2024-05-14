
pub mod factory {

    use crate::render::gui::{Button, Checkbox, GuiFactory};
    pub struct WindowsFactory {}

    impl GuiFactory for WindowsFactory {
        
        type B = WinButton; // Replace with your Button type definition
        type C = WinCheckbox;

        fn create_button(&self, label: &str) -> WinButton {
            // Implement logic to create a Windows button with the given text
            WinButton { label: label.to_string() }
        }
        fn create_checkbox(&self, label: &str) -> WinCheckbox {
            // Implement logic to create a Windows button with the given text
            WinCheckbox { label: label.to_string() }
        }
    }

    impl Button for WinButton {
        fn press(&self) {
            println!("{} in Windows Pressed.", self.label)  
        }
    }  

    impl Checkbox for WinCheckbox {
        fn switch(&self) {
            println!("{} in Windows Switched.", self.label)  
        }
    }  
        
    pub struct WinButton {
        pub label: String,
    }
    pub struct WinCheckbox {
        pub label: String,
    }
 }
