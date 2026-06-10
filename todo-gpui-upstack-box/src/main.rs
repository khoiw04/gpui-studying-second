use std::path::PathBuf;

use gpui::{
    App, Application, Bounds, ClickEvent, Context, FocusHandle, Focusable, KeyDownEvent,
    SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb,
    rgba, size,
};
use serde::{Deserialize, Serialize};

// ---------- Palette ----------
const BG: u32 = 0x16161e;
const PANEL: u32 = 0x1f1f2b;
const PANEL_HOVER: u32 = 0x2a2a3a;
const BORDER: u32 = 0x32324a;
const TEXT: u32 = 0xe6e6f0;
const MUTED: u32 = 0x8a8aa3;
const ACCENT: u32 = 0x7c6cf0;
const GREEN: u32 = 0x4ade80;
const RED: u32 = 0xf87171;

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    done: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Done,
}

impl Filter {
    fn label(self) -> &'static str {
        match self {
            Filter::All => "Tất cả",
            Filter::Active => "Đang làm",
            Filter::Done => "Hoàn thành",
        }
    }
}

struct TodoApp {
    todos: Vec<Todo>,
    input: String,
    filter: Filter,
    next_id: u64,
    focus_handle: FocusHandle,
}

fn store_path() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".todo-gpui.json")
}

impl TodoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let todos: Vec<Todo> = std::fs::read_to_string(store_path())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        let next_id = todos.iter().map(|t| t.id + 1).max().unwrap_or(0);
        Self {
            todos,
            input: String::new(),
            filter: Filter::All,
            next_id,
            focus_handle: cx.focus_handle(),
        }
    }

    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.todos) {
            let _ = std::fs::write(store_path(), json);
        }
    }

    fn add_todo(&mut self, cx: &mut Context<Self>) {
        let title = self.input.trim().to_string();
        if title.is_empty() {
            return;
        }
        self.todos.push(Todo {
            id: self.next_id,
            title,
            done: false,
        });
        self.next_id += 1;
        self.input.clear();
        self.save();
        cx.notify();
    }

    fn toggle(&mut self, id: u64, cx: &mut Context<Self>) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.done = !todo.done;
            self.save();
            cx.notify();
        }
    }

    fn delete(&mut self, id: u64, cx: &mut Context<Self>) {
        self.todos.retain(|t| t.id != id);
        self.save();
        cx.notify();
    }

    fn clear_done(&mut self, cx: &mut Context<Self>) {
        self.todos.retain(|t| !t.done);
        self.save();
        cx.notify();
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let ks = &event.keystroke;
        if ks.modifiers.control || ks.modifiers.alt || ks.modifiers.platform {
            return;
        }
        match ks.key.as_str() {
            "enter" => self.add_todo(cx),
            "backspace" => {
                self.input.pop();
                cx.notify();
            }
            "escape" => {
                self.input.clear();
                cx.notify();
            }
            _ => {
                if let Some(ch) = &ks.key_char {
                    self.input.push_str(ch);
                    cx.notify();
                }
            }
        }
    }

    // ---------- UI pieces ----------

    fn render_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let empty = self.input.is_empty();
        div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_3()
            .bg(rgb(PANEL))
            .border_1()
            .border_color(rgb(BORDER))
            .rounded_lg()
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .text_color(if empty { rgb(MUTED) } else { rgb(TEXT) })
                    .child(if empty {
                        SharedString::from("Nhập việc cần làm rồi nhấn Enter…")
                    } else {
                        SharedString::from(self.input.clone())
                    })
                    // text cursor
                    .child(div().w(px(2.)).h(px(18.)).ml(px(1.)).bg(rgb(ACCENT))),
            )
            .child(
                div()
                    .id("add")
                    .px_3()
                    .py_1()
                    .rounded_md()
                    .bg(rgb(ACCENT))
                    .text_color(rgb(0xffffff))
                    .text_sm()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(0x8f80f5)))
                    .child("Thêm")
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| this.add_todo(cx))),
            )
    }

    fn render_filters(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let remaining = self.todos.iter().filter(|t| !t.done).count();
        let has_done = self.todos.iter().any(|t| t.done);

        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(MUTED))
                    .child(format!("{remaining} chưa xong")),
            )
            .child(div().flex_1())
            .children([Filter::All, Filter::Active, Filter::Done].map(|f| {
                let active = self.filter == f;
                div()
                    .id(f.label())
                    .px_2p5()
                    .py_1()
                    .rounded_md()
                    .text_sm()
                    .cursor_pointer()
                    .when(active, |s| s.bg(rgb(ACCENT)).text_color(rgb(0xffffff)))
                    .when(!active, |s| {
                        s.text_color(rgb(MUTED)).hover(|s| s.bg(rgb(PANEL_HOVER)))
                    })
                    .child(f.label())
                    .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                        this.filter = f;
                        cx.notify();
                    }))
            }))
            .when(has_done, |s| {
                s.child(
                    div()
                        .id("clear-done")
                        .px_2p5()
                        .py_1()
                        .rounded_md()
                        .text_sm()
                        .text_color(rgb(RED))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(PANEL_HOVER)))
                        .child("Dọn xong")
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| this.clear_done(cx))),
                )
            })
    }

    fn render_item(&self, todo: &Todo, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let id = todo.id;
        let done = todo.done;
        div()
            .id(("todo", id))
            .flex()
            .items_center()
            .gap_3()
            .px_4()
            .py_3()
            .bg(rgb(PANEL))
            .border_1()
            .border_color(rgb(BORDER))
            .rounded_lg()
            .cursor_pointer()
            .hover(|s| s.bg(rgb(PANEL_HOVER)))
            .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| this.toggle(id, cx)))
            // checkbox
            .child(
                div()
                    .size_5()
                    .flex_none()
                    .rounded_full()
                    .border_2()
                    .border_color(if done { rgb(GREEN) } else { rgb(MUTED) })
                    .when(done, |s| s.bg(rgb(GREEN)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(BG))
                    .text_xs()
                    .when(done, |s| s.child("✓")),
            )
            // title
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .text_color(if done { rgb(MUTED) } else { rgb(TEXT) })
                    .when(done, |s| s.line_through())
                    .child(SharedString::from(todo.title.clone())),
            )
            // delete button
            .child(
                div()
                    .id(("delete", id))
                    .flex_none()
                    .px_2()
                    .rounded_md()
                    .text_color(rgb(MUTED))
                    .hover(|s| s.text_color(rgb(RED)).bg(rgb(BG)))
                    .child("✕")
                    .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                        cx.stop_propagation();
                        this.delete(id, cx);
                    })),
            )
    }
}

