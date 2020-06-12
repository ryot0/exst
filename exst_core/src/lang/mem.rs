//! メモリ領域
//! 
//! # Usage Examples
//! ```
//! use exst_core::lang::mem::*;
//! let mut buf = BufferMemory::<i32>::new();
//! assert_eq!(buf.push(11), 1);
//! assert_eq!(buf.push(12), 2);
//! assert_eq!(buf.push(13), 3);
//! assert_eq!(buf.push(14), 4);
//! assert_eq!(buf.push(15), 5);
//! assert_eq!(buf.push(16), 6);
//! //メモリサイズ
//! assert_eq!(buf.here(), 6);
//! 
//! // stack topを取得して破棄
//! assert_eq!(buf.pop().unwrap(), 16);
//! //一個破棄したので、メモリサイズが１小さくなる
//! assert_eq!(buf.here(), 5);
//! 
//! //指定したアドレスの値を書き換え
//! assert_eq!(buf.set(2, 23).unwrap(), 13);
//! assert_eq!(buf.get(2).unwrap(), 23);
//! 
//! //stack top を0とするindexで指定し、値をコピーしてpushする
//! buf.pick(1).unwrap();
//! assert_eq!(buf.here(), 6);
//! assert_eq!(buf.get(4).unwrap(), 15);
//! assert_eq!(buf.get(5).unwrap(), 14);
//! 
//! //stack topを0とするindexで指定し、指定した値をstack topに持ってくるようにローテートする
//! buf.roll(3).unwrap();
//! assert_eq!(buf.here(), 6);
//! assert_eq!(buf.get(0).unwrap(), 11);
//! assert_eq!(buf.get(1).unwrap(), 12);
//! assert_eq!(buf.get(2).unwrap(), 14);
//! assert_eq!(buf.get(3).unwrap(), 15);
//! assert_eq!(buf.get(4).unwrap(), 14);
//! assert_eq!(buf.get(5).unwrap(), 23);
//! 
//! //メモリを追加
//! //追加した領域はdefault()で埋める
//! buf.allocate(2);
//! assert_eq!(buf.here(), 8);
//! assert_eq!(buf.get(6).unwrap(), 0);
//! assert_eq!(buf.get(7).unwrap(), 0);
//! 
//! //stack topからの削除個数を指定して削除
//! buf.remove(3).unwrap();
//! assert_eq!(buf.here(), 5);
//! assert_eq!(buf.get(4).unwrap(), 14);
//! 
//! //hereが指定した値になるように巻き戻す
//! buf.rollback(2).unwrap();
//! assert_eq!(buf.here(), 2);
//! assert_eq!(buf.get(1).unwrap(), 12);
//! ```
//! 

use super::value::*;
use super::vm::*;
use std::vec::*;
use std::rc::Rc;
use std::fmt;

///////////////////////////////////////////////////////////
/// メモリアクセスエラー
/// 
#[derive(Debug,Clone,Copy)]
pub enum BufferErrorReason {
    /// 範囲外アクセス
    /// OutOfRangeAccess(メモリサイズ（len）, アクセス時のアドレス)
    OutOfRangeAccess(usize,usize),
    /// アンダーフローエラー
    UnderflowAccessError,
    /// ロールバック操作自のエラー
    /// InvalidRollbackPosition(メモリサイズ（len）, ロールバック位置)
    InvalidRollbackPosition(usize,usize),
}

