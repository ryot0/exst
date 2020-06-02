//! スクリプトなどのリソース管理
//! 
//! 

use super::tokenizer::*;
use super::utility::*;
use super::vm::*;
use super::value::*;
use std::vec::*;
use std::io;
use std::collections::HashMap;
use std::fs;
use std::path;
use std::env;
use std::io::Read;

///////////////////////////////////////////////////////////
/// スクリプトの呼び出しスタック
pub struct ScriptCallStack {
    stack: Vec<(Box<dyn TokenIterator>,VmState,VmExecutionState,CodeAddress)>
}
impl ScriptCallStack {

    /// コンストラクタ
    /// 
    /// メモリサイズ0で初期化した状態で作成する
    /// 
    pub fn new() -> Self {
        ScriptCallStack {
            stack: Vec::new(),
        }
    }
}
impl ScriptCallStack {

    /// スクリプトの戻り先を追加
    /// 
    /// # Arguments
    /// * item - 追加するスクリプト
    /// 
    pub fn push(&mut self, item: Box<dyn TokenIterator>, state: VmState, exec_state: VmExecutionState, pc: CodeAddress) {
        self.stack.push((item, state, exec_state, pc));
    }

    /// スタックの先頭を取得
    /// 
    /// # Return Values
    /// スタックの先頭。なければ、Option::None
    pub fn pop(&mut self) -> Option<(Box<dyn TokenIterator>,VmState,VmExecutionState,CodeAddress)> {
        self.stack.pop()
    }
}

///////////////////////////////////////////////////////////
/// 空のTokenStream
pub struct EmptyTokenStream{
    script_name: String,
}
impl EmptyTokenStream {
    pub fn new() -> EmptyTokenStream {
        EmptyTokenStream{
            script_name: String::from("###DUMMY###"),
        }
    }
}
impl Iterator for EmptyTokenStream
{
    type Item = TokenStreamItem;
    fn next(&mut self) -> Option<Self::Item> {
        Option::None
    }
}
impl TokenIterator for EmptyTokenStream
{
    fn next_token(&mut self) -> Option<Token> {
        Option::None
    }
    fn script_name(&self) -> &String {
        &self.script_name
    }
}

///////////////////////////////////////////////////////////
/// リソースアクセスエラー
#[derive(Debug)]
pub enum ResourceErrorReason {
    /// 存在しないリソースへのアクセス
    ResourceNotFound(String),
    /// IOエラー
    IOError(std::io::Error),
}
impl From<io::Error> for ResourceErrorReason {
    fn from(err: io::Error) -> Self {
        ResourceErrorReason::IOError(err)
    }
}

///////////////////////////////////////////////////////////
/// リソースへのアクセスを提供するtrait
pub trait Resources {

    /// 標準出力への文字列出力
    /// 
    /// # Arguments
    /// * data - 出力する文字列
    fn write_stdout(&self, data: &str);

    /// 標準エラーへの文字列出力
    /// 
    /// # Arguments
    /// * data - 出力する文字列
    fn write_stderr(&self, data: &str);

    /// TokenIteratorとしてリソースを取得する
    /// 
    /// # Arguments
    /// * resource_name - リソース名
    /// 
    /// # Return Values
    /// 対応するリソース
    fn get_token_iterator(&self, resource_name: &String) -> Result<Box<dyn TokenIterator>,ResourceErrorReason>;

    /// 文字列としてリソースを取得する
    /// 
    /// # Arguments
    /// * resource_name - リソース名
    /// 
    /// # Return Values
    /// 対応するリソース
    fn get_string(&self, resource_name: &String) -> Result<String,ResourceErrorReason>;
}

///////////////////////////////////////////////////////////
/// 標準的なリソースアクセスを提供する
/// 
/// リソース名は、以下の形式で指定する
/// * ':'で始まるリソース名 - `project_root`以下の相対パスとして解釈してそのファイル内容を取得する
/// * '$'で始まるリソース名 - `add_resource`で追加した文字列リソースを取得
/// * '&'で始まるリソース名 - ２文字目以降を環境変数名として環境変数から取得する
/// * '%STDIN' - 標準入力を取得
/// * 上記以外 - ファイルシステムのパスと解釈してそのファイルの内容を取得
/// 
pub struct StdResources {
    /// ファイルリソースのルートパス
    project_root: String,
    /// 文字列リソース
    internal_resource: HashMap<String,String>,
}
impl StdResources {
 
