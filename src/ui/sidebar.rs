use gpui::*;
use std::path::PathBuf;
use crate::theme::Theme;
use crate::ui::icons;

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
            .border_color(theme.border_variant)
            .child(
                div()
                    .flex()
                    .h(px(38.0))
                    .px_4()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_muted)
                            .child("PROJECT")
                    )
                    .child(
                        div()
                            .text_color(theme.text_muted)
                            .child("•••")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .py_1()
                    .children(self.entries.iter().map(|p| {
                        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let is_dir = p.is_dir();
                        
                        div()
                            .flex()
                            .items_center()
                            .px_4()
                            .py(px(3.0))
                            .text_size(px(13.0))
                            .text_color(theme.text)
                            .hover(|s| s.bg(theme.border_variant))
                            .child(
                                div()
                                    .w(px(20.0))
                                    .flex()
                                    .items_center()
                                    .child(if is_dir { 
                                        icons::folder_icon(theme.accent) 
                                    } else { 
                                        icons::file_icon(theme.text_muted) 
                                    })
                            )
                            .child(name)
                    }))
            )
    }
}
