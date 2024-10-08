#![allow(non_upper_case_globals)]

use std::path::{Path, PathBuf};
static mut BG: Col = (0, 0, 0);
static mut FG: Col = (0, 0, 0);
type Col = (u8, u8, u8);
const black: Col = (50, 50, 50);
const grey: Col = (92, 84, 77);
const orange: Col = (254, 128, 25);
const white: Col = (235, 219, 178);
const green: Col = (167, 169, 66);
const blue: Col = (122, 149, 138);
const yellow: Col = (227, 178, 88);
const red: Col = (251, 73, 52);

fn git() -> Option<String> {
    let contents = std::fs::read_to_string(Path::new(".git/HEAD")).ok()?;
    contents
        .split('/')
        .last()
        .map(|name| name.trim().to_owned())
}

fn main() {
    let home = std::env::var("HOME").unwrap();
    let mut out = String::with_capacity(200);

    begin(orange, black, &mut out);
    c_dir(&mut out);
    print_exes(&mut out, &home);
    finish(&mut out);

    println!("{out}");
}

fn finish(out: &mut String) {
    set_fg(black, out);
    out.push_str("\x1B[49;m\x1B[39;m\n  ");

    push_fg(red, out);
    out.push_str("->\x1B[49;m\x1B[39;m ");
}

fn c_dir(out: &mut String) {
    if let Some(name) = git() {
        out.push_str(" ");
        out.push_str(name.as_str());
    } else if let Ok(path) = std::env::current_dir() {
        path.to_str().map(|path| out.push_str(path));
    }
}

fn begin(bg: Col, fg: Col, out: &mut String) {
    out.push_str(" ");
    set_fg(bg, out);
    out.push_str("");
    set_fg(fg, out);
    set_bg(bg, out);
}

fn transition(bg: Col, st: &mut String) {
    unsafe {
        push_fg(BG, st);
        push_bg(bg, st);
        st.push_str("");
        push_bg(bg, st);
        push_fg(FG, st);
        BG = bg;
    }
    // set_bg(, st);
}

fn set_bg(rgb: Col, st: &mut String) {
    push_bg(rgb, st);
    unsafe {
        BG = rgb;
    }
}
fn set_fg(rgb: Col, st: &mut String) {
    push_fg(rgb, st);
    unsafe {
        FG = rgb;
    }
}
fn push_bg(rgb: Col, st: &mut String) {
    st.push_str(&format!("\x1B[48;2;{};{};{}m", rgb.0, rgb.1, rgb.2));
}
fn push_fg(rgb: Col, st: &mut String) {
    st.push_str(&format!("\x1B[38;2;{};{};{}m", rgb.0, rgb.1, rgb.2));
}

fn is_nix_store_path(path: PathBuf) -> bool {
    let Ok(p) = path.canonicalize() else {
        return false;
    };
    p.starts_with("/nix")
}

fn find_exexcutables<const N: usize>(
    names: [&str; N],
    home_dir: &str,
) -> Option<([bool; N], [bool; N], bool)> {
    let path = std::env::var("PATH").ok()?;
    let mut found_in_shell = [false; N];
    let mut found_system = [false; N];
    let mut in_nix_shell = false;
    let mut dir_in_nix_shell = false;
    for dir in find_path_paths(&path, home_dir).into_iter().rev() {
        if in_nix_shell {
            dir_in_nix_shell = true;
        }
        if dir.starts_with("/run/wrappers") {
            in_nix_shell = true;
        }
        for file in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            for (idx, name) in names.iter().enumerate() {
                if file.file_name().to_str() == Some(name) {
                    if in_nix_shell {
                        found_in_shell[idx] = true
                    } else {
                        found_system[idx] = true
                    }
                }
            }
        }
    }

    Some((found_system, found_in_shell, dir_in_nix_shell))
}

fn find_path_paths(path: &str, home_dir: &str) -> Vec<PathBuf> {
    std::env::split_paths(path)
        .map(|x| {
            if let Ok(b) = x.strip_prefix("~") {
                let mut p = PathBuf::from(home_dir);
                p.push(b);
                p
            } else {
                x
            }
        })
        .flat_map(|x| x.canonicalize())
        .collect()
}

fn print_programs<const N: usize>(st: &mut String, found: &[bool; N], fns: &[&str; N]) {
    for f in found
        .iter()
        .zip(fns.iter())
        .filter_map(|(f, c)| f.then_some(c))
    {
        st.push_str(f);
    }
}

fn print_exes(st: &mut String, home_dir: &str) -> Option<()> {
    let programs_exec_names = ["gcc", "rustc", "java", "python", "node"];
    let pro_name = [" ", "󱘗 ", " ", " ", " "];
    let (found_in_sys, found_in_shell, in_nix_shell) =
        find_exexcutables(programs_exec_names, home_dir)?;

    transition(grey, st);
    transition(black, st);

    st.push_str(" ");
    push_fg(yellow, st);
    print_programs(st, &found_in_sys, &pro_name);

    if in_nix_shell {
        push_fg(blue, st);
        st.push_str("");
        push_fg(black, st);
        push_bg(blue, st);
        st.push_str("");
        push_bg(black, st);
        push_fg(blue, st);
        st.push_str(" ");
        push_fg(orange, st);
        st.push_str(">>=");
        push_fg(blue, st);
        // transition(blue, st);
        // transition(black, st);

        st.push_str(" ");
        print_programs(st, &found_in_shell, &pro_name);
        transition(blue, st);
        transition(black, st);
    }

    Some(())
}
