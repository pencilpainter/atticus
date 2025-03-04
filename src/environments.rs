use floem::{
    peniko::Color,
    reactive::{create_rw_signal, SignalGet},
    views::{label, scroll, virtual_stack, Decorators},
    IntoView,
};

use crate::main_content::SIDEBAR_WIDTH;

const SIDEBAR_ITEM_HEIGHT: f64 = 21.0;

pub fn environment_view<T>() -> impl IntoView {
    let env_list: im::Vector<&str> = vec!["hi", "ehllo", "world"].into();
    let env_list = create_rw_signal(env_list);

    scroll({
        virtual_stack(
            move || env_list.get(),
            move |item| *item,
            move |item| {
                label(move || item.to_string()).style(move |s| {
                    s.padding(10.0)
                        .padding_top(3.0)
                        .padding_bottom(3.0)
                        .width(SIDEBAR_WIDTH)
                        .height(SIDEBAR_ITEM_HEIGHT)
                        .items_start()
                        .border_bottom(1.0)
                        .border_color(Color::from_rgb8(205, 205, 205))
                })
            },
        )
        .style(|s| s.flex_col().width(SIDEBAR_WIDTH + 1.0))
    })
    .style(|s| {
        s.width(SIDEBAR_WIDTH)
            .border_left(1.0)
            .border_top(1.0)
            .border_color(Color::from_rgb8(205, 205, 205))
    })
}
