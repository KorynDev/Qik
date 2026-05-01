use gpui::*;

pub struct Theme {
    pub background: Hsla,
    pub sidebar_bg: Hsla,
    pub editor_bg: Hsla,
    pub status_bg: Hsla,
    pub panel_bg: Hsla,
    pub toolbar_bg: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_accent: Hsla,
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub selection: Hsla,
    pub cursor: Hsla,
    pub line_number: Hsla,
    pub shadow: Hsla,
    pub tab_active_bg: Hsla,
    pub tab_inactive_bg: Hsla,
    pub git_added: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
}

impl Theme {
    pub fn zed_dark() -> Self {
        Self {
            background: hsla(0.0, 0.0, 0.09, 1.0),     // #171717
            sidebar_bg: hsla(0.0, 0.0, 0.07, 1.0),     // #121212
            editor_bg: hsla(0.0, 0.0, 0.09, 1.0),      // #171717
            status_bg: hsla(0.0, 0.0, 0.07, 1.0),      // #121212
            panel_bg: hsla(0.0, 0.0, 0.12, 1.0),       // #1f1f1f
            toolbar_bg: hsla(0.0, 0.0, 0.11, 1.0),     // #1c1c1c
            border: hsla(0.0, 0.0, 0.18, 1.0),         // #2e2e2e
            border_variant: hsla(0.0, 0.0, 0.14, 1.0), // #242424
            text: hsla(0.0, 0.0, 0.85, 1.0),           // #d9d9d9
            text_muted: hsla(0.0, 0.0, 0.55, 1.0),     // #8c8c8c
            text_accent: hsla(210.0 / 360.0, 0.85, 0.70, 1.0), // Light Blue
            accent: hsla(210.0 / 360.0, 0.80, 0.60, 1.0),      // Blue
            accent_hover: hsla(210.0 / 360.0, 0.80, 0.70, 1.0),
            selection: hsla(210.0 / 360.0, 0.50, 0.35, 0.4),
            cursor: hsla(210.0 / 360.0, 0.80, 0.60, 1.0),
            line_number: hsla(0.0, 0.0, 0.35, 1.0),    // #595959
            shadow: hsla(0.0, 0.0, 0.0, 0.5),
            tab_active_bg: hsla(0.0, 0.0, 0.11, 1.0),  // #1c1c1c
            tab_inactive_bg: hsla(0.0, 0.0, 0.07, 1.0), // #121212
            git_added: hsla(140.0 / 360.0, 0.55, 0.55, 1.0),
            git_modified: hsla(45.0 / 360.0, 0.75, 0.55, 1.0),
            git_deleted: hsla(0.0 / 360.0, 0.65, 0.55, 1.0),
        }
    }
}

impl Global for Theme {}
