use std::str::Utf8Error;

use egui::{text_selection::text_cursor_state::byte_index_from_char_index, TextBuffer};

use crate::{app, rms_error::RmsError};

pub fn csv_to_insert(csv: &str) -> Result<String, RmsError> {
    let mut output = String::new();

    if csv.is_empty() {
        return Err(RmsError::Format)
    }

    let mut csviter = csv.lines();
    let header = csviter.next().expect("Should always be valid since we know csv is not empty");
    output.push_str(&format!("({}) VALUES ", header));

    for line in csviter {
        if line.split(',').count() != header.split(',').count() {
            return Err(RmsError::Format)
        }
        output.push_str(&format!("({}),", line));
    }
    output.pop();

    Ok(output)
}

pub fn lines_to_insert(lines: &Vec<Vec<&str>>) -> Result<String, RmsError> {
    let mut output = String::new();

    if lines.is_empty() {
        return Err(RmsError::Format)
    }

    let header = &lines[0];
    output.push_str(&format!("({}) VALUES ", header.join(",")));

    for line in lines {
        if line.len() != header.len() {
            return Err(RmsError::Format)
        }
        output.push_str(&format!("({}),", line.join(",")));
    }
    output.pop();

    Ok(output)
}

pub fn lines_to_csv(lines: &[app::Product], skiplist: &[u8]) -> String {
    
    let mut printer = String::new();
    
    if skiplist[0] != 0 {
        printer.push_str("id,i");
        printer.push(';');
    }
    if skiplist[1] != 0 {
        printer.push_str("name,t");
        printer.push(';');
    }
    if skiplist[2] != 0 {
        printer.push_str("description,t");
        printer.push(';');
    }
    if skiplist[3] != 0 {
        printer.push_str("price,f");
        printer.push(';');
    }
    if skiplist[4] != 0 {
        printer.push_str("location,t");
        printer.push(';');
    }
    printer.pop();
    printer.push('\n');


    for item in lines {
        if skiplist[0] != 0 {
            printer.push_str(&item.id);
            printer.push(';');
        }
        if skiplist[1] != 0 {
            printer.push_str(&item.name);
            printer.push(';');
        }
        if skiplist[2] != 0 {
            printer.push_str(&item.description);
            printer.push(';');
        }
        if skiplist[3] != 0 {
            printer.push_str(&item.price);
            printer.push(';');
        }
        if skiplist[4] != 0 {
            printer.push_str(&item.location);
            printer.push(';');
        }
        printer.pop();
        printer.push('\n');
    }

    printer.pop();

    printer
}

/// Removes the trailing 0 bytes from a str created from a byte buffer
pub fn bytes_to_str(bytes: &[u8]) -> Result<&str, Utf8Error> {
    let mut index: usize = 0;
    let len = bytes.len();
    let mut start: usize = 0;
    
    while index < len {
        if bytes[index] != 0 {
            break
        }
        index += 1;
        start += 1;
    }

    if bytes.is_empty() {
        return Ok("")
    }

    if start >= bytes.len()-1 {
        return Ok("")
    }

    let mut stop: usize = start;
    while index < len {
        if bytes[index] == 0 {
            break
        }
        index += 1;
        stop += 1;
    }

    std::str::from_utf8(&bytes[start..stop])
}

#[derive(Clone, Copy, Hash, PartialEq)]
pub struct String64 {
    inner: [u8;64],
}

impl std::fmt::Debug for String64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("String64").field("inner", &self.as_str()).finish()
    }
}

impl std::fmt::Display for String64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = bytes_to_str(&self.inner).expect(&format!("A String64 should always be valid utf8.\nThe String64 that was just attempted to Display was:\n{:x?}", self.inner));
        write!(f, "{}", text)
    }
}

impl Default for String64 {
    fn default() -> Self {
        Self { inner: [0;64] }
    }
}

