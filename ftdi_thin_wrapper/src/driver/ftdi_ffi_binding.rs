///
/// libftd2xxから必要な関数のみをimportするためのFFI定義
/// 全の関数を利用するわけではなく、BitBangモードでのみ利用できれば良いので、それに必要なものだけ定義
/// 

/// libftd2xxで定義されている型:PVOIDの読み替え
type PVOID = *mut std::ffi::c_void;
/// libftd2xxで定義されている型:DWORDの読み替え
type DWORD = u32;
/// libftd2xxで定義されている型:ULONGの読み替え
type ULONG = u32;
/// libftd2xxで定義されている型:UCHARの読み替え
type UCHAR = u8;
/// C言語の型:intの読み替え
type INT = i32;
/// libftd2xxで定義されている型:LPDWORDの読み替え
type LPDWORD = *mut u32;
/// libftd2xxで定義されている型:PUCHARの読み替え
type PUCHAR = *mut u8;
/// libftd2xxで定義されている型:FT_STATUSの読み替え
#[allow(non_camel_case_types)]
type FT_STATUS = ULONG;
/// libftd2xxで定義されている型:FT_HANDLEの読み替え
#[allow(non_camel_case_types)]
type FT_HANDLE = PVOID;
/// libftd2xxのFT_Writeで使用されるバッファの型がLPVOIDとなっているが、使いやすいように読み替え
type BUFFER = *const u8;

/// libftd2xxの定数定義:FT_BITMODE_RESETの読み替え
/// 元のC言語の定義
/// ```c
/// #define FT_BITMODE_RESET                                        0x00
/// ```
pub const FT_BITMODE_RESET: u8 = 0x00;
/// libftd2xxの定数定義:FT_BITMODE_ASYNC_BITBANGs
/// 元のC言語の定義
/// ```c
/// #define FT_BITMODE_ASYNC_BITBANG                        0x01
/// ```
pub const FT_BITMODE_ASYNC_BITBANG: u8 = 0x01;

//今回は未使用だがlibftd2xxでは定義されている定数
//#define FT_BITMODE_MPSSE                                        0x02
//#define FT_BITMODE_SYNC_BITBANG                         0x04
//#define FT_BITMODE_MCU_HOST                                     0x08
//#define FT_BITMODE_FAST_SERIAL                          0x10
//#define FT_BITMODE_CBUS_BITBANG                         0x20
//#define FT_BITMODE_SYNC_FIFO                            0x40

#[link(name="ftd2xx")]
extern "C" {
    /// FT_OpenのFFI定義
    /// 元のC言語の定義は以下の通り
    /// ```c
    ///    FTD2XX_API
    ///            FT_STATUS WINAPI FT_Open(
    ///            int deviceNumber,
    ///            FT_HANDLE *pHandle
    ///            );
    /// ```
    pub fn FT_Open(deviceNumber: INT, pHandle: *mut FT_HANDLE) -> FT_STATUS;

    /// FT_SetBitModeのFFI定義
    /// ```c
    ///    FTD2XX_API
    ///            FT_STATUS WINAPI FT_SetBitMode(
    ///            FT_HANDLE ftHandle,
    ///            UCHAR ucMask,
    ///            UCHAR ucEnable
    ///            );
    /// ```
    pub fn FT_SetBitMode(handle: FT_HANDLE, mask: UCHAR, enable: UCHAR) -> FT_STATUS;

    /// FT_SetBaudRateのFFI定義
    /// 元のC言語の定義は以下の通り
    /// ```c
    ///    FTD2XX_API
    ///            FT_STATUS WINAPI FT_SetBaudRate(
    ///            FT_HANDLE ftHandle,
    ///            ULONG BaudRate
    ///            );
    /// ```
    pub fn FT_SetBaudRate(handle: FT_HANDLE, baudRate: ULONG) -> FT_STATUS;

    /// FT_WriteのFFI定義
    /// 元のC言語の定義は以下の通り
    /// ```c
    ///    FTD2XX_API 
    ///            FT_STATUS WINAPI FT_Write(
    ///            FT_HANDLE ftHandle,
    ///            LPVOID lpBuffer,
    ///            DWORD dwBytesToWrite,
    ///            LPDWORD lpBytesWritten
    ///            );
    /// ```
    pub fn FT_Write(handle: FT_HANDLE, buffer: BUFFER, bytesToWrite: DWORD, bytesWritten: LPDWORD) -> FT_STATUS;

    /// FT_GetBitModeのFFI定義
    /// ```c
    ///    FTD2XX_API
    ///            FT_STATUS WINAPI FT_GetBitMode(
    ///            FT_HANDLE ftHandle,
    ///            PUCHAR pucMode
    ///s            );
    /// ```
    pub fn FT_GetBitMode(handle: FT_HANDLE, mode: PUCHAR) -> FT_STATUS;

    /// FT_CloseのFFI定義
    /// 元のC言語の定義は以下の通り
    /// ```c
    ///    FTD2XX_API
    ///            FT_STATUS WINAPI FT_Close(
    ///            FT_HANDLE ftHandle
    ///            );
    /// ```
    pub fn FT_Close(handle: FT_HANDLE) -> FT_STATUS;
}
