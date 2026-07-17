use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

#[test]
fn keep_inline_images_in_applies_inside_layout_table_cells() {
    let html = r#"
        <table role="presentation"><tr><td>
            <p><span><img src="image.png" alt="Image"></span></p>
        </td></tr></table>
    "#;
    let options = ConversionOptions {
        keep_inline_images_in: vec!["td".to_string(), "th".to_string()],
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    };

    let markdown = convert(html, Some(options))
        .unwrap()
        .content
        .unwrap_or_default();

    assert!(markdown.contains("![Image](image.png)"), "{markdown}");
}

#[test]
fn deeply_nested_email_wrappers_do_not_drop_content() {
    let mut html = String::new();
    for _ in 0..128 {
        html.push_str("<div>");
    }
    html.push_str("<p>Signature text</p><p><img src=\"signature.png\"></p>");
    for _ in 0..128 {
        html.push_str("</div>");
    }

    let markdown = convert(&html, None).unwrap().content.unwrap_or_default();

    assert!(markdown.contains("Signature text"), "{markdown}");
    assert!(markdown.contains("![](signature.png)"), "{markdown}");
}
