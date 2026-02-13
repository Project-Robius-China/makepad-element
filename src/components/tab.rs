use makepad_widgets::*;
use makepad_widgets::page_flip::PageFlip;
use makepad_widgets::radio_button::RadioButtonAction;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Tab bar item with clear selected/unselected visual states
    // Unselected: transparent bg, white text | Selected: white bg, primary text
    pub ElementTabItem = <RadioButtonTab> {
        width: Fit, height: Fit,
        padding: {left: 16, right: 16, top: 10, bottom: 10},

        draw_bg: {
            border_size: 0.0,
            border_radius: 4.0,
            // Unselected: transparent
            color: #00000000,
            color_2: #00000000,
            // Selected: white with slight transparency
            color_active: #ffffffee,
            color_2_active: #ffffffee,
            // Disabled
            color_disabled: #00000000,
            color_2_disabled: #00000000,
            // Border colors (all transparent for clean look)
            border_color: #00000000,
            border_color_hover: #ffffff30,
            border_color_down: #ffffff50,
            border_color_active: #00000000,
            border_color_focus: #00000000,
            border_color_disabled: #00000000,
            border_color_2: #00000000,
            border_color_2_hover: #00000000,
            border_color_2_down: #00000000,
            border_color_2_active: #00000000,
            border_color_2_focus: #00000000,
            border_color_2_disabled: #00000000,
        }

        draw_text: {
            // Unselected: white text
            color: #ffffffcc,
            // Selected: primary blue text
            color_active: #2089dc,
            color_disabled: #ffffff66,
            text_style: { font_size: 14.0 }
        }
    }

    // Tab bar container (RNE: bg = primary #2089dc)
    pub ElementTabBar = <View> {
        width: Fill, height: Fit,
        flow: Down,

        bar = <View> {
            width: Fill, height: Fit,
            flow: Right,
            show_bg: true,
            draw_bg: { color: #2089dc }
        }

        // Indicator line (RNE: 2px, secondary #ad1457)
        indicator = <View> {
            width: Fill, height: 2,
            show_bg: true,
            draw_bg: { color: #ad1457 }
        }
    }

    // Interactive tab view: tab bar + PageFlip content
    // Tabs must be named tab0..tab9, pages must be named page0..page9
    pub ElementTabView = {{ElementTabView}} {
        width: Fill, height: Fill,
        flow: Down,

        tab_bar = <View> {
            width: Fill, height: Fit,
            flow: Down,

            bar = <View> {
                width: Fill, height: Fit,
                flow: Right,
                show_bg: true,
                draw_bg: { color: #2089dc }
            }

            indicator = <View> {
                width: Fill, height: 2,
                show_bg: true,
                draw_bg: { color: #ad1457 }
            }
        }

        content = <PageFlip> {
            width: Fill, height: Fill,
        }
    }

    // TabView content item (convenience wrapper)
    pub ElementTabViewItem = <View> {
        width: Fill, height: Fill,
        flow: Down,
        padding: 16,
    }
}

// Action emitted when the active tab changes
#[derive(Clone, Debug, DefaultNone)]
pub enum ElementTabViewAction {
    TabChanged(usize),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ElementTabView {
    #[deref]
    view: View,

    #[rust(0usize)]
    active_index: usize,
}

const TAB_IDS: &[LiveId] = &[
    live_id!(tab0), live_id!(tab1), live_id!(tab2),
    live_id!(tab3), live_id!(tab4), live_id!(tab5),
    live_id!(tab6), live_id!(tab7), live_id!(tab8),
    live_id!(tab9),
];

const PAGE_IDS: &[LiveId] = &[
    live_id!(page0), live_id!(page1), live_id!(page2),
    live_id!(page3), live_id!(page4), live_id!(page5),
    live_id!(page6), live_id!(page7), live_id!(page8),
    live_id!(page9),
];

impl Widget for ElementTabView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Collect tab widget UIDs from the bar
        let bar = self.view.view(ids!(tab_bar.bar));
        let mut tab_uids: Vec<(usize, WidgetUid)> = Vec::new();
        for (i, tab_id) in TAB_IDS.iter().enumerate() {
            let tab_widget = bar.widget(&[*tab_id]);
            if !tab_widget.is_empty() {
                tab_uids.push((i, tab_widget.widget_uid()));
            }
        }

        // Check captured actions for RadioButton clicks
        for action in &actions {
            if let Some(action) = action.as_widget_action() {
                match action.cast::<RadioButtonAction>() {
                    RadioButtonAction::Clicked => {
                        for &(index, tab_uid) in &tab_uids {
                            if action.widget_uid == tab_uid && index != self.active_index {
                                self.active_index = index;
                                let content = self.view.widget(ids!(content));
                                if let Some(mut pf) = content.borrow_mut::<PageFlip>() {
                                    pf.set_active_page(cx, PAGE_IDS[index]);
                                }
                                cx.widget_action(uid, &scope.path, ElementTabViewAction::TabChanged(index));
                                self.view.redraw(cx);
                                break;
                            }
                        }
                    }
                    _ => ()
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// Extension trait for handling TabView actions from parent widgets
pub trait ElementTabViewActionExt {
    fn tab_changed(&self, actions: &Actions) -> Option<usize>;
}

impl ElementTabViewActionExt for WidgetRef {
    fn tab_changed(&self, actions: &Actions) -> Option<usize> {
        for action in actions {
            if let Some(action) = action.as_widget_action() {
                if action.widget_uid == self.widget_uid() {
                    if let ElementTabViewAction::TabChanged(index) = action.cast() {
                        return Some(index);
                    }
                }
            }
        }
        None
    }
}
