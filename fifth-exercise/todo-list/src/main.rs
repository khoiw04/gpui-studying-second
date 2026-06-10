use gpui::*;
use gpui_component::{
    input::{InputEvent, *},
    label::*,
    scroll::*,
    *,
};

#[derive(Clone)]
struct Data {
    id: usize,
    title: SharedString,
}

struct TodoList {
    data: Vec<Data>,
}

impl Global for TodoList {}

struct HelloWorld {
    input: Entity<InputState>,
    _sub: Subscription,
}

impl HelloWorld {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));

        let sub = cx.subscribe_in(
            &input,
            window,
            |_view, state, event, _window, cx| match event {
                InputEvent::PressEnter {
                    secondary: _,
                    shift: _,
                } => {
                    let title = state.read(cx).value();

                    cx.update_global::<TodoList, _>(|some_state, _app| {
                        let id = some_state.data.iter().len();
                        some_state.data.push(Data { id, title });
                    });
                }
                _ => (),
            },
        );

        Self { input, _sub: sub }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let list = cx.global::<TodoList>();

        div()
            .v_flex()
            .size_full()
            .items_center()
            .justify_start()
            .child("Hello, World!")
            .bg(cx.theme().background)
            .child(Input::new(&self.input).max_w_1_4())
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .size_full()
                    .items_center()
                    .justify_start()
                    .overflow_y_scrollbar()
                    .children(list.data.iter().map(|data| {
                        div()
                            .p_4()
                            .w_full()
                            .id(data.id)
                            .max_w_1_5()
                            .rounded_xl()
                            .bg(cx.theme().secondary)
                            .child(Label::new(data.title.clone()).text_center())
                    })),
            )
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);

        cx.open_window(WindowOptions::default(), |window, app| {
            TodoList::set_global(app, TodoList { data: Vec::new() });
            let view = app.new(|cx| HelloWorld::new(window, cx));

            app.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
