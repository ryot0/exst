
//!
//! 組み込みワードを定義するための補助関数
//! 

use super::super::lang::vm::*;
use super::super::lang::tokenizer::*;
use super::super::lang::value::*;
use std::rc::Rc;

/// ( x x -- Result )のワードの呼び出し
fn call_xxfr<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(Rc<Value<V::ExtraValueType>>,Rc<Value<V::ExtraValueType>>) -> Result<Value<V::ExtraValueType>,VmErrorReason<E>>,
          E: std::fmt::Debug
{
    let rhs = vm.data_stack_mut().pop()?;
    let lhs = vm.data_stack_mut().pop()?;
    let v = f(lhs, rhs)?;
    vm.data_stack_mut().push(Rc::new(v));
    Result::Ok(())
}

/// ( x x -- x )のワードの呼び出し
pub fn call_xxfx<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(Rc<Value<V::ExtraValueType>>,Rc<Value<V::ExtraValueType>>) -> Value<V::ExtraValueType>,
          E: std::fmt::Debug
{
    let rhs = vm.data_stack_mut().pop()?;
    let lhs = vm.data_stack_mut().pop()?;
    vm.data_stack_mut().push(Rc::new(f(lhs, rhs)));
    Result::Ok(())
}

/// ( x -- x )のワードの呼び出し
pub fn call_xfx<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(Rc<Value<V::ExtraValueType>>) -> Value<V::ExtraValueType>,
          E: std::fmt::Debug
{
    let lhs = vm.data_stack_mut().pop()?;
    vm.data_stack_mut().push(Rc::new(f(lhs)));
    Result::Ok(())
}

/// ( int int -- int )のワードの呼び出し
pub fn call_iifi<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(i32,i32) -> i32,
          E: std::fmt::Debug
{
    call_xxfr(vm, |lhs,rhs|{
        match *rhs {
            Value::IntValue(rhs) => {
                match *lhs {
                    Value::IntValue(lhs) => {
                        Result::Ok(Value::IntValue(f(lhs, rhs)))
                    },
                    _ => { 
                        Result::Err(VmErrorReason::TypeMismatchError(int_type_name(), lhs.type_name()))
                     }
                }
            },
            _ => {
                Result::Err(VmErrorReason::TypeMismatchError(int_type_name(), rhs.type_name()))
            }
        }
    } )
}

/// 次のsymbol tokenを使用するワードの呼び出し
pub fn call_with_name<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(&mut V, &String) -> Result<(),VmErrorReason<E>>,
          E: std::fmt::Debug
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