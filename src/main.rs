use druid::{AppLauncher, WindowDesc, Widget, widget::Label};

fn main() {
    let window = WindowDesc::new(build_ui)
        .title("Trails")
        .window_size((800.0, 600.0));

    let initial_state = ();

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch app");
}

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}
