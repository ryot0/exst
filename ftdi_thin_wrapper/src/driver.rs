//! libftd2xxをBitBangモードで利用するための薄いラッパー
//! 
//! # Examples
//! 
//! ```no_run
//! use ftdi_thin_wrapper::driver::FTDIDriver;
//! //０番のデバイスを開く
//! let mut driver = FTDIDriver::open(0).unwrap();
//! //ボーレートを9600に設定
//! driver.set_baud_rate(9600).unwrap();
//! //D0を入力ピンとしてそれ以外は出力ピンに設定
//! driver.set_bit_mode_async(0xfe).unwrap();
//! //D7...D0に01010101を設定
//! driver.write(0b01010101).unwrap();
//! //各ピンのHIGT or LOWを読み取る
//! let value = driver.read().unwrap();
//! //明示的にcloseしなくてもスコープが外れた時にcloseされる
//! ```
//! 

mod ftdi_ffi_binding;

/// FT_Handleのエイリアス
type FTDIHandle = *mut std::ffi::c_void;

/// FT_STATUSのラッパー
#[derive(Debug, PartialEq)]
pub enum FTDIStatus {
    /// FT_OK: 0
    FTOk,
    /// FT_INVALID_HANDLE: 1
    FTInvalidHandle,
    /// FT_DEVICE_NOT_FOUND: 2
    FTDeviceNotFound,
    /// FT_DEVICE_NOT_OPENDED: 3
    FTDeviceNotOpened,
    /// FT_IO_ERROR: 4
    FTIOError,
    /// FT_INFFICIENT_RESOURCES: 5
    FTInfficientResources,
    /// FT_INVALID_PARAMETER: 6
    FTInvalidParameter,
    /// FT_INVALID_BAUD_RATE: 7
    FTInvalidBaudRate,
    /// FT_DEVICE_NOT_OPENED_FOR_ERASE: 8
    FTDeviceNotOpenedForErase,
    /// FT_DEVICE_NOT_OPENED_FOR_WRITE: 9
    FTDeviceNotOpenedForWrite,
    /// FT_FAILED_TO_WRITE_DEVICE: 10
    FTFailedToWriteDevice,
    /// FT_EEPROM_READ_FAILED: 11
    FTEEPROMReadFailed,
    /// FT_EEPROM_WRITE_FAILED: 12
    FTEEPROMWriteFailed,
    /// FT_EEPROM_ERASE_FAILED: 13
    FTEEPROMEraseFailed,
    /// FT_EEPROM_NOT_PRESENT: 14
    FTEEPROMNotPresent,
    /// FT_EEPROM_NOT_PROGRAMMED: 15
    FTEEPROMNotProgrammed,
    /// FT_INVALID_ARGS: 16
    FTInvalidArgs,
    /// FT_NOT_SUPPORTED: 17
    FTNotSupported,
    /// FT_OTHER_ERROR: 18
    FTOtherError,
    /// FT_DEVICE_LIST_NOT_READY: 19
    FTDeviceListNotReady,
    /// libftd2xxで未定義のFT_STATUSが帰ってきた場合
    Unknown(u32),
}
impl std::convert::From<u32> for FTDIStatus {

    /// FT_STATUSの値からFTDIStatusに変換する
    /// 
    /// # Arguments
    /// * `ftdi_status_code` - libftd2xxの関数から返却されるFT_STATUSの値
    /// # Return Value
    /// `ftdi_status_code`と対応する`FTDIStatus`
    /// 
    /// # Examples
    /// ```
    /// use ftdi_thin_wrapper::driver::FTDIStatus;
    /// let ok = FTDIStatus::from(0);
    /// assert_eq!(ok, FTDIStatus::FTOk);
    /// let err = FTDIStatus::from(1);
    /// assert_eq!(err, FTDIStatus::FTInvalidHandle);
    /// ```
    fn from(ftdi_status_code: u32) -> Self {
        match ftdi_status_code {
             0 => FTDIStatus::FTOk,
             1 => FTDIStatus::FTInvalidHandle,
             2 => FTDIStatus::FTDeviceNotFound,
             3 => FTDIStatus::FTDeviceNotOpened,
             4 => FTDIStatus::FTIOError,
             5 => FTDIStatus::FTInfficientResources,
             6 => FTDIStatus::FTInvalidParameter,
             7 => FTDIStatus::FTInvalidBaudRate,
             8 => FTDIStatus::FTDeviceNotOpenedForErase,
             9 => FTDIStatus::FTDeviceNotOpenedForWrite,
            10 => FTDIStatus::FTFailedToWriteDevice,
            11 => FTDIStatus::FTEEPROMReadFailed,
            12 => FTDIStatus::FTEEPROMWriteFailed,
            13 => FTDIStatus::FTEEPROMEraseFailed,
            14 => FTDIStatus::FTEEPROMNotPresent,
            15 => FTDIStatus::FTEEPROMNotProgrammed,
            16 => FTDIStatus::FTInvalidArgs,
            17 => FTDIStatus::FTNotSupported,
            18 => FTDIStatus::FTOtherError,
            19 => FTDIStatus::FTDeviceListNotReady,
             _ => FTDIStatus::Unknown(ftdi_status_code),
        }
    }
}

