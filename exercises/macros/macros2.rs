// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)


#[macro_export]
macro_rules! my_macro2 {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro2!();
}
