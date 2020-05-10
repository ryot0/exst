//! BufReadからcharのIteratorに変換する
//! 

use std::io;
use std::io::Read;

///////////////////////////////////////////////////////////
/// BufReadからcharのIteratorに変換する
/// 
/// # Type Parameters
/// * RB - BufReadの実装型
/// 
/// # Examples
/// ```
/// let mut stream = exst_core::lang::utility::char_stream_from_buf_read_from_str("ab\nA\n");
/// assert_eq!(stream.next().unwrap().unwrap(), 'a');
/// assert_eq!(stream.next().unwrap().unwrap(), 'b');
/// assert_eq!(stream.next().unwrap().unwrap(), '\n');
/// assert_eq!(stream.next().unwrap().unwrap(), 'A');
/// assert_eq!(stream.next().unwrap().unwrap(), '\n');
/// assert_eq!(stream.next().is_none(), true);
/// ```
pub struct CharStreamFromBufRead<RB> 
    where RB: io::BufRead
{
    /// 読み込み元のソース
    input: RB,
    /// 現在読み込み中の行の文字列
    current_chars: Option<Vec<char>>,
    /// 現在読み込み中の文字列のうち、読み込み位置
    current_pos: usize,
    /// 読み込み用のバッファ
    line: String,
}
///////////////////////////////////////////////////////////
/// コンストラクタと内部関数の実装
impl<RB> CharStreamFromBufRead<RB> 
    where RB: io::BufRead
{
    /// コンストラクタ
    /// 
    /// # Arguments
    /// * input - 入力ソース
    pub fn new(input: RB) -> Self {
        CharStreamFromBufRead {
            input: input,
            current_chars: None,
            current_pos: 0,
            line: String::new(),
        }
    }

    /// 次の行を読み込み、先頭の１文字を返す
    fn read_next_line(&mut self) -> Option<io::Result<char>> {
        self.line.clear();
        match self.input.read_line(&mut self.line) {
            io::Result::Ok(_) => {
                self.current_chars = Option::Some(self.line.chars().collect());
                self.current_pos = 0;
                match self.current_chars.as_ref().unwrap().get(self.current_pos) {
                    Some(c) => Option::Some(io::Result::Ok(*c)),
                    None => Option::None,
                }
            },
            io::Result::Err(e) => Option::Some(io::Result::Err(e)),
        }
    }
}
///////////////////////////////////////////////////////////
/// Iteratorの実装
impl<RB> Iterator for CharStreamFromBufRead<RB>
    where RB: io::BufRead
{
    /// Iteratorの要素の型
    type Item = io::Result<char>;

    /// Iteratorの実装
    fn next(&mut self) -> Option<io::Result<char>> {
        match self.current_chars {
            Some(ref mut cs) => {
                self.current_pos += 1;
                match cs.get(self.current_pos) {
                    Some(c) => {
                        return Option::Some(Result::Ok(*c));
                    },
                    None => {
                        return self.read_next_line();
                    },
                }
            },
            None => {
                return self.read_next_line();
            },
        }
    }
}

///////////////////////////////////////////////////////////
/// 文字列からReaderに変換
/// 
pub struct CharReaderFromString {
    src: Vec<u8>,
    current_pos: usize,
}
impl CharReaderFromString {

    /// コンストラクタ
    /// 
    /// # Arguments
    /// * s - 元の文字列
    /// 
    /// # Return Values
    /// Reader
    /// 
    pub fn new(s: String) -> CharReaderFromString {
        CharReaderFromString {
            src: s.into_bytes(),
            current_pos: 0,
        }
    }
}
impl Read for CharReaderFromString
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.current_pos < self.src.len() {
            let mut p: usize = 0;
            while p < buf.len() && self.current_pos < self.src.len() {
                buf[p] = *self.src.get(self.current_pos).unwrap();
                p = p + 1;
                self.current_pos = self.current_pos + 1
            }
            Result::Ok(p)
        } else {
            Result::Ok(0)
        }
    }
}


