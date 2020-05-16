
use super::super::lang::vm::*;
use super::super::lang::value::*;
use std::rc::Rc;

pub fn call_iifi<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(i32,i32) -> i32
{
    let lhs = vm.data_stack_mut().pop()?;
    match *lhs {
        Value::IntValue(lhs) => {
            let rhs = vm.data_stack_mut().pop()?;
            match *rhs {
                Value::IntValue(rhs) => {
                    vm.data_stack_mut().push(Rc::new(Value::IntValue(f(lhs, rhs))));
                    Result::Ok(())
                },
                _ => { 
                    Result::Err(VmErrorReason::TypeMismatchError(int_type_name(), rhs.type_name()))
                 }
            }
        },
        _ => {
            Result::Err(VmErrorReason::TypeMismatchError(int_type_name(), lhs.type_name()))
        }
    }
}