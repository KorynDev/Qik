use gpui::*;
use gpui::prelude::*;

pub fn file_icon(color: Hsla) -> AnyElement {
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
        .into_any_element()
}

pub fn folder_icon(color: Hsla) -> AnyElement {
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
        .into_any_element()
}

pub fn search_icon(color: Hsla) -> AnyElement {
    div()
        .size(px(16.0))
        .relative()
        .child(
            div()
                .size(px(11.0))
                .border_2()
                .border_color(color)
                .rounded_full()
        )
        .child(
            div()
                .absolute()
                .bottom(px(1.0))
                .right(px(1.0))
                .size(px(5.0))
                .bg(color)
        )
        .into_any_element()
}

pub fn settings_icon(color: Hsla) -> AnyElement {
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
        .into_any_element()
}

pub fn project_icon(color: Hsla) -> AnyElement {
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
        .into_any_element()
}

pub fn close_icon(color: Hsla) -> AnyElement {
    div()
        .size(px(10.0))
        .flex()
        .items_center()
        .justify_center()
        .text_color(color)
        .text_size(px(10.0))
        .child("×")
        .into_any_element()
}
