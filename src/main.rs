struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    pc: u16,
    screen: [bool; 64 * 32], // Screen layout: 64x32 pixels (true = on, false = off)
}

impl Chip8 {
    // Create and initialize a new Chip8 instance
    fn new() -> Self {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            pc: 0x200,
            screen: [true; 64 * 32], // Start with all pixels ON to test the clear function
        }
    }

    // Read an instruction from memory
    fn fetch(&mut self) -> u16 {
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[(self.pc + 1) as usize];
        
        let opcode = ((byte1 as u16) << 8) | (byte2 as u16);
        self.pc += 2;
        
        opcode
    }

    // Decode and execute a single instruction
    fn execute(&mut self, opcode: u16) {
        match opcode {
            0x00E0 => {
                // Clear the screen by setting all pixels to false
                self.screen = [false; 64 * 32];
                println!("Screen cleared!");
            }
            _ => {
                // Default case if the instruction is not implemented yet
                println!("Unknown opcode: {:#X}", opcode);
            }
        }
    }
}

fn main() {
    println!("Initializing Oxichip...");
    
    let mut console = Chip8::new();
    
    // Check the state of the first pixel before clearing
    println!("Pixel 0 status before: {}", console.screen[0]);

    // Inject the "Clear Screen" opcode (0x00E0) into memory
    console.memory[0x200] = 0x00;
    console.memory[0x201] = 0xE0;

    // Emulate one single CPU cycle
    let op = console.fetch();
    console.execute(op);

    // Check the state of the first pixel after clearing
    println!("Pixel 0 status after: {}", console.screen[0]);
}