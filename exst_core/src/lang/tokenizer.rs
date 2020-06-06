//! charのIteratorからトークンを切り出すIterator
//! 
//! # 構文定義
//! - LF := \r | \n
//! - SEPARATOR := LF | ' ' | '\t'
//! - TokenWithComment::Comment := '#' (.*) LF 
//! - TokenValue::IntValue := [+-]?(0b|0o|0x)?[0-9]+
//! - TokenValue::String := '"' (.*) '"'
//! -- ただし、\t,\n,\r,\uxxxxはエスケープ文字として使用できる
//! - TokenValue::Symbol := 上記以外の
//! 
//! # Usage Examples
//! 
//! ## `TokenStream::next_token_with_commnet()`の使用
//! `#`から行末までをコメントとし、next_token_with_commentはコメントを含めてTokenizeした結果を順番に返却する
//! ```
//! use exst_core::lang::tokenizer::{TokenStream,ValueToken,TokenWithComment};
//! use exst_core::lang::utility::*;
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("abc #this is comment\n\"a\\u0020\\\"\\t\\\\\\r\\n\""));
//! assert_eq!(stream.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Token(ValueToken::Symbol("abc".to_string())));
//! assert_eq!(stream.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("this is comment".to_string()));
//! assert_eq!(stream.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Token(ValueToken::StrValue("a \"\t\\\r\n".to_string())));
//! assert_eq!(stream.next_token_with_comment().is_none(), true);
//! ```
//! 
//! ## `TokenStream::next()`の使用
//! Iteratorの実装。コメントはスキップする
//! ```
//! use exst_core::lang::tokenizer::{TokenStream,ValueToken};
//! use exst_core::lang::utility::*;
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("abc #this is comment\n123 0xff -89 \"abcdefg\""));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::Symbol("abc".to_string()));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(123));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(0xff));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(-89));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::StrValue("abcdefg".to_string()));
//! assert_eq!(stream.next().is_none(), true);
//! ```
//! 
//! # 構文のExamples
//! ## 文字列の構文
//! `"`で始まり、`"`で終わる文字の並びを文字列とする。
//! `"`の中で`\\`が使用された場合は、エスケープ文字として解釈する。
//! ```
//! use exst_core::lang::tokenizer::{TokenStream,ValueToken,TokenizerErrorReason};
//! use exst_core::lang::utility::*;
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("\"abc\\nde\\tf\\r\\n\\\"ZZZ\\\"\""));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::StrValue("abc\nde\tf\r\n\"ZZZ\"".to_string()));
//! //\uxxxxでユニコードエスケープ
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("\"\\u0041\\u0042\\u0043\\u0044\""));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::StrValue("ABCD".to_string()));
//! //`"`で閉じられていない場合はエラーになる
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("\"aaa"));
//! assert_eq!(stream.next().unwrap().is_err(), true);
//! //ユニコードエスケープは４桁16進数じゃなかったらエラー
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("\"\\u004\""));
//! assert_eq!(stream.next().unwrap().is_err(), true);
//! ```
//! 
//! ## 数値とシンボルの構文
//! 妥当な数値表現であれば数値としてパースする。それ以外のものは全てシンボルとしてパースする。
//! 数値表現としては符号有無／１０進数／８進数／２進数をサポート。表記中の_は無視する。
//! ```
//! use exst_core::lang::tokenizer::{TokenStream,ValueToken,TokenizerErrorReason};
//! use exst_core::lang::utility::*;
//! //10進数
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("123"));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(123));
//! //符号付10進数
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("+10"));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(10));
//! //負数
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("-99"));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(-99));
//! //16進数
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("-0xff"));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(-255));
//! //2進数
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("0b00_00_00_11"));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::IntValue(3));
//! //上記以外の表現はシンボルになる（先頭で0x指定がないので10進数として解釈され、ABCは数値と解釈されずSymbolとなる）
//! let mut stream = TokenStream::new(char_stream_from_buf_read_from_str("00ABC"));
//! assert_eq!(stream.next().unwrap().unwrap(), ValueToken::Symbol("00ABC".to_string()));
//! ```
//! 

use super::utility::*;
use std::error;
use std::fmt;
use std::io;

/// 行番号
pub type LineNumber = u32;
/// カラム番号
pub type ColumnNumber = u32;
/// TokenStreamで保持している型
pub type TokenStreamItem = Result<ValueToken,TokenizerError>;
/// TokenStreamのIteratorの型
pub type TokenStreamIterator = dyn Iterator<Item=TokenStreamItem>;

