//windows

#[cfg(target_os = "windows")]
mod platform {
    use std::ptr::{null, null_mut};
    use winapi::um::winuser::*;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::shared::windef::HWND;

    pub fn create_window(title: &str, width: u32, height: u32) {
        unsafe {
            let h_instance = GetModuleHandleW(null());
            let class_name = wide_null("CrossPlatformWindowClass");

            let wnd_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(def_window_proc),
                hInstance: h_instance,
                lpszClassName: class_name.as_ptr(),
                hbrBackground: (COLOR_WINDOW + 1) as _,
                hCursor: LoadCursorW(null_mut(), IDC_ARROW),
                lpszMenuName: null(),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: null_mut(),
            };

            RegisterClassW(&wnd_class);

            let hwnd: HWND = CreateWindowExW(
                0,
                class_name.as_ptr(),
                wide_null(title).as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                null_mut(),
                null_mut(),
                h_instance,
                null_mut(),
            );

            let mut msg = std::mem::zeroed();
            while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    extern "system" fn def_window_proc(hwnd: HWND, msg: u32, w_param: usize, l_param: isize) -> isize {
        match msg {
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                0
            }
            _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
        }
    }

    fn wide_null(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
}

//linux
#[cfg(target_os = "linux")]
mod platform {
    use std::ptr;
    use x11::xlib::*;

    pub fn create_window(title: &str, width: u32, height: u32) {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                panic!("Cannot open X display");
            }

            let screen = XDefaultScreen(display);
            let root = XRootWindow(display, screen);

            let window = XCreateSimpleWindow(
                display,
                root,
                0,
                0,
                width,
                height,
                1,
                XBlackPixel(display, screen),
                XWhitePixel(display, screen),
            );

            XStoreName(display, window, title.as_ptr() as *const i8);
            XSelectInput(display, window, ExposureMask | KeyPressMask);
            XMapWindow(display, window);

            let mut event = std::mem::zeroed();
            loop {
                XNextEvent(display, &mut event);
                if event.type_ == KeyPress {
                    break;
                }
            }

            XDestroyWindow(display, window);
            XCloseDisplay(display);
        }
    }
}
