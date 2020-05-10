//! デバッグ情報
//! 

use super::tokenizer::*;
use super::value::*;

/// スクリプト名を管理するための内部ハンドル
pub type ScriptNameHandle = usize;

///////////////////////////////////////////////////////////
/// コードバッファと元のコードを紐づけるための情報
pub struct CodeDebugInfo {
    /// 元コードのスクリプト名
    pub script_name_handle: ScriptNameHandle,
    /// 元コード位置の行番号
    pub line_number: LineNumber,
    /// 元コード位置のカラム番号
    pub column_number: ColumnNumber,
}

///////////////////////////////////////////////////////////
/// コードバッファと元のコードの対応表
pub struct DebugInfoStore {
    /// コードアドレス → 元コード情報
    code_mapping: Vec<Option<CodeDebugInfo>>,
    /// リターン先のスクリプトを管理
    script_call_stack: Vec<ScriptNameHandle>,
    /// スクリプト名ハンドル　→　スクリプト名
    script_name_list: Vec<String>,
    /// 現在実行中（コンパイル中）のスクリプト名
    current_script_name_handle: ScriptNameHandle,
}
impl DebugInfoStore {

    /// コンストラクタ
    /// 
    pub fn new() -> DebugInfoStore {
        DebugInfoStore{
            code_mapping: Vec::new(),
            script_call_stack: Vec::new(),
            script_name_list: Vec::new(),
            current_script_name_handle: 0,
        }
    }
}
impl DebugInfoStore {

    /// デバッグ情報を取得
    /// 
    /// # Arguments
    /// * address - コードアドレス
    /// 
    /// # Return Values
    /// コードアドレスと対応するデバッグ情報
    /// 
    pub fn get(&self, address: CodeAddress) -> Option<&CodeDebugInfo> {
        match address {
            CodeAddress(Address::Entity(a)) => {
                match self.code_mapping.get(a) {
                    Option::Some(Option::Some(x)) => Option::Some(x),
                    Option::Some(Option::None) => Option::None,
                    Option::None => Option::None,
                }
            },
            CodeAddress(Address::Root) => {
                Option::None
            },
        }
    }

    /// 現在実行中のスクリプト名を取得
    /// 
    /// # Return Values
    /// スクリプト名
    /// 
    pub fn current_script_name(&self) -> Option<&String> {
        self.get_script_name(self.current_script_name_handle)
    }

    /// スクリプト名ハンドルからスクリプト名を取得
    /// 
    /// # Arguments
    /// * script_name_handle - スクリプト名ハンドル
    /// 
    /// # Return Values
    /// スクリプト名
    /// 
    pub fn get_script_name(&self, script_name_handle: ScriptNameHandle) -> Option<&String> {
        self.script_name_list.get(script_name_handle)
    }

    /// スクリプトの呼び出しをハンドリングする
    /// 
    /// # Arguments
    /// * script_name - 呼び出しスクリプト名
    /// 
    pub fn call_script(&mut self, script_name: &String) {
        self.script_name_list.push(script_name.clone());
        self.current_script_name_handle = self.script_name_list.len() - 1;
        self.script_call_stack.push(self.current_script_name_handle);
    }

    /// スクリプトからのリターンをハンドリング
    ///
    pub fn return_script(&mut self) {
        self.script_call_stack.pop();
        self.current_script_name_handle = *self.script_call_stack.last().unwrap_or(&0);
    }

