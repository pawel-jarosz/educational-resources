use marie_toolbox::Address;

type Label = String;

#[derive(Debug, PartialEq)]
pub enum HandleResult {
    Instruction(String),
    LabeledInstruction(Label, String),
    SectionData(Address),
    SectionCode(Address),
    Empty
}

pub struct AssemblerParser {

}

fn clear_comments(line: &str) -> &str {
    let has_comment = line.contains("--");
    if has_comment {
        let end = line.find("--").unwrap();
        &line[0..end].trim()
    }
    else {
        line.trim()
    }
}

fn extract_label(line: &str) -> Option<HandleResult> {
    let has_comment = line.find(':');
    if has_comment.is_some() {
        let label = line[..has_comment.unwrap()].to_string();
        let instruction = line[(has_comment.unwrap() + 1)..].trim().to_string();
        Some(HandleResult::LabeledInstruction(label, instruction))
    }
    else {
        None
    }
}

fn extract_sections(line: &str) -> Option<HandleResult> {
    println!("line: {}", line);
    if line.starts_with("..section data") {
        let line = line.replace("..section data 0x", "");
        Some(HandleResult::SectionCode(u16::from_str_radix(line.trim(), 16).unwrap()))
    }
    else if line.starts_with("..section code") {
        let line = line.replace("..section code 0x", "");
        Some(HandleResult::SectionData(u16::from_str_radix(line.trim(), 16).unwrap()))
    }
    else {
        None
    }
}

impl AssemblerParser {
    pub fn new() -> Self {
        AssemblerParser { 

        }
    }

    pub fn handle_line(&self, line: &str) -> HandleResult {
        let ignored_comments: &str = clear_comments(line);
        if ignored_comments.trim().is_empty() {
            return HandleResult::Empty
        }
        let handled_sections = extract_sections(ignored_comments);
        if let Some(result) = handled_sections {
            return result
        }
        let labeled_line = extract_label(ignored_comments);
        if let Some(result) = labeled_line {
            return result
        }
        else {
            HandleResult::Instruction(line.trim().to_string())
        }
    }
}