    /// コンストラクタ
    /// 
    /// # Arugments
    /// * project_root - ファイルリソースのルーとパス
    /// 
    pub fn new(project_root: String) -> Self {
        StdResources {
            project_root: project_root,
            internal_resource: HashMap::new(),
        }
    }

    /// 文字列としてリソースを追加する
    /// 
    /// # Arguments
    /// * name - リソース名。先頭の$は自動的に付与されるため、不要
    /// * value - リソースの実態
    /// 
    pub fn add_resource(&mut self, name: String, value: String) {
        self.internal_resource.insert(String::from("$") + &name, value);
    }
}
impl Resources for StdResources {

    /// 標準出力へ出力
    fn write_stdout(&self, data: &str) {
        print!("{}", data);
    }

    /// 標準エラーへ出力
    fn write_stderr(&self, data: &str) {
        eprint!("{}", data);
    }

    /// リソースを取得
    fn get_token_iterator(&self, resource_name: &String) -> Result<Box<dyn TokenIterator>,ResourceErrorReason> {
        match resource_name.chars().nth(0) {
            Some(first_char) => {
                match first_char {
                    ':' => {
                        //プロジェクトルートからの相対パス
                        let p = path::Path::new(&self.project_root).join(path::Path::new(resource_name.get(1..).unwrap()));
                        let f = fs::OpenOptions::new().read(true).open(p)?;
                        Result::Ok(create_token_iterator(resource_name, f))
                    },
                    '$' => {
                        //内部で保持している文字列Mapから取得
                        match self.internal_resource.get(resource_name) {
                            Some(value) => {
                                Result::Ok(create_token_iterator(resource_name, CharReaderFromString::new(value.clone())))
                            },
                            None => {
                                Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
                            },
                        }
                    },
                    '%' => {
                        //標準入力など
                        if resource_name == "%STDIN" {
                            Result::Ok(create_token_iterator(resource_name, io::stdin()))
                        } else {
                            Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
                        }
                    },
                    '&' => {
                        let name: String = resource_name.chars().skip(1).collect();
                        match env::var(name) {
                            Result::Ok(v) => {
                                Result::Ok(create_token_iterator(resource_name, CharReaderFromString::new(v)))
                            },
                            Result::Err(_) => {
                                Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
                            },
                        }
                    }
                    _ => {
                        //システムのファイルパスと解釈して、ファイルを取得
                        let f = fs::OpenOptions::new().read(true).open(resource_name)?;
                        Result::Ok(create_token_iterator(resource_name, f))
                    },
                }
            },
            None => {
                Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
            },
        }
    }

