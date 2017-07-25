#![crate_type = "dylib"]

use std::collections::HashMap;
use std::ffi::CString;
use linkroot;
use std::os::raw::c_char;
use {shfunc, getshfunc, doshfunc, Shfunc};

fn brackets_paint(buf: &str, widget: &str, cursor: usize) {
    let mut style: &str;
    let mut level: usize = 0;
    let mut matching_pos: usize = 0;
    let mut bracket_color_size: usize = 0;
    let mut buf_len: usize = 0;
    let mut pos: usize = 0;

    let mut level_pos: HashMap<usize, usize> = HashMap::new();
    let mut last_of_level: HashMap<usize, usize> = HashMap::new();
    let mut matching: HashMap<usize, usize> = HashMap::new();

    let chars: Vec<char> = buf.chars().collect();

    let mut it: Box<Iterator<Item=(usize, &char)>> = Box::new(chars.iter().enumerate());
    while let Some((i, chr)) = it.next() {
        match *chr {
            '(' | '[' | '{' => {
                level += 1;
                level_pos.insert(i, level);
                last_of_level.insert(level, i);
            },
            ')' | ']' | '}' => {
                matching_pos = *last_of_level.get(&level).unwrap();
                level_pos.insert(i, level);
                level -= 1;

                if brackets_match(*chars.get(matching_pos).unwrap_or(&' '), chars[i]) {
                    matching.insert(matching_pos, i);
                    matching.insert(i, matching_pos);
                }
            },
            '\"' | '\'' => {
                while let Some(val) = it.next() {
                    if val.1 != chr {
                        continue;
                    }
                }
                //it = Box::new(it.skip(chars[(pos+1)..].iter().position(|c| c == chr).unwrap_or(chars.len() + 1)));
            },
            _ => continue
        }
    }

    for pos in level_pos.values() {
       if matching.contains_key(pos) {
           style = "fg=yellow"; // TODO
       } else {
           style = "fg=red";
       }
       
    }

    if widget != "zle-line-finish" {
        pos = cursor + 1;
        if level_pos.get(&pos).is_some() && matching.get(&pos).is_some() {
            let other_pos = matching[&pos];
            do_highlight(other_pos - 1, other_pos, "standout");
        }
    }

}

fn brackets_match(first: char, second: char) -> bool {
    match first {
        '(' => second == ')',
        '[' => second == ']',
        '{' => second == '}',
        _ => false,
    }
}

fn do_highlight(start: usize, end: usize, style: &str) {
    let func_name = CString::new("_zsh_highlight_add_highlight").unwrap();
    unsafe {
        let func: Shfunc = getshfunc(func_name.as_ptr() as *mut c_char);
        doshfunc(func, 0 as *mut linkroot, 1);
    }
}
