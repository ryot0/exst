//! VM
//! 
//! 言語実行のための仮想機械
//! 
//! # Usage Examples
//! ```
//!     use exst_core::lang::vm::*;
//!     use exst_core::lang::resource::*;
//!     use exst_core::lang::value::*;
//!     use std::rc::Rc;
//!     
//!     fn plus(v: &mut Vm<i32,i32,StdResources>) -> Result<(),VmErrorReason<i32>> {
//!         if let Value::IntValue(v1) = *v.data_stack_mut().pop().unwrap() {
//!             if let Value::IntValue(v2) = *v.data_stack_mut().pop().unwrap() {
//!                 v.data_stack_mut().push(Rc::new(Value::IntValue(v1 + v2)));
//!             }
//!         }
//!         Result::Ok(())
//!     }
//! 
//!     //実行スクリプトを登録
//!     let mut r = StdResources::new(String::from("this"));
//!     r.add_resource(String::from(""), String::from("1 2 + 3 +"));
//!
//!     //VMの初期化
//!     let s = r.get_token_iterator(&String::from("$")).unwrap();
//!     let mut v: Vm<i32,i32,StdResources> = Vm::new(r);
//!     //組み込みワードを登録
//!     v.define_primitive_word(String::from("+"), false, String::from("NO COMMENT"), plus);
//!     //実行スクリプトを登録
//!     v.call_script(s);
//!     //実行
//!     v.exec().expect("!");
//!
//!     //計算結果の確認
//!     assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(6));
//! ```
//! 

pub mod dump;
pub mod compile;

use super::mem::*;
use super::word::*;
use super::value::*;
use super::tokenizer::*;
use super::resource::*;
use super::debug::*;
use std::fmt;
use std::rc::Rc;

///////////////////////////////////////////////////////////
/// Vm実行中のエラー
#[derive(Debug)]
pub enum VmErrorReason<E: fmt::Debug> {
    /// コードバッファのアクセスエラー
    CodeBufferAccessError(CodeBufferErrorReason),
    /// データバッファのアクセスエラー
    DataBufferAccessError(DataBufferErrorReason),
    /// データスタックのアクセスエラー
    DataStackAccessError(DataStackErrorReason),
    /// リターンスタックのアクセスエラー
    ReturnStackAccessError(ReturnStackErrorReason),
    /// 環境スタックのアクセスエラー
    EnvironmentStackAccessError(EnvironmentStackErrorReason),
    /// コントロールフロースタックのアクセスエラー
    ControlflowStackAccessError(ControlflowStackErrorReason),
    /// ロングジャンプスタックのアクセスエラー
    LongJumpStackAccessError(LongJumpStackErrorReason),
    /// ワード辞書のエラー
    WordError(WordErrorReason),
    /// トークン解析時のエラー
    TokenError(TokenizerError),
    /// リソースエラー
    ResourceError(ResourceErrorReason),
    /// 外部関数エラー
    ExtraPrimitiveWordError(E),
    /// 型変換エラー
    TypeMismatchError(TypeMismatchError),
    /// Trap
    TrapError(TrapReason),
    /// 命令実行エラー
    InstructionError(&'static str),
}
impl<E: fmt::Debug> From<CodeBufferErrorReason> for VmErrorReason<E> {
    fn from(e: CodeBufferErrorReason) -> Self {
        Self::CodeBufferAccessError(e)
    }
}
impl<E: fmt::Debug> From<DataBufferErrorReason> for VmErrorReason<E> {
    fn from(e: DataBufferErrorReason) -> Self {
        Self::DataBufferAccessError(e)
    }
}
impl<E: fmt::Debug> From<DataStackErrorReason> for VmErrorReason<E> {
    fn from(e: DataStackErrorReason) -> Self {
        Self::DataStackAccessError(e)
    }
}
impl<E: fmt::Debug> From<ReturnStackErrorReason> for VmErrorReason<E> {
    fn from(e: ReturnStackErrorReason) -> Self {
        Self::ReturnStackAccessError(e)
    }
}
impl<E: fmt::Debug> From<EnvironmentStackErrorReason> for VmErrorReason<E> {
    fn from(e: EnvironmentStackErrorReason) -> Self {
        Self::EnvironmentStackAccessError(e)
    }
}
impl<E: fmt::Debug> From<ControlflowStackErrorReason> for VmErrorReason<E> {
    fn from(e: ControlflowStackErrorReason) -> Self {
        Self::ControlflowStackAccessError(e)
    }
}
impl<E: fmt::Debug> From<LongJumpStackErrorReason> for VmErrorReason<E> {
    fn from(e: LongJumpStackErrorReason) -> Self {
        Self::LongJumpStackAccessError(e)
    }
}
impl<E: fmt::Debug> From<WordErrorReason> for VmErrorReason<E> {
    fn from(e: WordErrorReason) -> Self {
        Self::WordError(e)
    }
}
impl<E: fmt::Debug> From<TokenizerError> for VmErrorReason<E> {
    fn from(e: TokenizerError) -> Self {
        Self::TokenError(e)
    }
}
impl<E: fmt::Debug> From<ResourceErrorReason> for VmErrorReason<E> {
    fn from(e: ResourceErrorReason) -> Self {
        Self::ResourceError(e)
    }
}
impl<E: fmt::Debug> From<TypeMismatchError> for VmErrorReason<E> {
    fn from(e: TypeMismatchError) -> Self {
        Self::TypeMismatchError(e)
    }
}
impl<E: fmt::Debug> From<std::num::TryFromIntError> for VmErrorReason<E> {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self::TypeMismatchError(TypeMismatchError("?", "INT"))
    }
}

