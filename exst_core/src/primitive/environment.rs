//! 
//! 環境スタック操作関連ワード
//! 

use super::super::lang::vm::*;
use super::util;

/// 環境スタック操作関連ワード
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word(">E".to_string(), false, String::from("{S: a -- } {E: -- a}; push to environment stack"), to_e);
    vm.define_primitive_word("E>".to_string(), false, String::from("{S:  -- a} {E: a -- }; pop from environment stack"), pop_e);
    vm.define_primitive_word("E@".to_string(), false, String::from("{S: -- a} {E: a -- a}; peek from environment stack"), peek_e);
    vm.define_primitive_word("create-local".to_string(), true, String::from("{\"local variable name\" -- } ; create local variable with name"), create_local);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : 2>E ( {S: a b -- } {E: -- a b}; push to environment stack )
        swap >E >E
    ;

    : 2E> ( {S:  -- a b} {E: a b -- }; pop from environment stack )
        E> E> swap
    ;

    : 2E@ ( {S: -- a b} {E: a b -- a b}; peek from environment stack )
        E> E@ swap dup >E
    ;
"#}

/// >E
fn to_e<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    vm.env_stack_mut().push(top);
    Result::Ok(())
}

/// E>
fn pop_e<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.env_stack_mut().pop()?;
    vm.data_stack_mut().push(top);
    Result::Ok(())
}

/// E@
fn peek_e<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.env_stack_mut().peek()?;
    vm.data_stack_mut().push(top);
    Result::Ok(())
}

/// create-local
fn create_local<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        v.local_dictionary_mut().push(name);
        Result::Ok(())
    })
}