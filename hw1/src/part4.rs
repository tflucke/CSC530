pub fn find_unique<T>(list: &[Option<T>], needle: &T) -> Result<usize, Vec<usize>>
where T: Eq {
    let mut res = vec![];
    for (i, val) in list.iter().enumerate() {
        match val {
            Some(x) if x == needle => res.push(i),
            _                      => ()
        }
    }
    match res[..] {
        [i] => Ok(i),
        _   => Err(res)
    }
}