///////////////////////////////////////////////////////////
/// Vm実行trait
pub trait VmExecution {
    
    /// リソース
    type ResourcesType: Resources;
    /// 拡張エラー型
    type ExtraPrimitiveWordErrorReasonType: fmt::Debug;
    
    /// スクリプトを呼び出す
    fn call_script(&mut self, script: Box<dyn TokenIterator>);
    
    /// リソースを取得
    fn resources_mut(&mut self) -> &mut Self::ResourcesType;
    fn resources(&self) -> &Self::ResourcesType;

    /// 実行
    fn exec(&mut self) -> Result<(),VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>>;
    fn exec_with_args(&mut self, args: &Vec<String>) -> Result<(),VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>>;

    /// vmの状態をリセット
    fn reset_vm_state(&mut self);
}

///////////////////////////////////////////////////////////
/// Vm操作trait
pub trait VmManipulation: Sized + VmExecution {

    /// 拡張型
    type ExtraValueType: fmt::Display + Eq + PartialEq + PartialOrd + Ord;

    /// データスタックの取得
    fn data_stack_mut(&mut self) -> &mut DataStack<Self::ExtraValueType>;
    fn data_stack(&self) -> &DataStack<Self::ExtraValueType>;

    /// リターンスタックの取得
    fn return_stack_mut(&mut self) -> &mut ReturnStack;
    fn return_stack(&self) -> &ReturnStack;

    /// 環境スタックの取得
    fn env_stack_mut(&mut self) -> &mut EnvironmentStack<Self::ExtraValueType>;
    fn env_stack(&self) -> &EnvironmentStack<Self::ExtraValueType>;

    /// コントロールスタックの取得
    fn controlflow_stack_mut(&mut self) -> &mut ControlflowStack<Self::ExtraValueType>;
    fn controlflow_stack(&self) -> &ControlflowStack<Self::ExtraValueType>;
    
    /// コードバッファの取得
    fn code_buffer_mut(&mut self) -> &mut CodeBuffer<Self::ExtraValueType,Self,VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>>;
    fn code_buffer(&self) -> &CodeBuffer<Self::ExtraValueType,Self,VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>>;
    
    /// データバッファの取得
    fn data_buffer_mut(&mut self) -> &mut DataBuffer<Self::ExtraValueType>;
    fn data_buffer(&self) -> &DataBuffer<Self::ExtraValueType>;

    /// VM状態の取得
    fn state(&self) -> VmState;
    
    /// VM状態の変更
    /// 
    /// # Return Values
    /// 変更前のVM状態
    fn set_state(&mut self, new_state: VmState) -> VmState;

    /// VM状態の取得
    fn exec_state(&self) -> VmExecutionState;
    
    /// VM状態の変更
    /// 
    /// # Return Values
    /// 変更前のVM状態
    fn set_exec_state(&mut self, new_state: VmExecutionState) -> VmExecutionState;
    
    /// プログラムカウンタの取得
    fn program_counter(&self) -> CodeAddress;
    
    /// プログラムカウンタの変更
    /// 
    /// # Return Values
    /// 変更前のプログラムカウンタ
    fn set_program_counter(&mut self, new_program_counter: CodeAddress) -> CodeAddress;
    
    /// ワード辞書の取得
    fn word_dictionary_mut(&mut self) -> &mut Dictionary;
    fn word_dictionary(&self) -> &Dictionary;

    /// ローカル変数辞書を取得
    fn local_dictionary_mut(&mut self) -> &mut LocalDictionary;
    fn local_dictionary(&self) -> &LocalDictionary;
    
