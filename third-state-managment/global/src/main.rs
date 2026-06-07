use gpui::{Application, BorrowAppContext, Global, UpdateGlobal};

#[derive(Default)]
pub struct SomeState {
    some_value: bool,
}

impl Global for SomeState {}

fn main() {
    Application::new().run(|app| {
        // init
        app.set_global::<SomeState>(SomeState { some_value: true });

        // access
        let some_state = app.global::<SomeState>();

        // mutate
        let some_state = app.global_mut::<SomeState>();
        some_state.some_value = false;

        // access optional
        let some_state = app.try_global::<SomeState>();

        // check existence
        let is_set_bool = app.has_global::<SomeState>();

        // remove
        app.remove_global::<SomeState>();

        // set to default
        app.default_global::<SomeState>();

        // update
        app.update_global::<SomeState, _>(|some_state, _app| {
            some_state.some_value = false;
        });

        // listener
        app.observe_global::<SomeState>(|_app| {
            // Global update callback
        })
        .detach();
    });
}
