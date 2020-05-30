//! Wordの定義
//! 
//! # Usage Examples
//! ```
//! use exst_core::lang::word::*;
//! let mut d1 = Dictionary::new();
//! //ワードの予約定義
//! d1.reserve_word_def(String::from("w1"), Word::new(From::from(100)));
//! //この時点では、find_word_within_resavationのみで検索できる
//! assert_eq!(d1.find_word_within_resavation(&String::from("w1")).unwrap().code(), From::from(100));
//! //定義完了
//! d1.complate_word_def().unwrap();
//! //これでようやくfind_wordで検索できるようになる
//! assert_eq!(d1.find_word(&String::from("w1")).unwrap().code(), From::from(100));
//! ```
//! 

use super::value::*;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt;

///////////////////////////////////////////////////////////
/// ワードディクショナリのエラー
/// 
#[derive(Debug,Clone)]
pub enum WordErrorReason {
    /// 未定義の完了操作
    CompleteWordInUnreserved,
    /// 未定義のワードの参照
    UndefinedWord(String),
    /// まだ１つもワードが定義されてない（last wordがない）
    EmptyDictionary,
}

///////////////////////////////////////////////////////////
/// ワード構造体
/// 
#[derive(Debug,Clone)]
pub struct Word {
    code: CodeAddress,
    immediate: bool,
    document: String,
}
impl Word {

    /// コンストラクタ
    /// 
    /// # Arguments
    /// * code - 実行コードのアドレス
    pub fn new(code: CodeAddress) -> Self {
        Word {
            code: code,
            immediate: false,
            document: String::from(""),
        }
    }

    /// コードアドレスを取得する
    /// 
    /// # Return Values
    /// * 実行コードのアドレス
    pub fn code(&self) -> CodeAddress {
        self.code
    }

    //// コードアドレスを更新する
    /// 
    /// # Arguments
    /// * c - 新しいコードアドレス
    pub fn set_code(&mut self, c: CodeAddress) {
        self.code = c;
    }

    /// イミディエイトワードにする
    pub fn immediate(&mut self) {
        self.immediate = true;
    }

    /// イミディエイトワードかどうか
    /// 
    /// # Return Values
    /// * trueの場合、イミディエイトワード
    pub fn is_immediate(&self) -> bool {
        self.immediate
    }

    /// ドキュメントの登録
    /// 
    /// # Arguments
    /// * doc - ドキュメント
    pub fn set_document(&mut self, doc: String) {
        self.document = doc;
    }

    /// ドキュメントの取得
    /// 
    /// # Return Values
    /// ドキュメント
    pub fn document(&self) -> &String {
        &self.document
    }
}
impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Word({},{},{})", self.code, self.immediate, self.document)
    }
}

///////////////////////////////////////////////////////////
/// 辞書
/// 
pub struct Dictionary {
    dict: HashMap<String,Word>,
    inverse_dict: BTreeMap<CodeAddress,String>,
    reserved_name: Option<String>,
    reserved_word: Option<Word>,
}
impl Dictionary {

    /// コンストラクタ
    pub fn new() -> Self {
        Dictionary {
            dict: HashMap::new(),
            inverse_dict: BTreeMap::new(),
            reserved_name: Option::None,
            reserved_word: Option::None,
        }
    }

    /// 登録数を取得
    pub fn len(&self) -> usize {
        self.dict.len()
    }

    /// 定義されているワード名をすべて取得する
    /// 
    /// かなり重い処理なので、デバッグ用途のみで使用する
    /// 
    /// # Return Values
    /// すべてのワード名
    pub fn all_word_names(&self) -> Vec<&String> {
        let mut v: Vec<_> = self.dict.iter().collect();
        v.sort_by(|a,b|{ a.1.code().cmp(&b.1.code()) });
        v.iter().map(|a|{ a.0 }).collect()
    }

