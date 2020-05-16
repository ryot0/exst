mod arithmetic_word;
mod util;

use super::lang::vm::*;

pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    arithmetic_word::initialize(vm);
}
