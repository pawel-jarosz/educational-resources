enum InstructionSet {
    Jns(u16),
    Load(u16),
    Store(u16),
    Add(u16),
    Subt(u16),
    Input,
    Output,
    Halt,
    Skipcond(u16),
    Jump(u16),
    Clear,
    AddI(u16),
    JumpI(u16),
    LoadI(u16),
    StoreI(u16),
    Nop,
}

impl InstructionSet {
    fn encode_instruction(opcode: u8, address: u16) -> Result<u16, String> {
        if address > 0x0FFF {
            Err(format!("Address {} exceeds 12-bit limit", address))
        }
        else {
            Ok(((opcode as u16) << 12) | (address & 0x0FFF))
        }
    }

    
    fn from_string(instruction: &str) -> Result<InstructionSet, String> {
        let collected = instruction.trim().to_uppercase();
        let collected = collected.split(' ').collect::<Vec<&str>>();

        // TODO: Check address value and arguments count
        match (collected[0], collected.len()) {
            ("JNS", 2) => Ok(InstructionSet::Jns(collected[1].parse().unwrap())),
            ("LOAD", 2) => Ok(InstructionSet::Load(collected[1].parse().unwrap())),
            ("STORE", 2) => Ok(InstructionSet::Store(collected[1].parse().unwrap())),
            ("ADD", 2) => Ok(InstructionSet::Add(collected[1].parse().unwrap())),
            ("SUBT", 2) => Ok(InstructionSet::Subt(collected[1].parse().unwrap())),
            ("INPUT", 1) => Ok(InstructionSet::Input),
            ("OUTPUT", 1) => Ok(InstructionSet::Output),
            ("HALT", 1) => Ok(InstructionSet::Halt),
            ("SKIPCOND", 2) => Ok(InstructionSet::Skipcond(collected[1].parse().unwrap())),
            ("JUMP", 2) => Ok(InstructionSet::Jump(collected[1].parse().unwrap())),
            ("CLEAR", 1) => Ok(InstructionSet::Clear),
            ("ADDI", 2) => Ok(InstructionSet::AddI(collected[1].parse().unwrap())),
            ("JUMPI", 2) => Ok(InstructionSet::JumpI(collected[1].parse().unwrap())),
            ("LOADI", 2) => Ok(InstructionSet::LoadI(collected[1].parse().unwrap())),
            ("STOREI", 2) => Ok(InstructionSet::StoreI(collected[1].parse().unwrap())),
            ("NOP", 1) => Ok(InstructionSet::Nop),
            _ => Err("Invalid arguments number".to_string())
        }
    }
    
    fn to_string(&self) -> String {
        match self {
            InstructionSet::Jns(addr) => format!("JNS {}", addr),
            InstructionSet::Load(addr) => format!("LOAD {}", addr),
            InstructionSet::Store(addr) => format!("STORE {}", addr),
            InstructionSet::Add(addr) => format!("ADD {}", addr),
            InstructionSet::Subt(addr) => format!("SUBT {}", addr),
            InstructionSet::Input => "INPUT".to_string(),
            InstructionSet::Output => "OUTPUT".to_string(),
            InstructionSet::Halt => "HALT".to_string(),
            InstructionSet::Skipcond(code) => format!("SKIPCOND {}", code),
            InstructionSet::Jump(addr) => format!("JUMP {}", addr),
            InstructionSet::Clear => "CLEAR".to_string(),
            InstructionSet::AddI(addr) => format!("ADDI {}", addr),
            InstructionSet::JumpI(addr) => format!("JUMPI {}", addr),
            InstructionSet::LoadI(addr) => format!("LOADI {}", addr),
            InstructionSet::StoreI(addr) => format!("STOREI {}", addr),
            InstructionSet::Nop => "NOP".to_string(),
        }
    }

