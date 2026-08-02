use marie_toolbox::asm::assembly_helpers::HandleResult;

pub struct ObjectContentFactory {
    collected_instructions: Vec<HandleResult>,
}

impl ObjectContentFactory {
    pub fn new() -> Self {
        ObjectContentFactory {
            collected_instructions: Vec::new(),
        }
    }

    pub fn push_instruction(&mut self, instruction: assembly_helpers::HandleResult) {
    
    }
}
