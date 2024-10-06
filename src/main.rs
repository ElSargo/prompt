#![allow(non_upper_case_globals)]
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

fn c(st: &mut String) {
    push_fg(blue, st);
    st.push_str(" ");
    unsafe { push_fg(FG, st) };
}
fn rust(st: &mut String) {
    push_fg(orange, st);
    st.push_str("󱘗 ");
    unsafe { push_fg(FG, st) };
}
fn nix(st: &mut String) {
    push_fg(blue, st);
    st.push_str(" ");
    unsafe { push_fg(FG, st) };
}
fn java(st: &mut String) {
    push_fg(white, st);
    st.push_str(" ");
    unsafe { push_fg(FG, st) };
}

fn main() {
    let mut out = String::with_capacity(200);

    begin(orange, black, &mut out);
    out.push_str(" Main");
    // transition(black, &mut out);
    transition(grey, &mut out);
    transition(black, &mut out);
    // transition(green, &mut out);
    // transition(grey, &mut out);
    // set_fg(yellow, &mut out);
    // transition(yellow, &mut out);
    out.push_str(" ");
    c(&mut out);
    rust(&mut out);
    nix(&mut out);
    java(&mut out);
    transition(grey, &mut out);
    transition(black, &mut out);
    transition(grey, &mut out);
    transition(green, &mut out);
    out.push_str("Well");
    transition(grey, &mut out);
    transition(orange, &mut out);

    // transition([0; 3], &mut out);
    // out.push_str("Well");
    // transition([255; 3], &mut out);
    // out.push_str("Well");

    //    out.push_str(
    //        "\
    //  Main
    //    ",
    // println!("\x1B[38;2;256;0;0;mError\x1B[0m");

    println!("{out} {}", colored((230, 0, 150), "Nig"));
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

// https://stackoverflow.com/a/78372226
fn colored(rgb: (i32, i32, i32), text: &str) -> String {
    return format!("\x1B[48;2;{};{};{}m{}\x1B[0m", rgb.0, rgb.1, rgb.2, text);
}
