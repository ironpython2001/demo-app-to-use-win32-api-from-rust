fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA,
        Windows::Win32::UI::WindowsAndMessaging::SC_SCREENSAVE,
        Windows::Win32::UI::WindowsAndMessaging::SendMessageA,
        Windows::Win32::UI::WindowsAndMessaging::WM_SYSCOMMAND,
        Windows::Win32::UI::WindowsAndMessaging::SC_SCREENSAVE,
        Windows::Win32::Foundation::WPARAM,
        Windows::Win32::Foundation::LPARAM,
        Windows::Win32::Foundation::LRESULT,
        Windows::Win32::System::Diagnostics::Debug::MessageBeep
    };
}