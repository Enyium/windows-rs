#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRadio(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadio {
    type Vtable = IRadio_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadio {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x252118df_b33e_416a_875f_1cf38ae2d83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadio_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RadioState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RadioState) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RadioKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRadioStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadioStatics {
    type Vtable = IRadioStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadioStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fb6a12e_67cb_46ae_aae9_65919f86eff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRadiosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRadiosAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Radio(::windows_core::IUnknown);
impl Radio {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetStateAsync(&self, value: RadioState) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<RadioAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetStateAsync)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<Radio, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<RadioState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<RadioKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRadiosAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Radio>>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRadiosAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Radio>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<RadioAccessStatus>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadioStatics<R, F: FnOnce(&IRadioStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Radio, IRadioStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Radio {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Radio {
    type Vtable = IRadio_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Radio {
    const IID: ::windows_core::GUID = <IRadio as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Radio {
    const NAME: &'static str = "Windows.Devices.Radios.Radio";
}
::windows_core::imp::interface_hierarchy!(Radio, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Radio {}
unsafe impl ::core::marker::Sync for Radio {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RadioAccessStatus(pub i32);
impl RadioAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioAccessStatus {}
impl ::core::clone::Clone for RadioAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadioAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RadioAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RadioAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RadioAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RadioKind(pub i32);
impl RadioKind {
    pub const Other: Self = Self(0i32);
    pub const WiFi: Self = Self(1i32);
    pub const MobileBroadband: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const FM: Self = Self(4i32);
}
impl ::core::marker::Copy for RadioKind {}
impl ::core::clone::Clone for RadioKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadioKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RadioKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RadioKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RadioKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RadioState(pub i32);
impl RadioState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioState {}
impl ::core::clone::Clone for RadioState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadioState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RadioState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RadioState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RadioState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioState;i4)");
}
