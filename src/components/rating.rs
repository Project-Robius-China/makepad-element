use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Single star (filled)
    StarFilled = <Label> {
        width: Fit, height: Fit,
        draw_text: { color: #faad14, text_style: { font_size: 20.0 } }
        text: "★"
    }

    // Single star (empty)
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

    // Simple clickable star using Label
    ClickableStar = <View> {
        width: Fit, height: Fit,
        cursor: Hand,

        star_label = <Label> {
            width: Fit, height: Fit,
            draw_text: { color: #bdc6cf, text_style: { font_size: 24.0 } }
            text: "★"
        }
    }

    // Interactive rating component (tap to select)
    pub ElementRating = {{ElementRating}} {
        width: Fit, height: Fit,
        flow: Right, spacing: 4,
        align: { y: 0.5 }

        star_size: 24.0,
        filled_color: #faad14,
        empty_color: #bdc6cf,
        max_stars: 5,
        value: 0,
        read_only: false,
        show_rating: false,

        stars_container = <View> {
            width: Fit, height: Fit,
            flow: Right, spacing: 4,

            star0 = <ClickableStar> {}
            star1 = <ClickableStar> {}
            star2 = <ClickableStar> {}
            star3 = <ClickableStar> {}
            star4 = <ClickableStar> {}
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

    /// Rating value (0 to max_stars)
    #[live(0.0)]
    value: f64,

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
                    // Set value to clicked star index + 1
                    let new_value = (i + 1) as f64;

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
        // Update star colors and rating label before drawing
        self.update_stars_colors(cx);
        self.update_rating_label(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ElementRating {
    fn update_stars(&mut self, cx: &mut Cx) {
        self.update_stars_colors(cx);
        self.update_rating_label(cx);
        self.view.redraw(cx);
    }

    fn update_stars_colors(&mut self, cx: &mut Cx) {
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

            let star_index = (i + 1) as f64;

            // Star is filled if value >= star index
            let color = if self.value >= star_index {
                self.filled_color
            } else {
                self.empty_color
            };

            stars_container.label(&[star_id, id!(star_label)]).apply_over(cx, live! {
                draw_text: { color: (color) }
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
