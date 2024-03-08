mod error;
mod format_spec;
mod macros;
mod placeholder;
mod template;
mod utils;

pub use error::{Error, ErrorKind};
pub use template::Template;


#[test]
pub fn test_simple_3() {
    let template = "AA{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}BB";
    let text =formatx!(template, 24,3,6,16,40,23);
    println!("{}", text.unwrap());
}