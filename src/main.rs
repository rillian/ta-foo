extern crate test_assembler;
use test_assembler::{Section, Label, LabelMaker};

fn main() {
    let size = Label::new();
    let section = Section::new()
        .B32(&size)
        .append_bytes(b"test");
    size.set_const(section.size());

    let result = section.get_contents();
    assert!(result.is_some());

    println!("{:?}", result.unwrap());
}
