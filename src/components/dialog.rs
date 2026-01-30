use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;
    use crate::components::button::*;

    // Dialog card (used inside an overlay)
    pub ElementDialog = <RoundedView> {
        width: 320, height: Fit,
        padding: 20,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_radius: 3.0,
        }

        flow: Down,
        spacing: 16,

        title = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                color: #333333,
                text_style: { font_size: 18.0 }
            }
            text: "Dialog Title"
        }

        body = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                color: #8693a0,
                text_style: { font_size: 14.0 }
                wrap: Word,
            }
            text: "Dialog body text goes here."
        }

        // RNE: marginTop 10, flexDirection row-reverse, justifyContent flex-start
        actions = <View> {
            width: Fill, height: Fit,
            margin: {top: 10},
            flow: Right,
            spacing: 8,
            align: {x: 1.0},
        }
    }

    // Dialog with loading spinner
    // RNE: Dialog.Loading — centered ActivityIndicator with optional title
    pub ElementDialogLoading = <RoundedView> {
        width: 200, height: Fit,
        padding: 24,
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_radius: 3.0,
        }

        flow: Down,
        spacing: 16,

        spinner = <View> {
            width: Fill, height: Fit,
            align: {x: 0.5},

            <LoadingSpinner> {
                width: 48, height: 48,
                draw_bg: {
                    color: #2089dc,
                    stroke_width: 3.0,
                }
            }
        }

        title = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                color: #333333,
                text_style: { font_size: 14.0 }
            }
            text: "Loading..."
        }
    }

    // Dialog with preset Cancel + OK buttons
    pub ElementDialogConfirm = <ElementDialog> {
        actions = {
            cancel = <ElementButtonClear> { text: "Cancel" }
            confirm = <ElementButtonSolid> { text: "OK" }
        }
    }
}
