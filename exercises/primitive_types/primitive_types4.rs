// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!



#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    // slices from start_index (0 first) to end_index ( one more than last position)
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