///////////////////////////////////////////////////////////
/// メモリ領域
/// 
pub struct BufferMemory<T>
    where
        T: std::default::Default + std::clone::Clone,
{
    /// バッファの実態
    buffer: Vec<T>,
}
impl<T> BufferMemory<T>
    where
        T: std::default::Default + std::clone::Clone,
{
    /// コンストラクタ
    /// 
    /// メモリサイズ0で初期化した状態で作成する
    /// 
    pub fn new() -> Self {
        BufferMemory {
            buffer: Vec::new(),
        }
    }
}
impl<T> BufferMemory<T>
    where
        T: std::default::Default + std::clone::Clone,
{

    /// stack操作時のindexに変換する
    /// 
    /// # Arguments
    /// * pos - stack位置（stack topが0）
    /// 
    /// # Return Values
    /// buffer内の位置を指すaddress
    /// 
    fn to_index(&self, pos: usize) -> usize {
        self.buffer.len() - pos - 1
    }

    /// stack topをコピーして返す
    /// 
    /// # Return Values
    /// stack topの値。サイズが0の場合は、`UnderflowAccessError`
    /// 
    pub fn peek(&self) -> Result<T,BufferErrorReason> {
        match self.buffer.last() {
            Some(v) => Result::Ok(v.clone()),
            None => Result::Err(BufferErrorReason::UnderflowAccessError),
        }
    }

    /// stack topを返却して破棄する
    /// 
    /// # Return Values
    /// stack topの値。サイズが0の場合は、`UnderflowAccessError`
    /// 
    pub fn pop(&mut self) -> Result<T,BufferErrorReason> {
        match self.buffer.pop() {
            Some(v) => Result::Ok(v),
            None => Result::Err(BufferErrorReason::UnderflowAccessError),
        }
    }

    /// 指定した位置（stack topを0としたときの位置）の値をコピーしてpushする
    /// 
    /// # Arguments
    /// * pos - スタック位置（stack topを0として、stackの奥にいくほど値が大きくなる）
    ///
    pub fn pick(&mut self, pos: usize) -> Result<(),BufferErrorReason> {
        if pos < self.buffer.len() {
            match self.buffer.get(self.to_index(pos)).cloned() {
                Some(v) => Result::Ok({
                    self.buffer.push(v.clone());
                }),
                None => Result::Err(BufferErrorReason::OutOfRangeAccess(self.buffer.len(), pos)),
            }
        } else {
            Result::Err(BufferErrorReason::OutOfRangeAccess(self.buffer.len(), pos))
        }
    }

    /// 指定した位置（stack topを0とした時の位置）の値が先頭にくるように値をローテーションする
    /// 
    /// # Arguments
    /// * pos - スタック位置（stack topを0として、stackの奥にいくほど値が大きくなる）
    ///
    pub fn roll(&mut self, pos: usize) -> Result<(),BufferErrorReason> {
        if pos < self.buffer.len() {
            //対象となるデータを一旦退避
            let mut tmp: Vec<T> = Vec::new();
            for idx in 0..pos+1 {
                match self.buffer.get(self.to_index(idx)) {
                    Some(v) => {
                        tmp.push(v.clone());
                    },
                    None => {
                        return Result::Err(BufferErrorReason::OutOfRangeAccess(self.buffer.len(), pos));
                    }
                }
            }
            //対象となるデータを削除
            for _ in 0..pos+1 {
                self.buffer.pop();
            }
            let top = tmp.pop().unwrap();
            //一番上以外をpush
            while let Some(v) = tmp.pop() {
                self.buffer.push(v);
            }
            //一番上を最後にpush
            Result::Ok(self.buffer.push(top))
        } else {
            Result::Err(BufferErrorReason::OutOfRangeAccess(self.buffer.len(), pos))
        }
    }

    /// stack topから指定した個数分の領域を削除する
    /// 
    /// # Arguments
    /// * num - 削除する個数
    /// 
    /// # Return Values
    /// 削除後のメモリサイズ
    /// 
    pub fn remove(&mut self, num: usize) -> Result<usize,BufferErrorReason> {
        if num <= self.buffer.len() {
            self.buffer.resize_with(self.buffer.len() - num, Default::default);
            Result::Ok(self.buffer.len())
        } else {
            Result::Err(BufferErrorReason::UnderflowAccessError)
        }
    }

    /// 指定したアドレスに巻き戻す。ただし、stack topが指定したアドレスになるわけではなく、全体のメモリサイズが指定した値になる
    /// 
    /// # Arguments
    /// * address - 巻き戻す先のアドレス
    /// 
    /// # Return Values
    /// 削除したメモリ領域の個数
    /// 
    pub fn rollback(&mut self, address: usize) -> Result<usize,BufferErrorReason> {
        if address <= self.buffer.len() {
            let old_len = self.buffer.len();
            self.buffer.resize_with(address, Default::default);
            Result::Ok(old_len - address)
        } else {
            Result::Err(BufferErrorReason::InvalidRollbackPosition(self.buffer.len(), address))
        }
    }

    /// 指定したアドレスの値を取得する
    /// 
    /// # Arguments
    /// * address - アドレス
    /// 
    /// # Return Values
    /// 値。範囲外の値を取得しようとした場合、`BufferErrorReason::OutOfRangeAccess`エラー
    /// 
    pub fn get(&self, address: usize) -> Result<T,BufferErrorReason> {
        match self.buffer.get(address) {
            Some(v) => Result::Ok(v.clone()),
            None => Result::Err(BufferErrorReason::OutOfRangeAccess(self.buffer.len(), address)),
        }
    }

    /// すでに存在するメモリの内容を書き換える
    /// 
    /// # Arguments
    /// * address - 変更するアドレス
    /// * value - 変更後の値
    /// 
    /// # Return Values
    /// 変更前の値。範囲外のアドレスを書き換えようと下場合、`BufferErrorReason::OutOfRangeAccess`エラー
    /// 
    pub fn set(&mut self, address: usize, value: T) -> Result<T,BufferErrorReason> {
        match self.buffer.get_mut(address) {
            Some(v) => Result::Ok(std::mem::replace(v, value)),
            None => Result::Err(BufferErrorReason::OutOfRangeAccess(self.buffer.len(), address)),
        }
    }

    /// メモリの末尾に値を追加する
    /// 
    /// # Arguments
    /// * value - 追加する値
    /// 
    /// # Return Values
    /// 追加後のメモリサイズ
    /// 
    pub fn push(&mut self, value: T) -> usize {
        self.buffer.push(value);
        self.buffer.len()
    }

    /// メモリを追加する
    /// 
    /// # Arguments
    /// * size - 追加するサイズ
    /// 
    /// # Return Values
    /// 追加後のメモリサイズ
    /// 
    pub fn allocate(&mut self, size: usize) -> usize {
        self.buffer.resize_with(self.buffer.len() + size, Default::default);
        self.buffer.len()
    }

    /// メモリサイズを取得する
    /// 
    /// # Return Values
    /// * メモリサイズ
    /// 
    pub fn here(&self) -> usize {
        self.buffer.len()
    }
}
impl<T> fmt::Display for BufferMemory<T> 
    where T: std::default::Default + std::clone::Clone + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[")?;
        for (i, e) in self.buffer.iter().enumerate() {
            writeln!(f, "  <{},{}>", i, e)?
        }
        writeln!(f, "]<-StackTop")
    }
}

