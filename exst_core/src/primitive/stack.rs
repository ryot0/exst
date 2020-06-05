//! 
//! スタック操作関連ワード
//! 

use super::super::lang::vm::*;

/// スタック操作関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("drop".to_string(), false, String::from("a -- ; "), drop);
    vm.define_primitive_word("dup".to_string(), false, String::from("a -- a a; "), dup);
    vm.define_primitive_word("pick".to_string(), false, String::from("Ac Ac-1 Ac-2 ... A0 c -- Ac Ac-1 Ac-2 ... A0 Ac; "), pick);
    vm.define_primitive_word("roll".to_string(), false, String::from("Ac Ac-1 Ac-2 ... A0 c -- Ac-1 Ac-2 ... A0 Ac; "), roll);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    #?dup
    #swap
    #over
    #rot
    #down
    #nip
    #tuck
    #2drop
    #2dup
    #2over
    #2swap
"#}

/// drop
fn drop<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.data_stack_mut().pop()?;
    Result::Ok(())
}

/// dup
fn dup<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().peek()?;
    vm.data_stack_mut().push(top);
    Result::Ok(())
}

/// pick
fn pick<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let pos = (*top).try_into_usize()?;
    vm.data_stack_mut().pick(pos)?;
    Result::Ok(())
}

/// roll
fn roll<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let pos = (*top).try_into_usize()?;
    vm.data_stack_mut().roll(pos)?;
    Result::Ok(())
}