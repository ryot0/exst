//! VM内で扱う値の定義
//! 

use super::vm::*;
use std::rc::Rc;
use std::fmt;

///////////////////////////////////////////////////////////
/// アドレス計算エラー
/// 
#[derive(Debug,Clone,Copy)]
pub enum BufferAddressErrorReason {
    /// 無効なアドレス
    InvalidAddress,
}

///////////////////////////////////////////////////////////
/// アドレス計算トレイト
/// 
pub trait AddressAccess: From<usize> + fmt::Display {

    /// アドレスの具体値を取得
    fn adr(&self) -> Result<usize,BufferAddressErrorReason>;

    /// 次のアドレスを計算
    fn next(&self) -> Self;
}

///////////////////////////////////////////////////////////
/// アドレス
/// 
/// RootとEntityの２状態をとる
/// 
/// Root状態の場合、next()の結果は常にRootとなり、adr()の結果はエラーとなる。
/// ```
/// use exst_core::lang::value::*;
/// assert_eq!(Address::Root.next(), Address::Root);
/// ```
/// Entityの場合は、next()は次のアドレス値となり、adr()の結果は具体的なアドレス値を返す。
/// ```
/// use exst_core::lang::value::*;
/// assert_eq!(Address::Entity(1).next(), Address::Entity(2));
/// ```
/// 
#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
pub enum Address {
    Root,
    Entity(usize),
}
impl AddressAccess for Address {
    fn adr(&self) -> Result<usize,BufferAddressErrorReason> {
        match self {
            Address::Root => Result::Err(BufferAddressErrorReason::InvalidAddress),
            Address::Entity(v) => Result::Ok(*v),
        }
    }
    fn next(&self) -> Self {
        match *self {
            Address::Root => Address::Root,
            Address::Entity(a) => Address::Entity(a + 1),
        }
    }
}
impl From<usize> for Address {
    fn from(v: usize) -> Self {
        Address::Entity(v)
    }
}
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Address::Root => write!(f, "Root"),
            Address::Entity(a) => write!(f, "{}", a),
        }
    }
}

///////////////////////////////////////////////////////////
/// コードメモリ領域のアドレス
/// 
/// `Address`のラッパー
/// 
#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
pub struct CodeAddress(pub Address);
impl AddressAccess for CodeAddress {
    fn adr(&self) -> Result<usize,BufferAddressErrorReason> {
        self.0.adr()
    }
    fn next(&self) -> Self {
        CodeAddress(self.0.next())
    }
}
impl From<usize> for CodeAddress {
    fn from(v: usize) -> Self {
        CodeAddress(From::from(v))
    }
}
impl Default for CodeAddress {
    fn default() -> Self {
        CodeAddress(Address::Root)
    }
}
impl fmt::Display for CodeAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CodeAddress({})", self.0)
    }
}

///////////////////////////////////////////////////////////
/// データメモリ領域のアドレス
/// 
/// `Address`のラッパー
/// 
#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
pub struct DataAddress(pub Address);
impl AddressAccess for DataAddress {
    fn adr(&self) -> Result<usize, BufferAddressErrorReason> {
        match self.0 {
            Address::Root => Result::Err(BufferAddressErrorReason::InvalidAddress),
            Address::Entity(v) => Result::Ok(v),
        }
    }
    fn next(&self) -> Self {
        DataAddress(self.0.next())
    }
}
impl From<usize> for DataAddress {
    fn from(v: usize) -> Self {
        DataAddress(From::from(v))
    }
}
impl Default for DataAddress {
    fn default() -> Self {
        DataAddress(Address::Root)
    }
}
impl fmt::Display for DataAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataAddress({})", self.0)
    }
}

///////////////////////////////////////////////////////////
/// 環境スタック領域のアドレス
/// 
#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
pub struct EnvironmentStackAddress(pub usize);
impl From<usize> for EnvironmentStackAddress {
    fn from(v: usize) -> Self {
        EnvironmentStackAddress(v)
    }
}
impl Default for EnvironmentStackAddress {
    fn default() -> Self {
        EnvironmentStackAddress(0)
    }
}
impl fmt::Display for EnvironmentStackAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EnvironmentStackAddress({})", self.0)
    }
}

