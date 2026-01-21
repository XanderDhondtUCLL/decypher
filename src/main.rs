use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use std::fs;
use std::io;

struct ViewData {
    text: SharedString,
}

impl Render for ViewData {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {}
}

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
        b'A'..=b'Z' => b'A' + (byte - b'A' + (26 - shift)) % 26,
        // lowercase
        b'a'..=b'z' => b'a' + (byte - b'a' + (26 - shift)) % 26,
        // everything else
        _ => byte,
    };
    return c as char;
}

fn encypher(input_file: String, output_path: &str, shift: u8) -> io::Result<()> {
    // character encyphering
    // goes over entire input file
    // destructures byte
    let mut output_data = String::new();
    for &byte in input_file.as_bytes() {
        let encrypted_char = char_encypher(byte, shift);
        output_data.push(encrypted_char);
    }
    fs::write(output_path, output_data)?;
    Ok(())
}

fn decypher(input_file: String, output_path: &str, shift: u8) -> io::Result<()> {
    // character decyphering
    // goes over entire input file
    // destructures byte
    let mut output_data = String::new();
    for &byte in input_file.as_bytes() {
        let encrypted_char = char_decypher(byte, shift);
        output_data.push(encrypted_char);
    }
    fs::write(output_path, output_data)?;
    Ok(())
}

fn main() -> io::Result<()> {
    // file IO
    let input_file: String = fs::read_to_string("data/input.txt")?;
    let output_path: &str = "data/output.txt";

    // CLI IO
    println!("encypher (1)\ndecypher (2)");
    let mut operation_input: String = String::new();
    io::stdin().read_line(&mut operation_input)?;

    println!("Shift input by:");
    let mut shift_input: String = String::new();
    io::stdin().read_line(&mut shift_input)?;

    // logic that calculates the amount to shift the bytes with
    let shift = match shift_input.trim().parse::<i32>() {
        Ok(n) => ((n % 26 + 26) % 26) as u8,
        Err(_) => {
            eprint!("shift must be a number");
            return Ok(()); // break the loop
        }
    };

    // process the operational input wether to
    // - encypher
    // - decypher
    match operation_input.trim() {
        "1" => match encypher(input_file, output_path, shift) {
            Ok(n) => n,
            Err(t) => {
                eprintln!("{t}");
            }
        },
        "2" => match decypher(input_file, output_path, shift) {
            Ok(n) => n,
            Err(t) => {
                eprintln!("{t}");
            }
        },
        _ => {
            eprintln!("Invalid operation!");
            return Ok(());
        }
    }

    Ok(())
}
