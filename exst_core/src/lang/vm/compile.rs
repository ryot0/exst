//! 
//! コンパイル関連の関数
//! 

use super::*;
use super::super::value::*;
use std::rc::Rc;

/// Push命令のコンパイル
/// 
/// # Arguments
/// * value - pushする値
/// 
/// # Return Values
/// 命令
pub fn compile_push_value<V: VmManipulation>(value: Rc<Value<V::ExtraValueType>>) -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Push(value)
}

/// Call命令のコンパイル
/// 
/// # Arguments
/// * adr - 呼び出すワードのコードアドレス
/// 
/// # Return Values
/// 命令
pub fn compile_call_code_address<V: VmManipulation>(adr: CodeAddress) -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Call(adr)
}

/// LocalRef命令のコンパイル
/// 
/// # Arguments
/// * adr - 参照するローカル変数のアドレス
/// 
/// # Return Values
/// 命令
pub fn compile_local_ref<V: VmManipulation>(adr: EnvironmentStackRelativeAddress) -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::LocalRef(adr)
}

/// Trap(UserTrap)命令のコンパイル
/// 
/// # Return Values
/// 命令
pub fn compile_user_trap<V: VmManipulation>() -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Trap(TrapReason::UserTrap)
}

/// Trap(DummyInstructionExecution)命令のコンパイル
/// 
/// # Return Values
/// 命令
pub fn compile_dummy_instruction<V: VmManipulation>() -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Trap(TrapReason::DummyInstructionExecution)
}

/// Nop命令のコンパイル
/// 
/// # Return Values
/// 命令
pub fn compile_nop<V: VmManipulation>() -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Nop
}

/// Return命令のコンパイル
/// 
/// # Return Values
/// 命令
pub fn compile_return<V: VmManipulation>() -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Return
}

/// DebugLabel(WordTerminator)命令のコンパイル
/// 
/// # Return Values
/// 命令
pub fn compile_word_terminator<V: VmManipulation>() -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::DebugLabel(DebugLabel::WordTerminator)
}

/// Branch命令のコンパイル
/// 
/// # Arguments
/// * adr - 分岐先アドレス
/// 
/// # Return Values
/// 命令
pub fn compile_branch<V: VmManipulation>(adr: CodeAddress) -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Branch(adr)
}

/// Exec命令のコンパイル
/// 
/// # Arguments
/// 
/// # Return Values
/// 命令
pub fn compile_exec<V: VmManipulation>() -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::Exec
}

/// SetJump命令のコンパイル
/// 
/// # Arguments
/// * adr - long jump先のアドレス
/// 
/// # Return Values
/// 命令
pub fn compile_set_jump<V: VmManipulation>(adr: CodeAddress) -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::SetJump(adr)
}

/// LongJump命令のコンパイル
/// 
/// # Arguments
/// 
/// # Return Values
/// 命令
pub fn compile_longjump<V: VmManipulation>() -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::LongJump
}

/// シンボル名からLocalRef命令 or Call命令にコンパイル
/// 
/// # Arguments
/// * vm - 実行コンテキストのVm
/// * name - シンボル名
/// * recursable - 定義中のワードを含めて検索するかどうか
/// 
/// # Return Values
/// (命令, イミディエイトワードかどうか)
pub fn compile_symbol<V: VmManipulation>(vm: &V, name: &String, recursable: bool) -> 
    Result<
                (Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>>,bool),
                VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>,
            >
{
    match vm.local_dictionary().find(name) {
        Some(adr) => {
            Result::Ok((compile_local_ref(adr), false))
        },
        None => { 
            let w = if recursable {
                vm.word_dictionary().find_word_within_resavation(name)
            } else {
                vm.word_dictionary().find_word(name)
            }?;
            let c = w.code();
            Result::Ok((compile_call_code_address(c), w.is_immediate()))
        }
    }
}

/// Tokenをコンパイルする
/// 
/// # Arguments
/// * vm - 実行コンテキストのVm
/// * token - トークン
/// * recursable - 定義中のワードを含めて検索するかどうか
/// 
/// # Return Values
/// (命令, イミディエイトワードかどうか)
pub fn compile_token<V: VmManipulation>(vm: &V, token: ValueToken, recursable: bool) ->
Result<
    (Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>>,bool),
    VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>,
