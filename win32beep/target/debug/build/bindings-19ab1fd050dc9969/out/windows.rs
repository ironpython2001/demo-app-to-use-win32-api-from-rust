#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
                type DefaultType = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::Error::from_win32())
                    }
                }
                #[inline]
                #[track_caller]
                pub fn unwrap(self) {
                    self.ok().unwrap();
                }
                #[inline]
                #[track_caller]
                pub fn expect(self, msg: &str) {
                    self.ok().expect(msg);
                }
            }
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct HWND(pub isize);
            impl ::std::default::Default for HWND {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            unsafe impl ::windows::Handle for HWND {}
            unsafe impl ::windows::Abi for HWND {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct LPARAM(pub isize);
            impl ::std::default::Default for LPARAM {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            unsafe impl ::windows::Handle for LPARAM {}
            unsafe impl ::windows::Abi for LPARAM {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct LRESULT(pub i32);
            impl ::std::default::Default for LRESULT {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            unsafe impl ::windows::Handle for LRESULT {}
            unsafe impl ::windows::Abi for LRESULT {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            unsafe impl ::windows::Abi for PSTR {
                type Abi = Self;
                type DefaultType = Self;
                unsafe fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct WPARAM(pub usize);
            impl ::std::default::Default for WPARAM {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            unsafe impl ::windows::Handle for WPARAM {}
            unsafe impl ::windows::Abi for WPARAM {
                type Abi = Self;
                type DefaultType = Self;
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Diagnostics {
                #[allow(
                    unused_variables,
                    non_upper_case_globals,
                    non_snake_case,
                    unused_unsafe,
                    non_camel_case_types,
                    dead_code,
                    clippy::all
                )]
                pub mod Debug {
                    pub unsafe fn MessageBeep(utype: u32) -> super::super::super::Foundation::BOOL {
                        #[cfg(windows)]
                        {
                            #[link(name = "user32")]
                            extern "system" {
                                fn MessageBeep(utype: u32)
                                    -> super::super::super::Foundation::BOOL;
                            }
                            ::std::mem::transmute(MessageBeep(::std::mem::transmute(utype)))
                        }
                        #[cfg(not(windows))]
                        unimplemented!("Unsupported target OS");
                    }
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod UI {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WindowsAndMessaging {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct MESSAGEBOX_RESULT(pub i32);
                pub const IDOK: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(1i32);
                pub const IDCANCEL: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(2i32);
                pub const IDABORT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(3i32);
                pub const IDRETRY: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(4i32);
                pub const IDIGNORE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(5i32);
                pub const IDYES: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(6i32);
                pub const IDNO: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(7i32);
                pub const IDCLOSE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(8i32);
                pub const IDHELP: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(9i32);
                pub const IDTRYAGAIN: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(10i32);
                pub const IDCONTINUE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(11i32);
                pub const IDASYNC: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32001i32);
                pub const IDTIMEOUT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32000i32);
                impl ::std::convert::From<i32> for MESSAGEBOX_RESULT {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for MESSAGEBOX_RESULT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct MESSAGEBOX_STYLE(pub u32);
                pub const MB_ABORTRETRYIGNORE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2u32);
                pub const MB_CANCELTRYCONTINUE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(6u32);
                pub const MB_HELP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16384u32);
                pub const MB_OK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
                pub const MB_OKCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1u32);
                pub const MB_RETRYCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(5u32);
                pub const MB_YESNO: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4u32);
                pub const MB_YESNOCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3u32);
                pub const MB_ICONHAND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
                pub const MB_ICONQUESTION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32u32);
                pub const MB_ICONEXCLAMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
                pub const MB_ICONASTERISK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
                pub const MB_USERICON: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(128u32);
                pub const MB_ICONWARNING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
                pub const MB_ICONERROR: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
                pub const MB_ICONINFORMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
                pub const MB_ICONSTOP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
                pub const MB_DEFBUTTON1: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
                pub const MB_DEFBUTTON2: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(256u32);
                pub const MB_DEFBUTTON3: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(512u32);
                pub const MB_DEFBUTTON4: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(768u32);
                pub const MB_APPLMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
                pub const MB_SYSTEMMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4096u32);
                pub const MB_TASKMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(8192u32);
                pub const MB_NOFOCUS: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32768u32);
                pub const MB_SETFOREGROUND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(65536u32);
                pub const MB_DEFAULT_DESKTOP_ONLY: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(131072u32);
                pub const MB_TOPMOST: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
                pub const MB_RIGHT: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(524288u32);
                pub const MB_RTLREADING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1048576u32);
                pub const MB_SERVICE_NOTIFICATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2097152u32);
                pub const MB_SERVICE_NOTIFICATION_NT3X: MESSAGEBOX_STYLE =
                    MESSAGEBOX_STYLE(262144u32);
                pub const MB_TYPEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(15u32);
                pub const MB_ICONMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(240u32);
                pub const MB_DEFMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3840u32);
                pub const MB_MODEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(12288u32);
                pub const MB_MISCMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(49152u32);
                impl ::std::convert::From<u32> for MESSAGEBOX_STYLE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for MESSAGEBOX_STYLE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for MESSAGEBOX_STYLE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for MESSAGEBOX_STYLE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub unsafe fn MessageBoxA<
                    'a,
                    Param0: ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    Param1: ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                    Param2: ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                >(
                    hwnd: Param0,
                    lptext: Param1,
                    lpcaption: Param2,
                    utype: MESSAGEBOX_STYLE,
                ) -> MESSAGEBOX_RESULT {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn MessageBoxA(
                                hwnd: super::super::Foundation::HWND,
                                lptext: super::super::Foundation::PSTR,
                                lpcaption: super::super::Foundation::PSTR,
                                utype: MESSAGEBOX_STYLE,
                            ) -> MESSAGEBOX_RESULT;
                        }
                        ::std::mem::transmute(MessageBoxA(
                            hwnd.into_param().abi(),
                            lptext.into_param().abi(),
                            lpcaption.into_param().abi(),
                            ::std::mem::transmute(utype),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub const SC_SCREENSAVE: u32 = 61760u32;
                pub unsafe fn SendMessageA<
                    'a,
                    Param0: ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                >(
                    hwnd: Param0,
                    msg: u32,
                    wparam: super::super::Foundation::WPARAM,
                    lparam: super::super::Foundation::LPARAM,
                ) -> super::super::Foundation::LRESULT {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn SendMessageA(
                                hwnd: super::super::Foundation::HWND,
                                msg: u32,
                                wparam: super::super::Foundation::WPARAM,
                                lparam: super::super::Foundation::LPARAM,
                            ) -> super::super::Foundation::LRESULT;
                        }
                        ::std::mem::transmute(SendMessageA(
                            hwnd.into_param().abi(),
                            ::std::mem::transmute(msg),
                            ::std::mem::transmute(wparam),
                            ::std::mem::transmute(lparam),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub const WM_SYSCOMMAND: u32 = 274u32;
            }
        }
    }
}