///////////////////////////////////////////////////////////
/// tokenizerの中でのエラー
#[derive(Debug)]
pub struct TokenizerError (
    /// エラーが発生した行番号
    LineNumber,
    /// エラーが発生したカラム番号
    ColumnNumber,
    /// エラー理由
    TokenizerErrorReason,
);
impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenizerError(l, c, TokenizerErrorReason::StringLiteralIsNotClosed(v)) => write!(f, "line number: {}, column number: {}, reason: StringLiteralIsNotClosed({})", l, c, v),
            TokenizerError(l, c, TokenizerErrorReason::CannotParseUnicodeEscapeChar(v)) => write!(f, "line number: {}, column number: {}, reason: CannotParseUnicodeEscapeChar({})", l, c, v),
            TokenizerError(l, c, TokenizerErrorReason::IOError(err)) => write!(f, "line number: {}, column number: {}, reason: IOError({})", l, c, err),
        }
    }
}
impl error::Error for TokenizerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.2 {
            TokenizerErrorReason::IOError(ref e) => Some(e),
            _ => None,
        }
    }
}
///////////////////////////////////////////////////////////
/// エラー理由
#[derive(Debug)]
pub enum TokenizerErrorReason {
    /// 文字列リテラルが閉じられてなかった場合  
    /// StringLiteralIsNotClosed(エラー発生直前までにパースした文字列リテラル)
    StringLiteralIsNotClosed(String),
    /// 文字列リテラル中のユニコードエスケープ（\u）がパースできなかったエラー  
    /// CannotParseUnicodeEscapeChar(エラー発生直前までにパースした文字列リテラル)
    CannotParseUnicodeEscapeChar(String),
    /// IOエラー
    IOError(std::io::Error),
}
impl From<io::Error> for TokenizerErrorReason {
    fn from(err: io::Error) -> Self {
        TokenizerErrorReason::IOError(err)
    }
}
///////////////////////////////////////////////////////////
/// パース途中の状態を表す型
#[derive(Debug, PartialEq)]
enum TokenizerStatus<V> {
    /// パースが成功した状態  
    /// Ok(パース結果の値)
    Ok(V),
    /// パースエラーではなく、別の処理にパースを委譲する場合
    Skip,
}
///////////////////////////////////////////////////////////
/// コメントを含むパース結果のToken
#[derive(Debug, PartialEq)]
pub enum TokenWithComment {
    /// コメント  
    /// Comment(コメントの文字列)
    Comment(String),
    /// トークン  
    /// Token(具体的な値)
    Token(ValueToken),
}
///////////////////////////////////////////////////////////
/// パース結果を解釈した値
#[derive(Debug, PartialEq)]
pub enum ValueToken {
    /// 整数値  
    /// IntValue(パース結果の値)
    IntValue(i32),
    /// 文字列  
    /// StrValue(パース結果の値)
    StrValue(String),
    /// シンボル  
    /// Symbol(パース結果の値)
    Symbol(String),
}
///////////////////////////////////////////////////////////
/// Token情報+デバッグ情報
pub struct Token {
    /// 行番号
    pub line_number: LineNumber,
    /// 列番号
    pub column_number: ColumnNumber,
    /// トークン情報 or エラー情報
    pub value_token: TokenStreamItem,
}
///////////////////////////////////////////////////////////
/// デバッグ情報を付与したTokenを取得するためのtrait
pub trait TokenIterator: Iterator<Item=TokenStreamItem> {

    /// デバッグ情報を付与したTokenを取得する
    /// 
    /// # Return Values
    /// デバッグ情報を付与したToken
    fn next_token(&mut self) -> Option<Token>;

    /// スクリプト名を取得する
    /// 
    /// # Return Values
    /// スクリプト名
    fn script_name(&self) -> &String;

    /// 指定された文字が出現するまでスキップする
    /// 
    /// # Arguments
    /// * end_char - この文字が出現するまで読み飛ばす
    /// 
    /// # Return Values
    /// 読み飛ばした文字列
    fn skip(&mut self, end_char: char) -> Result<String,TokenizerError>;
}
///////////////////////////////////////////////////////////
/// Iterator<Item=io::Result<char>>に値の書き戻し機能を追加した構造体
/// 
/// ```ignore
/// let s = exst_core::lang::tokenizer::InputCharStream::new("abcd".chars());
/// assert_eq!(s.next(), Some('a')); assert_eq!(s.next(), Some('b'));
/// s.push('x'); s.push('y'); //xとyをs先頭に書き戻す
/// assert_eq!(s.next(), Some('x')); assert_eq!(s.next(), Some('y'));
/// assert_eq!(s.next(), Some('c')); assert_eq!(s.next(), Some('d'));
/// assert_eq!(s.next(), None);
/// ```
struct InputCharStream<I> 
    where I: Iterator<Item=io::Result<char>>
{
    //入力
    input: I,
    //書き戻した結果を保存しておくバッファ
    lookahead_buffer: String,
    //行番号（１始まり）
    line_number: LineNumber,
    //行内のカラム位置（1始まり）
    colmun_number: ColumnNumber,
}
///////////////////////////////////////////////////////////
//コンストラクタ
impl<I> InputCharStream<I>
    where I: Iterator<Item=io::Result<char>>
{
    /// コンストラクタ
    /// 
    /// # Arguments
    /// * input_iterator - パース対象のcharのIterator
    pub fn new(input_iterator: I) -> Self {
        InputCharStream {
            input: input_iterator,
            lookahead_buffer: String::new(),
            line_number: 1,
            colmun_number: 0,
        }
    }
}
///////////////////////////////////////////////////////////
//通常のIteratorの実装
impl<I> Iterator for InputCharStream<I>
    where I: Iterator<Item=io::Result<char>>
{
    type Item = io::Result<char>;
    fn next(&mut self) -> Option<io::Result<char>> {
        if self.lookahead_buffer.len() > 0 {
            //lookahead_bufferに値がある場合はそっちを優先で返す
            Some(io::Result::Ok(self.lookahead_buffer.remove(0)))
        } else {
            let ret: Option<io::Result<char>> = self.input.next();
            match ret {
                Some(io::Result::Ok(c)) if c == '\n' || c == '\r' => {
                    self.colmun_number = 0;
                    self.line_number += 1;
                },
                Some(io::Result::Ok(_)) => {
                    self.colmun_number += 1;
                },
                Some(io::Result::Err(_)) => { },
                None => { },
            };
            ret
        }
    }
}
//書き戻し機能の拡張の実装
impl<I> InputCharStream<I>
    where I: Iterator<Item=io::Result<char>>
{
    /// 書き戻す
    /// 
    /// # Arguments
    /// * c - 書き戻す文字
    pub fn push(&mut self, c: char) {
        self.lookahead_buffer.push(c);
    }

    /// 行番号を取得する
    /// 
    /// # Return Values
    /// 行番号
    pub fn line_number(&self) -> LineNumber {
        self.line_number
    }

    /// カラム番号を取得する
    /// 
    /// # Return Values
    /// カラム番号
    pub fn column_number(&self) -> ColumnNumber {
        self.colmun_number
    }
}


