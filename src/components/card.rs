use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Basic card container (RNE: padding 15, margin 15, borderWidth 1, borderColor grey5, borderRadius ~3)
    pub ElementCard = <RoundedView> {
        width: Fill,
        height: Fit,
        padding: 15,
        margin: 15,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_color: #e1e8ee,
            border_radius: 3.0,
            border_size: 1.0,
        }

        flow: Down,
        spacing: 8,
    }

    // Card with shadow
    pub ElementCardShadow = <RoundedShadowView> {
        width: Fill,
        height: Fit,
        padding: 15,
        margin: 15,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_radius: 3.0,
        }

        flow: Down,
        spacing: 8,
    }

    // Card Title — composable card title sub-component
    // RNE: Card.Title — fontSize 14, fontWeight bold, color grey1, marginBottom 15, textAlign center
    pub ElementCardTitle = <View> {
        width: Fill, height: Fit,
        padding: {left: 15, right: 15, top: 12, bottom: 12},

        label = <Label> {
            width: Fill, height: Fit,
            draw_text: {
                color: #333333,
                text_style: { font_size: 14.0 }
            }
            text: "Card Title"
        }
    }

    // Card Image — composable card image sub-component
    // RNE: Card.Image — width 100%, height 150
    pub ElementCardImage = <View> {
        width: Fill, height: 150,
        clip_x: true,
        clip_y: true,

        image = <Image> {
            width: Fill, height: Fill,
            fit: Smallest,
        }
    }

    // Card Divider — thin separator for card sections
    // RNE: Card.Divider — marginBottom 15
    pub ElementCardDivider = <View> {
        width: Fill, height: 1,
        margin: {bottom: 12},
        show_bg: true,
        draw_bg: { color: #e1e8ee }
    }

    // Card with title
    pub ElementCardTitled = <RoundedView> {
        width: Fill,
        height: Fit,
        margin: 15,

        show_bg: true,
        draw_bg: {
            color: #ffffff,
            border_color: #e1e8ee,
            border_radius: 3.0,
            border_size: 1.0,
        }

        flow: Down,
        spacing: 0,

        header = <View> {
            width: Fill, height: Fit,
            padding: {left: 15, right: 15, top: 12, bottom: 12},

            title = <Label> {
                width: Fit, height: Fit,
                draw_text: {
                    color: #333333,
                    text_style: { font_size: 16.0 }
                }
                text: "Card Title"
            }
        }

        <View> {
            width: Fill, height: 1.0,
            show_bg: true,
            draw_bg: { color: #e1e8ee }
        }

        content = <View> {
            width: Fill, height: Fit,
            padding: 15,
            flow: Down,
            spacing: 8,
        }
    }
}
