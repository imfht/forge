use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::content::post::TocEntry;

pub struct MarkdownRenderer {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    theme_name: String,
    syntax_highlighting: bool,
    generate_toc: bool,
}

impl MarkdownRenderer {
    pub fn new(theme_name: &str, syntax_highlighting: bool, generate_toc: bool) -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            theme_name: theme_name.to_string(),
            syntax_highlighting,
            generate_toc,
        }
    }

    pub fn render(&self, markdown: &str) -> (String, Vec<TocEntry>) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown, options);
        let events: Vec<Event> = parser.collect();

        let mut toc = Vec::new();
        let mut html_output = String::new();
        let mut in_code_block = false;
        let mut code_lang = String::new();
        let mut code_content = String::new();
        let mut in_heading = false;
        let mut heading_level = 0u32;
        let mut heading_text = String::new();
        let mut heading_counter: u32 = 0;

        for event in events {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    in_code_block = true;
                    code_content.clear();
                    code_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    if self.syntax_highlighting && !code_lang.is_empty() {
                        let syntax = self
                            .syntax_set
                            .find_syntax_by_token(&code_lang)
                            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
                        let theme = self
                            .theme_set
                            .themes
                            .get(&self.theme_name)
                            .unwrap_or_else(|| {
                                self.theme_set.themes.values().next().unwrap()
                            });
                        match highlighted_html_for_string(
                            &code_content,
                            &self.syntax_set,
                            syntax,
                            theme,
                        ) {
                            Ok(highlighted) => html_output.push_str(&highlighted),
                            Err(_) => {
                                html_output.push_str("<pre><code>");
                                html_output
                                    .push_str(&html_escape(&code_content));
                                html_output.push_str("</code></pre>");
                            }
                        }
                    } else {
                        let class = if code_lang.is_empty() {
                            String::new()
                        } else {
                            format!(" class=\"language-{}\"", code_lang)
                        };
                        html_output.push_str(&format!("<pre><code{class}>"));
                        html_output.push_str(&html_escape(&code_content));
                        html_output.push_str("</code></pre>\n");
                    }
                }
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    heading_text.clear();
                    heading_level = level as u32;
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    heading_counter += 1;
                    let id = slug::slugify(&heading_text);
                    let anchor_id = if id.is_empty() {
                        format!("heading-{heading_counter}")
                    } else {
                        id.clone()
                    };

                    html_output.push_str(&format!(
                        "<h{lvl} id=\"{anchor_id}\"><a href=\"#{anchor_id}\" class=\"anchor\">#</a> {text}</h{lvl}>\n",
                        lvl = heading_level,
                        text = heading_text,
                    ));

                    if self.generate_toc {
                        toc.push(TocEntry {
                            level: heading_level,
                            id: anchor_id,
                            title: heading_text.clone(),
                        });
                    }
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_content.push_str(&text);
                    } else if in_heading {
                        heading_text.push_str(&text);
                    } else {
                        html_output.push_str(&text);
                    }
                }
                Event::Code(code) => {
                    if in_heading {
                        heading_text.push_str(&format!("<code>{code}</code>"));
                    } else {
                        html_output.push_str(&format!("<code>{code}</code>"));
                    }
                }
                Event::SoftBreak => {
                    if in_heading {
                        heading_text.push(' ');
                    } else {
                        html_output.push('\n');
                    }
                }
                Event::HardBreak => {
                    html_output.push_str("<br />\n");
                }
                _ => {
                    // For all other events, use pulldown-cmark's HTML rendering
                    let mut tmp = String::new();
                    pulldown_cmark::html::push_html(&mut tmp, std::iter::once(event));
                    html_output.push_str(&tmp);
                }
            }
        }

        (html_output, toc)
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
