use crate::mem::Memory;

#[derive(PartialEq, Clone, Copy)]
pub enum OpCode {
    ADD(u8, u8),
    SUB(u8, u8),
    LD(u8, u8),
    LDA(u8, u8),
    AND(u8, u8),
    OR(u8, u8),
    XOR(u8, u8),
    OUT(u8),
    NOT(u8),
    HLT,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Register {
    R0 = 0x0,
    R1 = 0x1,
    R2 = 0x2,
    R3 = 0x3,
    R4 = 0x4,
    R5 = 0x5,
    R6 = 0x6,
    R7 = 0x7,
}

pub struct CPU {
    pub memory: Memory,
    pc: usize,
    regs: [u8; 8],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new(),
            pc: 0,
            regs: [0; 8],
        }
    }

    fn fetch(&mut self) -> OpCode {
        let instruction = self.memory.fetch(self.pc);
        self.pc += 1;
        return instruction
    }

    pub fn set_reg(&mut self, register: Register, value: u8) {
        let register_number = register as usize;
        self.regs[register_number] = value;
    }

    fn decode_and_execute(&mut self, instruction: OpCode) {
        match instruction {
            OpCode::ADD(a, b) => {
                let reg1 = self.regs[a as usize];
                let reg2 = self.regs[b as usize];

                self.regs[Register::R0 as usize] = reg1 + reg2;
            },
            OpCode::SUB(a, b) => {
                let reg1 = self.regs[a as usize];
                let reg2 = self.regs[b as usize];

                self.regs[Register::R0 as usize] = reg1 - reg2;
            },
            OpCode::LD(a, b) => {
                self.regs[a as usize] = b;
            }
            OpCode::LDA(a, b) => {
                let reg1 = self.regs[b as usize];

                self.regs[a as usize] = reg1;
            },
            OpCode::AND(a, b) => {
                let reg1 = self.regs[a as usize];
                let reg2 = self.regs[b as usize];

                self.regs[Register::R0 as usize] = reg1 & reg2;
            },
            OpCode::OR(a, b) => {
                let reg1 = self.regs[a as usize];
                let reg2 = self.regs[b as usize];

                self.regs[Register::R0 as usize] = reg1 | reg2;
            },
            OpCode::XOR(a, b) => {
                let reg1 = self.regs[a as usize];
                let reg2 = self.regs[b as usize];

                self.regs[Register::R0 as usize] = reg1 ^ reg2;
            },
            OpCode::OUT(a) => {
                let reg1 = self.regs[a as usize];
                println!("{}", reg1);
            },
            OpCode::NOT(a) => {
                let reg1 = self.regs[a as usize];
                self.regs[Register::R0 as usize] = if reg1 == 0 { 1 } else { 0 };
            },
            OpCode::HLT => {
                return
            }
        }
    }

    pub fn run(&mut self, instruction_set: &str) {

        let file = std::fs::read_to_string(instruction_set).expect("Unable to read file");

        let instructions: Vec<&str> = file.split("\n").collect();

        let instructions = instructions.iter().filter(|x| !x.is_empty()).collect::<Vec<_>>();

        for instruction in instructions {
            let instruction = instruction.trim();
            let instruction: Vec<&str> = instruction.split(" ").collect();

            let value = if let Some(value) = instruction.get(1) {
                value.trim().parse::<u8>().unwrap()
            } else {
                0
            };
            let second_value = if let Some(value) = instruction.get(2) {
                if value.contains('\''){

                }
                value.trim().parse::<u8>().unwrap()
            } else {
                0
            };

            let instruction = instruction[0];

            match instruction {
                "ADD" => self.memory.push(OpCode::ADD(value, second_value)),
                "SUB" => self.memory.push(OpCode::SUB(value, second_value)),
                "LD" => self.memory.push(OpCode::LD(value, second_value)),
                "LDA" => self.memory.push(OpCode::LDA(value, second_value)),
                "AND" => self.memory.push(OpCode::AND(value, second_value)),
                "OR" => self.memory.push(OpCode::OR(value, second_value)),
                "XOR" => self.memory.push(OpCode::XOR(value, second_value)),
                "OUT" => self.memory.push(OpCode::OUT(value)),
                "NOT" => self.memory.push(OpCode::NOT(value)),
                "HLT" => self.memory.push(OpCode::HLT),
                _ => panic!("Invalid instruction")
            }
        }

        while self.pc < self.memory.len() {
            let instruction = self.fetch();
            self.decode_and_execute(instruction);

            if instruction == OpCode::HLT {
                break;
            }
        }
    }
}