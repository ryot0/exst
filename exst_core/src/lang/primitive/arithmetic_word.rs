
use super::super::vm::*;
use super::util;

pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("+".to_string(), false, plus);
}

fn plus<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    util::call_iifi(vm, |lhs,rhs|{ lhs + rhs })
}
