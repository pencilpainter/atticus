pub mod auth_methods;
pub mod collections;
pub mod environments;
pub mod main_content;
pub mod request_methods;
//use std::path::Path;

use floem::{
    kurbo::Size,
    window::{Icon, WindowConfig},
    Application,
};
use main_content::full_window_view;

fn main() {
    let png_icon = load_svg_icon(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Created with Inkscape (http://www.inkscape.org/) -->

<svg
   width="138.76178mm"
   height="146.08646mm"
   viewBox="0 0 138.76178 146.08646"
   version="1.1"
   id="svg1"
   xml:space="preserve"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg"><defs
     id="defs1"><linearGradient
       id="swatch3"><stop
         style="stop-color:#191d63;stop-opacity:0;"
         offset="0"
         id="stop3" /></linearGradient></defs><g
     id="layer1"
     transform="translate(-36.21642,-53.71424)"><ellipse
       style="fill:#ffffff;fill-opacity:1;stroke-width:0.264583"
       id="path6"
       cx="105.59731"
       cy="126.75747"
       rx="69.38089"
       ry="73.043228" /><path
       style="fill:#191d63;fill-opacity:1;stroke-width:0.264583"
       d="m 104.98692,53.714239 -56.562718,107.021551 7.731594,11.39393 26.450194,0.40693 22.78786,-43.94801 21.97401,43.54108 h 29.7056 l 4.88311,-9.76622 z"
       id="path2" /><path
       style="fill:#ffffff;fill-opacity:1;stroke-width:0.191533"
       d="m 105.24447,81.792138 -42.984783,77.473252 11.533007,-0.2428 30.843956,-50.29811 28.2891,51.19746 11.53301,-2.24152 z"
       id="path3" /><path
       style="fill:#ffffff;fill-opacity:1;stroke-width:0.264583"
       d="m 85.8614,171.7228 h 15.05626"
       id="path4" /><path
       style="fill:#191d63;fill-opacity:1;stroke-width:0.264583"
       d="m 85.047547,171.7228 c 12.614708,0.40692 12.207782,-0.40693 12.207782,-0.40693 v -20.3463 l 16.277041,0.81385 -0.40693,18.31167 11.80086,0.40693 -19.12552,21.56708 z"
       id="path5" /><circle
       style="fill:#191d63;fill-opacity:1;stroke-width:0.264583"
       id="path7"
       cx="156.05614"
       cy="115.36354"
       r="0.20346303" /></g></svg>
"#,
    );

    let wcfg = WindowConfig::default()
        .size(Size::new(800.0, 660.0))
        .title("Window Icon Example")
        .window_icon(png_icon);

    Application::new()
        .window(move |_| full_window_view(), Some(wcfg.title("Atticus")))
        .run()
}

//fn load_png_icon(path: &Path) -> Icon {
//    let (icon_rgba, icon_width, icon_height) = {
//        let image = image::open(path)
//            .expect("Failed to open icon path")
//            .into_rgba8();
//        let (width, height) = image.dimensions();
//        let rgba = image.into_raw();
//        (rgba, width, height)
//    };
//    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
//}

fn load_svg_icon(svg: &str) -> Icon {
    let svg = nsvg::parse_str(svg, nsvg::Units::Pixel, 96.0).unwrap();
    let (icon_width, icon_height, icon_rgba) = svg.rasterize_to_raw_rgba(1.0).unwrap();
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
