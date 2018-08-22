#![windows_subsystem = "windows"] // Hides console window
extern crate winapi;

use std::mem;
use std::ptr::null_mut;

use winapi::ctypes::c_void;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::{HMONITOR, HDC, LPRECT, HWND};
use winapi::um::lowlevelmonitorconfigurationapi::SetVCPFeature;
use winapi::um::winuser::{GetMessageW,
	RegisterClassW, DefWindowProcW, CreateWindowExW,
	TranslateMessage, DispatchMessageW, EnumDisplayMonitors,
	CS_VREDRAW, CS_HREDRAW, MSG, CS_OWNDC, WNDCLASSW, WM_QUERYENDSESSION};
use winapi::um::physicalmonitorenumerationapi::{GetPhysicalMonitorsFromHMONITOR, PHYSICAL_MONITOR};

// Power states
enum Power {
	On = 0x01,
	Standby = 0x02,
	Suspend = 0x03,
	Off = 0x04,
	HardOff = 0x05
}

fn win32_string(str : &str) -> Vec<u16> {
	let mut vec: Vec<u16> = str.encode_utf16().collect();
	vec.push(0);
	vec
}

/*
* Sets the power state of `display_handle` to `state`
*/
fn set_power_state(display_handle: *mut c_void, state: Power) -> bool {
	if unsafe { SetVCPFeature(display_handle, 0xD6, state as u32)} == 0 { return false; }
	true
}

/*
* Returns a `Vec<*mut c_void>` containing display handles pointing to
* there respective displays.
*/
fn get_display_handles() -> Vec<*mut c_void> {
    let mut display_list: Vec<*mut c_void> = Vec::<*mut c_void>::new();
	let display_list_ptr: isize = &mut display_list as *mut _ as isize;

	unsafe {
		EnumDisplayMonitors(null_mut(), null_mut(), Some(monitor_enum_proc), display_list_ptr);
	}

	display_list.shrink_to_fit();
	display_list
}

/*
* Calls `set_power_state` for every display handle in `display_list`
* setting the power state to `Power::HardOff`.
*/
fn poweroff_displays(display_list: Vec<*mut c_void>) {
	for display in display_list {
		set_power_state(display, Power::HardOff);
	};
}

/*
* Main window loop
*/
fn run_loop(window: HWND) {
	unsafe {
		let mut message: MSG = mem::uninitialized();
		while GetMessageW(&mut message as *mut MSG, window, 0, 0) == 1 {
	        TranslateMessage(&message as *const MSG);
	        DispatchMessageW(&message as *const MSG);
		}
	}
}

/*
* A message only widnow used to receive a shutdown event
*/
fn create_window() -> HWND {
    let name = win32_string("dummy_class");
    let title = win32_string("ddc");
    let instance = unsafe { GetModuleHandleW(null_mut()) };

    unsafe {
        let wnd_class = WNDCLASSW {
            style : CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc : Some(win_proc),
            hInstance : instance,
            lpszClassName : name.as_ptr(),
            cbClsExtra : 0,
            cbWndExtra : 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        RegisterClassW(&wnd_class);

        CreateWindowExW(0, name.as_ptr(), title.as_ptr(), 0, 0, 0, 0, 0, null_mut(), null_mut(), instance, null_mut())
    }
}

/*
* Called by the windows api for every display
*/
unsafe extern "system" fn monitor_enum_proc(h_monitor: HMONITOR, _: HDC, _: LPRECT, data: isize) -> i32 {
	let display_list: &mut Vec<*mut c_void> = &mut *(data as *mut Vec<*mut c_void>);

	let mut temp_mon: Vec<PHYSICAL_MONITOR> = Vec::with_capacity(1);
	temp_mon.push(PHYSICAL_MONITOR{
		hPhysicalMonitor: null_mut(),
		szPhysicalMonitorDescription: [0_u16; 128]
	});

	GetPhysicalMonitorsFromHMONITOR(h_monitor, 1, temp_mon.as_mut_ptr());

	for monitor in temp_mon {
		display_list.push(monitor.hPhysicalMonitor);
	}
	
	1
}

/*
* Recives window messages. We only care about `WM_QUERYENDSESSION` as its the widnows shutdown event
*/
unsafe extern "system" fn win_proc(hwnd: HWND, msg: u32, w_param: usize, l_param: isize) -> isize {
	match msg {
	    WM_QUERYENDSESSION => {
	    	let display_handles: Vec<*mut c_void> = get_display_handles();
	    	poweroff_displays(display_handles);
	    },
	    _ => (),
	};
	DefWindowProcW(hwnd, msg, w_param, l_param)
}

fn main() {
	let window = create_window();
	run_loop(window);
}
