use gpui::{App, AppContext, Application};

pub struct SomeState {
    some_value: bool,
}

fn main() {
    Application::new().run(|app: &mut App| {
        let entity = app.new(|_cx| SomeState { some_value: true });

        entity.update(app, |this, cx| {
            this.some_value = false;

            cx.notify();
        });
    });
}
