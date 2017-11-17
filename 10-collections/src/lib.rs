#![allow(dead_code)]

#[cfg(test)]
mod insert_at_right_place_should {
    use super::*;

    #[test]
    fn insert_42_at_first_place_when_empty() {
        let mut vec = vec![];
        insert_at_right_place(&mut vec, 42);
        assert_eq!(vec![42], vec);
    }

    #[test]
    fn insert_13_at_first_place_when_contains_42() {
        let mut vec = vec![42];
        insert_at_right_place(&mut vec, 13);
        assert_eq!(vec![13, 42], vec);
    }

    #[test]
    fn insert_42_at_last_place_when_contains_13() {
        let mut vec = vec![13];
        insert_at_right_place(&mut vec, 42);
        assert_eq!(vec![13, 42], vec);
    }

    #[test]
    fn not_insert_42_when_contains_42() {
        let mut vec = vec![42];
        insert_at_right_place(&mut vec, 42);
        assert_eq!(vec![42], vec);
    }

    #[test]
    fn insert_42_at_2_when_13_21_314_1337() {
        let mut vec = vec![13, 21, 314, 1337];
        insert_at_right_place(&mut vec, 42);
        assert_eq!(vec![13, 21, 42, 314, 1337], vec);
    }
}

#[cfg(test)]
mod followed_by_sum_should {
    use super::followed_by_sum;

    #[test]
    fn returns_empty_when_empty() {
        assert_eq!(Vec::<(u64, u64)>::new(), followed_by_sum(vec![]));
    }

    #[test]
    fn returns_empty_when_contains_1() {
        assert_eq!(Vec::<(u64, u64)>::new(), followed_by_sum(vec![1]));
    }

    #[test]
    fn returns_empty_when_contains_1_2() {
        assert_eq!(Vec::<(u64, u64)>::new(), followed_by_sum(vec![1, 2]));
    }

    #[test]
    fn returns_1n2_when_contains_1_2_3() {
        assert_eq!(vec![(1, 2)], followed_by_sum(vec![1, 2, 3]));
    }

    #[test]
    fn returns_empty_when_contains_1_2_2() {
        assert_eq!(Vec::<(u64, u64)>::new(), followed_by_sum(vec![1, 2, 2]));
    }

    #[test]
    fn returns_4n5_when_contains_4_5_9() {
        assert_eq!(vec![(4, 5)], followed_by_sum(vec![4, 5, 9]));
    }

    #[test]
    fn returns_4n5_when_contains_2_4_5_9() {
        assert_eq!(vec![(4, 5)], followed_by_sum(vec![2, 4, 5, 9]));
    }

    #[test]
    fn returns_4n5_when_contains_4_5_9_10() {
        assert_eq!(vec![(4, 5)], followed_by_sum(vec![4, 5, 9, 10]));
    }

    #[test]
    fn returns_1n4_4n5_when_contains_1_4_5_9() {
        assert_eq!(vec![(1, 4),(4, 5)], followed_by_sum(vec![1, 4, 5, 9]));
    }
}