    /// 現在のスクリプトの取得
    fn input_stream_mut(&mut self) -> &mut dyn TokenIterator;
    fn input_stream(&self) -> &dyn TokenIterator;

    /// デバッグ情報の取得
    fn debug_info_store(&self) -> &DebugInfoStore;

    /// 現在実行中のトップレベルのワードの行番号を取得
    fn line_number(&self) -> LineNumber;

    /// 現在実行中のトップレベルのワードのカラム番号を取得
    fn column_number(&self) -> ColumnNumber;
}

///////////////////////////////////////////////////////////
/// Vmへのプリミティブワードの登録trait
pub trait VmPrimitiveWordStore {

    /// 拡張エラー型
    type ExtraPrimitiveWordErrorReasonType: fmt::Debug;
    /// Vmの実態
    type VmType: VmManipulation;

    /// プリミティブワードの定義を登録
    /// 
    /// # Arguments
    /// * name - ワード名
    /// * immidiate - イミディエイトワード
    /// * doc - ドキュメント
    /// * f - 実態の関数
    fn define_primitive_word(&mut self, name: String, immidiate: bool, doc: String, f: fn(&mut Self::VmType) -> Result<(),VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>>);
}

///////////////////////////////////////////////////////////
/// Vmの状態
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum VmState {
    /// コンパイル状態
    Compilation,
    /// ワードを再帰的に呼び出し可能なコンパイル状態
    RecursableCompilation,
    /// インタープリター状態
    Interpretation,
    /// スクリプトからのリターン
    Return,
    /// 停止
    Stop,
}
impl fmt::Display for VmState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VmState::Compilation => write!(f, "Compilation"),
            VmState::RecursableCompilation => write!(f, "RecursableCompilation"),
            VmState::Interpretation => write!(f, "Interpretation"),
            VmState::Return => write!(f, "Return"),
            VmState::Stop => write!(f, "Stop"),
        }
    }
}
///////////////////////////////////////////////////////////
/// Vmの実行状態
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum VmExecutionState {
    /// コードを実行する
    CodeExecution,
    /// Tokenを読んで実行する
    TokenIteration,
}
impl fmt::Display for VmExecutionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VmExecutionState::CodeExecution => write!(f, "CodeExecution"),
            VmExecutionState::TokenIteration => write!(f, "TokenIteration"),
        }
    }
}

