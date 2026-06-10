use gpui::{
    App, AppContext, Application, Context, IntoElement, ParentElement, Render, Styled, Window,
    WindowOptions, div, white,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(white())
            .child("Hello world!")
    }
}

fn main() {
    Application::new().run(|app: &mut App| {
        app.open_window(WindowOptions::default(), |_window, app: &mut App| {
            app.new(|_cx| RootView)
        })
        .unwrap();
    });
}
