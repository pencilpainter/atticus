use floem::{window::WindowConfig, Application};

use left_sidebar::full_window_view;
pub mod left_sidebar;
pub mod request_methods;

fn main() {
    let wcfg = WindowConfig::default();
    Application::new()
        .window(move |_| full_window_view(), Some(wcfg.title("Atticus")))
        .run()
}
