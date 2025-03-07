#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeDevicePreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDevicePreview {
    type Vtable = IGazeDevicePreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeDevicePreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee9_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDevicePreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CanTrackEyes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanTrackHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ConfigurationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GazeDeviceConfigurationStatePreview) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestCalibrationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCalibrationAsync: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetNumericControlDescriptions: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetBooleanControlDescriptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeDeviceWatcherAddedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7eed_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeDeviceWatcherPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeDeviceWatcherPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee7_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2631f08_0e3f_431f_a606_50b35af94a1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fe830ef_7f08_4737_88e1_4a83ae4e4885);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeEnteredPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeEnteredPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2567bf43_1225_489f_9dd1_daa7c50fbf4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeEnteredPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeExitedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeExitedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d0af07e_7d83_40ef_9f0a_fbc1bbdcc5ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeExitedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeInputSourcePreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeInputSourcePreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee8_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GazeMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeMoved: usize,
    #[cfg(feature = "Foundation")]
    pub GazeEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeEntered: usize,
    #[cfg(feature = "Foundation")]
    pub GazeExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeExited: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeInputSourcePreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeInputSourcePreviewStatics {
    type Vtable = IGazeInputSourcePreviewStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeInputSourcePreviewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7ee6_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazeMovedPreviewEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazeMovedPreviewEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7eeb_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeMovedPreviewEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGazePointPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGazePointPreview {
    type Vtable = IGazePointPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGazePointPreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe79e7eea_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazePointPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SourceDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EyeGazePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EyeGazePosition: usize,
    #[cfg(feature = "Foundation")]
    pub HeadGazePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadGazePosition: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub HidInputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_HumanInterfaceDevice"))]
    HidInputReport: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeDevicePreview(::windows_core::IUnknown);
impl GazeDevicePreview {
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanTrackEyes(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanTrackEyes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanTrackHead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanTrackHead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConfigurationState(&self) -> ::windows_core::Result<GazeDeviceConfigurationStatePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConfigurationState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCalibrationAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCalibrationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetNumericControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNumericControlDescriptions)(::windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetBooleanControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanControlDescriptions)(::windows_core::Interface::as_raw(this), usagepage, usageid, &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazeDevicePreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeDevicePreview {
    type Vtable = IGazeDevicePreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeDevicePreview {
    const IID: ::windows_core::GUID = <IGazeDevicePreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeDevicePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDevicePreview";
}
::windows_core::imp::interface_hierarchy!(GazeDevicePreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeDevicePreview {}
unsafe impl ::core::marker::Sync for GazeDevicePreview {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(::windows_core::IUnknown);
impl GazeDeviceWatcherAddedPreviewEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazeDeviceWatcherAddedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeDeviceWatcherAddedPreviewEventArgs {
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherAddedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherAddedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(GazeDeviceWatcherAddedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherAddedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherAddedPreviewEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeDeviceWatcherPreview(::windows_core::IUnknown);
impl GazeDeviceWatcherPreview {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherAddedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherRemovedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherUpdatedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for GazeDeviceWatcherPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeDeviceWatcherPreview {
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
}
::windows_core::imp::interface_hierarchy!(GazeDeviceWatcherPreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherPreview {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherPreview {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(::windows_core::IUnknown);
impl GazeDeviceWatcherRemovedPreviewEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazeDeviceWatcherRemovedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeDeviceWatcherRemovedPreviewEventArgs {
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherRemovedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherRemovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(GazeDeviceWatcherRemovedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherRemovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherRemovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(::windows_core::IUnknown);
impl GazeDeviceWatcherUpdatedPreviewEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const IID: ::windows_core::GUID = <IGazeDeviceWatcherUpdatedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(GazeDeviceWatcherUpdatedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherUpdatedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherUpdatedPreviewEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeEnteredPreviewEventArgs(::windows_core::IUnknown);
impl GazeEnteredPreviewEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazeEnteredPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeEnteredPreviewEventArgs {
    const IID: ::windows_core::GUID = <IGazeEnteredPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeEnteredPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(GazeEnteredPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeEnteredPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeEnteredPreviewEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeExitedPreviewEventArgs(::windows_core::IUnknown);
impl GazeExitedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazeExitedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeExitedPreviewEventArgs {
    const IID: ::windows_core::GUID = <IGazeExitedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeExitedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(GazeExitedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeExitedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeExitedPreviewEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeInputSourcePreview(::windows_core::IUnknown);
impl GazeInputSourcePreview {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GazeMoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeMovedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GazeMoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeMoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGazeMoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GazeEntered<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeEnteredPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GazeEntered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeEntered(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGazeEntered)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GazeExited<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeExitedPreviewEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GazeExited)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeExited(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGazeExited)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<GazeInputSourcePreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<GazeDeviceWatcherPreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGazeInputSourcePreviewStatics<R, F: FnOnce(&IGazeInputSourcePreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GazeInputSourcePreview, IGazeInputSourcePreviewStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GazeInputSourcePreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeInputSourcePreview {
    const IID: ::windows_core::GUID = <IGazeInputSourcePreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeInputSourcePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeInputSourcePreview";
}
::windows_core::imp::interface_hierarchy!(GazeInputSourcePreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeInputSourcePreview {}
unsafe impl ::core::marker::Sync for GazeInputSourcePreview {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazeMovedPreviewEventArgs(::windows_core::IUnknown);
impl GazeMovedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHandled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows_core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<GazePointPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediatePoints)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazeMovedPreviewEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazeMovedPreviewEventArgs {
    const IID: ::windows_core::GUID = <IGazeMovedPreviewEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazeMovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
}
::windows_core::imp::interface_hierarchy!(GazeMovedPreviewEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazeMovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeMovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GazePointPreview(::windows_core::IUnknown);
impl GazePointPreview {
    pub fn SourceDevice(&self) -> ::windows_core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceDevice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EyeGazePosition(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EyeGazePosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn HeadGazePosition(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeadGazePosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_HumanInterfaceDevice\"`"]
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub fn HidInputReport(&self) -> ::windows_core::Result<super::super::HumanInterfaceDevice::HidInputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HidInputReport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GazePointPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GazePointPreview {
    type Vtable = IGazePointPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GazePointPreview {
    const IID: ::windows_core::GUID = <IGazePointPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GazePointPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazePointPreview";
}
::windows_core::imp::interface_hierarchy!(GazePointPreview, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GazePointPreview {}
unsafe impl ::core::marker::Sync for GazePointPreview {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Configuring: Self = Self(2i32);
    pub const ScreenSetupNeeded: Self = Self(3i32);
    pub const UserCalibrationNeeded: Self = Self(4i32);
}
impl ::core::marker::Copy for GazeDeviceConfigurationStatePreview {}
impl ::core::clone::Clone for GazeDeviceConfigurationStatePreview {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GazeDeviceConfigurationStatePreview {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GazeDeviceConfigurationStatePreview {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GazeDeviceConfigurationStatePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceConfigurationStatePreview").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GazeDeviceConfigurationStatePreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview;i4)");
}
