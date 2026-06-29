use std::fs::File; // Import file handler
use std::io::Read; // Import read operations

// Main emulator structure
struct Chip8 {
    memory: [u8; 4096],      // 4096 bytes of RAM
    registers: [u8; 16],     // 16 8-bit registers (V0 to VF)
    pc: u16,                 // Program Counter (tracks current instruction address)
    screen: [bool; 64 * 32], // Monochrome screen (64x32 pixels)
}

impl Chip8 {
    // Constructor to initialize the system
    fn new() -> Self {
        Self {
            memory: [0; 4096],       // Clear memory
            registers: [0; 16],      // Clear registers
            pc: 0x200,               // PC starts at 0x200 (standard for CHIP-8)
            screen: [false; 64 * 32], // Clear screen (all pixels off)
        }
    }

    // Read a game file and load it into RAM
    fn load_rom(&mut self, filename: &str) {
        // Open the ROM file, crash if not found
        let mut file = File::open(filename).expect("Failed to open ROM file");
        
        // Dynamic buffer to hold file bytes
        let mut buffer = Vec::new();
        
        // Read file contents to buffer, crash if it fails
        file.read_to_end(&mut buffer).expect("Failed to read ROM data");

        // Copy buffer bytes into memory starting at 0x200
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
        // Print success message with size
        println!("ROM loaded successfully! Size: {} bytes", buffer.len());
    }

    // Run one CPU cycle (Fetch + Execute)
    fn run_cycle(&mut self) {
        let opcode = self.fetch(); // 1. Fetch the instruction
        self.execute(opcode);      // 2. Execute the instruction
    }

    // Fetch 16-bit opcode from 8-bit memory array
    fn fetch(&mut self) -> u16 {
        let byte1 = self.memory[self.pc as usize];       // Get first byte
        let byte2 = self.memory[(self.pc + 1) as usize];   // Get second byte
        
        // Combine bytes into a single u16 instruction
        let opcode = ((byte1 as u16) << 8) | (byte2 as u16);
        self.pc += 2; // Advance PC by 2 bytes
        
        opcode // Return the combined instruction
    }

    // Decode and execute opcode
    fn execute(&mut self, opcode: u16) {
        // Extract opcode components using bitmasks and shifts
        let c = ((opcode & 0xF000) >> 12) as u8; // First nibble (Operation)
        let x = ((opcode & 0x0F00) >> 8) as u8;  // Second nibble (Register X)
        let _y = ((opcode & 0x00F0) >> 4) as u8; // Third nibble (Register Y)
        let _n = (opcode & 0x000F) as u8;        // Fourth nibble (4-bit constant)
        let nnn = opcode & 0x0FFF;               // Last 12 bits (Address)
        let kk = (opcode & 0x00FF) as u8;        // Last 8 bits (8-bit constant)

        // Match components to find the instruction
        match (c, x, _y, _n) {
            // 0x00E0: Clear screen instruction
            (0, 0, 0xE, 0) => {
                self.screen = [false; 64 * 32]; // Turn off all pixels
                println!("Screen cleared!");
            }
            // 1NNN: Jump instruction
            (1, _, _, _) => {
                // Security: If a JUMP points to its own address, it's an infinite blocking loop.
                // We stop the emulator safely instead of looping forever in the terminal.
                if self.pc == nnn + 2 { 
                    println!("Infinite loop detected at address {:#X}. Stopping emulation.", nnn);
                    std::process::exit(0);
                }
                self.pc = nnn;
                println!("Jumped to address: {:#X}", nnn);
            }
            // 3XKK: Skip next instruction if VX == KK
            (3, _, _, _) => {
                if self.registers[x as usize] == kk {
                    self.pc += 2;
                    println!("Skipped next instruction because V{:X} == {:#X}", x, kk);
                }
            }
            // 4XKK: Skip next instruction if VX != KK
            (4, _, _, _) => {
                if self.registers[x as usize] != kk {
                    self.pc += 2;
                    println!("Skipped next instruction because V{:X} != {:#X}", x, kk);
                }
            }
            // 5XY0: Skip next instruction if VX == VY
            (5, _, _y, 0) => {
                if self.registers[x as usize] == self.registers[_y as usize] {
                    self.pc += 2;
                    println!("Skipped next instruction because V{:X} ({:#X}) == V{:X} ({:#X})", 
                        x, self.registers[x as usize], _y, self.registers[_y as usize]);
                }
            }
            // 6XKK: Set register VX to KK
            (6, _, _, _) => {
                self.registers[x as usize] = kk;
                println!("Set register V{:X} to {:#X}", x, kk);
            }
            // 7XKK: Add KK to register VX
            (7, _, _, _) => {
                self.registers[x as usize] += kk;
                println!("Added {:#X} to register V{:X}", kk, x);
            }
            // Unknown instruction fallback
            _ => {
                println!("Unknown opcode: {:#X} - Stopping emulation.", opcode);
                std::process::exit(0); // Exit program safely
            }
        }
    }
}

fn main() {
    println!("Starting Oxichip Emulator...");
    
    let mut console = Chip8::new(); // Create virtual console
    
    // Setup test program in memory
    // Address 0x200: 0x6005 (Set V0 to 0x05)
    console.memory[0x200] = 0x60;
    console.memory[0x201] = 0x05;

    // Address 0x202: 0x6105 (Set V1 to 0x05)
    console.memory[0x202] = 0x61;
    console.memory[0x203] = 0x05;

    // Address 0x204: 0x5010 (Skip next instruction if V0 == V1) -> SHOULD SKIP!
    console.memory[0x204] = 0x50;
    console.memory[0x205] = 0x10;

    // Address 0x206: 0x62AA (Set V2 to 0xAA) -> THIS SHOULD BE SKIPPED
    console.memory[0x206] = 0x62;
    console.memory[0x207] = 0xAA;

    // Address 0x208: 0x1208 (Infinite loop to stop here)
    console.memory[0x208] = 0x12;
    console.memory[0x209] = 0x08;

    // Infinite execution loop
    loop {
        console.run_cycle(); // Run one cycle
        
        // Sleep for 500ms to slow down execution for debugging
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}