#[cfg(feature = "System_Diagnostics")]
#[doc = "Required features: `\"System_Diagnostics\"`"]
pub mod Diagnostics;
#[cfg(feature = "System_Display")]
#[doc = "Required features: `\"System_Display\"`"]
pub mod Display;
#[cfg(feature = "System_Implementation")]
#[doc = "Required features: `\"System_Implementation\"`"]
pub mod Implementation;
#[cfg(feature = "System_Inventory")]
#[doc = "Required features: `\"System_Inventory\"`"]
pub mod Inventory;
#[cfg(feature = "System_Power")]
#[doc = "Required features: `\"System_Power\"`"]
pub mod Power;
#[cfg(feature = "System_Profile")]
#[doc = "Required features: `\"System_Profile\"`"]
pub mod Profile;
#[cfg(feature = "System_RemoteDesktop")]
#[doc = "Required features: `\"System_RemoteDesktop\"`"]
pub mod RemoteDesktop;
#[cfg(feature = "System_RemoteSystems")]
#[doc = "Required features: `\"System_RemoteSystems\"`"]
pub mod RemoteSystems;
#[cfg(feature = "System_Threading")]
#[doc = "Required features: `\"System_Threading\"`"]
pub mod Threading;
#[cfg(feature = "System_Update")]
#[doc = "Required features: `\"System_Update\"`"]
pub mod Update;
#[cfg(feature = "System_UserProfile")]
#[doc = "Required features: `\"System_UserProfile\"`"]
pub mod UserProfile;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppActivationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppActivationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b528900_f46e_4eb0_aa6c_38af557cf9ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppDiagnosticInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppDiagnosticInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe348a69a_8889_4ca3_be07_d5ffff5f0804);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppDiagnosticInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfo2 {
    type Vtable = IAppDiagnosticInfo2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppDiagnosticInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf46fbd7_191a_446c_9473_8fbc2374a354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResourceGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResourceGroups: usize,
    pub CreateResourceGroupWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppDiagnosticInfo3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfo3 {
    type Vtable = IAppDiagnosticInfo3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppDiagnosticInfo3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc895c63d_dd61_4c65_babd_81a10b4f9815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfo3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppDiagnosticInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoStatics {
    type Vtable = IAppDiagnosticInfoStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppDiagnosticInfoStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce6925bf_10ca_40c8_a9ca_c5c96501866e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppDiagnosticInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoStatics2 {
    type Vtable = IAppDiagnosticInfoStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppDiagnosticInfoStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05b24b86_1000_4c90_bb9f_7235071c50fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForPackageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestInfoForAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestInfoForAppUserModelId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppDiagnosticInfoWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppDiagnosticInfoWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75575070_01d3_489a_9325_52f9cc6ede0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppDiagnosticInfoWatcherEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppDiagnosticInfoWatcherEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7017c716_e1da_4c65_99df_046dff5be71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppDiagnosticInfoWatcherEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppDiagnosticInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppExecutionStateChangeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppExecutionStateChangeResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f039bf0_f91b_4df8_ae77_3033ccb69114);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppExecutionStateChangeResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppMemoryReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppMemoryReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d65339b_4d6f_45bc_9c5e_e49b3ff2758d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PeakPrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppMemoryReport2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppMemoryReport2 {
    type Vtable = IAppMemoryReport2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppMemoryReport2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f7f3738_51b7_42dc_b7ed_79ba46d28857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryReport2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExpectedTotalCommitLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppMemoryUsageLimitChangingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppMemoryUsageLimitChangingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79f86664_feca_4da5_9e40_2bc63efdc979);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppMemoryUsageLimitChangingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub NewLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupBackgroundTaskReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupBackgroundTaskReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2566e74e_b05d_40c2_9dc1_1a4f039ea120);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupBackgroundTaskReport_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb913f77a_e807_49f4_845e_7b8bdcfe8ee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IsShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBackgroundTaskReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBackgroundTaskReports: usize,
    pub GetMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub GetProcessDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_Diagnostics")))]
    GetProcessDiagnosticInfos: usize,
    pub GetStateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfo2 {
    type Vtable = IAppResourceGroupInfo2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee9b236d_d305_4d6b_92f7_6afdad72dedc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartSuspendAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSuspendAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartResumeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTerminateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTerminateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupInfoWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupInfoWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9b0a0fd_6e5a_4c72_8b17_09fec4a212bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub ExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveExecutionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveExecutionStateChanged: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupInfoWatcherEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupInfoWatcherEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a787637_6302_4d2f_bf89_1c12d0b2a6b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bdbedd7_fee6_4fd4_98dd_e92a2cc299f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppDiagnosticInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppDiagnosticInfos: usize,
    pub AppResourceGroupInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupMemoryReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupMemoryReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c8c06b1_7db1_4c51_a225_7fae2d49e431);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupMemoryReport_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CommitUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub CommitUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows_core::HRESULT,
    pub PrivateCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalCommitUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppResourceGroupStateReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppResourceGroupStateReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52849f18_2f70_4236_ab40_d04db0c7b931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppResourceGroupStateReport_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupExecutionState) -> ::windows_core::HRESULT,
    pub EnergyQuotaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerHost(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d50cac5_92d2_5409_b56f_7f73e10ea4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerHost2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerHost2 {
    type Vtable = IAppUriHandlerHost2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerHost2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a0bee95_29e4_51bf_8095_a3c068e3c72a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHost2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerHostFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerHostFactory {
    type Vtable = IAppUriHandlerHostFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerHostFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x257c3c96_ce04_5f98_96bb_3ebd3e9275bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerHostFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerRegistration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerRegistration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f73aeb1_4569_5c3f_9ba0_99123eea32c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAppAddedHostsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAppAddedHostsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAppAddedHostsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerRegistration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistration2 {
    type Vtable = IAppUriHandlerRegistration2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerRegistration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd54dac97_cb39_5f1f_883e_01853730bd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistration2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllHosts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateHosts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hosts: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateHosts: usize,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerRegistrationManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerRegistrationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe62c9a52_ac94_5750_ac1b_6cfb6f250263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryGetRegistration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerRegistrationManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManager2 {
    type Vtable = IAppUriHandlerRegistrationManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerRegistrationManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbddfcaf1_b51a_5e69_aefd_7088d9f2b123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerRegistrationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManagerStatics {
    type Vtable = IAppUriHandlerRegistrationManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerRegistrationManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5cedd9f_5729_5b76_a1d4_0285f295c124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppUriHandlerRegistrationManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUriHandlerRegistrationManagerStatics2 {
    type Vtable = IAppUriHandlerRegistrationManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppUriHandlerRegistrationManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14f78379_6890_5080_90a7_98824a7f079e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUriHandlerRegistrationManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetForPackageForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDateTimeSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDateTimeSettingsStatics {
    type Vtable = IDateTimeSettingsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDateTimeSettingsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d2150d1_47ee_48ab_a52b_9f1954278d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSystemDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utcdatetime: super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemDateTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x603e88e4_a338_4ffe_a457_a5cfb9ceb899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: DispatcherQueuePriority, callback: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShutdownCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShutdownCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueue2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueue2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc822c647_30ef_506e_bd1e_a647ae6675ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasThreadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueController {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22f34e66_50db_4e36_a98d_61c01b384d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DispatcherQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShutdownQueueAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShutdownQueueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueControllerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a6c98e0_5198_49a2_a313_3f70d1f13c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateOnDedicatedThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueShutdownStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueShutdownStartingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4724c4c_ff97_40c0_a226_cc0aaa545e89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa96d83d7_9371_4517_9245_d0824ac12c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueTimer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueTimer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5feabb1d_a31c_4727_b1ac_37454649d56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Interval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterval: usize,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRepeating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Tick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Tick: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTick: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFolderLauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFolderLauncherOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb91c27d_6b87_432a_bd04_776c6f5fb2ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderLauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub ItemsToSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    ItemsToSelect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKnownUserPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownUserPropertiesStatics {
    type Vtable = IKnownUserPropertiesStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKnownUserPropertiesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7755911a_70c5_48e5_b637_5ba3441e4ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GuestHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PrincipalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SessionInitiationProtocolUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKnownUserPropertiesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownUserPropertiesStatics2 {
    type Vtable = IKnownUserPropertiesStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKnownUserPropertiesStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b450782_f620_577e_b1b3_dd56644d79b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUserPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AgeEnforcementRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILaunchUriResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILaunchUriResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec27a8df_f6d5_45ca_913a_70a40c5c8221);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchUriResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LaunchUriStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Result: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbafa21d8_b071_4cd8_853e_341203e557d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetTreatAsUntrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub DisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDisplayApplicationPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub UI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPreferredApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPreferredApplicationDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions2 {
    type Vtable = ILauncherOptions2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherOptions2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ba08eb4_6e40_4dce_a1a3_2f53950afb49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
    #[cfg(feature = "Storage_Search")]
    pub SetNeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    SetNeighboringFilesQuery: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherOptions3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions3 {
    type Vtable = ILauncherOptions3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherOptions3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0770655_4b63_4e3a_9107_4e687841923a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIgnoreAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherOptions4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherOptions4 {
    type Vtable = ILauncherOptions4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherOptions4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef6fd10e_e6fb_4814_a44e_57e8b9d9a01b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherOptions4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetLimitPickerToCurrentAppAndAppUriHandlers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics {
    type Vtable = ILauncherStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x277151c3_9e3e_42f6_91a4_5dfdeb232451);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFileWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics2 {
    type Vtable = ILauncherStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59ba2fbb_24cb_4c02_a4c4_8294569d54f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, launchquerysupporttype: LaunchQuerySupportType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub QueryFileSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    QueryFileSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindUriSchemeHandlersWithLaunchUriTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheme: ::std::mem::MaybeUninit<::windows_core::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindUriSchemeHandlersWithLaunchUriTypeAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindFileHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extension: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindFileHandlersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics3 {
    type Vtable = ILauncherStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x234261a8_9db3_4683_aa42_dc6f51d33847);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LaunchFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LaunchFolderWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics4 {
    type Vtable = ILauncherStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9ec819f_b5a5_41c6_b3b3_dd1b3178bcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub QueryAppUriSupportWithPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QueryAppUriSupportWithPackageFamilyNameAsync: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub FindAppUriHandlersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation_Collections")))]
    FindAppUriHandlersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriWithDataForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchUriForResultsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchUriForResultsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LaunchUriForResultsWithDataForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaunchUriForResultsWithDataForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherStatics5 {
    type Vtable = ILauncherStatics5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherStatics5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b24ef84_d895_5fea_9153_1ac49aed9ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LaunchFolderPathWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchFolderPathWithOptionsForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherUIOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherUIOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b25da6e_8aa6_41e9_8251_4165f5985f49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherUIOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SetInvocationPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInvocationPoint: usize,
    #[cfg(feature = "Foundation")]
    pub SelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetSelectionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSelectionRect: usize,
    #[cfg(feature = "UI_Popups")]
    pub PreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    PreferredPlacement: usize,
    #[cfg(feature = "UI_Popups")]
    pub SetPreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Popups"))]
    SetPreferredPlacement: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILauncherViewOptions(::windows_core::IUnknown);
impl ILauncherViewOptions {
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows_core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRemainingView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRemainingView)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ILauncherViewOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ILauncherViewOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ILauncherViewOptions {
    type Vtable = ILauncherViewOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILauncherViewOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a9b29f1_7ca7_49de_9bd3_3c5b7184f616);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILauncherViewOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub DesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    DesiredRemainingView: usize,
    #[cfg(feature = "UI_ViewManagement")]
    pub SetDesiredRemainingView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    SetDesiredRemainingView: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMemoryManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics {
    type Vtable = IMemoryManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMemoryManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c6c279c_d7ca_4779_9188_4057219ce64c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppMemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AppMemoryUsageLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageIncreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageIncreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageDecreased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageDecreased: usize,
    #[cfg(feature = "Foundation")]
    pub AppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppMemoryUsageLimitChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAppMemoryUsageLimitChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAppMemoryUsageLimitChanging: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMemoryManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics2 {
    type Vtable = IMemoryManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMemoryManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eee351f_6d62_423f_9479_b01f9c9f7669);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAppMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProcessMemoryReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMemoryManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics3 {
    type Vtable = IMemoryManagerStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMemoryManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x149b59ce_92ad_4e35_89eb_50dfb4c0d91c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TrySetAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMemoryManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMemoryManagerStatics4 {
    type Vtable = IMemoryManagerStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMemoryManagerStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5a94828_e84e_4886_8a0d_44b3190e3b72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExpectedAppMemoryUsageLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProcessLauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProcessLauncherOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3080b9cf_f444_4a83_beaf_a549a0f3229c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub StandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardInput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardOutput: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StandardError: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStandardError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStandardError: usize,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProcessLauncherResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProcessLauncherResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x544c8934_86d8_4991_8e75_ece8a43b6b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProcessLauncherStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessLauncherStatics {
    type Vtable = IProcessLauncherStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProcessLauncherStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33ab66e7_2d0e_448b_a6a0_c13c3836d09c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLauncherStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, args: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunToCompletionAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, args: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunToCompletionAsyncWithOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProcessMemoryReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProcessMemoryReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x087305a8_9b70_4782_8741_3a982b6ce5e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessMemoryReport_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrivateWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub TotalWorkingSetUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtocolForResultsOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtocolForResultsOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd581293a_6de9_4d28_9378_f86782e182bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteLauncherOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteLauncherOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e3a2788_2891_4cdf_a2d6_9dff7d02e693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetFallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetFallbackUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredAppIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredAppIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoteLauncherStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteLauncherStatics {
    type Vtable = IRemoteLauncherStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoteLauncherStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7db7a93_a30c_48b7_9f21_051026a4e517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteLauncherStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub LaunchUriWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System_RemoteSystems")))]
    LaunchUriWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub LaunchUriWithDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, inputdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System_RemoteSystems")))]
    LaunchUriWithDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IShutdownManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShutdownManagerStatics {
    type Vtable = IShutdownManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IShutdownManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72e247ed_dd5b_4d6c_b1d0_c57a7bbb5f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BeginShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginShutdown: usize,
    pub CancelShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IShutdownManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShutdownManagerStatics2 {
    type Vtable = IShutdownManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IShutdownManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f69a02f_9c34_43c7_a8c3_70b30a7f7504);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShutdownManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPowerStateSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnterPowerState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterPowerStateWithTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterPowerStateWithTimeSpan: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITimeZoneSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeZoneSettingsStatics {
    type Vtable = ITimeZoneSettingsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITimeZoneSettingsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b3b2bea_a101_41ae_9fbd_028728bab73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentTimeZoneDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTimeZoneDisplayNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTimeZoneDisplayNames: usize,
    pub CanChangeTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ChangeTimeZoneByDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timezonedisplayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITimeZoneSettingsStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeZoneSettingsStatics2 {
    type Vtable = ITimeZoneSettingsStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITimeZoneSettingsStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x555c0db8_39a8_49fa_b4f6_a2c7fc2842ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneSettingsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AutoUpdateTimeZoneAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoUpdateTimeZoneAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUser {
    type Vtable = IUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf9a26c6_e746_4bcd_b5d4_120103c4209b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NonRoamableId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AuthenticationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: UserPictureSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUser2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUser2 {
    type Vtable = IUser2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUser2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98ba5628_a6e3_518e_89d9_d3b2b1991a10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUser2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CheckUserAgeConsentGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, consentgroup: UserAgeConsentGroup, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckUserAgeConsentGroupAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserAuthenticationStatusChangeDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserAuthenticationStatusChangeDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88b59568_bb30_42fb_a270_e9902e40efa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangeDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserAuthenticationStatusChangingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserAuthenticationStatusChangingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c030f28_a711_4c1e_ab48_04179c15938f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserAuthenticationStatusChangingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NewStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows_core::HRESULT,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x086459dc_18c6_48db_bc99_724fb9203ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserChangedEventArgs2 {
    type Vtable = IUserChangedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserChangedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b2ccb44_6f01_560c_97ad_fc7f32ec581f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ChangedPropertyKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ChangedPropertyKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserDeviceAssociationChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserDeviceAssociationChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd1f6f6c_bb5d_4d7b_a5f0_c8cd11a38d42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NewUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OldUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserDeviceAssociationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDeviceAssociationStatics {
    type Vtable = IUserDeviceAssociationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserDeviceAssociationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e491e14_f85a_4c07_8da9_7fe3d0542343);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDeviceAssociationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FindUserFromDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserDeviceAssociationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserDeviceAssociationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserDeviceAssociationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserPicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserPicker {
    type Vtable = IUserPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserPicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d548008_f1e3_4a6c_8ddc_a9bb0f488aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowGuestAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSuggestedSelectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSingleUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserPickerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserPickerStatics {
    type Vtable = IUserPickerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserPickerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde3290dc_7e73_4df6_a1ae_4d7eca82b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserPickerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserStatics {
    type Vtable = IUserStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByType: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub FindAllAsyncByTypeAndStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: UserType, status: UserAuthenticationStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    FindAllAsyncByTypeAndStatus: usize,
    pub GetFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonroamableid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserStatics2 {
    type Vtable = IUserStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74a37e11_2eb5_4487_b0d5_2c6790e013e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserWatcher {
    type Vtable = IUserWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x155eb23b_242a_45e0_a2e9_3171fc6a7fbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAuthenticationStatusChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAuthenticationStatusChanging: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppActivationResult(::windows_core::IUnknown);
impl AppActivationResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows_core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppResourceGroupInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppActivationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppActivationResult {
    type Vtable = IAppActivationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppActivationResult {
    const IID: ::windows_core::GUID = <IAppActivationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppActivationResult {
    const NAME: &'static str = "Windows.System.AppActivationResult";
}
::windows_core::imp::interface_hierarchy!(AppActivationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppActivationResult {}
unsafe impl ::core::marker::Sync for AppActivationResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppDiagnosticInfo(::windows_core::IUnknown);
impl AppDiagnosticInfo {
    #[doc = "Required features: `\"ApplicationModel\"`"]
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows_core::Result<super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResourceGroups(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>> {
        let this = &::windows_core::ComInterface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetResourceGroups)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateResourceGroupWatcher(&self) -> ::windows_core::Result<AppResourceGroupInfoWatcher> {
        let this = &::windows_core::ComInterface::cast::<IAppDiagnosticInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceGroupWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppActivationResult>> {
        let this = &::windows_core::ComInterface::cast::<IAppDiagnosticInfo3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<AppDiagnosticInfoWatcher> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForPackageAsync(packagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoForPackageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoForAppAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestInfoForAppUserModelId(appusermodelid: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>> {
        Self::IAppDiagnosticInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestInfoForAppUserModelId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appusermodelid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppDiagnosticInfoStatics<R, F: FnOnce(&IAppDiagnosticInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppDiagnosticInfoStatics2<R, F: FnOnce(&IAppDiagnosticInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppDiagnosticInfo, IAppDiagnosticInfoStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppDiagnosticInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppDiagnosticInfo {
    type Vtable = IAppDiagnosticInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppDiagnosticInfo {
    const IID: ::windows_core::GUID = <IAppDiagnosticInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfo";
}
::windows_core::imp::interface_hierarchy!(AppDiagnosticInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppDiagnosticInfo {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppDiagnosticInfoWatcher(::windows_core::IUnknown);
impl AppDiagnosticInfoWatcher {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<AppDiagnosticInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppDiagnosticInfoWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppDiagnosticInfoWatcher {
    type Vtable = IAppDiagnosticInfoWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppDiagnosticInfoWatcher {
    const IID: ::windows_core::GUID = <IAppDiagnosticInfoWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcher";
}
::windows_core::imp::interface_hierarchy!(AppDiagnosticInfoWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcher {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcher {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppDiagnosticInfoWatcherEventArgs(::windows_core::IUnknown);
impl AppDiagnosticInfoWatcherEventArgs {
    pub fn AppDiagnosticInfo(&self) -> ::windows_core::Result<AppDiagnosticInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDiagnosticInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppDiagnosticInfoWatcherEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppDiagnosticInfoWatcherEventArgs {
    type Vtable = IAppDiagnosticInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppDiagnosticInfoWatcherEventArgs {
    const IID: ::windows_core::GUID = <IAppDiagnosticInfoWatcherEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppDiagnosticInfoWatcherEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppDiagnosticInfoWatcherEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppDiagnosticInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppDiagnosticInfoWatcherEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppExecutionStateChangeResult(::windows_core::IUnknown);
impl AppExecutionStateChangeResult {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppExecutionStateChangeResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppExecutionStateChangeResult {
    type Vtable = IAppExecutionStateChangeResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppExecutionStateChangeResult {
    const IID: ::windows_core::GUID = <IAppExecutionStateChangeResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.AppExecutionStateChangeResult";
}
::windows_core::imp::interface_hierarchy!(AppExecutionStateChangeResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppExecutionStateChangeResult {}
unsafe impl ::core::marker::Sync for AppExecutionStateChangeResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppMemoryReport(::windows_core::IUnknown);
impl AppMemoryReport {
    pub fn PrivateCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrivateCommitUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PeakPrivateCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PeakPrivateCommitUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TotalCommitUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TotalCommitLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TotalCommitLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExpectedTotalCommitLimit(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<IAppMemoryReport2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpectedTotalCommitLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppMemoryReport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppMemoryReport {
    type Vtable = IAppMemoryReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppMemoryReport {
    const IID: ::windows_core::GUID = <IAppMemoryReport as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppMemoryReport {
    const NAME: &'static str = "Windows.System.AppMemoryReport";
}
::windows_core::imp::interface_hierarchy!(AppMemoryReport, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppMemoryReport {}
unsafe impl ::core::marker::Sync for AppMemoryReport {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppMemoryUsageLimitChangingEventArgs(::windows_core::IUnknown);
impl AppMemoryUsageLimitChangingEventArgs {
    pub fn OldLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NewLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppMemoryUsageLimitChangingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppMemoryUsageLimitChangingEventArgs {
    type Vtable = IAppMemoryUsageLimitChangingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppMemoryUsageLimitChangingEventArgs {
    const IID: ::windows_core::GUID = <IAppMemoryUsageLimitChangingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.AppMemoryUsageLimitChangingEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppMemoryUsageLimitChangingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppMemoryUsageLimitChangingEventArgs {}
unsafe impl ::core::marker::Sync for AppMemoryUsageLimitChangingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppResourceGroupBackgroundTaskReport(::windows_core::IUnknown);
impl AppResourceGroupBackgroundTaskReport {
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Trigger(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Trigger)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EntryPoint(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EntryPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupBackgroundTaskReport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppResourceGroupBackgroundTaskReport {
    type Vtable = IAppResourceGroupBackgroundTaskReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppResourceGroupBackgroundTaskReport {
    const IID: ::windows_core::GUID = <IAppResourceGroupBackgroundTaskReport as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupBackgroundTaskReport";
}
::windows_core::imp::interface_hierarchy!(AppResourceGroupBackgroundTaskReport, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupBackgroundTaskReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupBackgroundTaskReport {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppResourceGroupInfo(::windows_core::IUnknown);
impl AppResourceGroupInfo {
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsShared(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShared)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBackgroundTaskReports(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBackgroundTaskReports)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMemoryReport(&self) -> ::windows_core::Result<AppResourceGroupMemoryReport> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMemoryReport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"System_Diagnostics\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System_Diagnostics"))]
    pub fn GetProcessDiagnosticInfos(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProcessDiagnosticInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetStateReport(&self) -> ::windows_core::Result<AppResourceGroupStateReport> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStateReport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartSuspendAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows_core::ComInterface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartSuspendAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartResumeAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows_core::ComInterface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartResumeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartTerminateAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>> {
        let this = &::windows_core::ComInterface::cast::<IAppResourceGroupInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTerminateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfo {
    type Vtable = IAppResourceGroupInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppResourceGroupInfo {
    const IID: ::windows_core::GUID = <IAppResourceGroupInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfo";
}
::windows_core::imp::interface_hierarchy!(AppResourceGroupInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfo {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppResourceGroupInfoWatcher(::windows_core::IUnknown);
impl AppResourceGroupInfoWatcher {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ExecutionStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecutionStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveExecutionStateChanged(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveExecutionStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<AppResourceGroupInfoWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfoWatcher {
    type Vtable = IAppResourceGroupInfoWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppResourceGroupInfoWatcher {
    const IID: ::windows_core::GUID = <IAppResourceGroupInfoWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcher";
}
::windows_core::imp::interface_hierarchy!(AppResourceGroupInfoWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcher {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcher {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppResourceGroupInfoWatcherEventArgs(::windows_core::IUnknown);
impl AppResourceGroupInfoWatcherEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDiagnosticInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows_core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppResourceGroupInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcherEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfoWatcherEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppResourceGroupInfoWatcherEventArgs {
    const IID: ::windows_core::GUID = <IAppResourceGroupInfoWatcherEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppResourceGroupInfoWatcherEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppResourceGroupInfoWatcherExecutionStateChangedEventArgs(::windows_core::IUnknown);
impl AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppDiagnosticInfos(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppDiagnosticInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppResourceGroupInfo(&self) -> ::windows_core::Result<AppResourceGroupInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppResourceGroupInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    type Vtable = IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.AppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppResourceGroupInfoWatcherExecutionStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppResourceGroupInfoWatcherExecutionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppResourceGroupMemoryReport(::windows_core::IUnknown);
impl AppResourceGroupMemoryReport {
    pub fn CommitUsageLimit(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommitUsageLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CommitUsageLevel(&self) -> ::windows_core::Result<AppMemoryUsageLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommitUsageLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrivateCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrivateCommitUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TotalCommitUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TotalCommitUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupMemoryReport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppResourceGroupMemoryReport {
    type Vtable = IAppResourceGroupMemoryReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppResourceGroupMemoryReport {
    const IID: ::windows_core::GUID = <IAppResourceGroupMemoryReport as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupMemoryReport";
}
::windows_core::imp::interface_hierarchy!(AppResourceGroupMemoryReport, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupMemoryReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupMemoryReport {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppResourceGroupStateReport(::windows_core::IUnknown);
impl AppResourceGroupStateReport {
    pub fn ExecutionState(&self) -> ::windows_core::Result<AppResourceGroupExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnergyQuotaState(&self) -> ::windows_core::Result<AppResourceGroupEnergyQuotaState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnergyQuotaState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupStateReport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppResourceGroupStateReport {
    type Vtable = IAppResourceGroupStateReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppResourceGroupStateReport {
    const IID: ::windows_core::GUID = <IAppResourceGroupStateReport as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.AppResourceGroupStateReport";
}
::windows_core::imp::interface_hierarchy!(AppResourceGroupStateReport, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppResourceGroupStateReport {}
unsafe impl ::core::marker::Sync for AppResourceGroupStateReport {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppUriHandlerHost(::windows_core::IUnknown);
impl AppUriHandlerHost {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppUriHandlerHost, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppUriHandlerHost2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance(name: &::windows_core::HSTRING) -> ::windows_core::Result<AppUriHandlerHost> {
        Self::IAppUriHandlerHostFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerHostFactory<R, F: FnOnce(&IAppUriHandlerHostFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppUriHandlerHost, IAppUriHandlerHostFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppUriHandlerHost {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppUriHandlerHost {
    type Vtable = IAppUriHandlerHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppUriHandlerHost {
    const IID: ::windows_core::GUID = <IAppUriHandlerHost as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppUriHandlerHost {
    const NAME: &'static str = "Windows.System.AppUriHandlerHost";
}
::windows_core::imp::interface_hierarchy!(AppUriHandlerHost, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppUriHandlerHost {}
unsafe impl ::core::marker::Sync for AppUriHandlerHost {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppUriHandlerRegistration(::windows_core::IUnknown);
impl AppUriHandlerRegistration {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAppAddedHostsAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppAddedHostsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAppAddedHostsAsync<P0>(&self, hosts: P0) -> ::windows_core::Result<super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IIterable<AppUriHandlerHost>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAppAddedHostsAsync)(::windows_core::Interface::as_raw(this), hosts.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllHosts(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>> {
        let this = &::windows_core::ComInterface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllHosts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateHosts<P0>(&self, hosts: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IIterable<AppUriHandlerHost>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).UpdateHosts)(::windows_core::Interface::as_raw(this), hosts.try_into_param()?.abi()).ok() }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppUriHandlerRegistration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppUriHandlerRegistration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppUriHandlerRegistration {
    type Vtable = IAppUriHandlerRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppUriHandlerRegistration {
    const IID: ::windows_core::GUID = <IAppUriHandlerRegistration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistration";
}
::windows_core::imp::interface_hierarchy!(AppUriHandlerRegistration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppUriHandlerRegistration {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistration {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppUriHandlerRegistrationManager(::windows_core::IUnknown);
impl AppUriHandlerRegistrationManager {
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetRegistration(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<AppUriHandlerRegistration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetRegistration)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppUriHandlerRegistrationManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<AppUriHandlerRegistrationManager>
    where
        P0: ::windows_core::IntoParam<User>,
    {
        Self::IAppUriHandlerRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForPackage(packagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<AppUriHandlerRegistrationManager> {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForPackage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForPackageForUser<P0>(packagefamilyname: &::windows_core::HSTRING, user: P0) -> ::windows_core::Result<AppUriHandlerRegistrationManager>
    where
        P0: ::windows_core::IntoParam<User>,
    {
        Self::IAppUriHandlerRegistrationManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForPackageForUser)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerRegistrationManagerStatics<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppUriHandlerRegistrationManagerStatics2<R, F: FnOnce(&IAppUriHandlerRegistrationManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppUriHandlerRegistrationManager, IAppUriHandlerRegistrationManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppUriHandlerRegistrationManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppUriHandlerRegistrationManager {
    type Vtable = IAppUriHandlerRegistrationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppUriHandlerRegistrationManager {
    const IID: ::windows_core::GUID = <IAppUriHandlerRegistrationManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.AppUriHandlerRegistrationManager";
}
::windows_core::imp::interface_hierarchy!(AppUriHandlerRegistrationManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppUriHandlerRegistrationManager {}
unsafe impl ::core::marker::Sync for AppUriHandlerRegistrationManager {}
pub struct DateTimeSettings;
impl DateTimeSettings {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemDateTime(utcdatetime: super::Foundation::DateTime) -> ::windows_core::Result<()> {
        Self::IDateTimeSettingsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetSystemDateTime)(::windows_core::Interface::as_raw(this), utcdatetime).ok() })
    }
    #[doc(hidden)]
    pub fn IDateTimeSettingsStatics<R, F: FnOnce(&IDateTimeSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DateTimeSettings, IDateTimeSettingsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for DateTimeSettings {
    const NAME: &'static str = "Windows.System.DateTimeSettings";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueue(::windows_core::IUnknown);
impl DispatcherQueue {
    pub fn CreateTimer(&self) -> ::windows_core::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTimer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryEnqueue<P0>(&self, callback: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<DispatcherQueueHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryEnqueue)(::windows_core::Interface::as_raw(this), callback.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn TryEnqueueWithPriority<P0>(&self, priority: DispatcherQueuePriority, callback: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<DispatcherQueueHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryEnqueueWithPriority)(::windows_core::Interface::as_raw(this), priority, callback.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownStarting(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShutdownStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<DispatcherQueue, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShutdownCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShutdownCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HasThreadAccess(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasThreadAccess)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetForCurrentThread() -> ::windows_core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentThread)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueStatics<R, F: FnOnce(&IDispatcherQueueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueue {
    const IID: ::windows_core::GUID = <IDispatcherQueue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Windows.System.DispatcherQueue";
}
::windows_core::imp::interface_hierarchy!(DispatcherQueue, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueue {}
unsafe impl ::core::marker::Sync for DispatcherQueue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueController(::windows_core::IUnknown);
impl DispatcherQueueController {
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShutdownQueueAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownQueueAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateOnDedicatedThread() -> ::windows_core::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOnDedicatedThread)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueControllerStatics<R, F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DispatcherQueueController, IDispatcherQueueControllerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueController {
    const IID: ::windows_core::GUID = <IDispatcherQueueController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Windows.System.DispatcherQueueController";
}
::windows_core::imp::interface_hierarchy!(DispatcherQueueController, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueueController {}
unsafe impl ::core::marker::Sync for DispatcherQueueController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueShutdownStartingEventArgs(::windows_core::IUnknown);
impl DispatcherQueueShutdownStartingEventArgs {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueShutdownStartingEventArgs {
    const IID: ::windows_core::GUID = <IDispatcherQueueShutdownStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.DispatcherQueueShutdownStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(DispatcherQueueShutdownStartingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::core::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueTimer(::windows_core::IUnknown);
impl DispatcherQueueTimer {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Interval(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Interval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetInterval(&self, value: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRunning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRunning)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRepeating(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRepeating)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsRepeating(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRepeating)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Tick<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tick)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTick(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTick)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueTimer {
    const IID: ::windows_core::GUID = <IDispatcherQueueTimer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.DispatcherQueueTimer";
}
::windows_core::imp::interface_hierarchy!(DispatcherQueueTimer, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DispatcherQueueTimer {}
unsafe impl ::core::marker::Sync for DispatcherQueueTimer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FolderLauncherOptions(::windows_core::IUnknown);
impl FolderLauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FolderLauncherOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn ItemsToSelect(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemsToSelect)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows_core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows_core::ComInterface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRemainingView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRemainingView)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for FolderLauncherOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FolderLauncherOptions {
    type Vtable = IFolderLauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FolderLauncherOptions {
    const IID: ::windows_core::GUID = <IFolderLauncherOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FolderLauncherOptions {
    const NAME: &'static str = "Windows.System.FolderLauncherOptions";
}
::windows_core::imp::interface_hierarchy!(FolderLauncherOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILauncherViewOptions> for FolderLauncherOptions {}
unsafe impl ::core::marker::Send for FolderLauncherOptions {}
unsafe impl ::core::marker::Sync for FolderLauncherOptions {}
pub struct KnownUserProperties;
impl KnownUserProperties {
    pub fn DisplayName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FirstName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LastName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProviderName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AccountName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GuestHost() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GuestHost)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PrincipalName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrincipalName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DomainName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SessionInitiationProtocolUri() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionInitiationProtocolUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AgeEnforcementRegion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownUserPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AgeEnforcementRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownUserPropertiesStatics<R, F: FnOnce(&IKnownUserPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKnownUserPropertiesStatics2<R, F: FnOnce(&IKnownUserPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownUserProperties, IKnownUserPropertiesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownUserProperties {
    const NAME: &'static str = "Windows.System.KnownUserProperties";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LaunchUriResult(::windows_core::IUnknown);
impl LaunchUriResult {
    pub fn Status(&self) -> ::windows_core::Result<LaunchUriStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Result(&self) -> ::windows_core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LaunchUriResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LaunchUriResult {
    type Vtable = ILaunchUriResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LaunchUriResult {
    const IID: ::windows_core::GUID = <ILaunchUriResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LaunchUriResult {
    const NAME: &'static str = "Windows.System.LaunchUriResult";
}
::windows_core::imp::interface_hierarchy!(LaunchUriResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LaunchUriResult {}
unsafe impl ::core::marker::Sync for LaunchUriResult {}
pub struct Launcher;
impl Launcher {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileAsync<P0>(file: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::Storage::IStorageFile>,
    {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFileAsync)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFileWithOptionsAsync<P0, P1>(file: P0, options: P1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::Storage::IStorageFile>,
        P1: ::windows_core::IntoParam<LauncherOptions>,
    {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriAsync<P0>(uri: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsAsync<P0, P1>(uri: P0, options: P1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<LauncherOptions>,
    {
        Self::ILauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithOptionsAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsAsync<P0, P1>(uri: P0, options: P1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<LauncherOptions>,
    {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataAsync<P0, P1, P2>(uri: P0, options: P1, inputdata: P2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<LauncherOptions>,
        P2: ::windows_core::IntoParam<super::Foundation::Collections::ValueSet>,
    {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsWithDataAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataAsync<P0, P1, P2>(uri: P0, options: P1, inputdata: P2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<LauncherOptions>,
        P2: ::windows_core::IntoParam<super::Foundation::Collections::ValueSet>,
    {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithDataAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportAsync<P0>(uri: P0, launchquerysupporttype: LaunchQuerySupportType) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryUriSupportAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), launchquerysupporttype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn QueryUriSupportWithPackageFamilyNameAsync<P0>(uri: P0, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryUriSupportWithPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), launchquerysupporttype, ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportAsync<P0>(file: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>
    where
        P0: ::windows_core::IntoParam<super::Storage::StorageFile>,
    {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryFileSupportAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn QueryFileSupportWithPackageFamilyNameAsync<P0>(file: P0, packagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>
    where
        P0: ::windows_core::IntoParam<super::Storage::StorageFile>,
    {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryFileSupportWithPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersAsync(scheme: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindUriSchemeHandlersAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(scheme), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindUriSchemeHandlersWithLaunchUriTypeAsync(scheme: &::windows_core::HSTRING, launchquerysupporttype: LaunchQuerySupportType) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindUriSchemeHandlersWithLaunchUriTypeAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(scheme), launchquerysupporttype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindFileHandlersAsync(extension: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>> {
        Self::ILauncherStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindFileHandlersAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(extension), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderAsync<P0>(folder: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::Storage::IStorageFolder>,
    {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderAsync)(::windows_core::Interface::as_raw(this), folder.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LaunchFolderWithOptionsAsync<P0, P1>(folder: P0, options: P1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::Storage::IStorageFolder>,
        P1: ::windows_core::IntoParam<FolderLauncherOptions>,
    {
        Self::ILauncherStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), folder.try_into_param()?.abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportAsync<P0>(uri: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryAppUriSupportAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn QueryAppUriSupportWithPackageFamilyNameAsync<P0>(uri: P0, packagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryAppUriSupportWithPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections"))]
    pub fn FindAppUriHandlersAsync<P0>(uri: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAppUriHandlersAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForUserAsync<P0, P1>(user: P0, uri: P1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>
    where
        P0: ::windows_core::IntoParam<User>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriWithOptionsForUserAsync<P0, P1, P2>(user: P0, uri: P1, options: P2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>
    where
        P0: ::windows_core::IntoParam<User>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<LauncherOptions>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriWithDataForUserAsync<P0, P1, P2, P3>(user: P0, uri: P1, options: P2, inputdata: P3) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>
    where
        P0: ::windows_core::IntoParam<User>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<LauncherOptions>,
        P3: ::windows_core::IntoParam<super::Foundation::Collections::ValueSet>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithDataForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchUriForResultsForUserAsync<P0, P1, P2>(user: P0, uri: P1, options: P2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>
    where
        P0: ::windows_core::IntoParam<User>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<LauncherOptions>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaunchUriForResultsWithDataForUserAsync<P0, P1, P2, P3>(user: P0, uri: P1, options: P2, inputdata: P3) -> ::windows_core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>
    where
        P0: ::windows_core::IntoParam<User>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<LauncherOptions>,
        P3: ::windows_core::IntoParam<super::Foundation::Collections::ValueSet>,
    {
        Self::ILauncherStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriForResultsWithDataForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathAsync(path: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(path), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsAsync<P0>(path: &::windows_core::HSTRING, options: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<FolderLauncherOptions>,
    {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(path), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathForUserAsync<P0>(user: P0, path: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<User>,
    {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(path), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchFolderPathWithOptionsForUserAsync<P0, P1>(user: P0, path: &::windows_core::HSTRING, options: P1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<User>,
        P1: ::windows_core::IntoParam<FolderLauncherOptions>,
    {
        Self::ILauncherStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFolderPathWithOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(path), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILauncherStatics<R, F: FnOnce(&ILauncherStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Launcher, ILauncherStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics2<R, F: FnOnce(&ILauncherStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Launcher, ILauncherStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics3<R, F: FnOnce(&ILauncherStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Launcher, ILauncherStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics4<R, F: FnOnce(&ILauncherStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Launcher, ILauncherStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILauncherStatics5<R, F: FnOnce(&ILauncherStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Launcher, ILauncherStatics5> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for Launcher {
    const NAME: &'static str = "Windows.System.Launcher";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LauncherOptions(::windows_core::IUnknown);
impl LauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LauncherOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TreatAsUntrusted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TreatAsUntrusted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTreatAsUntrusted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTreatAsUntrusted)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayApplicationPicker(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayApplicationPicker)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayApplicationPicker)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UI(&self) -> ::windows_core::Result<LauncherUIOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreferredApplicationPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreferredApplicationPackageFamilyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PreferredApplicationDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredApplicationDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreferredApplicationDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredApplicationDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows_core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFallbackUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentType(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentType)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TargetApplicationPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTargetApplicationPackageFamilyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn SetNeighboringFilesQuery<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Storage::Search::StorageFileQueryResult>,
    {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNeighboringFilesQuery)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IgnoreAppUriHandlers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IgnoreAppUriHandlers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIgnoreAppUriHandlers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LimitPickerToCurrentAppAndAppUriHandlers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILauncherOptions4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLimitPickerToCurrentAppAndAppUriHandlers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn DesiredRemainingView(&self) -> ::windows_core::Result<super::UI::ViewManagement::ViewSizePreference> {
        let this = &::windows_core::ComInterface::cast::<ILauncherViewOptions>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRemainingView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILauncherViewOptions>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredRemainingView)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for LauncherOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LauncherOptions {
    type Vtable = ILauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LauncherOptions {
    const IID: ::windows_core::GUID = <ILauncherOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LauncherOptions {
    const NAME: &'static str = "Windows.System.LauncherOptions";
}
::windows_core::imp::interface_hierarchy!(LauncherOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILauncherViewOptions> for LauncherOptions {}
unsafe impl ::core::marker::Send for LauncherOptions {}
unsafe impl ::core::marker::Sync for LauncherOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LauncherUIOptions(::windows_core::IUnknown);
impl LauncherUIOptions {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InvocationPoint(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InvocationPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetInvocationPoint<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::Point>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInvocationPoint)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionRect(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectionRect)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetSelectionRect<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::Rect>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionRect)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"UI_Popups\"`"]
    #[cfg(feature = "UI_Popups")]
    pub fn PreferredPlacement(&self) -> ::windows_core::Result<super::UI::Popups::Placement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredPlacement)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Popups\"`"]
    #[cfg(feature = "UI_Popups")]
    pub fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredPlacement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for LauncherUIOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LauncherUIOptions {
    type Vtable = ILauncherUIOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LauncherUIOptions {
    const IID: ::windows_core::GUID = <ILauncherUIOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LauncherUIOptions {
    const NAME: &'static str = "Windows.System.LauncherUIOptions";
}
::windows_core::imp::interface_hierarchy!(LauncherUIOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LauncherUIOptions {}
unsafe impl ::core::marker::Sync for LauncherUIOptions {}
pub struct MemoryManager;
impl MemoryManager {
    pub fn AppMemoryUsage() -> ::windows_core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AppMemoryUsageLimit() -> ::windows_core::Result<u64> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AppMemoryUsageLevel() -> ::windows_core::Result<AppMemoryUsageLevel> {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageIncreased<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageIncreased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageIncreased(token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAppMemoryUsageIncreased)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageDecreased<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageDecreased)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageDecreased(token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAppMemoryUsageDecreased)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AppMemoryUsageLimitChanging<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>,
    {
        Self::IMemoryManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppMemoryUsageLimitChanging)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAppMemoryUsageLimitChanging(token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMemoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAppMemoryUsageLimitChanging)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn GetAppMemoryReport() -> ::windows_core::Result<AppMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAppMemoryReport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetProcessMemoryReport() -> ::windows_core::Result<ProcessMemoryReport> {
        Self::IMemoryManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProcessMemoryReport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TrySetAppMemoryUsageLimit(value: u64) -> ::windows_core::Result<bool> {
        Self::IMemoryManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetAppMemoryUsageLimit)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn ExpectedAppMemoryUsageLimit() -> ::windows_core::Result<u64> {
        Self::IMemoryManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpectedAppMemoryUsageLimit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics<R, F: FnOnce(&IMemoryManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MemoryManager, IMemoryManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics2<R, F: FnOnce(&IMemoryManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MemoryManager, IMemoryManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics3<R, F: FnOnce(&IMemoryManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MemoryManager, IMemoryManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMemoryManagerStatics4<R, F: FnOnce(&IMemoryManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MemoryManager, IMemoryManagerStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MemoryManager {
    const NAME: &'static str = "Windows.System.MemoryManager";
}
pub struct ProcessLauncher;
impl ProcessLauncher {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsync(filename: &::windows_core::HSTRING, args: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>> {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RunToCompletionAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filename), ::core::mem::transmute_copy(args), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RunToCompletionAsyncWithOptions<P0>(filename: &::windows_core::HSTRING, args: &::windows_core::HSTRING, options: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>
    where
        P0: ::windows_core::IntoParam<ProcessLauncherOptions>,
    {
        Self::IProcessLauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RunToCompletionAsyncWithOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(filename), ::core::mem::transmute_copy(args), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProcessLauncherStatics<R, F: FnOnce(&IProcessLauncherStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProcessLauncher, IProcessLauncherStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ProcessLauncher {
    const NAME: &'static str = "Windows.System.ProcessLauncher";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProcessLauncherOptions(::windows_core::IUnknown);
impl ProcessLauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProcessLauncherOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardInput(&self) -> ::windows_core::Result<super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StandardInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStandardInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardOutput(&self) -> ::windows_core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StandardOutput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardOutput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStandardOutput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StandardError(&self) -> ::windows_core::Result<super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StandardError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStandardError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStandardError)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn WorkingDirectory(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WorkingDirectory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWorkingDirectory(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWorkingDirectory)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::windows_core::RuntimeType for ProcessLauncherOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProcessLauncherOptions {
    type Vtable = IProcessLauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProcessLauncherOptions {
    const IID: ::windows_core::GUID = <IProcessLauncherOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.ProcessLauncherOptions";
}
::windows_core::imp::interface_hierarchy!(ProcessLauncherOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProcessLauncherOptions {}
unsafe impl ::core::marker::Sync for ProcessLauncherOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProcessLauncherResult(::windows_core::IUnknown);
impl ProcessLauncherResult {
    pub fn ExitCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProcessLauncherResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProcessLauncherResult {
    type Vtable = IProcessLauncherResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProcessLauncherResult {
    const IID: ::windows_core::GUID = <IProcessLauncherResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProcessLauncherResult {
    const NAME: &'static str = "Windows.System.ProcessLauncherResult";
}
::windows_core::imp::interface_hierarchy!(ProcessLauncherResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProcessLauncherResult {}
unsafe impl ::core::marker::Sync for ProcessLauncherResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProcessMemoryReport(::windows_core::IUnknown);
impl ProcessMemoryReport {
    pub fn PrivateWorkingSetUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrivateWorkingSetUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TotalWorkingSetUsage(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TotalWorkingSetUsage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProcessMemoryReport {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProcessMemoryReport {
    type Vtable = IProcessMemoryReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProcessMemoryReport {
    const IID: ::windows_core::GUID = <IProcessMemoryReport as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProcessMemoryReport {
    const NAME: &'static str = "Windows.System.ProcessMemoryReport";
}
::windows_core::imp::interface_hierarchy!(ProcessMemoryReport, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProcessMemoryReport {}
unsafe impl ::core::marker::Sync for ProcessMemoryReport {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtocolForResultsOperation(::windows_core::IUnknown);
impl ProtocolForResultsOperation {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted<P0>(&self, data: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for ProtocolForResultsOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProtocolForResultsOperation {
    type Vtable = IProtocolForResultsOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtocolForResultsOperation {
    const IID: ::windows_core::GUID = <IProtocolForResultsOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.ProtocolForResultsOperation";
}
::windows_core::imp::interface_hierarchy!(ProtocolForResultsOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtocolForResultsOperation {}
unsafe impl ::core::marker::Sync for ProtocolForResultsOperation {}
pub struct RemoteLauncher;
impl RemoteLauncher {
    #[doc = "Required features: `\"Foundation\"`, `\"System_RemoteSystems\"`"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriAsync<P0, P1>(remotesystemconnectionrequest: P0, uri: P1) -> ::windows_core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>
    where
        P0: ::windows_core::IntoParam<RemoteSystems::RemoteSystemConnectionRequest>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"System_RemoteSystems\"`"]
    #[cfg(all(feature = "Foundation", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithOptionsAsync<P0, P1, P2>(remotesystemconnectionrequest: P0, uri: P1, options: P2) -> ::windows_core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>
    where
        P0: ::windows_core::IntoParam<RemoteSystems::RemoteSystemConnectionRequest>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<RemoteLauncherOptions>,
    {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithOptionsAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"System_RemoteSystems\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System_RemoteSystems"))]
    pub fn LaunchUriWithDataAsync<P0, P1, P2, P3>(remotesystemconnectionrequest: P0, uri: P1, options: P2, inputdata: P3) -> ::windows_core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>
    where
        P0: ::windows_core::IntoParam<RemoteSystems::RemoteSystemConnectionRequest>,
        P1: ::windows_core::IntoParam<super::Foundation::Uri>,
        P2: ::windows_core::IntoParam<RemoteLauncherOptions>,
        P3: ::windows_core::IntoParam<super::Foundation::Collections::ValueSet>,
    {
        Self::IRemoteLauncherStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchUriWithDataAsync)(::windows_core::Interface::as_raw(this), remotesystemconnectionrequest.into_param().abi(), uri.into_param().abi(), options.into_param().abi(), inputdata.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteLauncherStatics<R, F: FnOnce(&IRemoteLauncherStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RemoteLauncher, IRemoteLauncherStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for RemoteLauncher {
    const NAME: &'static str = "Windows.System.RemoteLauncher";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RemoteLauncherOptions(::windows_core::IUnknown);
impl RemoteLauncherOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RemoteLauncherOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FallbackUri(&self) -> ::windows_core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetFallbackUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFallbackUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredAppIds(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredAppIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RemoteLauncherOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RemoteLauncherOptions {
    type Vtable = IRemoteLauncherOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RemoteLauncherOptions {
    const IID: ::windows_core::GUID = <IRemoteLauncherOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.RemoteLauncherOptions";
}
::windows_core::imp::interface_hierarchy!(RemoteLauncherOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RemoteLauncherOptions {}
unsafe impl ::core::marker::Sync for RemoteLauncherOptions {}
pub struct ShutdownManager;
impl ShutdownManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginShutdown(shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).BeginShutdown)(::windows_core::Interface::as_raw(this), shutdownkind, timeout).ok() })
    }
    pub fn CancelShutdown() -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).CancelShutdown)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn IsPowerStateSupported(powerstate: PowerState) -> ::windows_core::Result<bool> {
        Self::IShutdownManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPowerStateSupported)(::windows_core::Interface::as_raw(this), powerstate, &mut result__).from_abi(result__)
        })
    }
    pub fn EnterPowerState(powerstate: PowerState) -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).EnterPowerState)(::windows_core::Interface::as_raw(this), powerstate).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnterPowerStateWithTimeSpan(powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        Self::IShutdownManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).EnterPowerStateWithTimeSpan)(::windows_core::Interface::as_raw(this), powerstate, wakeupafter).ok() })
    }
    #[doc(hidden)]
    pub fn IShutdownManagerStatics<R, F: FnOnce(&IShutdownManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ShutdownManager, IShutdownManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IShutdownManagerStatics2<R, F: FnOnce(&IShutdownManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ShutdownManager, IShutdownManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ShutdownManager {
    const NAME: &'static str = "Windows.System.ShutdownManager";
}
pub struct TimeZoneSettings;
impl TimeZoneSettings {
    pub fn CurrentTimeZoneDisplayName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentTimeZoneDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTimeZoneDisplayNames() -> ::windows_core::Result<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTimeZoneDisplayNames)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CanChangeTimeZone() -> ::windows_core::Result<bool> {
        Self::ITimeZoneSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanChangeTimeZone)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ChangeTimeZoneByDisplayName(timezonedisplayname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::ITimeZoneSettingsStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ChangeTimeZoneByDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(timezonedisplayname)).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AutoUpdateTimeZoneAsync(timeout: super::Foundation::TimeSpan) -> ::windows_core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>> {
        Self::ITimeZoneSettingsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoUpdateTimeZoneAsync)(::windows_core::Interface::as_raw(this), timeout, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimeZoneSettingsStatics<R, F: FnOnce(&ITimeZoneSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITimeZoneSettingsStatics2<R, F: FnOnce(&ITimeZoneSettingsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimeZoneSettings, ITimeZoneSettingsStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for TimeZoneSettings {
    const NAME: &'static str = "Windows.System.TimeZoneSettings";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct User(::windows_core::IUnknown);
impl User {
    pub fn NonRoamableId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NonRoamableId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AuthenticationStatus(&self) -> ::windows_core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<UserType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<super::Foundation::IAsyncOperation<::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertyAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0>(&self, values: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPictureAsync)(::windows_core::Interface::as_raw(this), desiredsize, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows_core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>> {
        let this = &::windows_core::ComInterface::cast::<IUser2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckUserAgeConsentGroupAsync)(::windows_core::Interface::as_raw(this), consentgroup, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows_core::Result<UserWatcher> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByType(r#type: UserType) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncByType)(::windows_core::Interface::as_raw(this), r#type, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn FindAllAsyncByTypeAndStatus(r#type: UserType, status: UserAuthenticationStatus) -> ::windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncByTypeAndStatus)(::windows_core::Interface::as_raw(this), r#type, status, &mut result__).from_abi(result__)
        })
    }
    pub fn GetFromId(nonroamableid: &::windows_core::HSTRING) -> ::windows_core::Result<User> {
        Self::IUserStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(nonroamableid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefault() -> ::windows_core::Result<User> {
        Self::IUserStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserStatics<R, F: FnOnce(&IUserStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<User, IUserStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserStatics2<R, F: FnOnce(&IUserStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<User, IUserStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for User {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for User {
    type Vtable = IUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for User {
    const IID: ::windows_core::GUID = <IUser as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for User {
    const NAME: &'static str = "Windows.System.User";
}
::windows_core::imp::interface_hierarchy!(User, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for User {}
unsafe impl ::core::marker::Sync for User {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserAuthenticationStatusChangeDeferral(::windows_core::IUnknown);
impl UserAuthenticationStatusChangeDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for UserAuthenticationStatusChangeDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserAuthenticationStatusChangeDeferral {
    type Vtable = IUserAuthenticationStatusChangeDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserAuthenticationStatusChangeDeferral {
    const IID: ::windows_core::GUID = <IUserAuthenticationStatusChangeDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangeDeferral";
}
::windows_core::imp::interface_hierarchy!(UserAuthenticationStatusChangeDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangeDeferral {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangeDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserAuthenticationStatusChangingEventArgs(::windows_core::IUnknown);
impl UserAuthenticationStatusChangingEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<UserAuthenticationStatusChangeDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NewStatus(&self) -> ::windows_core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentStatus(&self) -> ::windows_core::Result<UserAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for UserAuthenticationStatusChangingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserAuthenticationStatusChangingEventArgs {
    type Vtable = IUserAuthenticationStatusChangingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserAuthenticationStatusChangingEventArgs {
    const IID: ::windows_core::GUID = <IUserAuthenticationStatusChangingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.UserAuthenticationStatusChangingEventArgs";
}
::windows_core::imp::interface_hierarchy!(UserAuthenticationStatusChangingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserAuthenticationStatusChangingEventArgs {}
unsafe impl ::core::marker::Sync for UserAuthenticationStatusChangingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserChangedEventArgs(::windows_core::IUnknown);
impl UserChangedEventArgs {
    pub fn User(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ChangedPropertyKinds(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>> {
        let this = &::windows_core::ComInterface::cast::<IUserChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangedPropertyKinds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for UserChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserChangedEventArgs {
    type Vtable = IUserChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserChangedEventArgs {
    const IID: ::windows_core::GUID = <IUserChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(UserChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserChangedEventArgs {}
pub struct UserDeviceAssociation;
impl UserDeviceAssociation {
    pub fn FindUserFromDeviceId(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<User> {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindUserFromDeviceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UserDeviceAssociationChanged<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>,
    {
        Self::IUserDeviceAssociationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserDeviceAssociationChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserDeviceAssociationChanged(token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IUserDeviceAssociationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveUserDeviceAssociationChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IUserDeviceAssociationStatics<R, F: FnOnce(&IUserDeviceAssociationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UserDeviceAssociation, IUserDeviceAssociationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for UserDeviceAssociation {
    const NAME: &'static str = "Windows.System.UserDeviceAssociation";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDeviceAssociationChangedEventArgs(::windows_core::IUnknown);
impl UserDeviceAssociationChangedEventArgs {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NewUser(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewUser)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OldUser(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldUser)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for UserDeviceAssociationChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDeviceAssociationChangedEventArgs {
    type Vtable = IUserDeviceAssociationChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDeviceAssociationChangedEventArgs {
    const IID: ::windows_core::GUID = <IUserDeviceAssociationChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.UserDeviceAssociationChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(UserDeviceAssociationChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserDeviceAssociationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDeviceAssociationChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserPicker(::windows_core::IUnknown);
impl UserPicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UserPicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AllowGuestAccounts(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowGuestAccounts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowGuestAccounts(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowGuestAccounts)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SuggestedSelectedUser(&self) -> ::windows_core::Result<User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedSelectedUser)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSuggestedSelectedUser<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<User>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuggestedSelectedUser)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleUserAsync(&self) -> ::windows_core::Result<super::Foundation::IAsyncOperation<User>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleUserAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IUserPickerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserPickerStatics<R, F: FnOnce(&IUserPickerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UserPicker, IUserPickerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for UserPicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserPicker {
    type Vtable = IUserPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserPicker {
    const IID: ::windows_core::GUID = <IUserPicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserPicker {
    const NAME: &'static str = "Windows.System.UserPicker";
}
::windows_core::imp::interface_hierarchy!(UserPicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserPicker {}
unsafe impl ::core::marker::Sync for UserPicker {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserWatcher(::windows_core::IUnknown);
impl UserWatcher {
    pub fn Status(&self) -> ::windows_core::Result<UserWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanged(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAuthenticationStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationStatusChanging<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationStatusChanging)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAuthenticationStatusChanging(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAuthenticationStatusChanging)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<UserWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<UserWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for UserWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserWatcher {
    type Vtable = IUserWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserWatcher {
    const IID: ::windows_core::GUID = <IUserWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserWatcher {
    const NAME: &'static str = "Windows.System.UserWatcher";
}
::windows_core::imp::interface_hierarchy!(UserWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserWatcher {}
unsafe impl ::core::marker::Sync for UserWatcher {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppDiagnosticInfoWatcherStatus(pub i32);
impl AppDiagnosticInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppDiagnosticInfoWatcherStatus {}
impl ::core::clone::Clone for AppDiagnosticInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppDiagnosticInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppDiagnosticInfoWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppDiagnosticInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppDiagnosticInfoWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppDiagnosticInfoWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.AppDiagnosticInfoWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppMemoryUsageLevel(pub i32);
impl AppMemoryUsageLevel {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
    pub const OverLimit: Self = Self(3i32);
}
impl ::core::marker::Copy for AppMemoryUsageLevel {}
impl ::core::clone::Clone for AppMemoryUsageLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppMemoryUsageLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppMemoryUsageLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppMemoryUsageLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppMemoryUsageLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppMemoryUsageLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.AppMemoryUsageLevel;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppResourceGroupEnergyQuotaState(pub i32);
impl AppResourceGroupEnergyQuotaState {
    pub const Unknown: Self = Self(0i32);
    pub const Over: Self = Self(1i32);
    pub const Under: Self = Self(2i32);
}
impl ::core::marker::Copy for AppResourceGroupEnergyQuotaState {}
impl ::core::clone::Clone for AppResourceGroupEnergyQuotaState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupEnergyQuotaState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppResourceGroupEnergyQuotaState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppResourceGroupEnergyQuotaState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupEnergyQuotaState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupEnergyQuotaState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupEnergyQuotaState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppResourceGroupExecutionState(pub i32);
impl AppResourceGroupExecutionState {
    pub const Unknown: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspending: Self = Self(2i32);
    pub const Suspended: Self = Self(3i32);
    pub const NotRunning: Self = Self(4i32);
}
impl ::core::marker::Copy for AppResourceGroupExecutionState {}
impl ::core::clone::Clone for AppResourceGroupExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupExecutionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppResourceGroupExecutionState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppResourceGroupExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupExecutionState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupExecutionState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupExecutionState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppResourceGroupInfoWatcherStatus(pub i32);
impl AppResourceGroupInfoWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for AppResourceGroupInfoWatcherStatus {}
impl ::core::clone::Clone for AppResourceGroupInfoWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppResourceGroupInfoWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppResourceGroupInfoWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppResourceGroupInfoWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppResourceGroupInfoWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppResourceGroupInfoWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.AppResourceGroupInfoWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutoUpdateTimeZoneStatus(pub i32);
impl AutoUpdateTimeZoneStatus {
    pub const Attempted: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoUpdateTimeZoneStatus {}
impl ::core::clone::Clone for AutoUpdateTimeZoneStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateTimeZoneStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutoUpdateTimeZoneStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutoUpdateTimeZoneStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateTimeZoneStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutoUpdateTimeZoneStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.AutoUpdateTimeZoneStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DiagnosticAccessStatus(pub i32);
impl DiagnosticAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
    pub const Allowed: Self = Self(3i32);
}
impl ::core::marker::Copy for DiagnosticAccessStatus {}
impl ::core::clone::Clone for DiagnosticAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DiagnosticAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DiagnosticAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DiagnosticAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DiagnosticAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.DiagnosticAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl ::core::marker::Copy for DispatcherQueuePriority {}
impl ::core::clone::Clone for DispatcherQueuePriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DispatcherQueuePriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DispatcherQueuePriority {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DispatcherQueuePriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueuePriority").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.DispatcherQueuePriority;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchFileStatus(pub i32);
impl LaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const DeniedByPolicy: Self = Self(2i32);
    pub const FileTypeNotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchFileStatus {}
impl ::core::clone::Clone for LaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LaunchFileStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LaunchFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchFileStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LaunchFileStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.LaunchFileStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchQuerySupportStatus(pub i32);
impl LaunchQuerySupportStatus {
    pub const Available: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const NotSupported: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for LaunchQuerySupportStatus {}
impl ::core::clone::Clone for LaunchQuerySupportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LaunchQuerySupportStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LaunchQuerySupportStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LaunchQuerySupportStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchQuerySupportType(pub i32);
impl LaunchQuerySupportType {
    pub const Uri: Self = Self(0i32);
    pub const UriForResults: Self = Self(1i32);
}
impl ::core::marker::Copy for LaunchQuerySupportType {}
impl ::core::clone::Clone for LaunchQuerySupportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchQuerySupportType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LaunchQuerySupportType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LaunchQuerySupportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchQuerySupportType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LaunchQuerySupportType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.LaunchQuerySupportType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LaunchUriStatus(pub i32);
impl LaunchUriStatus {
    pub const Success: Self = Self(0i32);
    pub const AppUnavailable: Self = Self(1i32);
    pub const ProtocolUnavailable: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for LaunchUriStatus {}
impl ::core::clone::Clone for LaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LaunchUriStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchUriStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LaunchUriStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.LaunchUriStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PowerState(pub i32);
impl PowerState {
    pub const ConnectedStandby: Self = Self(0i32);
    pub const SleepS3: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerState {}
impl ::core::clone::Clone for PowerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PowerState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PowerState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.PowerState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProcessorArchitecture(pub i32);
impl ProcessorArchitecture {
    pub const X86: Self = Self(0i32);
    pub const Arm: Self = Self(5i32);
    pub const X64: Self = Self(9i32);
    pub const Neutral: Self = Self(11i32);
    pub const Arm64: Self = Self(12i32);
    pub const X86OnArm64: Self = Self(14i32);
    pub const Unknown: Self = Self(65535i32);
}
impl ::core::marker::Copy for ProcessorArchitecture {}
impl ::core::clone::Clone for ProcessorArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProcessorArchitecture {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProcessorArchitecture {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProcessorArchitecture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessorArchitecture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProcessorArchitecture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.ProcessorArchitecture;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteLaunchUriStatus(pub i32);
impl RemoteLaunchUriStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const ProtocolUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const ValueSetTooLarge: Self = Self(5i32);
    pub const DeniedByLocalSystem: Self = Self(6i32);
    pub const DeniedByRemoteSystem: Self = Self(7i32);
}
impl ::core::marker::Copy for RemoteLaunchUriStatus {}
impl ::core::clone::Clone for RemoteLaunchUriStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteLaunchUriStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RemoteLaunchUriStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RemoteLaunchUriStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteLaunchUriStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RemoteLaunchUriStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.RemoteLaunchUriStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ShutdownKind(pub i32);
impl ShutdownKind {
    pub const Shutdown: Self = Self(0i32);
    pub const Restart: Self = Self(1i32);
}
impl ::core::marker::Copy for ShutdownKind {}
impl ::core::clone::Clone for ShutdownKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ShutdownKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ShutdownKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ShutdownKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.ShutdownKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserAgeConsentGroup(pub i32);
impl UserAgeConsentGroup {
    pub const Child: Self = Self(0i32);
    pub const Minor: Self = Self(1i32);
    pub const Adult: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAgeConsentGroup {}
impl ::core::clone::Clone for UserAgeConsentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentGroup {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserAgeConsentGroup {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserAgeConsentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentGroup").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserAgeConsentGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentGroup;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserAgeConsentResult(pub i32);
impl UserAgeConsentResult {
    pub const NotEnforced: Self = Self(0i32);
    pub const Included: Self = Self(1i32);
    pub const NotIncluded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const Ambiguous: Self = Self(4i32);
}
impl ::core::marker::Copy for UserAgeConsentResult {}
impl ::core::clone::Clone for UserAgeConsentResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAgeConsentResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserAgeConsentResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserAgeConsentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAgeConsentResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserAgeConsentResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserAgeConsentResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserAuthenticationStatus(pub i32);
impl UserAuthenticationStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const LocallyAuthenticated: Self = Self(1i32);
    pub const RemotelyAuthenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for UserAuthenticationStatus {}
impl ::core::clone::Clone for UserAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserAuthenticationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserAuthenticationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserAuthenticationStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserPictureSize(pub i32);
impl UserPictureSize {
    pub const Size64x64: Self = Self(0i32);
    pub const Size208x208: Self = Self(1i32);
    pub const Size424x424: Self = Self(2i32);
    pub const Size1080x1080: Self = Self(3i32);
}
impl ::core::marker::Copy for UserPictureSize {}
impl ::core::clone::Clone for UserPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserPictureSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserPictureSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPictureSize").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserPictureSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserPictureSize;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserType(pub i32);
impl UserType {
    pub const LocalUser: Self = Self(0i32);
    pub const RemoteUser: Self = Self(1i32);
    pub const LocalGuest: Self = Self(2i32);
    pub const RemoteGuest: Self = Self(3i32);
    pub const SystemManaged: Self = Self(4i32);
}
impl ::core::marker::Copy for UserType {}
impl ::core::clone::Clone for UserType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserWatcherStatus(pub i32);
impl UserWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for UserWatcherStatus {}
impl ::core::clone::Clone for UserWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserWatcherUpdateKind(pub i32);
impl UserWatcherUpdateKind {
    pub const Properties: Self = Self(0i32);
    pub const Picture: Self = Self(1i32);
}
impl ::core::marker::Copy for UserWatcherUpdateKind {}
impl ::core::clone::Clone for UserWatcherUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserWatcherUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserWatcherUpdateKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserWatcherUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserWatcherUpdateKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserWatcherUpdateKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.UserWatcherUpdateKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VirtualKey(pub i32);
impl VirtualKey {
    pub const None: Self = Self(0i32);
    pub const LeftButton: Self = Self(1i32);
    pub const RightButton: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const MiddleButton: Self = Self(4i32);
    pub const XButton1: Self = Self(5i32);
    pub const XButton2: Self = Self(6i32);
    pub const Back: Self = Self(8i32);
    pub const Tab: Self = Self(9i32);
    pub const Clear: Self = Self(12i32);
    pub const Enter: Self = Self(13i32);
    pub const Shift: Self = Self(16i32);
    pub const Control: Self = Self(17i32);
    pub const Menu: Self = Self(18i32);
    pub const Pause: Self = Self(19i32);
    pub const CapitalLock: Self = Self(20i32);
    pub const Kana: Self = Self(21i32);
    pub const Hangul: Self = Self(21i32);
    pub const ImeOn: Self = Self(22i32);
    pub const Junja: Self = Self(23i32);
    pub const Final: Self = Self(24i32);
    pub const Hanja: Self = Self(25i32);
    pub const Kanji: Self = Self(25i32);
    pub const ImeOff: Self = Self(26i32);
    pub const Escape: Self = Self(27i32);
    pub const Convert: Self = Self(28i32);
    pub const NonConvert: Self = Self(29i32);
    pub const Accept: Self = Self(30i32);
    pub const ModeChange: Self = Self(31i32);
    pub const Space: Self = Self(32i32);
    pub const PageUp: Self = Self(33i32);
    pub const PageDown: Self = Self(34i32);
    pub const End: Self = Self(35i32);
    pub const Home: Self = Self(36i32);
    pub const Left: Self = Self(37i32);
    pub const Up: Self = Self(38i32);
    pub const Right: Self = Self(39i32);
    pub const Down: Self = Self(40i32);
    pub const Select: Self = Self(41i32);
    pub const Print: Self = Self(42i32);
    pub const Execute: Self = Self(43i32);
    pub const Snapshot: Self = Self(44i32);
    pub const Insert: Self = Self(45i32);
    pub const Delete: Self = Self(46i32);
    pub const Help: Self = Self(47i32);
    pub const Number0: Self = Self(48i32);
    pub const Number1: Self = Self(49i32);
    pub const Number2: Self = Self(50i32);
    pub const Number3: Self = Self(51i32);
    pub const Number4: Self = Self(52i32);
    pub const Number5: Self = Self(53i32);
    pub const Number6: Self = Self(54i32);
    pub const Number7: Self = Self(55i32);
    pub const Number8: Self = Self(56i32);
    pub const Number9: Self = Self(57i32);
    pub const A: Self = Self(65i32);
    pub const B: Self = Self(66i32);
    pub const C: Self = Self(67i32);
    pub const D: Self = Self(68i32);
    pub const E: Self = Self(69i32);
    pub const F: Self = Self(70i32);
    pub const G: Self = Self(71i32);
    pub const H: Self = Self(72i32);
    pub const I: Self = Self(73i32);
    pub const J: Self = Self(74i32);
    pub const K: Self = Self(75i32);
    pub const L: Self = Self(76i32);
    pub const M: Self = Self(77i32);
    pub const N: Self = Self(78i32);
    pub const O: Self = Self(79i32);
    pub const P: Self = Self(80i32);
    pub const Q: Self = Self(81i32);
    pub const R: Self = Self(82i32);
    pub const S: Self = Self(83i32);
    pub const T: Self = Self(84i32);
    pub const U: Self = Self(85i32);
    pub const V: Self = Self(86i32);
    pub const W: Self = Self(87i32);
    pub const X: Self = Self(88i32);
    pub const Y: Self = Self(89i32);
    pub const Z: Self = Self(90i32);
    pub const LeftWindows: Self = Self(91i32);
    pub const RightWindows: Self = Self(92i32);
    pub const Application: Self = Self(93i32);
    pub const Sleep: Self = Self(95i32);
    pub const NumberPad0: Self = Self(96i32);
    pub const NumberPad1: Self = Self(97i32);
    pub const NumberPad2: Self = Self(98i32);
    pub const NumberPad3: Self = Self(99i32);
    pub const NumberPad4: Self = Self(100i32);
    pub const NumberPad5: Self = Self(101i32);
    pub const NumberPad6: Self = Self(102i32);
    pub const NumberPad7: Self = Self(103i32);
    pub const NumberPad8: Self = Self(104i32);
    pub const NumberPad9: Self = Self(105i32);
    pub const Multiply: Self = Self(106i32);
    pub const Add: Self = Self(107i32);
    pub const Separator: Self = Self(108i32);
    pub const Subtract: Self = Self(109i32);
    pub const Decimal: Self = Self(110i32);
    pub const Divide: Self = Self(111i32);
    pub const F1: Self = Self(112i32);
    pub const F2: Self = Self(113i32);
    pub const F3: Self = Self(114i32);
    pub const F4: Self = Self(115i32);
    pub const F5: Self = Self(116i32);
    pub const F6: Self = Self(117i32);
    pub const F7: Self = Self(118i32);
    pub const F8: Self = Self(119i32);
    pub const F9: Self = Self(120i32);
    pub const F10: Self = Self(121i32);
    pub const F11: Self = Self(122i32);
    pub const F12: Self = Self(123i32);
    pub const F13: Self = Self(124i32);
    pub const F14: Self = Self(125i32);
    pub const F15: Self = Self(126i32);
    pub const F16: Self = Self(127i32);
    pub const F17: Self = Self(128i32);
    pub const F18: Self = Self(129i32);
    pub const F19: Self = Self(130i32);
    pub const F20: Self = Self(131i32);
    pub const F21: Self = Self(132i32);
    pub const F22: Self = Self(133i32);
    pub const F23: Self = Self(134i32);
    pub const F24: Self = Self(135i32);
    pub const NavigationView: Self = Self(136i32);
    pub const NavigationMenu: Self = Self(137i32);
    pub const NavigationUp: Self = Self(138i32);
    pub const NavigationDown: Self = Self(139i32);
    pub const NavigationLeft: Self = Self(140i32);
    pub const NavigationRight: Self = Self(141i32);
    pub const NavigationAccept: Self = Self(142i32);
    pub const NavigationCancel: Self = Self(143i32);
    pub const NumberKeyLock: Self = Self(144i32);
    pub const Scroll: Self = Self(145i32);
    pub const LeftShift: Self = Self(160i32);
    pub const RightShift: Self = Self(161i32);
    pub const LeftControl: Self = Self(162i32);
    pub const RightControl: Self = Self(163i32);
    pub const LeftMenu: Self = Self(164i32);
    pub const RightMenu: Self = Self(165i32);
    pub const GoBack: Self = Self(166i32);
    pub const GoForward: Self = Self(167i32);
    pub const Refresh: Self = Self(168i32);
    pub const Stop: Self = Self(169i32);
    pub const Search: Self = Self(170i32);
    pub const Favorites: Self = Self(171i32);
    pub const GoHome: Self = Self(172i32);
    pub const GamepadA: Self = Self(195i32);
    pub const GamepadB: Self = Self(196i32);
    pub const GamepadX: Self = Self(197i32);
    pub const GamepadY: Self = Self(198i32);
    pub const GamepadRightShoulder: Self = Self(199i32);
    pub const GamepadLeftShoulder: Self = Self(200i32);
    pub const GamepadLeftTrigger: Self = Self(201i32);
    pub const GamepadRightTrigger: Self = Self(202i32);
    pub const GamepadDPadUp: Self = Self(203i32);
    pub const GamepadDPadDown: Self = Self(204i32);
    pub const GamepadDPadLeft: Self = Self(205i32);
    pub const GamepadDPadRight: Self = Self(206i32);
    pub const GamepadMenu: Self = Self(207i32);
    pub const GamepadView: Self = Self(208i32);
    pub const GamepadLeftThumbstickButton: Self = Self(209i32);
    pub const GamepadRightThumbstickButton: Self = Self(210i32);
    pub const GamepadLeftThumbstickUp: Self = Self(211i32);
    pub const GamepadLeftThumbstickDown: Self = Self(212i32);
    pub const GamepadLeftThumbstickRight: Self = Self(213i32);
    pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
    pub const GamepadRightThumbstickUp: Self = Self(215i32);
    pub const GamepadRightThumbstickDown: Self = Self(216i32);
    pub const GamepadRightThumbstickRight: Self = Self(217i32);
    pub const GamepadRightThumbstickLeft: Self = Self(218i32);
}
impl ::core::marker::Copy for VirtualKey {}
impl ::core::clone::Clone for VirtualKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKey {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VirtualKey {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VirtualKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKey").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VirtualKey {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VirtualKeyModifiers(pub u32);
impl VirtualKeyModifiers {
    pub const None: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Menu: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Windows: Self = Self(8u32);
}
impl ::core::marker::Copy for VirtualKeyModifiers {}
impl ::core::clone::Clone for VirtualKeyModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKeyModifiers {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VirtualKeyModifiers {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VirtualKeyModifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKeyModifiers").field(&self.0).finish()
    }
}
impl VirtualKeyModifiers {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VirtualKeyModifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VirtualKeyModifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VirtualKeyModifiers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VirtualKeyModifiers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VirtualKeyModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for VirtualKeyModifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKeyModifiers;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueHandler(pub ::windows_core::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DispatcherQueueHandlerBox::<F> { vtable: &DispatcherQueueHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct DispatcherQueueHandlerBox<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DispatcherQueueHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DispatcherQueueHandlerBox<F> {
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DispatcherQueueHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
unsafe impl ::windows_core::Interface for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfa2dc9c_1a2d_4917_98f2_939af1d6e0c8);
}
impl ::windows_core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
