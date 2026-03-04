// Component tests for makepad-element
// Tests will be added as components are implemented

#[test]
fn theme_colors_valid() {
    use makepad_element::theme::colors::*;
    // Verify primary colors are in valid range
    assert!(ELEMENT_PRIMARY.x >= 0.0 && ELEMENT_PRIMARY.x <= 1.0);
    assert!(ELEMENT_PRIMARY.y >= 0.0 && ELEMENT_PRIMARY.y <= 1.0);
    assert!(ELEMENT_PRIMARY.z >= 0.0 && ELEMENT_PRIMARY.z <= 1.0);
    assert!(ELEMENT_PRIMARY.w >= 0.0 && ELEMENT_PRIMARY.w <= 1.0);
}

#[test]
fn theme_spacing_positive() {
    use makepad_element::theme::spacing::*;
    assert!(ELEMENT_SPACING_XS > 0.0);
    assert!(ELEMENT_SPACING_SM > ELEMENT_SPACING_XS);
    assert!(ELEMENT_SPACING_MD > ELEMENT_SPACING_SM);
    assert!(ELEMENT_SPACING_LG > ELEMENT_SPACING_MD);
    assert!(ELEMENT_SPACING_XL > ELEMENT_SPACING_LG);
}

#[test]
fn theme_typography_sizes() {
    use makepad_element::theme::typography::*;
    assert!(ELEMENT_FONT_H1 > ELEMENT_FONT_H2);
    assert!(ELEMENT_FONT_H2 > ELEMENT_FONT_H3);
    assert!(ELEMENT_FONT_H3 > ELEMENT_FONT_H4);
    assert!(ELEMENT_FONT_H4 > ELEMENT_FONT_BODY);
    assert!(ELEMENT_FONT_BODY > ELEMENT_FONT_CAPTION);
}

#[test]
fn rne_parity_has_airbnb_rating_component() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let module_file = root.join("src/components/airbnb_rating.rs");
    assert!(
        module_file.exists(),
        "missing component module: {:?}",
        module_file
    );

    let mod_rs = std::fs::read_to_string(root.join("src/components/mod.rs"))
        .expect("failed to read src/components/mod.rs");
    assert!(
        mod_rs.contains("pub mod airbnb_rating;"),
        "components/mod.rs missing module export for airbnb_rating"
    );
    assert!(
        mod_rs.contains("self::airbnb_rating::live_design(cx);"),
        "components/mod.rs missing live_design registration for airbnb_rating"
    );
}
