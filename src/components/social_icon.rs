use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;

    // Generic social icon button (RNE: paddingV 14, paddingH 21, margin 7, borderRadius 30)
    pub ElementSocialIcon = <ElementCircleButtonBase> {
        width: Fit, height: 52,
        padding: {left: 21, right: 21, top: 14, bottom: 14},
        margin: 7,
        draw_bg: { color: #333333, color_hover: #555555 }
        draw_text: { text_style: { font_size: 18.0 } }
    }

    // GitHub social icon
    pub ElementSocialGithub = <ElementSocialIcon> {
        draw_bg: { color: #000000, color_hover: #333333 }
        text: "GH"
    }

    // Twitter/X social icon
    pub ElementSocialTwitter = <ElementSocialIcon> {
        draw_bg: { color: #00aced, color_hover: #0099d4 }
        text: "X"
    }

    // Discord social icon
    pub ElementSocialDiscord = <ElementSocialIcon> {
        draw_bg: { color: #5865f2, color_hover: #4752c4 }
        text: "DC"
    }

    // Facebook social icon
    pub ElementSocialFacebook = <ElementSocialIcon> {
        draw_bg: { color: #4267B2, color_hover: #365899 }
        text: "FB"
    }

    // LinkedIn social icon
    pub ElementSocialLinkedin = <ElementSocialIcon> {
        draw_bg: { color: #007bb6, color_hover: #006699 }
        text: "IN"
    }

    // YouTube social icon
    pub ElementSocialYoutube = <ElementSocialIcon> {
        draw_bg: { color: #bb0000, color_hover: #990000 }
        text: "YT"
    }
}
