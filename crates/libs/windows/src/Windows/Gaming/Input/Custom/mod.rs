#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomGameControllerFactory(::windows_core::IUnknown);
impl ICustomGameControllerFactory {
    pub fn CreateGameController<P0>(&self, provider: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::TryIntoParam<IGameControllerProvider>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateGameController)(::windows_core::Interface::as_raw(this), provider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn OnGameControllerAdded<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IGameController>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnGameControllerAdded)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn OnGameControllerRemoved<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IGameController>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnGameControllerRemoved)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ICustomGameControllerFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ICustomGameControllerFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICustomGameControllerFactory {
    type Vtable = ICustomGameControllerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomGameControllerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69a0ae5e_758e_4cbe_ace6_62155fe9126f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomGameControllerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGameControllerFactoryManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameControllerFactoryManagerStatics {
    type Vtable = IGameControllerFactoryManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGameControllerFactoryManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36cb66e3_d0a1_4986_a24c_40b137deba9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RegisterCustomFactoryForGipInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, interfaceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RegisterCustomFactoryForHardwareId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows_core::HRESULT,
    pub RegisterCustomFactoryForXusbType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGameControllerFactoryManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameControllerFactoryManagerStatics2 {
    type Vtable = IGameControllerFactoryManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGameControllerFactoryManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeace5644_19df_4115_b32a_2793e2aea3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryGetFactoryControllerFromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, gamecontroller: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGameControllerInputSink(::windows_core::IUnknown);
impl IGameControllerInputSink {
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IGameControllerInputSink, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IGameControllerInputSink {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IGameControllerInputSink {
    type Vtable = IGameControllerInputSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGameControllerInputSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ff6f922_c640_4c78_a820_9a715c558bcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnInputResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows_core::HRESULT,
    pub OnInputSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGameControllerProvider(::windows_core::IUnknown);
impl IGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IGameControllerProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IGameControllerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IGameControllerProvider {
    type Vtable = IGameControllerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGameControllerProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6d73982_2996_4559_b16c_3e57d46e58d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FirmwareVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows_core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub HardwareVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGipFirmwareUpdateResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGipFirmwareUpdateResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b794d32_8553_4292_8e03_e16651a2f8bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FinalComponentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GipFirmwareUpdateStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGipGameControllerInputSink(::windows_core::IUnknown);
impl IGipGameControllerInputSink {
    pub fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnKeyReceived)(::windows_core::Interface::as_raw(this), timestamp, keycode, ispressed).ok() }
    }
    pub fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnMessageReceived)(::windows_core::Interface::as_raw(this), timestamp, messageclass, messageid, sequenceid, messagebuffer.len().try_into().unwrap(), messagebuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IGipGameControllerInputSink, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGameControllerInputSink> for IGipGameControllerInputSink {}
impl ::windows_core::RuntimeType for IGipGameControllerInputSink {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IGipGameControllerInputSink {
    type Vtable = IGipGameControllerInputSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGipGameControllerInputSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2108abf_09f1_43bc_a140_80f899ec36fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnKeyReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_core::HRESULT,
    pub OnMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGipGameControllerProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGipGameControllerProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbcf1e19_1af5_45a8_bf02_a0ee50c823fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_core::HRESULT,
    pub SendReceiveMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, requestMessageBuffer_array_size: u32, requestmessagebuffer: *const u8, responseMessageBuffer_array_size: u32, responsemessagebuffer: *mut u8) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UpdateFirmwareAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firmwareimage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UpdateFirmwareAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHidGameControllerInputSink(::windows_core::IUnknown);
impl IHidGameControllerInputSink {
    pub fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputReportReceived)(::windows_core::Interface::as_raw(this), timestamp, reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IHidGameControllerInputSink, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGameControllerInputSink> for IHidGameControllerInputSink {}
impl ::windows_core::RuntimeType for IHidGameControllerInputSink {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IHidGameControllerInputSink {
    type Vtable = IHidGameControllerInputSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHidGameControllerInputSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf754c322_182d_40e4_a126_fcee4ffa1e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnInputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHidGameControllerProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHidGameControllerProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95ce3af4_abf0_4b68_a081_3b7de73ff0e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub GetFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *mut u8) -> ::windows_core::HRESULT,
    pub SendFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_core::HRESULT,
    pub SendOutputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXusbGameControllerInputSink(::windows_core::IUnknown);
impl IXusbGameControllerInputSink {
    pub fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputReceived)(::windows_core::Interface::as_raw(this), timestamp, reportid, inputbuffer.len().try_into().unwrap(), inputbuffer.as_ptr()).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IXusbGameControllerInputSink, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGameControllerInputSink> for IXusbGameControllerInputSink {}
impl ::windows_core::RuntimeType for IXusbGameControllerInputSink {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IXusbGameControllerInputSink {
    type Vtable = IXusbGameControllerInputSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXusbGameControllerInputSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2ac1d95_6ecb_42b3_8aab_025401ca4712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnInputReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXusbGameControllerProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXusbGameControllerProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e2971eb_0efb_48b4_808b_837643b2f216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetVibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows_core::HRESULT,
}
pub struct GameControllerFactoryManager;
impl GameControllerFactoryManager {
    pub fn RegisterCustomFactoryForGipInterface<P0>(factory: P0, interfaceid: ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterCustomFactoryForGipInterface)(::windows_core::Interface::as_raw(this), factory.try_into_param()?.abi(), interfaceid).ok() })
    }
    pub fn RegisterCustomFactoryForHardwareId<P0>(factory: P0, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterCustomFactoryForHardwareId)(::windows_core::Interface::as_raw(this), factory.try_into_param()?.abi(), hardwarevendorid, hardwareproductid).ok() })
    }
    pub fn RegisterCustomFactoryForXusbType<P0>(factory: P0, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICustomGameControllerFactory>,
    {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterCustomFactoryForXusbType)(::windows_core::Interface::as_raw(this), factory.try_into_param()?.abi(), xusbtype, xusbsubtype).ok() })
    }
    pub fn TryGetFactoryControllerFromGameController<P0, P1>(factory: P0, gamecontroller: P1) -> ::windows_core::Result<super::IGameController>
    where
        P0: ::windows_core::TryIntoParam<ICustomGameControllerFactory>,
        P1: ::windows_core::TryIntoParam<super::IGameController>,
    {
        Self::IGameControllerFactoryManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetFactoryControllerFromGameController)(::windows_core::Interface::as_raw(this), factory.try_into_param()?.abi(), gamecontroller.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameControllerFactoryManagerStatics<R, F: FnOnce(&IGameControllerFactoryManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGameControllerFactoryManagerStatics2<R, F: FnOnce(&IGameControllerFactoryManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for GameControllerFactoryManager {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GameControllerFactoryManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GipFirmwareUpdateResult(::windows_core::IUnknown);
impl GipFirmwareUpdateResult {
    pub fn ExtendedErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FinalComponentId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FinalComponentId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<GipFirmwareUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GipFirmwareUpdateResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GipFirmwareUpdateResult {
    const IID: ::windows_core::GUID = <IGipFirmwareUpdateResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
}
::windows_core::imp::interface_hierarchy!(GipFirmwareUpdateResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GipFirmwareUpdateResult {}
unsafe impl ::core::marker::Sync for GipFirmwareUpdateResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GipGameControllerProvider(::windows_core::IUnknown);
impl GipGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SendMessage(&self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendMessage)(::windows_core::Interface::as_raw(this), messageclass, messageid, messagebuffer.len().try_into().unwrap(), messagebuffer.as_ptr()).ok() }
    }
    pub fn SendReceiveMessage(&self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[u8], responsemessagebuffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendReceiveMessage)(::windows_core::Interface::as_raw(this), messageclass, messageid, requestmessagebuffer.len().try_into().unwrap(), requestmessagebuffer.as_ptr(), responsemessagebuffer.len().try_into().unwrap(), responsemessagebuffer.as_mut_ptr()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UpdateFirmwareAsync<P0>(&self, firmwareimage: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateFirmwareAsync)(::windows_core::Interface::as_raw(this), firmwareimage.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GipGameControllerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GipGameControllerProvider {
    const IID: ::windows_core::GUID = <IGipGameControllerProvider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipGameControllerProvider";
}
::windows_core::imp::interface_hierarchy!(GipGameControllerProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGameControllerProvider> for GipGameControllerProvider {}
unsafe impl ::core::marker::Send for GipGameControllerProvider {}
unsafe impl ::core::marker::Sync for GipGameControllerProvider {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HidGameControllerProvider(::windows_core::IUnknown);
impl HidGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFeatureReport(&self, reportid: u8, reportbuffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetFeatureReport)(::windows_core::Interface::as_raw(this), reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_mut_ptr()).ok() }
    }
    pub fn SendFeatureReport(&self, reportid: u8, reportbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendFeatureReport)(::windows_core::Interface::as_raw(this), reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_ptr()).ok() }
    }
    pub fn SendOutputReport(&self, reportid: u8, reportbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendOutputReport)(::windows_core::Interface::as_raw(this), reportid, reportbuffer.len().try_into().unwrap(), reportbuffer.as_ptr()).ok() }
    }
}
impl ::windows_core::RuntimeType for HidGameControllerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HidGameControllerProvider {
    const IID: ::windows_core::GUID = <IHidGameControllerProvider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.HidGameControllerProvider";
}
::windows_core::imp::interface_hierarchy!(HidGameControllerProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGameControllerProvider> for HidGameControllerProvider {}
unsafe impl ::core::marker::Send for HidGameControllerProvider {}
unsafe impl ::core::marker::Sync for HidGameControllerProvider {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XusbGameControllerProvider(::windows_core::IUnknown);
impl XusbGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVibration)(::windows_core::Interface::as_raw(this), lowfrequencymotorspeed, highfrequencymotorspeed).ok() }
    }
}
impl ::windows_core::RuntimeType for XusbGameControllerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XusbGameControllerProvider {
    const IID: ::windows_core::GUID = <IXusbGameControllerProvider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.XusbGameControllerProvider";
}
::windows_core::imp::interface_hierarchy!(XusbGameControllerProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGameControllerProvider> for XusbGameControllerProvider {}
unsafe impl ::core::marker::Send for XusbGameControllerProvider {}
unsafe impl ::core::marker::Sync for XusbGameControllerProvider {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: Self = Self(0i32);
    pub const UpToDate: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for GipFirmwareUpdateStatus {}
impl ::core::clone::Clone for GipFirmwareUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GipFirmwareUpdateStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GipFirmwareUpdateStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GipFirmwareUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GipFirmwareUpdateStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
    pub const StandardLatency: Self = Self(2i32);
}
impl ::core::marker::Copy for GipMessageClass {}
impl ::core::clone::Clone for GipMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GipMessageClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GipMessageClass {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GipMessageClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipMessageClass").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GipMessageClass {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipMessageClass;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
    pub const ArcadePad: Self = Self(2i32);
    pub const ArcadeStick: Self = Self(3i32);
    pub const FlightStick: Self = Self(4i32);
    pub const Wheel: Self = Self(5i32);
    pub const Guitar: Self = Self(6i32);
    pub const GuitarAlternate: Self = Self(7i32);
    pub const GuitarBass: Self = Self(8i32);
    pub const DrumKit: Self = Self(9i32);
    pub const DancePad: Self = Self(10i32);
}
impl ::core::marker::Copy for XusbDeviceSubtype {}
impl ::core::clone::Clone for XusbDeviceSubtype {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XusbDeviceSubtype {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XusbDeviceSubtype {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XusbDeviceSubtype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceSubtype").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XusbDeviceSubtype {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceSubtype;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
}
impl ::core::marker::Copy for XusbDeviceType {}
impl ::core::clone::Clone for XusbDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XusbDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XusbDeviceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XusbDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XusbDeviceType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceType;i4)");
}
#[repr(C)]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for GameControllerVersionInfo {}
impl ::core::clone::Clone for GameControllerVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GameControllerVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GameControllerVersionInfo").field("Major", &self.Major).field("Minor", &self.Minor).field("Build", &self.Build).field("Revision", &self.Revision).finish()
    }
}
impl ::windows_core::TypeKind for GameControllerVersionInfo {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for GameControllerVersionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GameControllerVersionInfo;u2;u2;u2;u2)");
}
impl ::core::cmp::PartialEq for GameControllerVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major && self.Minor == other.Minor && self.Build == other.Build && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for GameControllerVersionInfo {}
impl ::core::default::Default for GameControllerVersionInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl ::core::marker::Copy for GipFirmwareUpdateProgress {}
impl ::core::clone::Clone for GipFirmwareUpdateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GipFirmwareUpdateProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GipFirmwareUpdateProgress").field("PercentCompleted", &self.PercentCompleted).field("CurrentComponentId", &self.CurrentComponentId).finish()
    }
}
impl ::windows_core::TypeKind for GipFirmwareUpdateProgress {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for GipFirmwareUpdateProgress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress;f8;u4)");
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        self.PercentCompleted == other.PercentCompleted && self.CurrentComponentId == other.CurrentComponentId
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateProgress {}
impl ::core::default::Default for GipFirmwareUpdateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
