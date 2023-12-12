#[macro_export]
macro_rules! assert_questionset_eq {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        assert!(
            compare_question_sets(&left, &right),
            "Questions not equal: {:?} != {:?}",
            left,
            right
        );
    }};
}
