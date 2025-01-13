use floem::{
    peniko::Color,
    style::{Background, BorderColor, Outline, Style, TextColor, Transition},
    unit::DurationUnitExt,
    views::{
        dropdown::DropdownClass, ButtonClass, LabelClass, LabeledRadioButtonClass, TextInputClass,
    },
};

pub fn blue_style() -> Style {
    let main_color = Color::rgb8(230, 230, 230);
    let sixty_ms = Transition::ease_in_out(60.millis());
    let hundred_ms = Transition::ease_in_out(60.millis());

    let blue_button = Style::new()
        .background(main_color.clone())
        .color(Color::BLACK)
        .border(0.)
        .hover(|s| s.background(Color::rgb8(207, 215, 210)))
        .transition(TextColor, sixty_ms.clone())
        .transition(BorderColor, sixty_ms.clone())
        .transition(Background, sixty_ms.clone())
        .transition(Outline, hundred_ms.clone())
        .padding(5.0)
        .margin(1.0)
        .border_radius(0.);

    let blue_label = blue_button.clone();
    let blue_input = blue_button.clone();
    let blue_drop = blue_button.clone();
    let blue_radio = blue_button.clone();

    Style::new()
        .class(ButtonClass, move |_| blue_button)
        .class(TextInputClass, move |_| blue_input)
        .class(LabelClass, move |_| blue_label)
        .class(DropdownClass, move |_| blue_drop)
        .class(LabeledRadioButtonClass, move |_| blue_radio)
}
