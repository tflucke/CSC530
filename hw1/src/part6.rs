pub fn find_unique<T>(list: &[Option<T>], needle: &T) -> Result<usize, Vec<usize>>
where T: Eq {
    let res: Vec<usize> = list.iter().enumerate().filter(|x| match x.1 {
        Some(y) => y == needle,
        None => false,
    }).map(|p| p.0).collect();
    match res[..] {
        [i] => Ok(i),
        _   => Err(res)
    }
}
