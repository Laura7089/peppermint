#[test]
fn parse_sample_file() {
    let sample_string = std::fs::read_to_string("../sample_program.sasm").unwrap();
    println!("{:?}", peppermint::parse_final(&sample_string));
}
