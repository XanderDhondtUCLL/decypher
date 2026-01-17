use std::char;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    use std::collections::HashMap;

    let alphabet: HashMap<char, i8> = HashMap::from([
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
        ('k', 10),
        ('l', 11),
        ('m', 12),
        ('n', 13),
        ('o', 14),
        ('p', 15),
        ('q', 16),
        ('r', 17),
        ('s', 18),
        ('t', 19),
        ('u', 20),
        ('v', 21),
        ('w', 22),
        ('x', 23),
        ('y', 24),
        ('z', 25),
    ]);

    let alphabet_upper: HashMap<char, i8> = HashMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('D', 3),
        ('E', 4),
        ('F', 5),
        ('G', 6),
        ('H', 7),
        ('I', 8),
        ('J', 9),
        ('K', 10),
        ('L', 11),
        ('M', 12),
        ('N', 13),
        ('O', 14),
        ('P', 15),
        ('Q', 16),
        ('R', 17),
        ('S', 18),
        ('T', 19),
        ('U', 20),
        ('V', 21),
        ('W', 22),
        ('X', 23),
        ('Y', 24),
        ('Z', 25),
    ]);

    // file IO
    let input_path: String = fs::read_to_string("data/input.txt")?;
    let output_path: &str = "data/output.txt";

    // CLI IO
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    let shift: u8 = input.trim().parse().unwrap();

    //
    // NEED TO HANDLE NEGATIVE SHIFTS HERE
    //

    let mut output_data = String::new();

    //
    // FUTE IMPROVEMENTS:
    // change from hashmaps to using byte literals
    // - b'a'
    //

    for &b in input_path.as_bytes() {
        let c = match b {
            // uppercase
            b'A'..=b'Z' => b'A' + (b - b'A' + shift) % 26,
            // lowercase
            b'a'..=b'z' => b'a' + (b - b'a' + shift) % 26,
            // everything else
            _ => b,
        };
        output_data.push(c as char);
    }

    fs::write(output_path, output_data)?;
    Ok(())
}
