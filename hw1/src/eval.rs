use std::rc::Rc;

use super::arith::expr::{Expression,Expression::*};
use super::arith::build::*;

pub fn eval(exp: Rc<Expression>) -> Result<Rc<Expression>,Rc<Expression>> {
   let result = single_step(exp.clone());

   if is_num(&result) || is_bool(&result) {     // a value?
      Ok(result)
   } else if result == exp {                    // stuck, not a value?
      Err(result)
   } else {
      eval(result)
   }
}

// public for external testing/grading
pub fn single_step(exp: Rc<Expression>) -> Rc<Expression> {
   match &*exp {
      True | False | Num(_) => exp,
      Add {left: n, right: m} => add(n, m),
      Sub {left: n, right: m} => sub(n, m),
      IsZero(n) => is_zero(n),
      If {guard: g, then_: t, else_: e} => eval_if(g, t, e),
   }
}

fn add(left: &Rc<Expression>, right: &Rc<Expression>) -> Rc<Expression> {
   match (&**left, &**right) {      // borrow to prevent move into tuple
      (Num(n), Num(m)) => num_e(n + m),
      (Num(_), _) => add_e(Rc::clone(left), single_step(Rc::clone(right))),
      _ => add_e(single_step(Rc::clone(left)), Rc::clone(right)),
   }
}

fn sub(left: &Rc<Expression>, right: &Rc<Expression>) -> Rc<Expression> {
   match (&**left, &**right) {      // borrow to prevent move into tuple
      (Num(n), Num(m)) => num_e(n - m),
      (Num(_), _) => sub_e(Rc::clone(left), single_step(Rc::clone(right))),
      _ => sub_e(single_step(Rc::clone(left)), Rc::clone(right)),
   }
}

fn eval_if(guard: &Rc<Expression>, then_: &Rc<Expression>, else_: &Rc<Expression>)
           -> Rc<Expression> {
   match &**guard {
      Expression::True => Rc::clone(then_),
      Expression::False => Rc::clone(else_),
      _ => if_e(single_step(Rc::clone(guard)), Rc::clone(then_), Rc::clone(else_)),
   }
}

fn is_zero(exp: &Rc<Expression>) -> Rc<Expression> {
   match **exp {
      Num(0) => true_e(),
      Num(_) => false_e(),
      _ => is_zero_e(single_step(Rc::clone(exp))),
   }
}

fn is_num(exp: &Rc<Expression>) -> bool {
   match **exp {
      Num(_) => true,
      _ => false,
   }
}

fn is_bool(exp: &Rc<Expression>) -> bool {
   match **exp {
      True | False => true,
      _ => false,
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn add1() {
      assert_eq!(add(&num_e(1), &num_e(2)), num_e(3));
   }

   #[test]
   fn add2() {
      assert_eq!(add(&num_e(3), &add_e(num_e(1), num_e(2))),
         add_e(num_e(3), num_e(3)));
   }

   #[test]
   fn add3() {
      assert_eq!(add(&add_e(num_e(9), num_e(10)), &add_e(num_e(1), num_e(2))),
         add_e(num_e(19), add_e(num_e(1), num_e(2))));
   }

   #[test]
   fn sub1() {
      assert_eq!(sub(&num_e(9), &num_e(2)), num_e(7));
   }

   #[test]
   fn sub2() {
      assert_eq!(sub(&num_e(3), &sub_e(num_e(1), num_e(2))),
         sub_e(num_e(3), num_e(-1)));
   }

   #[test]
   fn sub3() {
      assert_eq!(sub(&sub_e(num_e(9), num_e(10)), &sub_e(num_e(1), num_e(2))),
         sub_e(num_e(-1), sub_e(num_e(1), num_e(2))));
   }

   #[test]
   fn iszero1() {
      assert_eq!(is_zero(&num_e(1)), false_e());
   }

   #[test]
   fn iszero2() {
      assert_eq!(is_zero(&num_e(0)), true_e());
   }

   #[test]
   fn iszero3() {
      assert_eq!(is_zero(&sub_e(num_e(1), num_e(1))), is_zero_e(num_e(0)));
   }

   #[test]
   fn if1() {
       assert_eq!(eval_if(&is_zero_e(num_e(7)),
                          &add_e(num_e(7), num_e(2)),
                          &sub_e(num_e(7), num_e(2))),
                  if_e(false_e(),
                       add_e(num_e(7), num_e(2)),
                       sub_e(num_e(7), num_e(2))))
   }
}
