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

    println!("encypher (1)\ndecypher (2)");
    let mut operation_input: String = String::new();
    io::stdin().read_line(&mut operation_input)?;

    match operation_input.trim(){
        "1" => println!("Encypher mode"),
        "2" => println!("Decypher mode"),
        _ => println!("test"),
    }

    println!("Shift input by:");
    let mut shift_input: String = String::new();
    io::stdin().read_line(&mut shift_input)?;
    let mut shift: u8;

    //
    // ensure that program doesn't panic when alphabetical characters are inputted instead of shift
    //
    if shift_input.chars().any(|c| c.is_ascii_alphabetic()) {
        eprintln!("Can only use numbers to input shift");
        return Ok(());
    }

    //
    // handling of negative shifts
    // if it starts negatively, since working with u8, do 26 - shift.
    // Has same effect as taking negative with i8
    //
    if shift_input.starts_with("-") {
        shift_input.remove(0);
        shift = shift_input.trim().parse().unwrap();
        shift = 26 - shift;
    } else {
        shift = shift_input.trim().parse().unwrap();
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
