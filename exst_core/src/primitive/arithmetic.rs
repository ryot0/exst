//! 
//! 算術ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::value::*;
use super::util;

/// 算術ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("+".to_string(), false, String::from("a b -- [a+b]; "), plus);
    vm.define_primitive_word("-".to_string(), false, String::from("a b -- [a-b]; "), minus);
    vm.define_primitive_word("*".to_string(), false, String::from("a b -- [a*b]; "), multiple);
    vm.define_primitive_word("/".to_string(), false, String::from("a b -- [a/b]; "), division);
    vm.define_primitive_word("%".to_string(), false, String::from("a b -- [a%b]; "), modulus);
    vm.define_primitive_word("c+".to_string(), false, String::from("a b -- [a+b]; result is CodeAddress"), cplus);
    vm.define_primitive_word("d+".to_string(), false, String::from("a b -- [a+b]; resutt is DataAddress"), dplus);
    vm.define_primitive_word("e+".to_string(), false, String::from("a b -- [a+b]; result is EnvAddress"), eplus);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : 1+ ( a -- [a+1]; increament )
        1 +
    ;
    : 1- ( a -- [a-1]; decreament )
        1 -
    ;
"#}

/// 足し算
fn plus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs + rhs })
}

/// 引き算
fn minus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs - rhs })
}

/// 掛け算
fn multiple<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs * rhs })
}

/// 割り算
fn division<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs / rhs })
}

/// 余り
fn modulus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_iifi(vm, |lhs,rhs|{ lhs % rhs })
}

/// CodeAddressの足し算
fn cplus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_aafa(vm, |lhs,rhs|{ Value::CodeAddress(From::from(lhs + rhs)) })
}

/// DataAddressの足し算
fn dplus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_aafa(vm, |lhs,rhs|{ Value::DataAddress(From::from(lhs + rhs)) })
}

/// EnvAddressの足し算
fn eplus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_aafa(vm, |lhs,rhs|{ Value::EnvAddress(From::from(lhs + rhs)) })
}