    /// コードアドレスからワード名を逆引き
    /// 
    /// # Aeguments
    /// * adr - コードアドレス
    /// 
    /// # Return Values
    /// adrを含むワードの名前
    pub fn guess_name(&self, adr: &CodeAddress) -> Option<&String> {
        match adr.adr() {
            Result::Ok(base) => {
                let mut ret = Option::None;
                for (a, n) in self.inverse_dict.iter() {
                    if a.adr().is_ok() && a.adr().unwrap() > base {
                        break;
                    } else {
                        ret = Option::Some(n);
                    }
                }
                ret
            },
            Result::Err(_) => {
                Option::None
            }
        }
    }

    /// コードアドレスを開始位置とするワードを逆引き
    /// 
    /// # Aeguments
    /// * adr - コードアドレス
    /// 
    /// # Return Values
    /// adrから始まるワードの名前
    pub fn find_name(&self, adr: &CodeAddress) -> Option<&String> {
        self.inverse_dict.get(adr)
    }

    /// ワード定義を予約登録
    /// 
    /// # Arguments
    /// * name - ワード名
    /// * word - ワード定義
    pub fn reserve_word_def(&mut self, name: String, word: Word) {
        self.reserved_name = Option::Some(name);
        self.reserved_word = Option::Some(word);
    }

    /// ワード定義を完了する
    pub fn complate_word_def(&mut self) -> Result<(),WordErrorReason> {
        let name = self.reserved_name.clone();
        let word = std::mem::replace(&mut self.reserved_word, Option::None);
        match name {
            Option::Some(n) => match word {
                Option::Some(w) => {
                    self.inverse_dict.insert(w.code, n.clone());
                    self.dict.insert(n.clone(), w);
                    Result::Ok(())
                },
                Option::None => Result::Err(WordErrorReason::CompleteWordInUnreserved),
            },
            Option::None => Result::Err(WordErrorReason::CompleteWordInUnreserved),
        }
    }

    /// 定義完了したワードのみを検索する
    /// 
    /// # Arguments
    /// * name - ワード名
    pub fn find_word_mut(&mut self, name: &String) -> Result<&mut Word,WordErrorReason> {
        match self.dict.get_mut(name) {
            Option::Some(w) => Result::Ok(w),
            Option::None => Result::Err(WordErrorReason::UndefinedWord(name.clone())),
        }
    }
    /// 定義完了したワードのみを検索する
    /// 
    /// # Arguments
    /// * name - ワード名
    pub fn find_word(&self, name: &String) -> Result<&Word,WordErrorReason> {
        match self.dict.get(name) {
            Option::Some(w) => Result::Ok(w),
            Option::None => Result::Err(WordErrorReason::UndefinedWord(name.clone())),
        }
    }

    /// 予約登録したワードも含めて検索する（主に再帰的に定義されるワードで使用する）
    /// 
    /// # Arguments
    /// * name - ワード名
    pub fn find_word_within_resavation_mut(&mut self, name: &String) -> Result<&mut Word,WordErrorReason> {
        match self.reserved_name {
            Option::Some(ref n) if n == name => match self.reserved_word {
                Option::Some(ref mut w) => {
                    Result::Ok(w)
                },
                _ => self.find_word_mut(name),
            },
            _ => self.find_word_mut(name),
        }
    }
    /// 予約登録したワードも含めて検索する（主に再帰的に定義されるワードで使用する）
    /// 
    /// # Arguments
    /// * name - ワード名
    pub fn find_word_within_resavation(&self, name: &String) -> Result<&Word,WordErrorReason> {
        match self.reserved_name {
            Option::Some(ref n) if n == name => match self.reserved_word {
                Option::Some(ref w) => {
                    Result::Ok(w)
                },
                _ => self.find_word(name),
            },
            _ => self.find_word(name),
        }
    }

