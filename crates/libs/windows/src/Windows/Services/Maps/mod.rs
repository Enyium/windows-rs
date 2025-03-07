#[cfg(feature = "Services_Maps_Guidance")]
#[doc = "Required features: `\"Services_Maps_Guidance\"`"]
pub mod Guidance;
#[cfg(feature = "Services_Maps_LocalSearch")]
#[doc = "Required features: `\"Services_Maps_LocalSearch\"`"]
pub mod LocalSearch;
#[cfg(feature = "Services_Maps_OfflineMaps")]
#[doc = "Required features: `\"Services_Maps_OfflineMaps\"`"]
pub mod OfflineMaps;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnhancedWaypoint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnhancedWaypoint {
    type Vtable = IEnhancedWaypoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnhancedWaypoint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed268c74_5913_11e6_8b77_86f30ca893d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedWaypoint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WaypointKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnhancedWaypointFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnhancedWaypointFactory {
    type Vtable = IEnhancedWaypointFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnhancedWaypointFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf868477_a2aa_46dd_b645_23b31b8aa6c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedWaypointFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, point: *mut ::core::ffi::c_void, kind: WaypointKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManeuverWarning(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManeuverWarning {
    type Vtable = IManeuverWarning_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManeuverWarning {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1a36d8a_2630_4378_9e4a_6e44253dceba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManeuverWarning_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManeuverWarningKind) -> ::windows_core::HRESULT,
    pub Severity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ManeuverWarningSeverity) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapAddress {
    type Vtable = IMapAddress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapAddress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfa7a973_a3b4_4494_b3ff_cba94db69699);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapAddress_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BuildingName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BuildingFloor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BuildingRoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BuildingWing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StreetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Street: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Neighborhood: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub District: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Town: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Region: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RegionCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Continent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapAddress2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapAddress2 {
    type Vtable = IMapAddress2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapAddress2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75cd6df1_e5ad_45a9_bf40_6cf256c1dd13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapAddress2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FormattedAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapLocation {
    type Vtable = IMapLocation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapLocation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c073f57_0da4_42e8_9ee2_a96fcf2371dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapLocationFinderResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapLocationFinderResult {
    type Vtable = IMapLocationFinderResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapLocationFinderResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43f1f179_e8cc_45f6_bed2_54ccbf965d9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Locations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Locations: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapLocationFinderStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapLocationFinderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapLocationFinderStatics {
    type Vtable = IMapLocationFinderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapLocationFinderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x318adb5d_1c5d_4f35_a2df_aaca94959517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAtAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querypoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAtAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchtext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, referencepoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsWithMaxCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchtext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, referencepoint: *mut ::core::ffi::c_void, maxcount: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsWithMaxCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapLocationFinderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapLocationFinderStatics2 {
    type Vtable = IMapLocationFinderStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapLocationFinderStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x959a8b96_6485_4dfd_851a_33ac317e3af6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapLocationFinderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocationsAtWithAccuracyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querypoint: *mut ::core::ffi::c_void, accuracy: MapLocationDesiredAccuracy, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocationsAtWithAccuracyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapManagerStatics {
    type Vtable = IMapManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x37e3e515_82b4_4d54_8fd9_af2624b3011c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowDownloadedMapsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShowMapsUpdateUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRoute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRoute {
    type Vtable = IMapRoute_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRoute {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb07b732_584d_4583_9c60_641fea274349);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedDuration: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Legs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Legs: usize,
    pub IsTrafficBased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRoute2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRoute2 {
    type Vtable = IMapRoute2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRoute2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1c5d40c_2213_4ab0_a260_46b38169beac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ViolatedRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteRestrictions) -> ::windows_core::HRESULT,
    pub HasBlockedRoads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRoute3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRoute3 {
    type Vtable = IMapRoute3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRoute3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x858d1eae_f2ad_429f_bb37_cd21094ffc92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DurationWithoutTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationWithoutTraffic: usize,
    pub TrafficCongestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TrafficCongestion) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRoute4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRoute4 {
    type Vtable = IMapRoute4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRoute4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x366c8ca5_3053_4fa1_80ff_d475f3ed1e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRoute4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsScenic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteDrivingOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteDrivingOptions {
    type Vtable = IMapRouteDrivingOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteDrivingOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6815364d_c6dc_4697_a452_b18f8f0b67a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteDrivingOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxAlternateRouteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxAlternateRouteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InitialHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitialHeading: usize,
    #[cfg(feature = "Foundation")]
    pub SetInitialHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInitialHeading: usize,
    pub RouteOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteOptimization) -> ::windows_core::HRESULT,
    pub SetRouteOptimization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapRouteOptimization) -> ::windows_core::HRESULT,
    pub RouteRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteRestrictions) -> ::windows_core::HRESULT,
    pub SetRouteRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapRouteRestrictions) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteDrivingOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteDrivingOptions2 {
    type Vtable = IMapRouteDrivingOptions2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteDrivingOptions2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35dc8670_c298_48d0_b5ad_825460645603);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteDrivingOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DepartureTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DepartureTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDepartureTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDepartureTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteFinderResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteFinderResult {
    type Vtable = IMapRouteFinderResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteFinderResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa868a31a_9422_46ac_8ca1_b1614d4bfbe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteFinderStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteFinderResult2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteFinderResult2 {
    type Vtable = IMapRouteFinderResult2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteFinderResult2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20709c6d_d90c_46c8_91c6_7d4be4efb215);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderResult2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateRoutes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteFinderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteFinderStatics {
    type Vtable = IMapRouteFinderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteFinderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8a5c50f_1c64_4c3a_81eb_1f7c152afbbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationAndRestrictionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationAndRestrictionsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsAndOptimizationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsAndOptimizationAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetWalkingRouteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetWalkingRouteAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub GetWalkingRouteFromWaypointsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation_Collections")))]
    GetWalkingRouteFromWaypointsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteFinderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteFinderStatics2 {
    type Vtable = IMapRouteFinderStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteFinderStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafcc2c73_7760_49af_b3bd_baf135b703e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub GetDrivingRouteWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut ::core::ffi::c_void, endpoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    GetDrivingRouteWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteFinderStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteFinderStatics3 {
    type Vtable = IMapRouteFinderStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteFinderStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6098134_5913_11e6_8b77_86f30ca893d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteFinderStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDrivingRouteFromEnhancedWaypointsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDrivingRouteFromEnhancedWaypointsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waypoints: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteLeg(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteLeg {
    type Vtable = IMapRouteLeg_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteLeg {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96f8b2f6_5bba_4d17_9db6_1a263fec7471);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteLeg_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EstimatedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EstimatedDuration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Maneuvers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Maneuvers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteLeg2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteLeg2 {
    type Vtable = IMapRouteLeg2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteLeg2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02e2062d_c9c6_45b8_8e54_1a10b57a17e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteLeg2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DurationWithoutTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationWithoutTraffic: usize,
    pub TrafficCongestion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TrafficCongestion) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteManeuver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteManeuver {
    type Vtable = IMapRouteManeuver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteManeuver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed5c17f0_a6ab_4d65_a086_fa8a7e340df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub StartingPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    StartingPoint: usize,
    pub LengthInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub InstructionText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapRouteManeuverKind) -> ::windows_core::HRESULT,
    pub ExitNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ManeuverNotices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapManeuverNotices) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteManeuver2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteManeuver2 {
    type Vtable = IMapRouteManeuver2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteManeuver2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d7bcd9c_7c9b_41df_838b_eae21e4b05a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub EndHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub StreetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapRouteManeuver3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapRouteManeuver3 {
    type Vtable = IMapRouteManeuver3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapRouteManeuver3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6a138df_0483_4166_85be_b99336c11875);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapRouteManeuver3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Warnings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Warnings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapServiceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapServiceStatics {
    type Vtable = IMapServiceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapServiceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0144ad85_c04c_4cdd_871a_a0726d097cd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetServiceToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapServiceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapServiceStatics2 {
    type Vtable = IMapServiceStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapServiceStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8193eed_9c85_40a9_8896_0fc3fd2b7c2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WorldViewRegionCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapServiceStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapServiceStatics3 {
    type Vtable = IMapServiceStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapServiceStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a11ce20_63a7_4854_b355_d6dcda223d1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DataAttributions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMapServiceStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMapServiceStatics4 {
    type Vtable = IMapServiceStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMapServiceStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x088a2862_6abc_420e_945f_4cfd89c67356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapServiceStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDataUsagePreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MapServiceDataUsagePreference) -> ::windows_core::HRESULT,
    pub DataUsagePreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MapServiceDataUsagePreference) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaceInfo {
    type Vtable = IPlaceInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaceInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a0810b6_31c8_4f6a_9f18_950b4c38951a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPreferredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPreferredPlacement: usize,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Geoshape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Geoshape: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaceInfoCreateOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaceInfoCreateOptions {
    type Vtable = IPlaceInfoCreateOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaceInfoCreateOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd33c125_67f1_4bb3_9907_ecce939b0399);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoCreateOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaceInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaceInfoStatics {
    type Vtable = IPlaceInfoStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaceInfoStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82b9ff71_6cd0_48a4_afd9_5ed82097936b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referencepoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Create: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateWithGeopointAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referencepoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateWithGeopointAndOptions: usize,
    pub CreateFromIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub CreateFromIdentifierWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::std::mem::MaybeUninit<::windows_core::HSTRING>, defaultpoint: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    CreateFromIdentifierWithOptions: usize,
    pub CreateFromMapLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsShowSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaceInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaceInfoStatics2 {
    type Vtable = IPlaceInfoStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaceInfoStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x730f0249_4047_44a3_8f81_2550a5216370);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromAddressWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayaddress: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct EnhancedWaypoint(::windows_core::IUnknown);
impl EnhancedWaypoint {
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WaypointKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Create<P0>(point: P0, kind: WaypointKind) -> ::windows_core::Result<EnhancedWaypoint>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IEnhancedWaypointFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), point.into_param().abi(), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnhancedWaypointFactory<R, F: FnOnce(&IEnhancedWaypointFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<EnhancedWaypoint, IEnhancedWaypointFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for EnhancedWaypoint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for EnhancedWaypoint {
    type Vtable = IEnhancedWaypoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EnhancedWaypoint {
    const IID: ::windows_core::GUID = <IEnhancedWaypoint as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EnhancedWaypoint {
    const NAME: &'static str = "Windows.Services.Maps.EnhancedWaypoint";
}
::windows_core::imp::interface_hierarchy!(EnhancedWaypoint, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for EnhancedWaypoint {}
unsafe impl ::core::marker::Sync for EnhancedWaypoint {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManeuverWarning(::windows_core::IUnknown);
impl ManeuverWarning {
    pub fn Kind(&self) -> ::windows_core::Result<ManeuverWarningKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Severity(&self) -> ::windows_core::Result<ManeuverWarningSeverity> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Severity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ManeuverWarning {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManeuverWarning {
    type Vtable = IManeuverWarning_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManeuverWarning {
    const IID: ::windows_core::GUID = <IManeuverWarning as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManeuverWarning {
    const NAME: &'static str = "Windows.Services.Maps.ManeuverWarning";
}
::windows_core::imp::interface_hierarchy!(ManeuverWarning, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ManeuverWarning {}
unsafe impl ::core::marker::Sync for ManeuverWarning {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapAddress(::windows_core::IUnknown);
impl MapAddress {
    pub fn BuildingName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BuildingName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BuildingFloor(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BuildingFloor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BuildingRoom(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BuildingRoom)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BuildingWing(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BuildingWing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StreetNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreetNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Street(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Street)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Neighborhood(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Neighborhood)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn District(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).District)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Town(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Town)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Region(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Region)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RegionCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegionCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Country(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Country)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CountryCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CountryCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PostCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PostCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Continent(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Continent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FormattedAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMapAddress2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormattedAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapAddress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapAddress {
    type Vtable = IMapAddress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapAddress {
    const IID: ::windows_core::GUID = <IMapAddress as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapAddress {
    const NAME: &'static str = "Windows.Services.Maps.MapAddress";
}
::windows_core::imp::interface_hierarchy!(MapAddress, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapAddress {}
unsafe impl ::core::marker::Sync for MapAddress {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapLocation(::windows_core::IUnknown);
impl MapLocation {
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Address(&self) -> ::windows_core::Result<MapAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapLocation {
    type Vtable = IMapLocation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapLocation {
    const IID: ::windows_core::GUID = <IMapLocation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapLocation {
    const NAME: &'static str = "Windows.Services.Maps.MapLocation";
}
::windows_core::imp::interface_hierarchy!(MapLocation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapLocation {}
unsafe impl ::core::marker::Sync for MapLocation {}
pub struct MapLocationFinder;
impl MapLocationFinder {
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsAtAsync<P0>(querypoint: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindLocationsAtAsync)(::windows_core::Interface::as_raw(this), querypoint.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsAsync<P0>(searchtext: &::windows_core::HSTRING, referencepoint: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindLocationsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(searchtext), referencepoint.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsWithMaxCountAsync<P0>(searchtext: &::windows_core::HSTRING, referencepoint: P0, maxcount: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapLocationFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindLocationsWithMaxCountAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(searchtext), referencepoint.into_param().abi(), maxcount, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocationsAtWithAccuracyAsync<P0>(querypoint: P0, accuracy: MapLocationDesiredAccuracy) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapLocationFinderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindLocationsAtWithAccuracyAsync)(::windows_core::Interface::as_raw(this), querypoint.into_param().abi(), accuracy, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapLocationFinderStatics<R, F: FnOnce(&IMapLocationFinderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapLocationFinder, IMapLocationFinderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapLocationFinderStatics2<R, F: FnOnce(&IMapLocationFinderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapLocationFinder, IMapLocationFinderStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MapLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.MapLocationFinder";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapLocationFinderResult(::windows_core::IUnknown);
impl MapLocationFinderResult {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Locations(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MapLocation>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Locations)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<MapLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapLocationFinderResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapLocationFinderResult {
    type Vtable = IMapLocationFinderResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapLocationFinderResult {
    const IID: ::windows_core::GUID = <IMapLocationFinderResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.MapLocationFinderResult";
}
::windows_core::imp::interface_hierarchy!(MapLocationFinderResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapLocationFinderResult {}
unsafe impl ::core::marker::Sync for MapLocationFinderResult {}
pub struct MapManager;
impl MapManager {
    pub fn ShowDownloadedMapsUI() -> ::windows_core::Result<()> {
        Self::IMapManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowDownloadedMapsUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ShowMapsUpdateUI() -> ::windows_core::Result<()> {
        Self::IMapManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowMapsUpdateUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn IMapManagerStatics<R, F: FnOnce(&IMapManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapManager, IMapManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MapManager {
    const NAME: &'static str = "Windows.Services.Maps.MapManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapRoute(::windows_core::IUnknown);
impl MapRoute {
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn BoundingBox(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundingBox)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LengthInMeters(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LengthInMeters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EstimatedDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EstimatedDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Legs(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MapRouteLeg>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Legs)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTrafficBased(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTrafficBased)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ViolatedRestrictions(&self) -> ::windows_core::Result<MapRouteRestrictions> {
        let this = &::windows_core::ComInterface::cast::<IMapRoute2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViolatedRestrictions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasBlockedRoads(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMapRoute2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasBlockedRoads)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DurationWithoutTraffic(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMapRoute3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DurationWithoutTraffic)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrafficCongestion(&self) -> ::windows_core::Result<TrafficCongestion> {
        let this = &::windows_core::ComInterface::cast::<IMapRoute3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrafficCongestion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsScenic(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMapRoute4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsScenic)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapRoute {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapRoute {
    type Vtable = IMapRoute_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapRoute {
    const IID: ::windows_core::GUID = <IMapRoute as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapRoute {
    const NAME: &'static str = "Windows.Services.Maps.MapRoute";
}
::windows_core::imp::interface_hierarchy!(MapRoute, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapRoute {}
unsafe impl ::core::marker::Sync for MapRoute {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapRouteDrivingOptions(::windows_core::IUnknown);
impl MapRouteDrivingOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapRouteDrivingOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxAlternateRouteCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxAlternateRouteCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxAlternateRouteCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxAlternateRouteCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InitialHeading(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitialHeading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetInitialHeading<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<f64>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitialHeading)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn RouteOptimization(&self) -> ::windows_core::Result<MapRouteOptimization> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RouteOptimization)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRouteOptimization(&self, value: MapRouteOptimization) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRouteOptimization)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RouteRestrictions(&self) -> ::windows_core::Result<MapRouteRestrictions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RouteRestrictions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRouteRestrictions(&self, value: MapRouteRestrictions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRouteRestrictions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DepartureTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteDrivingOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DepartureTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetDepartureTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMapRouteDrivingOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDepartureTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for MapRouteDrivingOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapRouteDrivingOptions {
    type Vtable = IMapRouteDrivingOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapRouteDrivingOptions {
    const IID: ::windows_core::GUID = <IMapRouteDrivingOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapRouteDrivingOptions {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteDrivingOptions";
}
::windows_core::imp::interface_hierarchy!(MapRouteDrivingOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapRouteDrivingOptions {}
unsafe impl ::core::marker::Sync for MapRouteDrivingOptions {}
pub struct MapRouteFinder;
impl MapRouteFinder {
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteAsync<P0, P1>(startpoint: P0, endpoint: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteAsync)(::windows_core::Interface::as_raw(this), startpoint.into_param().abi(), endpoint.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptimizationAsync<P0, P1>(startpoint: P0, endpoint: P1, optimization: MapRouteOptimization) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteWithOptimizationAsync)(::windows_core::Interface::as_raw(this), startpoint.into_param().abi(), endpoint.into_param().abi(), optimization, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptimizationAndRestrictionsAsync<P0, P1>(startpoint: P0, endpoint: P1, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteWithOptimizationAndRestrictionsAsync)(::windows_core::Interface::as_raw(this), startpoint.into_param().abi(), endpoint.into_param().abi(), optimization, restrictions, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync<P0, P1>(startpoint: P0, endpoint: P1, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync)(::windows_core::Interface::as_raw(this), startpoint.into_param().abi(), endpoint.into_param().abi(), optimization, restrictions, headingindegrees, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsAsync<P0>(waypoints: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteFromWaypointsAsync)(::windows_core::Interface::as_raw(this), waypoints.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsAndOptimizationAsync<P0>(waypoints: P0, optimization: MapRouteOptimization) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteFromWaypointsAndOptimizationAsync)(::windows_core::Interface::as_raw(this), waypoints.try_into_param()?.abi(), optimization, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync<P0>(waypoints: P0, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync)(::windows_core::Interface::as_raw(this), waypoints.try_into_param()?.abi(), optimization, restrictions, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync<P0>(waypoints: P0, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync)(::windows_core::Interface::as_raw(this), waypoints.try_into_param()?.abi(), optimization, restrictions, headingindegrees, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetWalkingRouteAsync<P0, P1>(startpoint: P0, endpoint: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetWalkingRouteAsync)(::windows_core::Interface::as_raw(this), startpoint.into_param().abi(), endpoint.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation_Collections"))]
    pub fn GetWalkingRouteFromWaypointsAsync<P0>(waypoints: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>,
    {
        Self::IMapRouteFinderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetWalkingRouteFromWaypointsAsync)(::windows_core::Interface::as_raw(this), waypoints.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn GetDrivingRouteWithOptionsAsync<P0, P1, P2>(startpoint: P0, endpoint: P1, options: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P2: ::windows_core::IntoParam<MapRouteDrivingOptions>,
    {
        Self::IMapRouteFinderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteWithOptionsAsync)(::windows_core::Interface::as_raw(this), startpoint.into_param().abi(), endpoint.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDrivingRouteFromEnhancedWaypointsAsync<P0>(waypoints: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>,
    {
        Self::IMapRouteFinderStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteFromEnhancedWaypointsAsync)(::windows_core::Interface::as_raw(this), waypoints.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync<P0, P1>(waypoints: P0, options: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>,
        P1: ::windows_core::IntoParam<MapRouteDrivingOptions>,
    {
        Self::IMapRouteFinderStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync)(::windows_core::Interface::as_raw(this), waypoints.try_into_param()?.abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapRouteFinderStatics<R, F: FnOnce(&IMapRouteFinderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapRouteFinder, IMapRouteFinderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapRouteFinderStatics2<R, F: FnOnce(&IMapRouteFinderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapRouteFinder, IMapRouteFinderStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapRouteFinderStatics3<R, F: FnOnce(&IMapRouteFinderStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapRouteFinder, IMapRouteFinderStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MapRouteFinder {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteFinder";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapRouteFinderResult(::windows_core::IUnknown);
impl MapRouteFinderResult {
    pub fn Route(&self) -> ::windows_core::Result<MapRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Route)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<MapRouteFinderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateRoutes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MapRoute>> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteFinderResult2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlternateRoutes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapRouteFinderResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapRouteFinderResult {
    type Vtable = IMapRouteFinderResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapRouteFinderResult {
    const IID: ::windows_core::GUID = <IMapRouteFinderResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapRouteFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteFinderResult";
}
::windows_core::imp::interface_hierarchy!(MapRouteFinderResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapRouteFinderResult {}
unsafe impl ::core::marker::Sync for MapRouteFinderResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapRouteLeg(::windows_core::IUnknown);
impl MapRouteLeg {
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn BoundingBox(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundingBox)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LengthInMeters(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LengthInMeters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EstimatedDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EstimatedDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Maneuvers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MapRouteManeuver>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Maneuvers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DurationWithoutTraffic(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteLeg2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DurationWithoutTraffic)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrafficCongestion(&self) -> ::windows_core::Result<TrafficCongestion> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteLeg2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrafficCongestion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapRouteLeg {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapRouteLeg {
    type Vtable = IMapRouteLeg_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapRouteLeg {
    const IID: ::windows_core::GUID = <IMapRouteLeg as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapRouteLeg {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteLeg";
}
::windows_core::imp::interface_hierarchy!(MapRouteLeg, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapRouteLeg {}
unsafe impl ::core::marker::Sync for MapRouteLeg {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MapRouteManeuver(::windows_core::IUnknown);
impl MapRouteManeuver {
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn StartingPoint(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartingPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LengthInMeters(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LengthInMeters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstructionText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstructionText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<MapRouteManeuverKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExitNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ManeuverNotices(&self) -> ::windows_core::Result<MapManeuverNotices> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManeuverNotices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartHeading(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartHeading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EndHeading(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndHeading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StreetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteManeuver2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreetName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Warnings(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ManeuverWarning>> {
        let this = &::windows_core::ComInterface::cast::<IMapRouteManeuver3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Warnings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MapRouteManeuver {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MapRouteManeuver {
    type Vtable = IMapRouteManeuver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MapRouteManeuver {
    const IID: ::windows_core::GUID = <IMapRouteManeuver as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MapRouteManeuver {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteManeuver";
}
::windows_core::imp::interface_hierarchy!(MapRouteManeuver, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MapRouteManeuver {}
unsafe impl ::core::marker::Sync for MapRouteManeuver {}
pub struct MapService;
impl MapService {
    pub fn SetServiceToken(value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IMapServiceStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetServiceToken)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    pub fn ServiceToken() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMapServiceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceToken)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn WorldViewRegionCode() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMapServiceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WorldViewRegionCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DataAttributions() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMapServiceStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataAttributions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetDataUsagePreference(value: MapServiceDataUsagePreference) -> ::windows_core::Result<()> {
        Self::IMapServiceStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).SetDataUsagePreference)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn DataUsagePreference() -> ::windows_core::Result<MapServiceDataUsagePreference> {
        Self::IMapServiceStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataUsagePreference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics<R, F: FnOnce(&IMapServiceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapService, IMapServiceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics2<R, F: FnOnce(&IMapServiceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapService, IMapServiceStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics3<R, F: FnOnce(&IMapServiceStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapService, IMapServiceStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMapServiceStatics4<R, F: FnOnce(&IMapServiceStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MapService, IMapServiceStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MapService {
    const NAME: &'static str = "Windows.Services.Maps.MapService";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlaceInfo(::windows_core::IUnknown);
impl PlaceInfo {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Show(&self, selection: super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this), selection).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowWithPreferredPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowWithPreferredPlacement)(::windows_core::Interface::as_raw(this), selection, preferredplacement).ok() }
    }
    pub fn Identifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Geoshape(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::IGeoshape> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Geoshape)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Create<P0>(referencepoint: P0) -> ::windows_core::Result<PlaceInfo>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), referencepoint.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateWithGeopointAndOptions<P0, P1>(referencepoint: P0, options: P1) -> ::windows_core::Result<PlaceInfo>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<PlaceInfoCreateOptions>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithGeopointAndOptions)(::windows_core::Interface::as_raw(this), referencepoint.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromIdentifier(identifier: &::windows_core::HSTRING) -> ::windows_core::Result<PlaceInfo> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdentifier)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identifier), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn CreateFromIdentifierWithOptions<P0, P1>(identifier: &::windows_core::HSTRING, defaultpoint: P0, options: P1) -> ::windows_core::Result<PlaceInfo>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Geolocation::Geopoint>,
        P1: ::windows_core::IntoParam<PlaceInfoCreateOptions>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdentifierWithOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identifier), defaultpoint.into_param().abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromMapLocation<P0>(location: P0) -> ::windows_core::Result<PlaceInfo>
    where
        P0: ::windows_core::IntoParam<MapLocation>,
    {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMapLocation)(::windows_core::Interface::as_raw(this), location.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn IsShowSupported() -> ::windows_core::Result<bool> {
        Self::IPlaceInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShowSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromAddress(displayaddress: &::windows_core::HSTRING) -> ::windows_core::Result<PlaceInfo> {
        Self::IPlaceInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromAddress)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(displayaddress), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromAddressWithName(displayaddress: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<PlaceInfo> {
        Self::IPlaceInfoStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromAddressWithName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(displayaddress), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaceInfoStatics<R, F: FnOnce(&IPlaceInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlaceInfo, IPlaceInfoStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlaceInfoStatics2<R, F: FnOnce(&IPlaceInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlaceInfo, IPlaceInfoStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PlaceInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PlaceInfo {
    type Vtable = IPlaceInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaceInfo {
    const IID: ::windows_core::GUID = <IPlaceInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaceInfo {
    const NAME: &'static str = "Windows.Services.Maps.PlaceInfo";
}
::windows_core::imp::interface_hierarchy!(PlaceInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PlaceInfo {}
unsafe impl ::core::marker::Sync for PlaceInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlaceInfoCreateOptions(::windows_core::IUnknown);
impl PlaceInfoCreateOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlaceInfoCreateOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayAddress)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlaceInfoCreateOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PlaceInfoCreateOptions {
    type Vtable = IPlaceInfoCreateOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaceInfoCreateOptions {
    const IID: ::windows_core::GUID = <IPlaceInfoCreateOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaceInfoCreateOptions {
    const NAME: &'static str = "Windows.Services.Maps.PlaceInfoCreateOptions";
}
::windows_core::imp::interface_hierarchy!(PlaceInfoCreateOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PlaceInfoCreateOptions {}
unsafe impl ::core::marker::Sync for PlaceInfoCreateOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ManeuverWarningKind(pub i32);
impl ManeuverWarningKind {
    pub const None: Self = Self(0i32);
    pub const Accident: Self = Self(1i32);
    pub const AdministrativeDivisionChange: Self = Self(2i32);
    pub const Alert: Self = Self(3i32);
    pub const BlockedRoad: Self = Self(4i32);
    pub const CheckTimetable: Self = Self(5i32);
    pub const Congestion: Self = Self(6i32);
    pub const Construction: Self = Self(7i32);
    pub const CountryChange: Self = Self(8i32);
    pub const DisabledVehicle: Self = Self(9i32);
    pub const GateAccess: Self = Self(10i32);
    pub const GetOffTransit: Self = Self(11i32);
    pub const GetOnTransit: Self = Self(12i32);
    pub const IllegalUTurn: Self = Self(13i32);
    pub const MassTransit: Self = Self(14i32);
    pub const Miscellaneous: Self = Self(15i32);
    pub const NoIncident: Self = Self(16i32);
    pub const Other: Self = Self(17i32);
    pub const OtherNews: Self = Self(18i32);
    pub const OtherTrafficIncidents: Self = Self(19i32);
    pub const PlannedEvent: Self = Self(20i32);
    pub const PrivateRoad: Self = Self(21i32);
    pub const RestrictedTurn: Self = Self(22i32);
    pub const RoadClosures: Self = Self(23i32);
    pub const RoadHazard: Self = Self(24i32);
    pub const ScheduledConstruction: Self = Self(25i32);
    pub const SeasonalClosures: Self = Self(26i32);
    pub const Tollbooth: Self = Self(27i32);
    pub const TollRoad: Self = Self(28i32);
    pub const TollZoneEnter: Self = Self(29i32);
    pub const TollZoneExit: Self = Self(30i32);
    pub const TrafficFlow: Self = Self(31i32);
    pub const TransitLineChange: Self = Self(32i32);
    pub const UnpavedRoad: Self = Self(33i32);
    pub const UnscheduledConstruction: Self = Self(34i32);
    pub const Weather: Self = Self(35i32);
}
impl ::core::marker::Copy for ManeuverWarningKind {}
impl ::core::clone::Clone for ManeuverWarningKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManeuverWarningKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ManeuverWarningKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ManeuverWarningKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarningKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ManeuverWarningKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.ManeuverWarningKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ManeuverWarningSeverity(pub i32);
impl ManeuverWarningSeverity {
    pub const None: Self = Self(0i32);
    pub const LowImpact: Self = Self(1i32);
    pub const Minor: Self = Self(2i32);
    pub const Moderate: Self = Self(3i32);
    pub const Serious: Self = Self(4i32);
}
impl ::core::marker::Copy for ManeuverWarningSeverity {}
impl ::core::clone::Clone for ManeuverWarningSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManeuverWarningSeverity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ManeuverWarningSeverity {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ManeuverWarningSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManeuverWarningSeverity").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ManeuverWarningSeverity {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.ManeuverWarningSeverity;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapLocationDesiredAccuracy(pub i32);
impl MapLocationDesiredAccuracy {
    pub const High: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for MapLocationDesiredAccuracy {}
impl ::core::clone::Clone for MapLocationDesiredAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapLocationDesiredAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapLocationDesiredAccuracy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapLocationDesiredAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationDesiredAccuracy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MapLocationDesiredAccuracy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapLocationDesiredAccuracy;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapLocationFinderStatus(pub i32);
impl MapLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const BadLocation: Self = Self(3i32);
    pub const IndexFailure: Self = Self(4i32);
    pub const NetworkFailure: Self = Self(5i32);
    pub const NotSupported: Self = Self(6i32);
}
impl ::core::marker::Copy for MapLocationFinderStatus {}
impl ::core::clone::Clone for MapLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapLocationFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapLocationFinderStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapLocationFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapLocationFinderStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MapLocationFinderStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapLocationFinderStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapManeuverNotices(pub u32);
impl MapManeuverNotices {
    pub const None: Self = Self(0u32);
    pub const Toll: Self = Self(1u32);
    pub const Unpaved: Self = Self(2u32);
}
impl ::core::marker::Copy for MapManeuverNotices {}
impl ::core::clone::Clone for MapManeuverNotices {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapManeuverNotices {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapManeuverNotices {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapManeuverNotices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapManeuverNotices").field(&self.0).finish()
    }
}
impl MapManeuverNotices {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MapManeuverNotices {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MapManeuverNotices {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MapManeuverNotices {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MapManeuverNotices {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MapManeuverNotices {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for MapManeuverNotices {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapManeuverNotices;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteFinderStatus(pub i32);
impl MapRouteFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NoRouteFound: Self = Self(3i32);
    pub const NoRouteFoundWithGivenOptions: Self = Self(4i32);
    pub const StartPointNotFound: Self = Self(5i32);
    pub const EndPointNotFound: Self = Self(6i32);
    pub const NoPedestrianRouteFound: Self = Self(7i32);
    pub const NetworkFailure: Self = Self(8i32);
    pub const NotSupported: Self = Self(9i32);
}
impl ::core::marker::Copy for MapRouteFinderStatus {}
impl ::core::clone::Clone for MapRouteFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapRouteFinderStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapRouteFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteFinderStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MapRouteFinderStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteFinderStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteManeuverKind(pub i32);
impl MapRouteManeuverKind {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Stopover: Self = Self(2i32);
    pub const StopoverResume: Self = Self(3i32);
    pub const End: Self = Self(4i32);
    pub const GoStraight: Self = Self(5i32);
    pub const UTurnLeft: Self = Self(6i32);
    pub const UTurnRight: Self = Self(7i32);
    pub const TurnKeepLeft: Self = Self(8i32);
    pub const TurnKeepRight: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLightRight: Self = Self(11i32);
    pub const TurnLeft: Self = Self(12i32);
    pub const TurnRight: Self = Self(13i32);
    pub const TurnHardLeft: Self = Self(14i32);
    pub const TurnHardRight: Self = Self(15i32);
    pub const FreewayEnterLeft: Self = Self(16i32);
    pub const FreewayEnterRight: Self = Self(17i32);
    pub const FreewayLeaveLeft: Self = Self(18i32);
    pub const FreewayLeaveRight: Self = Self(19i32);
    pub const FreewayContinueLeft: Self = Self(20i32);
    pub const FreewayContinueRight: Self = Self(21i32);
    pub const TrafficCircleLeft: Self = Self(22i32);
    pub const TrafficCircleRight: Self = Self(23i32);
    pub const TakeFerry: Self = Self(24i32);
}
impl ::core::marker::Copy for MapRouteManeuverKind {}
impl ::core::clone::Clone for MapRouteManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteManeuverKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapRouteManeuverKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapRouteManeuverKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteManeuverKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MapRouteManeuverKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteManeuverKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteOptimization(pub i32);
impl MapRouteOptimization {
    pub const Time: Self = Self(0i32);
    pub const Distance: Self = Self(1i32);
    pub const TimeWithTraffic: Self = Self(2i32);
    pub const Scenic: Self = Self(3i32);
}
impl ::core::marker::Copy for MapRouteOptimization {}
impl ::core::clone::Clone for MapRouteOptimization {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteOptimization {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapRouteOptimization {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapRouteOptimization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteOptimization").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MapRouteOptimization {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteOptimization;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapRouteRestrictions(pub u32);
impl MapRouteRestrictions {
    pub const None: Self = Self(0u32);
    pub const Highways: Self = Self(1u32);
    pub const TollRoads: Self = Self(2u32);
    pub const Ferries: Self = Self(4u32);
    pub const Tunnels: Self = Self(8u32);
    pub const DirtRoads: Self = Self(16u32);
    pub const Motorail: Self = Self(32u32);
}
impl ::core::marker::Copy for MapRouteRestrictions {}
impl ::core::clone::Clone for MapRouteRestrictions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapRouteRestrictions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapRouteRestrictions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapRouteRestrictions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapRouteRestrictions").field(&self.0).finish()
    }
}
impl MapRouteRestrictions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MapRouteRestrictions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MapRouteRestrictions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MapRouteRestrictions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MapRouteRestrictions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MapRouteRestrictions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for MapRouteRestrictions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapRouteRestrictions;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MapServiceDataUsagePreference(pub i32);
impl MapServiceDataUsagePreference {
    pub const Default: Self = Self(0i32);
    pub const OfflineMapDataOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MapServiceDataUsagePreference {}
impl ::core::clone::Clone for MapServiceDataUsagePreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MapServiceDataUsagePreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MapServiceDataUsagePreference {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MapServiceDataUsagePreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MapServiceDataUsagePreference").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MapServiceDataUsagePreference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.MapServiceDataUsagePreference;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TrafficCongestion(pub i32);
impl TrafficCongestion {
    pub const Unknown: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Mild: Self = Self(2i32);
    pub const Medium: Self = Self(3i32);
    pub const Heavy: Self = Self(4i32);
}
impl ::core::marker::Copy for TrafficCongestion {}
impl ::core::clone::Clone for TrafficCongestion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TrafficCongestion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TrafficCongestion {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TrafficCongestion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TrafficCongestion").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TrafficCongestion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.TrafficCongestion;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WaypointKind(pub i32);
impl WaypointKind {
    pub const Stop: Self = Self(0i32);
    pub const Via: Self = Self(1i32);
}
impl ::core::marker::Copy for WaypointKind {}
impl ::core::clone::Clone for WaypointKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WaypointKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WaypointKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WaypointKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WaypointKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WaypointKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.WaypointKind;i4)");
}
