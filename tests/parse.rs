use md_parser::{Builder, HeaderLevel, Line, Parser, Token};

fn parse(input: &str) -> Vec<Line> {
    let mut builder = Builder::new();
    let mut parser = Parser::new(&mut builder);
    parser.parse(input);
    builder.get_document()
}

#[test]
fn parse_header() {
    use HeaderLevel::*;
    use Token::*;
    assert_eq!(
        parse("# H1"),
        vec![Line::Header {
            level: H1,
            tokens: vec![Regular("H1".into())]
        }]
    );

    assert_eq!(
        parse("## H2"),
        vec![Line::Header {
            level: H2,
            tokens: vec![Regular("H2".into())]
        }]
    );

    assert_eq!(
        parse("### H3"),
        vec![Line::Header {
            level: H3,
            tokens: vec![Regular("H3".into())]
        }]
    );
}

#[test]
fn parse_bold() {
    // lexer.lex("regular **bold** word");
    // lexer.lex("and __another__ bold word");
    // lexer.lex("**bold with spaces**");

    use Token::*;
    assert_eq!(
        parse("**bold**"),
        &[Line::Paragraph(vec![Bold(vec![Regular(
            "bold".to_string()
        )])])]
    );
}

#[test]
fn parse_italic() {
    // lexer.lex("regular *italic* word");
    // lexer.lex("and _another_ italic word");
    // lexer.lex("*italic with spaces*");

    use Token::*;
    assert_eq!(
        parse("*italic*"),
        &[Line::Paragraph(vec![Italic(vec![Regular(
            "italic".to_string()
        )])])]
    );
}

#[test]
#[ignore]
fn parse_link() {
    // lexer.lex("and [Another Link](https://b.com) with spaces");

    use Token::*;
    assert_eq!(
        parse("a regular [Link](https://a.com)"),
        &[
            Line::Paragraph(vec![
                Regular("a".to_string()),
                Regular("regular".to_string()),
                Link {
                    label: vec![Token::Regular("Link".to_string())],
                    url: "https://a.com".to_string()
                }
            ]),
            Line::Paragraph(vec![
                Regular("and".to_string()),
                Link {
                    label: vec![Regular("Another".to_string()), Regular("Link".to_string())],
                    url: "https://b.com".to_string()
                },
                Regular("with".to_string()),
                Regular("spaces".to_string())
            ])
        ]
    );
}

#[test]
#[ignore]
fn parse_bold_link() {
    // lexer.lex("**[Bold](https://a.com)**");

    use Token::*;
    assert_eq!(
        parse("**[Bold](https://a.com)**"),
        &[Line::Paragraph(vec![Bold(vec![Link {
            label: vec![Regular("Bold".to_string()),],
            url: "https://a.com".to_string()
        }])])]
    );
}

#[test]
#[ignore]
fn lex_image() {
    // lexer.lex("![image](https://www.a.com)");

    use Token::*;
    assert_eq!(
        parse("![image](https://www.a.com)"),
        vec![Line::Image {
            label: vec![Regular("image".to_string())],
            url: "https://www.a.com".to_string()
        }]
    );
}

#[test]
#[ignore]
fn parse_inline_code() {
    // lexer.lex("a `code with spaces`.");

    use Token::*;
    assert_eq!(
        parse("regular `code` word"),
        vec![
            Line::Paragraph(
                [
                    Regular("regular".into()),
                    InlineCode([Regular("code".into())].to_vec()),
                    Regular("word".into())
                ]
                .to_vec()
            ),
            Line::Paragraph(
                [
                    Regular("a".into()),
                    InlineCode(
                        [
                            Regular("code".into()),
                            Regular("with".into()),
                            Regular("spaces.".into())
                        ]
                        .to_vec()
                    )
                ]
                .to_vec()
            )
        ]
    );
}

#[test]
#[ignore]
fn parse_bold_italic() {
    use Token::*;
    assert_eq!(
        parse("***strong emph***"),
        vec![Line::Paragraph(vec![Bold(vec![Italic(vec![
            Regular("strong".into()),
            Regular("emph".into())
        ])])])]
    );
}