///////////////////////////////////////////////////////////
/// Vmの実態
pub struct Vm<T,E,R>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, R: Resources, E: fmt::Debug
{
    state: VmState,
    execution_state: VmExecutionState,
    input: Box<dyn TokenIterator>,
    current_line_number: LineNumber,
    current_column_number: ColumnNumber,
    word_dictionary: Dictionary,
    local_dictionary: LocalDictionary,
    data_stack: DataStack<T>,
    return_stack: ReturnStack,
    env_stack: EnvironmentStack<T>,
    controlflow_stack: ControlflowStack<T>,
    long_jump_stack: LongJumpStack,
    code_buffer: CodeBuffer<T,Self,VmErrorReason<E>>,
    data_buffer: DataBuffer<T>,
    program_counter: CodeAddress,
    script_call_stack: ScriptCallStack,
    debug_info_store: DebugInfoStore,
    resources: R,
}
///////////////////////////////////////////////////////////
/// コンストラクタなど
impl<T,E,R> Vm<T,E,R>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, R: Resources, E: fmt::Debug
{
    /// コンストラクタ
    /// 
    /// # Arguments
    /// * resources - リソース
    pub fn new(resources: R) -> Self {
        Vm {
            state: VmState::Interpretation,
            execution_state: VmExecutionState::TokenIteration,
            //初期状態は、空のstreamを登録する。空のストリームはすぐにreturnするため、
            //call_scriptしたスクリプトに即座に処理がうつる仕組み
            input: Box::new(EmptyTokenStream::new()),
            current_line_number: 0,
            current_column_number: 0,
            word_dictionary: Dictionary::new(),
            local_dictionary: LocalDictionary::new(),
            data_stack: DataStack::new(),
            return_stack: ReturnStack::new(),
            env_stack: EnvironmentStack::new(),
            controlflow_stack: ControlflowStack::new(),
            long_jump_stack: LongJumpStack::new(),
            code_buffer: CodeBuffer::new(),
            data_buffer: DataBuffer::new(),
            program_counter: Default::default(),
            script_call_stack: ScriptCallStack::new(),
            debug_info_store: DebugInfoStore::new(),
            resources: resources,
        }
    }
}
///////////////////////////////////////////////////////////
/// 実行処理の実態
impl<T,E,R> Vm<T,E,R>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, R: Resources, E: fmt::Debug
{
    /// 1つの命令を実行する
    fn apply_instruction(&mut self, inst: Instruction<T,Self,VmErrorReason<E>>) -> Result<(),VmErrorReason<E>> {
        match inst {
            Instruction::Push(v) => {
                //データスタックにオペランドの値をプッシュ
                self.data_stack.push(v);
                self.program_counter = self.program_counter.next();
            },
            Instruction::Call(adr) => {
                //環境を退避して、オペランドのアドレスにジャンプ
                self.return_stack.push(self.program_counter.next(), self.env_stack.here());
                self.program_counter = adr;
            },
            Instruction::CallPrimitive(f) => {
                //オペランドの関数を呼び出す
                f(self)?;
                self.program_counter = self.program_counter.next();
            },
            Instruction::Return => {
                //環境の巻き戻し
                let frame = self.return_stack.pop()?;
                self.env_stack.rollback(frame.stack_address())?;
                self.program_counter = frame.return_address();
            },
            Instruction::LocalRef(adr) => {
                //オペランドで指定された環境スタックの場所の値をデータスタックにプッシュ
                let base = self.return_stack.peek()?;
                let val = self.env_stack.get(base.stack_address(), adr)?;
                self.data_stack.push(val);
                self.program_counter = self.program_counter.next();
            },
            Instruction::Nop => {
                //何もしない命令
                self.program_counter = self.program_counter.next();
            },
            Instruction::Trap(v) => {
                //次回再開できる状態で即時エラーで終了
                self.program_counter = self.program_counter.next();
                return Result::Err(VmErrorReason::TrapError(v));
            },
            Instruction::DebugLabel(_) => {
                //何もしない命令
                self.program_counter = self.program_counter.next();
            },
            Instruction::Branch(adr) => {
                let top = self.data_stack.pop()?;
                if Value::IntValue(0) == *top {
                    self.program_counter = self.program_counter.next();
                } else {
                    //stack topが０以外の場合、分岐
                    self.program_counter = adr;
                }
            },
            Instruction::Jump(adr) => {
                self.program_counter = adr;
            },
            Instruction::Exec => {
                let top = self.data_stack.pop()?;
                match *top {
                    Value::CodeAddress(adr) => {
                        //環境を退避して、オペランドのアドレスにジャンプ
                        self.return_stack.push(self.program_counter.next(), self.env_stack.here());
                        self.program_counter = adr;
                    },
                    Value::EnvAddress(adr) => {
                        //オペランドで指定された環境スタックの場所の値をデータスタックにプッシュ
                        let base = self.return_stack.peek()?;
                        let val = self.env_stack.get(base.stack_address(), adr)?;
                        self.data_stack.push(val);
                        self.program_counter = self.program_counter.next();
                    },
                    _ => {
                        self.data_stack_mut().push(top);
                    },
                }
            },
            Instruction::SetJump(adr) => {
                self.long_jump_stack.push(adr, self.return_stack.here(), self.env_stack.here(), self.data_stack.here());
                self.program_counter = self.program_counter.next();
            },
            Instruction::LongJump => {
                let top = self.long_jump_stack.pop()?;
                //データスタックの先頭を退避
                let data = self.data_stack.peek()?;
                self.program_counter = top.return_address();
                self.return_stack.rollback(top.return_stack_address())?;
                self.env_stack.rollback(top.env_stack_address())?;
                self.data_stack.rollback(top.data_stack_address())?;
                //データスタックの先頭のみを戻す
                self.data_stack.push(data);
            },
            Instruction::PopJump => {
                self.long_jump_stack.pop()?;
            }
        }
        Result::Ok(())
    }

    /// コンパイルコードの実行
    fn eval_instruction(&mut self, inst: Instruction<T,Self,VmErrorReason<E>>) -> Result<(),VmErrorReason<E>> {
        match inst {
            Instruction::Call(adr) => {
                self.return_stack.push(CodeAddress::default(), self.env_stack.here());
                self.program_counter = adr;
                self.execution_state = VmExecutionState::CodeExecution;
            },
            _ => {
                self.apply_instruction(inst)?;
            }
        }
        Result::Ok(())
    }

    /// Tokenをコンパイルする
    fn compile_token(&mut self, token: ValueToken, recursable: bool) ->
        Result<
            (Instruction<T,Self,VmErrorReason<E>>,bool),
            VmErrorReason<E>,
        >
    {
        compile::compile_token(self, token, recursable)
    }

