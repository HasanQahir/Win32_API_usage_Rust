// We need void pointers often due C using them
use core::ptr::{null, null_mut};

// A load of type aliases for the windows C functions
// All original types are kept, even if just repeats, for ease of understanding
// Aliases to literals and primitive types in C
type CUint = u32;
type CInt = i32;
type UintPtr = usize;
type LongPtr = isize;
type LONG = CLong;
type CLong = i32;
type BYTE = u8;
type WcharT = u16;
type PVOID = *mut core::ffi::c_void;
// Aliases of types which are defined as Aliases in C (i.e. Those above).
type LPARAM = LongPtr;
type LRESULT = LongPtr;
type WPARAM = UintPtr;
type UINT = CUint;
type LPCWSTR = *const WCHAR;
type WCHAR = WcharT;
type HANDLE = PVOID;
// Lastly, the type aliases which refer to those prior
type HDC = HANDLE;
type HINSTANCE = HANDLE;
type HMODULE = HINSTANCE;
type HICON = HANDLE;
type HCURSOR = HICON;
type HBRUSH = HANDLE;
type HWND = HANDLE;

//A type macro which calls the WNDPROC function, placed in an Option<> as C functions can return NULL.
type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

// Values defined by Windows for interactions with the OS
const WS_OVERLAPPED: u32 = 0x00000000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const CW_USEDEFAULT: CInt = 0x80000000_u32 as CInt;
const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
const COLOR_WINDOW: u32 = 5;
#[allow(dead_code)]
const MB_OKCANCEL: u32 = 1;
#[allow(dead_code)]
const IDOK: CInt = 1;
const GWLP_USERDATA: CInt = -21;
const IDC_ARROW: LPCWSTR = makeintresourcew(32512);

// Values used by the Window_Procedure to check for events we care about
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;

// C structs converted into Rust Structs
#[repr(C)]
pub struct WNDCLASSW {
    style: UINT,
    lpfn_wnd_proc: WNDPROC,
    cb_cls_extra: CInt,
    cb_wnd_extra: CInt,
    h_instance: HINSTANCE,
    h_icon: HICON,
    h_cursor: HCURSOR,
    hbr_background: HBRUSH,
    lpsz_menu_name: LPCWSTR,
    lpsz_class_name: LPCWSTR,
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
    hwnd: HWND,
    message: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
    time: DWORD,
    pt: POINT,
    l_private: DWORD,
}

#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    f_erase: BOOL,
    rc_paint: RECT,
    f_restore: BOOL,
    f_inc_update: BOOL,
    rgb_reserved: [BYTE; 32],
}

#[repr(C)]
pub struct POINT {
    x: LONG,
    y: LONG,
}

#[repr(C)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

#[repr(C)]
pub struct CREATESTRUCTW {
    lp_create_params: LPVOID,
    h_instance: HINSTANCE,
    h_menu: HMENU,
    hwnd_parent: HWND,
    cy: CInt,
    cx: CInt,
    y: CInt,
    x: CInt,
    style: LONG,
    lpsz_name: LPCWSTR,
    lpsz_class: LPCWSTR,
    dw_ex_style: DWORD,
}

//A bunch of C functions defined in various header files, any new type aliases are kept here with them.
type DWORD = CUlong;
type CUlong = u32;
#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
}

type LPWSTR = *mut WCHAR;
type ULongPtr = usize;
/// [`MAKEINTRESOURCEW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
pub const fn makeintresourcew(i: WORD) -> LPWSTR {
    i as ULongPtr as LPWSTR
}

type ATOM = WORD;
type WORD = CUshort;
type CUshort = u16;
type HMENU = HANDLE;
type LPVOID = *mut core::ffi::c_void;
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

const SW_SHOW: CInt = 5;
type BOOL = CInt;
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
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> CInt;

    /// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: CInt, dwNewLong: LongPtr) -> LongPtr;

    /// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: CInt) -> LongPtr;
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

// This function defines all events which the generated window will respond to.
// If the code given to us through Msg is not defined and included in the match
// statement, the window will continue its default procedure instead.
pub unsafe extern "system" fn window_procedure(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => drop(DestroyWindow(h_wnd)),
        WM_DESTROY => {
            let ptr = GetWindowLongPtrW(h_wnd, GWLP_USERDATA) as *mut i32;
            drop(Box::from_raw(ptr));
            println!("Cleaned up the box.");
            PostQuitMessage(0)
        }
        WM_PAINT => {
            let ptr = GetWindowLongPtrW(h_wnd, GWLP_USERDATA) as *mut i32;
            // println!("Current ptr: {}", *ptr);
            *ptr += 1;
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(h_wnd, &mut ps);
            let _success = FillRect(hdc, &ps.rc_paint, (COLOR_WINDOW + 1) as HBRUSH);
            EndPaint(h_wnd, &ps);
        }
        WM_NCCREATE => {
            // println!("NC Create");
            let createstruct: *mut CREATESTRUCTW = l_param as *mut _;
            if createstruct.is_null() {
                return 0;
            }
            let boxed_i32_ptr: *mut i32 = (*createstruct).lp_create_params.cast();
            SetWindowLongPtrW(h_wnd, GWLP_USERDATA, boxed_i32_ptr as LongPtr);
            return 1;
        }
        // WM_CREATE => println!("Create"),
        _ => return DefWindowProcW(h_wnd, msg, w_param, l_param),
    }
    0
}

unsafe_impl_default_zeroed!(MSG);
unsafe_impl_default_zeroed!(PAINTSTRUCT);
unsafe_impl_default_zeroed!(RECT);
unsafe_impl_default_zeroed!(POINT);
unsafe_impl_default_zeroed!(CREATESTRUCTW);

fn main() {
    let h_instance = unsafe { GetModuleHandleW(null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");
    let sample_window_name_wn = wide_null("Sample Window Name");
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));

    let mut wc = WNDCLASSW::default();
    wc.lpfn_wnd_proc = Some(window_procedure);
    wc.h_instance = h_instance;
    wc.lpsz_class_name = sample_window_class_wn.as_ptr();
    wc.h_cursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };

    let atom = unsafe { RegisterClassW(&wc) };
    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!(
            "Could not register the window class, error code: {}",
            last_error
        );
    }

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            sample_window_class_wn.as_ptr(),
            sample_window_name_wn.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            h_instance,
            lparam.cast(),
        )
    };
    if hwnd.is_null() {
        panic!("Failed to create a window.");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };
    let mut msg = MSG::default();

    loop {
        let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageW`, error code: {}", last_error);
        } else {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
