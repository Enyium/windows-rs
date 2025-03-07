#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESim(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESim {
    type Vtable = IESim_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESim {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f6e6e26_f123_437d_8ced_dc1d2bc0c3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESim_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AvailableMemoryInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailableMemoryInBytes: usize,
    pub Eid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirmwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MobileBroadbandModemDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProfiles: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadProfileMetadataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationcode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadProfileMetadataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProfileChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProfileChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProfileChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESim2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESim2 {
    type Vtable = IESim2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESim2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd4fd0a0_c68f_56eb_b99b_8f34b8100299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESim2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Discover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DiscoverWithServerAddressAndMatchingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serveraddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, matchingid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DiscoverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiscoverAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DiscoverWithServerAddressAndMatchingIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serveraddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, matchingid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiscoverWithServerAddressAndMatchingIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESim3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESim3 {
    type Vtable = IESim3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESim3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe1edf45_01b8_5d31_b8d3_d9cbebb2b831);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESim3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SlotIndex: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimAddedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimAddedEventArgs {
    type Vtable = IESimAddedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimAddedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38bd0a58_4d5a_4d08_8da7_e73eff369ddd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimAddedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ESim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimDiscoverEvent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimDiscoverEvent {
    type Vtable = IESimDiscoverEvent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimDiscoverEvent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe59ac3e3_39bc_5f6f_9321_0d4a182d261b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDiscoverEvent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MatchingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RspServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimDiscoverResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimDiscoverResult {
    type Vtable = IESimDiscoverResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimDiscoverResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56b4bb5e_ab2f_5ac6_b359_dd5a8e237926);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDiscoverResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Events: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Events: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimDiscoverResultKind) -> ::windows_core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimDownloadProfileMetadataResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimDownloadProfileMetadataResult {
    type Vtable = IESimDownloadProfileMetadataResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimDownloadProfileMetadataResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4234d9e_5ad6_426d_8d00_4434f449afec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimDownloadProfileMetadataResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProfileMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimManagerStatics {
    type Vtable = IESimManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bfa2c0c_df88_4631_bf04_c12e281b3962);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServiceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryCreateESimWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceInfoChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimOperationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimOperationResult {
    type Vtable = IESimOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimOperationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa67b63b1_309b_4e77_9e7e_cd93f1ddc7b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimOperationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimOperationStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimPolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimPolicy {
    type Vtable = IESimPolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimPolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41e1b99d_cf7e_4315_882b_6f1e74b0d38f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimPolicy_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShouldEnableManagingUi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimProfile {
    type Vtable = IESimProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee1e7880_06a9_4027_b4f8_ddb23d7810e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileClass) -> ::windows_core::HRESULT,
    pub Nickname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetNicknameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newnickname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNicknameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimProfileMetadata(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimProfileMetadata {
    type Vtable = IESimProfileMetadata_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimProfileMetadata {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed25831f_90db_498d_a7b4_ebce807d3c23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfileMetadata_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsConfirmationCodeRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ProviderIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProviderIcon: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileMetadataState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DenyInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DenyInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfirmInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfirmInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfirmInstallWithConfirmationCodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confirmationcode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfirmInstallWithConfirmationCodeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PostponeInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PostponeInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimProfilePolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimProfilePolicy {
    type Vtable = IESimProfilePolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimProfilePolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6dd0f1d_9c5c_46c5_a289_a948999bf062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimProfilePolicy_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsManagedByEnterprise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimRemovedEventArgs {
    type Vtable = IESimRemovedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimRemovedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdec5277b_2fd9_4ed9_8376_d9b5e41278a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ESim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimServiceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimServiceInfo {
    type Vtable = IESimServiceInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimServiceInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf16aabcf_7f59_4a51_8494_bd89d5ff50ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimServiceInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AuthenticationPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimAuthenticationPreference) -> ::windows_core::HRESULT,
    pub IsESimUiEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimUpdatedEventArgs {
    type Vtable = IESimUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimUpdatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c125cec_508d_4b88_83cb_68bef8168d12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ESim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IESimWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IESimWatcher {
    type Vtable = IESimWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IESimWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1f84ceb_a28d_4fbf_9771_6e31b81ccf22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IESimWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ESimWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFdnAccessManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFdnAccessManagerStatics {
    type Vtable = IFdnAccessManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFdnAccessManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2aa4395_f1e6_4319_aa3e_477ca64b2bdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFdnAccessManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestUnlockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactlistid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestUnlockAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHotspotAuthenticationContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHotspotAuthenticationContext {
    type Vtable = IHotspotAuthenticationContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHotspotAuthenticationContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe756c791_1003_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WirelessNetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    #[cfg(feature = "Foundation")]
    pub RedirectMessageUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RedirectMessageUrl: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub RedirectMessageXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    RedirectMessageXml: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticationUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticationUrl: usize,
    pub IssueCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, password: ::std::mem::MaybeUninit<::windows_core::HSTRING>, extraparameters: ::std::mem::MaybeUninit<::windows_core::HSTRING>, markasmanualconnectonfailure: bool) -> ::windows_core::HRESULT,
    pub AbortAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, markasmanual: bool) -> ::windows_core::HRESULT,
    pub SkipAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TriggerAttentionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagerelativeapplicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, applicationparameters: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHotspotAuthenticationContext2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHotspotAuthenticationContext2 {
    type Vtable = IHotspotAuthenticationContext2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHotspotAuthenticationContext2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe756c791_1004_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContext2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IssueCredentialsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, password: ::std::mem::MaybeUninit<::windows_core::HSTRING>, extraparameters: ::std::mem::MaybeUninit<::windows_core::HSTRING>, markasmanualconnectonfailure: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IssueCredentialsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHotspotAuthenticationContextStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHotspotAuthenticationContextStatics {
    type Vtable = IHotspotAuthenticationContextStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHotspotAuthenticationContextStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe756c791_1002_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationContextStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryGetAuthenticationContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventoken: ::std::mem::MaybeUninit<::windows_core::HSTRING>, context: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHotspotAuthenticationEventDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHotspotAuthenticationEventDetails {
    type Vtable = IHotspotAuthenticationEventDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHotspotAuthenticationEventDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe756c791_1001_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotAuthenticationEventDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EventToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHotspotCredentialsAuthenticationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHotspotCredentialsAuthenticationResult {
    type Vtable = IHotspotCredentialsAuthenticationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHotspotCredentialsAuthenticationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe756c791_1005_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHotspotCredentialsAuthenticationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasNetworkErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ResponseCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HotspotAuthenticationResponseCode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LogoffUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LogoffUrl: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub AuthenticationReplyXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    AuthenticationReplyXml: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKnownCSimFilePathsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownCSimFilePathsStatics {
    type Vtable = IKnownCSimFilePathsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKnownCSimFilePathsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb458aeed_49f1_4c22_b073_96d511bf9c35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownCSimFilePathsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKnownRuimFilePathsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownRuimFilePathsStatics {
    type Vtable = IKnownRuimFilePathsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKnownRuimFilePathsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3883c8b9_ff24_4571_a867_09f960426e14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRuimFilePathsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKnownSimFilePathsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownSimFilePathsStatics {
    type Vtable = IKnownSimFilePathsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKnownSimFilePathsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80cd1a63_37a5_43d3_80a3_ccd23e8fecee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimFilePathsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOns: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKnownUSimFilePathsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownUSimFilePathsStatics {
    type Vtable = IKnownUSimFilePathsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKnownUSimFilePathsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c34e581_1f1b_43f4_9530_8b092d32d71f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownUSimFilePathsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub EFSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFSpn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFOpl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFOpl: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EFPnn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EFPnn: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid1: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Gid2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gid2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAccount(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAccount {
    type Vtable = IMobileBroadbandAccount_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAccount {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36c24ccd_cee2_43e0_a603_ee86a36d6570);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ServiceProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CurrentNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CurrentDeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAccount2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAccount2 {
    type Vtable = IMobileBroadbandAccount2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAccount2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38f52f1c_1136_4257_959f_b658a352b6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking_Connectivity")))]
    GetConnectionProfiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAccount3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAccount3 {
    type Vtable = IMobileBroadbandAccount3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAccount3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x092a1e21_9379_4b9b_ad31_d5fee2f748c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccount3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AccountExperienceUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountExperienceUrl: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAccountEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAccountEventArgs {
    type Vtable = IMobileBroadbandAccountEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAccountEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3853c880_77de_4c04_bead_a123b08c9f59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAccountStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAccountStatics {
    type Vtable = IMobileBroadbandAccountStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAccountStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa7f4d24_afc1_4fc8_ae9a_a9175310faad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableNetworkAccountIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableNetworkAccountIds: usize,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAccountUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAccountUpdatedEventArgs {
    type Vtable = IMobileBroadbandAccountUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAccountUpdatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bc31d88_a6bd_49e1_80ab_6b91354a57d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasDeviceInformationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNetworkChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAccountWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAccountWatcher {
    type Vtable = IMobileBroadbandAccountWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAccountWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bf3335e_23b5_449f_928d_5e0d3e04471d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAccountWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AccountAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountAdded: usize,
    #[cfg(feature = "Foundation")]
    pub AccountUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub AccountRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccountRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccountRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccountRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandAccountWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAntennaSar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAntennaSar {
    type Vtable = IMobileBroadbandAntennaSar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAntennaSar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9af4b7e_cbf9_4109_90be_5c06bfd513b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AntennaIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SarBackoffIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandAntennaSarFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandAntennaSarFactory {
    type Vtable = IMobileBroadbandAntennaSarFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandAntennaSarFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa91e1716_c04d_4a21_8698_1459dc672c6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandAntennaSarFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, antennaindex: i32, sarbackoffindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellCdma(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellCdma {
    type Vtable = IMobileBroadbandCellCdma_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellCdma {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0601b3b4_411a_4f2e_8287_76f5650c60cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellCdma_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BaseStationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationId: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationPNCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationPNCode: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLatitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLatitude: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLongitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLongitude: usize,
    #[cfg(feature = "Foundation")]
    pub BaseStationLastBroadcastGpsTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationLastBroadcastGpsTime: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkId: usize,
    #[cfg(feature = "Foundation")]
    pub PilotSignalStrengthInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PilotSignalStrengthInDB: usize,
    #[cfg(feature = "Foundation")]
    pub SystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellGsm(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellGsm {
    type Vtable = IMobileBroadbandCellGsm_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellGsm {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc917f06_7ee0_47b8_9e1f_c3b48df9df5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellGsm_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BaseStationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BaseStationId: usize,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalStrengthInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalStrengthInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellLte(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellLte {
    type Vtable = IMobileBroadbandCellLte_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellLte {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9197c87b_2b78_456d_8b53_aaa25d0af741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellLte_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalCellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalCellId: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedPowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedQualityInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
    #[cfg(feature = "Foundation")]
    pub TrackingAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackingAreaCode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellNR(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellNR {
    type Vtable = IMobileBroadbandCellNR_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellNR {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa13f0deb_66fc_4b4b_83a9_a487a3a5a0a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellNR_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub PhysicalCellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhysicalCellId: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedPowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedPowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub ReferenceSignalReceivedQualityInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReferenceSignalReceivedQualityInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInNanoseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInNanoseconds: usize,
    #[cfg(feature = "Foundation")]
    pub TrackingAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackingAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalToNoiseRatioInDB: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellTdscdma(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellTdscdma {
    type Vtable = IMobileBroadbandCellTdscdma_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellTdscdma {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0eda1655_db0e_4182_8cda_cc419a7bde08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellTdscdma_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub CellParameterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellParameterId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub PathLossInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathLossInDB: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalCodePowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub TimingAdvanceInBitPeriods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimingAdvanceInBitPeriods: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellUmts(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellUmts {
    type Vtable = IMobileBroadbandCellUmts_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellUmts {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77b4b5ae_49c8_4f15_b285_4c26a7f67215);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellUmts_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CellId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellId: usize,
    #[cfg(feature = "Foundation")]
    pub ChannelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChannelNumber: usize,
    #[cfg(feature = "Foundation")]
    pub LocationAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocationAreaCode: usize,
    #[cfg(feature = "Foundation")]
    pub PathLossInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathLossInDB: usize,
    #[cfg(feature = "Foundation")]
    pub PrimaryScramblingCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrimaryScramblingCode: usize,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReceivedSignalCodePowerInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReceivedSignalCodePowerInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SignalToNoiseRatioInDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalToNoiseRatioInDB: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellsInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellsInfo {
    type Vtable = IMobileBroadbandCellsInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellsInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89a9562a_e472_4da5_929c_de61711dd261);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsCdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsGsm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsLte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsTdscdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsUmts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsUmts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsCdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsCdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsGsm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsGsm: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsLte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsLte: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsTdscdma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsTdscdma: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsUmts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsUmts: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCellsInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCellsInfo2 {
    type Vtable = IMobileBroadbandCellsInfo2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCellsInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66205912_b89f_4e12_bbb6_d5cf09a820ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCellsInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NeighboringCellsNR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NeighboringCellsNR: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ServingCellsNR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServingCellsNR: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    type Vtable = IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf718b184_c370_5fd4_a670_1846cb9bce47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceInformation {
    type Vtable = IMobileBroadbandDeviceInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6d08168_e381_4c6e_9be8_fe156969a446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkDeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkDeviceStatus) -> ::windows_core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FirmwareInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub CellularClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sms::CellularClass) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    CellularClass: usize,
    pub DataClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows_core::HRESULT,
    pub CustomDataClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MobileEquipmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TelephoneNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TelephoneNumbers: usize,
    pub SubscriberId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SimIccId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandDeviceType) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CurrentRadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceInformation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceInformation2 {
    type Vtable = IMobileBroadbandDeviceInformation2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceInformation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e467af1_f932_4737_a722_03ba72370cb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PinManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceInformation3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceInformation3 {
    type Vtable = IMobileBroadbandDeviceInformation3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceInformation3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe08bb4bd_5d30_4b5a_92cc_d54df881d49e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SimSpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SimPnn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SimGid1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceInformation4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceInformation4 {
    type Vtable = IMobileBroadbandDeviceInformation4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceInformation4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x263f3152_7b9d_582c_b17c_f80a60b50031);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceInformation4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SlotManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceService {
    type Vtable = IMobileBroadbandDeviceService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22be1a52_bd80_40ac_8e1f_2e07836a3dbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceService_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCommands: usize,
    pub OpenDataSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenCommandSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceServiceCommandResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceServiceCommandResult {
    type Vtable = IMobileBroadbandDeviceServiceCommandResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceServiceCommandResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0f46abb_94d6_44b9_a538_f0810b645389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ResponseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ResponseData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceServiceCommandSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceServiceCommandSession {
    type Vtable = IMobileBroadbandDeviceServiceCommandSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceServiceCommandSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc098a45_913b_4914_b6c3_ae6304593e75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceCommandSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendQueryCommandAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendQueryCommandAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendSetCommandAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u32, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendSetCommandAsync: usize,
    pub CloseSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    type Vtable = IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6aa13de_1380_40e3_8618_73cbca48138c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceServiceDataSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceServiceDataSession {
    type Vtable = IMobileBroadbandDeviceServiceDataSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceServiceDataSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdad62333_8bcf_4289_8a37_045c2169486a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceDataSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub WriteDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    WriteDataAsync: usize,
    pub CloseSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceServiceInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceServiceInformation {
    type Vtable = IMobileBroadbandDeviceServiceInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceServiceInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53d69b5b_c4ed_45f0_803a_d9417a6d9846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IsDataReadSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDataWriteSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceServiceTriggerDetails {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceServiceTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a055b70_b9ae_4458_9241_a6a5fbf18a0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DeviceServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ReceivedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReceivedData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandDeviceServiceTriggerDetails2 {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandDeviceServiceTriggerDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd83d5f16_336a_553f_94bb_0cd1a2ff0c81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandDeviceServiceTriggerDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EventId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModem {
    type Vtable = IMobileBroadbandModem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0356912_e9f9_4f67_a03d_43189a316bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MaxDeviceServiceCommandSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxDeviceServiceDataSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceServices: usize,
    pub GetDeviceService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceserviceid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsResetSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCurrentConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentConfigurationAsync: usize,
    pub CurrentNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModem2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModem2 {
    type Vtable = IMobileBroadbandModem2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12862b28_b9eb_4ee2_bbe3_711f53eea373);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetIsPassthroughEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPassthroughEnabledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsPassthroughEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsPassthroughEnabledAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModem3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModem3 {
    type Vtable = IMobileBroadbandModem3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModem3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9fec6ea_2f34_4582_9102_c314d2a87eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryGetPcoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetPcoAsync: usize,
    pub IsInEmergencyCallMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsInEmergencyCallModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsInEmergencyCallModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsInEmergencyCallModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsInEmergencyCallModeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModem4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModem4 {
    type Vtable = IMobileBroadbandModem4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModem4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a0398c2_91be_412b_b569_586e9f0030d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModem4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetIsPassthroughEnabledWithSlotIndexAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, slotindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsPassthroughEnabledWithSlotIndexAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsPassthroughEnabledWithSlotIndexAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPassthroughEnabledWithSlotIndexAsync: usize,
    pub SetIsPassthroughEnabledWithSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, slotindex: i32, result__: *mut MobileBroadbandModemStatus) -> ::windows_core::HRESULT,
    pub GetIsPassthroughEnabledWithSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModemConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModemConfiguration {
    type Vtable = IMobileBroadbandModemConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModemConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfce035a3_d6cd_4320_b982_be9d3ec7890f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uicc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HomeProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HomeProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModemConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModemConfiguration2 {
    type Vtable = IMobileBroadbandModemConfiguration2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModemConfiguration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x320ff5c5_e460_42ae_aa51_69621e7a4477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SarManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModemIsolation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModemIsolation {
    type Vtable = IMobileBroadbandModemIsolation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModemIsolation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5618fec_e661_4330_9bb4_3480212ec354);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemIsolation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddAllowedHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, host: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddAllowedHostRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, first: *mut ::core::ffi::c_void, last: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplyConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplyConfigurationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearConfigurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModemIsolationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModemIsolationFactory {
    type Vtable = IMobileBroadbandModemIsolationFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModemIsolationFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21d7ec58_c2b1_4c2f_a030_72820a24ecd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemIsolationFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modemdeviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, rulegroupid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandModemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandModemStatics {
    type Vtable = IMobileBroadbandModemStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandModemStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf99ed637_d6f1_4a78_8cbc_6421a65063c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandModemStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandNetwork(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandNetwork {
    type Vtable = IMobileBroadbandNetwork_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandNetwork {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb63928c_0309_4cb6_a8c1_6a5a3c8e1ff6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    pub NetworkRegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkRegistrationState) -> ::windows_core::HRESULT,
    pub RegistrationNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PacketAttachNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ActivationNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RegisteredDataClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows_core::HRESULT,
    pub RegisteredProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RegisteredProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ShowConnectionUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandNetwork2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandNetwork2 {
    type Vtable = IMobileBroadbandNetwork2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandNetwork2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a55db22_62f7_4bdd_ba1d_477441960ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetVoiceCallSupportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVoiceCallSupportAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RegistrationUiccApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegistrationUiccApps: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandNetwork3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandNetwork3 {
    type Vtable = IMobileBroadbandNetwork3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandNetwork3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33670a8a_c7ef_444c_ab6c_df7ef7a390fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetwork3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCellsInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCellsInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandNetworkRegistrationStateChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandNetworkRegistrationStateChange {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandNetworkRegistrationStateChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbeaf94e1_960f_49b4_a08d_7d85e968c7ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Network: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89135cff_28b8_46aa_b137_1c4b0f21edfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NetworkRegistrationStateChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NetworkRegistrationStateChanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandPco(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandPco {
    type Vtable = IMobileBroadbandPco_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandPco {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4e4fcbe_e3a3_43c5_a87b_6c86d229d7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPco_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub IsComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandPcoDataChangeTriggerDetails {
    type Vtable = IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandPcoDataChangeTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x263f5114_64e0_4493_909b_2d14a01962b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UpdatedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandPin(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandPin {
    type Vtable = IMobileBroadbandPin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandPin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe661d709_e779_45bf_8281_75323df9e321);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPin_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows_core::HRESULT,
    pub LockState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows_core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinFormat) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MinLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentpin: ::std::mem::MaybeUninit<::windows_core::HSTRING>, newpin: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnblockAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinunblockkey: ::std::mem::MaybeUninit<::windows_core::HSTRING>, newpin: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnblockAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandPinLockStateChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandPinLockStateChange {
    type Vtable = IMobileBroadbandPinLockStateChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandPinLockStateChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe16673e_1f04_4f95_8b90_e7f559dde7e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PinType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows_core::HRESULT,
    pub PinLockState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandPinLockStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandPinLockStateChangeTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd338c091_3e91_4d38_9036_aee83a6e79ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PinLockStateChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PinLockStateChanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandPinManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandPinManager {
    type Vtable = IMobileBroadbandPinManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandPinManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83567edd_6e1f_4b9b_a413_2b1f50cc36df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPins: usize,
    pub GetPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintype: MobileBroadbandPinType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandPinOperationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandPinOperationResult {
    type Vtable = IMobileBroadbandPinOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandPinOperationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11dddc32_31e7_49f5_b663_123d3bef0362);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandPinOperationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSuccessful: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AttemptsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandRadioStateChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandRadioStateChange {
    type Vtable = IMobileBroadbandRadioStateChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandRadioStateChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb054a561_9833_4aed_9717_4348b21a24b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RadioState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandRadioStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandRadioStateChangeTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71301ace_093c_42c6_b0db_ad1f75a65445);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RadioStateChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RadioStateChanges: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandSarManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandSarManager {
    type Vtable = IMobileBroadbandSarManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandSarManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5b26833_967e_40c9_a485_19c0dd209e22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSarManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsBackoffEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsWiFiHardwareIntegrated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSarControlledByHardware: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Antennas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Antennas: usize,
    #[cfg(feature = "Foundation")]
    pub HysteresisTimerPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HysteresisTimerPeriod: usize,
    #[cfg(feature = "Foundation")]
    pub TransmissionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransmissionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransmissionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransmissionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub EnableBackoffAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableBackoffAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableBackoffAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableBackoffAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, antennas: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetConfigurationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RevertSarToHardwareControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RevertSarToHardwareControlAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetTransmissionStateChangedHysteresisAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timerperiod: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTransmissionStateChangedHysteresisAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsTransmittingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsTransmittingAsync: usize,
    pub StartTransmissionStateMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopTransmissionStateMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandSlotInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandSlotInfo {
    type Vtable = IMobileBroadbandSlotInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandSlotInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd350b32_882e_542a_b17d_0bb1b49bae9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandSlotState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandSlotInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandSlotInfo2 {
    type Vtable = IMobileBroadbandSlotInfo2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandSlotInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x393cb039_ca44_524c_822d_83a3620f0efc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IccId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandSlotInfoChangedEventArgs {
    type Vtable = IMobileBroadbandSlotInfoChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandSlotInfoChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3158839f_950c_54ce_a48d_ba4529b48f0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotInfoChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SlotInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandSlotManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandSlotManager {
    type Vtable = IMobileBroadbandSlotManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandSlotManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba07cd6_2019_5f81_a294_cc364a11d0b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandSlotManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SlotInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SlotInfos: usize,
    pub CurrentSlotIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetCurrentSlot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut MobileBroadbandModemStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetCurrentSlotAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCurrentSlotAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SlotInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SlotInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSlotInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSlotInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentSlotIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentSlotIndexChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentSlotIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentSlotIndexChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandTransmissionStateChangedEventArgs {
    type Vtable = IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandTransmissionStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x612e3875_040a_4f99_a4f9_61d7c32da129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsTransmitting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandUicc(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandUicc {
    type Vtable = IMobileBroadbandUicc_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandUicc {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe634f691_525a_4ce2_8fce_aa4162579154);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUicc_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SimIccId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetUiccAppsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUiccAppsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandUiccApp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandUiccApp {
    type Vtable = IMobileBroadbandUiccApp_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandUiccApp {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d170556_98a1_43dd_b2ec_50c90cf248df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccApp_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAppKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecordDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiccfilepath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecordDetailsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiccfilepath: *mut ::core::ffi::c_void, recordindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadRecordAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandUiccAppReadRecordResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandUiccAppReadRecordResult {
    type Vtable = IMobileBroadbandUiccAppReadRecordResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandUiccAppReadRecordResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64c95285_358e_47c5_8249_695f383b2bdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppReadRecordResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandUiccAppRecordDetailsResult {
    type Vtable = IMobileBroadbandUiccAppRecordDetailsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandUiccAppRecordDetailsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd919682f_be14_4934_981d_2f57b9ed83e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppRecordDetailsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAppRecordKind) -> ::windows_core::HRESULT,
    pub RecordCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub RecordSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ReadAccessCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows_core::HRESULT,
    pub WriteAccessCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMobileBroadbandUiccAppsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMobileBroadbandUiccAppsResult {
    type Vtable = IMobileBroadbandUiccAppsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMobileBroadbandUiccAppsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x744930eb_8157_4a41_8494_6bf54c9b1d2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMobileBroadbandUiccAppsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UiccApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UiccApps: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorDataUsageTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorDataUsageTriggerDetails {
    type Vtable = INetworkOperatorDataUsageTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorDataUsageTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50e3126d_a465_4eeb_9317_28a167630cea);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorDataUsageTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NotificationKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorDataUsageNotificationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorNotificationEventDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorNotificationEventDetails {
    type Vtable = INetworkOperatorNotificationEventDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorNotificationEventDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc68a9d1_82e1_4488_9f2c_1276c2468fac);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationEventDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NotificationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorEventMessageType) -> ::windows_core::HRESULT,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EncodingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RuleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Sms")]
    pub SmsMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    SmsMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringAccessPointConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringAccessPointConfiguration {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringAccessPointConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bcc0284_412e_403d_acc6_b757e34774a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Ssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Passphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPassphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringAccessPointConfiguration2 {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringAccessPointConfiguration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1809142_7238_59a0_928b_74ab46fd64b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringAccessPointConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsBandSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsBandSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBandSupportedAsync: usize,
    pub Band: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TetheringWiFiBand) -> ::windows_core::HRESULT,
    pub SetBand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TetheringWiFiBand) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringClient(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringClient {
    type Vtable = INetworkOperatorTetheringClient_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringClient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x709d254c_595f_4847_bb30_646935542918);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClient_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MacAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HostNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringClientManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringClientManager {
    type Vtable = INetworkOperatorTetheringClientManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringClientManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91b14016_8dca_4225_bbed_eef8b8d718d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringClientManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTetheringClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTetheringClients: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringEntitlementCheck(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringEntitlementCheck {
    type Vtable = INetworkOperatorTetheringEntitlementCheck_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringEntitlementCheck {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0108916d_9e9a_4af6_8da3_60493b19c204);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringEntitlementCheck_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AuthorizeTethering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: bool, entitlementfailurereason: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringManager {
    type Vtable = INetworkOperatorTetheringManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd45a8da0_0e86_4d98_8ba4_dd70d4b764d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub ClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub TetheringOperationalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationalState) -> ::windows_core::HRESULT,
    pub GetCurrentAccessPointConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConfigureAccessPointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfigureAccessPointAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartTetheringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTetheringAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopTetheringAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopTetheringAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringManagerStatics {
    type Vtable = INetworkOperatorTetheringManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ebcbacc_f8c3_405c_9964_70a1eeabe194);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetTetheringCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut TetheringCapability) -> ::windows_core::HRESULT,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringManagerStatics2 {
    type Vtable = INetworkOperatorTetheringManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b235412_35f0_49e7_9b08_16d278fbaa42);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub GetTetheringCapabilityFromConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut TetheringCapability) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    GetTetheringCapabilityFromConnectionProfile: usize,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringManagerStatics3 {
    type Vtable = INetworkOperatorTetheringManagerStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fdaadb6_4af9_4f21_9b58_d53e9f24231e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub CreateFromConnectionProfileWithTargetAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    CreateFromConnectionProfileWithTargetAdapter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringManagerStatics4 {
    type Vtable = INetworkOperatorTetheringManagerStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringManagerStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3b9f9d0_ebff_46a4_a847_d663d8b0977e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsNoConnectionsTimeoutEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnableNoConnectionsTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableNoConnectionsTimeoutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableNoConnectionsTimeoutAsync: usize,
    pub DisableNoConnectionsTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisableNoConnectionsTimeoutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableNoConnectionsTimeoutAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorTetheringOperationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorTetheringOperationResult {
    type Vtable = INetworkOperatorTetheringOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorTetheringOperationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebd203a1_01ba_476d_b4b3_bf3d12c8f80c);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorTetheringOperationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationStatus) -> ::windows_core::HRESULT,
    pub AdditionalErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProvisionFromXmlDocumentResults(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProvisionFromXmlDocumentResults {
    type Vtable = IProvisionFromXmlDocumentResults_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProvisionFromXmlDocumentResults {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x217700e0_8203_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisionFromXmlDocumentResults_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllElementsProvisioned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ProvisionResultsXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProvisionedProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProvisionedProfile {
    type Vtable = IProvisionedProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProvisionedProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x217700e0_8202_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisionedProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub UpdateCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Connectivity::NetworkCostType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    UpdateCost: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProfileUsage) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateUsage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProvisioningAgent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProvisioningAgent {
    type Vtable = IProvisioningAgent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProvisioningAgent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x217700e0_8201_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningAgent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ProvisionFromXmlDocumentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provisioningxmldocument: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProvisionFromXmlDocumentAsync: usize,
    pub GetProvisionedProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediatype: ProfileMediaType, profilename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProvisioningAgentStaticMethods(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProvisioningAgentStaticMethods {
    type Vtable = IProvisioningAgentStaticMethods_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProvisioningAgentStaticMethods {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x217700e0_8101_11df_adb9_f4ce462d9137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvisioningAgentStaticMethods_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITetheringEntitlementCheckTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITetheringEntitlementCheckTriggerDetails {
    type Vtable = ITetheringEntitlementCheckTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITetheringEntitlementCheckTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03c65e9d_5926_41f3_a94e_b50926fc421b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITetheringEntitlementCheckTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowTethering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DenyTethering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entitlementfailurereason: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUssdMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUssdMessage {
    type Vtable = IUssdMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUssdMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f9acf82_2004_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DataCodingScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetDataCodingScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub GetPayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetPayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub PayloadAsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPayloadAsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUssdMessageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUssdMessageFactory {
    type Vtable = IUssdMessageFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUssdMessageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f9acf82_1003_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdMessageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUssdReply(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUssdReply {
    type Vtable = IUssdReply_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUssdReply {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f9acf82_2005_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdReply_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UssdResultCode) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUssdSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUssdSession {
    type Vtable = IUssdSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUssdSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f9acf82_2002_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SendMessageAndGetReplyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAndGetReplyAsync: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUssdSessionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUssdSessionStatics {
    type Vtable = IUssdSessionStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUssdSessionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f9acf82_1001_4d5d_bf81_2aba1b4be4a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUssdSessionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromNetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromNetworkInterfaceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkinterfaceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESim(::windows_core::IUnknown);
impl ESim {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AvailableMemoryInBytes(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AvailableMemoryInBytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Eid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Eid)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirmwareVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MobileBroadbandModemDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MobileBroadbandModemDeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Policy(&self) -> ::windows_core::Result<ESimPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Policy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<ESimState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProfiles(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ESimProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProfiles)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteProfileAsync(&self, profileid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteProfileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(profileid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadProfileMetadataAsync(&self, activationcode: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadProfileMetadataAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activationcode), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ResetAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResetAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProfileChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ESim, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProfileChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProfileChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Discover(&self) -> ::windows_core::Result<ESimDiscoverResult> {
        let this = &::windows_core::ComInterface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Discover)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DiscoverWithServerAddressAndMatchingId(&self, serveraddress: &::windows_core::HSTRING, matchingid: &::windows_core::HSTRING) -> ::windows_core::Result<ESimDiscoverResult> {
        let this = &::windows_core::ComInterface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DiscoverWithServerAddressAndMatchingId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(serveraddress), ::core::mem::transmute_copy(matchingid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DiscoverAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &::windows_core::ComInterface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DiscoverAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DiscoverWithServerAddressAndMatchingIdAsync(&self, serveraddress: &::windows_core::HSTRING, matchingid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>> {
        let this = &::windows_core::ComInterface::cast::<IESim2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DiscoverWithServerAddressAndMatchingIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(serveraddress), ::core::mem::transmute_copy(matchingid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SlotIndex(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = &::windows_core::ComInterface::cast::<IESim3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlotIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESim {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESim {
    type Vtable = IESim_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESim {
    const IID: ::windows_core::GUID = <IESim as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESim {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESim";
}
::windows_core::imp::interface_hierarchy!(ESim, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESim {}
unsafe impl ::core::marker::Sync for ESim {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimAddedEventArgs(::windows_core::IUnknown);
impl ESimAddedEventArgs {
    pub fn ESim(&self) -> ::windows_core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ESim)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimAddedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimAddedEventArgs {
    type Vtable = IESimAddedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimAddedEventArgs {
    const IID: ::windows_core::GUID = <IESimAddedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimAddedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimAddedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ESimAddedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimAddedEventArgs {}
unsafe impl ::core::marker::Sync for ESimAddedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimDiscoverEvent(::windows_core::IUnknown);
impl ESimDiscoverEvent {
    pub fn MatchingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MatchingId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RspServerAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RspServerAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimDiscoverEvent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimDiscoverEvent {
    type Vtable = IESimDiscoverEvent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimDiscoverEvent {
    const IID: ::windows_core::GUID = <IESimDiscoverEvent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimDiscoverEvent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverEvent";
}
::windows_core::imp::interface_hierarchy!(ESimDiscoverEvent, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimDiscoverEvent {}
unsafe impl ::core::marker::Sync for ESimDiscoverEvent {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimDiscoverResult(::windows_core::IUnknown);
impl ESimDiscoverResult {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Events(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Events)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ESimDiscoverResultKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProfileMetadata(&self) -> ::windows_core::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileMetadata)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Result(&self) -> ::windows_core::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimDiscoverResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimDiscoverResult {
    type Vtable = IESimDiscoverResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimDiscoverResult {
    const IID: ::windows_core::GUID = <IESimDiscoverResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimDiscoverResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDiscoverResult";
}
::windows_core::imp::interface_hierarchy!(ESimDiscoverResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimDiscoverResult {}
unsafe impl ::core::marker::Sync for ESimDiscoverResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimDownloadProfileMetadataResult(::windows_core::IUnknown);
impl ESimDownloadProfileMetadataResult {
    pub fn Result(&self) -> ::windows_core::Result<ESimOperationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProfileMetadata(&self) -> ::windows_core::Result<ESimProfileMetadata> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileMetadata)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimDownloadProfileMetadataResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimDownloadProfileMetadataResult {
    type Vtable = IESimDownloadProfileMetadataResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimDownloadProfileMetadataResult {
    const IID: ::windows_core::GUID = <IESimDownloadProfileMetadataResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimDownloadProfileMetadataResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimDownloadProfileMetadataResult";
}
::windows_core::imp::interface_hierarchy!(ESimDownloadProfileMetadataResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimDownloadProfileMetadataResult {}
unsafe impl ::core::marker::Sync for ESimDownloadProfileMetadataResult {}
pub struct ESimManager;
impl ESimManager {
    pub fn ServiceInfo() -> ::windows_core::Result<ESimServiceInfo> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TryCreateESimWatcher() -> ::windows_core::Result<ESimWatcher> {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateESimWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceInfoChanged<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IESimManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceInfoChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceInfoChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IESimManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveServiceInfoChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IESimManagerStatics<R, F: FnOnce(&IESimManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ESimManager, IESimManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ESimManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimOperationResult(::windows_core::IUnknown);
impl ESimOperationResult {
    pub fn Status(&self) -> ::windows_core::Result<ESimOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimOperationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimOperationResult {
    type Vtable = IESimOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimOperationResult {
    const IID: ::windows_core::GUID = <IESimOperationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimOperationResult";
}
::windows_core::imp::interface_hierarchy!(ESimOperationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimOperationResult {}
unsafe impl ::core::marker::Sync for ESimOperationResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimPolicy(::windows_core::IUnknown);
impl ESimPolicy {
    pub fn ShouldEnableManagingUi(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldEnableManagingUi)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimPolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimPolicy {
    type Vtable = IESimPolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimPolicy {
    const IID: ::windows_core::GUID = <IESimPolicy as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimPolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimPolicy";
}
::windows_core::imp::interface_hierarchy!(ESimPolicy, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimPolicy {}
unsafe impl ::core::marker::Sync for ESimPolicy {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimProfile(::windows_core::IUnknown);
impl ESimProfile {
    pub fn Class(&self) -> ::windows_core::Result<ESimProfileClass> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Class)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Nickname(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Nickname)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Policy(&self) -> ::windows_core::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Policy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderIcon)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<ESimProfileState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisableAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnableAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetNicknameAsync(&self, newnickname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetNicknameAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(newnickname), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimProfile {
    type Vtable = IESimProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimProfile {
    const IID: ::windows_core::GUID = <IESimProfile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfile";
}
::windows_core::imp::interface_hierarchy!(ESimProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimProfile {}
unsafe impl ::core::marker::Sync for ESimProfile {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimProfileMetadata(::windows_core::IUnknown);
impl ESimProfileMetadata {
    pub fn IsConfirmationCodeRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConfirmationCodeRequired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Policy(&self) -> ::windows_core::Result<ESimProfilePolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Policy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ProviderIcon(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderIcon)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<ESimProfileMetadataState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DenyInstallAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DenyInstallAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ConfirmInstallAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConfirmInstallAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ConfirmInstallWithConfirmationCodeAsync(&self, confirmationcode: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConfirmInstallWithConfirmationCodeAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(confirmationcode), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PostponeInstallAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PostponeInstallAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for ESimProfileMetadata {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimProfileMetadata {
    type Vtable = IESimProfileMetadata_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimProfileMetadata {
    const IID: ::windows_core::GUID = <IESimProfileMetadata as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimProfileMetadata {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfileMetadata";
}
::windows_core::imp::interface_hierarchy!(ESimProfileMetadata, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimProfileMetadata {}
unsafe impl ::core::marker::Sync for ESimProfileMetadata {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimProfilePolicy(::windows_core::IUnknown);
impl ESimProfilePolicy {
    pub fn CanDelete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanDelete)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanDisable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanDisable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsManagedByEnterprise(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsManagedByEnterprise)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimProfilePolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimProfilePolicy {
    type Vtable = IESimProfilePolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimProfilePolicy {
    const IID: ::windows_core::GUID = <IESimProfilePolicy as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimProfilePolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimProfilePolicy";
}
::windows_core::imp::interface_hierarchy!(ESimProfilePolicy, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimProfilePolicy {}
unsafe impl ::core::marker::Sync for ESimProfilePolicy {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimRemovedEventArgs(::windows_core::IUnknown);
impl ESimRemovedEventArgs {
    pub fn ESim(&self) -> ::windows_core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ESim)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimRemovedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimRemovedEventArgs {
    type Vtable = IESimRemovedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimRemovedEventArgs {
    const IID: ::windows_core::GUID = <IESimRemovedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimRemovedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimRemovedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ESimRemovedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimRemovedEventArgs {}
unsafe impl ::core::marker::Sync for ESimRemovedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimServiceInfo(::windows_core::IUnknown);
impl ESimServiceInfo {
    pub fn AuthenticationPreference(&self) -> ::windows_core::Result<ESimAuthenticationPreference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationPreference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsESimUiEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsESimUiEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimServiceInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimServiceInfo {
    type Vtable = IESimServiceInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimServiceInfo {
    const IID: ::windows_core::GUID = <IESimServiceInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimServiceInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimServiceInfo";
}
::windows_core::imp::interface_hierarchy!(ESimServiceInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimServiceInfo {}
unsafe impl ::core::marker::Sync for ESimServiceInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimUpdatedEventArgs(::windows_core::IUnknown);
impl ESimUpdatedEventArgs {
    pub fn ESim(&self) -> ::windows_core::Result<ESim> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ESim)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ESimUpdatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimUpdatedEventArgs {
    type Vtable = IESimUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimUpdatedEventArgs {
    const IID: ::windows_core::GUID = <IESimUpdatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimUpdatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ESimUpdatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for ESimUpdatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ESimWatcher(::windows_core::IUnknown);
impl ESimWatcher {
    pub fn Status(&self) -> ::windows_core::Result<ESimWatcherStatus> {
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
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for ESimWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ESimWatcher {
    type Vtable = IESimWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ESimWatcher {
    const IID: ::windows_core::GUID = <IESimWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ESimWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ESimWatcher";
}
::windows_core::imp::interface_hierarchy!(ESimWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ESimWatcher {}
unsafe impl ::core::marker::Sync for ESimWatcher {}
pub struct FdnAccessManager;
impl FdnAccessManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestUnlockAsync(contactlistid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IFdnAccessManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestUnlockAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contactlistid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFdnAccessManagerStatics<R, F: FnOnce(&IFdnAccessManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FdnAccessManager, IFdnAccessManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for FdnAccessManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.FdnAccessManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HotspotAuthenticationContext(::windows_core::IUnknown);
impl HotspotAuthenticationContext {
    pub fn WirelessNetworkId(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).WirelessNetworkId)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Networking_Connectivity\"`"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows_core::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAdapter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RedirectMessageUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RedirectMessageUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Data_Xml_Dom\"`"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn RedirectMessageXml(&self) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RedirectMessageXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticationUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IssueCredentials(&self, username: &::windows_core::HSTRING, password: &::windows_core::HSTRING, extraparameters: &::windows_core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).IssueCredentials)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), ::core::mem::transmute_copy(password), ::core::mem::transmute_copy(extraparameters), markasmanualconnectonfailure).ok() }
    }
    pub fn AbortAuthentication(&self, markasmanual: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AbortAuthentication)(::windows_core::Interface::as_raw(this), markasmanual).ok() }
    }
    pub fn SkipAuthentication(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SkipAuthentication)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TriggerAttentionRequired(&self, packagerelativeapplicationid: &::windows_core::HSTRING, applicationparameters: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).TriggerAttentionRequired)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagerelativeapplicationid), ::core::mem::transmute_copy(applicationparameters)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IssueCredentialsAsync(&self, username: &::windows_core::HSTRING, password: &::windows_core::HSTRING, extraparameters: &::windows_core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>> {
        let this = &::windows_core::ComInterface::cast::<IHotspotAuthenticationContext2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IssueCredentialsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), ::core::mem::transmute_copy(password), ::core::mem::transmute_copy(extraparameters), markasmanualconnectonfailure, &mut result__).from_abi(result__)
        }
    }
    pub fn TryGetAuthenticationContext(eventoken: &::windows_core::HSTRING, context: &mut ::core::option::Option<HotspotAuthenticationContext>) -> ::windows_core::Result<bool> {
        Self::IHotspotAuthenticationContextStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetAuthenticationContext)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(eventoken), context as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHotspotAuthenticationContextStatics<R, F: FnOnce(&IHotspotAuthenticationContextStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HotspotAuthenticationContext, IHotspotAuthenticationContextStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for HotspotAuthenticationContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HotspotAuthenticationContext {
    type Vtable = IHotspotAuthenticationContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HotspotAuthenticationContext {
    const IID: ::windows_core::GUID = <IHotspotAuthenticationContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HotspotAuthenticationContext {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationContext";
}
::windows_core::imp::interface_hierarchy!(HotspotAuthenticationContext, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HotspotAuthenticationEventDetails(::windows_core::IUnknown);
impl HotspotAuthenticationEventDetails {
    pub fn EventToken(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EventToken)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for HotspotAuthenticationEventDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HotspotAuthenticationEventDetails {
    type Vtable = IHotspotAuthenticationEventDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HotspotAuthenticationEventDetails {
    const IID: ::windows_core::GUID = <IHotspotAuthenticationEventDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HotspotAuthenticationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotAuthenticationEventDetails";
}
::windows_core::imp::interface_hierarchy!(HotspotAuthenticationEventDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HotspotCredentialsAuthenticationResult(::windows_core::IUnknown);
impl HotspotCredentialsAuthenticationResult {
    pub fn HasNetworkErrorOccurred(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasNetworkErrorOccurred)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseCode(&self) -> ::windows_core::Result<HotspotAuthenticationResponseCode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LogoffUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LogoffUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Data_Xml_Dom\"`"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn AuthenticationReplyXml(&self) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationReplyXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for HotspotCredentialsAuthenticationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HotspotCredentialsAuthenticationResult {
    type Vtable = IHotspotCredentialsAuthenticationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HotspotCredentialsAuthenticationResult {
    const IID: ::windows_core::GUID = <IHotspotCredentialsAuthenticationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HotspotCredentialsAuthenticationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.HotspotCredentialsAuthenticationResult";
}
::windows_core::imp::interface_hierarchy!(HotspotCredentialsAuthenticationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
pub struct KnownCSimFilePaths;
impl KnownCSimFilePaths {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EFSpn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownCSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownCSimFilePathsStatics<R, F: FnOnce(&IKnownCSimFilePathsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownCSimFilePaths, IKnownCSimFilePathsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownCSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownCSimFilePaths";
}
pub struct KnownRuimFilePaths;
impl KnownRuimFilePaths {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EFSpn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownRuimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownRuimFilePathsStatics<R, F: FnOnce(&IKnownRuimFilePathsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownRuimFilePaths, IKnownRuimFilePathsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownRuimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownRuimFilePaths";
}
pub struct KnownSimFilePaths;
impl KnownSimFilePaths {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOns() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EFOns)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EFSpn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownSimFilePathsStatics<R, F: FnOnce(&IKnownSimFilePathsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownSimFilePaths, IKnownSimFilePathsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownSimFilePaths";
}
pub struct KnownUSimFilePaths;
impl KnownUSimFilePaths {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFSpn() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EFSpn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFOpl() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EFOpl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EFPnn() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EFPnn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid1() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gid2() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        Self::IKnownUSimFilePathsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gid2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownUSimFilePathsStatics<R, F: FnOnce(&IKnownUSimFilePathsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownUSimFilePaths, IKnownUSimFilePathsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownUSimFilePaths {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.KnownUSimFilePaths";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandAccount(::windows_core::IUnknown);
impl MobileBroadbandAccount {
    pub fn NetworkAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceProviderGuid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceProviderGuid)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceProviderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceProviderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentNetwork(&self) -> ::windows_core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentNetwork)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentDeviceInformation(&self) -> ::windows_core::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentDeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Networking_Connectivity\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity"))]
    pub fn GetConnectionProfiles(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandAccount2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectionProfiles)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AccountExperienceUrl(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandAccount3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountExperienceUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableNetworkAccountIds() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AvailableNetworkAccountIds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<MobileBroadbandAccount> {
        Self::IMobileBroadbandAccountStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandAccountStatics<R, F: FnOnce(&IMobileBroadbandAccountStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandAccount, IMobileBroadbandAccountStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandAccount {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandAccount {
    type Vtable = IMobileBroadbandAccount_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandAccount {
    const IID: ::windows_core::GUID = <IMobileBroadbandAccount as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandAccount {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccount";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandAccount, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandAccountEventArgs(::windows_core::IUnknown);
impl MobileBroadbandAccountEventArgs {
    pub fn NetworkAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandAccountEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandAccountEventArgs {
    type Vtable = IMobileBroadbandAccountEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandAccountEventArgs {
    const IID: ::windows_core::GUID = <IMobileBroadbandAccountEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandAccountEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountEventArgs";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandAccountEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandAccountUpdatedEventArgs(::windows_core::IUnknown);
impl MobileBroadbandAccountUpdatedEventArgs {
    pub fn NetworkAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasDeviceInformationChanged(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasDeviceInformationChanged)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasNetworkChanged(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasNetworkChanged)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandAccountUpdatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandAccountUpdatedEventArgs {
    type Vtable = IMobileBroadbandAccountUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandAccountUpdatedEventArgs {
    const IID: ::windows_core::GUID = <IMobileBroadbandAccountUpdatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandAccountUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountUpdatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandAccountUpdatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandAccountWatcher(::windows_core::IUnknown);
impl MobileBroadbandAccountWatcher {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandAccountWatcher, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AccountAdded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountAdded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAccountAdded)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AccountUpdated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountUpdated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountUpdated(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAccountUpdated)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AccountRemoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccountRemoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAccountRemoved)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<MobileBroadbandAccountWatcherStatus> {
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
impl ::windows_core::RuntimeType for MobileBroadbandAccountWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandAccountWatcher {
    type Vtable = IMobileBroadbandAccountWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandAccountWatcher {
    const IID: ::windows_core::GUID = <IMobileBroadbandAccountWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandAccountWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcher";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandAccountWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandAntennaSar(::windows_core::IUnknown);
impl MobileBroadbandAntennaSar {
    pub fn AntennaIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AntennaIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SarBackoffIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SarBackoffIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWithIndex(antennaindex: i32, sarbackoffindex: i32) -> ::windows_core::Result<MobileBroadbandAntennaSar> {
        Self::IMobileBroadbandAntennaSarFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithIndex)(::windows_core::Interface::as_raw(this), antennaindex, sarbackoffindex, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandAntennaSarFactory<R, F: FnOnce(&IMobileBroadbandAntennaSarFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandAntennaSar, IMobileBroadbandAntennaSarFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandAntennaSar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandAntennaSar {
    type Vtable = IMobileBroadbandAntennaSar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandAntennaSar {
    const IID: ::windows_core::GUID = <IMobileBroadbandAntennaSar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandAntennaSar {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandAntennaSar";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandAntennaSar, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandAntennaSar {}
unsafe impl ::core::marker::Sync for MobileBroadbandAntennaSar {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCellCdma(::windows_core::IUnknown);
impl MobileBroadbandCellCdma {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseStationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationPNCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseStationPNCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLatitude(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseStationLatitude)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLongitude(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseStationLongitude)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationLastBroadcastGpsTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseStationLastBroadcastGpsTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NetworkId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PilotSignalStrengthInDB(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PilotSignalStrengthInDB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SystemId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCellCdma {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCellCdma {
    type Vtable = IMobileBroadbandCellCdma_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCellCdma {
    const IID: ::windows_core::GUID = <IMobileBroadbandCellCdma as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCellCdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellCdma";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCellCdma, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCellCdma {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellCdma {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCellGsm(::windows_core::IUnknown);
impl MobileBroadbandCellGsm {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BaseStationId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseStationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocationAreaCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalStrengthInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReceivedSignalStrengthInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimingAdvanceInBitPeriods)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCellGsm {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCellGsm {
    type Vtable = IMobileBroadbandCellGsm_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCellGsm {
    const IID: ::windows_core::GUID = <IMobileBroadbandCellGsm as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCellGsm {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellGsm";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCellGsm, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCellGsm {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellGsm {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCellLte(::windows_core::IUnknown);
impl MobileBroadbandCellLte {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PhysicalCellId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalCellId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReferenceSignalReceivedPowerInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReferenceSignalReceivedQualityInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimingAdvanceInBitPeriods)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TrackingAreaCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackingAreaCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCellLte {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCellLte {
    type Vtable = IMobileBroadbandCellLte_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCellLte {
    const IID: ::windows_core::GUID = <IMobileBroadbandCellLte as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCellLte {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellLte";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCellLte, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCellLte {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellLte {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCellNR(::windows_core::IUnknown);
impl MobileBroadbandCellNR {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PhysicalCellId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalCellId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReferenceSignalReceivedPowerInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReferenceSignalReceivedQualityInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInNanoseconds(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimingAdvanceInNanoseconds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TrackingAreaCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackingAreaCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SignalToNoiseRatioInDB(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignalToNoiseRatioInDB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCellNR {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCellNR {
    type Vtable = IMobileBroadbandCellNR_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCellNR {
    const IID: ::windows_core::GUID = <IMobileBroadbandCellNR as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCellNR {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellNR";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCellNR, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCellNR {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellNR {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCellTdscdma(::windows_core::IUnknown);
impl MobileBroadbandCellTdscdma {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CellParameterId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellParameterId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocationAreaCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PathLossInDB(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PathLossInDB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalCodePowerInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReceivedSignalCodePowerInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimingAdvanceInBitPeriods(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimingAdvanceInBitPeriods)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCellTdscdma {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCellTdscdma {
    type Vtable = IMobileBroadbandCellTdscdma_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCellTdscdma {
    const IID: ::windows_core::GUID = <IMobileBroadbandCellTdscdma as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCellTdscdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellTdscdma";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCellTdscdma, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCellTdscdma {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellTdscdma {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCellUmts(::windows_core::IUnknown);
impl MobileBroadbandCellUmts {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CellId(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ChannelNumber(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LocationAreaCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocationAreaCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PathLossInDB(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PathLossInDB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PrimaryScramblingCode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryScramblingCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReceivedSignalCodePowerInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReceivedSignalCodePowerInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SignalToNoiseRatioInDB(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignalToNoiseRatioInDB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCellUmts {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCellUmts {
    type Vtable = IMobileBroadbandCellUmts_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCellUmts {
    const IID: ::windows_core::GUID = <IMobileBroadbandCellUmts as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCellUmts {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellUmts";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCellUmts, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCellUmts {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellUmts {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCellsInfo(::windows_core::IUnknown);
impl MobileBroadbandCellsInfo {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsCdma(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringCellsCdma)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsGsm(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringCellsGsm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsLte(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringCellsLte)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsTdscdma(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringCellsTdscdma)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsUmts(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringCellsUmts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsCdma(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServingCellsCdma)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsGsm(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServingCellsGsm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsLte(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServingCellsLte)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsTdscdma(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServingCellsTdscdma)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsUmts(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServingCellsUmts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NeighboringCellsNR(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringCellsNR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServingCellsNR(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandCellsInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServingCellsNR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCellsInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCellsInfo {
    type Vtable = IMobileBroadbandCellsInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCellsInfo {
    const IID: ::windows_core::GUID = <IMobileBroadbandCellsInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCellsInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCellsInfo";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCellsInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCellsInfo {}
unsafe impl ::core::marker::Sync for MobileBroadbandCellsInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandCurrentSlotIndexChangedEventArgs(::windows_core::IUnknown);
impl MobileBroadbandCurrentSlotIndexChangedEventArgs {
    pub fn CurrentSlotIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSlotIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    type Vtable = IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const IID: ::windows_core::GUID = <IMobileBroadbandCurrentSlotIndexChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandCurrentSlotIndexChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandCurrentSlotIndexChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceInformation(::windows_core::IUnknown);
impl MobileBroadbandDeviceInformation {
    pub fn NetworkDeviceStatus(&self) -> ::windows_core::Result<NetworkDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkDeviceStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Manufacturer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Manufacturer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Model(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Model)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirmwareInformation(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Sms\"`"]
    #[cfg(feature = "Devices_Sms")]
    pub fn CellularClass(&self) -> ::windows_core::Result<super::super::Devices::Sms::CellularClass> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CellularClass)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DataClasses(&self) -> ::windows_core::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataClasses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CustomDataClass(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomDataClass)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MobileEquipmentId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MobileEquipmentId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TelephoneNumbers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TelephoneNumbers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SubscriberId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubscriberId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows_core::Result<MobileBroadbandDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentRadioState(&self) -> ::windows_core::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentRadioState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PinManager(&self) -> ::windows_core::Result<MobileBroadbandPinManager> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SerialNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimSpn(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimSpn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimPnn(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimPnn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SimGid1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceInformation3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimGid1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SlotManager(&self) -> ::windows_core::Result<MobileBroadbandSlotManager> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceInformation4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlotManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceInformation {
    type Vtable = IMobileBroadbandDeviceInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceInformation {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceInformation";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceService(::windows_core::IUnknown);
impl MobileBroadbandDeviceService {
    pub fn DeviceServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCommands(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedCommands)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OpenDataSession(&self) -> ::windows_core::Result<MobileBroadbandDeviceServiceDataSession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenDataSession)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OpenCommandSession(&self) -> ::windows_core::Result<MobileBroadbandDeviceServiceCommandSession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenCommandSession)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceService {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceService {
    type Vtable = IMobileBroadbandDeviceService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceService {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceService as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceService {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceService";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceService, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandDeviceService {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceService {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceServiceCommandResult(::windows_core::IUnknown);
impl MobileBroadbandDeviceServiceCommandResult {
    pub fn StatusCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ResponseData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceServiceCommandResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceServiceCommandResult {
    type Vtable = IMobileBroadbandDeviceServiceCommandResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceServiceCommandResult {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceServiceCommandResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceServiceCommandResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandResult";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceCommandResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceCommandResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceCommandResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceServiceCommandSession(::windows_core::IUnknown);
impl MobileBroadbandDeviceServiceCommandSession {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendQueryCommandAsync<P0>(&self, commandid: u32, data: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendQueryCommandAsync)(::windows_core::Interface::as_raw(this), commandid, data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendSetCommandAsync<P0>(&self, commandid: u32, data: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendSetCommandAsync)(::windows_core::Interface::as_raw(this), commandid, data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloseSession(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CloseSession)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceServiceCommandSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceServiceCommandSession {
    type Vtable = IMobileBroadbandDeviceServiceCommandSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceServiceCommandSession {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceServiceCommandSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceServiceCommandSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceCommandSession";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceCommandSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceCommandSession {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceCommandSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceServiceDataReceivedEventArgs(::windows_core::IUnknown);
impl MobileBroadbandDeviceServiceDataReceivedEventArgs {
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReceivedData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    type Vtable = IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceServiceDataReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceDataReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceServiceDataSession(::windows_core::IUnknown);
impl MobileBroadbandDeviceServiceDataSession {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteDataAsync<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteDataAsync)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CloseSession(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CloseSession)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DataReceived<P0>(&self, eventhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataReceived)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDataReceived)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceServiceDataSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceServiceDataSession {
    type Vtable = IMobileBroadbandDeviceServiceDataSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceServiceDataSession {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceServiceDataSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceServiceDataSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceDataSession";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceDataSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceDataSession {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceDataSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceServiceInformation(::windows_core::IUnknown);
impl MobileBroadbandDeviceServiceInformation {
    pub fn DeviceServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDataReadSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDataReadSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDataWriteSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDataWriteSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceServiceInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceServiceInformation {
    type Vtable = IMobileBroadbandDeviceServiceInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceServiceInformation {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceServiceInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceServiceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceInformation";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceInformation {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceInformation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceServiceTriggerDetails(::windows_core::IUnknown);
impl MobileBroadbandDeviceServiceTriggerDetails {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ReceivedData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReceivedData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EventId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandDeviceServiceTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EventId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceServiceTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceServiceTriggerDetails {
    type Vtable = IMobileBroadbandDeviceServiceTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceServiceTriggerDetails {
    const IID: ::windows_core::GUID = <IMobileBroadbandDeviceServiceTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceServiceTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandDeviceServiceTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandModem(::windows_core::IUnknown);
impl MobileBroadbandModem {
    pub fn CurrentAccount(&self) -> ::windows_core::Result<MobileBroadbandAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentAccount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceInformation(&self) -> ::windows_core::Result<MobileBroadbandDeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxDeviceServiceCommandSizeInBytes(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxDeviceServiceCommandSizeInBytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxDeviceServiceDataSizeInBytes(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxDeviceServiceDataSizeInBytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceServices(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceServices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceService(&self, deviceserviceid: ::windows_core::GUID) -> ::windows_core::Result<MobileBroadbandDeviceService> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceService)(::windows_core::Interface::as_raw(this), deviceserviceid, &mut result__).from_abi(result__)
        }
    }
    pub fn IsResetSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsResetSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ResetAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResetAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentConfigurationAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentConfigurationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentNetwork(&self) -> ::windows_core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentNetwork)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsPassthroughEnabledAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPassthroughEnabledAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsPassthroughEnabledAsync(&self, value: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetIsPassthroughEnabledAsync)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetPcoAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPcoAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsInEmergencyCallMode(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInEmergencyCallMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsInEmergencyCallModeChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInEmergencyCallModeChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsInEmergencyCallModeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIsInEmergencyCallModeChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsPassthroughEnabledWithSlotIndexAsync(&self, value: bool, slotindex: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetIsPassthroughEnabledWithSlotIndexAsync)(::windows_core::Interface::as_raw(this), value, slotindex, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsPassthroughEnabledWithSlotIndexAsync(&self, slotindex: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPassthroughEnabledWithSlotIndexAsync)(::windows_core::Interface::as_raw(this), slotindex, &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPassthroughEnabledWithSlotIndex(&self, value: bool, slotindex: i32) -> ::windows_core::Result<MobileBroadbandModemStatus> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetIsPassthroughEnabledWithSlotIndex)(::windows_core::Interface::as_raw(this), value, slotindex, &mut result__).from_abi(result__)
        }
    }
    pub fn GetIsPassthroughEnabledWithSlotIndex(&self, slotindex: i32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModem4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPassthroughEnabledWithSlotIndex)(::windows_core::Interface::as_raw(this), slotindex, &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FromId(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDefault() -> ::windows_core::Result<MobileBroadbandModem> {
        Self::IMobileBroadbandModemStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandModemStatics<R, F: FnOnce(&IMobileBroadbandModemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandModem, IMobileBroadbandModemStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandModem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandModem {
    type Vtable = IMobileBroadbandModem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandModem {
    const IID: ::windows_core::GUID = <IMobileBroadbandModem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandModem {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModem";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandModem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandModem {}
unsafe impl ::core::marker::Sync for MobileBroadbandModem {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandModemConfiguration(::windows_core::IUnknown);
impl MobileBroadbandModemConfiguration {
    pub fn Uicc(&self) -> ::windows_core::Result<MobileBroadbandUicc> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uicc)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HomeProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HomeProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HomeProviderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HomeProviderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SarManager(&self) -> ::windows_core::Result<MobileBroadbandSarManager> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandModemConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SarManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandModemConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandModemConfiguration {
    type Vtable = IMobileBroadbandModemConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandModemConfiguration {
    const IID: ::windows_core::GUID = <IMobileBroadbandModemConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandModemConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemConfiguration";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandModemConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandModemIsolation(::windows_core::IUnknown);
impl MobileBroadbandModemIsolation {
    pub fn AddAllowedHost<P0>(&self, host: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::HostName>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddAllowedHost)(::windows_core::Interface::as_raw(this), host.into_param().abi()).ok() }
    }
    pub fn AddAllowedHostRange<P0, P1>(&self, first: P0, last: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::HostName>,
        P1: ::windows_core::IntoParam<super::HostName>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddAllowedHostRange)(::windows_core::Interface::as_raw(this), first.into_param().abi(), last.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ApplyConfigurationAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplyConfigurationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ClearConfigurationAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearConfigurationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(modemdeviceid: &::windows_core::HSTRING, rulegroupid: &::windows_core::HSTRING) -> ::windows_core::Result<MobileBroadbandModemIsolation> {
        Self::IMobileBroadbandModemIsolationFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(modemdeviceid), ::core::mem::transmute_copy(rulegroupid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMobileBroadbandModemIsolationFactory<R, F: FnOnce(&IMobileBroadbandModemIsolationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandModemIsolation, IMobileBroadbandModemIsolationFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandModemIsolation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandModemIsolation {
    type Vtable = IMobileBroadbandModemIsolation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandModemIsolation {
    const IID: ::windows_core::GUID = <IMobileBroadbandModemIsolation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandModemIsolation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandModemIsolation";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandModemIsolation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandModemIsolation {}
unsafe impl ::core::marker::Sync for MobileBroadbandModemIsolation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandNetwork(::windows_core::IUnknown);
impl MobileBroadbandNetwork {
    #[doc = "Required features: `\"Networking_Connectivity\"`"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows_core::Result<super::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAdapter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkRegistrationState(&self) -> ::windows_core::Result<NetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkRegistrationState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegistrationNetworkError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegistrationNetworkError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PacketAttachNetworkError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PacketAttachNetworkError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivationNetworkError(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivationNetworkError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccessPointName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessPointName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegisteredDataClass(&self) -> ::windows_core::Result<DataClasses> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisteredDataClass)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegisteredProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisteredProviderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegisteredProviderName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisteredProviderName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShowConnectionUI(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowConnectionUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetVoiceCallSupportAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVoiceCallSupportAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegistrationUiccApps(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandNetwork2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegistrationUiccApps)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetCellsInfoAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandNetwork3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCellsInfoAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandNetwork {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandNetwork {
    type Vtable = IMobileBroadbandNetwork_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandNetwork {
    const IID: ::windows_core::GUID = <IMobileBroadbandNetwork as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandNetwork {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetwork";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandNetwork, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandNetworkRegistrationStateChange(::windows_core::IUnknown);
impl MobileBroadbandNetworkRegistrationStateChange {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Network(&self) -> ::windows_core::Result<MobileBroadbandNetwork> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Network)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandNetworkRegistrationStateChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandNetworkRegistrationStateChange {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandNetworkRegistrationStateChange {
    const IID: ::windows_core::GUID = <IMobileBroadbandNetworkRegistrationStateChange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandNetworkRegistrationStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChange";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandNetworkRegistrationStateChange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandNetworkRegistrationStateChange {}
unsafe impl ::core::marker::Sync for MobileBroadbandNetworkRegistrationStateChange {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandNetworkRegistrationStateChangeTriggerDetails(::windows_core::IUnknown);
impl MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NetworkRegistrationStateChanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkRegistrationStateChanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const IID: ::windows_core::GUID = <IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandNetworkRegistrationStateChangeTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandNetworkRegistrationStateChangeTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPco(::windows_core::IUnknown);
impl MobileBroadbandPco {
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsComplete(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsComplete)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPco {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandPco {
    type Vtable = IMobileBroadbandPco_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPco {
    const IID: ::windows_core::GUID = <IMobileBroadbandPco as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPco {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPco";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPco, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandPco {}
unsafe impl ::core::marker::Sync for MobileBroadbandPco {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPcoDataChangeTriggerDetails(::windows_core::IUnknown);
impl MobileBroadbandPcoDataChangeTriggerDetails {
    pub fn UpdatedData(&self) -> ::windows_core::Result<MobileBroadbandPco> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdatedData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPcoDataChangeTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandPcoDataChangeTriggerDetails {
    type Vtable = IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPcoDataChangeTriggerDetails {
    const IID: ::windows_core::GUID = <IMobileBroadbandPcoDataChangeTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPcoDataChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPcoDataChangeTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPcoDataChangeTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandPcoDataChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandPcoDataChangeTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPin(::windows_core::IUnknown);
impl MobileBroadbandPin {
    pub fn Type(&self) -> ::windows_core::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LockState(&self) -> ::windows_core::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LockState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Format(&self) -> ::windows_core::Result<MobileBroadbandPinFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinLength)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttemptsRemaining(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttemptsRemaining)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self, currentpin: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnableAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(currentpin), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self, currentpin: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisableAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(currentpin), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnterAsync(&self, currentpin: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(currentpin), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ChangeAsync(&self, currentpin: &::windows_core::HSTRING, newpin: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChangeAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(currentpin), ::core::mem::transmute_copy(newpin), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UnblockAsync(&self, pinunblockkey: &::windows_core::HSTRING, newpin: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnblockAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(pinunblockkey), ::core::mem::transmute_copy(newpin), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPin {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandPin {
    type Vtable = IMobileBroadbandPin_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPin {
    const IID: ::windows_core::GUID = <IMobileBroadbandPin as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPin {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPin";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPin, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandPin {}
unsafe impl ::core::marker::Sync for MobileBroadbandPin {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPinLockStateChange(::windows_core::IUnknown);
impl MobileBroadbandPinLockStateChange {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PinType(&self) -> ::windows_core::Result<MobileBroadbandPinType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PinLockState(&self) -> ::windows_core::Result<MobileBroadbandPinLockState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinLockState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinLockStateChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandPinLockStateChange {
    type Vtable = IMobileBroadbandPinLockStateChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPinLockStateChange {
    const IID: ::windows_core::GUID = <IMobileBroadbandPinLockStateChange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPinLockStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChange";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPinLockStateChange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandPinLockStateChange {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinLockStateChange {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPinLockStateChangeTriggerDetails(::windows_core::IUnknown);
impl MobileBroadbandPinLockStateChangeTriggerDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PinLockStateChanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinLockStateChanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinLockStateChangeTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandPinLockStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPinLockStateChangeTriggerDetails {
    const IID: ::windows_core::GUID = <IMobileBroadbandPinLockStateChangeTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPinLockStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinLockStateChangeTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPinLockStateChangeTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandPinLockStateChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinLockStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPinManager(::windows_core::IUnknown);
impl MobileBroadbandPinManager {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPins(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPins)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPin(&self, pintype: MobileBroadbandPinType) -> ::windows_core::Result<MobileBroadbandPin> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPin)(::windows_core::Interface::as_raw(this), pintype, &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandPinManager {
    type Vtable = IMobileBroadbandPinManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPinManager {
    const IID: ::windows_core::GUID = <IMobileBroadbandPinManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPinManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinManager";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPinManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandPinManager {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPinOperationResult(::windows_core::IUnknown);
impl MobileBroadbandPinOperationResult {
    pub fn IsSuccessful(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSuccessful)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AttemptsRemaining(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttemptsRemaining)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinOperationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandPinOperationResult {
    type Vtable = IMobileBroadbandPinOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPinOperationResult {
    const IID: ::windows_core::GUID = <IMobileBroadbandPinOperationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPinOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandPinOperationResult";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPinOperationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandPinOperationResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinOperationResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandRadioStateChange(::windows_core::IUnknown);
impl MobileBroadbandRadioStateChange {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RadioState(&self) -> ::windows_core::Result<MobileBroadbandRadioState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadioState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandRadioStateChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandRadioStateChange {
    type Vtable = IMobileBroadbandRadioStateChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandRadioStateChange {
    const IID: ::windows_core::GUID = <IMobileBroadbandRadioStateChange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandRadioStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChange";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandRadioStateChange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandRadioStateChange {}
unsafe impl ::core::marker::Sync for MobileBroadbandRadioStateChange {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandRadioStateChangeTriggerDetails(::windows_core::IUnknown);
impl MobileBroadbandRadioStateChangeTriggerDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RadioStateChanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadioStateChanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandRadioStateChangeTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandRadioStateChangeTriggerDetails {
    type Vtable = IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandRadioStateChangeTriggerDetails {
    const IID: ::windows_core::GUID = <IMobileBroadbandRadioStateChangeTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandRadioStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandRadioStateChangeTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandRadioStateChangeTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandRadioStateChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for MobileBroadbandRadioStateChangeTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandSarManager(::windows_core::IUnknown);
impl MobileBroadbandSarManager {
    pub fn IsBackoffEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBackoffEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsWiFiHardwareIntegrated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsWiFiHardwareIntegrated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSarControlledByHardware(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSarControlledByHardware)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Antennas(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Antennas)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn HysteresisTimerPeriod(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HysteresisTimerPeriod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TransmissionStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransmissionStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransmissionStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTransmissionStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnableBackoffAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnableBackoffAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DisableBackoffAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisableBackoffAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetConfigurationAsync<P0>(&self, antennas: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetConfigurationAsync)(::windows_core::Interface::as_raw(this), antennas.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RevertSarToHardwareControlAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RevertSarToHardwareControlAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetTransmissionStateChangedHysteresisAsync(&self, timerperiod: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTransmissionStateChangedHysteresisAsync)(::windows_core::Interface::as_raw(this), timerperiod, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsTransmittingAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsTransmittingAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartTransmissionStateMonitoring(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartTransmissionStateMonitoring)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopTransmissionStateMonitoring(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopTransmissionStateMonitoring)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandSarManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandSarManager {
    type Vtable = IMobileBroadbandSarManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandSarManager {
    const IID: ::windows_core::GUID = <IMobileBroadbandSarManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandSarManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSarManager";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandSarManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandSarManager {}
unsafe impl ::core::marker::Sync for MobileBroadbandSarManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandSlotInfo(::windows_core::IUnknown);
impl MobileBroadbandSlotInfo {
    pub fn Index(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Index)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<MobileBroadbandSlotState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMobileBroadbandSlotInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IccId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandSlotInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandSlotInfo {
    type Vtable = IMobileBroadbandSlotInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandSlotInfo {
    const IID: ::windows_core::GUID = <IMobileBroadbandSlotInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandSlotInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfo";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandSlotInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandSlotInfo {}
unsafe impl ::core::marker::Sync for MobileBroadbandSlotInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandSlotInfoChangedEventArgs(::windows_core::IUnknown);
impl MobileBroadbandSlotInfoChangedEventArgs {
    pub fn SlotInfo(&self) -> ::windows_core::Result<MobileBroadbandSlotInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlotInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandSlotInfoChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandSlotInfoChangedEventArgs {
    type Vtable = IMobileBroadbandSlotInfoChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandSlotInfoChangedEventArgs {
    const IID: ::windows_core::GUID = <IMobileBroadbandSlotInfoChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandSlotInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotInfoChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandSlotInfoChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandSlotInfoChangedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandSlotInfoChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandSlotManager(::windows_core::IUnknown);
impl MobileBroadbandSlotManager {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SlotInfos(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlotInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentSlotIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSlotIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCurrentSlot(&self, slotindex: i32) -> ::windows_core::Result<MobileBroadbandModemStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetCurrentSlot)(::windows_core::Interface::as_raw(this), slotindex, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetCurrentSlotAsync(&self, slotindex: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetCurrentSlotAsync)(::windows_core::Interface::as_raw(this), slotindex, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SlotInfoChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlotInfoChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSlotInfoChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSlotInfoChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentSlotIndexChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSlotIndexChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentSlotIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentSlotIndexChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandSlotManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandSlotManager {
    type Vtable = IMobileBroadbandSlotManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandSlotManager {
    const IID: ::windows_core::GUID = <IMobileBroadbandSlotManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandSlotManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandSlotManager";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandSlotManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandSlotManager {}
unsafe impl ::core::marker::Sync for MobileBroadbandSlotManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandTransmissionStateChangedEventArgs(::windows_core::IUnknown);
impl MobileBroadbandTransmissionStateChangedEventArgs {
    pub fn IsTransmitting(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTransmitting)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandTransmissionStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandTransmissionStateChangedEventArgs {
    type Vtable = IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandTransmissionStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IMobileBroadbandTransmissionStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandTransmissionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandTransmissionStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandTransmissionStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandTransmissionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MobileBroadbandTransmissionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandUicc(::windows_core::IUnknown);
impl MobileBroadbandUicc {
    pub fn SimIccId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimIccId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetUiccAppsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUiccAppsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandUicc {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandUicc {
    type Vtable = IMobileBroadbandUicc_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandUicc {
    const IID: ::windows_core::GUID = <IMobileBroadbandUicc as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandUicc {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUicc";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandUicc, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandUicc {}
unsafe impl ::core::marker::Sync for MobileBroadbandUicc {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandUiccApp(::windows_core::IUnknown);
impl MobileBroadbandUiccApp {
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<UiccAppKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecordDetailsAsync<P0>(&self, uiccfilepath: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<u32>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRecordDetailsAsync)(::windows_core::Interface::as_raw(this), uiccfilepath.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadRecordAsync<P0>(&self, uiccfilepath: P0, recordindex: i32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<u32>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadRecordAsync)(::windows_core::Interface::as_raw(this), uiccfilepath.try_into_param()?.abi(), recordindex, &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandUiccApp {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandUiccApp {
    type Vtable = IMobileBroadbandUiccApp_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandUiccApp {
    const IID: ::windows_core::GUID = <IMobileBroadbandUiccApp as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandUiccApp {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccApp";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandUiccApp, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandUiccApp {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccApp {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandUiccAppReadRecordResult(::windows_core::IUnknown);
impl MobileBroadbandUiccAppReadRecordResult {
    pub fn Status(&self) -> ::windows_core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandUiccAppReadRecordResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandUiccAppReadRecordResult {
    type Vtable = IMobileBroadbandUiccAppReadRecordResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandUiccAppReadRecordResult {
    const IID: ::windows_core::GUID = <IMobileBroadbandUiccAppReadRecordResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandUiccAppReadRecordResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppReadRecordResult";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandUiccAppReadRecordResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandUiccAppReadRecordResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccAppReadRecordResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandUiccAppRecordDetailsResult(::windows_core::IUnknown);
impl MobileBroadbandUiccAppRecordDetailsResult {
    pub fn Status(&self) -> ::windows_core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<UiccAppRecordKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RecordCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RecordSize(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadAccessCondition(&self) -> ::windows_core::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAccessCondition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WriteAccessCondition(&self) -> ::windows_core::Result<UiccAccessCondition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAccessCondition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandUiccAppRecordDetailsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandUiccAppRecordDetailsResult {
    type Vtable = IMobileBroadbandUiccAppRecordDetailsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandUiccAppRecordDetailsResult {
    const IID: ::windows_core::GUID = <IMobileBroadbandUiccAppRecordDetailsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandUiccAppRecordDetailsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppRecordDetailsResult";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandUiccAppRecordDetailsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandUiccAppRecordDetailsResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccAppRecordDetailsResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandUiccAppsResult(::windows_core::IUnknown);
impl MobileBroadbandUiccAppsResult {
    pub fn Status(&self) -> ::windows_core::Result<MobileBroadbandUiccAppOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UiccApps(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UiccApps)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandUiccAppsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MobileBroadbandUiccAppsResult {
    type Vtable = IMobileBroadbandUiccAppsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandUiccAppsResult {
    const IID: ::windows_core::GUID = <IMobileBroadbandUiccAppsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandUiccAppsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.MobileBroadbandUiccAppsResult";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandUiccAppsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MobileBroadbandUiccAppsResult {}
unsafe impl ::core::marker::Sync for MobileBroadbandUiccAppsResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorDataUsageTriggerDetails(::windows_core::IUnknown);
impl NetworkOperatorDataUsageTriggerDetails {
    pub fn NotificationKind(&self) -> ::windows_core::Result<NetworkOperatorDataUsageNotificationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NotificationKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorDataUsageTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NetworkOperatorDataUsageTriggerDetails {
    type Vtable = INetworkOperatorDataUsageTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorDataUsageTriggerDetails {
    const IID: ::windows_core::GUID = <INetworkOperatorDataUsageTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorDataUsageTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorDataUsageTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorDataUsageTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkOperatorDataUsageTriggerDetails {}
unsafe impl ::core::marker::Sync for NetworkOperatorDataUsageTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorNotificationEventDetails(::windows_core::IUnknown);
impl NetworkOperatorNotificationEventDetails {
    pub fn NotificationType(&self) -> ::windows_core::Result<NetworkOperatorEventMessageType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NotificationType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EncodingType(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodingType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RuleId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RuleId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Sms\"`"]
    #[cfg(feature = "Devices_Sms")]
    pub fn SmsMessage(&self) -> ::windows_core::Result<super::super::Devices::Sms::ISmsMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmsMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AuthorizeTethering(&self, allow: bool, entitlementfailurereason: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INetworkOperatorTetheringEntitlementCheck>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AuthorizeTethering)(::windows_core::Interface::as_raw(this), allow, ::core::mem::transmute_copy(entitlementfailurereason)).ok() }
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorNotificationEventDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NetworkOperatorNotificationEventDetails {
    type Vtable = INetworkOperatorNotificationEventDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorNotificationEventDetails {
    const IID: ::windows_core::GUID = <INetworkOperatorNotificationEventDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorNotificationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorNotificationEventDetails";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorNotificationEventDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkOperatorNotificationEventDetails {}
unsafe impl ::core::marker::Sync for NetworkOperatorNotificationEventDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorTetheringAccessPointConfiguration(::windows_core::IUnknown);
impl NetworkOperatorTetheringAccessPointConfiguration {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorTetheringAccessPointConfiguration, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Ssid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ssid)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSsid(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSsid)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Passphrase(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Passphrase)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPassphrase(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPassphrase)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsBandSupported(&self, band: TetheringWiFiBand) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBandSupported)(::windows_core::Interface::as_raw(this), band, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsBandSupportedAsync(&self, band: TetheringWiFiBand) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBandSupportedAsync)(::windows_core::Interface::as_raw(this), band, &mut result__).from_abi(result__)
        }
    }
    pub fn Band(&self) -> ::windows_core::Result<TetheringWiFiBand> {
        let this = &::windows_core::ComInterface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Band)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBand(&self, value: TetheringWiFiBand) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<INetworkOperatorTetheringAccessPointConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBand)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorTetheringAccessPointConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NetworkOperatorTetheringAccessPointConfiguration {
    type Vtable = INetworkOperatorTetheringAccessPointConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorTetheringAccessPointConfiguration {
    const IID: ::windows_core::GUID = <INetworkOperatorTetheringAccessPointConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorTetheringAccessPointConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringAccessPointConfiguration";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringAccessPointConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkOperatorTetheringAccessPointConfiguration {}
unsafe impl ::core::marker::Sync for NetworkOperatorTetheringAccessPointConfiguration {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorTetheringClient(::windows_core::IUnknown);
impl NetworkOperatorTetheringClient {
    pub fn MacAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MacAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HostNames(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HostNames)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorTetheringClient {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NetworkOperatorTetheringClient {
    type Vtable = INetworkOperatorTetheringClient_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorTetheringClient {
    const IID: ::windows_core::GUID = <INetworkOperatorTetheringClient as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorTetheringClient {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringClient";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringClient, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkOperatorTetheringClient {}
unsafe impl ::core::marker::Sync for NetworkOperatorTetheringClient {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorTetheringManager(::windows_core::IUnknown);
impl NetworkOperatorTetheringManager {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTetheringClients(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>> {
        let this = &::windows_core::ComInterface::cast::<INetworkOperatorTetheringClientManager>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTetheringClients)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxClientCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxClientCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ClientCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TetheringOperationalState(&self) -> ::windows_core::Result<TetheringOperationalState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TetheringOperationalState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentAccessPointConfiguration(&self) -> ::windows_core::Result<NetworkOperatorTetheringAccessPointConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentAccessPointConfiguration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ConfigureAccessPointAsync<P0>(&self, configuration: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<NetworkOperatorTetheringAccessPointConfiguration>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConfigureAccessPointAsync)(::windows_core::Interface::as_raw(this), configuration.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartTetheringAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTetheringAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StopTetheringAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopTetheringAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetTetheringCapability(networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<TetheringCapability> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTetheringCapability)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<NetworkOperatorTetheringManager> {
        Self::INetworkOperatorTetheringManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Networking_Connectivity\"`"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn GetTetheringCapabilityFromConnectionProfile<P0>(profile: P0) -> ::windows_core::Result<TetheringCapability>
    where
        P0: ::windows_core::IntoParam<super::Connectivity::ConnectionProfile>,
    {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTetheringCapabilityFromConnectionProfile)(::windows_core::Interface::as_raw(this), profile.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Networking_Connectivity\"`"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfile<P0>(profile: P0) -> ::windows_core::Result<NetworkOperatorTetheringManager>
    where
        P0: ::windows_core::IntoParam<super::Connectivity::ConnectionProfile>,
    {
        Self::INetworkOperatorTetheringManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromConnectionProfile)(::windows_core::Interface::as_raw(this), profile.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Networking_Connectivity\"`"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn CreateFromConnectionProfileWithTargetAdapter<P0, P1>(profile: P0, adapter: P1) -> ::windows_core::Result<NetworkOperatorTetheringManager>
    where
        P0: ::windows_core::IntoParam<super::Connectivity::ConnectionProfile>,
        P1: ::windows_core::IntoParam<super::Connectivity::NetworkAdapter>,
    {
        Self::INetworkOperatorTetheringManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromConnectionProfileWithTargetAdapter)(::windows_core::Interface::as_raw(this), profile.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn IsNoConnectionsTimeoutEnabled() -> ::windows_core::Result<bool> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNoConnectionsTimeoutEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn EnableNoConnectionsTimeout() -> ::windows_core::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).EnableNoConnectionsTimeout)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EnableNoConnectionsTimeoutAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnableNoConnectionsTimeoutAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DisableNoConnectionsTimeout() -> ::windows_core::Result<()> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).DisableNoConnectionsTimeout)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DisableNoConnectionsTimeoutAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::INetworkOperatorTetheringManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisableNoConnectionsTimeoutAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics2<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics3<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkOperatorTetheringManagerStatics4<R, F: FnOnce(&INetworkOperatorTetheringManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorTetheringManager, INetworkOperatorTetheringManagerStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorTetheringManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NetworkOperatorTetheringManager {
    type Vtable = INetworkOperatorTetheringManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorTetheringManager {
    const IID: ::windows_core::GUID = <INetworkOperatorTetheringManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorTetheringManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringManager";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorTetheringOperationResult(::windows_core::IUnknown);
impl NetworkOperatorTetheringOperationResult {
    pub fn Status(&self) -> ::windows_core::Result<TetheringOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AdditionalErrorMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalErrorMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorTetheringOperationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NetworkOperatorTetheringOperationResult {
    type Vtable = INetworkOperatorTetheringOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorTetheringOperationResult {
    const IID: ::windows_core::GUID = <INetworkOperatorTetheringOperationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorTetheringOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.NetworkOperatorTetheringOperationResult";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorTetheringOperationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProvisionFromXmlDocumentResults(::windows_core::IUnknown);
impl ProvisionFromXmlDocumentResults {
    pub fn AllElementsProvisioned(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllElementsProvisioned)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProvisionResultsXml(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionResultsXml)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProvisionFromXmlDocumentResults {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProvisionFromXmlDocumentResults {
    type Vtable = IProvisionFromXmlDocumentResults_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProvisionFromXmlDocumentResults {
    const IID: ::windows_core::GUID = <IProvisionFromXmlDocumentResults as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProvisionFromXmlDocumentResults {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionFromXmlDocumentResults";
}
::windows_core::imp::interface_hierarchy!(ProvisionFromXmlDocumentResults, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProvisionedProfile(::windows_core::IUnknown);
impl ProvisionedProfile {
    #[doc = "Required features: `\"Networking_Connectivity\"`"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn UpdateCost(&self, value: super::Connectivity::NetworkCostType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateCost)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateUsage(&self, value: ProfileUsage) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateUsage)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for ProvisionedProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProvisionedProfile {
    type Vtable = IProvisionedProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProvisionedProfile {
    const IID: ::windows_core::GUID = <IProvisionedProfile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProvisionedProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisionedProfile";
}
::windows_core::imp::interface_hierarchy!(ProvisionedProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProvisioningAgent(::windows_core::IUnknown);
impl ProvisioningAgent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProvisioningAgent, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProvisionFromXmlDocumentAsync(&self, provisioningxmldocument: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProvisionFromXmlDocumentAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(provisioningxmldocument), &mut result__).from_abi(result__)
        }
    }
    pub fn GetProvisionedProfile(&self, mediatype: ProfileMediaType, profilename: &::windows_core::HSTRING) -> ::windows_core::Result<ProvisionedProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProvisionedProfile)(::windows_core::Interface::as_raw(this), mediatype, ::core::mem::transmute_copy(profilename), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<ProvisioningAgent> {
        Self::IProvisioningAgentStaticMethods(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProvisioningAgentStaticMethods<R, F: FnOnce(&IProvisioningAgentStaticMethods) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProvisioningAgent, IProvisioningAgentStaticMethods> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ProvisioningAgent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProvisioningAgent {
    type Vtable = IProvisioningAgent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProvisioningAgent {
    const IID: ::windows_core::GUID = <IProvisioningAgent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProvisioningAgent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ProvisioningAgent";
}
::windows_core::imp::interface_hierarchy!(ProvisioningAgent, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TetheringEntitlementCheckTriggerDetails(::windows_core::IUnknown);
impl TetheringEntitlementCheckTriggerDetails {
    pub fn NetworkAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowTethering(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AllowTethering)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DenyTethering(&self, entitlementfailurereason: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DenyTethering)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(entitlementfailurereason)).ok() }
    }
}
impl ::windows_core::RuntimeType for TetheringEntitlementCheckTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TetheringEntitlementCheckTriggerDetails {
    type Vtable = ITetheringEntitlementCheckTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TetheringEntitlementCheckTriggerDetails {
    const IID: ::windows_core::GUID = <ITetheringEntitlementCheckTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TetheringEntitlementCheckTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.TetheringEntitlementCheckTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(TetheringEntitlementCheckTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TetheringEntitlementCheckTriggerDetails {}
unsafe impl ::core::marker::Sync for TetheringEntitlementCheckTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UssdMessage(::windows_core::IUnknown);
impl UssdMessage {
    pub fn DataCodingScheme(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataCodingScheme)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDataCodingScheme(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataCodingScheme)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetPayload(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetPayload)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn SetPayload(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPayload)(::windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn PayloadAsText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PayloadAsText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPayloadAsText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPayloadAsText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateMessage(messagetext: &::windows_core::HSTRING) -> ::windows_core::Result<UssdMessage> {
        Self::IUssdMessageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(messagetext), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUssdMessageFactory<R, F: FnOnce(&IUssdMessageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UssdMessage, IUssdMessageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for UssdMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UssdMessage {
    type Vtable = IUssdMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UssdMessage {
    const IID: ::windows_core::GUID = <IUssdMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UssdMessage {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdMessage";
}
::windows_core::imp::interface_hierarchy!(UssdMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UssdMessage {}
unsafe impl ::core::marker::Sync for UssdMessage {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UssdReply(::windows_core::IUnknown);
impl UssdReply {
    pub fn ResultCode(&self) -> ::windows_core::Result<UssdResultCode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<UssdMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for UssdReply {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UssdReply {
    type Vtable = IUssdReply_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UssdReply {
    const IID: ::windows_core::GUID = <IUssdReply as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UssdReply {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdReply";
}
::windows_core::imp::interface_hierarchy!(UssdReply, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UssdSession(::windows_core::IUnknown);
impl UssdSession {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAndGetReplyAsync<P0>(&self, message: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<UssdReply>>
    where
        P0: ::windows_core::IntoParam<UssdMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendMessageAndGetReplyAsync)(::windows_core::Interface::as_raw(this), message.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateFromNetworkAccountId(networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNetworkAccountId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromNetworkInterfaceId(networkinterfaceid: &::windows_core::HSTRING) -> ::windows_core::Result<UssdSession> {
        Self::IUssdSessionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromNetworkInterfaceId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(networkinterfaceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUssdSessionStatics<R, F: FnOnce(&IUssdSessionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UssdSession, IUssdSessionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for UssdSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UssdSession {
    type Vtable = IUssdSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UssdSession {
    const IID: ::windows_core::GUID = <IUssdSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UssdSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.UssdSession";
}
::windows_core::imp::interface_hierarchy!(UssdSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DataClasses(pub u32);
impl DataClasses {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const NewRadioNonStandalone: Self = Self(64u32);
    pub const NewRadioStandalone: Self = Self(128u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for DataClasses {}
impl ::core::clone::Clone for DataClasses {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataClasses {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DataClasses {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DataClasses {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataClasses").field(&self.0).finish()
    }
}
impl DataClasses {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DataClasses {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DataClasses {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DataClasses {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DataClasses {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DataClasses {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for DataClasses {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.DataClasses;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimAuthenticationPreference(pub i32);
impl ESimAuthenticationPreference {
    pub const OnEntry: Self = Self(0i32);
    pub const OnAction: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimAuthenticationPreference {}
impl ::core::clone::Clone for ESimAuthenticationPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimAuthenticationPreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimAuthenticationPreference {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimAuthenticationPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimAuthenticationPreference").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimAuthenticationPreference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimAuthenticationPreference;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimDiscoverResultKind(pub i32);
impl ESimDiscoverResultKind {
    pub const None: Self = Self(0i32);
    pub const Events: Self = Self(1i32);
    pub const ProfileMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimDiscoverResultKind {}
impl ::core::clone::Clone for ESimDiscoverResultKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimDiscoverResultKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimDiscoverResultKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimDiscoverResultKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDiscoverResultKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimDiscoverResultKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimDiscoverResultKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimOperationStatus(pub i32);
impl ESimOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAuthorized: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const PolicyViolation: Self = Self(3i32);
    pub const InsufficientSpaceOnCard: Self = Self(4i32);
    pub const ServerFailure: Self = Self(5i32);
    pub const ServerNotReachable: Self = Self(6i32);
    pub const TimeoutWaitingForUserConsent: Self = Self(7i32);
    pub const IncorrectConfirmationCode: Self = Self(8i32);
    pub const ConfirmationCodeMaxRetriesExceeded: Self = Self(9i32);
    pub const CardRemoved: Self = Self(10i32);
    pub const CardBusy: Self = Self(11i32);
    pub const Other: Self = Self(12i32);
    pub const CardGeneralFailure: Self = Self(13i32);
    pub const ConfirmationCodeMissing: Self = Self(14i32);
    pub const InvalidMatchingId: Self = Self(15i32);
    pub const NoEligibleProfileForThisDevice: Self = Self(16i32);
    pub const OperationAborted: Self = Self(17i32);
    pub const EidMismatch: Self = Self(18i32);
    pub const ProfileNotAvailableForNewBinding: Self = Self(19i32);
    pub const ProfileNotReleasedByOperator: Self = Self(20i32);
    pub const OperationProhibitedByProfileClass: Self = Self(21i32);
    pub const ProfileNotPresent: Self = Self(22i32);
    pub const NoCorrespondingRequest: Self = Self(23i32);
    pub const TimeoutWaitingForResponse: Self = Self(24i32);
    pub const IccidAlreadyExists: Self = Self(25i32);
    pub const ProfileProcessingError: Self = Self(26i32);
    pub const ServerNotTrusted: Self = Self(27i32);
    pub const ProfileDownloadMaxRetriesExceeded: Self = Self(28i32);
}
impl ::core::marker::Copy for ESimOperationStatus {}
impl ::core::clone::Clone for ESimOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimOperationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimOperationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimOperationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimProfileClass(pub i32);
impl ESimProfileClass {
    pub const Operational: Self = Self(0i32);
    pub const Test: Self = Self(1i32);
    pub const Provisioning: Self = Self(2i32);
}
impl ::core::marker::Copy for ESimProfileClass {}
impl ::core::clone::Clone for ESimProfileClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimProfileClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimProfileClass {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimProfileClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileClass").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimProfileClass {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileClass;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimProfileMetadataState(pub i32);
impl ESimProfileMetadataState {
    pub const Unknown: Self = Self(0i32);
    pub const WaitingForInstall: Self = Self(1i32);
    pub const Downloading: Self = Self(2i32);
    pub const Installing: Self = Self(3i32);
    pub const Expired: Self = Self(4i32);
    pub const RejectingDownload: Self = Self(5i32);
    pub const NoLongerAvailable: Self = Self(6i32);
    pub const DeniedByPolicy: Self = Self(7i32);
}
impl ::core::marker::Copy for ESimProfileMetadataState {}
impl ::core::clone::Clone for ESimProfileMetadataState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimProfileMetadataState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimProfileMetadataState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimProfileMetadataState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileMetadataState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimProfileMetadataState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileMetadataState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimProfileState(pub i32);
impl ESimProfileState {
    pub const Unknown: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const Deleted: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimProfileState {}
impl ::core::clone::Clone for ESimProfileState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimProfileState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimProfileState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimProfileState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimProfileState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimProfileState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimState(pub i32);
impl ESimState {
    pub const Unknown: Self = Self(0i32);
    pub const Idle: Self = Self(1i32);
    pub const Removed: Self = Self(2i32);
    pub const Busy: Self = Self(3i32);
}
impl ::core::marker::Copy for ESimState {}
impl ::core::clone::Clone for ESimState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ESimWatcherStatus(pub i32);
impl ESimWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
}
impl ::core::marker::Copy for ESimWatcherStatus {}
impl ::core::clone::Clone for ESimWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESimWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ESimWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ESimWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ESimWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ESimWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HotspotAuthenticationResponseCode(pub i32);
impl HotspotAuthenticationResponseCode {
    pub const NoError: Self = Self(0i32);
    pub const LoginSucceeded: Self = Self(50i32);
    pub const LoginFailed: Self = Self(100i32);
    pub const RadiusServerError: Self = Self(102i32);
    pub const NetworkAdministratorError: Self = Self(105i32);
    pub const LoginAborted: Self = Self(151i32);
    pub const AccessGatewayInternalError: Self = Self(255i32);
}
impl ::core::marker::Copy for HotspotAuthenticationResponseCode {}
impl ::core::clone::Clone for HotspotAuthenticationResponseCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HotspotAuthenticationResponseCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HotspotAuthenticationResponseCode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HotspotAuthenticationResponseCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationResponseCode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HotspotAuthenticationResponseCode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.HotspotAuthenticationResponseCode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandAccountWatcherStatus(pub i32);
impl MobileBroadbandAccountWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Aborted: Self = Self(4i32);
}
impl ::core::marker::Copy for MobileBroadbandAccountWatcherStatus {}
impl ::core::clone::Clone for MobileBroadbandAccountWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandAccountWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandAccountWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandAccountWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandAccountWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandAccountWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandDeviceType(pub i32);
impl MobileBroadbandDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Embedded: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
    pub const Remote: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandDeviceType {}
impl ::core::clone::Clone for MobileBroadbandDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandDeviceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandDeviceType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandModemStatus(pub i32);
impl MobileBroadbandModemStatus {
    pub const Success: Self = Self(0i32);
    pub const OtherFailure: Self = Self(1i32);
    pub const Busy: Self = Self(2i32);
    pub const NoDeviceSupport: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandModemStatus {}
impl ::core::clone::Clone for MobileBroadbandModemStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandModemStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandModemStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandModemStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandModemStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandModemStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandPinFormat(pub i32);
impl MobileBroadbandPinFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Numeric: Self = Self(1i32);
    pub const Alphanumeric: Self = Self(2i32);
}
impl ::core::marker::Copy for MobileBroadbandPinFormat {}
impl ::core::clone::Clone for MobileBroadbandPinFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandPinFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandPinFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandPinFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinFormat;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandPinLockState(pub i32);
impl MobileBroadbandPinLockState {
    pub const Unknown: Self = Self(0i32);
    pub const Unlocked: Self = Self(1i32);
    pub const PinRequired: Self = Self(2i32);
    pub const PinUnblockKeyRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandPinLockState {}
impl ::core::clone::Clone for MobileBroadbandPinLockState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandPinLockState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandPinLockState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandPinLockState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinLockState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinLockState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandPinType(pub i32);
impl MobileBroadbandPinType {
    pub const None: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
    pub const Pin1: Self = Self(2i32);
    pub const Pin2: Self = Self(3i32);
    pub const SimPin: Self = Self(4i32);
    pub const FirstSimPin: Self = Self(5i32);
    pub const NetworkPin: Self = Self(6i32);
    pub const NetworkSubsetPin: Self = Self(7i32);
    pub const ServiceProviderPin: Self = Self(8i32);
    pub const CorporatePin: Self = Self(9i32);
    pub const SubsidyLock: Self = Self(10i32);
}
impl ::core::marker::Copy for MobileBroadbandPinType {}
impl ::core::clone::Clone for MobileBroadbandPinType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandPinType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandPinType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandPinType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandPinType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandRadioState(pub i32);
impl MobileBroadbandRadioState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl ::core::marker::Copy for MobileBroadbandRadioState {}
impl ::core::clone::Clone for MobileBroadbandRadioState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandRadioState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandRadioState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandRadioState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandRadioState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandRadioState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandSlotState(pub i32);
impl MobileBroadbandSlotState {
    pub const Unmanaged: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const OffEmpty: Self = Self(2i32);
    pub const Off: Self = Self(3i32);
    pub const Empty: Self = Self(4i32);
    pub const NotReady: Self = Self(5i32);
    pub const Active: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const ActiveEsim: Self = Self(8i32);
    pub const ActiveEsimNoProfile: Self = Self(9i32);
}
impl ::core::marker::Copy for MobileBroadbandSlotState {}
impl ::core::clone::Clone for MobileBroadbandSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandSlotState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandSlotState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandSlotState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandSlotState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandSlotState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MobileBroadbandUiccAppOperationStatus(pub i32);
impl MobileBroadbandUiccAppOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidUiccFilePath: Self = Self(1i32);
    pub const AccessConditionNotHeld: Self = Self(2i32);
    pub const UiccBusy: Self = Self(3i32);
}
impl ::core::marker::Copy for MobileBroadbandUiccAppOperationStatus {}
impl ::core::clone::Clone for MobileBroadbandUiccAppOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MobileBroadbandUiccAppOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MobileBroadbandUiccAppOperationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MobileBroadbandUiccAppOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppOperationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandUiccAppOperationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.MobileBroadbandUiccAppOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkDeviceStatus(pub i32);
impl NetworkDeviceStatus {
    pub const DeviceNotReady: Self = Self(0i32);
    pub const DeviceReady: Self = Self(1i32);
    pub const SimNotInserted: Self = Self(2i32);
    pub const BadSim: Self = Self(3i32);
    pub const DeviceHardwareFailure: Self = Self(4i32);
    pub const AccountNotActivated: Self = Self(5i32);
    pub const DeviceLocked: Self = Self(6i32);
    pub const DeviceBlocked: Self = Self(7i32);
}
impl ::core::marker::Copy for NetworkDeviceStatus {}
impl ::core::clone::Clone for NetworkDeviceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkDeviceStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkDeviceStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkDeviceStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkDeviceStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkOperatorDataUsageNotificationKind(pub i32);
impl NetworkOperatorDataUsageNotificationKind {
    pub const DataUsageProgress: Self = Self(0i32);
}
impl ::core::marker::Copy for NetworkOperatorDataUsageNotificationKind {}
impl ::core::clone::Clone for NetworkOperatorDataUsageNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkOperatorDataUsageNotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkOperatorDataUsageNotificationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkOperatorDataUsageNotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorDataUsageNotificationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorDataUsageNotificationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorDataUsageNotificationKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkOperatorEventMessageType(pub i32);
impl NetworkOperatorEventMessageType {
    pub const Gsm: Self = Self(0i32);
    pub const Cdma: Self = Self(1i32);
    pub const Ussd: Self = Self(2i32);
    pub const DataPlanThresholdReached: Self = Self(3i32);
    pub const DataPlanReset: Self = Self(4i32);
    pub const DataPlanDeleted: Self = Self(5i32);
    pub const ProfileConnected: Self = Self(6i32);
    pub const ProfileDisconnected: Self = Self(7i32);
    pub const RegisteredRoaming: Self = Self(8i32);
    pub const RegisteredHome: Self = Self(9i32);
    pub const TetheringEntitlementCheck: Self = Self(10i32);
    pub const TetheringOperationalStateChanged: Self = Self(11i32);
    pub const TetheringNumberOfClientsChanged: Self = Self(12i32);
}
impl ::core::marker::Copy for NetworkOperatorEventMessageType {}
impl ::core::clone::Clone for NetworkOperatorEventMessageType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkOperatorEventMessageType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkOperatorEventMessageType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkOperatorEventMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorEventMessageType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorEventMessageType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkOperatorEventMessageType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkRegistrationState(pub i32);
impl NetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl ::core::marker::Copy for NetworkRegistrationState {}
impl ::core::clone::Clone for NetworkRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkRegistrationState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkRegistrationState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkRegistrationState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.NetworkRegistrationState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProfileMediaType(pub i32);
impl ProfileMediaType {
    pub const Wlan: Self = Self(0i32);
    pub const Wwan: Self = Self(1i32);
}
impl ::core::marker::Copy for ProfileMediaType {}
impl ::core::clone::Clone for ProfileMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProfileMediaType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProfileMediaType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProfileMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProfileMediaType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProfileMediaType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.ProfileMediaType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TetheringCapability(pub i32);
impl TetheringCapability {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledByGroupPolicy: Self = Self(1i32);
    pub const DisabledByHardwareLimitation: Self = Self(2i32);
    pub const DisabledByOperator: Self = Self(3i32);
    pub const DisabledBySku: Self = Self(4i32);
    pub const DisabledByRequiredAppNotInstalled: Self = Self(5i32);
    pub const DisabledDueToUnknownCause: Self = Self(6i32);
    pub const DisabledBySystemCapability: Self = Self(7i32);
}
impl ::core::marker::Copy for TetheringCapability {}
impl ::core::clone::Clone for TetheringCapability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringCapability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TetheringCapability {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TetheringCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringCapability").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TetheringCapability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringCapability;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TetheringOperationStatus(pub i32);
impl TetheringOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const MobileBroadbandDeviceOff: Self = Self(2i32);
    pub const WiFiDeviceOff: Self = Self(3i32);
    pub const EntitlementCheckTimeout: Self = Self(4i32);
    pub const EntitlementCheckFailure: Self = Self(5i32);
    pub const OperationInProgress: Self = Self(6i32);
    pub const BluetoothDeviceOff: Self = Self(7i32);
    pub const NetworkLimitedConnectivity: Self = Self(8i32);
}
impl ::core::marker::Copy for TetheringOperationStatus {}
impl ::core::clone::Clone for TetheringOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TetheringOperationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TetheringOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringOperationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TetheringOperationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TetheringOperationalState(pub i32);
impl TetheringOperationalState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const InTransition: Self = Self(3i32);
}
impl ::core::marker::Copy for TetheringOperationalState {}
impl ::core::clone::Clone for TetheringOperationalState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringOperationalState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TetheringOperationalState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TetheringOperationalState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringOperationalState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TetheringOperationalState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringOperationalState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TetheringWiFiBand(pub i32);
impl TetheringWiFiBand {
    pub const Auto: Self = Self(0i32);
    pub const TwoPointFourGigahertz: Self = Self(1i32);
    pub const FiveGigahertz: Self = Self(2i32);
}
impl ::core::marker::Copy for TetheringWiFiBand {}
impl ::core::clone::Clone for TetheringWiFiBand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TetheringWiFiBand {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TetheringWiFiBand {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TetheringWiFiBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringWiFiBand").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TetheringWiFiBand {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.TetheringWiFiBand;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UiccAccessCondition(pub i32);
impl UiccAccessCondition {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const Pin1: Self = Self(1i32);
    pub const Pin2: Self = Self(2i32);
    pub const Pin3: Self = Self(3i32);
    pub const Pin4: Self = Self(4i32);
    pub const Administrative5: Self = Self(5i32);
    pub const Administrative6: Self = Self(6i32);
    pub const NeverAllowed: Self = Self(7i32);
}
impl ::core::marker::Copy for UiccAccessCondition {}
impl ::core::clone::Clone for UiccAccessCondition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UiccAccessCondition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UiccAccessCondition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UiccAccessCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAccessCondition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UiccAccessCondition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAccessCondition;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UiccAppKind(pub i32);
impl UiccAppKind {
    pub const Unknown: Self = Self(0i32);
    pub const MF: Self = Self(1i32);
    pub const MFSim: Self = Self(2i32);
    pub const MFRuim: Self = Self(3i32);
    pub const USim: Self = Self(4i32);
    pub const CSim: Self = Self(5i32);
    pub const ISim: Self = Self(6i32);
}
impl ::core::marker::Copy for UiccAppKind {}
impl ::core::clone::Clone for UiccAppKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UiccAppKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UiccAppKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UiccAppKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAppKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UiccAppKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UiccAppRecordKind(pub i32);
impl UiccAppRecordKind {
    pub const Unknown: Self = Self(0i32);
    pub const Transparent: Self = Self(1i32);
    pub const RecordOriented: Self = Self(2i32);
}
impl ::core::marker::Copy for UiccAppRecordKind {}
impl ::core::clone::Clone for UiccAppRecordKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UiccAppRecordKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UiccAppRecordKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UiccAppRecordKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAppRecordKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UiccAppRecordKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UiccAppRecordKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UssdResultCode(pub i32);
impl UssdResultCode {
    pub const NoActionRequired: Self = Self(0i32);
    pub const ActionRequired: Self = Self(1i32);
    pub const Terminated: Self = Self(2i32);
    pub const OtherLocalClient: Self = Self(3i32);
    pub const OperationNotSupported: Self = Self(4i32);
    pub const NetworkTimeout: Self = Self(5i32);
}
impl ::core::marker::Copy for UssdResultCode {}
impl ::core::clone::Clone for UssdResultCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UssdResultCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UssdResultCode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UssdResultCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdResultCode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UssdResultCode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.NetworkOperators.UssdResultCode;i4)");
}
#[repr(C)]
pub struct ESimProfileInstallProgress {
    pub TotalSizeInBytes: i32,
    pub InstalledSizeInBytes: i32,
}
impl ::core::marker::Copy for ESimProfileInstallProgress {}
impl ::core::clone::Clone for ESimProfileInstallProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ESimProfileInstallProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ESimProfileInstallProgress").field("TotalSizeInBytes", &self.TotalSizeInBytes).field("InstalledSizeInBytes", &self.InstalledSizeInBytes).finish()
    }
}
impl ::windows_core::TypeKind for ESimProfileInstallProgress {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for ESimProfileInstallProgress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ESimProfileInstallProgress;i4;i4)");
}
impl ::core::cmp::PartialEq for ESimProfileInstallProgress {
    fn eq(&self, other: &Self) -> bool {
        self.TotalSizeInBytes == other.TotalSizeInBytes && self.InstalledSizeInBytes == other.InstalledSizeInBytes
    }
}
impl ::core::cmp::Eq for ESimProfileInstallProgress {}
impl ::core::default::Default for ESimProfileInstallProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub struct ProfileUsage {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::DateTime,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ProfileUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for ProfileUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProfileUsage").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows_core::TypeKind for ProfileUsage {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeType for ProfileUsage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.NetworkOperators.ProfileUsage;u4;struct(Windows.Foundation.DateTime;i8))");
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for ProfileUsage {
    fn eq(&self, other: &Self) -> bool {
        self.UsageInMegabytes == other.UsageInMegabytes && self.LastSyncTime == other.LastSyncTime
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for ProfileUsage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
