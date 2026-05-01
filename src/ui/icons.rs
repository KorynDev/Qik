use gpui::*;
use gpui::prelude::*;

pub fn file_icon(color: Hsla) -> AnyElement {
    div()
        .size(px(16.0))
        .relative()
        .child(
            // Main body
            div()
                .size_full()
                .bg(color.opacity(0.1))
                .border_1()
                .border_color(color.opacity(0.4))
                .rounded_sm()
        )
        .child(
            // Top right fold
            div()
                .absolute()
                .top_0()
                .right_0()
                .size(px(5.0))
                .bg(color.opacity(0.3))
                .rounded_bl_sm()
        )
        .child(
            // Internal lines for "file" look
            div()
                .absolute()
                .top(px(6.0))
                .left(px(3.0))
                .w(px(8.0))
                .h(px(1.0))
                .bg(color.opacity(0.2))
        )
        .child(
            div()
                .absolute()
                .top(px(9.0))
                .left(px(3.0))
                .w(px(6.0))
                .h(px(1.0))
                .bg(color.opacity(0.2))
        )
        .into_any_element()
}

pub fn folder_icon(color: Hsla) -> AnyElement {
    div()
        .w(px(18.0))
        .h(px(14.0))
        .relative()
        .child(
            // Back layer
            div()
                .size_full()
                .bg(color.opacity(0.15))
                .border_1()
                .border_color(color.opacity(0.5))
                .rounded_sm()
        )
        .child(
            // Tab
            div()
                .absolute()
                .top(px(-3.0))
                .left_0()
                .w(px(8.0))
                .h(px(4.0))
                .bg(color.opacity(0.3))
                .border_1()
                .border_color(color.opacity(0.5))
                .rounded_t_sm()
        )
        .child(
            // Front flap overlay
            div()
                .absolute()
                .bottom_0()
                .left_0()
                .w_full()
                .h(px(8.0))
                .bg(color.opacity(0.1))
                .border_t_1()
                .border_color(color.opacity(0.3))
        )
        .into_any_element()
}

pub fn search_icon(color: Hsla) -> AnyElement {
    div()
        .size(px(18.0))
        .relative()
        .child(
            // Ring
            div()
                .size(px(13.0))
                .border_2()
                .border_color(color.opacity(0.7))
                .rounded_full()
        )
        .child(
            // Handle
            div()
                .absolute()
                .bottom(px(1.0))
                .right(px(1.0))
                .w(px(6.0))
                .h(px(2.5))
                .bg(color.opacity(0.7))
                .rounded_full()
        )
        .into_any_element()
}

pub fn settings_icon(color: Hsla) -> AnyElement {
    div()
        .size(px(18.0))
        .relative()
        .child(
            // Outer ring
            div()
                .size_full()
                .border_2()
                .border_color(color.opacity(0.6))
                .rounded_full()
        )
        .child(
            // Internal gear teeth (simulated with 4 dots)
            div()
                .absolute()
                .top_1_2()
                .left_1_2()
                .size(px(6.0))
                .ml(px(-3.0))
                .mt(px(-3.0))
                .bg(color.opacity(0.8))
                .rounded_full()
        )
        .into_any_element()
}

pub fn project_icon(color: Hsla) -> AnyElement {
    div()
        .size(px(18.0))
        .relative()
        .child(
            div()
                .size_full()
                .border_2()
                .border_color(color.opacity(0.6))
                .rounded_sm()
        )
        .child(
            div()
                .absolute()
                .left(px(4.0))
                .w_px()
                .h_full()
                .bg(color.opacity(0.4))
        )
        .child(
            div()
                .absolute()
                .top(px(4.0))
                .w_full()
                .h_px()
                .bg(color.opacity(0.4))
        )
        .into_any_element()
}

pub fn close_icon(color: Hsla) -> AnyElement {
    div()
        .size(px(12.0))
        .flex()
        .items_center()
        .justify_center()
        .hover(|s| s.bg(hsla(0.0, 0.0, 1.0, 0.1)).rounded_sm())
        .child(
            div()
                .text_size(px(14.0))
                .text_color(color)
                .child("×")
        )
        .into_any_element()
}

pub fn terminal_icon(color: Hsla) -> AnyElement {
    div()
        .w(px(18.0))
        .h(px(14.0))
        .border_1()
        .border_color(color.opacity(0.6))
        .rounded_sm()
        .relative()
        .child(
            div()
                .absolute()
                .top(px(4.0))
                .left(px(3.0))
                .child(div().text_size(px(8.0)).text_color(color).child(">_"))
        )
        .into_any_element()
}