///////////////////////////////////////////////////////////
/// コードバッファのエラー
/// 
/// `BufferErrorReason`のラッパー
/// 
#[derive(Debug,Clone,Copy)]
pub enum CodeBufferErrorReason {
    BufferAccessError(BufferErrorReason),
    BufferAddressError(BufferAddressErrorReason),
}
impl From<BufferErrorReason> for CodeBufferErrorReason {
    fn from(err: BufferErrorReason) -> Self {
        CodeBufferErrorReason::BufferAccessError(err)
   }
}
impl From<BufferAddressErrorReason> for CodeBufferErrorReason {
    fn from(err: BufferAddressErrorReason) -> Self {
        CodeBufferErrorReason::BufferAddressError(err)
    }
}
///////////////////////////////////////////////////////////
/// コードバッファ
/// 
/// `BufferMemory`のラッパー
/// 
pub struct CodeBuffer<T,V,E>(BufferMemory<Instruction<T,V,E>>)
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, V: VmManipulation<ExtraValueType=T>
;
impl<T,V,E> CodeBuffer<T,V,E> 
    where T: fmt::Display + PartialEq + Eq + PartialOrd + Ord, V: VmManipulation<ExtraValueType=T>
{
    pub fn new() -> Self {
        CodeBuffer(
            BufferMemory::new()
        )
    }
    pub fn get(&self, address: CodeAddress) -> Result<Instruction<T,V,E>,CodeBufferErrorReason> {
        let address = address.adr()?;
        self.0.get(address).map_err(From::from)
    }
    pub fn set(&mut self, address: CodeAddress, value: Instruction<T,V,E>) -> Result<Instruction<T,V,E>,CodeBufferErrorReason> {
        let address = address.adr()?;
        self.0.set(address, value).map_err(From::from)
    }
    pub fn pop(&mut self) -> Result<Instruction<T,V,E>,CodeBufferErrorReason> {
        self.0.pop().map_err(From::from)
    }
    pub fn push(&mut self, value: Instruction<T,V,E>) -> CodeAddress {
        From::from(self.0.push(value))
    }
    pub fn allocate(&mut self, size: usize) -> CodeAddress {
        From::from(self.0.allocate(size))
    }
    pub fn here(&self) -> CodeAddress {
        From::from(self.0.here())
    }
}
impl<T,V,E> fmt::Display for CodeBuffer<T,V,E> 
    where V: VmManipulation<ExtraValueType=T>,
    T: fmt::Display + PartialEq + Eq + PartialOrd + Ord
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CodeBuffer{}", self.0)
    }
}