///////////////////////////////////////////////////////////
/// 文字列からBufReaderに変換する
/// 
/// # Arguments
/// * v - 入力元の文字列
/// 
/// # Return Values
/// * vをBufReaderに変換した結果
pub fn buf_reader_from_str(v: &str) -> io::BufReader<&[u8]> {
    io::BufReader::new(v.as_bytes())
}

///////////////////////////////////////////////////////////
/// 文字列からBufReaderに変換する
/// 
/// # Arguments
/// * v - 入力元の文字列
/// 
/// # Return Values
/// * vをBufReaderに変換した結果
pub fn buf_reader_from_string(v: String) -> io::BufReader<CharReaderFromString> {
    io::BufReader::new(CharReaderFromString::new(v))
}

///////////////////////////////////////////////////////////
/// 文字列からCharStreamFromBufReadに変換する
/// 
/// # Arguments
/// * v - 入力元の文字列
/// 
/// # Return Values
/// * vをCharStreamBufReadに変換した結果
pub fn char_stream_from_buf_read_from_str(v: &str) -> CharStreamFromBufRead<io::BufReader<&[u8]>> {
    CharStreamFromBufRead::new(buf_reader_from_str(v))
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multi_line() {
        let mut stream = char_stream_from_buf_read_from_str("abcde\nABCDE\rST\r\n\n");
        assert_eq!(stream.next().unwrap().unwrap(), 'a');
        assert_eq!(stream.next().unwrap().unwrap(), 'b');
        assert_eq!(stream.next().unwrap().unwrap(), 'c');
        assert_eq!(stream.next().unwrap().unwrap(), 'd');
        assert_eq!(stream.next().unwrap().unwrap(), 'e');
        assert_eq!(stream.next().unwrap().unwrap(), '\n');
        assert_eq!(stream.next().unwrap().unwrap(), 'A');
        assert_eq!(stream.next().unwrap().unwrap(), 'B');
        assert_eq!(stream.next().unwrap().unwrap(), 'C');
        assert_eq!(stream.next().unwrap().unwrap(), 'D');
        assert_eq!(stream.next().unwrap().unwrap(), 'E');
        assert_eq!(stream.next().unwrap().unwrap(), '\r');
        assert_eq!(stream.next().unwrap().unwrap(), 'S');
        assert_eq!(stream.next().unwrap().unwrap(), 'T');
        assert_eq!(stream.next().unwrap().unwrap(), '\r');
        assert_eq!(stream.next().unwrap().unwrap(), '\n');
        assert_eq!(stream.next().unwrap().unwrap(), '\n');
        assert_eq!(stream.next().is_none(), true);
        assert_eq!(stream.next().is_none(), true);
    }

    #[test]
    fn test_io_error() {
        let src: &[u8] = &[0x41, 0x42, 0x0a, 0xe3];
        let read = io::BufReader::new(src);
        let mut stream = CharStreamFromBufRead::new(read);
        assert_eq!(stream.next().unwrap().unwrap(), 'A');
        assert_eq!(stream.next().unwrap().unwrap(), 'B');
        assert_eq!(stream.next().unwrap().unwrap(), '\n');
        assert_eq!(stream.next().unwrap().is_err(), true);
        assert_eq!(stream.next().is_none(), true);
        assert_eq!(stream.next().is_none(), true);
    }

    #[test]
    fn test_string_stream() {
        let mut stream = CharReaderFromString::new(String::from("abcdefg"));
        let mut buf = [0u8; 3];
        assert_eq!(stream.read(&mut buf).unwrap(), 3);
        assert_eq!(buf[0], 'a' as u8);
        assert_eq!(buf[1], 'b' as u8);
        assert_eq!(buf[2], 'c' as u8);
        assert_eq!(stream.read(&mut buf).unwrap(), 3);
        assert_eq!(buf[0], 'd' as u8);
        assert_eq!(buf[1], 'e' as u8);
        assert_eq!(buf[2], 'f' as u8);
        assert_eq!(stream.read(&mut buf).unwrap(), 1);
        assert_eq!(buf[0], 'g' as u8);
        assert_eq!(stream.read(&mut buf).unwrap(), 0);
    }
}