#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILanguageFont(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageFont {
    type Vtable = ILanguageFont_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILanguageFont {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb12e5c3a_b76d_459b_beeb_901151cd77d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFont_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontWeight) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStretch) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::FontStyle) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    pub ScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILanguageFontGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageFontGroup {
    type Vtable = ILanguageFontGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILanguageFontGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf33a7fc3_3a5c_4aea_b9ff_b39fb242f7f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFontGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UITextFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UIHeadingFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UITitleFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UICaptionFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UINotificationHeadingFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TraditionalDocumentFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ModernDocumentFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DocumentHeadingFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FixedWidthTextFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DocumentAlternate1Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DocumentAlternate2Font: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILanguageFontGroupFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanguageFontGroupFactory {
    type Vtable = ILanguageFontGroupFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILanguageFontGroupFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcaeac67_4e77_49c7_b856_dde934fc735b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageFontGroupFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateLanguageFontGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LanguageFont(::windows_core::IUnknown);
impl LanguageFont {
    pub fn FontFamily(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FontFamily(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Text\"`"]
    #[cfg(feature = "UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<super::super::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FontWeight(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Text\"`"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<super::super::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FontStretch(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Text\"`"]
    #[cfg(feature = "UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<super::super::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FontStyle(&mut result__)).from_abi(result__)
        }
    }
    pub fn ScaleFactor(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ScaleFactor(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LanguageFont {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Fonts.LanguageFont;{b12e5c3a-b76d-459b-beeb-901151cd77d1})");
}
unsafe impl ::windows_core::Interface for LanguageFont {
    type Vtable = ILanguageFont_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LanguageFont {
    const IID: ::windows_core::GUID = <ILanguageFont as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LanguageFont {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFont";
}
::windows_core::imp::interface_hierarchy!(LanguageFont, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LanguageFont {}
unsafe impl ::core::marker::Sync for LanguageFont {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LanguageFontGroup(::windows_core::IUnknown);
impl LanguageFontGroup {
    pub fn UITextFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UITextFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn UIHeadingFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UIHeadingFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn UITitleFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UITitleFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn UICaptionFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UICaptionFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn UINotificationHeadingFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UINotificationHeadingFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn TraditionalDocumentFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TraditionalDocumentFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn ModernDocumentFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ModernDocumentFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn DocumentHeadingFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DocumentHeadingFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn FixedWidthTextFont(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FixedWidthTextFont(&mut result__)).from_abi(result__)
        }
    }
    pub fn DocumentAlternate1Font(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DocumentAlternate1Font(&mut result__)).from_abi(result__)
        }
    }
    pub fn DocumentAlternate2Font(&self) -> ::windows_core::Result<LanguageFont> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DocumentAlternate2Font(&mut result__)).from_abi(result__)
        }
    }
    pub fn CreateLanguageFontGroup(languagetag: &::windows_core::HSTRING) -> ::windows_core::Result<LanguageFontGroup> {
        Self::ILanguageFontGroupFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateLanguageFontGroup(::core::mem::transmute_copy(languagetag), &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILanguageFontGroupFactory<R, F: FnOnce(&ILanguageFontGroupFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LanguageFontGroup, ILanguageFontGroupFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LanguageFontGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Globalization.Fonts.LanguageFontGroup;{f33a7fc3-3a5c-4aea-b9ff-b39fb242f7f6})");
}
unsafe impl ::windows_core::Interface for LanguageFontGroup {
    type Vtable = ILanguageFontGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LanguageFontGroup {
    const IID: ::windows_core::GUID = <ILanguageFontGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LanguageFontGroup {
    const NAME: &'static str = "Windows.Globalization.Fonts.LanguageFontGroup";
}
::windows_core::imp::interface_hierarchy!(LanguageFontGroup, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LanguageFontGroup {}
unsafe impl ::core::marker::Sync for LanguageFontGroup {}
