use std::char;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
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
