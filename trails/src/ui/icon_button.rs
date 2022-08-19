use druid::{widget::{LabelText, Button, Label}, TextAlignment, Data};

pub fn icon_button<T>(text: impl Into<LabelText<T>>, icon_size: f64) -> Button<T> where T: Data {
    Button::from_label(
        Label::new(text)
            .with_text_size(icon_size)
            .with_text_alignment(TextAlignment::Start)
    )
}
