use makepad_widgets::*;

use crate::components::rating::ElementRatingAction;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::components::rating::ElementRating;

    // Airbnb-style tap rating with review label above stars.
    pub ElementAirbnbRating = {{ElementAirbnbRating}} {
        width: Fit, height: Fit,
        flow: Down,
        align: {x: 0.5},
        spacing: 10.0,

        // TapRating defaults from RNE.
        show_rating: true,
        default_rating: 3,
        is_disabled: false,
        review_size: 25.0,
        review_color: #e6c42e,
        review_1: "Terrible",
        review_2: "Bad",
        review_3: "Okay",
        review_4: "Good",
        review_5: "Great",

        review_label = <Label> {
            width: Fit, height: Fit,
            draw_text: {
                color: #e6c42e,
                text_style: { font_size: 25.0 }
            }
            text: "Okay"
        }

        rating = <ElementRating> {
            value: 3.0,
            fractions: 1,
            show_rating: false,
            read_only: false,
            star_size: 40.0,
            filled_color: #004666,
            empty_color: #BDC3C7,
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ElementAirbnbRatingAction {
    Changed(i64),
    None,
}

#[derive(Live, Widget)]
pub struct ElementAirbnbRating {
    #[deref]
    view: View,

    #[live(true)]
    show_rating: bool,

    #[live(3)]
    default_rating: i64,

    #[live(false)]
    is_disabled: bool,

    #[live(25.0)]
    review_size: f64,

    #[live]
    review_color: Vec4,

    #[live]
    review_1: String,
    #[live]
    review_2: String,
    #[live]
    review_3: String,
    #[live]
    review_4: String,
    #[live]
    review_5: String,

    #[rust(3)]
    current_rating: i64,
    #[rust(false)]
    initialized: bool,
}

impl LiveHook for ElementAirbnbRating {
    fn after_apply(
        &mut self,
        _cx: &mut Cx,
        _apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        self.current_rating = self.default_rating.clamp(1, 5);
        self.initialized = false;
    }
}

impl Widget for ElementAirbnbRating {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.initialized {
            self.sync_children(cx);
            self.initialized = true;
        }

        let uid = self.widget_uid();
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        let rating_widget = self.view.widget(ids!(rating));
        if let Some(action) = actions.find_widget_action(rating_widget.widget_uid()) {
            if let ElementRatingAction::Changed(value) = action.cast() {
                let value = value.round().clamp(1.0, 5.0) as i64;
                if value != self.current_rating {
                    self.current_rating = value;
                    self.update_review_label(cx);
                    cx.widget_action(
                        uid,
                        &scope.path,
                        ElementAirbnbRatingAction::Changed(self.current_rating),
                    );
                    self.view.redraw(cx);
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.initialized {
            self.sync_children(cx);
            self.initialized = true;
        }
        self.update_review_label(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ElementAirbnbRating {
    fn sync_children(&mut self, cx: &mut Cx) {
        let rating_widget = self.view.widget(ids!(rating));
        rating_widget.apply_over(
            cx,
            live! {
                value: (self.current_rating as f64),
                fractions: 1,
                show_rating: false,
                read_only: (self.is_disabled),
            },
        );
        self.update_review_label(cx);
    }

    fn update_review_label(&mut self, cx: &mut Cx) {
        let label = self.view.label(ids!(review_label));
        label.apply_over(
            cx,
            live! {
                draw_text: {
                    color: (self.review_color),
                    text_style: { font_size: (self.review_size) }
                }
            },
        );

        if self.show_rating {
            let text = self.review_text(self.current_rating);
            label.set_text(cx, text);
        } else {
            label.set_text(cx, "");
        }
    }

    fn review_text(&self, rating: i64) -> &str {
        match rating.clamp(1, 5) {
            1 => self.review_1.as_str(),
            2 => self.review_2.as_str(),
            3 => self.review_3.as_str(),
            4 => self.review_4.as_str(),
            _ => self.review_5.as_str(),
        }
    }
}

impl ElementAirbnbRatingRef {
    pub fn changed(&self, actions: &Actions) -> Option<i64> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ElementAirbnbRatingAction::Changed(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }
}
