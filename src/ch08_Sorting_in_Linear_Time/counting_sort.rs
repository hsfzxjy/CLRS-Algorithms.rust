pub fn counting_sort(A: &mut [i32], lower: i32, upper: i32) {
    assert!(lower <= upper);

    let n_slots = (upper - lower + 1) as usize;
    let mut slots: Vec<usize> = vec![0; n_slots];
    for x in A.iter() {
        slots[(x - lower) as usize] += 1;
    }
    let mut i = 0;
    for slot in 0..n_slots {
        if slots[slot] == 0 {
            continue;
        }
        for _ in 0..slots[slot] {
            A[i] = slot as i32 + lower;
            i += 1;
        }
    }
}