///////////////////////////////////////////////////////////
/// リターンスタック領域のアドレス
/// 
#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
pub struct ReturnStackAddress(pub usize);
impl From<usize> for ReturnStackAddress {
    fn from(v: usize) -> Self {
        ReturnStackAddress(v)
    }
}
impl Default for ReturnStackAddress {
    fn default() -> Self {
        ReturnStackAddress(0)
    }
}
impl fmt::Display for ReturnStackAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReturnStackAddress({})", self.0)
    }
}

///////////////////////////////////////////////////////////
/// データスタック領域のアドレス
/// 
#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
pub struct DataStackAddress(pub usize);
impl From<usize> for DataStackAddress {
    fn from(v: usize) -> Self {
        DataStackAddress(v)
    }
}
impl Default for DataStackAddress {
    fn default() -> Self {
        DataStackAddress(0)
    }
}
impl fmt::Display for DataStackAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataStackAddress({})", self.0)
    }
}

///////////////////////////////////////////////////////////
/// 環境スタックの相対アドレス
/// 
#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
pub struct EnvironmentStackRelativeAddress(pub usize);
impl EnvironmentStackRelativeAddress {
    pub fn next(&self) -> Self {
        EnvironmentStackRelativeAddress(self.0 + 1)
    }
}
impl From<usize> for EnvironmentStackRelativeAddress {
    fn from(v: usize) -> Self {
        EnvironmentStackRelativeAddress(v)
    }
}
impl Default for EnvironmentStackRelativeAddress {
    fn default() -> Self {
        EnvironmentStackRelativeAddress(0)
    }
}
impl fmt::Display for EnvironmentStackRelativeAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EnvironmentStackRelativeAddress({})", self.0)
    }
}

///////////////////////////////////////////////////////////
/// フレーム
/// 
#[derive(Debug,Clone,Copy,Default,Eq,PartialEq)]
pub struct CallFrame {
    /// 戻り先のコードアドレス
    return_address: CodeAddress,
    /// 環境スタックの戻り先アドレス
    stack_address: EnvironmentStackAddress,
}
impl CallFrame {

    /// コンストラクタ
    /// 
    /// # Arguments
    /// * return_address - 戻り先のコードアドレス
    /// * stack_address - 戻り先の環境スタックアドレス
    pub fn new(return_address: CodeAddress, stack_address: EnvironmentStackAddress) -> Self {
        CallFrame {
            return_address,
            stack_address,
        }
    }

    /// リターンアドレスを取得
    pub fn return_address(&self) -> CodeAddress {
        self.return_address
    }

    /// 環境スタックアドレスを取得
    pub fn stack_address(&self) -> EnvironmentStackAddress {
        self.stack_address
    }
}
impl fmt::Display for CallFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CallFrame({},{})", self.return_address, self.stack_address)
    }
}

///////////////////////////////////////////////////////////
/// ロングジャンプの時に保存するフレーム
/// 
#[derive(Debug,Clone,Copy,Default,Eq,PartialEq)]
pub struct LongJumpFrame {
    /// 戻り先のコードアドレス
    return_address: CodeAddress,
    /// リターンスタックの戻り先アドレス
    return_stack_address: ReturnStackAddress,
    /// 環境スタックの戻り先アドレス
    env_stack_address: EnvironmentStackAddress,
    /// データスタックの戻り先アドレス
    data_stack_address: DataStackAddress,
}
impl LongJumpFrame {

    /// コンストラクタ
    /// 
    /// # Arguments
    /// * return_address - 戻り先のコードアドレス
    /// * return_stack_address - 戻り先のリターンスタックアドレス
    /// * env_stack_address - 戻り先の環境スタックアドレス
    /// * data_stack_address - 戻り先の環境スタックのアドレス
    pub fn new(return_address: CodeAddress, return_stack_address: ReturnStackAddress, env_stack_address: EnvironmentStackAddress, data_stack_address: DataStackAddress) -> Self {
        LongJumpFrame {
            return_address,
            return_stack_address,
            env_stack_address,
            data_stack_address,
        }
    }

    /// リターンアドレスを取得
    pub fn return_address(&self) -> CodeAddress {
        self.return_address
    }