/// libftd2xxにアクセスするためのラッパー構造体
/// 
/// openでインスタンスを作って、インスタンス経由で各操作を行う。
/// 各メソッドは対応するlibftd2xxのAPIを呼び出し、FT_OKが帰ってきた場合は結果を`Result::Ok`にラップして返し、
/// FT_OK以外が帰ってきた場合は、`Result::Err`として返却する。
/// 
/// `drop`関数でFT_Closeを呼び出すため、明示的にClose操作は不要
/// 
#[derive(Debug)]
pub struct FTDIDriver(FTDIHandle);
impl FTDIDriver {

    /// FTDIデバイスを開く
    /// 
    /// # Arguments
    /// * `device_number` - デバイス番号
    /// 
    /// # Return Value
    /// FT_OKの場合は、FTDIDriverを返す。それ以外のの場合Errを返す。
    pub fn open(device_number: i32) -> Result<FTDIDriver, FTDIStatus> {
        let mut raw_handle = 0 as FTDIHandle;
        let raw_status = unsafe{ ftdi_ffi_binding::FT_Open(device_number, &mut raw_handle) };
        match FTDIStatus::from(raw_status) {
            FTDIStatus::FTOk => Ok(FTDIDriver(raw_handle)),
            err => Err(err),
        }
    }

    /// ボーレートを変更する
    /// 
    /// # Arguments
    /// * `baud_rate` - ボーレート
    /// 
    /// # Return Value
    /// FT_OKの場合は、()を返す。それ以外のの場合Errを返す。
    pub fn set_baud_rate(&mut self, baud_rate: u32) -> Result<(), FTDIStatus> {
        let raw_status = unsafe{ ftdi_ffi_binding::FT_SetBaudRate(self.0, baud_rate) };
        match FTDIStatus::from(raw_status) {
            FTDIStatus::FTOk => Ok(()),
            err => Err(err),
        }
    }

    /// FTDIデバイスを非同期モードに設定してピンごとの出力/入力を変更する
    /// 
    /// # Arguments
    /// * `mask` - ピンごとの入出力の設定。1の場合出力。0の場合は入力。
    /// 
    /// # Return Value
    /// FT_OKの場合は、()を返す。それ以外のの場合Errを返す。
    pub fn set_bit_mode_async(&mut self, mask: u8) -> Result<(), FTDIStatus> {
        let raw_status = unsafe{ ftdi_ffi_binding::FT_SetBitMode(self.0, mask, ftdi_ffi_binding::FT_BITMODE_ASYNC_BITBANG) };
        match FTDIStatus::from(raw_status) {
            FTDIStatus::FTOk => Ok(()),
            err => Err(err),
        }
    }

    /// 各ピンのHIGHT or LOWの状態を読み取る
    /// 
    /// # Return Value
    /// FT_OKの場合は、読み取った値を返す。それ以外の場合はErrを返す。
    /// デバイスのD0がLSB...D7がMSBに入る。HIGHTの場合は1、LOWの場合は0。
    pub fn read(&mut self) -> Result<u8, FTDIStatus> {
        let mut read_data = 0;
        let raw_status = unsafe { ftdi_ffi_binding::FT_GetBitMode(self.0, &mut read_data) };
        match FTDIStatus::from(raw_status) {
            FTDIStatus::FTOk => Ok(read_data),
            err => Err(err),
        }
    }

    /// 各ピンのHIGHT or LOWを変更する
    /// 
    /// # Arguments
    /// * `data` - デバイスに書き込む値。D0の値をLSB...D7をMSBに入れる。HIGHTにする場合は1にして、LOWにする場合は0にする。
    /// 
    /// # Return Value
    /// FT_OKの場合は、()を返す。それ以外のの場合Errを返す。
    pub fn write(&mut self, data: u8) -> Result<u8, FTDIStatus> {
        let mut writed_size = 0;
        let raw_status = unsafe { ftdi_ffi_binding::FT_Write(self.0, &data, 1, &mut writed_size) };
        match FTDIStatus::from(raw_status) {
            FTDIStatus::FTOk => self.read(),
            err => Err(err),
        }
    }

    /// リセットする
    /// 
    /// # Return Value
    /// FT_OKの場合は、()を返す。それ以外のの場合Errを返す。
    pub fn reset(&mut self) -> Result<(), FTDIStatus> {
        let raw_status = unsafe{ ftdi_ffi_binding::FT_SetBitMode(self.0, 0, ftdi_ffi_binding::FT_BITMODE_RESET) };
        match FTDIStatus::from(raw_status) {
            FTDIStatus::FTOk => Ok(()),
            err => Err(err),
        }
    }
}
impl Drop for FTDIDriver {
    /// drop時にcloseする
    fn drop(&mut self) {
        unsafe{ ftdi_ffi_binding::FT_Close(self.0) };
    }
}