impl Focusable for TodoApp {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TodoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible: Vec<Todo> = self
            .todos
            .iter()
            .filter(|t| match self.filter {
                Filter::All => true,
                Filter::Active => !t.done,
                Filter::Done => t.done,
            })
            .cloned()
            .collect();

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .flex()
            .flex_col()
            .size_full()
            .gap_4()
            .p_6()
            .bg(rgb(BG))
            .text_color(rgb(TEXT))
            .font_family("DejaVu Sans")
            // header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .size_8()
                            .flex_none()
                            .rounded_md()
                            .bg(rgb(ACCENT))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xffffff))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("✓"),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Việc cần làm"),
                    ),
            )
            .child(self.render_input(cx))
            .child(self.render_filters(cx))
            // list
            .child(
                div()
                    .id("todo-list")
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .overflow_y_scroll()
                    .when(visible.is_empty(), |s| {
                        s.items_center().justify_center().child(
                            div()
                                .text_color(rgba(MUTED << 8 | 0x99))
                                .child(match self.filter {
                                    Filter::All => "Chưa có việc nào — thêm một việc ở trên ↑",
                                    Filter::Active => "Không còn việc đang làm ✓",
                                    Filter::Done => "Chưa hoàn thành việc nào",
                                }),
                        )
                    })
                    .children(visible.iter().map(|t| self.render_item(t, cx))),
            )
    }
}

fn main() {
    env_logger::init();
    Application::new().run(|cx: &mut App| {
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        let bounds = Bounds::centered(None, size(px(460.), px(640.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Todo".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                cx.activate(true);
                cx.new(|cx| {
                    let app = TodoApp::new(cx);
                    app.focus_handle.focus(window);
                    app
                })
            },
        )
        .unwrap();
    });
}
