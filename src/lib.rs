mod errors;
mod instructions;

use std::convert::TryInto;
use crate::instructions::{Instruction, Instruction::*};
use crate::errors::{CPUError, CPUError::*};


#[cfg(test)]
mod tests {
    use std::time::Instant;

    #[test]
    fn it_works() {
    }
}

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Cpu {
    a: u64,
    b: u64,
    s: u64,
    x: u64,
    cache: [u64; 65535],
    instructions: [u8; 65535],
    memory: Vec<u64>,
    devices: Vec<Box<dyn Device>>
}

pub trait Device {
    fn get_address_space(&self) -> (u64, u64);
    fn load(&self, address: u64) -> u64;
    fn push(&self, address: u64, value:u64);
}

impl Cpu {
    pub fn new(instructions: [u8; 65535], devices: Vec<Box<dyn Device>>) -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            s: 0,
            x: 0,
            cache: [0; 65535],
            instructions,
            memory: vec![],
            devices
        }
    }

    pub fn debug(&self) -> &Cpu {
        self
    }

    pub fn tick(&mut self) -> Result<(), CPUError>{
        self.x += 1;
        self.process_instruction(self.read_instruction()?)
    }
    fn read_instruction(&self) -> Result<Instruction, CPUError>{
        match self.instructions.get((self.x - 1 )as usize) {
            None => Err(OutOfInstructions(format!("Out of instructions at position {}", self.x))),
            Some(i) => {
                match i {
                    0 => Ok(NoOp),
                    1 => Ok(LoadBusA(self.get_args(self.x)?)),
                    2 => Ok(LoadBusB(self.get_args(self.x)?)),
                    3 => Ok(Add),
                    4 => Ok(Subtract),
                    5 => Ok(Multiply),
                    6 => Ok(Divide),
                    7 => Ok(CopyAB),
                    8 => Ok(CopyBA),
                    9 => Ok(SwapAB),
                    10 => Ok(PushABus(self.get_args(self.x)?)),
                    11 => Ok(PushBBus(self.get_args(self.x)?)),
                    12 => Ok(LoadA(self.get_args(self.x)?)),
                    13 => Ok(LoadBusX(self.get_args(self.x)?)),
                    14 => Ok(CopyAX),
                    15 => Ok(CopyBX),
                    16 => Ok(PushXBus(self.get_args(self.x)?)),
                    17 => Ok(LoadX(self.get_args(self.x)?)),
                    18 => Ok(CopyXA),
                    19 => Ok(CopyXB),
                    20 => Ok(LoadBusAS),
                    21 => Ok(LoadBusBS),
                    22 => Ok(CopyAS),
                    23 => Ok(CopyBS),
                    24 => Ok(CopyXS),
                    25 => Ok(CopySA),
                    26 => Ok(CopySB),
                    27 => Ok(CopySX),
                    28 => Ok(SwapAS),
                    29 => Ok(SwapBS),
                    30 => Ok(PushABusS),
                    31 => Ok(PushBBusS),
                    32 => Ok(LoadBusXS),
                    33 => Ok(PushXBusS),
                    34 => Ok(SkipEq),
                    35 => Ok(SkipGrEq),
                    36 => Ok(SkipGr),
                    37 => Ok(SkipLe),
                    38 => Ok(SkipLeEq),
                    e => Err(IllegalInstruction(format!("{} is not a valid instruction", e)))
                }
            }
        }
    }
    fn get_args(&self, start: u64) -> Result<u64, CPUError> {
        Ok(u64::from_be_bytes(self.instructions[(start) as usize..(start+8) as usize].try_into().unwrap()))
    }
    fn process_instruction(&self, inst: Instruction) -> Result<(), CPUError> {
        Ok(())
    }
}
