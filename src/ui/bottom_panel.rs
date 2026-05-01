use gpui::*;
use gpui::prelude::*;
use crate::theme::Theme;
use crate::ui::icons;

pub struct BottomPanel {
    pub active_tab: String,
    pub history: Vec<String>,
}

impl BottomPanel {
    pub fn new() -> Self {
        Self {
            active_tab: "TERMINAL".to_string(),
            history: vec![
                "Compiling qik v0.1.0 (C:\\Users\\Tyson\\Desktop\\Qik)".to_string(),
                "    Finished dev [unoptimized + debuginfo] target(s) in 0.05s".to_string(),
                "     Running `target\\debug\\qik.exe`".to_string(),
            ],
        }
    }
}

impl Render for BottomPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        
        div()
            .flex()
            .flex_col()
            .h(px(240.0))
            .bg(theme.terminal_bg)
            .border_t_1()
            .border_color(theme.border)
            .child(
                // Tab Bar
                div()
                    .flex()
                    .h(px(34.0))
                    .bg(theme.toolbar_bg)
                    .border_b_1()
                    .border_color(theme.border)
                    .px_4()
                    .gap(px(20.0))
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .text_size(px(12.0))
                            .text_color(theme.text)
                            .border_b_2()
                            .border_color(theme.accent)
                            .h_full()
                            .child(icons::terminal_icon(theme.accent))
                            .child("TERMINAL")
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.text_muted)
                            .child("OUTPUT")
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(theme.text_muted)
                            .child("DEBUG CONSOLE")
                    )
            )
            .child(
                // Terminal Content
                div()
                    .flex_grow()
                    .p_4()
                    .text_size(px(13.0))
                    .font_family("JetBrains Mono")
                    .text_color(theme.text)
                    .overflow_y(Overflow::Scroll)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(self.history.iter().map(|line| {
                                div().child(line.clone())
                            }))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .child(div().text_color(theme.terminal_prompt).child("qik-shell >"))
                                    .child(
                                        div()
                                            .flex()
                                            .child("cargo check")
                                            .child(
                                                div()
                                                    .w(px(8.0))
                                                    .h(px(18.0))
                                                    .bg(theme.cursor)
                                                    .ml(px(2.0))
                                            )
                                    )
                            )
                    )
            )
    }
}
