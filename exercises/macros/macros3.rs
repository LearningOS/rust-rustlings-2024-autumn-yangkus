// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // 公开宏以便在其他地方使用
    pub(crate) use my_macro;
}

fn main() {
    macros::my_macro!();
}
