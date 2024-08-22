// We need void pointers often due C using them
use core::ptr::null_mut;
use std::ffi::c_void;

use triangle_from_scratch::win32::*;

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
            //println!("Current ptr: {}", *ptr);
            *ptr += 1;
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(h_wnd, &mut ps);
            let _success = FillRect(hdc, &ps.rc_paint, (COLOR_WINDOW + 1) as HBRUSH);

            // Create an array of TRIVERTEX structures that describe
            // positional and color values for each vertex.
            let vertex = [ 
            TRIVERTIX { 
                x: 150,
                y: 0,
                red: 0xff00,
                green: 0x8000,
                blue: 0x0000,
                alpha: 0x0000},
            TRIVERTIX { 
                x: 0,
                y: 150,
                red: 0x9000,
                green: 0x0000,
                blue: 0x9000,
                alpha: 0x0000},
            TRIVERTIX { 
                x: 300,
                y: 150,
                red: 0x900,
                green: 0x8000,
                blue: 0x9000,
                alpha: 0x0000}
            ];

            // Create a GRADIENT_TRIANGLE structure that
            // references the TRIVERTEX vertices.
            let g_triangle: *const GradientTriangle = &GradientTriangle {vertex1: 0, vertex2: 1, vertex3: 2};
            // Draw a shaded triangle.
            let _triangle_check = unsafe {GradientFill(hdc, &vertex[0], 
                3, g_triangle as *const c_void, 
                1, GRADIENT_FILL_TRIANGLE)};
            //println!("{}", triangle_check);
            EndPaint(h_wnd, &ps);
        }
        WM_NCCREATE => {
            println!("NC Create");
            let createstruct: *mut CREATESTRUCTW = l_param as *mut _;
            if createstruct.is_null() {
                return 0;
            }
            let boxed_i32_ptr: *mut i32 = (*createstruct).lp_create_params.cast();
            SetWindowLongPtrW(h_wnd, GWLP_USERDATA, boxed_i32_ptr as LongPtr);
            return 1;
        }
        WM_CREATE => println!("Create"),
        _ => return DefWindowProcW(h_wnd, msg, w_param, l_param),
    }
    0
}


fn main() {
    let h_instance = get_process_handle();
    let sample_window_class_wn = wide_null("Sample Window Class");
    let sample_window_name_wn = wide_null("Sample Window Name");
    let lparam: *mut i32 = Box::leak(Box::new(5_i32));

    let mut wc = WNDCLASSW::default();
    wc.lpfn_wnd_proc = Some(window_procedure);
    wc.h_instance = h_instance;
    wc.lpsz_class_name = sample_window_class_wn.as_ptr();
    wc.h_cursor = load_predefined_cursor(IDCursor::Arrow).unwrap();

    
    let _atom = unsafe { register_class(&wc) }.unwrap_or_else(|_| {
      let last_error = unsafe { GetLastError() };
      panic!("Could not register the window class, error code: {}", last_error);
    });

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
