use gpui::*;

struct RootView {
    count: isize,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .bg(white())
            .child(
                div()
                    .id("decrement_button")
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_8()
                    .rounded_md()
                    .border_1()
                    .cursor_pointer()
                    .border_color(black())
                    .child("-")
                    .hover(|style| style.bg(red()))
                    .on_click(cx.listener(Self::decrement)),
            )
            .child(
                div()
                    .min_w_16()
                    .text_3xl()
                    .text_center()
                    .child(self.count.to_string()),
            )
            .child(
                div()
                    .id("increment_button")
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_8()
                    .rounded_md()
                    .border_1()
                    .cursor_pointer()
                    .border_color(black())
                    .child("+")
                    .hover(|style| style.bg(red()))
                    .on_click(cx.listener(Self::increment)),
            )
    }
}

impl RootView {
    fn increment(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();
    }
    fn decrement(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count -= 1;
        cx.notify();
    }
}

fn main() {
    Application::new().run(|app: &mut App| {
        app.open_window(
            WindowOptions::default(),
            |_cx: &mut Window, app: &mut App| app.new(|_cx| RootView { count: 0 }),
        )
        .unwrap();
    });
}