    /// コード実行状態での実行
    fn exec_execution(&mut self) -> Result<(),VmErrorReason<E>> {
        while let CodeAddress(Address::Entity(_)) = self.program_counter {
            let inst = self.code_buffer.get(self.program_counter)?;
            self.apply_instruction(inst)?;
            if self.execution_state != VmExecutionState::CodeExecution {
                break;
            }
        }
        self.execution_state = VmExecutionState::TokenIteration;
        Result::Ok(())
    }

    /// インタープリター状態での実行
    fn exec_interpretation(&mut self) -> Result<(),VmErrorReason<E>> {
        while let Some(next_token) = self.input.next_token() {
            self.current_line_number = next_token.line_number;
            self.current_column_number = next_token.column_number;
            let token = next_token.value_token?;
            let (inst, _) = self.compile_token(token, false)?;
            self.eval_instruction(inst)?;
            if self.execution_state != VmExecutionState::TokenIteration || self.state != VmState::Interpretation {
                return Result::Ok(());
            }
        }
        self.state = VmState::Return;
        Result::Ok(())
    }

    /// コンパイル状態での実行
    fn exec_compilation(&mut self, recursable: bool) -> Result<(),VmErrorReason<E>> {
        while let Some(next_token) = self.input.next_token() {
            self.current_line_number = next_token.line_number;
            self.current_column_number = next_token.column_number;
            let token = next_token.value_token?;
            let (inst, immidiate) = self.compile_token(token, recursable)?;
            if immidiate {
                self.eval_instruction(inst)?;
                if self.execution_state != VmExecutionState::TokenIteration || self.state != VmState::Compilation {
                    return Result::Ok(());
                }
            } else {
                self.debug_info_store.put(self.code_buffer.here(), self.current_line_number, self.current_column_number);
                self.code_buffer.push(inst);
            }
        }
        self.state = VmState::Return;
        Result::Ok(())
    }

    /// スクリプトからのリターン処理の実行
    fn exec_return(&mut self) -> Result<(),VmErrorReason<E>> {
        match self.script_call_stack.pop() {
            Some((i,s,e,p)) => {
                self.input = i;
                self.state = s;
                self.execution_state = e;
                self.program_counter = p;
                self.debug_info_store.return_script();
            },
            None => {
                self.state = VmState::Stop;
            },
        };
        Result::Ok(())
    }
}
///////////////////////////////////////////////////////////
/// ワード登録の実態
impl<T,E,R> VmPrimitiveWordStore for Vm<T,E,R>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, R: Resources, E: fmt::Debug
{
    type ExtraPrimitiveWordErrorReasonType = E;
    type VmType = Self;

    fn define_primitive_word(&mut self, name: String, immidiate: bool, doc: String, f: fn(&mut Self::VmType) -> Result<(),VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>>) {
        self.word_dictionary.reserve_word_def(name, Word::new(self.code_buffer.here()));
        self.code_buffer.push(compile::compile_call_primitive(f));
        self.code_buffer.push(compile::compile_return());
        self.code_buffer.push(compile::compile_word_terminator());
        if immidiate {
            self.word_dictionary.last_word_change_immidiate();
        }
        self.word_dictionary.last_word_set_document(doc);
        self.word_dictionary.complate_word_def().unwrap();
    }
}
///////////////////////////////////////////////////////////
/// Vm実行の実態
impl<T,E,R> VmExecution for Vm<T,E,R>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, R: Resources, E: fmt::Debug
{
    type ResourcesType = R;
    type ExtraPrimitiveWordErrorReasonType = E;

    fn call_script(&mut self, script: Box<dyn TokenIterator>) {
        self.debug_info_store.call_script(script.script_name());
        self.script_call_stack.push(std::mem::replace(&mut self.input, script), self.state, self.execution_state, self.program_counter);
    }
    fn resources_mut(&mut self) -> &mut Self::ResourcesType {
        &mut self.resources
    }
    fn resources(&self) -> &Self::ResourcesType {
        &self.resources
    }

    /// Vmの実行
    fn exec(&mut self) -> Result<(),VmErrorReason<E>> {
        loop {
            match self.execution_state {
                VmExecutionState::TokenIteration => match self.state {
                    VmState::Interpretation => { self.exec_interpretation()?; },
                    VmState::Compilation => { self.exec_compilation(false)?; },
                    VmState::RecursableCompilation => { self.exec_compilation(true)?; },
                    VmState::Return => { self.exec_return()?; },
                    VmState::Stop => { break; },
                },
                VmExecutionState::CodeExecution => {
                    self.exec_execution()?;
                },
            }
        }
        self.reset_vm_state();
        Result::Ok(())
    }

    /// 引数月でVmの実行
    fn exec_with_args(&mut self, args: &Vec<String>) -> Result<(),VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>> {
        for a in args.iter() {
            self.env_stack_mut().push(Rc::new(Value::StrValue(a.clone())));
        }
        self.exec()
    }

