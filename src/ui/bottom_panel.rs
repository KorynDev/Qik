use gpui::*;
use crate::theme::Theme;

pub struct BottomPanel {
    pub active_tab: String,
}

impl Render for BottomPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .flex()
            .flex_col()
            .h(px(192.0))
            .bg(theme.status_bg)
            .border_t_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .h(px(36.0))
                    .px_4()
                    .gap(px(16.0))
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .child("TERMINAL")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("OUTPUT")
                    )
            )
            .child(
                div()
                    .p_4()
                    .text_sm()
                    .font_family("JetBrains Mono")
                    .child("$ cargo check\n  Finished dev [unoptimized + debuginfo] target(s) in 0.05s")
            )
    }
}
