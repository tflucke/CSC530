#[cfg(test)]
mod pbs3_q1_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::{Type,Type::*};
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
    fn part1() {
        let b_data = Record(vec![("B", Bool), ("Data", Int)]);
        let c_data = Record(vec![("C", Int), ("Data", Int)]);
        let a = variant_t(vec![
            ("B", Record(vec![("f", Fn {
                param: Rc::new(Type::clone(&b_data)),
                ret: Rc::new(Int)
            }), ("data", b_data)])),
            ("C", Record(vec![("f", Fn {
                param: Rc::new(Type::clone(&c_data)),
                ret: Rc::new(Int)
            }), ("data", c_data)])),
        ]);
        assert_eq!(typecheck(&lam_e(
            "a", Type::clone(&a),
            case_e(var_e("a"), vec![("B", "b", app_e(
                project_e(var_e("b"), "f"),
                project_e(var_e("b"), "data")
            )), ("C", "c", app_e(
                project_e(var_e("c"), "f"),
                project_e(var_e("c"), "data")
            ))])
        ), &vec![]), Ok(Fn {
            param: Rc::new(a),
            ret: Rc::new(Int)
        }));
    }

    #[test]
    fn part1_apped() {
        let b_data = Record(vec![("B", Bool), ("Data", Int)]);
        let c_data = Record(vec![("C", Int), ("Data", Int)]);
        let a = variant_t(vec![
            ("B", Record(vec![("f", Fn {
                param: Rc::new(Type::clone(&b_data)),
                ret: Rc::new(Int)
            }), ("data", Type::clone(&b_data))])),
            ("C", Record(vec![("f", Fn {
                param: Rc::new(Type::clone(&c_data)),
                ret: Rc::new(Int)
            }), ("data", c_data)])),
        ]);
        assert_eq!(typecheck(&app_e(lam_e(
            "a", Type::clone(&a),
            case_e(var_e("a"), vec![("B", "b", app_e(
                project_e(var_e("b"), "f"),
                project_e(var_e("b"), "data")
            )), ("C", "c", app_e(
                project_e(var_e("c"), "f"),
                project_e(var_e("c"), "data")
            ))])
        ), variant_e("B", record_e(vec![
            ("f", lam_e("x", b_data, num_e(-4))),
            ("data", record_e(vec![
                ("B", true_e()),
                ("Data", num_e(7))
            ]))
        ]), a)), &vec![]), Ok(Int));
    }

    #[test]
    fn part1_apped_mismatched_type() {
        let b_data = Record(vec![("B", Bool), ("Data", Int)]);
        let c_data = Record(vec![("C", Int), ("Data", Int)]);
        let a = variant_t(vec![
            ("B", Record(vec![("f", Fn {
                param: Rc::new(Type::clone(&b_data)),
                ret: Rc::new(Int)
            }), ("data", b_data)])),
            ("C", Record(vec![("f", Fn {
                param: Rc::new(Type::clone(&c_data)),
                ret: Rc::new(Int)
            }), ("data", c_data)])),
        ]);
        assert!(typecheck(&app_e(lam_e(
            "a", Type::clone(&a),
            case_e(var_e("a"), vec![("B", "b", app_e(
                project_e(var_e("b"), "f"),
                project_e(var_e("b"), "data")
            )), ("C", "c", app_e(
                project_e(var_e("c"), "f"),
                project_e(var_e("c"), "data")
            ))])
        ), variant_e("B", record_e(vec![
            ("f", lam_e("x", Type::clone(&a), num_e(-4))),
            ("data", record_e(vec![
                ("B", true_e()),
                ("Data", num_e(7))
            ]))
        ]), a)), &vec![]).is_err());
    }
}
