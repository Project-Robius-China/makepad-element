use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;

    // Speed dial action item (small FAB)
    pub ElementSpeedDialAction = <ElementCircleButtonBase> {
        width: 40, height: 40,
        draw_bg: { color: #8693a0, color_hover: #5e6977 }
        draw_text: { text_style: { font_size: 14.0 } }
    }

    // Speed dial container (vertical stack of actions + main FAB)
    pub ElementSpeedDial = <View> {
        width: Fit, height: Fit,
        flow: Down,
        spacing: 12,
        align: {x: 0.5},

        // RNE: action marginBottom 16
        actions = <View> {
            width: Fit, height: Fit,
            flow: Down,
            spacing: 16,
            align: {x: 0.5},
        }

        // RNE: margin 16, marginTop 0
        main_button = <ElementCircleButtonBase> {
            width: 56, height: 56,
            margin: {left: 16, right: 16, bottom: 16},
            draw_bg: { color: #2089dc, color_hover: #1975be }
            draw_text: { text_style: { font_size: 24.0 } }
            text: "+"
        }
    }
}
