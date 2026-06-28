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
        let nnn = opcode & 0x0FFF;              // Last 12 bits (Address)
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
                self.pc = nnn; // Move PC to target address
                println!("Jumped to address: {:#X}", nnn);
            }
            // 6XKK: Set register VX instruction
            (6, _, _, _) => {
                self.registers[x as usize] = kk; // Store value in register
                println!("Set register V{:X} to {:#X}", x, kk);
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
    console.memory[0x200] = 0x60; // Opcode 0x60AB: Set V0 to 0xAB
    console.memory[0x201] = 0xAB;
    console.memory[0x202] = 0x12; // Opcode 0x1200: Jump to 0x200
    console.memory[0x203] = 0x00;

    // Infinite execution loop
    loop {
        console.run_cycle(); // Run one cycle
        
        // Sleep for 500ms to slow down execution for debugging
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}