///////////////////////////////////////////////////////////
/// データバッファのエラー
/// 
/// `BufferErrorReason`のラッパー
/// 
#[derive(Debug,Clone,Copy)]
pub enum DataBufferErrorReason {
    BufferAccessError(BufferErrorReason),
    BufferAddressError(BufferAddressErrorReason),
}
impl From<BufferErrorReason> for DataBufferErrorReason {
    fn from(err: BufferErrorReason) -> Self {
        DataBufferErrorReason::BufferAccessError(err)
   }
}
impl From<BufferAddressErrorReason> for DataBufferErrorReason {
    fn from(err: BufferAddressErrorReason) -> Self {
        DataBufferErrorReason::BufferAddressError(err)
    }
}
///////////////////////////////////////////////////////////
/// データバッファ
/// 
/// `BufferMemory`のラッパー
/// 
pub struct DataBuffer<T>(BufferMemory<Rc<Value<T>>>);
impl<T> DataBuffer<T> {
    pub fn new() -> Self {
        DataBuffer(
            BufferMemory::new()
        )
    }
    pub fn get(&self, address: DataAddress) -> Result<Rc<Value<T>>,DataBufferErrorReason> {
        let address = address.adr()?;
        self.0.get(address).map_err(From::from)
    }
    pub fn set(&mut self, address: DataAddress, value: Rc<Value<T>>) -> Result<Rc<Value<T>>,DataBufferErrorReason> {
        let address = address.adr()?;
        self.0.set(address, value).map_err(From::from)
    }
    pub fn push(&mut self, value: Rc<Value<T>>) -> DataAddress {
        From::from(self.0.push(value))
    }
    pub fn allocate(&mut self, size: usize) -> DataAddress {
        From::from(self.0.allocate(size))
    }
    pub fn here(&self) -> DataAddress {
        From::from(self.0.here())
    }
}
impl<T> fmt::Display for DataBuffer<T> 
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataBuffer{}", self.0)
    }
}

