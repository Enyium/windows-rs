#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScreenReaderPositionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScreenReaderPositionChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x557eb5e5_54d0_5ccd_9fc5_ed33357f8a9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderPositionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ScreenPositionInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenPositionInRawPixels: usize,
    pub IsReadingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScreenReaderService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScreenReaderService {
    type Vtable = IScreenReaderService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScreenReaderService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19475427_eac0_50d3_bdd9_9b487a226256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenReaderService_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentScreenReaderPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenReaderPositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScreenReaderPositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScreenReaderPositionChanged: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ScreenReaderPositionChangedEventArgs(::windows_core::IUnknown);
impl ScreenReaderPositionChangedEventArgs {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenPositionInRawPixels(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScreenPositionInRawPixels)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsReadingText(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadingText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ScreenReaderPositionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ScreenReaderPositionChangedEventArgs {
    type Vtable = IScreenReaderPositionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScreenReaderPositionChangedEventArgs {
    const IID: ::windows_core::GUID = <IScreenReaderPositionChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScreenReaderPositionChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderPositionChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ScreenReaderPositionChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ScreenReaderPositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for ScreenReaderPositionChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ScreenReaderService(::windows_core::IUnknown);
impl ScreenReaderService {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ScreenReaderService, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CurrentScreenReaderPosition(&self) -> ::windows_core::Result<ScreenReaderPositionChangedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentScreenReaderPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenReaderPositionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ScreenReaderService, ScreenReaderPositionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScreenReaderPositionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScreenReaderPositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScreenReaderPositionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for ScreenReaderService {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ScreenReaderService {
    type Vtable = IScreenReaderService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScreenReaderService {
    const IID: ::windows_core::GUID = <IScreenReaderService as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScreenReaderService {
    const NAME: &'static str = "Windows.UI.Accessibility.ScreenReaderService";
}
::windows_core::imp::interface_hierarchy!(ScreenReaderService, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ScreenReaderService {}
unsafe impl ::core::marker::Sync for ScreenReaderService {}
