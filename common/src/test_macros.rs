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

#[macro_export]
macro_rules! assert_censored_question_eq {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        assert!(
            compare_censored_questions(&left, &right),
            "Censored questions not equal: {:?} != {:?}",
            left,
            right
        );
    }};
}