///////////////////////////////////////////////////////////
/// Iterator<Item=io::Result<char>>からTokenを切り出すIterator
/// 
/// 使用方法はファイルヘッダコメントを参照
pub struct TokenStream<I> 
    where I: Iterator<Item=io::Result<char>>
{
    input: InputCharStream<I>,
    script_name: String,
}
//コンストラクタ
impl<I> TokenStream<I> 
    where I: Iterator<Item=io::Result<char>>
{
    /// コンストラクタ
    /// 
    /// スクリプト名はデフォルトの名前を設定
    /// 
    /// # Arguments
    /// * input_iterator - パース対象のcharのIterator
    pub fn new(input_iterator: I) -> Self {
        TokenStream {
            input: InputCharStream::new(input_iterator),
            script_name: String::from("##UNKNOWN###"),
        }
    }

   /// コンストラクタ
    /// 
    /// # Arguments
    /// * input_iterator - パース対象のcharのIterator
    /// * script_name - スクリプト名
    pub fn new_with_name(input_iterator: I, script_name: String) -> Self {
        TokenStream {
            input: InputCharStream::new(input_iterator),
            script_name: script_name,
        }
    }
}
///////////////////////////////////////////////////////////
//公開メソッド
impl<I> TokenStream<I> 
    where I: Iterator<Item=io::Result<char>>
{
    /// 次のコメントまたはトークンを返す
    /// 
    /// # Return Values
    /// 次のコメントまたはトークン
    pub fn next_token_with_comment(&mut self) -> Option<Result<TokenWithComment,TokenizerError>> {
        //空白を読み飛ばす
        if let Result::Err(e) = self.consume_separator() {
            return Option::Some(Result::Err(
                TokenizerError(
                    self.input.line_number(),
                    self.input.column_number(),
                    e,
                ))
            )
        }
        let line_number = self.input.line_number();
        let column_number = self.input.column_number();
        //①コメントとしてパースしてみる
        match self.parse_comment() {
            Result::Ok(TokenizerStatus::Ok(v)) => Option::Some(Result::Ok(TokenWithComment::Comment(v))),
            Result::Ok(TokenizerStatus::Skip) => {
                //②文字列としてパースしてみる
                match self.parse_string('"', '"') {
                    Result::Ok(TokenizerStatus::Ok(v)) => Option::Some(Result::Ok(TokenWithComment::Token(
                        ValueToken::StrValue(v)
                    ))),
                    Result::Ok(TokenizerStatus::Skip) => {
                        let mut parsed_string = String::new();
                        //③数字としてパースしてみる
                        match self.parse_number(&mut parsed_string) {
                            //数字の場合はIntValue
                            Result::Ok(TokenizerStatus::Ok(v)) => Option::Some(Result::Ok(TokenWithComment::Token(
                                ValueToken::IntValue(v)
                            ))),
                            //数値としてのパースに失敗した場合かつ対象が0文字以上なら、Symbol
                            Result::Ok(TokenizerStatus::Skip) if parsed_string.len() != 0 => Option::Some(Result::Ok(TokenWithComment::Token(
                                ValueToken::Symbol(parsed_string)
                            ))),
                            //0文字なら、読み込み対象がなかった場合なので、EOF
                            Result::Ok(TokenizerStatus::Skip) => Option::None,
                            Result::Err(e) => Option::Some(Result::Err(TokenizerError(line_number, column_number, e))),
                        }
                    },
                    Result::Err(e) => Option::Some(Result::Err(TokenizerError(line_number, column_number, e))),
                }
            },
            Result::Err(e) => Option::Some(Result::Err(TokenizerError(line_number, column_number, e))),
        }
    }
}
///////////////////////////////////////////////////////////
//Iteratorの実装
impl<I> Iterator for TokenStream<I>
    where I: Iterator<Item=io::Result<char>>
{
    type Item = TokenStreamItem;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.next_token_with_comment();
            match next {
                Some(Result::Ok(TokenWithComment::Comment(_))) => {
                    //コメントはスキップする
                },
                Some(Result::Ok(TokenWithComment::Token(v))) => {
                    return Some(Result::Ok(v));
                },
                Some(Result::Err(e)) => {
                    return Some(Result::Err(e));
                },
                None => {
                    return None;
                }
            }
        }
    }
}
///////////////////////////////////////////////////////////
//デバッグ情報を付与したTokenを取得する
impl<I> TokenIterator for TokenStream<I>
    where I: Iterator<Item=io::Result<char>>
{
    /// デバッグ情報を付与したTokenを取得する
    fn next_token(&mut self) -> Option<Token> {
        let line_number = self.input.line_number();
        let column_number = self.input.column_number();
        match self.next() {
            Some(x) => {
                Some(
                    Token{
                        line_number: line_number,
                        column_number: column_number,
                        value_token: x
                    }
                )
            },
            None => Option::None,
        }
    }

    /// スクリプト名を取得
    fn script_name(&self) -> &String {
        &self.script_name
    }

    /// 読み飛ばす
    fn skip(&mut self, end_char: char) -> Result<String,TokenizerError> {
        let mut skip_str = String::new();
        for next in &mut self.input {
            match next {
                Result::Ok(c) if c == end_char => {
                    return Result::Ok(skip_str);
                },
                Result::Ok(c) => {
                    skip_str.push(c);
                },
                Result::Err(e) => {
                    return Result::Err(
                        TokenizerError(
                            self.input.line_number(),
                            self.input.column_number(),
                            TokenizerErrorReason::IOError(e),
                        )
                    );
                },
            }
        };
        Result::Ok(skip_str)
    }
}
///////////////////////////////////////////////////////////
//パース用の関連関数
impl<I> TokenStream<I> 
    where I: Iterator<Item=io::Result<char>>
{
    /// 引数の文字がトークンの区切りかどうかを判定する
    /// 
    /// # Arguments
    /// * c - 判定対象の文字
    /// 
    /// # Return Values
    /// trueの場合、区切り文字
    fn is_token_separator(c: char) -> bool {
        if Self::is_line_separator(c) {
            true
        } else {
            match c {
                ' ' | '\t'  => true,
                _ => false,
            }
        }
    }

    /// 引数の文字が行の区切りかどうかを判定する
    /// 
    /// # Arguments
    /// * c - 判定対象の文字
    /// 
    /// # Return Values
    /// trueの場合、行の区切り文字
    fn is_line_separator(c: char) -> bool {
        match c {
            '\n' | '\r' => true,
            _ => false,
        }
    }

    /// 引数の文字を数値に変換する
    /// 
    /// 例えば、`'0'`を`0`に変換したり、`'a'`を`10`に変換したりする
    /// 
    /// # Arguments
    /// * c - 変換対象の文字
    /// * radix - 基数
    /// 
    /// # Return Values
    /// * Some(u32)の場合、変換結果の数値
    /// * Noneの場合、変換できなかった場合s
    fn conver_number_from_char(c: char, radix: u8) -> Option<u32> {
        if '0' <= c && c <= '9' {
            let num = (c as u32) - ('0' as u32);
            if num < radix.into() { Some(num) } else { None }
        } else if 'a' <= c && c <= 'z' {
            let num = (c as u32) - ('a' as u32) + 10;
            if num < radix.into() { Some(num) } else { None }
        } else if 'A' <= c && c <= 'Z' {
            let num = (c as u32) - ('A' as u32) + 10;
            if num < radix.into() { Some(num) } else { None }
        } else {
            None
        }
    }

    /// 文字列を数字に変換する
    /// 
    /// 例えば、16進数の場合は`"fe"`を`254`に変換する
    /// s
    /// # Arguments
    /// * value - 変換対象の文字列
    /// * radix - 基数
    /// 
    /// # Return Values
    /// * Some(u32)の場合、変換結果の数値
    /// * Noneの場合、変換できなかった場合
    fn convert_number(value: &str, radix: u8) -> Option<u32> {
        if value.len() == 0 {
            return None;
        }
        let mut result: u32 = 0;
        let mut std: u32 = 1;
        //先頭の連続する0を無視して、逆順にして順番に足しこむ
        for c in value.chars().skip_while(|&c|{ c == '0'}).collect::<Vec<_>>().iter().rev() {
            match Self::conver_number_from_char(*c, radix) {
                Some(num) => {
                    result += num * std;
                    std *= radix as u32;
                },
                None => {
                    return None;
                },
            }
        }
        Some(result)
    }
}
///////////////////////////////////////////////////////////
//コメントと空白のパース
impl<I> TokenStream<I> 
    where I: Iterator<Item=io::Result<char>>
{
    /// 連続する区切り文字を消費する
    /// 
    /// # Return Values
    /// 消費した区切り文字の数
    fn consume_separator(&mut self) -> Result<usize,TokenizerErrorReason> {
        let mut separator_num = 0;
        for next in &mut self.input {
            let c = next?;
            if Self::is_token_separator(c) {
                separator_num += 1;
            } else {
                self.input.push(c);
                break;
            }
        }
        Result::Ok(separator_num)
    }

    /// コメントをパースする
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(String) - コメントの内容
    /// * TokenizerStatus::Skip - コメントではない場合
    fn parse_comment(&mut self) -> Result<TokenizerStatus<String>,TokenizerErrorReason> {
        match self.input.next() {
            Some(v) => {
                let next_char = v?;
                if next_char == '#' {
                    //#で始まる場合は、コメントとして行末まで読む
                    let mut result = String::new();
                    for c in &mut self.input {
                        let next_next_char = c?;
                        if !Self::is_line_separator(next_next_char) {
                            result.push(next_next_char);
                        } else {
                            self.input.push(next_next_char);
                            break;
                        }
                    }
                    Result::Ok(TokenizerStatus::Ok(result))
                } else {
                    //#以外の場合は、コメントではないのでSkip
                    self.input.push(next_char);
                    Result::Ok(TokenizerStatus::Skip)
                }
            },
            None => Result::Ok(TokenizerStatus::Skip),
        }
    }
}
///////////////////////////////////////////////////////////
//文字列リテラルのパース
impl<I> TokenStream<I> 
    where I: Iterator<Item=io::Result<char>>
{
    /// 文字列リテラルをパースする
    /// 
    /// # Arguments
    /// * start_char - 文字列リテラルの開き記号
    /// * end_char - 文字列リテラルの閉じ記号
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(String)) - 文字列リテラル
    /// * TokenizerStatus::Skip - 文字列リテラルではない場合
    fn parse_string(&mut self, start_char: char, end_char: char) -> Result<TokenizerStatus<String>,TokenizerErrorReason> {
        if let Some(next) = self.input.next() {
            let next_char = next?;
            if next_char == start_char {
                let mut result = String::new();
                let mut next_char_result;
                loop {
                    next_char_result = self.parse_string_internal_next_char(end_char)?;
                    match  next_char_result {
                        TokenizerStatus::Ok(next_internal_char) => result.push(next_internal_char),
                        _ => break,
                    };
                }
                //inputのIteratorの末尾またはend_charまできたら上記ループは停止する
                //end_charはinputに書き戻してあるので、再度end_charかどうかを判定する
                if let Some(last) = self.input.next() {
                    let last_char = last?;
                    if last_char == end_char {
                        Result::Ok(TokenizerStatus::Ok(result))
                    } else {
                        Result::Err(TokenizerErrorReason::StringLiteralIsNotClosed(result))
                    }
                } else {
                    Result::Err(TokenizerErrorReason::StringLiteralIsNotClosed(result))
                }
            } else {
                //start_charで開始していないのに場合、文字列ではないのでこの処理はSkip
                self.input.push(next_char);
                Result::Ok(TokenizerStatus::Skip)
            }
        } else {
            Result::Ok(TokenizerStatus::Skip)
        }
    }

    /// 文字列リテラル中の次の１文字をパースする
    /// 
    /// エスケープ文字があった場合はそのエスケープ文字を本来の文字に解釈した結果をcharとして返す。
    /// 
    /// # Arguments
    /// * end_char - 文字列リテラルの閉じ記号
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(char) - 次の１文字
    /// * TokenizerStatus::Skip - 文字列リテラルの末尾
    fn parse_string_internal_next_char(&mut self, end_char: char) -> Result<TokenizerStatus<char>,TokenizerErrorReason> {
        match self.input.next() {
            Some(next) => {
                let next_char = next?;
                if next_char == '\\' {
                    self.parse_string_internal_escape_char()
                } else if next_char == end_char {
                    self.input.push(next_char);
                    Result::Ok(TokenizerStatus::Skip)
                } else {
                    Result::Ok(TokenizerStatus::Ok(next_char))
                }
            },
            None => Result::Ok(TokenizerStatus::Skip),
        }
    }

    /// エスケープ文字リテラルをそれがさす文字に変換する
    /// 
    /// \n,\r,\tなどを変換する。
    /// このメソッドが呼び出された時点では`"\"`までは消費された状態。
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(char) - \で始まるエスケープ文字リテラルがさす文字
    /// * TokenizerStatus::Skip - \の次の文字がない場合
    fn parse_string_internal_escape_char(&mut self) -> Result<TokenizerStatus<char>,TokenizerErrorReason> {
        match self.input.next() {
            Some(next) => {
                let next_char = next?;
                if next_char == 'n' {
                    Result::Ok(TokenizerStatus::Ok('\n'))
                }else if next_char == 'r' {
                    Result::Ok(TokenizerStatus::Ok('\r'))
                } else if next_char == 't' {
                    Result::Ok(TokenizerStatus::Ok('\t'))
                } else if next_char == 'u' {
                    self.parse_string_internal_unicode_escape_char()
                } else {
                    //エスケープ文字として定義されていない場合は\を読み飛ばす
                    Result::Ok(TokenizerStatus::Ok(next_char))
                }
            },
            None => Result::Ok(TokenizerStatus::Skip),
        }
    }

    /// ユニコードエスケープ文字のリテラルを文字に変換する
    /// 
    /// `"\uxxxx"`の表記（xxxxはユニコードコードポイントを表す16進数値）をそのコードポイントがさすcharに変換する。
    /// このメソッドの呼び出し時点では`"\u"`までは消費された状態。
    /// xxxxがユニコードコードポイントとして解釈出来ない場合はエラーとする。
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(char) - 入力の\uxxxxがさす文字
    /// * TokenizerStatus::Skip - なし
    fn parse_string_internal_unicode_escape_char(&mut self) -> Result<TokenizerStatus<char>,TokenizerErrorReason> {
        let mut value = String::new();
        //４文字を抜き取る
        for next in self.input.by_ref().take(4) {
            let c = next?;
            value.push(c);
        }
        //４文字に満たない時に末尾に到達する可能性があるため、４文字かどうかをチェック
        if value.len() == 4 {
            match Self::convert_number(&value, 16) {
                Some(num) => match std::char::from_u32(num) {
                    //ユニコードエスケープ(\u)の場合は、必ず\uの後ろはコードポイントとして解釈可能な４桁の16進数値でなければならない
                    //それ以外はエラー(CannotParseUnicodeEscapeChar)とする
                    Some(c) => Result::Ok(TokenizerStatus::Ok(c)),
                    None => Result::Err(TokenizerErrorReason::CannotParseUnicodeEscapeChar(value)),
                },
                None => Result::Err(TokenizerErrorReason::CannotParseUnicodeEscapeChar(value)),
            }
        } else {
            Result::Err(TokenizerErrorReason::CannotParseUnicodeEscapeChar(value))
        }
    }
}
///////////////////////////////////////////////////////////
//数値のパース
impl<I> TokenStream<I> 
    where I: Iterator<Item=io::Result<char>>
{

    /// 数値としてパースする
    /// 
    /// # Arguments
    /// * parsed_string - あとでSymbolとしてパースする可能性があるので、読み取った文字列を全てこれに保存して返す
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(i32) - 数値
    /// * TokenizerStatus::Skip - 数値としてパースできなかった場合
    fn parse_number(&mut self, parsed_string: &mut String) -> Result<TokenizerStatus<i32>,TokenizerErrorReason> {
        let sign_result = self.parse_number_prefix_sign(parsed_string)?;
        let sign = match sign_result {
            TokenizerStatus::Ok(sign) => sign,
            TokenizerStatus::Skip => 1, //デフォルトで'+'として解釈する
        };
        let radix_result = self.parse_number_prefix_radix(parsed_string)?;
        let radix = match radix_result {
            TokenizerStatus::Ok(radix) => radix,
            TokenizerStatus::Skip => 10, //デフォルトで10進数として解釈する
        };
        let body_result = self.parse_number_body(parsed_string)?;
        match body_result {
            TokenizerStatus::Ok(body) => {
                match Self::convert_number(&body, radix) {
                    //符号／基数指定部分／数値が全てパースできた場合は数値に変換する
                    Some(number) => Result::Ok(TokenizerStatus::Ok(sign * number as i32)),
                    None => Result::Ok(TokenizerStatus::Skip),
                }
            },
            TokenizerStatus::Skip => Result::Ok(TokenizerStatus::Skip),
        }
    }

    /// 数値の符号をパースする
    /// 
    /// # Arguments
    /// * parsed_string - あとでSymbolとしてパースする可能性があるので、読み取った文字列を全てこれに保存して返す
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(i32) - '+'の場合は＋1。'-'の場合は-1。
    /// * TokenizerStatus::Skip - '+','-'以外の場合
    fn parse_number_prefix_sign(&mut self, parsed_string: &mut String) -> Result<TokenizerStatus<i32>,TokenizerErrorReason> {
        match self.input.next() {
            Some(next) => {
                let next_char = next?;
                if next_char == '+' {
                    parsed_string.push(next_char);
                    Result::Ok(TokenizerStatus::Ok(1))
                } else if next_char == '-' {
                    parsed_string.push(next_char);
                    Result::Ok(TokenizerStatus::Ok(-1))
                } else {
                    self.input.push(next_char);
                    Result::Ok(TokenizerStatus::Skip)
                }
            },
            None => Result::Ok(TokenizerStatus::Skip),
        }
    }

    /// 数値の基数指定部分をパースする
    /// 
    /// # Arguments
    /// * parsed_string - あとでSymbolとしてパースする可能性があるので、読み取った文字列を全てこれに保存して返す
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(u8) - 基数
    /// * TokenizerStatus::Skip - 基数指定以外の文字列の場合
    /// * TokenizerStatus::Error - IOエラー
    fn parse_number_prefix_radix(&mut self, parsed_string: &mut String) -> Result<TokenizerStatus<u8>,TokenizerErrorReason> {
        match self.input.next() {
            Some(next) => { 
                let next_char = next?;
                if next_char == '0' {
                    match self.input.next() {
                        Some(next_next) => {
                            let next_next_char = next_next?;
                            if next_next_char == 'b' {
                                //'0b'は2進数
                                parsed_string.push(next_char);
                                parsed_string.push(next_next_char);
                                Result::Ok(TokenizerStatus::Ok(2))
                            } else if next_next_char == 'o' {
                                //'0o'は8進数
                                parsed_string.push(next_char);
                                parsed_string.push(next_next_char);
                                Result::Ok(TokenizerStatus::Ok(8))
                            } else if next_next_char == 'x' {
                                //'0x'は16進数
                                parsed_string.push(next_char);
                                parsed_string.push(next_next_char);
                                Result::Ok(TokenizerStatus::Ok(16))
                            } else {
                                self.input.push(next_char);
                                self.input.push(next_next_char);
                                Result::Ok(TokenizerStatus::Skip)
                            }
                        },
                        None => {
                            self.input.push(next_char);
                            Result::Ok(TokenizerStatus::Skip)
                        },
                    }
                } else {
                    self.input.push(next_char);
                    Result::Ok(TokenizerStatus::Skip)
                }
            },
            None => Result::Ok(TokenizerStatus::Skip),
        }
    }

    /// 符号と基数指定部分を省いた数値部分をパースする
    /// 
    /// # Arguments
    /// * parsed_string - あとでSymbolとしてパースする可能性があるので、読み取った文字列を全てこれに保存して返す
    /// 
    /// # Return Values
    /// * TokenizerStatus::Ok(String) - 数値部分の文字列
    /// * TokenizerStatus::Skip - なし
    fn parse_number_body(&mut self, parsed_string: &mut String) -> Result<TokenizerStatus<String>,TokenizerErrorReason> {
        let mut result = String::new();
        for next in &mut self.input {
            let c = next?;
            if !Self::is_token_separator(c) {
                if c == '_' {
                    //数値表現中の'_'は読み飛ばす
                    parsed_string.push(c);
                } else {
                    result.push(c);
                    parsed_string.push(c);
                }
            } else {
                self.input.push(c);
                break;
            }
        }
        Result::Ok(TokenizerStatus::Ok(result))
    }
}

