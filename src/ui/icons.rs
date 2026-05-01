use gpui::*;
use gpui::prelude::*;

pub fn file_icon(color: Hsla) -> impl IntoElement {
    div()
        .size(px(14.0))
        .relative()
        .child(
            div()
                .size_full()
                .border_1()
                .border_color(color)
                .rounded_sm()
        )
        .child(
            div()
                .absolute()
                .top_0()
                .right_0()
                .size(px(4.0))
                .bg(color)
                .rounded_bl_sm()
        )
}

pub fn folder_icon(color: Hsla) -> impl IntoElement {
    div()
        .w(px(16.0))
        .h(px(12.0))
        .relative()
        .child(
            div()
                .size_full()
                .bg(color.opacity(0.2))
                .border_1()
                .border_color(color)
                .rounded_sm()
        )
        .child(
            div()
                .absolute()
                .top(px(-2.0))
                .left_0()
                .w(px(6.0))
                .h(px(3.0))
                .bg(color)
                .rounded_t_sm()
        )
}

pub fn search_icon(color: Hsla) -> impl IntoElement {
    div()
        .size(px(16.0))
        .relative()
        .child(
            div()
                .size(px(12.0))
                .border_2()
                .border_color(color)
                .rounded_full()
        )
        .child(
            div()
                .absolute()
                .bottom_0()
                .right_0()
                .w(px(6.0))
                .h(px(2.0))
                .bg(color)
                .rotate(Degrees(45.0))
        )
}

pub fn settings_icon(color: Hsla) -> impl IntoElement {
    div()
        .size(px(16.0))
        .border_2()
        .border_color(color)
        .rounded_full()
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .size(px(4.0))
                .bg(color)
                .rounded_full()
        )
}

pub fn project_icon(color: Hsla) -> impl IntoElement {
    div()
        .size(px(16.0))
        .border_2()
        .border_color(color)
        .rounded_sm()
        .flex()
        .flex_col()
        .p_px()
        .gap_px()
        .child(div().h_px().w_full().bg(color))
        .child(div().h_px().w_full().bg(color))
}

pub fn close_icon(color: Hsla) -> impl IntoElement {
    div()
        .size(px(10.0))
        .relative()
        .child(
            div()
                .absolute()
                .top_1_2()
                .left_0()
                .w_full()
                .h_px()
                .bg(color)
                .rotate(Degrees(45.0))
        )
        .child(
            div()
                .absolute()
                .top_1_2()
                .left_0()
                .w_full()
                .h_px()
                .bg(color)
                .rotate(Degrees(-45.0))
        )
}
