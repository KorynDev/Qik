use gpui::*;

pub struct Theme {
    pub background: Hsla,
    pub sidebar_bg: Hsla,
    pub editor_bg: Hsla,
    pub status_bg: Hsla,
    pub panel_bg: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_accent: Hsla,
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub selection: Hsla,
    pub cursor: Hsla,
    pub line_number: Hsla,
    pub shadow: Hsla,
    pub git_added: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
}

impl Theme {
    pub fn qik_dark() -> Self {
        Self {
            background: hsla(220.0 / 360.0, 0.13, 0.18, 1.0),
            sidebar_bg: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
            editor_bg: hsla(220.0 / 360.0, 0.13, 0.18, 1.0),
            status_bg: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
            panel_bg: hsla(220.0 / 360.0, 0.13, 0.22, 1.0),
            border: hsla(220.0 / 360.0, 0.13, 0.25, 1.0),
            text: hsla(220.0 / 360.0, 0.10, 0.85, 1.0),
            text_muted: hsla(220.0 / 360.0, 0.10, 0.50, 1.0),
            text_accent: hsla(200.0 / 360.0, 0.80, 0.80, 1.0),
            accent: hsla(200.0 / 360.0, 0.80, 0.60, 1.0),
            accent_hover: hsla(200.0 / 360.0, 0.80, 0.70, 1.0),
            selection: hsla(200.0 / 360.0, 0.50, 0.30, 0.5),
            cursor: hsla(200.0 / 360.0, 0.80, 0.60, 1.0),
            line_number: hsla(220.0 / 360.0, 0.10, 0.35, 1.0),
            shadow: hsla(0.0, 0.0, 0.0, 0.3),
            git_added: hsla(140.0 / 360.0, 0.60, 0.50, 1.0),
            git_modified: hsla(45.0 / 360.0, 0.80, 0.50, 1.0),
            git_deleted: hsla(0.0 / 360.0, 0.70, 0.50, 1.0),
        }
    }
}

impl Global for Theme {}
