use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Bottom Sheet — modal overlay that slides content up from the bottom
    // RNE: backdrop rgba(0,0,0,0.4), content bg white, borderTopLeftRadius 15,
    //       borderTopRightRadius 15, scrollable content
    pub ElementBottomSheet = <View> {
        width: Fill, height: Fill,
        flow: Overlay,
        align: {x: 0.5, y: 1.0},

        // Backdrop
        backdrop = <View> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: { color: #00000066 }
        }

        // Content panel — anchored to bottom, rounded top corners
        sheet = <RoundedView> {
            width: Fill, height: Fit,
            padding: {left: 16, right: 16, top: 12, bottom: 16},
            flow: Down,
            spacing: 12,

            show_bg: true,
            draw_bg: {
                color: #ffffff,
                border_radius: 15.0,
            }

            // Drag handle indicator (RNE pattern)
            handle = <View> {
                width: Fill, height: Fit,
                align: {x: 0.5},
                padding: {bottom: 4},

                <RoundedView> {
                    width: 40, height: 4,
                    show_bg: true,
                    draw_bg: {
                        color: #d0d0d0,
                        border_radius: 2.0,
                    }
                }
            }

            // Content slot — scrollable
            content = <View> {
                width: Fill, height: Fit,
                flow: Down,
                spacing: 8,
            }
        }
    }

    // Bottom Sheet with fixed height (half screen)
    pub ElementBottomSheetHalf = <View> {
        width: Fill, height: Fill,
        flow: Overlay,
        align: {x: 0.5, y: 1.0},

        backdrop = <View> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: { color: #00000066 }
        }

        sheet = <RoundedView> {
            width: Fill, height: 400,
            padding: {left: 16, right: 16, top: 12, bottom: 16},
            flow: Down,
            spacing: 12,

            show_bg: true,
            draw_bg: {
                color: #ffffff,
                border_radius: 15.0,
            }

            handle = <View> {
                width: Fill, height: Fit,
                align: {x: 0.5},
                padding: {bottom: 4},

                <RoundedView> {
                    width: 40, height: 4,
                    show_bg: true,
                    draw_bg: {
                        color: #d0d0d0,
                        border_radius: 2.0,
                    }
                }
            }

            content = <View> {
                width: Fill, height: Fill,
                flow: Down,
                spacing: 8,
            }
        }
    }

    // Bottom Sheet with list items (common pattern: action sheet)
    pub ElementBottomSheetActions = <View> {
        width: Fill, height: Fill,
        flow: Overlay,
        align: {x: 0.5, y: 1.0},

        backdrop = <View> {
            width: Fill, height: Fill,
            show_bg: true,
            draw_bg: { color: #00000066 }
        }

        sheet = <RoundedView> {
            width: Fill, height: Fit,
            padding: {top: 12, bottom: 16},
            flow: Down,

            show_bg: true,
            draw_bg: {
                color: #ffffff,
                border_radius: 15.0,
            }

            handle = <View> {
                width: Fill, height: Fit,
                align: {x: 0.5},
                padding: {bottom: 8},

                <RoundedView> {
                    width: 40, height: 4,
                    show_bg: true,
                    draw_bg: {
                        color: #d0d0d0,
                        border_radius: 2.0,
                    }
                }
            }

            // Title (optional)
            title = <View> {
                width: Fill, height: Fit,
                padding: {left: 16, right: 16, bottom: 8},

                <Label> {
                    width: Fill, height: Fit,
                    draw_text: {
                        color: #333333,
                        text_style: { font_size: 16.0 }
                    }
                    text: "Actions"
                }
            }

            // Divider
            <View> {
                width: Fill, height: 1,
                show_bg: true,
                draw_bg: { color: #e1e8ee }
            }

            // Action items slot
            actions = <View> {
                width: Fill, height: Fit,
                flow: Down,
            }
        }
    }
}
