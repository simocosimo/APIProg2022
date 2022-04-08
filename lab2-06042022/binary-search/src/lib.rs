//TODO: make this work for every kind of sortable type
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // Iterative way
    // let mut res: Option<usize> = None;
    // let mut curr_slice = &array[..];
    // let mut ret_index = 0;
    // while curr_slice.len() != 0 {
    //     let mid_index = curr_slice.len() / 2;
    //     let mid_item = *curr_slice.get(mid_index).unwrap();
    //
    //     let found = mid_item == key;
    //     let is_len_one = curr_slice.len() == 1;
    //     match (found, is_len_one) {
    //         (true, _) => { res = Some(ret_index + mid_index); break; },
    //         (false, false) => { },
    //         (false, true) => break
    //     }
    //
    //     if key < mid_item { curr_slice = curr_slice.split_at(mid_index).0; }
    //     if key > mid_item {
    //         ret_index += mid_index;
    //         curr_slice = curr_slice.split_at(mid_index).1;
    //     }
    // }
    // res

    // Recursive way
    if array.len() == 0 { return None; }

    let mut curr_slice = &array[..];
    let mid_index = curr_slice.len() / 2;
    let mid_item = *curr_slice.get(mid_index).unwrap();
    let mut ret_index = 0;
    let mut res = None;

    match (mid_item == key, curr_slice.len() == 1) {
        (true, _) => return Some(mid_index),
        (false, false) => { },
        (false, true) => return None
    }
    if key < mid_item { curr_slice = curr_slice.split_at(mid_index).0; }
    if key > mid_item {
        ret_index += mid_index;
        curr_slice = curr_slice.split_at(mid_index).1;
    }
    res = find(curr_slice, key);
    if res.is_some() { res = Some(res.unwrap() + ret_index); }
    res
}
