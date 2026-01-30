use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // ─── Base: Solid Button (rounded box with hover/down) ───
    // Overrides only the pixel shader; color fields are inherited from Button.
    pub ElementButtonBase = <Button> {
        width: Fit,
        height: Fit,
        padding: {left: 16, right: 16, top: 8, bottom: 8},

        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(
                    mix(self.color, self.color_hover, self.hover),
                    self.color_down,
                    self.down
                );
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 16.0 }
        }
    }

    // ─── Base: Circle Button (for FAB, social icons, speed dial) ───
    pub ElementCircleButtonBase = <Button> {
        width: 56,
        height: 56,
        padding: 0,
        align: {x: 0.5, y: 0.5},

        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(self.color, self.color_hover, self.hover);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);
                sdf.circle(c.x, c.y, r);
                sdf.fill(color);
                return sdf.result;
            }
        }
        draw_text: {
            color: #ffffff,
            text_style: { font_size: 24.0 }
        }
    }

    // ─── Base: Circle View (for avatar background, badge dot) ───
    pub ElementCircleViewBase = <View> {
        show_bg: true,
        draw_bg: {
            instance bg_color: #2089dc,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // ─── Base: Badge (pill shape with white border) ───
    pub ElementBadgeBase = <View> {
        width: Fit, height: 18,
        padding: {left: 4, right: 4, top: 0, bottom: 0},
        align: {x: 0.5, y: 0.5},

        show_bg: true,
        draw_bg: {
            instance bg_color: #ff190c,
            instance border_color: #ffffff,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bw = 0.5;
                let hw = bw * 0.5;
                let w = self.rect_size.x - bw;
                let h = self.rect_size.y - bw;
                let r = min(w, h) * 0.25;
                sdf.box(hw, hw, w, h, max(1.0, r));
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, bw);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #ffffff,
                text_style: { font_size: 12.0 }
            }
            text: "1"
        }
    }

    // ─── Base: Header (3-section app bar) ───
    pub ElementHeaderBase = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: 56,
            flow: Right,
            padding: {left: 10, right: 10, top: 10, bottom: 10},
            align: {y: 0.5},

            show_bg: true,
            draw_bg: { color: #2089dc }

            left = <View> {
                width: Fit, height: Fit,
            }

            center = <View> {
                width: Fill, height: Fit,
                padding: {left: 15, right: 15},
                align: {x: 0.5},
                title = <Label> {
                    width: Fit, height: Fit,
                    draw_text: {
                        color: #ffffff,
                        text_style: { font_size: 18.0 }
                    }
                    text: "Title"
                }
            }

            right = <View> {
                width: Fit, height: Fit,
            }
        }

        <View> {
            width: Fill, height: 1,
            show_bg: true,
            draw_bg: { color: #f2f2f2 }
        }
    }

    // ─── Base: Linear Progress Bar ───
    pub ElementLinearProgressBase = <View> {
        width: Fill, height: 4,
        show_bg: true,
        draw_bg: {
            instance progress: 0.5,
            instance fill_color: #2089dc,
            instance border_radius: 2.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let track_color = vec4(self.fill_color.x, self.fill_color.y, self.fill_color.z, 0.4);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(track_color);
                let fill_width = self.rect_size.x * self.progress;
                if fill_width > 0.0 {
                    sdf.box(0., 0., fill_width, self.rect_size.y, self.border_radius);
                    sdf.fill(self.fill_color);
                }
                return sdf.result;
            }
        }
    }

    // ─── Base: Skeleton Shimmer (rectangle with animated highlight band) ───
    pub ElementSkeletonBase = <View> {
        show_bg: true,
        draw_bg: {
            instance bg_color: #bdc6cf,
            instance highlight_color: #f5f5f5,
            instance border_radius: 2.0,
            instance shimmer_pos: 0.0,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                let band_center = self.shimmer_pos * 1.4 - 0.2;
                let dist = abs(self.pos.x - band_center);
                let band = smoothstep(0.15, 0.0, dist);
                let color = mix(self.bg_color, self.highlight_color, band);
                sdf.fill(color);
                return sdf.result;
            }
        }
        animator: {
            shimmer = {
                default: on,
                on = {
                    from: {all: Loop {duration: 1.5, end: 1.0}}
                    apply: {
                        draw_bg: { shimmer_pos: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}] }
                    }
                }
            }
        }
    }

    // ─── Base: Chevron Icon (right-pointing arrow drawn with Sdf2d) ───
    // Note: uses stroke_color (not color) to avoid conflict with View's DrawColor.color field.
    pub ElementChevronRight = <View> {
        width: 8, height: 14,
        show_bg: true,
        draw_bg: {
            instance stroke_color: #bdc6cf,

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.move_to(0., 0.);
                sdf.line_to(self.rect_size.x, self.rect_size.y * 0.5);
                sdf.line_to(0., self.rect_size.y);
                sdf.stroke(self.stroke_color, 1.5);
                return sdf.result;
            }
        }
    }
}
