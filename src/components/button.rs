use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;

    // Solid button (primary)
    pub ElementButtonSolid = <ElementButtonBase> {
        draw_bg: {
            color: #2089dc,
            color_hover: #1975be,
            color_down: #1461a0,
        }
    }

    // Outline button
    pub ElementButtonOutline = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #ffffff00,
            instance color_hover: #2089dc10,
            instance border_color: #2089dc,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.color, self.color_hover, self.hover);
                let bw = 1.0;
                let hw = bw * 0.5;
                sdf.box(hw, hw, self.rect_size.x - bw, self.rect_size.y - bw, self.border_radius);
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, bw);
                return sdf.result;
            }
        }
        draw_text: {
            color: #2089dc,
            text_style: { font_size: 16.0 }
        }
    }

    // Clear/text button
    pub ElementButtonClear = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            instance color: #ffffff00,
            instance color_hover: #2089dc10,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.color, self.color_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(bg);
                return sdf.result;
            }
        }
        draw_text: {
            color: #2089dc,
            text_style: { font_size: 16.0 }
        }
    }

    // Error/danger button
    pub ElementButtonError = <ElementButtonBase> {
        draw_bg: {
            color: #ff190c,
            color_hover: #d9150a,
        }
    }

    // Success button
    pub ElementButtonSuccess = <ElementButtonBase> {
        draw_bg: {
            color: #52c41a,
            color_hover: #46a817,
        }
    }

    // Secondary button
    pub ElementButtonSecondary = <ElementButtonBase> {
        draw_bg: {
            color: #ad1457,
            color_hover: #8e1148,
        }
    }

    // Warning button
    pub ElementButtonWarning = <ElementButtonBase> {
        draw_bg: {
            color: #faad14,
            color_hover: #e09a00,
        }
    }

    // Disabled button
    pub ElementButtonDisabled = <ElementButtonBase> {
        enabled: false,
        draw_bg: {
            color: #e5e5e5,
            color_hover: #e5e5e5,
            color_down: #e5e5e5,
        }
        draw_text: {
            color: #9e9e9e,
        }
    }
}
