use markdown::mdast::Node;
extern crate markdown;

pub fn parse_string(input: &str) -> String {
    let md = markdown::to_mdast(input, &markdown::ParseOptions::gfm()).unwrap();
    match_span(&md)
}

fn match_span(node: &Node) -> String {
    match node {
        Node::Root(root) => root.children.iter().map(match_span).collect(),
        Node::BlockQuote(_) => unimplemented!(),
        Node::FootnoteDefinition(_) => unimplemented!(),
        Node::MdxJsxFlowElement(_) => unimplemented!(),
        Node::List(span) => span.children.iter().map(match_span).collect::<String>(),
        Node::MdxjsEsm(_) => unimplemented!(),
        Node::Toml(_) => unimplemented!(),
        Node::Yaml(_) => unimplemented!(),
        Node::Break(_) => unimplemented!(),
        Node::InlineCode(_) => unimplemented!(),
        Node::InlineMath(_) => unimplemented!(),
        Node::Delete(_) => unimplemented!(),
        Node::Emphasis(_) => unimplemented!(),
        Node::MdxTextExpression(_) => unimplemented!(),
        Node::FootnoteReference(_) => unimplemented!(),
        Node::Html(_) => unimplemented!(),
        Node::Image(_) => unimplemented!(),
        Node::ImageReference(_) => unimplemented!(),
        Node::MdxJsxTextElement(_) => unimplemented!(),
        Node::Link(link) => {
            if link.url.contains("/pull/") {
                let pos = link.url.rfind('/').unwrap();
                return format!("#{}, ", link.url[pos + 1..].to_string());
            }
            "".to_string()
        }
        Node::LinkReference(_) => unimplemented!(),
        Node::Strong(span) => format!(
            "**{}**",
            span.children.iter().map(match_span).collect::<String>()
        ),
        Node::Text(span) => span.value.to_string(),
        Node::Code(_) => unimplemented!(),
        Node::Math(_) => unimplemented!(),
        Node::MdxFlowExpression(_) => unimplemented!(),
        Node::Heading(span) => format!(
            "\n**{}**\n",
            span.children.iter().map(match_span).collect::<String>()
        ),
        Node::Table(_) => unimplemented!(),
        Node::ThematicBreak(_) => unimplemented!(),
        Node::TableRow(_) => unimplemented!(),
        Node::TableCell(_) => unimplemented!(),
        Node::ListItem(span) => span
            .children
            .iter()
            .map(|child| {
                let string = format!("- {}", match_span(child));
                // dbg!(&string.ends_with(','), &string);
                if string.ends_with(", ") {
                    return format!("{}\n", string[0..string.len() - 2].to_string());
                }
                format!("{string}\n")
            })
            .collect::<String>(),
        Node::Definition(_) => unimplemented!(),
        Node::Paragraph(span) => span.children.iter().map(match_span).collect::<String>(),
    }
}
