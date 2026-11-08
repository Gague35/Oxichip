# Oxichip 🦀

> ⚠️ **Project Status: On Hold**  
> This project is currently on indefinite pause. Development may resume in the future.

Oxichip is an experimental **CHIP-8 emulator** written in Rust, built with a focus on low-level architecture understanding, bitwise operations, and memory management.

---

## 🛠️ Features Implemented

- [x] **Core Architecture (`Chip8` struct)**
  - 4096 bytes of RAM (`[u8; 4096]`)
  - 16 8-bit general-purpose registers (`V0` to `VF`)
  - Program Counter (`PC`) starting at `0x200`
  - Fetch-Decode-Execute instruction loop using pattern matching
- [x] **Validated Instructions (Opcodes)**
  - `1NNN` - Jump to address `NNN`
  - `3XKK` - Skip next instruction if `VX == KK`
  - `4XKK` - Skip next instruction if `VX != KK`
  - `5XY0` - Skip next instruction if `VX == VY`
  - `6XKK` - Set `VX = KK`
  - `7XKK` - Set `VX = VX + KK`
  - `8XY0` - Set `VX = VY`
  - `8XY1` - Set `VX = VX OR VY`
  - `8XY2` - Set `VX = VX AND VY`
  - `8XY3` - Set `VX = VX XOR VY`
  - `8XY4` - Set `VX = VX + VY` (with Carry flag in `VF`)
  - `8XY5` - Set `VX = VX - VY` (with NOT borrow flag in `VF`)

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (cargo, rustc)

### Run Emulation Tests
```bash
cargo run

```

---

## 📝 Roadmap for Future Development

When development resumes, the following steps remain to complete the emulator:

1. Complete remaining arithmetic/shift opcodes (`8XY6`, `8XY7`, `8XYE`).
2. Implement graphics rendering instruction (`DXYN`) using a GUI window library (e.g., `minifb` or `SDL2`).
3. Implement timers (`FX07`, `FX15`) and delay systems.
4. Implement keypad input (`EX9E`, `EXA1`, `FX0A`).
5. Add binary ROM loading mechanism from file.

---

# Oxichip Architecture & Technical Details

## Memory Map

| Address Range | Description |
|---|---|
| `0x000 - 0x1FF` | Reserved for CHIP-8 interpreter / font set |
| `0x200 - 0xFFF` | Program / RAM space (ROM instructions loaded starting at `0x200`) |

## Register Definitions

- **`registers[0..=14]` (`V0` to `VE`)**: General purpose registers.
- **`registers[15]` (`VF`)**: Status flag register.
  - Used as **Carry Flag** (`1` on overflow, `0` otherwise) for `8XY4`.
  - Used as **NOT Borrow Flag** (`1` if `VX >= VY`, `0` if `VX < VY`) for `8XY5`.

## Execution Cycle

1. **Fetch**: Read two consecutive bytes at `PC` and `PC + 1` from `memory`. Combine into a 16-bit opcode (`(byte1 as u16) << 8 | (byte2 as u16)`).
2. **Decode**: Extract nibbles `(op, x, y, sub)` using bit shifts and masks.
3. **Execute**: Match against instruction patterns, update state (`registers`, `memory`, `PC`).
