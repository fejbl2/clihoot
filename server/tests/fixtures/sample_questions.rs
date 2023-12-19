use common::questions::QuestionSet;
use rstest::fixture;

use crate::utils::sample_questions as _sample_questions;

#[must_use]
#[fixture]
pub fn sample_questions() -> QuestionSet {
    _sample_questions()
}
