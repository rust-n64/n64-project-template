use alloc::{fmt, format};
use alloc::string::{String, ToString};
use core::fmt::Arguments;
use core::slice;

const LENGTH_REG: *mut u32 = 0xB3FF0014u32 as *mut u32;
const BUF_START: *mut u32 = 0xB3FF0020u32 as *mut u32;
const BUF_SIZE: usize = 0x200;

pub fn write_fmt(args: Arguments) {
    write_raw(fmt::format(args));
}

pub fn dump_range(start_ptr: u32, count: usize, bytes_per_line: usize) {
    let start_ptr = (start_ptr & 0xFFFFFFFC) as *const u32;
    let data = unsafe { slice::from_raw_parts(start_ptr, count) };
    
    dump(data, bytes_per_line);
}

/// Prints a hex and char dump of the provided data.
pub fn dump<T: AsRef<[u32]>>(data: T, bytes_per_line: usize) {
    let data = data.as_ref();
    let mut text = String::new();
    
    let mut count = 0;
    let mut chars = String::new();
    for word in data {
        for byte in word.to_be_bytes() {
            text.push_str(&format!("{byte:02X} "));
            
            let c = byte as char;
            chars.push(if c.is_ascii_alphanumeric() || c.is_ascii_graphic() { c } else { '.' });
            
            count += 1;
            if count == bytes_per_line {
                text.push_str(&format!("|{chars}|\n"));
                count = 0;
                chars.clear();
            }
        }
    }
    if count > 0 {
        let spacing = " ".repeat((bytes_per_line - count) * 3);
        text.push_str(&format!("{spacing}|{chars}|\n"));
    }
    text = text.trim_end().to_string();
    text.push('\n');
    
    write_raw(text);
}

pub fn dump_u8<T: AsRef<[u8]>>(data: T, bytes_per_line: usize) {
    let data = data.as_ref();
    let mut text = String::new();
    
    let mut count = 0;
    let mut chars = String::new();
    for byte in data {
        text.push_str(&format!("{byte:02X} "));
        
        let c = *byte as char;
        chars.push(if c.is_ascii_alphanumeric() || c.is_ascii_graphic() { c } else { '.' });
        
        count += 1;
        if count == bytes_per_line {
            text.push_str(&format!("|{chars}|\n"));
            count = 0;
            chars.clear();
        }
    }
    if count > 0 {
        let spacing = " ".repeat((bytes_per_line - count) * 3);
        text.push_str(&format!("{spacing}|{chars}|\n"));
    }
    text = text.trim_end().to_string();
    text.push('\n');
    
    write_raw(text);
}

//TODO Write dump function for AsRef<[u8]> which reads in 4-byte aligned words and then outputs only the expected range of bytes.
//     This should prevent any weird bugs like non-u32 reads over the PI bus resulting in corruption.
//
// If slice is over 0x01 through 0x09 below:
//     00 01 02 03
//     04 05 06 07
//     08 09 0A 0C
//
// then function should read 3 words, and dump only:
//     01 02 03 04
//     05 06 07 08
//     09

pub fn write_raw<T: AsRef<[u8]>>(data: T) { //TODO add critical section here
    // Credit to Lemmy for algorithm: https://github.com/lemmy-64/n64-systemtest/blob/main/src/isviewer.rs
    for chunk in data.as_ref().chunks(BUF_SIZE) {
        let mut value = 0;
        let mut shift = 24u32;
        let mut i = 0;
        for byte in chunk {
            value |= (*byte as u32) << shift;
            if shift == 0 {
                push(i, value);
                i += 1;
                shift = 24;
                value = 0;
            } else {
                shift -= 8;
            }
        }
        if shift < 24 {
            push(i, value);
        }
        flush(chunk.len());
    }
}

#[inline(always)]
fn push(word_count: usize, word: u32) {
    unsafe { BUF_START.add(word_count).write_volatile(word); }
}

#[inline(always)]
fn flush(byte_count: usize) {
    unsafe { LENGTH_REG.write_volatile(byte_count as u32); }
}