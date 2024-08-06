#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use core::ffi::c_void;
use core::panic::PanicInfo;

use windows_sys::Win32::System::Console::GetStdHandle;
use windows_sys::Win32::System::Console::WriteConsoleA;
use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;
use windows_sys::Win32::System::Threading::ExitProcess;

// used when `windows_subsystem = "windows"`
//use windows_sys::Win32::System::Console::AttachConsole;
//use windows_sys::Win32::System::Console::ATTACH_PARENT_PROCESS;


use prealloc::prealloc_from_config;

prealloc_from_config!("config.json");

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct MyItem {
    a: u32,
    b: u32,
}


#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    unsafe {
        ExitProcess(1);
    }
}

#[macro_export]
macro_rules! println {
    ($msg:tt) => {
        unsafe {
            // need this when `windows_subsystem = "windows"`
            // AttachConsole(ATTACH_PARENT_PROCESS);

            // get a handle to the console output buffer
            let console = GetStdHandle(STD_OUTPUT_HANDLE);

            // write the message to the console buffer
            // alternatively, `WriteFile` can be used in this case too, need additional feature flags for `windows-sys` crate
            WriteConsoleA(
                console,
                $msg.as_ptr().cast::<c_void>(),
                $msg.len() as u32,
                core::ptr::null_mut(),
                core::ptr::null(),
            );
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
fn mainCRTStartup() -> ! {
    
    const ITEM1_INITIALIZER: MyItem = MyItem { a: 1, b: 2 };
    assert_eq!(
        dispatch_static!(Item1, ITEM1_INITIALIZER),
        Some(&mut MyItem { a: 1, b: 2 })
    );

    let message = "hello world\n";

    println!(message);

    unsafe { ExitProcess(0) }
}