    fn from_binary(word: u16) -> Result<InstructionSet, String> {
        let opcode = ((word >> 12) & 0x000F) as u8;
        let address = word & 0x0FFF;

        match opcode {
            0b0000 => Ok(InstructionSet::Jns(address)),
            0b0001 => Ok(InstructionSet::Load(address)),
            0b0010 => Ok(InstructionSet::Store(address)),
            0b0011 => Ok(InstructionSet::Add(address)),
            0b0100 => Ok(InstructionSet::Subt(address)),
            0b0101 => Ok(InstructionSet::Input),
            0b0110 => Ok(InstructionSet::Output),
            0b0111 => Ok(InstructionSet::Halt),
            0b1000 => Ok(InstructionSet::Skipcond(address)),
            0b1001 => Ok(InstructionSet::Jump(address)),
            0b1010 => Ok(InstructionSet::Clear),
            0b1011 => Ok(InstructionSet::AddI(address)),
            0b1100 => Ok(InstructionSet::JumpI(address)),
            0b1101 => Ok(InstructionSet::LoadI(address)),
            0b1110 => Ok(InstructionSet::StoreI(address)),
            0b1111 => Ok(InstructionSet::Nop),
            _ => Err(format!("Invalid opcode: {}", opcode)),
        }
    }

    fn to_binary(&self) -> Result<u16, String> {
        match self {
            InstructionSet::Jns(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::Load(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::Store(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::Add(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::Subt(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::Input => InstructionSet::encode_instruction(self.get_opcode(), 0),
            InstructionSet::Output => InstructionSet::encode_instruction(self.get_opcode(), 0),
            InstructionSet::Halt => InstructionSet::encode_instruction(self.get_opcode(), 0),
            InstructionSet::Skipcond(code) => InstructionSet::encode_instruction(self.get_opcode(), *code),
            InstructionSet::Jump(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::Clear => InstructionSet::encode_instruction(self.get_opcode(), 0),
            InstructionSet::AddI(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::JumpI(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::LoadI(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::StoreI(addr) => InstructionSet::encode_instruction(self.get_opcode(), *addr),
            InstructionSet::Nop => InstructionSet::encode_instruction(self.get_opcode(), 0),
        }
    }

    fn get_opcode(&self) -> u8 {
        match self {
            InstructionSet::Jns(_) => 0b0000,
            InstructionSet::Load(_) => 0b0001,
            InstructionSet::Store(_) => 0b0010,
            InstructionSet::Add(_) => 0b0011,
            InstructionSet::Subt(_) => 0b0100,
            InstructionSet::Input => 0b0101,
            InstructionSet::Output => 0b0110,
            InstructionSet::Halt => 0b0111,
            InstructionSet::Skipcond(_) => 0b1000,
            InstructionSet::Jump(_) => 0b1001,
            InstructionSet::Clear => 0b1010,
            InstructionSet::AddI(_) => 0b1011,
            InstructionSet::JumpI(_) => 0b1100,
            InstructionSet::LoadI(_) => 0b1101,
            InstructionSet::StoreI(_) => 0b1110,
            InstructionSet::Nop => 0b1111,
        }
    }

    fn get_address(&self) -> Option<u16> {
        match self {
            InstructionSet::Jns(addr) => Some(*addr),
            InstructionSet::Load(addr) => Some(*addr),
            InstructionSet::Store(addr) => Some(*addr),
            InstructionSet::Add(addr) => Some(*addr),
            InstructionSet::Subt(addr) => Some(*addr),
            InstructionSet::Skipcond(code) => Some(*code),
            InstructionSet::Jump(addr) => Some(*addr),
            InstructionSet::AddI(addr) => Some(*addr),
            InstructionSet::JumpI(addr) => Some(*addr),
            InstructionSet::LoadI(addr) => Some(*addr),
            InstructionSet::StoreI(addr) => Some(*addr),
            _ => None,
        }
    }
}

impl std::fmt::Display for InstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_instruction_to_string() {
        let instr = InstructionSet::Add(42);
        assert_eq!(instr.to_string(), "ADD 42");

        let instr = InstructionSet::Input;
        assert_eq!(instr.to_string(), "INPUT");
    }

    #[test]
    fn test_instruction_to_binary() {
        let instr = InstructionSet::Add(42);
        assert_eq!(instr.to_binary().unwrap(), 0b0011_0000_0010_1010);

        let instr = InstructionSet::Input;
        assert_eq!(instr.to_binary().unwrap(), 0b0101_0000_0000_0000);

        let instr: InstructionSet = InstructionSet::Add(0xffff);
        assert!(instr.to_binary().is_err());
    }

    #[test]
    fn test_instruction_from_string() {
        assert_eq!(InstructionSet::from_string("JUMPI 42").unwrap().to_string(), "JUMPI 42".to_string());
    }
}