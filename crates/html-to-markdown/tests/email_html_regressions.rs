#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn convert_tier2(html: &str, configure: impl FnOnce(&mut ConversionOptions)) -> String {
    let mut options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    configure(&mut options);
    convert(html, Some(options))
        .unwrap()
        .content
        .unwrap_or_default()
        .trim_end()
        .to_owned()
}

#[test]
fn newline_only_text_between_inline_elements_preserves_a_word_separator() {
    let html = "<p><b><span>1 mezzo</span></b><span>\n</span><span>con carico</span></p>";

    let markdown = convert_tier2(html, |_| {});

    assert_eq!(markdown, "**1 mezzo** con carico");
}

#[test]
fn paragraph_after_table_in_blockquote_starts_on_a_new_line() {
    let html = concat!(
        "<blockquote>",
        "<table><tr><td>A</td><td>B</td></tr></table>",
        "<p><span>After</span></p>",
        "</blockquote>"
    );

    let markdown = convert_tier2(html, |options| options.compact_tables = true);

    assert_eq!(markdown, "> | A | B |\n> | --- | --- |\n>\n> After");
}

#[test]
fn explicit_br_in_table_uses_literal_html_break_when_enabled() {
    let html = "<table><tr><td>First line<br>Second line</td><td>Other</td></tr></table>";

    let markdown = convert_tier2(html, |options| {
        options.br_in_tables = true;
        options.compact_tables = true;
    });

    assert_eq!(markdown, "| First line<br>Second line | Other |\n| --- | --- |");
}
