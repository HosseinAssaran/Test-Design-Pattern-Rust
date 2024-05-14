mod render;

use render::render;
mod macos_gui;
mod windows_gui;

use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;

fn main() {
    let windows = true;

    if windows {
        render(WindowsFactory {});
    } else {
        render(MacFactory {});
    }
}
