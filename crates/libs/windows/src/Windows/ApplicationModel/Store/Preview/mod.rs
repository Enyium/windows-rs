#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
#[doc = "Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`"]
pub mod InstallControl;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeliveryOptimizationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeliveryOptimizationSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1810fda0_e853_565e_b874_7a8a7b9a0e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DownloadMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadMode) -> ::windows_core::HRESULT,
    pub DownloadModeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadModeSource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeliveryOptimizationSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeliveryOptimizationSettingsStatics {
    type Vtable = IDeliveryOptimizationSettingsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeliveryOptimizationSettingsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c817caf_aed5_5999_b4c9_8c60898bc4f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStoreConfigurationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreConfigurationStatics {
    type Vtable = IStoreConfigurationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStoreConfigurationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x728f7fc0_8628_42ec_84a2_07780eb44d8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSystemConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cataloghardwaremanufacturerid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, catalogstorecontentmodifierid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemConfiguration: usize,
    pub SetMobileOperatorConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mobileoperatorid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows_core::HRESULT,
    pub SetStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HardwareManufacturerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FilterUnsupportedSystemFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, systemfeatures: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FilterUnsupportedSystemFeaturesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStoreConfigurationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreConfigurationStatics2 {
    type Vtable = IStoreConfigurationStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStoreConfigurationStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x657c4595_c8b7_4fe9_9f4c_4d71027d347e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PurchasePromptingPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PurchasePromptingPolicy: usize,
    #[cfg(feature = "Foundation")]
    pub SetPurchasePromptingPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPurchasePromptingPolicy: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStoreConfigurationStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreConfigurationStatics3 {
    type Vtable = IStoreConfigurationStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStoreConfigurationStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d45f57c_f144_4cb5_9d3f_4eb05e30b6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasStoreWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub HasStoreWebAccountForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    HasStoreWebAccountForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetStoreLogDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: StoreLogOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetStoreLogDataAsync: usize,
    #[cfg(feature = "System")]
    pub SetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetStoreWebAccountIdForUser: usize,
    #[cfg(feature = "System")]
    pub IsStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsStoreWebAccountIdForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetPurchasePromptingPolicyForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SetPurchasePromptingPolicyForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStoreConfigurationStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreConfigurationStatics4 {
    type Vtable = IStoreConfigurationStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStoreConfigurationStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20ff56d2_4ee3_4cf0_9b12_552c03310f75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetStoreWebAccountIdForUser: usize,
    pub SetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub SetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetEnterpriseStoreWebAccountIdForUser: usize,
    pub GetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetEnterpriseStoreWebAccountIdForUser: usize,
    pub ShouldRestrictToEnterpriseStoreOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub ShouldRestrictToEnterpriseStoreOnlyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ShouldRestrictToEnterpriseStoreOnlyForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStoreConfigurationStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreConfigurationStatics5 {
    type Vtable = IStoreConfigurationStatics5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStoreConfigurationStatics5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7613191_8fa9_49db_822b_0160e7e4e5c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPinToDesktopSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPinToTaskbarSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPinToStartSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PinToDesktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub PinToDesktopForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    PinToDesktopForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStoreHardwareManufacturerInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStoreHardwareManufacturerInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf292dc08_c654_43ac_a21f_34801c9d3388);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreHardwareManufacturerInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HardwareManufacturerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StoreContentModifierId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorePreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePreview {
    type Vtable = IStorePreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorePreview {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a157241_840e_49a9_bc01_5d5b01fbc8e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseByProductIdAndSkuIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseByProductIdAndSkuIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadAddOnProductInfosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadAddOnProductInfosAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorePreviewProductInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorePreviewProductInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1937dbb3_6c01_4c9d_85cd_5babaac2b351);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewProductInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProductType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SkuInfoList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SkuInfoList: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorePreviewPurchaseResults(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorePreviewPurchaseResults {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0daaed1_d6c5_4e53_a043_fba0d8e61231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewPurchaseResults_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductPurchaseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePreviewProductPurchaseStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorePreviewSkuInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorePreviewSkuInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81fd76e2_0b26_48d9_98ce_27461c669d6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewSkuInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SkuId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SkuType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormattedListPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExtendedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebAuthenticationCoreManagerHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAuthenticationCoreManagerHelper {
    type Vtable = IWebAuthenticationCoreManagerHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebAuthenticationCoreManagerHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06a50525_e715_4123_9276_9d6f865ba55f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, uielement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAndWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uielement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAndWebAccountAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeliveryOptimizationSettings(::windows_core::IUnknown);
impl DeliveryOptimizationSettings {
    pub fn DownloadMode(&self) -> ::windows_core::Result<DeliveryOptimizationDownloadMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DownloadModeSource(&self) -> ::windows_core::Result<DeliveryOptimizationDownloadModeSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadModeSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentSettings() -> ::windows_core::Result<DeliveryOptimizationSettings> {
        Self::IDeliveryOptimizationSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeliveryOptimizationSettingsStatics<R, F: FnOnce(&IDeliveryOptimizationSettingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeliveryOptimizationSettings, IDeliveryOptimizationSettingsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DeliveryOptimizationSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeliveryOptimizationSettings {
    const IID: ::windows_core::GUID = <IDeliveryOptimizationSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeliveryOptimizationSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings";
}
::windows_core::imp::interface_hierarchy!(DeliveryOptimizationSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeliveryOptimizationSettings {}
unsafe impl ::core::marker::Sync for DeliveryOptimizationSettings {}
pub struct StoreConfiguration;
impl StoreConfiguration {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemConfiguration(cataloghardwaremanufacturerid: &::windows_core::HSTRING, catalogstorecontentmodifierid: &::windows_core::HSTRING, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetSystemConfiguration)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(cataloghardwaremanufacturerid), ::core::mem::transmute_copy(catalogstorecontentmodifierid), systemconfigurationexpiration, ::core::mem::transmute_copy(cataloghardwaredescriptor)).ok() })
    }
    pub fn SetMobileOperatorConfiguration(mobileoperatorid: &::windows_core::HSTRING, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows_core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetMobileOperatorConfiguration)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(mobileoperatorid), appdownloadlimitinmegabytes, updatedownloadlimitinmegabytes).ok() })
    }
    pub fn SetStoreWebAccountId(webaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetStoreWebAccountId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    pub fn IsStoreWebAccountId(webaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStoreWebAccountId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountid), &mut result__).from_abi(result__)
        })
    }
    pub fn HardwareManufacturerInfo() -> ::windows_core::Result<StoreHardwareManufacturerInfo> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareManufacturerInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FilterUnsupportedSystemFeaturesAsync<P0>(systemfeatures: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>,
    {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FilterUnsupportedSystemFeaturesAsync)(::windows_core::Interface::as_raw(this), systemfeatures.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PurchasePromptingPolicy() -> ::windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PurchasePromptingPolicy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetPurchasePromptingPolicy<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::IReference<u32>>,
    {
        Self::IStoreConfigurationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetPurchasePromptingPolicy)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() })
    }
    pub fn HasStoreWebAccount() -> ::windows_core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasStoreWebAccount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn HasStoreWebAccountForUser<P0>(user: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasStoreWebAccountForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetStoreLogDataAsync(options: StoreLogOptions) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStoreLogDataAsync)(::windows_core::Interface::as_raw(this), options, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn SetStoreWebAccountIdForUser<P0>(user: P0, webaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetStoreWebAccountIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn IsStoreWebAccountIdForUser<P0>(user: P0, webaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStoreWebAccountIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(webaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"System\"`"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetPurchasePromptingPolicyForUser<P0>(user: P0) -> ::windows_core::Result<super::super::super::Foundation::IReference<u32>>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPurchasePromptingPolicyForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"System\"`"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SetPurchasePromptingPolicyForUser<P0, P1>(user: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
        P1: ::windows_core::TryIntoParam<super::super::super::Foundation::IReference<u32>>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetPurchasePromptingPolicyForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), value.try_into_param()?.abi()).ok() })
    }
    pub fn GetStoreWebAccountId() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStoreWebAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn GetStoreWebAccountIdForUser<P0>(user: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStoreWebAccountIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn SetEnterpriseStoreWebAccountId(webaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).SetEnterpriseStoreWebAccountId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn SetEnterpriseStoreWebAccountIdForUser<P0>(user: P0, webaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).SetEnterpriseStoreWebAccountIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    pub fn GetEnterpriseStoreWebAccountId() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnterpriseStoreWebAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn GetEnterpriseStoreWebAccountIdForUser<P0>(user: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnterpriseStoreWebAccountIdForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn ShouldRestrictToEnterpriseStoreOnly() -> ::windows_core::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldRestrictToEnterpriseStoreOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn ShouldRestrictToEnterpriseStoreOnlyForUser<P0>(user: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldRestrictToEnterpriseStoreOnlyForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn IsPinToDesktopSupported() -> ::windows_core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPinToDesktopSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsPinToTaskbarSupported() -> ::windows_core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPinToTaskbarSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsPinToStartSupported() -> ::windows_core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPinToStartSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PinToDesktop(apppackagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).PinToDesktop)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(apppackagefamilyname)).ok() })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn PinToDesktopForUser<P0>(user: P0, apppackagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::User>,
    {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).PinToDesktopForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(apppackagefamilyname)).ok() })
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics<R, F: FnOnce(&IStoreConfigurationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics2<R, F: FnOnce(&IStoreConfigurationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics3<R, F: FnOnce(&IStoreConfigurationStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics4<R, F: FnOnce(&IStoreConfigurationStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics5<R, F: FnOnce(&IStoreConfigurationStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StoreConfiguration, IStoreConfigurationStatics5> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for StoreConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreConfiguration";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StoreHardwareManufacturerInfo(::windows_core::IUnknown);
impl StoreHardwareManufacturerInfo {
    pub fn HardwareManufacturerId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareManufacturerId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StoreContentModifierId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StoreContentModifierId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ModelName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModelName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ManufacturerName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManufacturerName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for StoreHardwareManufacturerInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for StoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StoreHardwareManufacturerInfo {
    const IID: ::windows_core::GUID = <IStoreHardwareManufacturerInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StoreHardwareManufacturerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo";
}
::windows_core::imp::interface_hierarchy!(StoreHardwareManufacturerInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for StoreHardwareManufacturerInfo {}
unsafe impl ::core::marker::Sync for StoreHardwareManufacturerInfo {}
pub struct StorePreview;
impl StorePreview {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestProductPurchaseByProductIdAndSkuIdAsync(productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestProductPurchaseByProductIdAndSkuIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadAddOnProductInfosAsync() -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadAddOnProductInfosAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorePreview<R, F: FnOnce(&IStorePreview) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StorePreview, IStorePreview> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for StorePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreview";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StorePreviewProductInfo(::windows_core::IUnknown);
impl StorePreviewProductInfo {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProductType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SkuInfoList(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkuInfoList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for StorePreviewProductInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for StorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorePreviewProductInfo {
    const IID: ::windows_core::GUID = <IStorePreviewProductInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorePreviewProductInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo";
}
::windows_core::imp::interface_hierarchy!(StorePreviewProductInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for StorePreviewProductInfo {}
unsafe impl ::core::marker::Sync for StorePreviewProductInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StorePreviewPurchaseResults(::windows_core::IUnknown);
impl StorePreviewPurchaseResults {
    pub fn ProductPurchaseStatus(&self) -> ::windows_core::Result<StorePreviewProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductPurchaseStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for StorePreviewPurchaseResults {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for StorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorePreviewPurchaseResults {
    const IID: ::windows_core::GUID = <IStorePreviewPurchaseResults as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorePreviewPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults";
}
::windows_core::imp::interface_hierarchy!(StorePreviewPurchaseResults, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for StorePreviewPurchaseResults {}
unsafe impl ::core::marker::Sync for StorePreviewPurchaseResults {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StorePreviewSkuInfo(::windows_core::IUnknown);
impl StorePreviewSkuInfo {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SkuId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkuId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SkuType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkuType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CustomDeveloperData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomDeveloperData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrencyCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FormattedListPrice(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormattedListPrice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for StorePreviewSkuInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for StorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorePreviewSkuInfo {
    const IID: ::windows_core::GUID = <IStorePreviewSkuInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorePreviewSkuInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo";
}
::windows_core::imp::interface_hierarchy!(StorePreviewSkuInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for StorePreviewSkuInfo {}
unsafe impl ::core::marker::Sync for StorePreviewSkuInfo {}
pub struct WebAuthenticationCoreManagerHelper;
impl WebAuthenticationCoreManagerHelper {
    #[doc = "Required features: `\"Foundation\"`, `\"Security_Authentication_Web_Core\"`, `\"UI_Xaml\"`"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAsync<P0, P1>(request: P0, uielement: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>,
        P1: ::windows_core::TryIntoParam<super::super::super::UI::Xaml::UIElement>,
    {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestTokenWithUIElementHostingAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), uielement.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`, `\"UI_Xaml\"`"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAndWebAccountAsync<P0, P1, P2>(request: P0, webaccount: P1, uielement: P2) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Security::Authentication::Web::Core::WebTokenRequest>,
        P1: ::windows_core::IntoParam<super::super::super::Security::Credentials::WebAccount>,
        P2: ::windows_core::TryIntoParam<super::super::super::UI::Xaml::UIElement>,
    {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestTokenWithUIElementHostingAndWebAccountAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), webaccount.into_param().abi(), uielement.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerHelper<R, F: FnOnce(&IWebAuthenticationCoreManagerHelper) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAuthenticationCoreManagerHelper, IWebAuthenticationCoreManagerHelper> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for WebAuthenticationCoreManagerHelper {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: Self = Self(0i32);
    pub const HttpOnly: Self = Self(1i32);
    pub const Lan: Self = Self(2i32);
    pub const Group: Self = Self(3i32);
    pub const Internet: Self = Self(4i32);
    pub const Bypass: Self = Self(5i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadMode {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeliveryOptimizationDownloadMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeliveryOptimizationDownloadMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeliveryOptimizationDownloadMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeliveryOptimizationDownloadMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: Self = Self(0i32);
    pub const Policy: Self = Self(1i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadModeSource {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadModeSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeliveryOptimizationDownloadModeSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeliveryOptimizationDownloadModeSource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeliveryOptimizationDownloadModeSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadModeSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeliveryOptimizationDownloadModeSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadModeSource;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreLogOptions(pub u32);
impl StoreLogOptions {
    pub const None: Self = Self(0u32);
    pub const TryElevate: Self = Self(1u32);
}
impl ::core::marker::Copy for StoreLogOptions {}
impl ::core::clone::Clone for StoreLogOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreLogOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StoreLogOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StoreLogOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreLogOptions").field(&self.0).finish()
    }
}
impl StoreLogOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for StoreLogOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StoreLogOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StoreLogOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StoreLogOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StoreLogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for StoreLogOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreLogOptions;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl ::core::marker::Copy for StorePreviewProductPurchaseStatus {}
impl ::core::clone::Clone for StorePreviewProductPurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorePreviewProductPurchaseStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StorePreviewProductPurchaseStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StorePreviewProductPurchaseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewProductPurchaseStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorePreviewProductPurchaseStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StorePreviewProductPurchaseStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: Self = Self(0i32);
    pub const ArchitectureX64: Self = Self(1i32);
    pub const ArchitectureArm: Self = Self(2i32);
    pub const DirectX9: Self = Self(3i32);
    pub const DirectX10: Self = Self(4i32);
    pub const DirectX11: Self = Self(5i32);
    pub const D3D12HardwareFL11: Self = Self(6i32);
    pub const D3D12HardwareFL12: Self = Self(7i32);
    pub const Memory300MB: Self = Self(8i32);
    pub const Memory750MB: Self = Self(9i32);
    pub const Memory1GB: Self = Self(10i32);
    pub const Memory2GB: Self = Self(11i32);
    pub const CameraFront: Self = Self(12i32);
    pub const CameraRear: Self = Self(13i32);
    pub const Gyroscope: Self = Self(14i32);
    pub const Hover: Self = Self(15i32);
    pub const Magnetometer: Self = Self(16i32);
    pub const Nfc: Self = Self(17i32);
    pub const Resolution720P: Self = Self(18i32);
    pub const ResolutionWvga: Self = Self(19i32);
    pub const ResolutionWvgaOr720P: Self = Self(20i32);
    pub const ResolutionWxga: Self = Self(21i32);
    pub const ResolutionWvgaOrWxga: Self = Self(22i32);
    pub const ResolutionWxgaOr720P: Self = Self(23i32);
    pub const Memory4GB: Self = Self(24i32);
    pub const Memory6GB: Self = Self(25i32);
    pub const Memory8GB: Self = Self(26i32);
    pub const Memory12GB: Self = Self(27i32);
    pub const Memory16GB: Self = Self(28i32);
    pub const Memory20GB: Self = Self(29i32);
    pub const VideoMemory2GB: Self = Self(30i32);
    pub const VideoMemory4GB: Self = Self(31i32);
    pub const VideoMemory6GB: Self = Self(32i32);
    pub const VideoMemory1GB: Self = Self(33i32);
    pub const ArchitectureArm64: Self = Self(34i32);
}
impl ::core::marker::Copy for StoreSystemFeature {}
impl ::core::clone::Clone for StoreSystemFeature {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreSystemFeature {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StoreSystemFeature {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StoreSystemFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSystemFeature").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StoreSystemFeature {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreSystemFeature;i4)");
}
