//! 
//! 入出力ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::resource::Resources;

/// 入出力ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word(".".to_string(), false, String::from("a -- ; print a"), dot);
    vm.define_primitive_word("cr".to_string(), false, String::from(" -- ; print cr"), cr);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"

"#}

/// スタックトップの印字
fn dot<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let s = format!("{}", top);
    vm.resources().write_stdout(s.as_str());
    Result::Ok(())
}

/// 改行の印字
fn cr<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.resources().write_stdout("\n");
    Result::Ok(())
}