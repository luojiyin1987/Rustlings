// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(3);
}

pub fn  call_me(num: u32) {
    for i in 1..num {
        println!("Ring! Call number {}", i);
    }
}