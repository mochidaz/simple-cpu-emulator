use crate::cpu::OpCode;

pub enum MemoryCell {
    Data(u8),
    Instruction(OpCode),
}

pub struct Memory {
    memory: Vec<MemoryCell>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: Vec::with_capacity(2048)
        }
    }

    pub fn load(&mut self, data: Vec<u8>) {
        for byte in data {
            self.memory.push(MemoryCell::Data(byte));
        }
    }

    pub fn fetch(&mut self, pc: usize) -> OpCode {
        match self.memory[pc] {
            MemoryCell::Instruction(instruction) => instruction,
            _ => panic!("Invalid instruction"),
        }
    }

    pub fn push(&mut self, instruction: OpCode) {
        self.memory.push(MemoryCell::Instruction(instruction));
    }

    pub fn pop(&mut self) -> OpCode {
        match self.memory.pop() {
            Some(MemoryCell::Instruction(instruction)) => instruction,
            _ => panic!("Invalid instruction"),
        }
    }

    pub fn read(&mut self, address: usize) -> u8 {
        match self.memory[address] {
            MemoryCell::Data(data) => data,
            _ => panic!("Invalid data"),
        }
    }

    pub fn write(&mut self, address: usize, data: u8) {
        self.memory[address] = MemoryCell::Data(data);
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }
}