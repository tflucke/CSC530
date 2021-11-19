pub fn unzip_ints(vec: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];
    for p in vec {
        left.push(p.0);
        right.push(p.1);
    }
    (left, right)
}
