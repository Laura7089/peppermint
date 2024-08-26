#[test]
fn parse_sample_file() {
    let sample_string = std::fs::read_to_string("../sample_program.ppr").unwrap();
    println!(
        "{:?}",
        peppermint::Program::parse_source(&sample_string).expect("parse error")
    );
}
