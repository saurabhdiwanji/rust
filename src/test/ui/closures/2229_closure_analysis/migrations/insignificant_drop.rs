#![deny(disjoint_capture_drop_reorder)]
//~^ NOTE: the lint level is defined here

// Test cases for types that implement a insignificant drop (stlib defined)

// `t` needs Drop because one of its elements needs drop,
// therefore precise capture might affect drop ordering
fn test1_all_need_migration() {
    let t = (String::new(), String::new());
    let t1 = (String::new(), String::new());
    let t2 = (String::new(), String::new());

    let c = || {
    //~^ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| NOTE: let (t, t1, t2) = (t, t1, t2);
        let _t = t.0;
        let _t1 = t1.0;
        let _t2 = t2.0;
    };

    c();
}

// String implements drop and therefore should be migrated.
// But in this test cases, `t2` is completely captured and when it is dropped won't be affected
fn test2_only_precise_paths_need_migration() {
    let t = (String::new(), String::new());
    let t1 = (String::new(), String::new());
    let t2 = (String::new(), String::new());

    let c = || {
    //~^ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| NOTE: let (t, t1) = (t, t1);
        let _t = t.0;
        let _t1 = t1.0;
        let _t2 = t2;
    };

    c();
}

// If a variable would've not been captured by value then it would've not been
// dropped with the closure and therefore doesn't need migration.
fn test3_only_by_value_need_migration() {
    let t = (String::new(), String::new());
    let t1 = (String::new(), String::new());
    let c = || {
    //~^ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| NOTE: let (t) = (t);
        let _t = t.0;
        println!("{}", t1.1);
    };

    c();
}

// Copy types get copied into the closure instead of move. Therefore we don't need to
// migrate then as their drop order isn't tied to the closure.
fn test4_only_non_copy_types_need_migration() {
    let t = (String::new(), String::new());

    // `t1` is Copy because all of its elements are Copy
    let t1 = (0i32, 0i32);

    let c = || {
    //~^ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| NOTE: let (t) = (t);
        let _t = t.0;
        let _t1 = t1.0;
    };

    c();
}

fn test5_only_drop_types_need_migration() {
    struct S(i32, i32);

    let t = (String::new(), String::new());

    // `s` doesn't implement Drop or any elements within it, and doesn't need migration
    let s = S(0i32, 0i32);

    let c = || {
    //~^ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| NOTE: let (t) = (t);
        let _t = t.0;
        let _s = s.0;
    };

    c();
}

// Since we are using a move closure here, both `t` and `t1` get moved
// even though they are being used by ref inside the closure.
fn test6_move_closures_non_copy_types_might_need_migration() {
    let t = (String::new(), String::new());
    let t1 = (String::new(), String::new());
    let c = move || {
    //~^ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| NOTE: let (t1, t) = (t1, t);
        println!("{} {}", t1.1, t.1);
    };

    c();
}

// Test migration analysis in case of Drop + Non Drop aggregates.
// Note we need migration here only because the non-copy (because Drop type) is captured,
// otherwise we won't need to, since we can get away with just by ref capture in that case.
fn test7_drop_non_drop_aggregate_need_migration() {
    let t = (String::new(), 0i32);

    let c = || {
    //~^ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| NOTE: let (t) = (t);
        let _t = t.0;
    };

    c();
}

fn main() {
    test1_all_need_migration();
    test2_only_precise_paths_need_migration();
    test3_only_by_value_need_migration();
    test4_only_non_copy_types_need_migration();
    test5_only_drop_types_need_migration();
    test6_move_closures_non_copy_types_might_need_migration();
    test7_drop_non_drop_aggregate_need_migration();
}
