use druid::{im::Vector, text::RichText, Data};

use crate::core::style::LINK_COLOR;


#[derive(Debug, PartialEq, Data, Clone)]
enum HtmlTag {
    Header(u8),
    Link(i32),
    Image(String),
    Paragraph,
    Bold,
    Italic,
    Underline,
    StrikeThrough,
    Title,
    Unhandled,
}

impl From<&str> for HtmlTag {
    fn from(tag_string: &str) -> Self {
        match tag_string {
            "h1" => HtmlTag::Header(1),
            "h2" => HtmlTag::Header(2),
            "h3" => HtmlTag::Header(3),
            "h4" => HtmlTag::Header(4),
            "h5" => HtmlTag::Header(5),
            "h6" => HtmlTag::Header(6),
            "a" => HtmlTag::Link(-1),
            "img" => HtmlTag::Image("".to_string()),
            "p" => HtmlTag::Paragraph,
            "strong" | "b" => HtmlTag::Bold,
            "em" | "i" => HtmlTag::Italic,
            "u" => HtmlTag::Underline,
            "del" | "s" => HtmlTag::StrikeThrough,
            "title" => HtmlTag::Title,
            _ => HtmlTag::Unhandled,
        }
    }
}
impl HtmlTag {
    pub fn add_newline_after_tag(&self) -> bool {
        matches!(
            self,
            HtmlTag::Paragraph | HtmlTag::Header(_) | HtmlTag::Image(_) | HtmlTag::Link(_)
        )
    }

    pub fn should_tag_be_written(&self) -> bool {
        matches!(self, HtmlTag::Title)
    }

    pub fn add_attribute_for_token(&self, mut attrs: druid::text::AttributesAdder, font_size: f64) {
        match self {
            HtmlTag::Header(lvl) => {
                let font_size = font_size
                    * match lvl {
                        1 => 2.,
                        2 => 1.5,
                        3 => 1.17,
                        4 => 1.,
                        5 => 0.8375,
                        6 => 0.67,
                        _ => 1.,
                    };
                attrs.size(font_size).weight(druid::FontWeight::BOLD);
            }
            HtmlTag::Bold => {
                attrs.weight(druid::FontWeight::BOLD);
            }
            HtmlTag::Italic => {
                attrs.style(druid::FontStyle::Italic);
            }
            HtmlTag::Underline => {
                attrs.underline(true);
            }
            HtmlTag::StrikeThrough => {
                attrs.strikethrough(true);
            }
            HtmlTag::Link(_target) => {
                //Tag::Link(_link_ty, target, _title) => {
                attrs.underline(true).text_color(LINK_COLOR);
            }
            HtmlTag::Image(_img) => {}
            _ => {
                return;
            }
        }
    }
}


// Create an enum both for render images or text
enum Renderable {
    Image(()),
    Text(RichText),
}


impl Renderable {
    fn render(&self) {
        match self {
            Renderable::Image(()) => {
                // Render image
            }
            Renderable::Text(ref text) => {
                // Render text
            }
        }
    }
}

pub fn rebuild_rendered_text(text: &str, font_size: f64) -> Vector<RichText> {
    let mut current_pos = 0;
    let mut builder = druid::text::RichTextBuilder::new();
    let mut token_stack: Vec<(usize, HtmlTag)> = Vec::new();

    let mut richtexts: Vector<RichText> = Vector::new();

    for tok_result in xmlparser::Tokenizer::from(text) {
        if tok_result.is_err() {
            // handle error
            continue;
        }
        let token = tok_result.unwrap();
        match token {
            xmlparser::Token::ElementStart {
                prefix: _,
                local,
                span: _,
            } => {
                token_stack.push((current_pos, HtmlTag::from(local.as_str())));
            }
            xmlparser::Token::ElementEnd { end, span: _ } => {
                match end {
                    xmlparser::ElementEnd::Open => {
                        continue;
                    }

                    xmlparser::ElementEnd::Close(_, closed_token) => {
                        let (pos, tk) = token_stack.pop().expect("No token on stack");
                        if tk != HtmlTag::from(closed_token.as_str()) {
                            println!(
                                "ERROR: closing tag {:?} does not match started tag {:?}",
                                closed_token.as_str(),
                                tk
                            );
                            continue;
                        }

                        tk.add_attribute_for_token(
                            builder.add_attributes_for_range(pos..current_pos),
                            font_size,
                        );

                        if tk != HtmlTag::Unhandled && tk.add_newline_after_tag() {
                            //current_pos += 1;

                            //builder.push("\n");
                        }

                        if matches!(
                            tk,
                            HtmlTag::Paragraph
                                | HtmlTag::Header(_)
                                | HtmlTag::Image(_)
                                | HtmlTag::Link(_)
                        ) {
                            if current_pos == 0 {
                                continue;
                            }
                            let text = builder.build();
                            richtexts.push_back(text);

                            builder = druid::text::RichTextBuilder::new();
                            current_pos = 0;
                        }
                    }
                    xmlparser::ElementEnd::Empty => {
                        token_stack.pop().expect("No token on stack");
                    }
                }
            }

            xmlparser::Token::Text { text } => {
                // TODO: Handle case of no tags, text only
                let (_, inner_tag) = token_stack.last().unwrap();

                if inner_tag.should_tag_be_written() || text.trim().is_empty() {
                    continue;
                } else {
                    let t = text.as_str().replace("\n", "");
                    current_pos = current_pos + t.len();

                    builder.push(&t);
                }
            }
            xmlparser::Token::Attribute {
                prefix: _,
                local: _,
                value: _,
                span: _,
            } => {
                //println!("attr: {:?} = {:?}", loc, val);
                continue;
            }

            _ => continue,
            /*
            xmlparser::Token::Declaration { version, encoding, standalone, span } => {
                // for now, ignore declarations
                continue;
            },
            xmlparser::Token::EmptyDtd { nfame, external_id, span } => {
                // for now, ignore the DTD
                continue;
            },

            xmlparser::Token::ProcessingInstruction { target, content, span } => todo!(),
            xmlparser::Token::DtdStart { name, external_id, span } => todo!(),
            xmlparser::Token::EntityDeclaration { name, definition, span } => todo!(),
            xmlparser::Token::DtdEnd { span } => todo!(),

            */
        }
    }
    richtexts
}
