pub fn unzip_mixed_vec(vec: Vec<(i32, String)>) -> (Vec<i32>, Vec<String>) {
    let mut left = vec![];
    let mut right = vec![];
    for p in vec {
        left.push(p.0);
        right.push(p.1);
    }
    (left, right)
}
