use discord_md::ast::{self, MarkdownDocument, MarkdownElement};
use markdown::ListItem;
use markdown::{
    Block::{Blockquote, CodeBlock, Header, Hr, OrderedList, Paragraph, Raw, UnorderedList},
    Span::{self, Break, Code, Emphasis, Image, Link, Strong, Text},
};

pub fn parse_string(input: &str) -> String {
    println!("Hello, world!");

    // let str = "## What's Changed\r\n* fix dockerfile envs by @awareness481 in https://github.com/awareness481/test-repo/pull/1\r\n\r\n## New Contributors\r\n* @awareness481 made their first contribution in https://github.com/awareness481/test-repo/pull/1\r\n\r\n**Full Changelog**: https://github.com/awareness481/test-repo/commits/v0.0.2test";
    let t = markdown::tokenize(input);
    let mut md_ast: Vec<discord_md::ast::MarkdownElement> = vec![];

    for i in t {
        match i {
            Header(headers, _) => {
                let mut string = String::new();
                for header in headers {
                    string.push_str(&match_span(&header))
                }
                md_ast.push(MarkdownElement::Bold(Box::new(ast::Bold::new(format!(
                    "\n{string}"
                )))));
            }
            Paragraph(paragraph) => {
                let mut string = String::new();
                for text in paragraph {
                    string.push_str(&match_span(&text))
                }
                md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(format!(
                    "\n{string}"
                )))));
            }
            Blockquote(content) => {
                println!("Quote: {:?}", content);
            }
            CodeBlock(content, _) => {
                println!("Codeblock: {:?}", content);
            }
            OrderedList(content, _) => {
                for item in content {
                    md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(
                        "\n".to_string(),
                    ))));
                    match item {
                        ListItem::Simple(content) => {
                            let mut string = String::new();
                            for text in content {
                                string.push_str(&match_span(&text))
                            }
                            md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(
                                format!("\n• {string}"),
                            ))));
                        }
                        ListItem::Paragraph(content) => {
                            println!("Paragraph: {:?}", content);
                        }
                    }
                }
                md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(
                    "\n".to_string(),
                ))));
            }
            UnorderedList(content) => {
                for item in content {
                    md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(
                        "\n".to_string(),
                    ))));
                    match item {
                        ListItem::Simple(content) => {
                            let mut string = String::new();
                            for text in content {
                                string.push_str(&match_span(&text))
                            }
                            md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(
                                format!("\n• {string}"),
                            ))));
                        }
                        ListItem::Paragraph(content) => {
                            println!("Paragraph: {:?}", content);
                        }
                    }
                }
                md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(
                    "\n".to_string(),
                ))));
            }
            Raw(content) => {
                md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new(format!(
                    " {content} "
                )))));
            }
            Hr => {
                md_ast.push(MarkdownElement::Plain(Box::new(ast::Plain::new("\n"))));
            }
        }
    }
    let md = MarkdownDocument::new(md_ast);

    // dbg!(&md_ast);
    // dbg!(md.to_string());
    md.to_string()
}

fn match_span(span: &Span) -> String {
    match span {
        Break => String::from(""),
        Text(text) => format!(" {text} "),
        Code(code) => format!(" {code} "),
        Link(_, _, _) => "".to_string(),
        Image(_, _, _) => todo!(),
        Emphasis(_) => todo!(),
        Strong(s) => {
            let mut string = String::new();
            for text in s {
                string.push_str(&match_span(text))
            }
            format!("**{string}**")
        }
    }
}
