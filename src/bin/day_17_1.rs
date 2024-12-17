use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::join;

const OUTPUT_MESSAGE: &str = "Output";
const INPUT_FILE: &str = "input/day_17.txt";

type ProgNum = u8;
type RegNum = u64;
type OutNum = RegNum;
const PROG_LEN: usize = 16;
type Program = [ProgNum; PROG_LEN];

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
        self.a /= b.pow(self.combo_operand(operand).try_into().expect("invalid exponent"));
        None
    }

    fn bxl(&mut self, operand: ProgNum) -> Option<OutNum> {
        let o: RegNum = self.literal_operand(operand).try_into().unwrap();
        self.b ^= o;
        None
    }

    fn bst(&mut self, operand: ProgNum) -> Option<OutNum> {
        self.b = self.combo_operand(operand) % 8;
        None
    }

    fn jnz(&mut self, operand: ProgNum) -> Option<OutNum> {
        if self.a != 0 {
            println!("Jumping to {operand} - A register contains {}", self.a);
            self.inst_ptr = self.literal_operand(operand).try_into().unwrap();
        } else {
            println!("A register contains 0 - not jumping");
            self.inst_ptr += 2;
        }
        None
    }

    fn bxc(&mut self, _operand: ProgNum) -> Option<OutNum> {
        self.b ^= self.c;
        None
    }

    fn out(&mut self, operand: ProgNum) -> Option<OutNum> {
        Some(self.combo_operand(operand) % 8)
    }

    fn bdv(&mut self, operand: ProgNum) -> Option<OutNum> {
        let b: RegNum = 2;
        self.b = self.a / b.pow(self.combo_operand(operand).try_into().expect("invalid exponent"));
        None
    }

    fn cdv(&mut self, operand: ProgNum) -> Option<OutNum> {
        let b: RegNum = 2;
        self.c = self.a / b.pow(self.combo_operand(operand).try_into().expect("invalid exponent"));
        None
    }

    fn execute_instruction(&mut self, opcode: ProgNum, operand: ProgNum) -> Option<OutNum> {
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("invalid opcode"),
        }
    }

    fn run_program(&mut self, program: Program) -> Vec<OutNum> {
        println!("Running program: {program:?}");
        println!("on computer: {self:?}");
        let mut output = Vec::new();
        while self.inst_ptr < program.len() {
            let (opcode, operand) = (program[self.inst_ptr], program[self.inst_ptr + 1]);
            if let Some(o) = self.execute_instruction(opcode, operand) {
                output.push(o);
            }
            if opcode != 3 { self.inst_ptr += 2; }
        }
        output
    }
}

fn parse_input(lines: Lines<BufReader<File>>) -> (Computer, Program) {
    let lines_vec: Vec<String> = lines.map(|l| l.expect("unable to read line")).collect();
    let a = str::parse::<RegNum>(&lines_vec[0][12..]).expect("unable to parse number").try_into().expect("invalid register value");
    let b = str::parse::<RegNum>(&lines_vec[1][12..]).expect("unable to parse number").try_into().expect("invalid register value");
    let c = str::parse::<RegNum>(&lines_vec[2][12..]).expect("unable to parse number").try_into().expect("invalid register value");
    let prog = lines_vec[4][9..].split(",").map(|s| str::parse::<ProgNum>(s).expect("invalid program number")).collect::<Vec<ProgNum>>().try_into().expect("incorrect program length");

    (
        Computer { a, b, c, inst_ptr: 0 },
        prog
    )
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<String, ()> {
    let (mut computer, program) = parse_input(lines);
    let out_vec = computer.run_program(program);
    Ok(join(out_vec, ","))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