    /// 環境スタックアドレスを取得
    pub fn return_stack_address(&self) -> ReturnStackAddress {
        self.return_stack_address
    }
    
    /// 環境スタックアドレスを取得
    pub fn env_stack_address(&self) -> EnvironmentStackAddress {
        self.env_stack_address
    }

    /// 環境スタックアドレスを取得
    pub fn data_stack_address(&self) -> DataStackAddress {
        self.data_stack_address
    }
}
impl fmt::Display for LongJumpFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LongJumpFrame({},{},{},{})", self.return_address, self.return_stack_address, self.env_stack_address, self.data_stack_address)
    }
}

pub trait ValueTryInto<T> {
    type Error;
    fn try_into(&self) -> Result<&T,Self::Error>;
}
///////////////////////////////////////////////////////////
/// 値の変換エラー
/// 
#[derive(Debug)]
pub struct TypeMismatchError(pub &'static str, pub &'static str);
///////////////////////////////////////////////////////////
/// 値
/// 
#[derive(Debug,Eq,PartialEq,Ord,PartialOrd)]
pub enum Value<T> {
    /// 整数値
    IntValue(i32),
    /// 文字列値
    StrValue(String),
    /// コードアドレス
    CodeAddress(CodeAddress),
    /// データアドレス
    DataAddress(DataAddress),
    /// 拡張値
    ExtValue(T),
    /// 空（デフォルト値）
    Empty,
}
impl<T> Default for Value<T> {
    fn default() -> Self {
        Self::Empty
    }
}
impl<T> Value<T> {
    /// 型名を文字列で取得
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::IntValue(_) => int_type_name(),
            Self::StrValue(_) => str_type_name(),
            Self::CodeAddress(_) => code_address_type_name(),
            Self::DataAddress(_) => data_address_type_name(),
            Self::ExtValue(_) => extra_type_name(),
            Self::Empty => empty_type_name(),
        }
    }
}
impl<T> fmt::Display for Value<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IntValue(v) => write!(f, "{}", v),
            Self::StrValue(v) => write!(f, "{}", v),
            Self::CodeAddress(v) => write!(f, "Value({})", v),
            Self::DataAddress(v) => write!(f, "Value({})", v),
            Self::ExtValue(v) => write!(f, "{}", v),
            Self::Empty => write!(f, "Value(EMPTY)"),
        }
    }
}
impl<T> ValueTryInto<i32> for Value<T>
    where T: fmt::Display
{
    type Error = TypeMismatchError;
    fn try_into(&self) -> Result<&i32,Self::Error> {
        match self {
            Value::IntValue(v) => Result::Ok(v),
            _ => Result::Err(TypeMismatchError(int_type_name(), self.type_name())),
        }
    }
}
impl<T> ValueTryInto<String> for Value<T>
    where T: fmt::Display
{
    type Error = TypeMismatchError;
    fn try_into(&self) -> Result<&String,Self::Error> {
        match self {
            Value::StrValue(v) => Result::Ok(v),
            _ => Result::Err(TypeMismatchError(str_type_name(), self.type_name())),
        }
    }
}
impl<T> ValueTryInto<CodeAddress> for Value<T>
    where T: fmt::Display
{
    type Error = TypeMismatchError;
    fn try_into(&self) -> Result<&CodeAddress,Self::Error> {
        match self {
            Value::CodeAddress(v) => Result::Ok(v),
            _ => Result::Err(TypeMismatchError(code_address_type_name(), self.type_name())),
        }
    }
}
impl<T> ValueTryInto<DataAddress> for Value<T>
    where T: fmt::Display
{
    type Error = TypeMismatchError;
    fn try_into(&self) -> Result<&DataAddress,Self::Error> {
        match self {
            Value::DataAddress(v) => Result::Ok(v),
            _ => Result::Err(TypeMismatchError(data_address_type_name(), self.type_name())),
        }
    }
}
impl<T> Value<T>
    where T: fmt::Display
{
    pub fn try_into_usize(&self) -> Result<usize,TypeMismatchError> {
        match self {
            Value::IntValue(v) => {
                Result::Ok(*v as usize)
            },
            _ => Result::Err(TypeMismatchError(int_type_name(), self.type_name())),
        }
    }
}

