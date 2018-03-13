extern crate test;

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use std::cell::RefCell;
use std::ptr::null_mut;

struct Highlight<'a> {
    start: usize,
    end: usize,
    style: &'a str
}

fn set_at_pos<T: Default>(vec: &mut Vec<T>, pos: usize, elem: T) {
    for _ in 0..((pos+1)-vec.len()) {
        vec.push(Default::default());
    }
    vec[pos] = elem;
}

#[cfg(test)]
fn get_styles() -> impl Iterator<Item=(String,String)> {
    use std::iter::empty;
    empty()
}

#[cfg(not(test))]
fn get_styles() -> impl Iterator<Item=(String, String)> {
    use {gethparam, gethkparam};

    let styles_keys;
    let styles_vals;
    unsafe {
        styles_keys = c_array_to_vec(gethkparam(str_to_ptr("ZSH_HIGHLIGHT_STYLES") as *mut c_char));
        styles_vals = c_array_to_vec(gethparam(str_to_ptr("ZSH_HIGHLIGHT_STYLES") as *mut c_char));
    }

    styles_keys.into_iter().zip(styles_vals)
}

pub fn brackets_paint(bracket_color_size: usize, buf: &str, cursor: usize, widget: &str) {

    let mut bracket_error = "".to_owned();
    let mut cursor_matching_bracket = "".to_owned();
    let mut bracket_level: Vec<String> = Vec::new();
    let empty = "".to_owned();

    for (key, val) in get_styles() {
        match key.as_ref() {
            "bracket-error" => bracket_error = val,
            "cursor-matchingbracket" => cursor_matching_bracket = val,
            _ => {
                if key.starts_with("bracket-level-") {
                    let num = &key["bracket-level-".len()..];
                    set_at_pos(&mut bracket_level, num.parse().unwrap(), val);
                }
            }
        }
    }

    let mut level: usize = 0;

    // We keep the full usize range of level by tracking when the level goes negative separately
    // Due to the way the highlighting logic works, a bracket match can never occur on a negative
    // level. This allows us to skip tracking information for negative levels, which would make it
    // difficult to use a Vec
    let mut level_neg: usize = 0;

    let mut cursor_level = false;
    let mut level_pos: Vec<(usize, usize)> = Vec::new();
    let mut last_of_level: Vec<usize> = Vec::new();
    //let mut matching: HashMap<usize, usize> = HashMap::new();

    let chars: Vec<(char, RefCell<Option<usize>>)> = buf.chars().map(|c| (c, RefCell::new(None))).collect();

    let mut highlights: Vec<Highlight> = Vec::new();

    let mut it = chars.iter().enumerate();
    while let Some((i, &(ref chr, ref match_pos))) = it.next() {
        match *chr {
            '(' | '[' | '{' => {
                if level_neg == 0 {
                    level += 1;
                    if last_of_level.get(level - 1).is_some() {
                        last_of_level[level - 1] = i;
                    } else {
                        last_of_level.push(i);
                    }
                } else {
                    level_neg -= 1;
                }

                level_pos.push((i, level));
            },
            ')' | ']' | '}' => {
                level_pos.push((i, level));

                if level == 0 {
                    level_neg += 1;
                    continue;
                }


                let matching_pos: Option<&usize> = last_of_level.get(level - 1);
                level -= 1;

                if brackets_match(matching_pos.and_then(|p| chars.get(*p).map(|s| s.0)).unwrap_or(' '), chars[i].0) {
                    let matching_pos = *matching_pos.unwrap();
                    //
                    *match_pos.borrow_mut() = Some(matching_pos);

                    let mut a = chars[matching_pos].1.borrow_mut();
                    *a = Some(i);

                    //matching.insert(matching_pos, i);
                    //matching.insert(i, matching_pos);
                }
            },
            _ => continue
        }
    }

    for &(pos, level) in &level_pos {
        if cursor == pos {
            cursor_level = true;
        }

        if chars[pos].1.borrow().is_some() {
            if bracket_color_size != 0 {
                highlights.push(Highlight {
                    start: pos,
                    end: pos + 1,
                    style: bracket_level.get((level - 1) % bracket_color_size + 1).unwrap_or(&empty)
                });
            }
        } else {
            highlights.push(Highlight {
                start: pos,
                end: pos + 1,
                style: &bracket_error
            });
        }
    }

    if widget != "zle-line-finish" {
        let pos = cursor; // cursor is already zero-based
        if cursor_level {
            let other_pos = chars[pos].1.borrow();
            if let Some(real_pos) = *other_pos {
                highlights.push(Highlight {
                    start: real_pos,
                    end: real_pos + 1,
                    style: &cursor_matching_bracket
                });
            }
        }
    }

    add_highlights(&highlights);

}

#[cfg(test)]
fn add_highlights(highlights: &[Highlight]) {
    test::black_box(highlights);
}

#[cfg(not(test))]
fn add_highlights(highlights: &[Highlight]) {
    use {getaparam, setaparam, zalloc};
    use std::mem;

    unsafe {
        let region_highlight_str = str_to_ptr("region_highlight") as *mut c_char;

        let mut param_ptr = getaparam(region_highlight_str);
        let mut param_len = 0;

        if !param_ptr.is_null() {
            while !(*param_ptr).is_null() {
                param_len += 1;
                //zsh_highlights.push(CStr::from_ptr(*param_ptr as *const c_char).to_owned().into_string().unwrap());
                param_ptr = param_ptr.offset(1)
            }
        }

        let alloc_len = param_len + highlights.len();
        let alloc_size = alloc_len * mem::size_of::<*const c_char>() + 1; // store NULL at the end

        let buf: *mut *mut c_char = zalloc(alloc_size) as *mut *mut c_char;
        *buf.offset(alloc_len as isize) = null_mut() as *mut c_char;
        for (i, highlight) in highlights.iter().enumerate() {
            let highlight_str =format!("{} {} {}", highlight.start, highlight.end, highlight.style); 
            *buf.offset(i as isize) = str_to_ptr(&highlight_str) as *mut c_char;
        }

        setaparam(region_highlight_str, buf);
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

unsafe fn c_array_to_vec(mut array: *mut *mut c_char) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    if !array.is_null() {
        while !(*array).is_null() {
            vec.push(CStr::from_ptr(*array as *const c_char).to_owned().into_string().unwrap());
            array = array.offset(1)
        }
    }
    vec
}

fn str_to_ptr(s: &str) -> *mut c_void {
    CString::new(s.to_string()).unwrap().into_raw() as *mut c_void
}
