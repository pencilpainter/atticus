use std::rc::Rc;

use floem::{
    peniko::Color,
    reactive::{create_signal, ReadSignal, SignalGet, SignalUpdate, WriteSignal},
    style::{CursorStyle, Position},
    text::Weight,
    views::{
        container, editor::text::Document, h_stack, label, scroll, tab, text_editor, v_stack,
        Decorators,
    },
    IntoView,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
enum Tab {
    Response,
    Headers,
    Stats,
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Tab::Response => write!(f, "Response"),
            Tab::Headers => write!(f, "Headers"),
            Tab::Stats => write!(f, "Stats"),
        }
    }
}

fn tab_button(
    this_tab: Tab,
    position: usize,
    set_active_tab: WriteSignal<usize>,
    active_tab: ReadSignal<usize>,
) -> impl IntoView {
    label(move || this_tab.clone())
        .keyboard_navigable()
        .on_click_stop(move |_| {
            set_active_tab.update(|v: &mut usize| {
                *v = position;
            });
        })
        .style(move |s| {
            s.width(70)
                .hover(|s| s.font_weight(Weight::BOLD).cursor(CursorStyle::Pointer))
                .apply_if(active_tab.get() == position, |s| {
                    s.font_weight(Weight::BOLD)
                })
        })
}

const TABBAR_HEIGHT: f64 = 37.0;
const CONTENT_PADDING: f64 = 10.0;

pub fn tab_navigation_view(
    doc: Rc<dyn Document>,
    headers: Rc<dyn Document>,
    stats: Rc<dyn Document>,
) -> impl IntoView {
    let tabs = vec![Tab::Response, Tab::Headers, Tab::Stats]
        .into_iter()
        .collect::<im::Vector<Tab>>();
    let (_tabs, _set_tabs) = create_signal(tabs);
    let (active_tab, set_active_tab) = create_signal(0);

    let tabs_bar = h_stack((
        tab_button(Tab::Response, 0, set_active_tab, active_tab),
        tab_button(Tab::Headers, 1, set_active_tab, active_tab),
        tab_button(Tab::Stats, 2, set_active_tab, active_tab),
    ))
    .style(|s| {
        s.flex_row()
            .width_full()
            .height(TABBAR_HEIGHT)
            .row_gap(5)
            .padding(CONTENT_PADDING)
            .border_bottom(1)
            .border_color(Color::from_rgba8(209, 209, 209, 255))
    });

    let main_content = container(
        scroll(
            tab(
                move || active_tab.get(),
                move || _tabs.get(),
                |it| *it,
                move |it| match it {
                    Tab::Response => text_editor("")
                        .use_doc(doc.clone())
                        .style(|s| s.width_full().height_full()),
                    Tab::Headers => text_editor("")
                        .use_doc(headers.clone())
                        .style(|s| s.width_full().height_full()),
                    Tab::Stats => text_editor("")
                        .use_doc(stats.clone())
                        .style(|s| s.width_full().height_full()),
                },
            )
            .style(|s| {
                s.padding(CONTENT_PADDING)
                    .padding_bottom(10.0)
                    .width_full()
                    .height_full()
            }),
        )
        .style(|s| s.flex_col().flex_basis(0).min_width(0).flex_grow(1.0)),
    )
    .style(|s| {
        s.flex_col()
            .flex_basis(0)
            .min_width(0)
            .flex_grow(1.0)
            .position(Position::Absolute)
            .inset_top(TABBAR_HEIGHT)
            .inset_bottom(0.0)
            .width_full()
    });

    v_stack((tabs_bar, main_content)).style(|s| s.width_full().height_full())
}
