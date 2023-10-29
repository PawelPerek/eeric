use std::ascii;

use super::Data;

fn parse_single_byte(byte: &str) -> Result<u8, String> {
    if byte.len() == 3 && byte.starts_with('\'') && byte.ends_with('\'') {
        let ch = byte.as_bytes()[1];
        if ch.is_ascii() {
            return Ok(ch);
        }
    }
    byte.parse::<u8>().map_err(|err| err.to_string())
}

pub fn parse_bytes(bytes: &str) -> Result<Data, String> {
    let bytes = bytes
        .split(',')
        .map(str::trim)
        .map(parse_single_byte)
        .collect::<Result<_, _>>()?;
    Ok(Data::Byte(bytes))
}

fn parse_single_half(half: &str) -> Result<u16, String> {
    half.parse::<u16>().map_err(|err| err.to_string())
}

pub fn parse_halves(halves: &str) -> Result<Data, String> {
    let halves = halves
        .split(',')
        .map(str::trim)
        .map(parse_single_half)
        .collect::<Result<_, _>>()?;
    Ok(Data::Half(halves))
}

fn parse_single_word(word: &str) -> Result<u32, String> {
    word.parse::<u32>().map_err(|err| err.to_string())
}

pub fn parse_words(words: &str) -> Result<Data, String> {
    let words = words
        .split(',')
        .map(str::trim)
        .map(parse_single_word)
        .collect::<Result<_, _>>()?;
    Ok(Data::Word(words))
}

fn parse_single_quad(quad: &str) -> Result<u64, String> {
    quad.parse::<u64>().map_err(|err| err.to_string())
}

pub fn parse_quads(quads: &str) -> Result<Data, String> {
    let quads = quads
        .split(',')
        .map(str::trim)
        .map(parse_single_quad)
        .collect::<Result<_, _>>()?;
    Ok(Data::Quad(quads))
}

fn parse_single_float(float: &str) -> Result<f32, String> {
    float.parse::<f32>().map_err(|err| err.to_string())
}

pub fn parse_floats(floats: &str) -> Result<Data, String> {
    let floats = floats
        .split(',')
        .map(str::trim)
        .map(parse_single_float)
        .collect::<Result<_, _>>()?;
    Ok(Data::Float(floats))
}

fn parse_single_double(double: &str) -> Result<f64, String> {
    double.parse::<f64>().map_err(|err| err.to_string())
}

pub fn parse_doubles(doubles: &str) -> Result<Data, String> {
    let doubles = doubles
        .split(',')
        .map(str::trim)
        .map(parse_single_double)
        .collect::<Result<_, _>>()?;
    Ok(Data::Double(doubles))
}

fn parse_ascii(ascii: &str) -> Result<Vec<ascii::Char>, String> {
    if !ascii.starts_with('"') || !ascii.ends_with('"') {
        return Err("Ascii string needs to be wrapped in quotes".to_owned());
    }

    let unwrapped = &ascii[1..ascii.len() - 1];
    let ascii = unwrapped
        .as_ascii()
        .ok_or_else(|| format!("Unable to parse {} as an ascii string", ascii))?;
    Ok(ascii.to_vec())
}

pub fn parse_string(string: &str) -> Result<Data, String> {
    let string = parse_ascii(string)?;
    Ok(Data::Ascii(string))
}

pub fn parse_asciz(asciz: &str) -> Result<Data, String> {
    let mut asciz = parse_ascii(asciz)?;
    asciz.push(ascii::Char::Null);
    Ok(Data::Ascii(asciz))
}

pub fn parse_zero(length: &str) -> Result<Data, String> {
    length
        .parse::<usize>()
        .map(Data::Zero)
        .map_err(|err| err.to_string())
}
