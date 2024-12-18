use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Output";
const INPUT_FILE: &str = "input/day_17.txt";

type ProgNum = u8;
type RegNum = u128;
type OutNum = ProgNum;
const PROG_LEN: usize = 16;
type Program = [ProgNum; PROG_LEN];
const PROGRAM: Program = [2,4,1,1,7,5,1,5,4,0,5,5,0,3,3,0];
const EXP_BASE: RegNum = 2;

#[derive(Debug)]
struct Computer {
    a: RegNum,
    b: RegNum,
    c: RegNum,
    inst_ptr: usize,
}

impl Computer {
    fn literal_operand(&self, operand: ProgNum) -> ProgNum {
        operand
    }

    fn combo_operand(&self, o: ProgNum) -> RegNum {
        match o {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo operand"),
        }
    }

    fn adv(&mut self, operand: ProgNum) -> Option<OutNum> {
        let b: RegNum = 2;
        self.a /= b.pow(self.combo_operand(operand) as u32);
        None
    }

    fn bxl(&mut self, operand: ProgNum) -> Option<OutNum> {
        let o: RegNum = self.literal_operand(operand) as RegNum;
        self.b ^= o;
        None
    }

    fn bst(&mut self, operand: ProgNum) -> Option<OutNum> {
        self.b = self.combo_operand(operand) % 8;
        None
    }

    fn jnz(&mut self, operand: ProgNum) -> Option<OutNum> {
        self.inst_ptr = if self.a != 0 {
            self.literal_operand(operand) as usize
        } else {
            self.inst_ptr + 2
        };
        None
    }

    fn bxc(&mut self, _operand: ProgNum) -> Option<OutNum> {
        self.b ^= self.c;
        None
    }

    fn out(&mut self, operand: ProgNum) -> Option<OutNum> {
        Some((self.combo_operand(operand) % 8) as OutNum)
    }

    fn bdv(&mut self, operand: ProgNum) -> Option<OutNum> {
        let b: RegNum = 2;
        self.b = self.a / b.pow(self.combo_operand(operand) as u32);
        None
    }

    fn cdv(&mut self, operand: ProgNum) -> Option<OutNum> {
        let b: RegNum = 2;
        self.c = self.a / b.pow(self.combo_operand(operand) as u32);
        None
    }

    fn execute_hardcoded_instruction(&mut self) -> Option<OutNum> {
        let out = self.inst_ptr == 10;
        match self.inst_ptr {
            0 => { self.b = self.a % 8 },
            2 => { self.b ^= 1 },  // 1 = 0b...00001
            4 => { self.c = self.a / EXP_BASE.pow(self.b as u32) },
            6 => { self.b ^= 5 },  // 5 = 0b...00101
            8 => { self.b ^= self.c },
            10 => (),
            12 => { self.a /= 8 },
            14 => (),
            _ => panic!("invalid value for self.inst_ptr"),
        };
        self.inst_ptr = if self.inst_ptr == 14 && self.a != 0 { 0 } else { self.inst_ptr + 2 };
        if out { Some((self.b % 8) as u8) } else { None }
    }

    fn run_program(&mut self, program: Program) -> Vec<OutNum> {
        let mut output = Vec::new();
        while self.inst_ptr < program.len() {
            if let Some(o) = self.execute_hardcoded_instruction() {
                output.push(o);
            }
        }
        output
    }

    // Each run of program:
    // 1. Set B to A % 8
    // 2. Set B to B XOR 0b...00001
    // 3. Set C to A / 2^B
    // 4. Set B to B XOR 0b...00101
    // 5. Set B to B XOR C
    // 6. Output B % 8
    // 7. Set A to A // 8
    // 8. Jump to start if A != 0
    //
    // 1-2. Set B to (A % 8) XOR 0b...00001
    // 3. Set C to A / 2^B
    // 4-5. Set B to (B XOR 0b...00101) XOR C
    // 6. Output B % 8
    // 7. Set A to A // 8
    //
    // 1-3. Set (B, C) to ((A%8)x1, A/2^((A%8)x1))
    // 4-5. Set B to (B XOR 0b...00101) XOR C
    // 6. Output B % 8
    // 7. Set A to A // 8
    //
    // 1-5. Set (B, C) to ((((A%8)x1)x5)x(A/2^((A%8)x1)), A/2^((A%8)x1))
    // 6. Output B % 8
    // 7. Set A to A // 8

    fn test_program_hardcoded_backtracking(&mut self) -> bool {
        let mut i = 0;
        while self.a > 0 {
            if i >= PROGRAM.len() { return false; }
            let a_mod_8 = self.a % 8;
            let divisor = EXP_BASE.pow((a_mod_8 ^ 1) as u32);
            self.b = ((a_mod_8 ^ 1) ^ 5) ^ (self.a / divisor);
            if (self.b % 8) != PROGRAM[i] as u128 { return false; }
            self.a /= 8;
            i += 1;
        }
        i == PROG_LEN  // true if terminated correctly
    }
}

fn start_computer(a: RegNum) -> Computer {
    Computer { a, b: 0, c: 0, inst_ptr: 0 }
}

fn iter_a_only(a: RegNum) -> (RegNum, OutNum) {
    let rsh = (a & 0b111) as u8 ^ 0b001;
    let out = (((a + 4) & 0b111) as u8) ^ (((a >> rsh) & 0b111) as u8);
    let new_a = a / 8;
    (new_a, out)
}

fn test_program_a_only_backtracking(a: RegNum)  -> bool {
    // (validated successfully using part 1 input)
    let mut a = a.clone();
    let mut i = 0;
    while a > 0 {
        if i >= PROGRAM.len() { return false; }
        let (new_a, out) = iter_a_only(a);
        if out != PROGRAM[i] { return false; }
        a = new_a;
        i += 1;
    }
    i == PROG_LEN
}

fn calculate_result(_lines: Lines<BufReader<File>>) -> Result<RegNum, ()> {
    println!("Program from input produces {:?}", start_computer(64854237).run_program(PROGRAM));
    let mut a = 0;
    while ! test_program_a_only_backtracking(a) {
        if a % 100_000_000 == 0 { println!("Tested up to a = {a:e}"); }
        a += 1;
    }
    Ok(a)
    // No result for values up to 2e9!
    // Brute-force solution not viable -> see day_17_2.rs for working solution
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
