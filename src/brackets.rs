use std::collections::HashMap;
use std::ffi::CString;
use linkroot;
use std::os::raw::{c_char, c_void};
use {getshfunc, doshfunc, newlinklist, insertlinknode, linknode};

pub fn brackets_paint(buf: &str, widget: &str, cursor: usize) {
    let mut style: String;
    let mut level: usize = 0;
    let bracket_color_size: usize = 5; // TODO
    let pos;

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
                let matching_pos = match last_of_level.get(&level) {
                    Some(val) => *val,
                    None => continue
                };
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

    for pos in level_pos.keys() {
       if matching.contains_key(pos) {
           style = format!("bracket-level-{}", ((level_pos[pos] - 1) % bracket_color_size) + 1);
       } else {
           style = "bracket-error".to_owned();
       }
       do_highlight(*pos, *pos + 1, &style);
       
    }

    if widget != "zle-line-finish" {
        pos = cursor + 1;
        if level_pos.get(&pos).is_some() && matching.get(&pos).is_some() {
            let other_pos = matching[&pos];
            do_highlight(other_pos, other_pos + 1, "standout");
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

#[cfg(not(test))]
fn do_highlight(start: usize, end: usize, style: &str) {
    unsafe {
        let func_name = str_to_ptr("_zsh_highlight_add_highlight");
        let func = getshfunc(func_name as *mut c_char);

        let list = newlinklist();
        insertlinknode(list, (*list).list.as_ref().last as *const linknode as *mut linknode, func_name);
        insertlinknode(list, (*list).list.as_ref().last as *const linknode as *mut linknode, str_to_ptr(&start.to_string()));
        insertlinknode(list, (*list).list.as_ref().last as *const linknode as *mut linknode, str_to_ptr(&end.to_string()));
        insertlinknode(list, (*list).list.as_ref().last as *const linknode as *mut linknode, str_to_ptr(style));

        doshfunc(func, list as *mut linkroot, 1);
    }
}

#[cfg(test)]
fn do_highlight(start: usize, end: usize, style: &str) {
}


fn str_to_ptr(s: &str) -> *mut c_void {
    CString::new(s.to_string()).unwrap().into_raw() as *mut c_void
}
