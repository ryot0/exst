
//!
//! 組み込みワードを定義するための補助関数
//! 

use super::super::lang::vm::*;
use super::super::lang::tokenizer::*;
use super::super::lang::value::*;
use std::rc::Rc;

/// ( int int -- int )のワードの呼び出し
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

/// 次のsymbol tokenを使用するワードの呼び出し
pub fn call_with_name<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(&mut V, &String) -> Result<(),VmErrorReason<E>>
{
    match vm.input_stream_mut().next_token() {
        Option::Some(t) => {
            let token = t.value_token?;
            match token {
                ValueToken::Symbol(name) => {
                    f(vm, &name)
                },
                _ => {
                    Result::Err(VmErrorReason::TypeMismatchError("Symbol", "UNKNOWN"))
                }
            }
        },
        Option::None => {
            Result::Err(VmErrorReason::TypeMismatchError("Symbol", "EMPTY"))
        },
    }
}