    /// 最後に定義したワードを検索
    /// 
    /// # Return Values
    /// 最後に定義したワード
    /// 
    pub fn find_last_word_mut(&mut self) -> Result<&mut Word,WordErrorReason> {
        match self.reserved_name.clone() {
            Some(n) => self.find_word_within_resavation_mut(&n),
            None => Result::Err(WordErrorReason::EmptyDictionary),
        }
    }
    /// 最後に定義したワードを検索
    /// 
    /// # Return Values
    /// 最後に定義したワード
    /// 
    pub fn find_last_word(&self) -> Result<&Word,WordErrorReason> {
        match self.reserved_name {
            Some(ref n) => self.find_word_within_resavation(n),
            None => Result::Err(WordErrorReason::EmptyDictionary),
        }
    }

    /// 最後に定義したワードをイミディエイトワードに変更する。
    /// ワードがなければ、何もしない。
    pub fn last_word_change_immidiate(&mut self) {
        match self.find_last_word_mut() {
            Result::Ok(w) => w.immediate(),
            Result::Err(_) => { },
        }
    }

    /// 最後に定義したワードにドキュメントを登録する
    /// ワードがなければ、何もしない。
    pub fn last_word_set_document(&mut self, doc: String) {
        match self.find_last_word_mut() {
            Result::Ok(w) => w.set_document(doc),
            Result::Err(_) => { },
        }
    }
}
impl fmt::Display for Dictionary
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Dictionary[")?;
        for (n, w) in self.dict.iter() {
            writeln!(f, "  <{},{}>;", n, w)?
        }
        writeln!(f, "]")
    }
}

///////////////////////////////////////////////////////////
/// ローカル変数辞書
/// 
pub struct LocalDictionary {
    dict: HashMap<String,EnvironmentStackRelativeAddress>,
    current_address: EnvironmentStackRelativeAddress, 
}
impl LocalDictionary {

    /// コンストラクタ
    pub fn new() -> Self {
        LocalDictionary {
            dict: HashMap::new(),
            current_address: Default::default(),
        }
    }

    /// 登録数を取得
    pub fn len(&self) -> usize {
        self.dict.len()
    }

    /// 定義されているローカル変数名をすべて取得する
    /// 
    /// # Return Values
    /// すべてのワード名
    pub fn get_all_local_names(&self) -> Vec<&String> {
        self.dict.keys().collect()
    }

    /// ローカル変数を追加
    /// 
    /// # Arguments
    /// * name - 変数名
    pub fn push(&mut self, name: String) {
        self.dict.insert(name, self.current_address);
        self.current_address = self.current_address.next();
    }

    /// ローカル変数を検索
    /// 
    /// # Arguments
    /// * name - 変数名
    /// 
    /// # Return Values
    /// 環境スタックのアドレス
    pub fn find(&self, name: &String) -> Option<EnvironmentStackRelativeAddress> {
        match self.dict.get(name) {
            Option::Some(adr) => Option::Some(*adr),
            Option::None => Option::None,
        }
    }

