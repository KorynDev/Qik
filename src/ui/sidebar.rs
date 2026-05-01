use gpui::*;
use gpui::prelude::*;
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
            .w(px(260.0))
            .bg(theme.sidebar_bg)
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .h(px(36.0))
                    .px_4()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_muted)
                            .child("PROJECT")
                    )
                    .child(
                        div()
                            .text_color(theme.text_muted)
                            .child("...")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .py_2()
                    .children(self.entries.iter().map(|p| {
                        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let is_dir = p.is_dir();
                        
                        div()
                            .flex()
                            .items_center()
                            .px_4()
                            .py_1()
                            .text_sm()
                            .text_color(theme.text)
                            .hover(|s| s.bg(theme.border_variant))
                            .child(
                                div()
                                    .w(px(16.0))
                                    .text_color(if is_dir { theme.accent } else { theme.text_muted })
                                    .mr_2()
                                    .child(if is_dir { "󰉋" } else { "󰈚" }) // Nerd Font symbols if supported, or generic
                            )
                            .child(name)
                    }))
            )
    }
}
