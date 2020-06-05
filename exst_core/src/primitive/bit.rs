//! 
//! ビット演算ワード
//! 

use super::super::lang::vm::*;
use super::util;

/// ビット演算ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("and".to_string(), false, String::from("a b -- [a&b]; "), and);
    vm.define_primitive_word("or".to_string(), false, String::from("a b -- [a|b]; "), or);
    vm.define_primitive_word("xor".to_string(), false, String::from("a b -- [a^b]; "), xor);
    vm.define_primitive_word("not".to_string(), false, String::from("a -- [!a]; "), not);
    vm.define_primitive_word("<<".to_string(), false, String::from("a b -- [a<<b]; left shift"), lshift);
    vm.define_primitive_word(">>".to_string(), false, String::from("a b -- [a>>b]; logical right shift"), rshift);
    vm.define_primitive_word("A>>".to_string(), false, String::from("a b -- [a<<b]; arithmetic right shift"), arshift);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"

"#}

/// and
fn and<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs & rhs })
}

/// or
fn or<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs | rhs })
}

/// xor
fn xor<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs ^ rhs })
}

/// not
fn not<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_ifi(vm, |rhs|{ !rhs })
}

/// <<
fn lshift<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs << rhs })
}

/// >>
fn rshift<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ ((lhs as u32) >> rhs) as i32 })
}

/// A>>
fn arshift<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs >> rhs })
}