use gpui::*;
use gpui::prelude::*;
use ropey::Rope;
use crate::theme::Theme;

pub struct Editor {
    pub content: Rope,
    pub cursor_char: usize,
    pub focus_handle: FocusHandle,
}

impl Editor {
    pub fn new(content: &str, cx: &mut Context<Self>) -> Self {
        Self {
            content: Rope::from_str(content),
            cursor_char: 0,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn cursor_coords(&self) -> (usize, usize) {
        let line = self.content.char_to_line(self.cursor_char);
        let col = self.cursor_char - self.content.line_to_char(line);
        (line, col)
    }

    pub fn handle_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        match event.keystroke.key.as_str() {
            "backspace" => {
                if self.cursor_char > 0 {
                    self.content.remove(self.cursor_char - 1..self.cursor_char);
                    self.cursor_char -= 1;
                }
            }
            "left" => {
                if self.cursor_char > 0 {
                    self.cursor_char -= 1;
                }
            }
            "right" => {
                if self.cursor_char < self.content.len_chars() {
                    self.cursor_char += 1;
                }
            }
            "up" => {
                let (line, col) = self.cursor_coords();
                if line > 0 {
                    let prev_line_len = self.content.line(line - 1).len_chars();
                    let target_col = col.min(prev_line_len.saturating_sub(1));
                    self.cursor_char = self.content.line_to_char(line - 1) + target_col;
                }
            }
            "down" => {
                let (line, col) = self.cursor_coords();
                if line + 1 < self.content.len_lines() {
                    let next_line_len = self.content.line(line + 1).len_chars();
                    let target_col = col.min(next_line_len.saturating_sub(1));
                    self.cursor_char = self.content.line_to_char(line + 1) + target_col;
                }
            }
            "enter" => {
                self.content.insert(self.cursor_char, "\n");
                self.cursor_char += 1;
            }
            key if key.len() == 1 => {
                self.content.insert(self.cursor_char, key);
                self.cursor_char += 1;
            }
            _ => {}
        }
        cx.notify();
    }
}

impl Focusable for Editor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let (cursor_line, cursor_col) = self.cursor_coords();
        
        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .flex()
            .flex_grow()
            .bg(theme.editor_bg)
            .text_color(theme.text)
            .font_family("JetBrains Mono")
            .text_size(px(14.5))
            .line_height(px(24.0))
            .child(
                // Gutter
                div()
                    .flex()
                    .flex_col()
                    .w(px(64.0))
                    .h_full()
                    .bg(theme.sidebar_bg)
                    .border_r_1()
                    .border_color(theme.border_variant)
                    .py_4()
                    .items_end()
                    .pr_4()
                    .children((0..self.content.len_lines()).map(|i| {
                        let is_active = i == cursor_line;
                        div()
                            .h(px(24.0))
                            .text_size(px(12.0))
                            .text_color(if is_active { theme.text } else { theme.line_number })
                            .child((i + 1).to_string())
                    }))
            )
            .child(
                // Content area
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .relative()
                    .bg(theme.editor_bg)
                    .child(
                        // Active Line Highlight
                        div()
                            .absolute()
                            .top(px(cursor_line as f32 * 24.0 + 16.0))
                            .w_full()
                            .h(px(24.0))
                            .bg(hsla(0.0, 0.0, 1.0, 0.03))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .py_4()
                            .px_6()
                            .children(self.content.lines().enumerate().map(|(i, line)| {
                                let is_active = i == cursor_line;
                                div()
                                    .flex()
                                    .h(px(24.0))
                                    .relative()
                                    .child(
                                        div()
                                            .w(px(2.0))
                                            .h_full()
                                            .absolute()
                                            .left(px(-24.0))
                                            .when(i % 15 == 0, |s| s.bg(theme.git_modified))
                                            .when(i % 25 == 0, |s| s.bg(theme.git_added))
                                    )
                                    .child(line.to_string().replace("\n", "").replace("\r", ""))
                                    .when(is_active, |s| {
                                        s.child(
                                            div()
                                                .absolute()
                                                .left(px(cursor_col as f32 * 8.7))
                                                .w(px(2.0))
                                                .h_full()
                                                .bg(theme.cursor)
                                        )
                                    })
                            }))
                    )
            )
    }
}