///////////////////////////////////////////////////////////
/// コードスタックのエラー
/// 
/// `BufferErrorReason`のラッパー
/// 
#[derive(Debug,Clone,Copy)]
pub enum DataStackErrorReason {
    BufferAccessError(BufferErrorReason),
}
impl From<BufferErrorReason> for DataStackErrorReason {
    fn from(err: BufferErrorReason) -> Self {
        DataStackErrorReason::BufferAccessError(err)
   }
}
///////////////////////////////////////////////////////////
/// データスタック
/// 
/// `BufferMemory`のラッパー
/// 
pub struct DataStack<T>(BufferMemory<Rc<Value<T>>>);
impl<T> DataStack<T> {
    pub fn new() -> Self {
        DataStack(
            BufferMemory::new()
        )
    }
    pub fn push(&mut self, value: Rc<Value<T>>) {
        self.0.push(value);
    }
    pub fn peek(&self) -> Result<Rc<Value<T>>,DataStackErrorReason> {
        self.0.peek().map_err(From::from)
    }
    pub fn pop(&mut self) -> Result<Rc<Value<T>>,DataStackErrorReason> {
        self.0.pop().map_err(From::from)
    }
    pub fn pick(&mut self, pos: usize) -> Result<(),DataStackErrorReason> {
        self.0.pick(pos).map_err(From::from)
    }
    pub fn roll(&mut self, pos: usize) -> Result<(),DataStackErrorReason> {
        self.0.roll(pos).map_err(From::from)
    }
    pub fn here(&self) -> DataStackAddress {
        From::from(self.0.here())
    }
    pub fn rollback(&mut self, address: DataStackAddress) -> Result<(),DataStackErrorReason> {
        self.0.roll(address.0).map_err(From::from)
    }
}
impl<T> fmt::Display for DataStack<T> 
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataStack{}", self.0)
    }
}

///////////////////////////////////////////////////////////
/// コントロールスタックのエラー
/// 
/// `BufferErrorReason`のラッパー
/// 
#[derive(Debug,Clone,Copy)]
pub enum ControlflowStackErrorReason {
    BufferAccessError(BufferErrorReason),
}
impl From<BufferErrorReason> for ControlflowStackErrorReason {
    fn from(err: BufferErrorReason) -> Self {
        ControlflowStackErrorReason::BufferAccessError(err)
   }
}
///////////////////////////////////////////////////////////
/// コントロールフロースタック
/// 
/// `BufferMemory`のラッパー
/// 
pub struct ControlflowStack<T>(BufferMemory<Rc<Value<T>>>);
impl<T> ControlflowStack<T> {
    pub fn new() -> Self {
        ControlflowStack(
            BufferMemory::new()
        )
    }
    pub fn push(&mut self, value: Rc<Value<T>>) {
        self.0.push(value);
    }
    pub fn peek(&self) -> Result<Rc<Value<T>>,DataStackErrorReason> {
        self.0.peek().map_err(From::from)
    }
    pub fn pop(&mut self) -> Result<Rc<Value<T>>,ControlflowStackErrorReason> {
        self.0.pop().map_err(From::from)
    }
    pub fn pick(&mut self, pos: usize) -> Result<(),ControlflowStackErrorReason> {
        self.0.pick(pos).map_err(From::from)
    }
    pub fn roll(&mut self, pos: usize) -> Result<(),ControlflowStackErrorReason> {
        self.0.roll(pos).map_err(From::from)
    }
    pub fn here(&self) -> usize {
        From::from(self.0.here())
    }
}
impl<T> fmt::Display for ControlflowStack<T> 
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ControlFlowStack{}", self.0)
    }
}

