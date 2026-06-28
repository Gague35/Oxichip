use std::fs::File; // Import file system tools
use std::io::Read; // Import input/output traits for reading files

struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    pc: u16,
    screen: [bool; 64 * 32],
}

impl Chip8 {
    fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            pc: 0x200,
            screen: [false; 64 * 32],
        }
    }

    // Load a ROM file into the emulator's memory
    fn load_rom(&mut self, filename: &str) {
        // Open the file
        let mut file = File::open(filename).expect("Failed to open ROM file");
        
        // Create a temporary buffer (a dynamic list of bytes)
        let mut buffer = Vec::new();
        
        // Read the entire file content into the buffer
        file.read_to_end(&mut buffer).expect("Failed to read ROM data");

        // Copy the buffer bytes into the CHIP-8 memory, starting at address 0x200
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
        
        println!("ROM loaded successfully! Size: {} bytes", buffer.len());
    }

    fn fetch(&mut self) -> u16 {
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[(self.pc + 1) as usize];
        
        let opcode = ((byte1 as u16) << 8) | (byte2 as u16);
        self.pc += 2;
        
        opcode
    }

    fn execute(&mut self, opcode: u16) {
        match opcode {
            0x00E0 => {
                self.screen = [false; 64 * 32];
                println!("Screen cleared!");
            }
            _ => {
                println!("Unknown opcode: {:#X}", opcode);
            }
        }
    }
}

fn main() {
    println!("Initializing Oxichip...");
    
    let mut console = Chip8::new();
    
    // We will try to load a ROM (we'll create a fake one just below to test)
    console.load_rom("test_games/ibm.ch8");
}