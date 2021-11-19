#[cfg(test)]
mod part2b_tests {
   use hw1::part2b::unzip_mixed_vec;

   #[test]
   fn check_unzip_mixed_vec1() {
      let tuples = vec![
         (1,String::from("first")),
         (3,String::from("middle")),
         (5,String::from("last"))];
      assert_eq!(unzip_mixed_vec(tuples),
         (vec![1,3,5],vec![
               String::from("first"),
               String::from("middle"),
               String::from("last")]));
   }
}

