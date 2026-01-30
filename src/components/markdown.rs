use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_code_editor::code_view::CodeView;
    use math_widget::math::MathView;

    // Element Markdown — themed wrapper around Makepad's Markdown widget
    // Note: font_color is set because TextFlow.draw_text() overwrites
    // draw_normal.color with font_color each frame (see text_flow.rs:706).
    pub ElementMarkdown = <Markdown> {
        width: Fill, height: Fit,

        font_color: #212121,

        // Enable syntax-highlighted code blocks via CodeView
        use_code_block_widget: true,

        // Enable math rendering via MathView widget (typst + mitex)
        use_math_widget: true,
        inline_math = <MathView> {
            color: #212121,
        }
        display_math = <MathView> {
            color: #212121,
        }

        code_block = <View> {
            width: Fill, height: Fit,
            flow: Overlay,
            margin: {top: 4, bottom: 4},

            code_view = <CodeView> {
                editor: {
                    height: Fit,
                    width: Fill,
                    pad_left_top: vec2(8.0, 8.0),
                    draw_bg: { color: #f5f5f5 }

                    // Light-theme token colors (VS Code Light+ inspired)
                    token_colors: {
                        unknown: #333333,
                        branch_keyword: #af00db,
                        comment: #008000,
                        constant: #0451a5,
                        delimiter: #333333,
                        identifier: #001080,
                        loop_keyword: #af00db,
                        number: #098658,
                        other_keyword: #0000ff,
                        punctuator: #333333,
                        string: #a31515,
                        function: #795e26,
                        typename: #267f99,
                    }

                    draw_text: {
                        text_style: { font_size: 9.0 }
                    }
                }
            }
        }

        draw_normal: {
            color: #212121,
        }
        draw_italic: {
            color: #212121,
        }
        draw_bold: {
            color: #111111,
        }
        draw_bold_italic: {
            color: #111111,
        }
        draw_fixed: {
            color: #212121,
        }
    }
}
