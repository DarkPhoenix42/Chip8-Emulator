use crate::emulator::consts;
use rand;
use std::{error, fmt};

use super::consts::FONTSET;

#[derive(Debug)]
pub enum ProcessorError {
    IoError(std::io::Error),
    InvalidRom,
}

impl fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessorError::IoError(err) => write!(f, "{}", err),
            _ => write!(f, "Hello World"),
        }
    }
}

impl error::Error for ProcessorError {}

#[derive(Debug)]
pub struct Processor {
    registers: [u8; consts::N_REGISTERS],
    memory: [u8; consts::MEMORY_SIZE],
    pub keys_pressed: [bool; consts::N_KEYS],
    index: u16,
    pc: u16,
    stack: [u16; consts::STACK_SIZE],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub display: [u8; (consts::DISPLAY_WIDTH * consts::DISPLAY_HEIGHT) as usize],
}

#[derive(Debug)]
pub enum Instruction {
    CLS,
    RET,
    NOP,
    JUMP(u16),
    CALL(u16),
    SE(u8, u8),
    SER(u8, u8),
    SNE(u8, u8),
    SNER(u8, u8),
    LD(u8, u8),
    ADD(u8, u8),
    LDR(u8, u8),
    OR(u8, u8),
    AND(u8, u8),
    XOR(u8, u8),
    ADDR(u8, u8),
    SUBR(u8, u8),
    SHR(u8),
    SUBN(u8, u8),
    SHL(u8),
    LDA(u16),
    JUMPV0(u16),
    RND(u8, u8),
    DRW(u8, u8, u8),
    SKP(u8),
    SKNP(u8),

    LDT(u8),
    WKEY(u8),
    SDT(u8),
    SST(u8),
    ADDI(u8),

    LDSPR(u8),
    STBCD(u8),

    STREG(u8),
    LDREG(u8),
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: [0; consts::N_REGISTERS],
            memory: [0; consts::MEMORY_SIZE],
            keys_pressed: [false; consts::N_KEYS],
            index: 0,
            pc: consts::ROM_START as u16,
            stack: [0; consts::STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [0; (consts::DISPLAY_WIDTH * consts::DISPLAY_HEIGHT) as usize],
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), ProcessorError> {
        let rom = std::fs::read(path).map_err(ProcessorError::IoError)?;

        if rom.len() > consts::MEMORY_SIZE - consts::ROM_START {
            return Err(ProcessorError::InvalidRom);
        }

        self.memory[consts::ROM_START..consts::ROM_START + rom.len()].copy_from_slice(&rom);
        self.memory[consts::FONTSET_START..consts::FONTSET_END].copy_from_slice(&FONTSET);

        Ok(())
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        let first_byte: u16 = self.memory[self.pc as usize].into();
        let second_byte: u16 = self.memory[(self.pc + 1) as usize].into();
        self.pc += 2;

        let opcode = (first_byte << 8) | second_byte;

        opcode
    }

