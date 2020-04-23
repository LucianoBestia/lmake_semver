//! utilsmod.rs

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_NUMBER_SPACE: Regex = Regex::new(r#"[0-9] "#).unwrap();
}

// Here I distinguish between simple delimiters and markers.
// Markers can be INACTIVE if before them we put a number and a space.

// region: markers can be INACTIVE

/// return the position of start of the marked data
/// it is after the start marker. The newline is NOT jumped over.
/// The marker can be INACTIVE if before the marker there is a number and space.
pub fn find_pos_start_data_after_marker(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    // exclude the non active markers, that are just descriptions
    // they are recognized because they have a number and a space before.
    let mut pos = pos;
    loop {
        if let Some(pos_start_data) = find_from(md_text_content, pos, delimiter) {
            // before the marker there MUST NOT be a number and a space ex. "1 ",
            // because it means it is a description, not a marker.
            if pos_start_data < 2
                || !REGEX_NUMBER_SPACE
                    .is_match(&md_text_content[pos_start_data - 2..pos_start_data])
            {
                let pos_start_data = pos_start_data + delimiter.len();
                //break the loop, return
                return Some(pos_start_data);
            } else {
                pos = pos_start_data + 1;
                //continue the loop, find an active marker
            }
        } else {
            break;
        }
    }
    return None;
}

/// return the position of end of the delimited data before the marker
/// The marker can be INACTIVE if before the marker there is a number and space.
pub fn find_pos_end_data_before_marker(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    // exclude the false marker, that are just descriptions
    // they are recognized because they have a number and a space before.
    let mut pos = pos;
    loop {
        if let Some(pos_end_data) = find_from(md_text_content, pos, delimiter) {
            // before the marker there MUST NOT be a number and a space ex. "1 ",
            // because it means it is a description, not a marker.
            if pos_end_data < 2
                || !REGEX_NUMBER_SPACE.is_match(&md_text_content[pos_end_data - 2..pos_end_data])
            {
                //break the loop
                return Some(pos_end_data);
            } else {
                pos = pos_end_data + 1;
                //continue the loop
            }
        } else {
            break;
        }
    }
    return None;
}
// endregion: markers can be INACTIVE

// region: delimiters cannot be INACTIVE like markers

/// return the position of start of the delimited data after the delimiter
pub fn find_pos_start_data_after_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_start_data) = find_from(md_text_content, pos, delimiter) {
        let pos_start_data = pos_start_data + delimiter.len();
        return Some(pos_start_data);
    }
    // return
    None
}

/// return the position of end of the delimited data before the delimiter
pub fn find_pos_end_data_before_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_end_data) = find_from(md_text_content, pos, delimiter) {
        return Some(pos_end_data);
    }
    //return
    None
}

// endregion: delimiters cannot be INACTIVE like markers

#[allow(clippy::integer_arithmetic)]
/// find from_pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Option<usize> {
    let slice01 = text.get(from_pos..).unwrap();
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        // return Option with usize
        Some(from_pos + location)
    } else {
        // return Option with none
        option_location
    }
}