/// strからTokenIteratorを生成して返す
/// 
/// # Arguments
/// * v - ソース
/// 
/// # Return Values
/// TokenIterator
pub fn new_token_stream_from_str(v: &str) -> TokenStream<CharStreamFromBufRead<io::BufReader<&[u8]>>> {
    TokenStream::new(CharStreamFromBufRead::new(buf_reader_from_str(v)))
}

/// Read TraitからTokenIteratorを生成して返す
/// 
/// # Arguments
/// * resource_name - リソース名
/// * read - リーダー
/// 
/// # Return Values
/// TokenIterator
pub fn create_token_iterator<'a, R>(resource_name: &String, read: R) -> Box<dyn TokenIterator + 'a>
    where R: io::Read + 'a
{
    Box::new(
        TokenStream::new_with_name(
            CharStreamFromBufRead::new(
                std::io::BufReader::new(
                    read
                )
            ),
            resource_name.clone()
        )
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io;

    #[test]
    fn test_int_value_radix10() {
        let mut tokens = new_token_stream_from_str("1");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(1));
        let mut tokens = new_token_stream_from_str("999");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(999));
        let mut tokens = new_token_stream_from_str("+1");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(1));
        let mut tokens = new_token_stream_from_str("+999");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(999));
        let mut tokens = new_token_stream_from_str("-1");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-1));
        let mut tokens = new_token_stream_from_str("-999");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-999));
        let mut tokens = new_token_stream_from_str("01");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(1));
        let mut tokens = new_token_stream_from_str("0999");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(999));
        let mut tokens = new_token_stream_from_str("-01");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-1));
        let mut tokens = new_token_stream_from_str("-0999");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-999));
        let mut tokens = new_token_stream_from_str("123_456_789");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(123456789));
        let mut tokens = new_token_stream_from_str("-123_456_789");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-123456789));
        let mut tokens = new_token_stream_from_str("_000_123_456_789");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(123456789));
        let mut tokens = new_token_stream_from_str("-000_123_456_789");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-123456789));
        let mut tokens = new_token_stream_from_str("0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("-0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("0000");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("-0000");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
    }

    #[test]
    fn test_int_value_radix16() {
        let mut tokens = new_token_stream_from_str("0xa");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(10));
        let mut tokens = new_token_stream_from_str("+0xff");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(255));
        let mut tokens = new_token_stream_from_str("-0xf");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-15));
        let mut tokens = new_token_stream_from_str("+0x0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("-0x0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("0x_0_f_0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(240));
        let mut tokens = new_token_stream_from_str("-0x_0_f_a");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-250));
        let mut tokens = new_token_stream_from_str("0x_00_00_00");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
    }

    #[test]
    fn test_int_value_radix8() {
        let mut tokens = new_token_stream_from_str("0o21");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(17));
        let mut tokens = new_token_stream_from_str("+0o22");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(18));
        let mut tokens = new_token_stream_from_str("-0o23");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-19));
        let mut tokens = new_token_stream_from_str("+0o0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("-0o0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("0o_0_7_7");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(63));
        let mut tokens = new_token_stream_from_str("-0o_0_3_0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-24));
        let mut tokens = new_token_stream_from_str("0o_00_00_00");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
    }

    #[test]
    fn test_int_value_radix2() {
        let mut tokens = new_token_stream_from_str("0b11");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(3));
        let mut tokens = new_token_stream_from_str("+0b10");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(2));
        let mut tokens = new_token_stream_from_str("-0b101");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-5));
        let mut tokens = new_token_stream_from_str("+0b0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("-0b0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
        let mut tokens = new_token_stream_from_str("0b_0_1_0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(2));
        let mut tokens = new_token_stream_from_str("-0b_0_1_0");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(-2));
        let mut tokens = new_token_stream_from_str("0b_00_00_00");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(0));
    }

    #[test]
    fn test_symbol() {
        let mut tokens = new_token_stream_from_str("abcd");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("abcd".to_string()));
        let mut tokens = new_token_stream_from_str("ABCD");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("ABCD".to_string()));
        let mut tokens = new_token_stream_from_str("-");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("-".to_string()));
        let mut tokens = new_token_stream_from_str("+");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("+".to_string()));
        let mut tokens = new_token_stream_from_str("0x");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("0x".to_string()));
        let mut tokens = new_token_stream_from_str("0o");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("0o".to_string()));
        let mut tokens = new_token_stream_from_str("0b");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("0b".to_string()));
        let mut tokens = new_token_stream_from_str("0xZZZ");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("0xZZZ".to_string()));
        let mut tokens = new_token_stream_from_str("(\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("(\"".to_string()));
    }

    #[test]
    fn test_str_value() {
        let mut tokens = new_token_stream_from_str("\"\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("".to_string()));
        let mut tokens = new_token_stream_from_str("\"abc\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("abc".to_string()));
        let mut tokens = new_token_stream_from_str("\"\\t\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("\t".to_string()));
        let mut tokens = new_token_stream_from_str("\"\\r\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("\r".to_string()));
        let mut tokens = new_token_stream_from_str("\"\\n\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("\n".to_string()));
        let mut tokens = new_token_stream_from_str("\"\\\"\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("\"".to_string()));
        let mut tokens = new_token_stream_from_str("\"\\u0040\"");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("@".to_string()));
        let mut tokens = new_token_stream_from_str("\"abcd");
        assert_eq!(
            match tokens.next() {
                Some(Result::Err(TokenizerError(l, c, TokenizerErrorReason::StringLiteralIsNotClosed(ref v)))) if l == 1 && c == 1 && v == "abcd" => true,
                _ => false,
            },
            true
        );
        let mut tokens = new_token_stream_from_str("\"abc\\\"");
        assert_eq!(
            match tokens.next() {
                Some(Result::Err(TokenizerError(l, c, TokenizerErrorReason::StringLiteralIsNotClosed(ref v)))) if l == 1 && c == 1 && v == "abc\"" => true,
                _ => false,
            },
            true
        );
        let mut tokens = new_token_stream_from_str("\"\\u012\"");
        assert_eq!(
            match tokens.next() {
                Some(Result::Err(TokenizerError(l, c, TokenizerErrorReason::CannotParseUnicodeEscapeChar(ref v)))) if l == 1 && c == 1 && v == "012\"" => true,
                _ => false,
            },
            true
        );
        let mut tokens = new_token_stream_from_str("\"\\u00fg\"");
        assert_eq!(
            match tokens.next() {
                Some(Result::Err(TokenizerError(l, c, TokenizerErrorReason::CannotParseUnicodeEscapeChar(ref v)))) if l == 1 && c == 1 && v == "00fg" => true,
                _ => false,
            },
            true
        );
    }

    #[test]
    fn test_comment() {
        let mut tokens = new_token_stream_from_str("#123 abc \"aaa\"");
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("123 abc \"aaa\"".to_string()));
        let mut tokens = new_token_stream_from_str("#123 abc \"aaa\"\n");
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("123 abc \"aaa\"".to_string()));
        let mut tokens = new_token_stream_from_str("#123 abc \"aaa\"\r\n");
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("123 abc \"aaa\"".to_string()));
        let mut tokens = new_token_stream_from_str("#");
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("".to_string()));
        let mut tokens = new_token_stream_from_str("#\n");
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("".to_string()));
        let mut tokens = new_token_stream_from_str("#\r\n");
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("".to_string()));
    }

    #[test]
    fn test_next_token_with_comment() {
        let mut tokens = new_token_stream_from_str("123 aaa #aaa\n \"aaa\" #aaa");
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Token(ValueToken::IntValue(123)));
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Token(ValueToken::Symbol("aaa".to_string())));
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("aaa".to_string()));
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Token(ValueToken::StrValue("aaa".to_string())));
        assert_eq!(tokens.next_token_with_comment().unwrap().unwrap(), TokenWithComment::Comment("aaa".to_string()));
        assert_eq!(tokens.next_token_with_comment().is_none(), true);
        assert_eq!(tokens.next_token_with_comment().is_none(), true);
    }

    #[test]
    fn test_next() {
        let mut tokens = new_token_stream_from_str("123 aaa #aaa\n \"aaa\" #aaa");
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::IntValue(123));
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("aaa".to_string()));
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::StrValue("aaa".to_string()));
        assert_eq!(tokens.next().is_none(), true);
        assert_eq!(tokens.next().is_none(), true);
    }

    #[test]
    fn test_io_error() {
        let src: &[u8] = &[0x41, 0x42, 0x0a, 0xe3];
        let read = io::BufReader::new(src);
        let stream = CharStreamFromBufRead::new(read);
        let mut tokens = TokenStream::new(stream);
        assert_eq!(tokens.next().unwrap().unwrap(), ValueToken::Symbol("AB".to_string()));
        let result = tokens.next().unwrap();
        assert_eq!(result.is_err(), true);
        assert_eq!(
            match result {
                Result::Err(TokenizerError(_, _, TokenizerErrorReason::IOError(_))) => true,
                _ => false,
            },
            true
        );
        assert_eq!(tokens.next().is_none(), true);
        assert_eq!(tokens.next().is_none(), true);
    }
}