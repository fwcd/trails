use druid::{AppLauncher, WindowDesc, Widget, widget::Label};

fn main() {
    let window = WindowDesc::new(build_ui)
        .title("Trails");
    let data = ();
    AppLauncher::with_window(window)
        .launch(data)
        .expect("Failed to launch app");
}

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}
