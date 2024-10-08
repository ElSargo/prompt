#![allow(non_upper_case_globals)]

use core::panic;
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    io::{stdout, Write},
    mem::MaybeUninit,
    path::{Path, PathBuf},
};
static mut BG: Col = (0, 0, 0);
static mut FG: Col = (0, 0, 0);
type Col = (u8, u8, u8);
const black: Col = (50, 50, 50);
const grey: Col = (92, 84, 77);
const orange: Col = (254, 128, 25);
// const white: Col = (235, 219, 178);
// const green: Col = (167, 169, 66);
const blue: Col = (122, 149, 138);
const yellow: Col = (227, 178, 88);
const red: Col = (251, 73, 52);

fn git() -> Option<OsString> {
    let contents = std::fs::read_to_string(Path::new(".git/HEAD")).ok()?;
    contents
        .split('/')
        .last()
        .map(|name| name.trim().to_owned().into())
}

fn main() {
    let home = OsString::from(std::env::var("HOME").unwrap());
    let mut out = OsString::with_capacity(200);

    let sig_dirs = [
        (".config", " "),
        ("sys-nix", " "),
        ("Documents", "󰈙 "),
        ("Videos", " "),
        ("Pictures", " "),
        ("Downloads", " "),
        ("Music", "󰝚 "),
    ]
    .iter()
    .map(|x| unsafe { *std::mem::transmute::<&(&str, &str), &(&OsStr, &OsStr)>(x) })
    .collect();

    begin(orange, black, &mut out);
    c_dir(&mut out, &home, &sig_dirs);
    print_exes(&mut out, &home);
    finish(&mut out);

    let mut stdo = stdout().lock();
    stdo.write(&out.as_encoded_bytes()).unwrap();
}

fn finish(out: &mut OsString) {
    set_fg(black, out);
    out.push("\x1B[49;m\x1B[39;m\n  ");

    push_fg(red, out);
    out.push("->\x1B[49;m\x1B[39;m ");
}

struct Components<T, const SIZE: usize> {
    data: [MaybeUninit<T>; SIZE],
    len: usize,
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for Components<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len == 0 {
            return write!(f, "Components:[]");
        }
        write!(f, "Components:[")?;
        for i in 0..self.len - 1 {
            write!(f, "{:?}, ", unsafe { self.data[i].assume_init_ref() })?;
        }
        write!(f, "{:?}]", unsafe {
            self.data[self.len - 1].assume_init_ref()
        })
    }
}

impl<T: std::fmt::Debug, const N: usize> Components<T, N> {
    const ZEROS: MaybeUninit<T> = MaybeUninit::zeroed();
    fn new() -> Components<T, N> {
        Components {
            data: [Self::ZEROS; N],
            len: 0,
        }
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            Err(value)
        } else {
            self.data[self.len].write(value);
            self.len += 1;
            Ok(())
        }
    }

    fn get(&self, idx: usize) -> Option<&T> {
        (idx < self.len).then_some(unsafe { self.data[idx].assume_init_ref() })
    }
    fn slice(&self, range: std::ops::Range<usize>) -> &[T] {
        if range.end >= self.len {
            panic!("Out of bounds");
        }
        unsafe { std::mem::transmute(&self.data[range]) }
    }
}

impl<T, const N: usize> Drop for Components<T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                drop(self.data[i].assume_init_read());
            }
        }
    }
}

fn components<const N: usize>(input: &OsStr) -> Components<&OsStr, N> {
    let mut output = Components::new();
    let mut start_idx = 0;
    let input_bytes = input.as_encoded_bytes();
    input_bytes
        .iter()
        .chain(std::iter::once(&b'/'))
        .enumerate()
        .filter_map(|(idx, b)| (*b == b'/').then_some(idx))
        .filter_map(|slice_idx| {
            let range = start_idx..slice_idx;
            start_idx = slice_idx + 1;
            (range.start != range.end).then_some(range)
        })
        .take(N)
        .for_each(|range| {
            output
                .push(unsafe { OsStr::from_encoded_bytes_unchecked(&input_bytes[range]) })
                .expect("Never fails bc we only took 5 elements");
        });

    output
}

