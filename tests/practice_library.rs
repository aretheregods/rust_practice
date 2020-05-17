mod tests {
    use practice::practice_library;

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = practice_library::Rectangle {
            height: 100,
            width: 200,
        };

        let smaller = practice_library::Rectangle {
            height: 99,
            width: 199,
        };

        assert_ne!(true, smaller.absolutely_larger(&larger))
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = practice_library::Rectangle {
            height: 100,
            width: 200,
        };

        let smaller = practice_library::Rectangle {
            height: 99,
            width: 199,
        };

        assert_eq!(true, larger.absolutely_larger(&smaller));
    }

    #[test]
    #[should_panic]
    fn guess_panics() {
        practice_library::Guess::new(150);
    }
}