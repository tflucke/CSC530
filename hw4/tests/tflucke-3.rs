#[cfg(test)]
mod pbs2_q3_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;

   #[test]
   fn missing_case() {
       assert!(typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", num_e(0)),
           ("u", "x", num_e(-1)),
       ])), &vec![]).is_err());
   }

   #[test]
   fn extra_case() {
       assert!(typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", num_e(0)),
           ("u", "x", num_e(-1)),
           ("i", "n", var_e("n")),
           ("j", "v", var_e("v")),
       ])), &vec![]).is_err());
   }

   #[test]
   fn mislabeled_case() {
       assert!(typecheck(&lam_e("a", variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ]), case_e(var_e("a"), vec![
           ("b", "y", num_e(0)),
           ("u", "x", num_e(-1)),
           ("j", "n", var_e("n")),
       ])), &vec![]).is_err());
   }

   #[test]
   fn non_variant_case() {
       assert!(typecheck(&case_e(var_e("a"), vec![
           ("b", "y", num_e(0)),
           ("u", "x", num_e(-1)),
           ("j", "n", var_e("n")),
       ]), &vec![("a", Int)]).is_err());
   }

   #[test]
   fn no_type_case() {
       assert!(typecheck(&case_e(var_e("a"), vec![
           ("b", "y", num_e(0)),
           ("u", "x", num_e(-1)),
           ("j", "n", var_e("n")),
       ]), &vec![]).is_err());
   }
    
   #[test]
   fn variant_wrong_type() {
       assert!(typecheck(&variant_e("i", true_e(), variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ])), &vec![]).is_err());
   }
    
   #[test]
   fn variant_bad_label() {
       assert!(typecheck(&variant_e("t", true_e(), variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ])), &vec![]).is_err());
   }
    
   #[test]
   fn variant_mismatched_type() {
       assert!(typecheck(&variant_e("i", true_e(), variant_t(vec![
           ("b", Bool),
           ("i", Int),
           ("u", Unit)
       ])), &vec![]).is_err());
   }
}
