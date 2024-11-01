pub mod auth_methods;
pub mod collections;
pub mod environments;
pub mod main_content;
pub mod request_methods;

use floem::{window::WindowConfig, Application};
use main_content::full_window_view;

fn main() {
    let wcfg = WindowConfig::default();
    Application::new()
        .window(move |_| full_window_view(), Some(wcfg.title("Atticus")))
        .run()
}
