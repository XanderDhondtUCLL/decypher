use std::char;
use std::fs;
use std::io;

//
// Function that shifts a byte representing an ASCII character
// byte: u8
// shift: u8
//
// formula:
// byte - b'A' so we work in 0-25
// add the shift. Take modulo of 26 of this new number so it wraps around the alphabet (from z back to a)
// Add shifted value to actual byte vallue
//
fn char_encypher(byte: u8, shift: u8) -> char {
    let c = match byte {
        // uppercase
        b'A'..=b'Z' => b'A' + (byte - b'A' + shift) % 26,
        // lowercase
        b'a'..=b'z' => b'a' + (byte - b'a' + shift) % 26,
        // everything else
        _ => byte,
    };
    return c as char;
}

//
// Function that shifts a byte representing an ASCII character
// byte: u8
// shift: u8
//
// Similar to encyphering but this time shifting to decypher
//
fn char_decypher(byte: u8, shift: u8) -> char {
    let c = match byte {
        // uppercase
        b'A'..=b'Z' => b'A' + (byte - b'A' - shift) % 26,
        // lowercase
        b'a'..=b'z' => b'a' + (byte - b'a' - shift) % 26,
        // everything else
        _ => byte,
    };
    return c as char;
}

fn main() -> io::Result<()> {
    // file IO
    let input_path: String = fs::read_to_string("data/input.txt")?;
    let output_path: &str = "data/output.txt";

    // CLI IO
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let mut shift: u8;

    //
    // ensure that program doesn't panic when alphabetical characters are inputted instead of shift
    //
    if input.chars().any(|c| c.is_ascii_alphabetic()) {
        eprintln!("Can only use numbers to input shift");
        return Ok(());
    }

    //
    // handling of negative shifts
    // if it starts negatively, since working with u8, do 26 - shift.
    // Has same effect as taking negative with i8
    //
    if input.starts_with("-") {
        input.remove(0);
        shift = input.trim().parse().unwrap();
        shift = 26 - shift;
    } else {
        shift = input.trim().parse().unwrap();
    }

    let mut output_data = String::new();

    // character encyphering
    // goes over entire input file
    // destructures byte
    for &byte in input_path.as_bytes() {
        let encrypted_char = char_encypher(byte, shift);
        output_data.push(encrypted_char);
    }

    fs::write(output_path, output_data)?;
    Ok(())
}
