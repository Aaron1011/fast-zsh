use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::ffi::CString;
use linkroot;
use std::os::raw::{c_char, c_void};
use {getshfunc, doshfunc, newlinklist, insertlinknode, linknode, LinkList};

/*struct IntHasher;

impl Hasher for IntHasher {

    fn write(&mut self, bytes: &[u8]) {
		assert!(bytes.len(), mem::size_of::<usize>());

    }

    fn finish(&self) -> u64 {
        // Your hashing algorithm goes here!
        unimplemented!()
    }
}*/

pub fn brackets_paint(bracket_color_size: usize, buf: &str, cursor: usize, widget: &str) {
    let mut style: String = "".to_owned();
    let mut level: isize = 0;

    let mut cursor_level = false;

    let mut level_pos: Vec<(usize, isize)> = Vec::new();
    //let mut level_pos: HashMap<usize, isize> = HashMap::new();

    let mut last_of_level: HashMap<isize, usize> = HashMap::new();

    let mut matching: HashMap<usize, usize> = HashMap::new();

    let chars: Vec<char> = buf.chars().collect();

    let mut it = chars.iter().enumerate();
    while let Some((i, chr)) = it.next() {
        match *chr {
            '(' | '[' | '{' => {
                level += 1;
                level_pos.push((i, level));
                last_of_level.insert(level, i);
            },
            ')' | ']' | '}' => {
                let matching_pos = *last_of_level.get(&level).unwrap_or(&0);
                level_pos.push((i, level));
                level = level.saturating_sub(1);

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
                    break;
                }
            },
            _ => continue
        }
    }

    for &(pos, level) in level_pos.iter() {
       if matching.contains_key(&pos) {
           if bracket_color_size != 0 {
               style = format!("bracket-level-{}", (level - 1) % bracket_color_size as isize + 1);
           }
       } else {
           style = "bracket-error".to_owned();
       }
       if cursor == pos {
           cursor_level = true;
       }
       do_highlight(pos, pos + 1, &style);

    }

    if widget != "zle-line-finish" {
        let pos = cursor; // cursor is already zero-based
        if cursor_level && matching.get(&pos).is_some() {
            let other_pos = matching[&pos];
            do_highlight(other_pos, other_pos + 1, "cursor-matchingbracket");
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
        insertlinknode(list, latest_node(list), func_name);
        insertlinknode(list, latest_node(list), str_to_ptr(&start.to_string()));
        insertlinknode(list, latest_node(list), str_to_ptr(&end.to_string()));
        insertlinknode(list, latest_node(list), str_to_ptr(style));

        doshfunc(func, list as *mut linkroot, 1);
    }
}

fn latest_node(list: LinkList) -> *mut linknode {
    unsafe { (*list).list }.last as *const linknode as *mut linknode
}

#[cfg(test)]
fn do_highlight(start: usize, end: usize, style: &str) {
}


fn str_to_ptr(s: &str) -> *mut c_void {
    CString::new(s.to_string()).unwrap().into_raw() as *mut c_void
}
