extern crate test_assembler;
use test_assembler::{Section, Label, LabelMaker};

fn make() -> Section {
    let size = Label::new();
    Section::new()
        .B32(&size)
        .append_bytes(b"test")
        .append_repeated(0xab, 8)
        .mark(&size)
}

fn main() {
    let section = make();
    println!("{:?}", section.get_contents().unwrap());
}
