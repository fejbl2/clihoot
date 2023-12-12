use std::path::Path;

use common::questions;

use std::net::TcpListener;

#[must_use]
pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("0.0.0.0", port)).is_ok()
}

#[must_use]
pub fn sample_questions() -> questions::QuestionSet {
    questions::QuestionSet::from_file(Path::new("../common/tests/files/ok_minimal.yaml")).unwrap()
}
