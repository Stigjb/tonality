use num_traits::FromPrimitive;
use proptest::prelude::*;
use tonality::{Interval, Key, Step, Tpc};

fn tpcs() -> BoxedStrategy<Tpc> {
    let min = Tpc::MIN as i8;
    let max = Tpc::MAX as i8;
    (min..=max)
        .prop_map(|v| FromPrimitive::from_i8(v).unwrap())
        .boxed()
}

fn keys() -> BoxedStrategy<Key> {
    let min = Key::MIN as i8;
    let max = Key::MAX as i8;
    (min..=max)
        .prop_map(|v| FromPrimitive::from_i8(v).unwrap())
        .boxed()
}

fn steps() -> BoxedStrategy<Step> {
    let min = Step::MIN as i8;
    let max = Step::MAX as i8;
    (min..=max)
        .prop_map(|v| FromPrimitive::from_i8(v).unwrap())
        .boxed()
}

fn intervals() -> BoxedStrategy<Interval> {
    let min = Interval::MIN as i8;
    let max = Interval::MAX as i8;
    (min..=max)
        .prop_map(|v| FromPrimitive::from_i8(v).unwrap())
        .boxed()
}

proptest! {
    #[test]
    fn prop_alter_keeps_step(tpc in tpcs(), alter in -3..=3_i8) {
        if let Some(altered) = tpc.alter(alter) {
            assert_eq!(tpc.step(), altered.step())
        }
    }
}

proptest! {
    #[test]
    fn prop_adding_key_keeps_step(step in steps(), key in keys()) {
        assert_eq!(step, step.with_key(key).step())
    }
}

proptest! {
    #[test]
    fn tpc_interval_interval_associative(tpc in tpcs(), i1 in intervals(), i2 in intervals()) {
        // One branch can fail while the other succeeds
        let res1 = (i1 + i2).map(|i| tpc + i);
        let res2 = (tpc + i1).map(|t| t + i2);
        if let (Some(a), Some(b)) = (res1, res2) {
            assert_eq!(a, b);
        }
    }
}

proptest! {
    #[test]
    fn steps_accidentals_can_recompose(tpc in tpcs(), key in keys()) {
        let (step, acc) = tpc.altered_step(Some(key));
        let reconstructed = match acc {
            None => step.with_key(key),
            Some(acc) => step.with_accidental(acc),
        };
        assert_eq!(tpc, reconstructed);
    }
}

proptest! {
    #[test]
    fn first_scale_degree_is_root(key in keys()) {
        assert_eq!(key.root(), key.scale_degree(0));
    }
}
