use std::rc::Rc;

use super::expr::Expression;
use super::ty::Type;

pub fn true_e() -> Rc<Expression> {
   Rc::new(Expression::True)
}

pub fn false_e() -> Rc<Expression> {
   Rc::new(Expression::False)
}

pub fn var_e(id: &'static str) -> Rc<Expression> {
   Rc::new(Expression::Var(id))
}

pub fn num_e(n: i32) -> Rc<Expression> {
   Rc::new(Expression::Num(n))
}

pub fn if_e(guard: Rc<Expression>, then_: Rc<Expression>, else_: Rc<Expression>)
   -> Rc<Expression> {
   Rc::new(Expression::If {guard, then_, else_,})
}

pub fn add_e(left: Rc<Expression>, right: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::Add {left, right})
}

pub fn sub_e(left: Rc<Expression>, right: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::Sub {left, right})
}

pub fn is_zero_e(exp: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::IsZero(exp))
}

pub fn lam_e(id: &'static str, ty: Type, body: Rc<Expression>) -> Rc<Expression> {
    Rc::new(Expression::Lambda {param: id, param_type: ty, body: body})
}

pub fn app_e(f: Rc<Expression>, arg: Rc<Expression>) -> Rc<Expression> {
    Rc::new(Expression::App{f: f, arg: arg})
}

#[cfg(test)]
mod tests {
   use super::{*,Expression::*};

   #[test]
   fn num_eq() {
      assert_eq!(*num_e(0), Num(0));
   }

   #[test]
   fn true_eq() {
      assert_eq!(*true_e(), True);
   }

   #[test]
   fn false_eq() {
      assert_eq!(*false_e(), False);
   }

   #[test]
   fn if_eq() {
      assert_eq!(
         *if_e(true_e(), num_e(0), num_e(1)),
         If {guard: true_e(), then_: num_e(0), else_: num_e(1)});
   }

   #[test]
   fn add_eq() {
      assert_eq!(
         *add_e(num_e(9), num_e(4)),
         Add {left: num_e(9), right: num_e(4)});
   }

   #[test]
   fn sub_eq() {
      assert_eq!(
         *sub_e(num_e(9), num_e(4)),
         Sub {left: num_e(9), right: num_e(4)});
   }

   #[test]
   fn is_zero_eq() {
      assert_eq!(
         *is_zero_e(num_e(4)),
         IsZero(num_e(4)));
   }
}
