use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;

    // Basic list item (RNE: padding 14/16, bg = white, hairline border)
    pub ElementListItem = <View> {
        width: Fill, height: Fit,
        padding: {left: 16, right: 16, top: 14, bottom: 14},
        flow: Right,
        spacing: 12,
        align: {y: 0.5},

        show_bg: true,
        draw_bg: { color: #ffffff }

        content = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: 2,

            title = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #242424,
                    text_style: { font_size: 14.0 }
                }
                text: "List Item"
            }

            subtitle = <Label> {
                width: Fill, height: Fit,
                draw_text: {
                    color: #8693a0,
                    text_style: { font_size: 12.0 }
                }
                text: "Subtitle"
            }
        }

        // Chevron (Sdf2d drawn arrow)
        chevron = <ElementChevronRight> {}
    }

    // List item accordion — expandable/collapsible list item
    // RNE: ListItem.Accordion with expand icon rotation, animated body reveal
    pub ElementListItemAccordion = <FoldHeader> {
        width: Fill, height: Fit,

        header = <View> {
            width: Fill, height: Fit,
            padding: {left: 16, right: 16, top: 14, bottom: 14},
            flow: Right,
            spacing: 12,
            align: {y: 0.5},

            show_bg: true,
            draw_bg: { color: #ffffff }

            content = <View> {
                width: Fill, height: Fit,
                flow: Down,
                spacing: 2,

                title = <Label> {
                    width: Fill, height: Fit,
                    draw_text: {
                        color: #242424,
                        text_style: { font_size: 14.0 }
                    }
                    text: "Accordion Item"
                }

                subtitle = <Label> {
                    width: Fill, height: Fit,
                    draw_text: {
                        color: #8693a0,
                        text_style: { font_size: 12.0 }
                    }
                    text: ""
                }
            }

            fold_button = <FoldButton> {
                draw_bg: {
                    color: #bdc6cf,
                    color_hover: #8693a0,
                    color_active: #8693a0,
                }
            }
        }

        body = <View> {
            width: Fill, height: Fit,
            padding: {left: 16, right: 16, bottom: 8},
            flow: Down,
            spacing: 4,
        }
    }

    // List item with left avatar area
    pub ElementListItemAvatar = <View> {
        width: Fill, height: Fit,
        padding: {left: 16, right: 16, top: 14, bottom: 14},
        flow: Right,
        spacing: 12,
        align: {y: 0.5},

        show_bg: true,
        draw_bg: { color: #ffffff }

        avatar = <ElementCircleViewBase> {
            width: 40, height: 40,
            align: {x: 0.5, y: 0.5},
            <Label> {
                width: Fit, height: Fit,
                draw_text: { color: #ffffff, text_style: { font_size: 14.0 } }
                text: "A"
            }
        }

        content = <View> {
            width: Fill, height: Fit,
            flow: Down,
            spacing: 2,

            title = <Label> {
                width: Fill, height: Fit,
                draw_text: { color: #242424, text_style: { font_size: 14.0 } }
                text: "List Item"
            }

            subtitle = <Label> {
                width: Fill, height: Fit,
                draw_text: { color: #8693a0, text_style: { font_size: 12.0 } }
                text: "Subtitle"
            }
        }

        // Chevron (Sdf2d drawn arrow)
        chevron = <ElementChevronRight> {}
    }
}