>
{
    match token {
        ValueToken::IntValue(i) => {
            Result::Ok((compile_push_value(Rc::new(Value::IntValue(i))), false))
        },
        ValueToken::StrValue(s) => {
            Result::Ok((compile_push_value(Rc::new(Value::StrValue(s))), false))
        },
        ValueToken::Symbol(ref s) => {
            compile_symbol(vm, s, recursable)
        },
    }
}

/// CallPrimitive命令をコンパイル
/// 
/// # Arguments
/// * f - 呼び出し先関数
/// 
/// # Return Values
/// 命令
pub fn compile_call_primitive<V: VmManipulation>(f: fn(&mut V) -> Result<(),VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>>) -> Instruction<V::ExtraValueType,V,VmErrorReason<V::ExtraPrimitiveWordErrorReasonType>> {
    Instruction::CallPrimitive(f)
}


mod tests {

    #[test]
    fn test_compile_token() {
        
        use super::*;

        //VMの初期化
        let r = StdResources::new(String::from("this"));
        let mut v: Vm<i32,i32,StdResources> = Vm::new(r);

        //辞書に登録
        v.local_dictionary_mut().push(String::from("local01"));
        v.local_dictionary_mut().push(String::from("local02"));
        v.word_dictionary_mut().reserve_word_def(String::from("word01"), Word::new(From::from(0)));
        v.word_dictionary_mut().complate_word_def().unwrap();
        v.word_dictionary_mut().reserve_word_def(String::from("word02"), Word::new(From::from(1)));
        v.word_dictionary_mut().last_word_change_immidiate();
        v.word_dictionary_mut().complate_word_def().unwrap();
        v.word_dictionary_mut().reserve_word_def(String::from("local02"), Word::new(From::from(2)));
        v.word_dictionary_mut().complate_word_def().unwrap();
        v.word_dictionary_mut().reserve_word_def(String::from("word03"), Word::new(From::from(3)));

        //内容を検証

        assert_eq!(
            match compile_token(&mut v, ValueToken::IntValue(100), false).unwrap() {
                (Instruction::Push(_),imm) if imm == false => true,
                _ => false,
            },
            true
        );
        assert_eq!(
            match compile_token(&mut v, ValueToken::StrValue(String::from("a")), false).unwrap() {
                (Instruction::Push(_),imm) if imm == false => true,
                _ => false,
            },
            true
        );
        assert_eq!(
            match compile_token(&mut v, ValueToken::Symbol(String::from("local01")), false).unwrap() {
                (Instruction::LocalRef(a),imm) if a == From::from(0) && imm == false => true,
                _ => false,
            },
            true
        );
        //local02はwordでもローカル変数でも定義されているが、検索の優先はローカル変数なのでローカル変数が取得できる
        assert_eq!(
            match compile_token(&mut v, ValueToken::Symbol(String::from("local02")), false).unwrap() {
                (Instruction::LocalRef(a),imm) if a == From::from(1) && imm == false => true,
                _ => false,
            },
            true
        );
        assert_eq!(
            match compile_token(&mut v, ValueToken::Symbol(String::from("word01")), false).unwrap() {
                (Instruction::Call(a),imm) if a == From::from(0) && imm == false => true,
                _ => false,
            },
            true
        );
        assert_eq!(
            match compile_token(&mut v, ValueToken::Symbol(String::from("word02")), false).unwrap() {
                (Instruction::Call(a),imm) if a == From::from(1) && imm == true => true,
                _ => false,
            },
            true
        );
        //word03は定義途中の状態なので、recursibleフラグをtrueにしないと検索することができない
        assert_eq!(compile_token(&mut v, ValueToken::Symbol(String::from("word03")), false).is_err(), true);
        assert_eq!(
            match compile_token(&mut v, ValueToken::Symbol(String::from("word03")), true).unwrap() {
                (Instruction::Call(a),imm) if a == From::from(3) && imm == false => true,
                _ => false,
            },
            true
        );
    }
}