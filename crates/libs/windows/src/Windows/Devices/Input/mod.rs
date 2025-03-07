#[cfg(feature = "Devices_Input_Preview")]
#[doc = "Required features: `\"Devices_Input_Preview\"`"]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKeyboardCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardCapabilities {
    type Vtable = IKeyboardCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKeyboardCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a3f9b56_6798_4bbc_833e_0f34b17c65ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyboardPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMouseCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseCapabilities {
    type Vtable = IMouseCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMouseCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbca5e023_7dd9_4b6b_9a92_55d43cb38f73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MousePresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub VerticalWheelPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub HorizontalWheelPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SwapButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NumberOfButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMouseDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseDevice {
    type Vtable = IMouseDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMouseDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88edf458_f2c8_49f4_be1f_c256b388bc11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MouseMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MouseMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMouseMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMouseMoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMouseDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseDeviceStatics {
    type Vtable = IMouseDeviceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMouseDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x484a9045_6d70_49db_8e68_46ffbd17d38d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMouseEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseEventArgs {
    type Vtable = IMouseEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMouseEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf625aa5d_2354_4cc7_9230_96941c969fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MouseDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MouseDelta) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenButtonListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenButtonListener {
    type Vtable = IPenButtonListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenButtonListener {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8245c376_1ee3_53f7_b1f7_8334a16f2815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenButtonListener_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonDoubleClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonDoubleClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonDoubleClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonDoubleClicked: usize,
    #[cfg(feature = "Foundation")]
    pub TailButtonLongPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TailButtonLongPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTailButtonLongPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTailButtonLongPressed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenButtonListenerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenButtonListenerStatics {
    type Vtable = IPenButtonListenerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenButtonListenerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19a8a584_862f_5f69_bfea_05f6584f133f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenButtonListenerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDevice {
    type Vtable = IPenDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31856eba_a738_5a8c_b8f6_f97ef68d18ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PenId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDevice2 {
    type Vtable = IPenDevice2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0207d327_7fb8_5566_8c34_f8342037b7f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDeviceStatics {
    type Vtable = IPenDeviceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9dfbbe01_0966_5180_bcb4_b85060e39479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetFromPointerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenDockListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDockListener {
    type Vtable = IPenDockListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenDockListener {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x759f4d90_1dc0_55cb_ad18_b9101456f592);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockListener_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Docked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Docked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDocked: usize,
    #[cfg(feature = "Foundation")]
    pub Undocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Undocked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUndocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUndocked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenDockListenerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDockListenerStatics {
    type Vtable = IPenDockListenerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenDockListenerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcab75e9a_0016_5c72_969e_a97e11992a93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockListenerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenDockedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDockedEventArgs {
    type Vtable = IPenDockedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenDockedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd4277c6_ca63_5d4e_9ed3_a28a54521c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDockedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenTailButtonClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenTailButtonClickedEventArgs {
    type Vtable = IPenTailButtonClickedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenTailButtonClickedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d2fb7b6_6ad3_5d3e_ab29_05ea2410e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenTailButtonDoubleClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenTailButtonDoubleClickedEventArgs {
    type Vtable = IPenTailButtonDoubleClickedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenTailButtonDoubleClickedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x846321a2_618a_5478_b04c_b358231da4a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonDoubleClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenTailButtonLongPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenTailButtonLongPressedEventArgs {
    type Vtable = IPenTailButtonLongPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenTailButtonLongPressedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf37c606e_c60a_5f42_b818_a53112406c13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenTailButtonLongPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenUndockedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenUndockedEventArgs {
    type Vtable = IPenUndockedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenUndockedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccd09150_261b_59e6_a5d4_c1964cd03feb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenUndockedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDevice {
    type Vtable = IPointerDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93c9bafc_ebcb_467e_82c6_276feae36b5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PointerDeviceType) -> ::windows_core::HRESULT,
    pub IsIntegrated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaxContacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PhysicalDeviceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalDeviceRect: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedUsages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedUsages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDevice2 {
    type Vtable = IPointerDevice2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8a6d2a0_c484_489f_ae3e_30d2ee1ffd3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDevice2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxPointersWithZDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDeviceStatics {
    type Vtable = IPointerDeviceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8b89aa1_d1c6_416e_bd8d_5790914dc563);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetPointerDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPointerDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPointerDevices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITouchCapabilities(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITouchCapabilities {
    type Vtable = ITouchCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITouchCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20dd55f9_13f1_46c8_9285_2c05fa3eda6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITouchCapabilities_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TouchPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Contacts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct KeyboardCapabilities(::windows_core::IUnknown);
impl KeyboardCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KeyboardCapabilities, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn KeyboardPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardPresent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for KeyboardCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for KeyboardCapabilities {
    type Vtable = IKeyboardCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyboardCapabilities {
    const IID: ::windows_core::GUID = <IKeyboardCapabilities as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyboardCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.KeyboardCapabilities";
}
::windows_core::imp::interface_hierarchy!(KeyboardCapabilities, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for KeyboardCapabilities {}
unsafe impl ::core::marker::Sync for KeyboardCapabilities {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MouseCapabilities(::windows_core::IUnknown);
impl MouseCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MouseCapabilities, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MousePresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MousePresent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VerticalWheelPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticalWheelPresent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HorizontalWheelPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalWheelPresent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SwapButtons(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SwapButtons)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NumberOfButtons(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumberOfButtons)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MouseCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MouseCapabilities {
    type Vtable = IMouseCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MouseCapabilities {
    const IID: ::windows_core::GUID = <IMouseCapabilities as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MouseCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.MouseCapabilities";
}
::windows_core::imp::interface_hierarchy!(MouseCapabilities, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MouseCapabilities {}
unsafe impl ::core::marker::Sync for MouseCapabilities {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MouseDevice(::windows_core::IUnknown);
impl MouseDevice {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MouseMoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MouseMoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMouseMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMouseMoved)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<MouseDevice> {
        Self::IMouseDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMouseDeviceStatics<R, F: FnOnce(&IMouseDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MouseDevice, IMouseDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MouseDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MouseDevice {
    type Vtable = IMouseDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MouseDevice {
    const IID: ::windows_core::GUID = <IMouseDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MouseDevice {
    const NAME: &'static str = "Windows.Devices.Input.MouseDevice";
}
::windows_core::imp::interface_hierarchy!(MouseDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MouseEventArgs(::windows_core::IUnknown);
impl MouseEventArgs {
    pub fn MouseDelta(&self) -> ::windows_core::Result<MouseDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MouseDelta)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MouseEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MouseEventArgs {
    type Vtable = IMouseEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MouseEventArgs {
    const IID: ::windows_core::GUID = <IMouseEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MouseEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.MouseEventArgs";
}
::windows_core::imp::interface_hierarchy!(MouseEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenButtonListener(::windows_core::IUnknown);
impl PenButtonListener {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PenButtonListener, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TailButtonClicked<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TailButtonClicked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTailButtonClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTailButtonClicked)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TailButtonDoubleClicked<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TailButtonDoubleClicked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTailButtonDoubleClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTailButtonDoubleClicked)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TailButtonLongPressed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TailButtonLongPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTailButtonLongPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTailButtonLongPressed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<PenButtonListener> {
        Self::IPenButtonListenerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenButtonListenerStatics<R, F: FnOnce(&IPenButtonListenerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PenButtonListener, IPenButtonListenerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PenButtonListener {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenButtonListener {
    type Vtable = IPenButtonListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenButtonListener {
    const IID: ::windows_core::GUID = <IPenButtonListener as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenButtonListener {
    const NAME: &'static str = "Windows.Devices.Input.PenButtonListener";
}
::windows_core::imp::interface_hierarchy!(PenButtonListener, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenButtonListener {}
unsafe impl ::core::marker::Sync for PenButtonListener {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenDevice(::windows_core::IUnknown);
impl PenDevice {
    pub fn PenId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PenId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Haptics\"`"]
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<super::Haptics::SimpleHapticsController> {
        let this = &::windows_core::ComInterface::cast::<IPenDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFromPointerId(pointerid: u32) -> ::windows_core::Result<PenDevice> {
        Self::IPenDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromPointerId)(::windows_core::Interface::as_raw(this), pointerid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenDeviceStatics<R, F: FnOnce(&IPenDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PenDevice, IPenDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PenDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenDevice {
    type Vtable = IPenDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenDevice {
    const IID: ::windows_core::GUID = <IPenDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenDevice {
    const NAME: &'static str = "Windows.Devices.Input.PenDevice";
}
::windows_core::imp::interface_hierarchy!(PenDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenDevice {}
unsafe impl ::core::marker::Sync for PenDevice {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenDockListener(::windows_core::IUnknown);
impl PenDockListener {
    pub fn IsSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PenDockListener, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Docked<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Docked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDocked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDocked)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Undocked<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Undocked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUndocked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUndocked)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> ::windows_core::Result<PenDockListener> {
        Self::IPenDockListenerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenDockListenerStatics<R, F: FnOnce(&IPenDockListenerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PenDockListener, IPenDockListenerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PenDockListener {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenDockListener {
    type Vtable = IPenDockListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenDockListener {
    const IID: ::windows_core::GUID = <IPenDockListener as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenDockListener {
    const NAME: &'static str = "Windows.Devices.Input.PenDockListener";
}
::windows_core::imp::interface_hierarchy!(PenDockListener, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenDockListener {}
unsafe impl ::core::marker::Sync for PenDockListener {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenDockedEventArgs(::windows_core::IUnknown);
impl PenDockedEventArgs {}
impl ::windows_core::RuntimeType for PenDockedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenDockedEventArgs {
    type Vtable = IPenDockedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenDockedEventArgs {
    const IID: ::windows_core::GUID = <IPenDockedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenDockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenDockedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PenDockedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenDockedEventArgs {}
unsafe impl ::core::marker::Sync for PenDockedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenTailButtonClickedEventArgs(::windows_core::IUnknown);
impl PenTailButtonClickedEventArgs {}
impl ::windows_core::RuntimeType for PenTailButtonClickedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenTailButtonClickedEventArgs {
    type Vtable = IPenTailButtonClickedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenTailButtonClickedEventArgs {
    const IID: ::windows_core::GUID = <IPenTailButtonClickedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenTailButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonClickedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PenTailButtonClickedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenTailButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonClickedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenTailButtonDoubleClickedEventArgs(::windows_core::IUnknown);
impl PenTailButtonDoubleClickedEventArgs {}
impl ::windows_core::RuntimeType for PenTailButtonDoubleClickedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenTailButtonDoubleClickedEventArgs {
    type Vtable = IPenTailButtonDoubleClickedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenTailButtonDoubleClickedEventArgs {
    const IID: ::windows_core::GUID = <IPenTailButtonDoubleClickedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenTailButtonDoubleClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PenTailButtonDoubleClickedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenTailButtonDoubleClickedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonDoubleClickedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenTailButtonLongPressedEventArgs(::windows_core::IUnknown);
impl PenTailButtonLongPressedEventArgs {}
impl ::windows_core::RuntimeType for PenTailButtonLongPressedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenTailButtonLongPressedEventArgs {
    type Vtable = IPenTailButtonLongPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenTailButtonLongPressedEventArgs {
    const IID: ::windows_core::GUID = <IPenTailButtonLongPressedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenTailButtonLongPressedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonLongPressedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PenTailButtonLongPressedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenTailButtonLongPressedEventArgs {}
unsafe impl ::core::marker::Sync for PenTailButtonLongPressedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PenUndockedEventArgs(::windows_core::IUnknown);
impl PenUndockedEventArgs {}
impl ::windows_core::RuntimeType for PenUndockedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PenUndockedEventArgs {
    type Vtable = IPenUndockedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PenUndockedEventArgs {
    const IID: ::windows_core::GUID = <IPenUndockedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PenUndockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenUndockedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PenUndockedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PenUndockedEventArgs {}
unsafe impl ::core::marker::Sync for PenUndockedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointerDevice(::windows_core::IUnknown);
impl PointerDevice {
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsIntegrated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsIntegrated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxContacts(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxContacts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PhysicalDeviceRect(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalDeviceRect)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenRect(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScreenRect)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedUsages(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PointerDeviceUsage>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedUsages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxPointersWithZDistance(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IPointerDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxPointersWithZDistance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPointerDevice(pointerid: u32) -> ::windows_core::Result<PointerDevice> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPointerDevice)(::windows_core::Interface::as_raw(this), pointerid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPointerDevices() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PointerDevice>> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPointerDevices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerDeviceStatics<R, F: FnOnce(&IPointerDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PointerDevice, IPointerDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PointerDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PointerDevice {
    type Vtable = IPointerDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointerDevice {
    const IID: ::windows_core::GUID = <IPointerDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointerDevice {
    const NAME: &'static str = "Windows.Devices.Input.PointerDevice";
}
::windows_core::imp::interface_hierarchy!(PointerDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TouchCapabilities(::windows_core::IUnknown);
impl TouchCapabilities {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TouchCapabilities, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TouchPresent(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchPresent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Contacts(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contacts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for TouchCapabilities {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TouchCapabilities {
    type Vtable = ITouchCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TouchCapabilities {
    const IID: ::windows_core::GUID = <ITouchCapabilities as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TouchCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.TouchCapabilities";
}
::windows_core::imp::interface_hierarchy!(TouchCapabilities, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TouchCapabilities {}
unsafe impl ::core::marker::Sync for TouchCapabilities {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0i32);
    pub const Pen: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for PointerDeviceType {}
impl ::core::clone::Clone for PointerDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PointerDeviceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PointerDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDeviceType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PointerDeviceType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.PointerDeviceType;i4)");
}
#[repr(C)]
pub struct MouseDelta {
    pub X: i32,
    pub Y: i32,
}
impl ::core::marker::Copy for MouseDelta {}
impl ::core::clone::Clone for MouseDelta {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MouseDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MouseDelta").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::windows_core::TypeKind for MouseDelta {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for MouseDelta {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.MouseDelta;i4;i4)");
}
impl ::core::cmp::PartialEq for MouseDelta {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for MouseDelta {}
impl ::core::default::Default for MouseDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PointerDeviceUsage {
    pub UsagePage: u32,
    pub Usage: u32,
    pub MinLogical: i32,
    pub MaxLogical: i32,
    pub MinPhysical: i32,
    pub MaxPhysical: i32,
    pub Unit: u32,
    pub PhysicalMultiplier: f32,
}
impl ::core::marker::Copy for PointerDeviceUsage {}
impl ::core::clone::Clone for PointerDeviceUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PointerDeviceUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PointerDeviceUsage").field("UsagePage", &self.UsagePage).field("Usage", &self.Usage).field("MinLogical", &self.MinLogical).field("MaxLogical", &self.MaxLogical).field("MinPhysical", &self.MinPhysical).field("MaxPhysical", &self.MaxPhysical).field("Unit", &self.Unit).field("PhysicalMultiplier", &self.PhysicalMultiplier).finish()
    }
}
impl ::windows_core::TypeKind for PointerDeviceUsage {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for PointerDeviceUsage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.PointerDeviceUsage;u4;u4;i4;i4;i4;i4;u4;f4)");
}
impl ::core::cmp::PartialEq for PointerDeviceUsage {
    fn eq(&self, other: &Self) -> bool {
        self.UsagePage == other.UsagePage && self.Usage == other.Usage && self.MinLogical == other.MinLogical && self.MaxLogical == other.MaxLogical && self.MinPhysical == other.MinPhysical && self.MaxPhysical == other.MaxPhysical && self.Unit == other.Unit && self.PhysicalMultiplier == other.PhysicalMultiplier
    }
}
impl ::core::cmp::Eq for PointerDeviceUsage {}
impl ::core::default::Default for PointerDeviceUsage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
