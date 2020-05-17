//! 
//! 算術ワード
//! 

use super::super::lang::vm::*;
use super::util;

/// 算術ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("+".to_string(), false, plus);
    vm.define_primitive_word("-".to_string(), false, minus);
    vm.define_primitive_word("*".to_string(), false, multiple);
    vm.define_primitive_word("/".to_string(), false, division);
    vm.define_primitive_word("%".to_string(), false, modulus);
}

/// 足し算
fn plus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    util::call_iifi(vm, |lhs,rhs|{ lhs + rhs })
}

/// 引き算
fn minus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    util::call_iifi(vm, |lhs,rhs|{ lhs - rhs })
}

/// 掛け算
fn multiple<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    util::call_iifi(vm, |lhs,rhs|{ lhs * rhs })
}

/// 割り算
fn division<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    util::call_iifi(vm, |lhs,rhs|{ lhs / rhs })
}

/// 余り
fn modulus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    util::call_iifi(vm, |lhs,rhs|{ lhs % rhs })
}