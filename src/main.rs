// use bindings::Windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
// use bindings::Windows::Win32::UI::WindowsAndMessaging::IDCANCEL;
// use bindings::Windows::Win32::UI::WindowsAndMessaging::IDOK;
// use bindings::Windows::Win32::UI::WindowsAndMessaging::MB_OKCANCEL;
use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::System::Diagnostics::Debug::*;
use bindings::Windows::Win32::UI::WindowsAndMessaging::*;
use std::convert::TryFrom;

fn main() -> windows::Result<()> {
    unsafe {
        let result = MessageBoxA(None, "Do you want Play Sound..?", "Play Sound", MB_OKCANCEL);

        if result == IDCANCEL {
            println!("cancel");
        } else if result == IDOK {
            //let i = WPARAM(WM_SYSCOMMAND.try_into().unwrap());
            //let x = usize::try_from(WM_SYSCOMMAND).unwrap();
            let x = usize::try_from(0x112).unwrap();
            let i = WPARAM(x);
            let y = isize::try_from(0xF140).unwrap();
            //let j = LPARAM(SC_SCREENSAVE.try_into().unwrap());
            //let y = isize.tr SC_SCREENSAVE);
            let j = LPARAM(y);
            //turn scren save on
            SendMessageA(None, 0, i, j);
            let beepresult = MessageBeep(u32::MAX);
            println!("{}", beepresult.as_bool());
            print!("ok");
        }
    }
    return Ok(());
}
