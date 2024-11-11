pub mod auth_methods;
pub mod collections;
pub mod environments;
pub mod main_content;
pub mod request_methods;
use std::path::Path;

use floem::{window::{WindowConfig, Icon}, Application, kurbo::Size};
use main_content::full_window_view;

fn main() {
    let png_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/ferris.png");
    let png_icon = load_png_icon(Path::new(png_icon_path));

    let wcfg = WindowConfig::default()
        .size(Size::new(800.0, 250.0))
        .title("Window Icon Example")
        .window_icon(png_icon)
        ;
  


    Application::new()
        .window(move |_| full_window_view(), Some(wcfg.title("Atticus")))
        .run()
}

fn load_png_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
