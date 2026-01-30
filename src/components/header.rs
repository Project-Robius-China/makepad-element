use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::live_theme::*;

    // App bar / header (RNE: paddingH 10, paddingV 10, borderBottomColor #f2f2f2)
    pub ElementHeader = <ElementHeaderBase> {}

    // Transparent header
    pub ElementHeaderTransparent = <ElementHeaderBase> {
        bar = {
            show_bg: false,
            center = {
                title = {
                    draw_text: { color: #333333 }
                }
            }
        }
    }
}
