use gpui::*;
use std::path::PathBuf;
use crate::theme::Theme;

pub struct Sidebar {
    pub entries: Vec<PathBuf>,
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .flex()
            .flex_col()
            .w(px(256.0))
            .bg(theme.sidebar_bg)
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .h(px(40.0))
                    .px_4()
                    .items_center()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_muted)
                            .child("EXPLORER")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .px_2()
                    .children(self.entries.iter().map(|p| {
                        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .hover(|s| s.bg(theme.background))
                            .child(name)
                    }))
            )
    }
}
