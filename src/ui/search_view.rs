use gpui::*;
use crate::theme::Theme;

pub struct SearchView {
    pub search_query: String,
    pub replace_query: String,
}

impl Render for SearchView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .flex()
            .flex_col()
            .p_4()
            .bg(theme.sidebar_bg)
            .border_b_1()
            .border_color(theme.border)
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_grow()
                            .bg(theme.background)
                            .p_1()
                            .rounded_sm()
                            .child("Find")
                    )
                    .child(
                        div()
                            .bg(theme.accent)
                            .px_2()
                            .rounded_sm()
                            .child("All")
                    )
            )
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_grow()
                            .bg(theme.background)
                            .p_1()
                            .rounded_sm()
                            .child("Replace")
                    )
                    .child(
                        div()
                            .bg(theme.border)
                            .px_2()
                            .rounded_sm()
                            .child("Replace All")
                    )
            )
    }
}
