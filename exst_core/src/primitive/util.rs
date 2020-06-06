
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

/// ( x -- Result )のワードの呼び出し
fn call_xfr<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(Rc<Value<V::ExtraValueType>>) -> Result<Value<V::ExtraValueType>,VmErrorReason<E>>,
          E: std::fmt::Debug
{
    let rhs = vm.data_stack_mut().pop()?;
    let v = f(rhs)?;
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
        let lv = (*lhs).try_into()?;
        let rv = (*rhs).try_into()?;
        Result::Ok(Value::IntValue(f(*lv, *rv)))
    })
}

/// ( int -- int )のワードの呼び出し
pub fn call_ifi<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(i32) -> i32,
          E: std::fmt::Debug
{
    call_xfr(vm, |rhs|{
        let v = (*rhs).try_into()?;
        Result::Ok(Value::IntValue(f(*v)))
    })
}

/// 次のsymbol tokenを使用するワードの呼び出し
pub fn call_with_name<V,E,F>(vm: &mut V, f: F) -> Result<(),VmErrorReason<E>>
    where V: VmManipulation,
          F: Fn(&mut V, String) -> Result<(),VmErrorReason<E>>,
          E: std::fmt::Debug
{
    match vm.input_stream_mut().next_token() {
        Option::Some(t) => {
            let token = t.value_token?;
            match token {
                ValueToken::Symbol(name) => {
                    f(vm, name)
                },
                _ => {
                    Result::Err(From::from(TypeMismatchError("Symbol", "UNKNOWN")))
                }
            }
        },
        Option::None => {
            Result::Err(From::from(TypeMismatchError("Symbol", "EMPTY")))
        },
    }
}