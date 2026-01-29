use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;

    // Floating action button (RNE: default color = secondary #ad1457, large 56x56 padding 16, borderRadius 28)
    pub ElementFab = <ElementCircleButtonBase> {
        width: 56, height: 56,
        padding: 16,
        draw_bg: { color: #ad1457, color_hover: #8e1148 }
        draw_text: { text_style: { font_size: 24.0 } }
        text: "+"
    }

    // Small FAB (RNE: 40x40 padding 8)
    pub ElementFabSmall = <ElementCircleButtonBase> {
        width: 40, height: 40,
        padding: 8,
        draw_bg: { color: #ad1457, color_hover: #8e1148 }
        draw_text: { text_style: { font_size: 18.0 } }
        text: "+"
    }

    // Extended FAB (with text, RNE: large height 48 padding 16, borderRadius 28)
    pub ElementFabExtended = <ElementButtonBase> {
        width: Fit, height: 48,
        padding: {left: 16, right: 16, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},
        draw_bg: {
            color: #ad1457,
            color_hover: #8e1148,
            border_radius: 28.0,
        }
        draw_text: { text_style: { font_size: 14.0 } }
        text: "+ Create"
    }
}
