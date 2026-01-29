use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Button Group Item — individual segment in a button group
    // RNE: borderColor #e3e3e3, selectedButtonStyle bg primary, selectedTextStyle color white
    // Uses RadioButton with custom draw_bg that switches color based on `active` state.
    pub ElementButtonGroupItem = <RadioButton> {
        width: Fill,
        height: Fit,
        padding: {left: 12, right: 12, top: 8, bottom: 8},
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            instance color_active: #2089dc,
            instance color_inactive: #ffffff,
            instance border_color: #e3e3e3,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix(self.color_inactive, self.color_active, self.active);
                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
        draw_text: {
            instance color_active: #ffffff,
            instance color_inactive: #5e6977,

            fn get_color(self) -> vec4 {
                return mix(self.color_inactive, self.color_active, self.active);
            }

            text_style: { font_size: 14.0 }
        }
    }

    // Button Group — horizontal row of segmented buttons (1×N, border-collapsed)
    // RNE: flexDirection row, borderWidth 1, borderColor #e3e3e3, borderRadius 3,
    //       overflow hidden, marginH 10
    pub ElementButtonGroup = <View> {
        width: Fill, height: Fit,
        margin: {left: 10, right: 10},
        flow: Right,

        show_bg: true,
        draw_bg: {
            instance border_color: #e3e3e3,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
    }

    // Button Group Vertical — vertical stack of segmented buttons
    pub ElementButtonGroupVertical = <View> {
        width: Fill, height: Fit,
        margin: {left: 10, right: 10},
        flow: Down,

        show_bg: true,
        draw_bg: {
            instance border_color: #e3e3e3,
            instance border_radius: 3.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.stroke(self.border_color, 1.0);
                return sdf.result;
            }
        }
    }

    // Secondary color button group item
    pub ElementButtonGroupItemSecondary = <ElementButtonGroupItem> {
        draw_bg: {
            color_active: #ad1457,
        }
    }
}