/// 整数型名を返す
pub fn int_type_name() -> &'static str { "INT" }
/// 文字列型名を返す
pub fn str_type_name() -> &'static str { "STR" }
/// コードアドレス型名を返す
pub fn code_address_type_name() -> &'static str { "CODE_ADDRESS" }
/// データアドレス型名を返す
pub fn data_address_type_name() -> &'static str { "DATA_ADDRESS" }
/// 拡張型名を返す
pub fn extra_type_name() -> &'static str { "EXTRA" }
/// 空型名を返す
pub fn empty_type_name() -> &'static str { "EMPTY" }

///////////////////////////////////////////////////////////
/// デバッグ用のラベル
#[derive(Debug,Clone,Copy)]
pub enum DebugLabel {
    /// ワード定義の終了
    WordTerminator,
}
impl fmt::Display for DebugLabel
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DebugLabel::WordTerminator => write!(f, "WordTerminator"),
        }
    }
}
///////////////////////////////////////////////////////////
/// Trap理由
#[derive(Debug,Clone,Copy)]
pub enum TrapReason {
    /// コンパイル途中の命令の実行
    DummyInstructionExecution,
    /// 明示的なTrapの実行
    UserTrap,
}
impl fmt::Display for TrapReason
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrapReason::DummyInstructionExecution => write!(f, "DummyInstructionExecution"),
            TrapReason::UserTrap => write!(f, "UserTrap"),
        }
    }
}
///////////////////////////////////////////////////////////
/// 命令
/// 
pub enum Instruction<T,V,E> 
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, V: VmManipulation<ExtraValueType=T>
{
    /// PUSH命令 - 値のPUSH
    Push(Rc<Value<T>>),
    /// CALL命令 - ワードの呼び出し
    Call(CodeAddress),
    /// CALL_PRIMITIVE命令 - プリミティブ関数の呼び出し
    CallPrimitive(fn(&mut V) -> Result<(),E>),
    /// RETURN命令 - ワードの呼び出しからリターン
    Return,
    /// ローカル変数の参照
    LocalRef(EnvironmentStackRelativeAddress),
    /// NOP命令 - 何もしない
    Nop,
    /// Trap命令 - 異常終了させる
    Trap(TrapReason),
    /// デバッグ用のラベル
    DebugLabel(DebugLabel),
    /// Branch命令 - 条件分岐。stack topが0以外の場合、分岐する
    Branch(CodeAddress),
    /// Jump命令 - 無条件条件分岐
    Jump(CodeAddress),
    /// stack topのCodeAddressを呼び出す
    Exec,
    /// SetJump命令 - LongJumpのジャンプ先を登録
    SetJump(CodeAddress),
    /// LongJump命令 - 直近のSetJumpされた場所へジャンプ
    LongJump,
    /// PopJump命令 - LongJumpスタックからPop
    PopJump,
}
impl<T,V,E> Default for Instruction<T,V,E>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, V: VmManipulation<ExtraValueType=T>
{
    fn default() -> Self {
        Self::Nop
    }
}
impl<T,V,E> std::clone::Clone for Instruction<T,V,E>
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, V: VmManipulation<ExtraValueType=T>
{
    fn clone(&self) -> Self {
        match *self {
            Self::Push(ref v) => Self::Push(v.clone()),
            Self::Call(ref a) => Self::Call(a.clone()),
            Self::CallPrimitive(ref f) => Self::CallPrimitive(f.clone()),
            Self::Return => Self::Return,
            Self::LocalRef(ref a) => Self::LocalRef(a.clone()),
            Self::Nop => Self::Nop,
            Self::Trap(ref v) => Self::Trap(v.clone()),
            Self::DebugLabel(ref v) => Self::DebugLabel(v.clone()),
            Self::Branch(ref a) => Self::Branch(a.clone()),
            Self::Jump(ref a) => Self::Jump(a.clone()),
            Self::Exec => Self::Exec,
            Self::SetJump(ref a) => Self::SetJump(a.clone()),
            Self::LongJump => Self::LongJump,
            Self::PopJump => Self::PopJump,
        }
    }
}
impl<T,V,E> fmt::Display for Instruction<T,V,E>
    where V: VmManipulation<ExtraValueType=T>,
    T: fmt::Display + PartialEq + Eq + PartialOrd + Ord
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Push(v) => write!(f, "Push({})", v),
            Self::Call(a) => write!(f, "Call({})", a),
            Self::CallPrimitive(_) => write!(f, "CallPrimitive(_)"),
            Self::Return => write!(f, "Return"),
            Self::LocalRef(a) => write!(f, "LocalRef({})", a),
            Self::Nop => write!(f, "Nop"),
            Self::Trap(v) => write!(f, "Trap({})", v),
            Self::DebugLabel(v) => write!(f, "DebugLabel({})", v),
            Self::Branch(a) => write!(f, "Branch({})", a),
            Self::Jump(a) => write!(f, "Jump({})", a),
            Self::Exec => write!(f, "Exec"),
            Self::SetJump(a) => write!(f, "SetJump({})", a),
            Self::LongJump => write!(f, "LongJump"),
            Self::PopJump => write!(f, "PopJump"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_address() {
        assert_eq!(Address::Root.next(), Address::Root);
        assert_eq!(Address::Entity(0).next(), Address::Entity(1));
        assert_eq!(Address::Entity(1).next(), From::from(2));
        assert_eq!(Address::Entity(0).next().adr().unwrap(), 1);
        assert_eq!(Address::Entity(1).next().adr().unwrap(), 2);
        assert_eq!(
            match Address::Root.adr() {
                Result::Err(BufferAddressErrorReason::InvalidAddress) => true,
                _ => false,
            }
            ,true
        );
    }

    #[test]
    fn test_code_address() {
        assert_eq!(CodeAddress(Address::Root.next()), CodeAddress(Address::Root));
        assert_eq!(CodeAddress(Address::Entity(0)).next(), CodeAddress(Address::Entity(1)));
        assert_eq!(CodeAddress(Address::Entity(1)).next(), From::from(2));
        assert_eq!(CodeAddress(Address::Entity(0)).next().adr().unwrap(), 1);
        assert_eq!(CodeAddress(Address::Entity(1)).next().adr().unwrap(), 2);
        assert_eq!(
            match CodeAddress(Address::Root).adr() {
                Result::Err(BufferAddressErrorReason::InvalidAddress) => true,
                _ => false,
            }
            ,true
        );
    }

    #[test]
    fn test_data_address() {
        assert_eq!(DataAddress(Address::Root.next()), DataAddress(Address::Root));
        assert_eq!(DataAddress(Address::Entity(0)).next(), DataAddress(Address::Entity(1)));
        assert_eq!(DataAddress(Address::Entity(1)).next(), From::from(2));
        assert_eq!(DataAddress(Address::Entity(0)).next().adr().unwrap(), 1);
        assert_eq!(DataAddress(Address::Entity(1)).next().adr().unwrap(), 2);
        assert_eq!(
            match DataAddress(Address::Root).adr() {
                Result::Err(BufferAddressErrorReason::InvalidAddress) => true,
                _ => false,
            }
            ,true
        );
    }

    #[test]
    fn test_environment_stack_address() {
        assert_eq!(EnvironmentStackAddress(0), Default::default());
        assert_eq!(EnvironmentStackAddress(1), From::from(1));
    }

    #[test]
    fn test_call_frame() {
        assert_eq!(CallFrame::new(From::from(123), From::from(456)).return_address(), From::from(123));
        assert_eq!(CallFrame::new(From::from(123), From::from(456)).stack_address(), From::from(456));
    }

    #[test]
    fn test_value() {
        assert_eq!(Value::<f32>::IntValue(100).type_name(), "INT");
        assert_eq!(Value::<f32>::StrValue("a".to_string()).type_name(), "STR");
        assert_eq!(Value::<f32>::CodeAddress(From::from(100)).type_name(), "CODE_ADDRESS");
        assert_eq!(Value::<f32>::DataAddress(From::from(100)).type_name(), "DATA_ADDRESS");
        assert_eq!(Value::<f32>::Empty.type_name(), "EMPTY");
        assert_eq!(Value::<f32>::ExtValue(1.1).type_name(), "EXTRA");
        assert_eq!(Value::<f32>::Empty, Default::default());
    }
}