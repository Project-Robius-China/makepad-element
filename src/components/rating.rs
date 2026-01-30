use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::svg::ElementSvg;

    // Single star (filled) - for static variants
    StarFilled = <Label> {
        width: Fit, height: Fit,
        draw_text: { color: #faad14, text_style: { font_size: 20.0 } }
        text: "★"
    }

    // Single star (empty) - for static variants
    StarEmpty = <Label> {
        width: Fit, height: Fit,
        draw_text: { color: #bdc6cf, text_style: { font_size: 20.0 } }
        text: "★"
    }

    // 5-star rating (0 filled)
    pub ElementRating0 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarEmpty> {} <StarEmpty> {} <StarEmpty> {}
        <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (1 filled)
    pub ElementRating1 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {}
        <StarEmpty> {} <StarEmpty> {} <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (2 filled)
    pub ElementRating2 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {}
        <StarEmpty> {} <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (3 filled)
    pub ElementRating3 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {} <StarFilled> {}
        <StarEmpty> {} <StarEmpty> {}
    }

    // 5-star rating (4 filled)
    pub ElementRating4 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {} <StarFilled> {} <StarFilled> {}
        <StarEmpty> {}
    }

    // 5-star rating (5 filled)
    pub ElementRating5 = <View> {
        width: Fit, height: Fit,
        flow: Right, spacing: 2,
        <StarFilled> {} <StarFilled> {} <StarFilled> {}
        <StarFilled> {} <StarFilled> {}
    }

    // SVG-based clickable star
    SvgStar = <View> {
        width: Fit, height: Fit,
        cursor: Hand,
        star_svg = <ElementSvg> {
            width: 24, height: 24,
            scale: 2.0,
        }
    }

    // Interactive rating component with SVG stars (supports half-star)
    pub ElementRating = {{ElementRating}} {
        width: Fit, height: Fit,
        flow: Right, spacing: 4,
        align: { y: 0.5 }

        star_size: 24.0,
        filled_color: #faad14,
        empty_color: #bdc6cf,
        max_stars: 5,
        value: 0.0,
        fractions: 1,
        read_only: false,
        show_rating: false,

        stars_container = <View> {
            width: Fit, height: Fit,
            flow: Right, spacing: 4,

            star0 = <SvgStar> {}
            star1 = <SvgStar> {}
            star2 = <SvgStar> {}
            star3 = <SvgStar> {}
            star4 = <SvgStar> {}
        }

        rating_label = <Label> {
            width: Fit, height: Fit,
            margin: { left: 8.0 }
            draw_text: { color: #666666, text_style: { font_size: 14.0 } }
            text: ""
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ElementRatingAction {
    Changed(f64),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementRating {
    #[deref]
    view: View,

    #[live(24.0)]
    star_size: f64,

    #[live]
    filled_color: Vec4,

    #[live]
    empty_color: Vec4,

    #[live(5)]
    max_stars: i64,

    /// Rating value (supports fractions like 3.5)
    #[live(0.0)]
    value: f64,

    /// Number of fraction steps (1 = whole stars, 2 = half stars)
    #[live(1)]
    fractions: i64,

    #[live(false)]
    read_only: bool,

    /// Show rating value as text next to stars
    #[live(false)]
    show_rating: bool,
}

impl Widget for ElementRating {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if self.read_only {
            return;
        }

        // Handle clicks on each star
        for i in 0..(self.max_stars.min(5) as usize) {
            let star_id = match i {
                0 => id!(star0),
                1 => id!(star1),
                2 => id!(star2),
                3 => id!(star3),
                4 => id!(star4),
                _ => continue,
            };

            let star = self.view.view(ids!(stars_container)).view(&[star_id]);
            if let Hit::FingerUp(fe) = event.hits(cx, star.area()) {
                if fe.is_over {
                    // Calculate new value based on fractions
                    let base_value = (i + 1) as f64;
                    let new_value = if self.fractions >= 2 {
                        // For half-star support, check click position
                        let rel_x = fe.abs.x - star.area().rect(cx).pos.x;
                        let star_width = star.area().rect(cx).size.x;
                        if rel_x < star_width / 2.0 {
                            base_value - 0.5
                        } else {
                            base_value
                        }
                    } else {
                        base_value
                    };

                    if (new_value - self.value).abs() > 0.01 {
                        self.value = new_value;
                        self.update_stars(cx);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            ElementRatingAction::Changed(self.value),
                        );
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update star SVGs and rating label before drawing
        self.update_stars_svg(cx);
        self.update_rating_label(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ElementRating {
    /// Generate SVG star with specified fill amount (0.0, 0.5, or 1.0)
    fn generate_star_svg(&self, fill_amount: f64) -> String {
        let size = self.star_size as i32;
        let filled_color = format!(
            "#{:02x}{:02x}{:02x}",
            (self.filled_color.x * 255.0) as u8,
            (self.filled_color.y * 255.0) as u8,
            (self.filled_color.z * 255.0) as u8
        );
        let empty_color = format!(
            "#{:02x}{:02x}{:02x}",
            (self.empty_color.x * 255.0) as u8,
            (self.empty_color.y * 255.0) as u8,
            (self.empty_color.z * 255.0) as u8
        );

        // Star path (5-pointed star centered in viewBox)
        let star_path = "M12,2 L15.09,8.26 L22,9.27 L17,14.14 L18.18,21.02 L12,17.77 L5.82,21.02 L7,14.14 L2,9.27 L8.91,8.26 Z";

        if fill_amount >= 1.0 {
            // Fully filled star
            format!(
                "<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' viewBox='0 0 24 24'>\
                <path d='{}' fill='{}'/>\
                </svg>",
                size, size, star_path, filled_color
            )
        } else if fill_amount >= 0.5 {
            // Half-filled star using clipPath
            format!(
                "<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' viewBox='0 0 24 24'>\
                <defs>\
                <clipPath id='half'><rect x='0' y='0' width='12' height='24'/></clipPath>\
                </defs>\
                <path d='{}' fill='{}'/>\
                <path d='{}' fill='{}' clip-path='url(#half)'/>\
                </svg>",
                size, size, star_path, empty_color, star_path, filled_color
            )
        } else {
            // Empty star
            format!(
                "<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' viewBox='0 0 24 24'>\
                <path d='{}' fill='{}'/>\
                </svg>",
                size, size, star_path, empty_color
            )
        }
    }

    fn update_stars(&mut self, cx: &mut Cx) {
        self.update_stars_svg(cx);
        self.update_rating_label(cx);
        self.view.redraw(cx);
    }

    fn update_stars_svg(&mut self, cx: &mut Cx) {
        let stars_container = self.view.view(ids!(stars_container));

        for i in 0..(self.max_stars.min(5) as usize) {
            let star_id = match i {
                0 => id!(star0),
                1 => id!(star1),
                2 => id!(star2),
                3 => id!(star3),
                4 => id!(star4),
                _ => continue,
            };

            let star_index = i as f64;
            let fill_amount = (self.value - star_index).clamp(0.0, 1.0);

            // Generate SVG based on fill amount
            let svg_text = self.generate_star_svg(fill_amount);

            // Update the SVG widget using apply_over
            let star_view = stars_container.view(&[star_id]);
            star_view.apply_over(cx, live! {
                star_svg = { text: (svg_text) }
            });
        }
    }

    fn update_rating_label(&mut self, cx: &mut Cx) {
        let label = self.view.label(ids!(rating_label));
        if self.show_rating {
            // Format value: show decimal only if it's not a whole number
            let text = if (self.value - self.value.round()).abs() < 0.01 {
                format!("{:.0}", self.value)
            } else {
                format!("{:.1}", self.value)
            };
            label.set_text(cx, &text);
        } else {
            label.set_text(cx, "");
        }
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: f64) {
        self.value = value.clamp(0.0, self.max_stars as f64);
        self.update_stars(cx);
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl ElementRatingRef {
    pub fn changed(&self, actions: &Actions) -> Option<f64> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ElementRatingAction::Changed(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }

    pub fn set_value(&self, cx: &mut Cx, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }

    pub fn get_value(&self) -> f64 {
        if let Some(inner) = self.borrow() {
            inner.get_value()
        } else {
            0.0
        }
    }
}
