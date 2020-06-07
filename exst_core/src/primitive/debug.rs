//! 
//! デバッグ関連ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::vm::dump;
use super::super::lang::resource::*;
use super::util;

/// デバッグ関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("dump-all".to_string(), false, String::from(" -- ; dump all debug info"), dump_all);
    vm.define_primitive_word("dump-vm-state".to_string(), false, String::from(" -- ; dump all debug info"), dump_vm_state);
    vm.define_primitive_word("see".to_string(), false, String::from("\"word name\" -- ; dump word code"), dump_word);
    vm.define_primitive_word("dump-all-word".to_string(), false, String::from(" -- ; dump all word code"), dump_all_word);
    vm.define_primitive_word("words".to_string(), false, String::from(" -- ; dump dictionary"), dump_dictionary);
    vm.define_primitive_word("dump-stack".to_string(), false, String::from(" -- ; dump data stack"), dump_stack);
    vm.define_primitive_word("dump-env".to_string(), false, String::from(" -- ; dump environment stack"), dump_env);
    vm.define_primitive_word("dump-controlflow-stack".to_string(), false, String::from(" -- ; dump controlflow stack"), dump_controlflow_stack);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : trap ( -- ; trap anytime )
        [__user_trap__]
    ;

    : assert ( flg -- ; if flg is false then trap )
        ! if [__user_trap__] endif
    ;

    : assert-eq ( a b -- ; if a <> b then trap )
        <> if [__user_trap__] endif
    ;
"#}

/// dump-all
fn dump_all<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let msg = format!("{}", dump::VmDump(vm, dump::dump_all_info));
    vm.resources().write_stdout(&msg);
    Result::Ok(())
}

/// dump-vm-state
fn dump_vm_state<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let msg = format!("{}", dump::VmDump(vm, dump::dump_vm_state));
    vm.resources().write_stdout(&msg);
    Result::Ok(())
}

/// see
fn dump_word<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let msg = format!("{}", dump::VmDump1(v, &name, dump::dump_word_code));
        v.resources().write_stdout(&msg);
        Result::Ok(())
    })
}

/// dump-all-word
fn dump_all_word<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let msg = format!("{}", dump::VmDump(vm, dump::dump_all_word_code));
    vm.resources().write_stdout(&msg);
    Result::Ok(())
}

/// words
fn dump_dictionary<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let msg = format!("{}", dump::VmDump(vm, dump::dump_all_word));
    vm.resources().write_stdout(&msg);
    Result::Ok(())
}

/// dump-stack
fn dump_stack<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let msg = format!("{}", dump::VmDump(vm, dump::dump_data_stack));
    vm.resources().write_stdout(&msg);
    Result::Ok(())
}

/// dump-env
fn dump_env<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let msg = format!("{}", dump::VmDump(vm, dump::dump_env));
    vm.resources().write_stdout(&msg);
    Result::Ok(())
}

/// dump-controlflow-stack
fn dump_controlflow_stack<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let msg = format!("{}", dump::VmDump(vm, dump::dump_controlflow_stack));
    vm.resources().write_stdout(&msg);
    Result::Ok(())
}