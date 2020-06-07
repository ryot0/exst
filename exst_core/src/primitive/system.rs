//! 
//! システム関連ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::resource::*;
use super::util;

/// システム関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("quit".to_string(), false, String::from(" -- ; return module"), quit);
    vm.define_primitive_word("bye".to_string(), false, String::from(" -- ; stop program"), bye);
    vm.define_primitive_word("[".to_string(), true, String::from(" -- ; start interpletation mode"), to_interpletation);
    vm.define_primitive_word("]".to_string(), false, String::from(" -- ; start compilation mode"), to_compilation);
    vm.define_primitive_word("load".to_string(), false, String::from("\"module name\" -- ; load module"), load);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"

"#}

/// モジュールの終了
fn quit<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.set_state(VmState::Return);
    Result::Ok(())
}

/// プログラムの終了
fn bye<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.set_state(VmState::Stop);
    Result::Ok(())
}

/// コンパイルモードへ遷移
fn to_compilation<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.set_state(VmState::Compilation);
    Result::Ok(())
}

/// コンパイルモードへ遷移
fn to_interpletation<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.set_state(VmState::Interpretation);
    Result::Ok(())
}

/// モジュールのロード
fn load<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let target = v.resources().get_token_iterator(&name)?;
        v.call_script(target);
        v.set_state(VmState::Interpretation);
        v.set_exec_state(VmExecutionState::TokenIteration);
        Result::Ok(())
    })
}
