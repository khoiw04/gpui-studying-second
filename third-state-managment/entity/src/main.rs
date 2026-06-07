// https://matinaniss.github.io/gpui-book/state-management/entity.html
use gpui::{AppContext, Application};

pub struct SomeState {
    some_value: bool,
}

fn main() {
    Application::new().run(|app| {
        let entity = app.new(|_cx| SomeState { some_value: true });

        let weak_entity = entity.downgrade();
    });
}
