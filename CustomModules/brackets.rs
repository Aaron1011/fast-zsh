use std;

fn brackets_paint(buf: &str, widget: &str, cursor: usize) {
    let mut style: &str;
    let mut level: u32 = 0;
    let mut matching_pos: usize = 0;
    let (bracket_color_size, buf_len, matchingpos, pos): u32 = 0;

    let mut level_pos: Vec<u32> = Vec::with_capacity(buf.len());
    let mut last_of_level: Vec<u32> = Vec::with_capacity(buf.len());
    let mut matching: Vec<u32> = Vec::with_capacity(buf.len());

    for _ in 0..matching.len() {
        matching.push(-1);
    }

    let i = 0;
    while i < buf.len() {
        let chr = buf[i];
        match chr {
            "(" | "[" | "{" => {
                level += 1;
                level_pos[i] = level;
                last_of_level[level] = i;
            },
            ")" | "]" | "}" => {
                matching_pos = last_of_level[level];
                level_pos[i] = level;
                level -= 1;

                if brackets_match(buf[matching_pos], buf[i]) {
                    matching[matching_pos] = i;
                    matching[i] = matching_pos;
                }
            },
            "\"" | "'" => {
                i = buf[pos+1:].find(chr).unwrap_or(buf.len() + 1);
            }
        }
    }

    for pos in level_pos.iter() {
       if matching[pos] != -1 {
           style = "fg=yellow"; // TODO
       } else {
           style = "fg=red";
       }
       
    }

    if widget != "zle-line-finish" {
        pos = cursor + 1;
        if level_pos[pos] != -1 && matching[pos] != -1 {
            let other_pos = matching[pos];
            do_highlight(other_pos - 1, other_pos);
        }
    }

}

fn brackets_match(first: char, second: char) -> bool {
    match first {
        "(" => second == ")",
        "[" => second == "]",
        "{" => second == "}",
    }
}

fn do_highlight(first: &str, second: &str) {
    let func: Shfunc = getshfunc("_zsh_highlight_add_highlight");
    doshfunc(func, std::ptr::null(), 1);
}
