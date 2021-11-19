#[cfg(test)]
mod part4_tests {
   use hw1::part4::find_unique;

   #[test]
   fn check_find1() {
      assert_eq!(find_unique(
            &vec![Some(1),None,Some(3),Some(4),None,Some(6)],
            &3),
         Ok(2));
   }

   #[test]
   fn check_find2() {
      assert_eq!(find_unique(
            &vec![Some("one"), Some("fish"), Some("two"), Some("fish"),
               Some("red"), Some("fish"), Some("blue"), Some("fish")],
            &"fish"),
         Err(vec![1,3,5,7]));
   }
}

