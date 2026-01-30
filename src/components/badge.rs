use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;

    // Badge - small count indicator (RNE: size 18, borderRadius 9, fontSize 12, paddingH 4)
    pub ElementBadge = <ElementBadgeBase> {}

    // Badge dot (no text) - RNE mini size 8
    pub ElementBadgeDot = <ElementCircleViewBase> {
        width: 8, height: 8,
        draw_bg: { bg_color: #ff190c }
    }

    // Primary badge
    pub ElementBadgePrimary = <ElementBadgeBase> {
        draw_bg: { bg_color: #2089dc }
    }

    // Success badge
    pub ElementBadgeSuccess = <ElementBadgeBase> {
        draw_bg: { bg_color: #52c41a }
    }
}