    /// 文字列リソースを取得
    fn get_string(&self, resource_name: &String) -> Result<String,ResourceErrorReason> {
        match resource_name.chars().nth(0) {
            Some(first_char) => {
                match first_char {
                    ':' => {
                        //プロジェクトルートからの相対パス
                        let p = path::Path::new(&self.project_root).join(path::Path::new(resource_name.get(1..).unwrap()));
                        let mut f = fs::OpenOptions::new().read(true).open(p)?;
                        let mut out = String::new();
                        f.read_to_string(&mut out)?;
                        Result::Ok(out)
                    },
                    '$' => {
                        //内部で保持している文字列Mapから取得
                        match self.internal_resource.get(resource_name) {
                            Some(value) => {
                                Result::Ok(value.clone())
                            },
                            None => {
                                Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
                            },
                        }
                    },
                    '%' => {
                        //標準入力など
                        if resource_name == "%STDIN" {
                            let mut out = String::new();
                            io::stdin().read_to_string(&mut out)?;
                            Result::Ok(out)
                        } else {
                            Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
                        }
                    },
                    '&' => {
                        let name: String = resource_name.chars().skip(1).collect();
                        match env::var(name) {
                            Result::Ok(v) => {
                                Result::Ok(v)
                            },
                            Result::Err(_) => {
                                Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
                            },
                        }
                    }
                    _ => {
                        //システムのファイルパスと解釈して、ファイルを取得
                        let mut f = fs::OpenOptions::new().read(true).open(resource_name)?;
                        let mut out = String::new();
                        f.read_to_string(&mut out)?;
                        Result::Ok(out)
                    },
                }
            },
            None => {
                Result::Err(ResourceErrorReason::ResourceNotFound(resource_name.clone()))
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_script_call_stack() {

        //テスト対象の作成
        let mut stack = ScriptCallStack::new();
        let mut str1 = Box::new(new_token_stream_from_str("1 2 3"));
        let str2 = Box::new(new_token_stream_from_str("4 5 6"));
        let str3 = Box::new(new_token_stream_from_str("7 8 9"));
        
        //str1をとりあえず、１こ消費
        assert_eq!(str1.next().unwrap().unwrap(), ValueToken::IntValue(1));

        //スタックに全部登録
        stack.push(str1, VmState::Interpretation, VmExecutionState::TokenIteration, From::from(0));
        stack.push(str2, VmState::Interpretation, VmExecutionState::TokenIteration, From::from(0));
        stack.push(str3, VmState::Interpretation, VmExecutionState::TokenIteration, From::from(0));

        //この時点では、スタックトップはstr3なので、１こ消費して内容を確認
        let (mut str3, _, _, _) = stack.pop().unwrap();
        assert_eq!(str3.next().unwrap().unwrap(), ValueToken::IntValue(7));

        //スタックに戻す
        stack.push(str3, VmState::Interpretation, VmExecutionState::TokenIteration, From::from(0));

        //順番に内容を確認

        //str3は１こ消費しているので、２こ目のTokenが帰ってくる
        let (mut str3, _, _, _) = stack.pop().unwrap();
        assert_eq!(str3.next().unwrap().unwrap(), ValueToken::IntValue(8));
        //str2は何もしてないので、１こ目のTokenが帰ってくる
        let (mut str2, _, _, _) = stack.pop().unwrap();
        assert_eq!(str2.next().unwrap().unwrap(), ValueToken::IntValue(4));
        //str1は最初に１こ消費しているので、２こ目のTokenが帰ってくる
        let (mut str1, _, _, _) = stack.pop().unwrap();
        assert_eq!(str1.next().unwrap().unwrap(), ValueToken::IntValue(2));
        //stackが空になったことを確認
        assert_eq!(stack.pop().is_none(), true);
    }

    #[test]
    fn test_std_resources() {

        let mut r = StdResources::new(String::from("this"));
        r.add_resource("A".to_string(), "a b c".to_string());
        r.add_resource("B".to_string(), "b c d".to_string());

        //$**で検索
        let mut itr = r.get_token_iterator(&"$A".to_string()).unwrap();
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("a".to_string()));
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("b".to_string()));
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("c".to_string()));

        let mut itr = r.get_token_iterator(&"$B".to_string()).unwrap();
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("b".to_string()));
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("c".to_string()));
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("d".to_string()));

        let mut itr = r.get_token_iterator(&"$A".to_string()).unwrap();
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("a".to_string()));
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("b".to_string()));
        assert_eq!(itr.next().unwrap().unwrap(), ValueToken::Symbol("c".to_string()));

        //存在しない$**
        assert_eq!(
            match r.get_token_iterator(&"$C".to_string()) {
                Result::Err(ResourceErrorReason::ResourceNotFound(ref n)) if n == "$C" => true,
                _ => false
            }
            ,true
        );

        //%STDIN
        assert_eq!(r.get_token_iterator(&"%STDIN".to_string()).is_ok(), true);

        //存在しない%**
        assert_eq!(
            match r.get_token_iterator(&"%XXX".to_string()) {
                Result::Err(ResourceErrorReason::ResourceNotFound(ref n)) if n == "%XXX" => true,
                _ => false
            }
            ,true
        );

        //空文字
        assert_eq!(
            match r.get_token_iterator(&"".to_string()) {
                Result::Err(ResourceErrorReason::ResourceNotFound(ref n)) if n == "" => true,
                _ => false
            }
            ,true
        );
    }
}