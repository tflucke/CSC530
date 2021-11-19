#[cfg(test)]
mod part1_tests {
   use hw1::part1::double;

   #[test]
   fn check1() {
      assert_eq!(double(-9), -18);
   }
}

