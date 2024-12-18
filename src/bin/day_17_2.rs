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
type FragmentLibrary = [Vec<RegNum>; 8];

// The specified program can be expressed purely in terms of register A's value:
//  1. Output (last 3 binary digits of A) XOR (last 3 binary digits of (A with (A mod 8 XOR 1) digits removed)))
//  2. Remove the last 3 digits from A
//  3. If A == 0 halt, else return to 1.
// Note: (A mod 8 XOR 1) can be expressed by taking the last 3 digits of A, then swapping the values 0/1, 2/3, 4/5 and 6/7.
//
// This can be expressed in Rust as:
// fn iter(a: RegNum) -> (RegNum, OutNum) {
//     let rsh = (a & 0b111) as u8 ^ 0b001;
//     let out = (((a + 4) & 0b111) as u8) ^ (((a >> rsh) & 0b111) as u8);
//     let new_a = a / 8;
//     (new_a, out)
// }
//
// This means that each iteration outputs a number based only on (at most) the last 10 binary digits of A,
// then reduces A by 3 digits.
// Therefore, each "window" of 10 digits produces a unique, defined output digit.
// The problem can then be reformulated as finding the correct "fragments" to assemble into the original value A.
// As the program contains 16 numbers, A must have 16*3 "possibly significant" binary digits
// (plus 7 leading zeros to complete the last "window"),
// corresponding to 16 assembled "fragments".
//
// When a "library" of fragments is first constructed, it contains around 200 possibilities at each position,
// meaning 200^16 total combinations.
// This can first be reduced by basic filtering to see which fragments are viable at all based on the previous/next position,
// before finally enumerating all possibilities explicitly once the library's size is more reasonable.

fn get_fragment_output(f: RegNum) -> OutNum {
    let rsh = (f & 0b111) as u8 ^ 0b001;
    (((f + 4) & 0b111) as u8) ^ (((f >> rsh) & 0b111) as u8)
}

fn build_fragment_library() -> FragmentLibrary {
    let mut library: FragmentLibrary = (0..8).map(|_| Vec::new()).collect::<Vec<Vec<RegNum>>>().try_into().unwrap();
    (0..=0b11_1111_1111).for_each(|f| library[get_fragment_output(f) as usize].push(f.clone()));
    library
}

fn do_fragments_overlap(l: RegNum, r: RegNum) -> bool {
    // true if first 7 digits of l overlap with last 7 digits of r
    (l & 0b111_1111) == (r >> 3)
}

fn do_all_fragments_overlap(fragments: [RegNum; PROG_LEN]) -> bool {
    (1..fragments.len()).all(|i| do_fragments_overlap(fragments[i - 1], fragments[i]))
}

fn combine_overlapping_fragments(fragments: [RegNum; PROG_LEN]) -> RegNum {
    if ! do_all_fragments_overlap(fragments) { panic!("fragments do not overlap"); }
    fragments.iter().cloned().reduce(|acc, f| (acc << 3) + (f & 0b111)).expect("unable to combine fragments").clone()
}

fn assemble_fragments(library: &FragmentLibrary, program: &Program) -> RegNum {
    // (note .rev()! A is deconstructed from the right, therefore the fragments need to be reversed w.r.t. the program)
    let mut possible_fragments_per_position: Vec<Vec<RegNum>> = program.iter().map(|&n| library[n as usize].clone()).rev().collect();

    let num_filter_steps = 5;  // 5 iterations here brings the total number of combinations down to ~3e8

    for t in 1..=num_filter_steps {
        // "Basic filtering": at each position, find all fragments which overlap with *any* fragment at the next/previous position
        // Each iteration reduces the number of possible fragments further without explicitly needing to compare distant positions

        for ii in 1..PROG_LEN {
            let i = PROG_LEN - ii;
            possible_fragments_per_position[i] = possible_fragments_per_position[i].iter().filter(|&f| {
                possible_fragments_per_position[i - 1].iter().any(|&f2| do_fragments_overlap(f2, *f))
                    && (i > 4 || (*f >> 3 * (i + 1)) == 0)
            }).cloned().collect()
        }

        for i in 0..(PROG_LEN - 1) {
            possible_fragments_per_position[i] = possible_fragments_per_position[i].iter().filter(|&f| {
                possible_fragments_per_position[i + 1].iter().any(|&f2| do_fragments_overlap(*f, f2))
            }).cloned().collect()
        }

        println!("Possible fragments per position after {t} filtering steps: {:?}", possible_fragments_per_position.iter().map(|v| v.len()).collect::<Vec<usize>>());
        println!("Total nr. of possibilities: {:.3e}", possible_fragments_per_position.iter().map(|v| v.len() as u128).product::<u128>());
    }

    let mut fragments: [RegNum; PROG_LEN];
    let mut frag_idxs: [usize; PROG_LEN] = [0; PROG_LEN];
    let max_frag_idxs: [usize; PROG_LEN] = possible_fragments_per_position.iter().map(|v| v.len()).collect::<Vec<usize>>().try_into().unwrap();
    let mut t: u128 = 0;
    loop {
        t += 1;
        if t % 10_000_000 == 0 { println!("starting {t:e}th iteration"); }

        // get fragments by index
        fragments = frag_idxs.iter().enumerate().map(|(i, &idx)| possible_fragments_per_position[i][idx]).collect::<Vec<RegNum>>().try_into().unwrap();

        // check for overlap
        if do_all_fragments_overlap(fragments) { break; }

        // increment indices
        let mut i = frag_idxs.len() - 1;
        frag_idxs[i] += 1;
        while frag_idxs[i] >= max_frag_idxs[i] {
            frag_idxs[i] = 0;
            i -= 1;
            frag_idxs[i] += 1;
        }
    }
    combine_overlapping_fragments(fragments)
}

fn calculate_result(_lines: Lines<BufReader<File>>) -> Result<RegNum, ()> {
    let library = build_fragment_library();
    println!("Fragment library constructed: {:?} items", library.iter().map(|l| l.len()).collect::<Vec<usize>>());
    let a = assemble_fragments(&library, &PROGRAM);
    Ok(a)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
