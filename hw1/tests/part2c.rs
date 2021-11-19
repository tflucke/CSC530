#[cfg(test)]
mod part2c_tests {
   use hw1::part2c::unzip_mixed_slice;

   #[test]
   fn check_unzip_mixed_slice1() {
      let tuples = vec![
         (1,String::from("first")),
         (3,String::from("middle")),
         (5,String::from("last"))];
      assert_eq!(unzip_mixed_slice(&tuples[1..]),
         (vec![3,5],vec![
               &String::from("middle"),
               &String::from("last")]));
   }
}

