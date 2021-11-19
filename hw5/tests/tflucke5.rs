#[cfg(test)]
mod tflucke_4_tests {
    use hw2::arith::build::*;
    use hw2::arith::ty::Type::*;
    use hw2::typecheck::*;
    use std::rc::Rc;

   #[test]
   fn polymorphic_fn() {
       assert_eq!(typecheck(&polylam_e("T", unit_e()), &vec![]),
                  Ok(TypeFn {param: "T", ret: Rc::new(Unit)})
       );
   }

   #[test]
   fn polymorphic_app() {
       assert_eq!(typecheck(&polyapp_e(polylam_e("T", unit_e()), Int), &vec![]),
                  Ok(Unit)
       );
   }

   #[test]
   fn polymorphic_var() {
       assert_eq!(typecheck(
           &polylam_e("T", lam_e("x", Polymorphic("T"), var_e("x"))), &vec![]),
                  Ok(TypeFn {param: "T", ret: Rc::new(Fn {
                      param: Rc::new(Polymorphic("T")),
                      ret: Rc::new(Polymorphic("T"))
                  })})
       );
   }

   #[test]
   fn polymorphic_non_polymorphic_fn() {
       assert!(typecheck(
           &polyapp_e(lam_e("x", Polymorphic("T"), var_e("x")), Int),
           &vec![]).is_err()
       );
   }

   #[test]
   fn polymorphic_var_subsitution() {
       assert_eq!(typecheck(
           &polyapp_e(
               polylam_e("T", lam_e("x", Polymorphic("T"), var_e("x"))), Int),
           &vec![]),
                  Ok(Fn {
                      param: Rc::new(Int),
                      ret: Rc::new(Int)
                  })
       );
   }
    
   #[test]
   fn polymorphic_shadowing() {
       assert_eq!(typecheck(
           &polyapp_e(
               polyapp_e(
                   polylam_e(
                       "T", polylam_e(
                           "T", lam_e("x", Polymorphic("T"), var_e("x"))
                       )
                   ), Int),
               Bool),
           &vec![]),
                  Ok(Fn {
                      param: Rc::new(Bool),
                      ret: Rc::new(Bool)
                  })
       );
   }
    
   #[test]
   fn multiple_polymorphic_adhoc() {
       assert!(typecheck(
           &polyapp_e(
               polyapp_e(
                   polylam_e(
                       "T", polylam_e(
                           "U", lam_e("x", Polymorphic("T"),
                                      lam_e("y", Polymorphic("U"),
                                            if_e(var_e("y"),
                                                 var_e("x"),
                                                 num_e(0))))
                       )
                   ), Int),
               Bool),
           &vec![]).is_err()
       );
   }

   #[test]
   fn keen_example() {
       assert_eq!(typecheck(
           &polyapp_e(
                   polylam_e(
                       "T", lam_e("f", Fn {
                           param: Rc::new(Polymorphic("T")),
                           ret: Rc::new(Bool)
                       },
                                  lam_e("x", Polymorphic("T"),
                                        if_e(app_e(var_e("f"), var_e("x")),
                                             num_e(1),
                                             num_e(0))))
                   ),
               Int),
           &vec![]),
                  Ok(Fn {
                      param: Rc::new(Fn {
                          param: Rc::new(Int),
                          ret: Rc::new(Bool)
                      }),
                      ret: Rc::new(Fn {
                          param: Rc::new(Int),
                          ret: Rc::new(Int)
                      })
                  })
       );
   }

    #[test]
    fn multiple_polymorphic() {
        assert_eq!(typecheck(
            &polyapp_e(
                polyapp_e(
                    polylam_e(
                        "T", polylam_e(
                            "U", lam_e(
                                "f", Fn {
                                    param: Rc::new(Polymorphic("T")),
                                    ret: Rc::new(Unit)
                                }, lam_e(
                                    "g", Fn {
                                        param: Rc::new(Polymorphic("U")),
                                        ret: Rc::new(Bool)
                                    }, lam_e("x", Polymorphic("T"),
                                             lam_e("y", Polymorphic("U"),
                                                   if_e(app_e(var_e("g"),
                                                              var_e("y")),
                                                        app_e(var_e("f"),
                                                              var_e("x")),
                                                        unit_e())))))
                        )
                    ),
                    Bool),
                Int),
            &vec![]),
                   Ok(Fn {
                       param: Rc::new(Fn {
                           param: Rc::new(Bool),
                           ret: Rc::new(Unit)
                       }),
                       ret: Rc::new(Fn {
                           param: Rc::new(Fn {
                               param: Rc::new(Int),
                               ret: Rc::new(Bool)
                           }),
                           ret: Rc::new(Fn {
                               param: Rc::new(Bool),
                               ret: Rc::new(Fn {
                                   param: Rc::new(Int),
                                   ret: Rc::new(Unit)
                               })
                           })
                       })
                   })
        );
    }    

}
