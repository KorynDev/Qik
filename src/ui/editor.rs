use gpui::*;
use ropey::Rope;
use crate::theme::Theme;

pub struct Editor {
    pub content: Rope,
    pub cursor_char: usize,
}

impl Editor {
    pub fn new(content: &str) -> Self {
        Self {
            content: Rope::from_str(content),
            cursor_char: 0,
        }
    }

    pub fn cursor_coords(&self) -> (usize, usize) {
        let line = self.content.char_to_line(self.cursor_char);
        let col = self.cursor_char - self.content.line_to_char(line);
        (line, col)
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let (cursor_line, cursor_col) = self.cursor_coords();
        
        div()
            .flex()
            .flex_grow()
            .bg(theme.editor_bg)
            .text_color(theme.text)
            .font_family("JetBrains Mono")
            .text_size(px(14.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(56.0))
                    .h_full()
                    .bg(theme.sidebar_bg)
                    .text_color(theme.line_number)
                    .py_4()
                    .items_end()
                    .pr_4()
                    .children((0..self.content.len_lines()).map(|i| {
                        div()
                            .h(px(24.0))
                            .child((i + 1).to_string())
                            .when(i == cursor_line, |s| s.text_color(theme.text))
                    }))
            )
            .child(
                div()
                    .flex()
                    .flex_grow()
                    .relative()
                    .p_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(self.content.lines().enumerate().map(|(i, line)| {
                                let is_active = i == cursor_line;
                                div()
                                    .flex()
                                    .h(px(24.0))
                                    .relative()
                                    .when(is_active, |s| s.bg(hsla(0.0, 0.0, 1.0, 0.05)))
                                    .child(
                                        div()
                                            .w(px(4.0))
                                            .h_full()
                                            .absolute()
                                            .left(px(-16.0))
                                            .when(i % 10 == 0, |s| s.bg(theme.git_modified))
                                    )
                                    .child(line.to_string().replace("\n", ""))
                                    .when(is_active, |s| {
                                        s.child(
                                            div()
                                                .absolute()
                                                .left(px(cursor_col as f32 * 8.4))
                                                .w_px()
                                                .h_full()
                                                .bg(theme.cursor)
                                        )
                                    })
                            }))
                    )
            )
    }
}
