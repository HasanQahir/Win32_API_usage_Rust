use std::{ptr::{null, null_mut}, ffi::c_void};

// A load of type aliases for the windows C functions
// All original types are kept, even if just repeats, for ease of understanding
// Aliases to literals and primitive types in C
pub type CUint = u32;
pub type CInt = i32;
pub type UintPtr = usize;
pub type LongPtr = isize;
pub type LONG = CLong;
pub type CLong = i32;
pub type BYTE = u8;
pub type WcharT = u16;
pub type PVOID = *mut core::ffi::c_void;

// Aliases of types which are defined as Aliases in C (i.e. Those above).
pub type LPARAM = LongPtr;
pub type LRESULT = LongPtr;
pub type WPARAM = UintPtr;
pub type UINT = CUint;
pub type LPCWSTR = *const WCHAR;
pub type WCHAR = WcharT;
pub type HANDLE = PVOID;

// Lastly, the type aliases which refer to those prior
pub type HDC = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HICON = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;
pub type HWND = HANDLE;
pub type COLOR16 = CUshort;

//A pub macro which calls the WNDPROC function, placed in an Option<> as C functions can return NULL.
pub type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

// Values defined by Windows for interactions with the OS
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const CW_USEDEFAULT: CInt = 0x80000000_u32 as CInt;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const COLOR_WINDOW: u32 = 5;
pub const SW_SHOW: CInt = 5;
#[allow(dead_code)]
pub const MB_OKCANCEL: u32 = 1;
#[allow(dead_code)]
pub const IDOK: CInt = 1;
pub const GWLP_USERDATA: CInt = -21;
pub const IDC_ARROW: LPCWSTR = makeintresourcew(32512);
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER:DWORD = 0x00000100;
pub const FORMAT_MESSAGE_FROM_SYSTEM:DWORD = 0x00001000;
pub const FORMAT_MESSAGE_IGNORE_INSERTS:DWORD = 0x00000200;
pub const GRADIENT_FILL_RECT_H: CUlong= 0x00000000;
pub const GRADIENT_FILL_RECT_V: CUlong= 0x00000001;
pub const GRADIENT_FILL_TRIANGLE: CUlong= 0x00000002;

// Values used by the Window_Procedure to check for events we care about
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;

/// The predefined cursor styles.
pub enum IDCursor {
    /// Standard arrow and small hourglass
    AppStarting = 32650,
    /// Standard arrow
    Arrow = 32512,
    /// Crosshair
    Cross = 32515,
    /// Hand
    Hand = 32649,
    /// Arrow and question mark
    Help = 32651,
    /// I-beam
    IBeam = 32513,
    /// Slashed circle
    No = 32648,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll = 32646,
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw = 32643,
    /// Double-pointed arrow pointing north and south
    SizeNS = 32645,
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe = 32642,
    /// Double-pointed arrow pointing west and east
    SizeWE = 32644,
    /// Vertical arrow
    UpArrow = 32516,
    /// Hourglass
    Wait = 32514,
}

/// See [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor {
    _3dDarkShadow = 21,
    _3dLight = 22,
    ActiveBorder = 10,
    ActiveCaption = 2,
    AppWorkspace = 12,
    /// Button face, also "3D face" color.
    ButtonFace = 15,
    /// Button highlight, also "3D highlight" color.
    ButtonHighlight = 20,
    /// Button shadow, also "3D shadow" color.
    ButtonShadow = 16,
    ButtonText = 18,
    CaptionText = 9,
    /// Desktop background color
    Desktop = 1,
    GradientActiveCaption = 27,
    GradientInactiveCaption = 28,
    GrayText = 17,
    Highlight = 13,
    HighlightText = 14,
    HotLight = 26,
    InactiveBorder = 11,
    InactiveCaption = 3,
    InactiveCaptionText = 19,
    InfoBackground = 24,
    InfoText = 23,
    Menu = 4,
    MenuHighlight = 29,
    MenuBar = 30,
    MenuText = 7,
    ScrollBar = 0,
    Window = 5,
    WindowFrame = 6,
    WindowText = 8,
  }

// C structs converted into Rust Structs
#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfn_wnd_proc: WNDPROC,
    pub cb_cls_extra: CInt,
    pub cb_wnd_extra: CInt,
    pub h_instance: HINSTANCE,
    pub h_icon: HICON,
    pub h_cursor: HCURSOR,
    pub hbr_background: HBRUSH,
    pub lpsz_menu_name: LPCWSTR,
    pub lpsz_class_name: LPCWSTR,
}
impl Default for WNDCLASSW {
    #[inline]
    #[must_use]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub w_param: WPARAM,
    pub l_param: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub l_private: DWORD,
}

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub f_erase: BOOL,
    pub rc_paint: RECT,
    pub f_restore: BOOL,
    pub f_inc_update: BOOL,
    pub rgb_reserved: [BYTE; 32],
}

#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

#[repr(C)]
pub struct CREATESTRUCTW {
    pub lp_create_params: LPVOID,
    pub h_instance: HINSTANCE,
    pub h_menu: HMENU,
    pub hwnd_parent: HWND,
    pub cy: CInt,
    pub cx: CInt,
    pub y: CInt,
    pub x: CInt,
    pub style: LONG,
    pub lpsz_name: LPCWSTR,
    pub lpsz_class: LPCWSTR,
    pub dw_ex_style: DWORD,
}

#[repr(C)]
#[derive(Debug)]
pub struct TRIVERTIX {
    pub x: LONG,
    pub y: LONG,
    pub red: COLOR16,
    pub green: COLOR16,
    pub blue: COLOR16,
    pub alpha: COLOR16,
}