///////////////////////////////////////////////////////////
/// 環境スタックのエラー
/// 
/// `BufferErrorReason`のラッパー
/// 
#[derive(Debug,Clone,Copy)]
pub enum EnvironmentStackErrorReason {
    BufferAccessError(BufferErrorReason),
}
impl From<BufferErrorReason> for EnvironmentStackErrorReason {
    fn from(err: BufferErrorReason) -> Self {
        EnvironmentStackErrorReason::BufferAccessError(err)
   }
}
///////////////////////////////////////////////////////////
/// 環境スタック
/// 
/// `BufferMemory`のラッパー
/// 
pub struct EnvironmentStack<T>(BufferMemory<Rc<Value<T>>>);
impl<T> EnvironmentStack<T> {
    pub fn new() -> Self {
        EnvironmentStack(
            BufferMemory::new()
        )
    }
    pub fn push(&mut self, value: Rc<Value<T>>) {
        self.0.push(value);
    }
    pub fn peek(&self) -> Result<Rc<Value<T>>,DataStackErrorReason> {
        self.0.peek().map_err(From::from)
    }
    pub fn pop(&mut self) -> Result<Rc<Value<T>>,EnvironmentStackErrorReason> {
        self.0.pop().map_err(From::from)
    }
    pub fn pick(&mut self, pos: usize) -> Result<(),EnvironmentStackErrorReason> {
        self.0.pick(pos).map_err(From::from)
    }
    pub fn roll(&mut self, pos: usize) -> Result<(),EnvironmentStackErrorReason> {
        self.0.roll(pos).map_err(From::from)
    }
    pub fn rollback(&mut self, address: EnvironmentStackAddress) -> Result<EnvironmentStackAddress,EnvironmentStackErrorReason> {
        let r = self.0.rollback(address.0)?;
        Result::Ok(From::from(r))
    }
    pub fn here(&self) -> EnvironmentStackAddress {
        From::from(self.0.here())
    }

    /// 指定下位置の値を取得
    /// 
    /// # Arguments
    /// * base - 基準アドレス
    /// * pos - 基準アドレスからの相対アドレス
    /// 
    /// # Return Values
    /// 値
    /// 
    pub fn get(&self, base: EnvironmentStackAddress, pos: EnvironmentStackRelativeAddress) -> Result<Rc<Value<T>>,EnvironmentStackErrorReason> {
        self.0.get(From::from(base.0 + pos.0)).map_err(From::from)
    }

    /// 指定下位置の値を設定
    /// 
    /// # Arguments
    /// * base - 基準アドレス
    /// * pos - 基準アドレスからの相対アドレス
    /// * value - 値
    /// 
    pub fn set(&mut self, base: EnvironmentStackAddress, pos: EnvironmentStackRelativeAddress, value: Rc<Value<T>>) -> Result<Rc<Value<T>>,EnvironmentStackErrorReason> {
        self.0.set(From::from(base.0 + pos.0), value).map_err(From::from)
    }
}
impl<T> fmt::Display for EnvironmentStack<T> 
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EnvironmentStack{}", self.0)
    }
}

///////////////////////////////////////////////////////////
/// リターンスタックのエラー
/// 
/// `BufferErrorReason`のラッパー
/// 
#[derive(Debug,Clone,Copy)]
pub enum ReturnStackErrorReason {
    BufferAccessError(BufferErrorReason),
}
impl From<BufferErrorReason> for ReturnStackErrorReason {
    fn from(err: BufferErrorReason) -> Self {
        ReturnStackErrorReason::BufferAccessError(err)
   }
}
///////////////////////////////////////////////////////////
/// リターンスタック
/// 
/// `BufferMemory`のラッパー
/// 
pub struct ReturnStack(BufferMemory<CallFrame>);
impl ReturnStack {
    pub fn new() -> Self {
        ReturnStack(
            BufferMemory::new()
        )
    }
    pub fn peek(&self) -> Result<CallFrame,ReturnStackErrorReason> {
        self.0.peek().map_err(From::from)
    }
    pub fn push(&mut self, return_address: CodeAddress, stack_address: EnvironmentStackAddress) {
        self.0.push(CallFrame::new(return_address, stack_address));
    }
    pub fn pop(&mut self) -> Result<CallFrame,ReturnStackErrorReason> {
        self.0.pop().map_err(From::from)
    }
    pub fn here(&self) -> ReturnStackAddress {
        From::from(self.0.here())
    }
    pub fn get(&self, address: ReturnStackAddress) -> Result<CallFrame,ReturnStackErrorReason> {
        self.0.get(address.0).map_err(From::from)
    }
    pub fn rollback(&mut self, address: ReturnStackAddress) -> Result<(),ReturnStackErrorReason> {
        self.0.roll(address.0).map_err(From::from)
    }
}
impl fmt::Display for ReturnStack
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReturnStack{}", self.0)
    }
}

