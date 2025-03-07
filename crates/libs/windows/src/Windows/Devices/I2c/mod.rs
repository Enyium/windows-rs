#[cfg(feature = "Devices_I2c_Provider")]
#[doc = "Required features: `\"Devices_I2c_Provider\"`"]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct II2cConnectionSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cConnectionSettings {
    type Vtable = II2cConnectionSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for II2cConnectionSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2db1307_ab6f_4639_a767_54536dc3460f);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut I2cBusSpeed) -> ::windows_core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: I2cBusSpeed) -> ::windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut I2cSharingMode) -> ::windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: I2cSharingMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct II2cConnectionSettingsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cConnectionSettingsFactory {
    type Vtable = II2cConnectionSettingsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for II2cConnectionSettingsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81b586b3_9693_41b1_a243_ded4f6e66926);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettingsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slaveaddress: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct II2cController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cController {
    type Vtable = II2cController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for II2cController {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc48ab1b2_87a0_4166_8e3e_b4b8f97cd729);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct II2cControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cControllerStatics {
    type Vtable = II2cControllerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for II2cControllerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40fc0365_5f05_4e7e_84bd_100db8e0aec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct II2cDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for II2cDevice {
    type Vtable = II2cDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for II2cDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8636c136_b9c5_4f70_9449_cc46dc6f57eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> ::windows_core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut I2cTransferResult) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> ::windows_core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows_core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> ::windows_core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut I2cTransferResult) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct II2cDeviceStatics(::windows_core::IUnknown);
impl II2cDeviceStatics {
    pub fn GetDeviceSelector(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<P0>(&self, deviceid: &::windows_core::HSTRING, settings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>>
    where
        P0: ::windows_core::IntoParam<I2cConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), settings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(II2cDeviceStatics, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for II2cDeviceStatics {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for II2cDeviceStatics {
    type Vtable = II2cDeviceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for II2cDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91a33be3_7334_4512_96bc_fbae9459f5f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct I2cConnectionSettings(::windows_core::IUnknown);
impl I2cConnectionSettings {
    pub fn SlaveAddress(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlaveAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSlaveAddress(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSlaveAddress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BusSpeed(&self) -> ::windows_core::Result<I2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BusSpeed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBusSpeed(&self, value: I2cBusSpeed) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBusSpeed)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> ::windows_core::Result<I2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SharingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSharingMode(&self, value: I2cSharingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSharingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(slaveaddress: i32) -> ::windows_core::Result<I2cConnectionSettings> {
        Self::II2cConnectionSettingsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), slaveaddress, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn II2cConnectionSettingsFactory<R, F: FnOnce(&II2cConnectionSettingsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<I2cConnectionSettings, II2cConnectionSettingsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for I2cConnectionSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for I2cConnectionSettings {
    type Vtable = II2cConnectionSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for I2cConnectionSettings {
    const IID: ::windows_core::GUID = <II2cConnectionSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for I2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.I2cConnectionSettings";
}
::windows_core::imp::interface_hierarchy!(I2cConnectionSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for I2cConnectionSettings {}
unsafe impl ::core::marker::Sync for I2cConnectionSettings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct I2cController(::windows_core::IUnknown);
impl I2cController {
    pub fn GetDevice<P0>(&self, settings: P0) -> ::windows_core::Result<I2cDevice>
    where
        P0: ::windows_core::IntoParam<I2cConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDevice)(::windows_core::Interface::as_raw(this), settings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_I2c_Provider\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_I2c_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<P0>(provider: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<I2cController>>>
    where
        P0: ::windows_core::TryIntoParam<Provider::II2cProvider>,
    {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetControllersAsync)(::windows_core::Interface::as_raw(this), provider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<I2cController>> {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn II2cControllerStatics<R, F: FnOnce(&II2cControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<I2cController, II2cControllerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for I2cController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for I2cController {
    type Vtable = II2cController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for I2cController {
    const IID: ::windows_core::GUID = <II2cController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for I2cController {
    const NAME: &'static str = "Windows.Devices.I2c.I2cController";
}
::windows_core::imp::interface_hierarchy!(I2cController, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for I2cController {}
unsafe impl ::core::marker::Sync for I2cController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct I2cDevice(::windows_core::IUnknown);
impl I2cDevice {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConnectionSettings(&self) -> ::windows_core::Result<I2cConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Write)(::windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_ptr()).ok() }
    }
    pub fn WritePartial(&self, buffer: &[u8]) -> ::windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WritePartial)(::windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Read)(::windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_mut_ptr()).ok() }
    }
    pub fn ReadPartial(&self, buffer: &mut [u8]) -> ::windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadPartial)(::windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_mut_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WriteRead)(::windows_core::Interface::as_raw(this), writebuffer.len().try_into().unwrap(), writebuffer.as_ptr(), readbuffer.len().try_into().unwrap(), readbuffer.as_mut_ptr()).ok() }
    }
    pub fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> ::windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteReadPartial)(::windows_core::Interface::as_raw(this), writebuffer.len().try_into().unwrap(), writebuffer.as_ptr(), readbuffer.len().try_into().unwrap(), readbuffer.as_mut_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName(friendlyname: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(friendlyname), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<P0>(deviceid: &::windows_core::HSTRING, settings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>>
    where
        P0: ::windows_core::IntoParam<I2cConnectionSettings>,
    {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), settings.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn II2cDeviceStatics<R, F: FnOnce(&II2cDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<I2cDevice, II2cDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for I2cDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for I2cDevice {
    type Vtable = II2cDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for I2cDevice {
    const IID: ::windows_core::GUID = <II2cDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for I2cDevice {
    const NAME: &'static str = "Windows.Devices.I2c.I2cDevice";
}
::windows_core::imp::interface_hierarchy!(I2cDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for I2cDevice {}
unsafe impl ::core::marker::Send for I2cDevice {}
unsafe impl ::core::marker::Sync for I2cDevice {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cBusSpeed {}
impl ::core::clone::Clone for I2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cBusSpeed {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for I2cBusSpeed {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for I2cBusSpeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cBusSpeed").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for I2cBusSpeed {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cBusSpeed;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cSharingMode {}
impl ::core::clone::Clone for I2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for I2cSharingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for I2cSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cSharingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for I2cSharingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cSharingMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
    pub const ClockStretchTimeout: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for I2cTransferStatus {}
impl ::core::clone::Clone for I2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for I2cTransferStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for I2cTransferStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for I2cTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cTransferStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for I2cTransferStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cTransferStatus;i4)");
}
#[repr(C)]
pub struct I2cTransferResult {
    pub Status: I2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for I2cTransferResult {}
impl ::core::clone::Clone for I2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for I2cTransferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("I2cTransferResult").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
impl ::windows_core::TypeKind for I2cTransferResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for I2cTransferResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.I2cTransferResult;enum(Windows.Devices.I2c.I2cTransferStatus;i4);u4)");
}
impl ::core::cmp::PartialEq for I2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred
    }
}
impl ::core::cmp::Eq for I2cTransferResult {}
impl ::core::default::Default for I2cTransferResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