#[repr(C)]
pub struct GradientTriangle {
    pub vertex1: CUlong,
    pub vertex2: CUlong,
    pub vertex3: CUlong,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Win32Error(pub DWORD);
impl std::error::Error for Win32Error {}

impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.0 & (1 << 29) > 0 {
            return write!(f, "Win32ApplicationError({})", self.0);
        }
        let dw_flags = FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS;
        let lp_source = null_mut();
        let dw_message_id = self.0;
        let dw_language_id = 0;
        let mut buffer: *mut u16 = null_mut();
        let lp_buffer = &mut buffer as *mut *mut u16 as *mut u16;
        let n_size = 0;
        let arguments = null_mut();
        let tchar_count_excluding_null = unsafe {
            FormatMessageW(
                dw_flags,
                lp_source,
                dw_message_id,
                dw_language_id,
                lp_buffer,
                n_size,
                arguments,
            )
        };
        if tchar_count_excluding_null == 0 || buffer.is_null() {
        // some sort of problem happened. we can't usefully get_last_error since
        // Display formatting doesn't let you give an error value.
            return Err(core::fmt::Error);
        } else {
            struct OnDropLocalFree(HLOCAL);
            impl Drop for OnDropLocalFree {
                fn drop(&mut self) {
                    unsafe { LocalFree(self.0) };
                }
            }
            let _on_drop = OnDropLocalFree(buffer as HLOCAL);            
            let buffer_slice: &[u16] = unsafe {
                core::slice::from_raw_parts(buffer, tchar_count_excluding_null as usize)};
            
            for decode_result in core::char::decode_utf16(buffer_slice.iter().copied()) {
                match decode_result {
                    Ok('\r') | Ok('\n') => write!(f, " ")?,
                    Ok(ch) => write!(f, "{}", ch)?,
                    Err(_) => write!(f, "�")?,
                }
            }
            Ok(())
        }
    }
}

/// See [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> Win32Error {
    Win32Error(unsafe { GetLastError() })
}

//A bunch of C functions defined in various header files, any new type aliases are kept here with them.
pub type DWORD = CUlong;
pub type CUlong = u32;
pub type LPCVOID = *const core::ffi::c_void;
pub type VaList = *mut CChar;
pub type CChar = i8;
pub type HLOCAL = HANDLE;
#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;

    /// [`FormatMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
    pub fn FormatMessageW(
    dwFlags: DWORD, lpSource: LPCVOID, dwMessageId: DWORD, dwLanguageId: DWORD,
    lpBuffer: LPWSTR, nSize: DWORD, Arguments: VaList) -> DWORD;

    /// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
}

/// See [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
pub fn get_process_handle() -> HMODULE {
    unsafe { GetModuleHandleW(null()) }
}

pub type LPWSTR = *mut WCHAR;
pub type ULongPtr = usize;
/// [`MAKEINTRESOURCEW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
pub const fn makeintresourcew(i: WORD) -> LPWSTR {
    i as ULongPtr as LPWSTR
}

pub fn load_predefined_cursor(cursor: IDCursor) -> Result<HCURSOR, Win32Error> {
    // Safety: The enum only allows values from the approved list. See MSDN.
    let hcursor =
        unsafe { LoadCursorW(null_mut(), makeintresourcew(cursor as WORD)) };
    if hcursor.is_null() {
        Err(get_last_error())
    } else {
        Ok(hcursor)
    }
}

pub type ATOM = WORD;
pub type WORD = CUshort;
pub type CUshort = u16;
pub type HMENU = HANDLE;
pub type LPVOID = *mut core::ffi::c_void;
#[link(name = "User32")]
extern "system" {
    /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;

    /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: CInt,
        Y: CInt,
        nWidth: CInt,
        nHeight: CInt,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;

    /// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
    pub fn BeginPaint(hWnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HDC;

    /// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> CInt;

    /// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
}

/// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
pub unsafe fn fill_rect_with_sys_color(hdc: HDC, rect: &RECT, color: SysColor) -> Result<(), ()> {
    if FillRect(hdc, rect, (color as u32 + 1) as HBRUSH) != 0 {
      Ok(())
    } else {
      Err(())
    }
}

/// See [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
pub unsafe fn register_class(window_class: &WNDCLASSW) -> Result<ATOM, Win32Error>{
    let atom = RegisterClassW(window_class);
    if atom == 0 {
        Err(get_last_error())
    } else {
        Ok(atom)
    }
}

pub type BOOL = CInt;
#[link(name = "User32")]
extern "system" {
    /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: CInt) -> BOOL;

    /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;

    /// [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(
        lpMsg: *mut MSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;

    /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;

    /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExitCode: CInt);

    /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

    /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

    /// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

    /// [`SetCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursor)
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;

    /// [`MessageBoxW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, utype: UINT) -> CInt;

    /// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: CInt, dwNewLong: LongPtr) -> LongPtr;

    /// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: CInt) -> LongPtr;
}

type PGradientTriangle = *const c_void;
type PTRIVERTIX = *const TRIVERTIX;
#[link(name = "Msimg32")]
extern "system" {
    /// [`GradientFill `](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn GradientFill(hdc: HDC, PVertex: PTRIVERTIX, NVertex: CUlong, PMesh: PGradientTriangle, NMesh: CUlong, UlMode: CUlong) -> BOOL;
}

// Macro to automatically apply an impl, sets all values for the initialised type to 0
macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

unsafe_impl_default_zeroed!(MSG);
unsafe_impl_default_zeroed!(PAINTSTRUCT);
unsafe_impl_default_zeroed!(RECT);
unsafe_impl_default_zeroed!(POINT);
unsafe_impl_default_zeroed!(CREATESTRUCTW);
unsafe_impl_default_zeroed!(TRIVERTIX);
unsafe_impl_default_zeroed!(GradientTriangle);