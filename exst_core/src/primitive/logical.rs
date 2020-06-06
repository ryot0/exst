//! 
//! 論理演算ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::value::*;
use super::util;

/// 論理演算ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("&&".to_string(), false, String::from("a b -- [a and b]; "), and);
    vm.define_primitive_word("||".to_string(), false, String::from("a b -- [a or b]; "), or);
    vm.define_primitive_word("!".to_string(), false, String::from("a -- [not a]; "), not);
    vm.define_primitive_word("=".to_string(), false, String::from("a b -- [a==b]; "), eq);
    vm.define_primitive_word("<>".to_string(), false, String::from("a b -- [a<>b]; "), noteq);
    vm.define_primitive_word("<".to_string(), false, String::from("a b -- [a<b]; "), less);
    vm.define_primitive_word("<=".to_string(), false, String::from("a b -- [a<=b]; "), lesseq);
    vm.define_primitive_word(">".to_string(), false, String::from("a b -- [a>b]; "), gt);
    vm.define_primitive_word(">=".to_string(), false, String::from("a b -- [a>=b]; "), gteq);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : 0= ( a -- [a==0]; ) 0 = ;
    : 0<> ( a -- [a<>0]; ) 0 <> ;
    : 0< ( a -- [a<0]; ) 0 < ;
    : 0> ( a -- [a>0]; ) 0 > ;
    : 0<= ( a -- [a<=0]; ) 0 <= ;
    : 0>= ( a -- [a>=0]; ) 0 >= ;
"#}

/// 論理積
fn and<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs != Value::IntValue(0) && *rhs != Value::IntValue(0) {
            1
        } else {
            0
        }
    )} )
}

/// 論理和
fn or<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs != Value::IntValue(0) || *rhs != Value::IntValue(0) {
            1
        } else {
            0
        }
    )} )
}

/// 否定
fn not<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xfx(vm, |lhs|{ Value::IntValue(
        if *lhs != Value::IntValue(0) {
            0
        } else {
            1
        }
    )} )
}

/// 同値
fn eq<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs == *rhs {
            1
        } else {
            0
        }
    )} )
}

/// not同値
fn noteq<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs != *rhs {
            1
        } else {
            0
        }
    )} )
}

/// 大小比較<
fn less<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs < *rhs {
            1
        } else {
            0
        }
    )} )
}

/// 大小比較<=
fn lesseq<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs <= *rhs {
            1
        } else {
            0
        }
    )} )
}

/// 大小比較>
fn gt<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs > *rhs {
            1
        } else {
            0
        }
    )} )
}

/// 大小比較>=
fn gteq<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_xxfx(vm, |lhs,rhs|{ Value::IntValue(
        if *lhs >= *rhs {
            1
        } else {
            0
        }
    )} )
}