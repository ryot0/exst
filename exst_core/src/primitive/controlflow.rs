//! 
//! 制御フロー関連ワード
//! 

use super::super::lang::vm::*;

/// 制御フロー関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word(">C".to_string(), false, String::from("{S: a -- } {C: -- a}; push to controlflow stack"), to_c);
    vm.define_primitive_word("C>".to_string(), false, String::from("{S:  -- a} {C: a -- }; pop from controlflow stack"), pop_c);
    vm.define_primitive_word("C@".to_string(), false, String::from("{S: -- a} {C: a -- a}; peek from controlflow stack"), peek_c);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : 2>C ( {S: a b -- } {C: -- a b}; push to controlflow stack )
        swap >C >C
    ;

    : 2C> ( {S:  -- a b} {C: a b -- }; pop from controlflow stack )
        C> C> swap
    ;

    : 2C@ ( {S: -- a b} {C: a b -- a b}; peek from controlflow stack )
        C> C@ swap dup >C
    ;

    : if ( cond -- ; if statement )
        postpone !!
        cdp >C
        __dummy_instruction__
    ; immediate
    
    : else ( -- ; if statement )
        C>
        1 [compile] literal
        cdp >C
        __dummy_instruction__
        cdp __branch__
        instruction-at
    ; immediate
    
    : endif ( -- ; if statement )
        cdp __branch__
        C> instruction-at
    ; immediate
"#}

/// >C
fn to_c<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    vm.controlflow_stack_mut().push(top);
    Result::Ok(())
}

/// C>
fn pop_c<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.controlflow_stack_mut().pop()?;
    vm.data_stack_mut().push(top);
    Result::Ok(())
}

/// C@
fn peek_c<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.controlflow_stack_mut().peek()?;
    vm.data_stack_mut().push(top);
    Result::Ok(())
}