    pub fn decode_instruction(opcode: u16) -> Instruction {
        let instruction = match (opcode >> 12) & 0xf {
            0x0 => match opcode & 0xff {
                0xe0 => Instruction::CLS,
                0xee => Instruction::RET,
                _ => Instruction::NOP,
            },

            0x1 => Instruction::JUMP(opcode & 0x0fff),
            0x2 => Instruction::CALL(opcode & 0x0fff),
            0x3 => Instruction::SE(((opcode >> 8) & 0xf) as u8, (opcode & 0x00ff) as u8),
            0x4 => Instruction::SNE(((opcode >> 8) & 0xf) as u8, (opcode & 0x00ff) as u8),
            0x5 => Instruction::SER(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
            0x6 => Instruction::LD(((opcode >> 8) & 0xf) as u8, (opcode & 0x00ff) as u8),
            0x7 => Instruction::ADD(((opcode >> 8) & 0xf) as u8, (opcode & 0xff) as u8),

            0x8 => match opcode & 0xf {
                0x0 => Instruction::LDR(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
                0x1 => Instruction::OR(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
                0x2 => Instruction::AND(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
                0x3 => Instruction::XOR(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
                0x4 => Instruction::ADDR(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
                0x5 => Instruction::SUBR(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
                0x6 => Instruction::SHR(((opcode >> 8) & 0xf) as u8),
                0x7 => Instruction::SUBN(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
                0xE => Instruction::SHL(((opcode >> 8) & 0xf) as u8),
                _ => Instruction::NOP,
            },

            0x9 => Instruction::SNER(((opcode >> 8) & 0xf) as u8, ((opcode >> 4) & 0xf) as u8),
            0xA => Instruction::LDA(opcode & 0x0fff),
            0xB => Instruction::JUMPV0(opcode & 0x0fff),
            0xC => Instruction::RND(((opcode >> 8) & 0xf) as u8, (opcode & 0x00ff) as u8),

            0xD => Instruction::DRW(
                ((opcode >> 8) & 0xf) as u8,
                ((opcode >> 4) & 0xf) as u8,
                (opcode & 0xf) as u8,
            ),

            0xE => match opcode & 0xff {
                0x9E => Instruction::SKP(((opcode >> 8) & 0xf) as u8),
                0xA1 => Instruction::SKNP(((opcode >> 8) & 0xf) as u8),
                _ => Instruction::NOP,
            },

            0xF => match opcode & 0xff {
                0x07 => Instruction::LDT(((opcode >> 8) & 0xf) as u8),
                0x0A => Instruction::WKEY(((opcode >> 8) & 0xf) as u8),
                0x15 => Instruction::SDT(((opcode >> 8) & 0xf) as u8),
                0x18 => Instruction::SST(((opcode >> 8) & 0xf) as u8),
                0x1E => Instruction::ADDI(((opcode >> 8) & 0xf) as u8),
                0x29 => Instruction::LDSPR(((opcode >> 8) & 0xf) as u8),
                0x33 => Instruction::STBCD(((opcode >> 8) & 0xf) as u8),
                0x55 => Instruction::STREG(((opcode >> 8) & 0xf) as u8),
                0x65 => Instruction::LDREG(((opcode >> 8) & 0xf) as u8),
                _ => Instruction::NOP,
            },

            _ => Instruction::NOP,
        };

        instruction
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::CLS => self.clear_display(),
            Instruction::RET => self.ret(),
            Instruction::NOP => {}
            Instruction::JUMP(addr) => self.jump(addr),
            Instruction::CALL(addr) => self.call(addr),
            Instruction::SE(reg, value) => {
                self.skip_next_instruction_if(self.registers[reg as usize] == value)
            }
            Instruction::SER(reg1, reg2) => self.skip_next_instruction_if(
                self.registers[reg1 as usize] == self.registers[reg2 as usize],
            ),
            Instruction::SNE(reg, value) => {
                self.skip_next_instruction_if(self.registers[reg as usize] != value)
            }
            Instruction::SNER(reg1, reg2) => self.skip_next_instruction_if(
                self.registers[reg1 as usize] != self.registers[reg2 as usize],
            ),
            Instruction::LD(reg, value) => self.load_val(reg, value),
            Instruction::ADD(reg, value) => self.add_val(reg, value),
            Instruction::LDR(reg1, reg2) => self.load_reg(reg1, reg2),
            Instruction::OR(reg1, reg2) => self.or(reg1, reg2),
            Instruction::AND(reg1, reg2) => self.and(reg1, reg2),
            Instruction::XOR(reg1, reg2) => self.xor(reg1, reg2),
            Instruction::ADDR(reg1, reg2) => self.add_registers(reg1, reg2),
            Instruction::SUBR(reg1, reg2) => self.sub_reg(reg1, reg2),
            Instruction::SHR(reg) => self.shr(reg),
            Instruction::SUBN(reg1, reg2) => self.subn_reg(reg1, reg2),
            Instruction::SHL(reg) => self.shl(reg),
            Instruction::LDA(addr) => self.load_addr(addr),
            Instruction::JUMPV0(addr) => self.jump_addr(addr),
            Instruction::RND(reg, value) => self.rand(reg, value),
            Instruction::DRW(reg1, reg2, n) => self.display(reg1, reg2, n),
            Instruction::SKP(reg) => self.skp(reg),
            Instruction::SKNP(reg) => self.sknp(reg),
            Instruction::LDT(reg) => self.load_delay_timer(reg),
            Instruction::WKEY(reg) => self.wait_key(reg),
            Instruction::SDT(reg) => self.set_delay_timer_reg(reg),
            Instruction::SST(reg) => self.set_sound_timer_reg(reg),
            Instruction::ADDI(reg) => self.add_index(reg),
            Instruction::LDSPR(reg) => self.load_sprite(reg),
            Instruction::STBCD(reg) => self.store_bcd(reg),
            Instruction::STREG(reg) => self.save_registers(reg),
            Instruction::LDREG(reg) => self.load_registers(reg),
        }
    }

    pub fn emulate_cycle(&mut self) -> Result<(), ProcessorError> {
        let opcode = self.fetch_instruction();
        let instruction = Processor::decode_instruction(opcode);
        self.execute_instruction(instruction);

        Ok(())
    }

    fn clear_display(&mut self) {
        self.display.fill(0);
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn skip_next_instruction_if(&mut self, condition: bool) {
        if condition {
            self.pc += 2;
        }
    }

    fn load_val(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = value;
    }

    fn add_val(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(value);
    }

    fn load_reg(&mut self, reg1: u8, reg2: u8) {
        self.registers[reg1 as usize] = self.registers[reg2 as usize];
    }

    fn or(&mut self, reg1: u8, reg2: u8) {
        self.registers[reg1 as usize] |= self.registers[reg2 as usize];
    }

    fn and(&mut self, reg1: u8, reg2: u8) {
        self.registers[reg1 as usize] &= self.registers[reg2 as usize];
    }

    fn xor(&mut self, reg1: u8, reg2: u8) {
        self.registers[reg1 as usize] ^= self.registers[reg2 as usize];
    }

    fn add_registers(&mut self, reg1: u8, reg2: u8) {
        let (result, overflow) =
            self.registers[reg1 as usize].overflowing_add(self.registers[reg2 as usize]);
        self.registers[reg1 as usize] = result;
        self.registers[consts::N_REGISTERS - 1] = if overflow { 1 } else { 0 };
    }

    fn sub_reg(&mut self, reg1: u8, reg2: u8) {
        if self.registers[reg1 as usize] > self.registers[reg2 as usize] {
            self.registers[consts::N_REGISTERS - 1] = 1;
        } else {
            self.registers[consts::N_REGISTERS - 1] = 0;
        }

        self.registers[reg1 as usize] =
            self.registers[reg1 as usize].wrapping_sub(self.registers[reg2 as usize]);
    }

    fn shr(&mut self, reg: u8) {
        self.registers[consts::N_REGISTERS - 1] = self.registers[reg as usize] & 0x1;
        self.registers[reg as usize] >>= 1;
    }

    fn subn_reg(&mut self, reg1: u8, reg2: u8) {
        if self.registers[reg1 as usize] > self.registers[reg2 as usize] {
            self.registers[consts::N_REGISTERS - 1] = 0;
        } else {
            self.registers[consts::N_REGISTERS - 1] = 1;
        }

        self.registers[reg1 as usize] =
            self.registers[reg2 as usize].wrapping_sub(self.registers[reg1 as usize]);
    }

    fn shl(&mut self, reg: u8) {
        self.registers[consts::N_REGISTERS - 1] = self.registers[reg as usize] >> 7;
        self.registers[reg as usize] = self.registers[reg as usize].wrapping_shl(1);
    }

    fn load_addr(&mut self, addr: u16) {
        self.index = addr;
    }

    fn jump_addr(&mut self, addr: u16) {
        self.pc = addr + self.registers[0] as u16;
    }

    fn display(&mut self, reg1: u8, reg2: u8, n: u8) {
        let x = (self.registers[reg1 as usize] as usize) % consts::DISPLAY_WIDTH;
        let y = (self.registers[reg2 as usize] as usize) % consts::DISPLAY_HEIGHT;
        self.registers[consts::N_REGISTERS - 1] = 0;

        for i in 0..n {
            let sprite_byte = self.memory[self.index as usize + i as usize];

            for j in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - j)) & 0x1;

                let screen_pixel =
                    &mut self.display[(y + i as usize) * consts::DISPLAY_WIDTH + (x + j as usize)];

                if sprite_pixel > 0 {
                    if screen_pixel > &mut 0 {
                        self.registers[consts::N_REGISTERS - 1] = 1;
                    }

                    *screen_pixel ^= 1;
                }
            }
        }
    }

    fn rand(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = rand::random::<u8>() & value;
    }

    fn skp(&mut self, reg: u8) {
        self.skip_next_instruction_if(self.keys_pressed[self.registers[reg as usize] as usize]);
    }

    fn sknp(&mut self, reg: u8) {
        self.skip_next_instruction_if(!self.keys_pressed[self.registers[reg as usize] as usize]);
    }

    fn load_delay_timer(&mut self, reg: u8) {
        self.registers[reg as usize] = self.delay_timer;
    }

    fn wait_key(&mut self, reg: u8) {
        for (i, key) in self.keys_pressed.iter().enumerate() {
            if *key {
                self.registers[reg as usize] = i as u8;
                return;
            }
        }

        self.pc -= 2;
    }

    fn set_delay_timer_reg(&mut self, reg: u8) {
        self.delay_timer = self.registers[reg as usize];
    }

    fn set_sound_timer_reg(&mut self, reg: u8) {
        self.sound_timer = self.registers[reg as usize];
    }

    fn add_index(&mut self, reg: u8) {
        self.index += self.registers[reg as usize] as u16;
    }

    fn load_sprite(&mut self, reg: u8) {
        self.index = consts::FONTSET_START as u16 + (self.registers[reg as usize] as u16) * 5;
    }

    fn store_bcd(&mut self, reg: u8) {
        self.memory[self.index as usize] = self.registers[reg as usize] / 100;
        self.memory[self.index as usize + 1] = (self.registers[reg as usize] / 10) % 10;
        self.memory[self.index as usize + 2] = self.registers[reg as usize] % 10;
    }

    fn save_registers(&mut self, reg: u8) {
        for i in 0..reg + 1 {
            self.memory[self.index as usize + i as usize] = self.registers[i as usize];
        }
    }

    fn load_registers(&mut self, reg: u8) {
        for i in 0..reg + 1 {
            self.registers[i as usize] = self.memory[self.index as usize + i as usize];
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_processor_test() {
        Processor::new();
    }
}