    /// デバッグ情報を登録
    /// 
    /// # Arguments
    /// * address - 対象のコードアドレス
    /// * line_number - 対応するコードの行番号
    /// * column_number - 対応するコードのカラム番号
    /// 
    pub fn put(&mut self, address: CodeAddress, line_number: LineNumber, column_number: ColumnNumber) {
        match address {
            CodeAddress(Address::Entity(a)) => {
                while self.code_mapping.len() < a + 1 {
                    self.code_mapping.push(Option::None);
                }
                self.code_mapping[a] = Option::Some(CodeDebugInfo{
                    script_name_handle: self.current_script_name_handle,
                    line_number: line_number,
                    column_number: column_number,
                });
            },
            CodeAddress(Address::Root) => { }
        };
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_debug_store() {

        let mut db = DebugInfoStore::new();

        //階層的な呼び出し系列の検証
        db.call_script(&String::from("S001")); {

            db.put(From::from(0), 10, 100);
            assert_eq!(db.get_script_name(db.get(From::from(0)).unwrap().script_name_handle).unwrap(), &String::from("S001"));
            assert_eq!(db.get(From::from(0)).unwrap().line_number, 10);
            assert_eq!(db.get(From::from(0)).unwrap().column_number, 100);
            
            db.put(From::from(1), 11, 101);
            assert_eq!(db.get_script_name(db.get(From::from(1)).unwrap().script_name_handle).unwrap(), &String::from("S001"));
            assert_eq!(db.get(From::from(1)).unwrap().line_number, 11);
            assert_eq!(db.get(From::from(1)).unwrap().column_number, 101);

            db.call_script(&String::from("S002")); {

                db.put(From::from(2), 12, 102);
                assert_eq!(db.get_script_name(db.get(From::from(2)).unwrap().script_name_handle).unwrap(), &String::from("S002"));
                assert_eq!(db.get(From::from(2)).unwrap().line_number, 12);
                assert_eq!(db.get(From::from(2)).unwrap().column_number, 102);

                db.put(From::from(3), 13, 103);
                assert_eq!(db.get_script_name(db.get(From::from(3)).unwrap().script_name_handle).unwrap(), &String::from("S002"));
                assert_eq!(db.get(From::from(3)).unwrap().line_number, 13);
                assert_eq!(db.get(From::from(3)).unwrap().column_number, 103);

            } db.return_script();
            db.call_script(&String::from("S002")); {
                db.call_script(&String::from("S003")); {

                    db.put(From::from(4), 14, 104);
                    assert_eq!(db.get_script_name(db.get(From::from(4)).unwrap().script_name_handle).unwrap(), &String::from("S003"));
                    assert_eq!(db.get(From::from(4)).unwrap().line_number, 14);
                    assert_eq!(db.get(From::from(4)).unwrap().column_number, 104);

                } db.return_script();

                db.put(From::from(5), 15, 105);
                assert_eq!(db.get_script_name(db.get(From::from(5)).unwrap().script_name_handle).unwrap(), &String::from("S002"));
                assert_eq!(db.get(From::from(5)).unwrap().line_number, 15);
                assert_eq!(db.get(From::from(5)).unwrap().column_number, 105);

            } db.return_script();

            db.put(From::from(6), 16, 106);
            assert_eq!(db.get_script_name(db.get(From::from(6)).unwrap().script_name_handle).unwrap(), &String::from("S001"));
            assert_eq!(db.get(From::from(6)).unwrap().line_number, 16);
            assert_eq!(db.get(From::from(6)).unwrap().column_number, 106);

            db.put(From::from(7), 17, 107);
            assert_eq!(db.get_script_name(db.get(From::from(7)).unwrap().script_name_handle).unwrap(), &String::from("S001"));
            assert_eq!(db.get(From::from(7)).unwrap().line_number, 17);
            assert_eq!(db.get(From::from(7)).unwrap().column_number, 107);

        } db.return_script();

        //範囲外アクセス
        assert_eq!(db.get_script_name(4), Option::None);
        assert_eq!(db.get(From::from(8)).is_none(), true);

        //バッファの自動拡張
        db.call_script(&String::from("S009")); {
            db.put(From::from(10), 20, 110);
            assert_eq!(db.get(From::from(8)).is_none(), true);
            assert_eq!(db.get(From::from(9)).is_none(), true);
            assert_eq!(db.get_script_name(db.get(From::from(10)).unwrap().script_name_handle).unwrap(), &String::from("S009"));
            assert_eq!(db.get(From::from(10)).unwrap().line_number, 20);
            assert_eq!(db.get(From::from(10)).unwrap().column_number, 110);
        } db.return_script();
    }
}