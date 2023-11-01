use std::{fs::File, io::{Read, SeekFrom, Seek}, fmt::Display};

pub fn get_index(vector: &Vec<String>, key: &str) -> i32 {
    let lookup = vector.iter().position(|v| v == key);
    match lookup {
        Some(index) => return index as i32,
        None => return -1
    }
}

pub fn get_index_of_line(txt: &str, index: usize) -> usize {
    let lines_slice = &txt
        .split("\n")
        .collect::<Vec<&str>>()
        [0..index];
    let lines = lines_slice
        .into_iter()
        .map(|line| {line.len()})
        .collect::<Vec<usize>>();
    // Count chars of lines before index
    let result_index = lines
        .into_iter()
        .reduce(|accum, line_len| {
            // Don't forget the \n char
            return accum + line_len + 1;
        })
        .unwrap();
    return result_index;
}

pub fn count_occurences_not_in_string(txt: &str, pat: char) -> usize {
    let mut count = 0 as usize;
    let mut is_in_string = false;
    let mut is_escaped = false;
    let mut chars = txt.chars();
    while let Some(character) = chars.next() {
        if is_escaped {
            is_escaped = false;
            continue;
        }
        if character == '\\' {
            is_escaped = true;
            continue;
        }
        if character == pat && !is_in_string {
            count += 1;
        }
        if character == '"' || character == '\''{
            is_in_string = !is_in_string;
        }
    }
    return count;
}

#[derive(Debug)]
pub struct Conveyor<T> {
    items: Vec<T>,
    size: usize,
}

impl<T> Conveyor<T> {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Size of conveyor cannot be 0");
        }
        Self {
            items: Vec::new(),
            size,
        }
    }
    /**
     * @returns Possible popped element if size exceeded
     */
    pub fn push(&mut self, item: T) -> Option<T> {
        let pop =if self.items.len() + 1 > self.size {
            self.items.pop()
        } else {
            None
        };
        self.items.push(item);
        return pop;
    }
    pub fn next(&self) -> Option<&T> {
        return self.items.first()
    }
    pub fn pop(&mut self) -> Option<T> {
        return self.items.pop()
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }
    pub fn last(&self) -> Option<&T> {
        self.items.last()
    }
}

impl Conveyor<char> {
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for c in &self.items {
            string.push(*c);
        }
        return string;
    }
}

pub fn get_char_length(buf: [u8; 4]) -> u8 {
    /* 0xxxxxxx */
    const SMALL_FIRST_BYTE: u8      = 0b01111111;
    /* 110yyyyy 10xxxxxx */
    const MEDIUM_FIRST_BYTE: u8     = 0b11011111;
    const MEDIUM_SECOND_BYTE: u8    = 0b10111111;
    /* 1110zzzz 10yyyyyy 10xxxxxx */
    const LARGE_FIRST_BYTE: u8      = 0b11101111;
    const LARGE_SECOND_BYTE: u8     = 0b10111111;
    const LARGE_THIRD_BYTE: u8      = 0b10111111;
    /* 11110uuu 10uuzzzz 10yyyyyy 10xxxxxx */
    const COLLOSAL_FIRST_BYTE: u8   = 0b11110111;
    const COLLOSAL_SECOND_BYTE: u8  = 0b10111111;
    const COLLOSAL_THIRD_BYTE: u8   = 0b10111111;
    const COLLOSAL_FOURTH_BYTE: u8  = 0b10111111;
    if buf[0] | SMALL_FIRST_BYTE    == SMALL_FIRST_BYTE {
        return 1;
    }
    if buf[0] | MEDIUM_FIRST_BYTE   == MEDIUM_FIRST_BYTE
    && buf[1] | MEDIUM_SECOND_BYTE  == MEDIUM_SECOND_BYTE {
        return 2;
    }
    if buf[0] | LARGE_FIRST_BYTE    == LARGE_FIRST_BYTE
    && buf[1] | LARGE_SECOND_BYTE   == LARGE_SECOND_BYTE 
    && buf[2] | LARGE_THIRD_BYTE    == LARGE_THIRD_BYTE {
        return 3;
    }
    if buf[0] | COLLOSAL_FIRST_BYTE == COLLOSAL_FIRST_BYTE
    && buf[1] | COLLOSAL_SECOND_BYTE== COLLOSAL_SECOND_BYTE 
    && buf[2] | COLLOSAL_THIRD_BYTE == COLLOSAL_THIRD_BYTE 
    && buf[3] | COLLOSAL_FOURTH_BYTE== COLLOSAL_FOURTH_BYTE {
        return 4;
    }
    panic!("What");
}


pub struct CharReader {
    low_mem: bool,
    file: File,
    file_contents: Option<Vec<char>>,
    current_pos: usize,
    file_length: usize,
}

impl CharReader {
    pub fn new(mut file: File, low_mem: Option<bool>) -> Self {
        let mut file_contents = None;
        let mut buf = String::new();
        if !(low_mem.unwrap_or(false)) {
            file.read_to_string(&mut buf)
                .expect("Cannot read file contents");
            let chars = buf.chars();
            let chars = chars.collect();
            file_contents = Some(chars);
        }
        // Get the file length
        let seek = file.seek(SeekFrom::End(0)).unwrap();
        Self {
            low_mem: low_mem.unwrap_or(false), 
            file, 
            file_contents,
            current_pos: 0,
            file_length: seek as usize,
        }
    }
    fn read(&mut self) -> Option<char> {
        let next = self.file_contents.as_ref().unwrap()
            .get(self.current_pos);
        self.current_pos += 1;
        match next {
            Some(next) => {
                Some(*next)
            },
            None => {
                None
            },
        }
    }
    fn read_low_mem(&mut self) -> Option<char> {
        let mut buf = [0; 4];
        if self.file_length <= self.current_pos {
            return None
        }
        self.file.seek(SeekFrom::Start(self.current_pos as u64))
            .expect(format!("Could not read file, attempted to seek to: {}", self.current_pos).as_str());
        let _bytes_read = self.file.read(&mut buf[..])
            .expect("Could not read file");
        let char_length = get_char_length(buf) as usize;
        self.current_pos += char_length;
        let buf = [
            buf[0],
            if char_length > 1 {buf[1]} else {0},
            if char_length > 2 {buf[2]} else {0},
            if char_length > 3 {buf[3]} else {0},
        ];
        char::from_u32(u32::from_le_bytes(buf))
    }
}

impl Iterator for CharReader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.low_mem {
            self.read_low_mem()
        } else {
            self.read()
        }
    }
}