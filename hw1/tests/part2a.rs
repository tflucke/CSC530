#[cfg(test)]
mod part2a_tests {
   use hw1::part2a::unzip_ints;

   #[test]
   fn check_unzip_ints1() {
      let tuples = vec![(1,2),(3,4),(5,6)];
      assert_eq!(unzip_ints(&tuples), (vec![1,3,5], vec![2,4,6]));
   }

   #[test]
   fn check_unzip_ints2() {
      let tuples = vec![(1,2),(3,4),(5,6)];
      assert_eq!(unzip_ints(&tuples[1..]), (vec![3,5], vec![4,6]));
   }
}

