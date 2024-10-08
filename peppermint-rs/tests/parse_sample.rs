#[test]
fn parse_sample_file() {
    let sample_string = std::fs::read_to_string("../sample_program.ppr").unwrap();
    let parsed = peppermint::Program::parse_source(&sample_string)
        .map_err(|e| e.spans_to_source(&sample_string))
        .expect("parse error");
    println!("{parsed:?}",);
}