fn c_dir(out: &mut OsString, home_dir: &OsStr, sig_dirs: &HashMap<&OsStr, &OsStr>) {
    if let Ok(path) = std::env::current_dir() {
        let path = path.into_os_string();
        let path_comps = components::<5>(&path);
        let home_comps = components::<2>(home_dir);
        let home_range = 0..home_comps.len - 1;
        if path_comps.len >= home_comps.len
            && path_comps.slice(home_range.clone()) == home_comps.slice(home_range)
        {
            let searched_until = if let Some(search_component) = path_comps.get(2) {
                out.push(sig_dirs.get(search_component).unwrap_or(search_component));
                3
            } else {
                out.push(" ");
                2
            };
            for i in searched_until..path_comps.len {
                out.push("/");
                out.push(path_comps.get(i).unwrap());
            }
        } else {
            out.push(&path);
        }
    }
    if let Some(name) = git() {
        out.push("  ");
        out.push(name);
    }
}

fn begin(bg: Col, fg: Col, out: &mut OsString) {
    out.push(" ");
    set_fg(bg, out);
    out.push("");
    set_fg(fg, out);
    set_bg(bg, out);
}

fn transition(bg: Col, st: &mut OsString) {
    unsafe {
        push_fg(BG, st);
        push_bg(bg, st);
        st.push("");
        push_bg(bg, st);
        push_fg(FG, st);
        BG = bg;
    }
    // set_bg(, st);
}

fn set_bg(rgb: Col, st: &mut OsString) {
    push_bg(rgb, st);
    unsafe {
        BG = rgb;
    }
}
fn set_fg(rgb: Col, st: &mut OsString) {
    push_fg(rgb, st);
    unsafe {
        FG = rgb;
    }
}
fn push_bg(rgb: Col, st: &mut OsString) {
    st.push(&format!("\x1B[48;2;{};{};{}m", rgb.0, rgb.1, rgb.2));
}
fn push_fg(rgb: Col, st: &mut OsString) {
    st.push(&format!("\x1B[38;2;{};{};{}m", rgb.0, rgb.1, rgb.2));
}

fn find_exexcutables<const N: usize>(
    names: [&str; N],
    home_dir: &OsStr,
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

fn find_path_paths(path: &str, home_dir: &OsStr) -> Vec<PathBuf> {
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

fn print_programs<const N: usize>(st: &mut OsString, found: &[bool; N], fns: &[&str; N]) {
    for f in found
        .iter()
        .zip(fns.iter())
        .filter_map(|(f, c)| f.then_some(c))
    {
        st.push(f);
    }
}

fn print_exes(st: &mut OsString, home_dir: &OsStr) -> Option<()> {
    let programs_exec_names = ["gcc", "rustc", "java", "python", "node"];
    let pro_name = [" ", "󱘗 ", " ", " ", " "];
    let (found_in_sys, found_in_shell, in_nix_shell) =
        find_exexcutables(programs_exec_names, home_dir)?;

    transition(grey, st);
    transition(black, st);

    st.push(" ");
    push_fg(yellow, st);
    print_programs(st, &found_in_sys, &pro_name);

    if in_nix_shell {
        push_fg(blue, st);
        st.push("");
        push_fg(black, st);
        push_bg(blue, st);
        st.push("");
        push_bg(black, st);
        push_fg(blue, st);
        st.push(" ");
        push_fg(orange, st);
        st.push(">>=");
        push_fg(blue, st);
        // transition(blue, st);
        // transition(black, st);

        st.push(" ");
        print_programs(st, &found_in_shell, &pro_name);
        transition(blue, st);
        transition(black, st);
    }

    Some(())
}
