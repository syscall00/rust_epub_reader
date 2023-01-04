use druid::{im::Vector, text::RichText, Data, ArcStr};

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
#[derive(Debug, Data, Clone)]
pub enum Renderable {
    Image(ArcStr),
    Text(RichText),
}


pub fn generate_renderable_tree(text: &str, font_size: f64) -> Vector<Renderable> {
    let mut renderables: Vector<Renderable> = Vector::new();
    let mut current_pos = 0;
    let mut builder = druid::text::RichTextBuilder::new();
    let mut token_stack: Vec<(usize, HtmlTag)> = Vec::new();

    for tok_result in xmlparser::Tokenizer::from(text) {
        if tok_result.is_err() {
            // handle error
            continue;
        }
        let token = tok_result.unwrap();
        match token {
            xmlparser::Token::ElementStart {
                prefix : _,
                local,
                span: _,
            } => {
                token_stack.push((current_pos, HtmlTag::from(local.as_str())));
            }
            xmlparser::Token::ElementEnd { end, span : _} => {
                match end {
                    xmlparser::ElementEnd::Open => {
                        continue;
                    }

                    xmlparser::ElementEnd::Close(t, closed_token) => {
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
                            renderables.push_back(Renderable::Text(text));

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
                local,
                value,
                span : _,
            } => {
                // check if tag is image
                let (_, inner_tag) = token_stack.last().unwrap_or(&(0, HtmlTag::Unhandled));
                if matches!(HtmlTag::Image, inner_tag) && local.as_str() == "src"  {
                    println!("Attribute: {:?} {:?} ", local, value);
                    //renderables.push_back(Renderable::Image(ArcStr::from(value.as_str())));

                }
            }

            _ => continue,

        }
    }

    renderables
}