    /// VMの状態をリセット
    fn reset_vm_state(&mut self) {
        self.execution_state = VmExecutionState::TokenIteration;
        self.state = VmState::Interpretation;
    }
}
///////////////////////////////////////////////////////////
/// Vm操作の実態
impl<T,E,R> VmManipulation for Vm<T,E,R>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, R: Resources, E: fmt::Debug
{
    type ExtraValueType = T;

    fn data_stack_mut(&mut self) -> &mut DataStack<Self::ExtraValueType> {
        &mut self.data_stack
    }
    fn data_stack(&self) -> &DataStack<Self::ExtraValueType> {
        &self.data_stack
    }
    fn return_stack_mut(&mut self) -> &mut ReturnStack {
        &mut self.return_stack
    }
    fn return_stack(&self) -> &ReturnStack {
        &self.return_stack
    }
    fn env_stack_mut(&mut self) -> &mut EnvironmentStack<Self::ExtraValueType> {
        &mut self.env_stack
    }
    fn env_stack(&self) -> &EnvironmentStack<Self::ExtraValueType> {
        &self.env_stack
    }
    fn controlflow_stack_mut(&mut self) -> &mut ControlflowStack<Self::ExtraValueType> {
        &mut self.controlflow_stack
    }
    fn controlflow_stack(&self) -> &ControlflowStack<Self::ExtraValueType> {
        &self.controlflow_stack
    }
    fn code_buffer_mut(&mut self) -> &mut CodeBuffer<Self::ExtraValueType,Self,VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>> {
        &mut self.code_buffer
    }
    fn code_buffer(&self) -> &CodeBuffer<Self::ExtraValueType,Self,VmErrorReason<Self::ExtraPrimitiveWordErrorReasonType>> {
        &self.code_buffer
    }
    fn data_buffer_mut(&mut self) -> &mut DataBuffer<Self::ExtraValueType> {
        &mut self.data_buffer
    }
    fn data_buffer(&self) -> &DataBuffer<Self::ExtraValueType> {
        &self.data_buffer
    }
    fn state(&self) -> VmState {
        self.state
    }
    fn set_state(&mut self, new_state: VmState) -> VmState {
        std::mem::replace(&mut self.state, new_state)
    }
    fn exec_state(&self) -> VmExecutionState {
        self.execution_state
    }
    fn set_exec_state(&mut self, new_state: VmExecutionState) -> VmExecutionState {
        std::mem::replace(&mut self.execution_state, new_state)
    }
    fn program_counter(&self) -> CodeAddress {
        self.program_counter
    }
    fn set_program_counter(&mut self, new_program_counter: CodeAddress) -> CodeAddress {
        std::mem::replace(&mut self.program_counter, new_program_counter)
    }
    fn word_dictionary_mut(&mut self) -> &mut Dictionary {
        &mut self.word_dictionary
    }
    fn word_dictionary(&self) -> &Dictionary {
        &self.word_dictionary
    }
    fn local_dictionary_mut(&mut self) -> &mut LocalDictionary {
        &mut self.local_dictionary
    }
    fn local_dictionary(&self) -> &LocalDictionary {
        &self.local_dictionary
    }
    fn input_stream_mut(&mut self) -> &mut dyn TokenIterator {
        &mut *self.input
    }
    fn input_stream(&self) -> &dyn TokenIterator {
        &*self.input
    }
    fn debug_info_store(&self) -> &DebugInfoStore {
        &self.debug_info_store
    }
    fn line_number(&self) -> LineNumber {
        self.current_line_number
    }
    fn column_number(&self) -> ColumnNumber {
        self.current_column_number
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::rc::Rc;

    type V = Vm<i32,i32,StdResources>;
    type VError = VmErrorReason<i32>;

    //２項目の足し算
    fn plus(v: &mut V) -> Result<(),VError> {
        if let Value::IntValue(v1) = *v.data_stack_mut().pop().unwrap() {
            if let Value::IntValue(v2) = *v.data_stack_mut().pop().unwrap() {
                v.data_stack_mut().push(Rc::new(Value::IntValue(v1 + v2)));
            }
        }
        Result::Ok(())
    }

    #[test]
    fn test_simple_interpretation() {

        //実行スクリプトを登録
        let mut r = StdResources::new(String::from("this"));
        r.add_resource(String::from(""), String::from("1 \"2\" 3"));

        //VMの初期化と実行
        let s = r.get_token_iterator(&String::from("$")).unwrap();
        let mut v: V = Vm::new(r);
        //実行スクリプトを登録
        v.call_script(s);
        //実行
        v.exec().expect("!");

        //スタックに順番に値が積まれるだけ
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(3));
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::StrValue(String::from("2")));
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(1));
    }

    #[test]
    fn test_simple_compilation() {

        //実行スクリプトを登録
        let mut r = StdResources::new(String::from("this"));
        r.add_resource(String::from(""), String::from("1 \"2\" 3"));

        //VMの初期化と実行
        let s = r.get_token_iterator(&String::from("$")).unwrap();
        let mut v: V = Vm::new(r);
        //実行スクリプトを登録
        v.call_script(s);
        //初期状態を変更
        v.set_state(VmState::Compilation);
        //実行
        v.exec().expect("!");

        //データスタックは空
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
        //コードバッファの内容にコンパイルコードが登録されているはず
        assert_eq!(
            match v.code_buffer_mut().get(From::from(0)) {
                Result::Ok(Instruction::Push(ref val)) if **val == Value::IntValue(1) => true,
                _ => false,
            },
            true
        );
        assert_eq!(
            match v.code_buffer_mut().get(From::from(1)) {
                Result::Ok(Instruction::Push(ref val)) if **val == Value::StrValue(String::from("2")) => true,
                _ => false,
            },
            true
        );
        assert_eq!(
            match v.code_buffer_mut().get(From::from(2)) {
                Result::Ok(Instruction::Push(ref val)) if **val == Value::IntValue(3) => true,
                _ => false,
            },
            true
        );
    }

    #[test]
    fn test_word_primitive_call() {

        //実行スクリプトを登録
        let mut r = StdResources::new(String::from("this"));
        r.add_resource(String::from(""), String::from("1 2 + 3 +"));

        //VMの初期化と実行
        let s = r.get_token_iterator(&String::from("$")).unwrap();
        let mut v: V = Vm::new(r);
        //組み込みワードを登録
        v.define_primitive_word(String::from("+"), false, String::from("NO COMMENT"), plus);
        //実行スクリプトを登録
        v.call_script(s);
        //実行
        v.exec().expect("!");

        //計算結果の確認
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(6));
        //データスタックは空
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
    }

    #[test]
    fn test_script_call() {

        //実行スクリプトを登録
        let mut r = StdResources::new(String::from("this"));
        r.add_resource(String::from("1"), String::from("1 2 + 3 +")); //6
        r.add_resource(String::from("2"), String::from("2 3 + 4 +")); //9
        r.add_resource(String::from("3"), String::from("1 + +")); //6 + 9 + 1 = 16

        //VMの初期化と実行
        let s1 = r.get_token_iterator(&String::from("$1")).unwrap();
        let s2 = r.get_token_iterator(&String::from("$2")).unwrap();
        let s3 = r.get_token_iterator(&String::from("$3")).unwrap();
        let mut v: V = Vm::new(r);
        //組み込みワードを登録
        v.define_primitive_word(String::from("+"), false, String::from("NO COMMENT"), plus);
        //実行スクリプトを登録
        v.call_script(s3);
        v.call_script(s2);
        v.call_script(s1);
        //実行
        v.exec().expect("!");

        //計算結果の確認
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(16));
        //データスタックは空
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
    }

    #[test]
    fn test_user_word_call() {

        //実行スクリプトを登録
        let mut r = StdResources::new(String::from("this"));
        r.add_resource(String::from("1"), String::from("1 +")); //1 + x
        r.add_resource(String::from("2"), String::from("2 w1 +")); //2 + ( 1 + x )
        r.add_resource(String::from("3"), String::from("3 w2 +")); //3 + ( 2 + ( 1 + x ) )
        r.add_resource(String::from("4"), String::from("4 w3")); //3 + ( 2 + ( 1 + 4 ) ) = 10

        //VMの初期化と実行
        let w1 = r.get_token_iterator(&String::from("$1")).unwrap();
        let w2 = r.get_token_iterator(&String::from("$2")).unwrap();
        let w3 = r.get_token_iterator(&String::from("$3")).unwrap();
        let s1 = r.get_token_iterator(&String::from("$4")).unwrap();
        let mut v: V = Vm::new(r);

        //組み込みワードを登録
        v.define_primitive_word(String::from("+"), false, String::from("NO COMMENT"), plus);

        //w1をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w1"), Word::new(code_point));
        v.call_script(w1);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.word_dictionary_mut().complate_word_def().unwrap();
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
        //w2をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w2"), Word::new(code_point));
        v.call_script(w2);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.word_dictionary_mut().complate_word_def().unwrap();
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
        //w3をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w3"), Word::new(code_point));
        v.call_script(w3);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.word_dictionary_mut().complate_word_def().unwrap();
        assert_eq!(v.data_stack_mut().pop().is_err(), true);

        //s1を実行
        v.set_state(VmState::Interpretation);
        v.call_script(s1);
        v.exec().expect("!");

        //計算結果の確認
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(10));
        //データスタックは空
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
    }

    #[test]
    fn test_user_immidiate_word_call() {

        //実行スクリプトを登録
        let mut r = StdResources::new(String::from("this"));
        r.add_resource(String::from("1"), String::from("1 1 +"));
        r.add_resource(String::from("2"), String::from("2 w1 +")); 

        //VMの初期化と実行
        let w1 = r.get_token_iterator(&String::from("$1")).unwrap();
        let w2 = r.get_token_iterator(&String::from("$2")).unwrap();
        let mut v: V = Vm::new(r);

        //組み込みワードを登録
        v.define_primitive_word(String::from("+"), false, String::from("NO COMMENT"), plus);

        //w1をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w1"), Word::new(code_point));
        v.call_script(w1);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.word_dictionary_mut().complate_word_def().unwrap();
        //イミディエイトワードに変更
        v.word_dictionary_mut().last_word_change_immidiate();
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
        //w2をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w2"), Word::new(code_point));
        v.call_script(w2);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.word_dictionary_mut().complate_word_def().unwrap();

        //コンパル状態でデータスタックの内容が変化したことを確認
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(2));
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
    }

    #[test]
    fn test_user_trap() {

        //実行スクリプトを登録
        let mut r = StdResources::new(String::from("this"));
        r.add_resource(String::from("1"), String::from("1 +")); //1 + x
        r.add_resource(String::from("2"), String::from("2 w1 trap +")); //2 + ( 1 + x )
        r.add_resource(String::from("3"), String::from("3 w2 +")); //3 + ( 2 + ( 1 + x ) )
        r.add_resource(String::from("4"), String::from("4 w3")); //3 + ( 2 + ( 1 + 4 ) ) = 10

        //VMの初期化と実行
        let w1 = r.get_token_iterator(&String::from("$1")).unwrap();
        let w2 = r.get_token_iterator(&String::from("$2")).unwrap();
        let w3 = r.get_token_iterator(&String::from("$3")).unwrap();
        let s1 = r.get_token_iterator(&String::from("$4")).unwrap();
        let mut v: V = Vm::new(r);

        //組み込みワードを登録
        v.define_primitive_word(String::from("+"), false, String::from("NO COMMENT"), plus);

        //trapをコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("trap"), Word::new(code_point));
        v.code_buffer_mut().push(Instruction::Trap(TrapReason::UserTrap));
        v.code_buffer_mut().push(Instruction::Return);
        v.code_buffer_mut().push(Instruction::DebugLabel(DebugLabel::WordTerminator));
        v.word_dictionary_mut().complate_word_def().unwrap();
        //w1をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w1"), Word::new(code_point));
        v.call_script(w1);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.code_buffer_mut().push(Instruction::DebugLabel(DebugLabel::WordTerminator));
        v.word_dictionary_mut().complate_word_def().unwrap();
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
        //w2をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w2"), Word::new(code_point));
        v.call_script(w2);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.code_buffer_mut().push(Instruction::DebugLabel(DebugLabel::WordTerminator));
        v.word_dictionary_mut().complate_word_def().unwrap();
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
        //w3をコンパイルして登録
        let code_point = v.code_buffer_mut().here();
        v.word_dictionary_mut().reserve_word_def(String::from("w3"), Word::new(code_point));
        v.call_script(w3);
        v.set_state(VmState::Compilation);
        v.exec().expect("!");
        v.code_buffer_mut().push(Instruction::Return);
        v.code_buffer_mut().push(Instruction::DebugLabel(DebugLabel::WordTerminator));
        v.word_dictionary_mut().complate_word_def().unwrap();
        assert_eq!(v.data_stack_mut().pop().is_err(), true);

        //s1を実行
        v.set_state(VmState::Interpretation);
        v.call_script(s1);
        assert_eq!(
            match v.exec() {
                Result::Err(VmErrorReason::TrapError(TrapReason::UserTrap)) => true,
                _ => false,
            }
            ,true
        );

        print!("{}", dump::VmDump(&v, dump::dump_all_info));

        //再度実行
        v.exec().expect("!");

        //計算結果の確認
        assert_eq!(*v.data_stack_mut().pop().unwrap(), Value::IntValue(10));
        //データスタックは空
        assert_eq!(v.data_stack_mut().pop().is_err(), true);
    }
}