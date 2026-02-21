use std::borrow::Cow;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag, TagEnd};

pub(super) fn convert<'a, 'b>(events: impl IntoIterator<Item = Event<'a>> + 'b) -> impl Iterator<Item = Event<'a>> + 'b {
    let mut in_codeblock = None;
    events.into_iter().map(move |mut event| {
        if let Some(is_rust) = in_codeblock {
            match &mut event {
                Event::Text(text) => {
                    if !text.ends_with('\n') {
                        *text = format!("{text}\n").into();
                    }
                    if is_rust {
                        *text = text
                            .lines()
                            .filter_map(|line| {
                                let trimmed = line.trim();
                                if trimmed.starts_with("##") {
                                    Some(Cow::Owned(line.replacen("##", "#", 1)))
                                } else if trimmed.starts_with("# ") {
                                    None
                                } else if trimmed == "#" {
                                    None
                                } else {
                                    Some(Cow::Borrowed(line))
                                }
                            })
                            .flat_map(|line| [line, Cow::Borrowed("\n")])
                            .collect::<String>()
                            .into();
                    }
                }
                Event::End(TagEnd::CodeBlock) => {}
                _ => unreachable!(),
            }
        }

        match &mut event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let is_rust;
                match kind {
                    CodeBlockKind::Indented => {
                        is_rust = true;
                        *kind = CodeBlockKind::Fenced("rust".into());
                    }
                    CodeBlockKind::Fenced(tag) => {
                        is_rust = update_codeblock_tag(tag);
                    }
                }

                assert!(in_codeblock.is_none());
                in_codeblock = Some(is_rust);
            }
            Event::End(TagEnd::CodeBlock) => {
                assert!(in_codeblock.is_some());
                in_codeblock = None;
            }
            _ => {}
        }
        event
    })
}

fn is_attribute_tag(tag: &str) -> bool {
    matches!(tag, "" | "ignore" | "should_panic" | "no_run" | "compile_fail" | "standalone_crate" | "test_harness")
        || tag
            .strip_prefix("edition")
            .map(|x| x.len() == 4 && x.chars().all(|ch| ch.is_ascii_digit()))
            .unwrap_or_default()
}

fn update_codeblock_tag(tag: &mut CowStr<'_>) -> bool {
    let mut tag_count = 0;
    let is_rust = tag.split(',').filter(|tag| !is_attribute_tag(tag)).all(|tag| {
        tag_count += 1;
        tag == "rust"
    });
    if is_rust && tag_count == 0 {
        if tag.is_empty() {
            *tag = "rust".into();
        } else {
            *tag = format!("rust,{tag}").into();
        }
    }
    is_rust
}