///////////////////////////////////////////////////////////
/// LongJumpStackのエラー
/// 
/// `BufferErrorReason`のラッパー
/// 
#[derive(Debug,Clone,Copy)]
pub enum LongJumpStackErrorReason {
    BufferAccessError(BufferErrorReason),
}
impl From<BufferErrorReason> for LongJumpStackErrorReason {
    fn from(err: BufferErrorReason) -> Self {
        LongJumpStackErrorReason::BufferAccessError(err)
   }
}
///////////////////////////////////////////////////////////
/// LongJumpStack
/// 
/// `BufferMemory`のラッパー
/// 
pub struct LongJumpStack(BufferMemory<LongJumpFrame>);
impl LongJumpStack {
    pub fn new() -> Self {
        LongJumpStack(
            BufferMemory::new()
        )
    }
    pub fn peek(&self) -> Result<LongJumpFrame,LongJumpStackErrorReason> {
        self.0.peek().map_err(From::from)
    }
    pub fn push(&mut self, return_address: CodeAddress, return_stack_address: ReturnStackAddress, env_stack_address: EnvironmentStackAddress, data_stack_address: DataStackAddress) {
        self.0.push(LongJumpFrame::new(return_address, return_stack_address, env_stack_address, data_stack_address));
    }
    pub fn pop(&mut self) -> Result<LongJumpFrame,LongJumpStackErrorReason> {
        self.0.pop().map_err(From::from)
    }
    pub fn here(&self) -> usize {
        From::from(self.0.here())
    }
    pub fn get(&self, address: ReturnStackAddress) -> Result<LongJumpFrame,LongJumpStackErrorReason> {
        self.0.get(address.0).map_err(From::from)
    }
    pub fn rollback(&mut self, address: ReturnStackAddress) -> Result<(),LongJumpStackErrorReason> {
        self.0.roll(address.0).map_err(From::from)
    }
}
impl fmt::Display for LongJumpStack
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LongJumpStack{}", self.0)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_stack() {

        let mut buf = BufferMemory::<i32>::new();

        //値を２こpush
        assert_eq!(buf.here(), 0);
        buf.push(10);
        assert_eq!(buf.here(), 1);
        buf.push(20);
        assert_eq!(buf.here(), 2);

        //getで参照
        assert_eq!(buf.get(0).unwrap(), 10);
        assert_eq!(buf.get(1).unwrap(), 20);
        //getでアクセス範囲エラー
        assert_eq!(
            match buf.get(2).unwrap_err() {
                BufferErrorReason::OutOfRangeAccess(2, 2) => true,
                _ => false,
            }
            ,true
        );

        //setで書き換え
        assert_eq!(buf.set(1, 21).unwrap(), 20);
        assert_eq!(buf.get(1).unwrap(), 21);
        //setでアクセス範囲エラー
        assert_eq!(
            match buf.set(2, 10).unwrap_err() {
                BufferErrorReason::OutOfRangeAccess(2, 2) => true,
                _ => false,
            }
            ,true
        );

        //pop
        assert_eq!(buf.pop().unwrap(), 21);
        assert_eq!(buf.pop().unwrap(), 10);
        //popでアクセス範囲エラー
        assert_eq!(
            match buf.pop().unwrap_err() {
                BufferErrorReason::UnderflowAccessError => true,
                _ => false,
            }
            ,true
        );
    }

    #[test]
    fn test_seq_mem() {

        let mut buf = BufferMemory::<i32>::new();
        
        //メモリ確保
        assert_eq!(buf.allocate(4), 4);
        assert_eq!(buf.here(), 4);
        assert_eq!(buf.get(0).unwrap(), 0);
        assert_eq!(buf.get(1).unwrap(), 0);
        assert_eq!(buf.get(2).unwrap(), 0);
        assert_eq!(buf.get(3).unwrap(), 0);

        //内容の書き換え
        buf.set(0, 1).unwrap();
        buf.set(1, 2).unwrap();
        buf.set(2, 3).unwrap();
        buf.set(3, 4).unwrap();
        assert_eq!(buf.get(0).unwrap(), 1);
        assert_eq!(buf.get(1).unwrap(), 2);
        assert_eq!(buf.get(2).unwrap(), 3);
        assert_eq!(buf.get(3).unwrap(), 4);

        //拡張
        assert_eq!(buf.allocate(2), 6);
        assert_eq!(buf.here(), 6);
        assert_eq!(buf.get(0).unwrap(), 1);
        assert_eq!(buf.get(1).unwrap(), 2);
        assert_eq!(buf.get(2).unwrap(), 3);
        assert_eq!(buf.get(3).unwrap(), 4);
        assert_eq!(buf.get(4).unwrap(), 0);
        assert_eq!(buf.get(5).unwrap(), 0);

        //再拡張
        assert_eq!(buf.allocate(2), 8);
        //削除
        assert_eq!(buf.remove(3).unwrap(), 5);
        assert_eq!(buf.here(), 5);
        assert_eq!(buf.get(0).unwrap(), 1);
        assert_eq!(buf.get(1).unwrap(), 2);
        assert_eq!(buf.get(2).unwrap(), 3);
        assert_eq!(buf.get(3).unwrap(), 4);
        assert_eq!(buf.get(4).unwrap(), 0);

        //再拡張
        assert_eq!(buf.allocate(2), 7);
        //巻き戻し
        assert_eq!(buf.rollback(3).unwrap(), 4);
        assert_eq!(buf.here(), 3);
        assert_eq!(buf.get(0).unwrap(), 1);
        assert_eq!(buf.get(1).unwrap(), 2);
        assert_eq!(buf.get(2).unwrap(), 3);

        //削除のエラー
        assert_eq!(
            match buf.remove(4).unwrap_err() {
                BufferErrorReason::UnderflowAccessError => true,
                _ => false,
            }
            ,true
        );
        //巻き戻しのエラー
        assert_eq!(
            match buf.rollback(4).unwrap_err() {
                BufferErrorReason::InvalidRollbackPosition(3, 4) => true,
                _ => false,
            }
            ,true
        );
    }

    #[test]
    fn test_complex_stack() {

        let mut buf = BufferMemory::<i32>::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        buf.push(5);
        assert_eq!(buf.here(), 5);

        //pick
        buf.pick(0).unwrap();
        buf.pick(3).unwrap();
        assert_eq!(buf.here(), 7);
        assert_eq!(buf.get(0).unwrap(), 1);
        assert_eq!(buf.get(1).unwrap(), 2);
        assert_eq!(buf.get(2).unwrap(), 3);
        assert_eq!(buf.get(3).unwrap(), 4);
        assert_eq!(buf.get(4).unwrap(), 5);
        assert_eq!(buf.get(5).unwrap(), 5);
        assert_eq!(buf.get(6).unwrap(), 3);
        //pickのエラー
        assert_eq!(
            match buf.pick(7).unwrap_err() {
                BufferErrorReason::OutOfRangeAccess(7, 7) => true,
                _ => false,
            }
            ,true
        );

        //roll
        buf.roll(3).unwrap();
        assert_eq!(buf.here(), 7);
        assert_eq!(buf.get(0).unwrap(), 1);
        assert_eq!(buf.get(1).unwrap(), 2);
        assert_eq!(buf.get(2).unwrap(), 3);
        assert_eq!(buf.get(3).unwrap(), 5);
        assert_eq!(buf.get(4).unwrap(), 5);
        assert_eq!(buf.get(5).unwrap(), 3);
        assert_eq!(buf.get(6).unwrap(), 4);
        //rollのエラー
        assert_eq!(
            match buf.roll(7).unwrap_err() {
                BufferErrorReason::OutOfRangeAccess(7, 7) => true,
                _ => false,
            }
            ,true
        );

    }
}
