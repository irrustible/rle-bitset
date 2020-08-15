#![feature(test)]

use rle_bitset::*;

#[test]
fn empty() {
    let mut x: [usize; 4] = [0, 0, 0, 0];
    let over = WORD_WIDTH * 4;
    for i in 0..over {
        assert_eq!(x.get_bit(i).unwrap(), false);
    }
    assert!(x.get_bit(over).is_err());
    x.set_bit(WORD_WIDTH, false).unwrap();
    for i in 0..over {
        assert_eq!(x.get_bit(i).unwrap(), false);
    }
    {
        let mut iter = x.run_lengths(..).unwrap();
        assert_eq!(Some(RL::new(false, 0, over)), iter.next());
        assert_eq!(None, iter.next());
    }
    assert!(x.get_bit(over).is_err());
}

#[test]
fn one_two() {
    let mut x: [usize; 4] = [0, 0, 0, 0];
    let over = WORD_WIDTH * 4;
    x.set_bit(WORD_WIDTH, true).unwrap();
    assert_eq!(x.get_bit(WORD_WIDTH).unwrap(), true);
    {
        let mut iter = x.run_lengths(..).unwrap();
        assert_eq!(Some(RL::new(false, 0, WORD_WIDTH)), iter.next());
        assert_eq!(Some(RL::new(true, WORD_WIDTH, WORD_WIDTH + 1)), iter.next());
        assert_eq!(Some(RL::new(false, WORD_WIDTH + 1, over)), iter.next());
        assert_eq!(None, iter.next());
    }
    x.set_bit(WORD_WIDTH - 1, true).unwrap();
    assert_eq!(x.get_bit(WORD_WIDTH - 1).unwrap(), true);
    {
        let mut iter = x.run_lengths(..).unwrap();
        assert_eq!(Some(RL::new(false, 0, WORD_WIDTH - 1)), iter.next());
        assert_eq!(Some(RL::new(true, WORD_WIDTH -1, WORD_WIDTH + 1)), iter.next());
        assert_eq!(Some(RL::new(false, WORD_WIDTH + 1, over)), iter.next());
        assert_eq!(None, iter.next());
    }
}