/// Turns a &str into a String64. If the &str has more than 64 bytes, the last bytes will be cut.
impl From<&str> for String64 {
    fn from(s: &str) -> Self {

        let mut inner = [0u8;64];

        let mut min = std::cmp::min(s.len(), 64);
        inner[0..min].copy_from_slice(&s.as_bytes()[0..min]);

        loop {
            if min == 0 {break}
            match std::str::from_utf8(&inner[0..min]) {
                Ok(_) => break,
                Err(_) => min -= 1,
            }
        }

        String64 {
            inner
        }

    }
}


impl TryFrom<&[u8]> for String64 {
    type Error = Utf8Error;

    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        let mut inner = [0u8;64];

        let min = std::cmp::min(s.len(), 64);
        inner[0..min].copy_from_slice(&s[0..min]);

        match std::str::from_utf8(&inner) {
            Ok(_) => {
                Ok(String64 {inner})
            },
            Err(e) => Err(e)
        }
    }
}

impl Eq for String64 {}

impl Ord for String64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for String64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl TextBuffer for String64 {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        // Get the byte index from the character index
        let byte_idx = byte_index_from_char_index(self.as_str(), char_index);

        // Then insert the string64
        let mut temp = self.to_string();
        temp.insert_str(byte_idx, text);
        *self = String64::from(temp.as_str());

        text.chars().count()
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        assert!(char_range.start <= char_range.end);

        // Get both byte indices
        let byte_start = byte_index_from_char_index(self.as_str(), char_range.start);
        let byte_end = byte_index_from_char_index(self.as_str(), char_range.end);

        // Then drain all characters within this range
        let mut temp = self.to_string();
        temp.drain(byte_start..byte_end);
        *self = String64::from(temp.as_str());
    }

    fn clear(&mut self) {
        *self = String64::new();
    }

    fn replace_with(&mut self, text: &str) {
        *self = String64::from(text);
    }
}

impl String64 {

    pub fn new() -> Self {
        String64 {
            inner: [0u8; 64]
        }
    }

    pub fn len(&self) -> usize {
        let mut output = 0;
        for byte in self.inner {
            match byte {
                0 => break,
                _ => output += 1,
            }
        }
        output
    }

    pub fn push(&mut self, s: &str) {

        if self.len() + s.len() > 64 {
            return
        }

        let mut end_index = 0;
        for (index, byte) in self.inner.iter().enumerate() {
            if byte == &0 {
                end_index = index+1;
            }
        }

        for (index, byte) in s.as_bytes().iter().enumerate() {
            self.inner[index+end_index] = *byte;
        }

    }

    pub fn as_str(&self) -> &str {
        // This is safe since an enforced invariant of String64 is that it is utf8
        std::str::from_utf8(&self.inner[0..self.len()]).unwrap()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[0..self.len()]
    }

    pub fn raw(&self) -> &[u8] {
        &self.inner
    }

    /// These functions may panic and should only be called if you are certain that the String64 contains a valid number
    pub fn to_i32(&self) -> i32 {
        self.as_str().parse::<i32>().unwrap()
    }

    /// These functions may panic and should only be called if you are certain that the String64 contains a valid number
    pub fn to_f32(&self) -> f32 {
        self.as_str().parse::<f32>().unwrap()
    }

    pub fn to_i32_checked(&self) -> Result<i32, std::num::ParseIntError> {
        self.as_str().parse::<i32>()
    }

    pub fn to_f32_checked(&self) -> Result<f32, std::num::ParseFloatError> {
        self.as_str().parse::<f32>()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

   #[test]
    pub fn test_csv_to_insert() {
        let csv = "one, two, three, four, five\n1,2,3,4,5\n6,7,8,9,10";
        let parsed = csv_to_insert(csv).unwrap();

        println!("{:?}", parsed)
    }

    #[test]
    pub fn test_lines_to_insert() {
        let csv = vec![vec!["one", "two", "three", "four", "five"], vec!["1","2","3","4","5"], vec!["6","7","8","9","10"]];
        let parsed = lines_to_insert(&csv).unwrap();

        println!("{:?}", parsed)
    }

}