    /// クリアする
    pub fn clear(&mut self) {
        self.dict.clear();
        self.current_address = Default::default();
    }
}
impl fmt::Display for LocalDictionary
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "LocalDictionary[")?;
        for (n, a) in self.dict.iter() {
            writeln!(f, "  <{},{}>;", n, a)?
        }
        writeln!(f, "]")
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_word() {
        
        let mut w1 = Word::new(From::from(100));
        assert_eq!(w1.is_immediate(), false);
        assert_eq!(w1.code(), From::from(100));
        w1.immediate();
        assert_eq!(w1.is_immediate(), true);
    }

    #[test]
    fn test_dictionary() {

        let mut d1 = Dictionary::new();

        //予約
        d1.reserve_word_def(String::from("w1"), Word::new(From::from(100)));

        //find_word_within_resavationのみで見つかる
        assert_eq!(
            match d1.find_word(&String::from("w1")).unwrap_err() {
                WordErrorReason::UndefinedWord(ref name) if name == "w1" => true,
                _ => false
            }
            ,true
        );
        assert_eq!(d1.find_word_within_resavation(&String::from("w1")).unwrap().code(), From::from(100));
        assert_eq!(
            match d1.find_word_within_resavation(&String::from("ZZZ")).unwrap_err() {
                WordErrorReason::UndefinedWord(ref name) if name == "ZZZ" => true,
                _ => false
            }
            ,true
        );

        //確定
        d1.complate_word_def().unwrap();

        //find_wordでもfind_word_within_resavationでもどっちでも見つかる
        assert_eq!(d1.find_word(&String::from("w1")).unwrap().code(), From::from(100));
        assert_eq!(d1.find_word_within_resavation(&String::from("w1")).unwrap().code(), From::from(100));
        assert_eq!(
            match d1.find_word(&String::from("ZZZ")).unwrap_err() {
                WordErrorReason::UndefinedWord(ref name) if name == "ZZZ" => true,
                _ => false
            }
            ,true
        );

        //resavationされていない状態でcomplateするとエラーになる
        assert_eq!(
            match d1.complate_word_def().unwrap_err() {
                WordErrorReason::CompleteWordInUnreserved => true,
                _ => false,
            },
            true
        );

        //上書き
        d1.reserve_word_def(String::from("w1"), Word::new(From::from(200)));
        d1.complate_word_def().unwrap();
        assert_eq!(d1.find_word(&String::from("w1")).unwrap().code(), From::from(200));
    }

    #[test]
    fn test_last_word_in_resavation() {

        let mut d1 = Dictionary::new();
        d1.reserve_word_def(String::from("w1"), Word::new(From::from(111)));
        
        assert_eq!(d1.find_last_word().unwrap().code(), From::from(111));
    }

    #[test]
    fn test_local_dictionary() {

        let mut d1 = LocalDictionary::new();
        d1.push(String::from("A"));
        assert_eq!(d1.find(&String::from("A")).unwrap(), From::from(0));
        d1.push(String::from("B"));
        assert_eq!(d1.find(&String::from("B")).unwrap(), From::from(1));
        d1.push(String::from("C"));
        assert_eq!(d1.find(&String::from("C")).unwrap(), From::from(2));
        
        assert_eq!(
            match d1.find(&String::from("X")) {
                None => true,
                _ => false,
            },
            true
        );
    }

    #[test]
    fn test_inverse_dict() {

        let mut d1 = Dictionary::new();
        d1.reserve_word_def(String::from("A"), Word::new(From::from(20)));
        d1.complate_word_def().unwrap();
        d1.reserve_word_def(String::from("B"), Word::new(From::from(30)));
        d1.complate_word_def().unwrap();
        d1.reserve_word_def(String::from("C"), Word::new(From::from(10)));
        d1.complate_word_def().unwrap();

        assert_eq!(d1.find_name(&From::from(9)).is_none(), true);
        assert_eq!(d1.guess_name(&From::from(9)).is_none(), true);
        assert_eq!(d1.find_name(&From::from(10)).unwrap(), "C");
        assert_eq!(d1.guess_name(&From::from(10)).unwrap(), "C");
        assert_eq!(d1.find_name(&From::from(11)).is_none(), true);
        assert_eq!(d1.guess_name(&From::from(11)).unwrap(), "C");
        assert_eq!(d1.find_name(&From::from(19)).is_none(), true);
        assert_eq!(d1.guess_name(&From::from(19)).unwrap(), "C");
        assert_eq!(d1.find_name(&From::from(20)).unwrap(), "A");
        assert_eq!(d1.guess_name(&From::from(20)).unwrap(), "A");
        assert_eq!(d1.find_name(&From::from(21)).is_none(), true);
        assert_eq!(d1.guess_name(&From::from(21)).unwrap(), "A");
        assert_eq!(d1.find_name(&From::from(29)).is_none(), true);
        assert_eq!(d1.guess_name(&From::from(29)).unwrap(), "A");
        assert_eq!(d1.find_name(&From::from(30)).unwrap(), "B");
        assert_eq!(d1.guess_name(&From::from(30)).unwrap(), "B");
        assert_eq!(d1.find_name(&From::from(31)).is_none(), true);
        assert_eq!(d1.guess_name(&From::from(31)).unwrap(), "B");

        assert_eq!(d1.all_word_names(), ["C", "A", "B"]);
    }
}