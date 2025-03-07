#[cfg(feature = "Devices_Geolocation_Geofencing")]
#[doc = "Required features: `\"Devices_Geolocation_Geofencing\"`"]
pub mod Geofencing;
#[cfg(feature = "Devices_Geolocation_Provider")]
#[doc = "Required features: `\"Devices_Geolocation_Provider\"`"]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICivicAddress(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICivicAddress {
    type Vtable = ICivicAddress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICivicAddress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8567a1a_64f4_4d48_bcea_f6b008eca34c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddress_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Country: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeoboundingBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoboundingBox {
    type Vtable = IGeoboundingBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeoboundingBox {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0896c80b_274f_43da_9a06_cbfcdaeb4ec2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NorthwestCorner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub SoutheastCorner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub MinAltitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MaxAltitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeoboundingBoxFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoboundingBoxFactory {
    type Vtable = IGeoboundingBoxFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeoboundingBoxFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4dfba589_0411_4abc_b3b5_5bbccb57d98c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeoboundingBoxStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoboundingBoxStatics {
    type Vtable = IGeoboundingBoxStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeoboundingBoxStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67b80708_e61a_4cd0_841b_93233792b5ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub TryCompute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryCompute: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altituderefsystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryComputeWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryComputeWithAltitudeReferenceAndSpatialReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocircle(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocircle {
    type Vtable = IGeocircle_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocircle {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39e45843_a7f9_4e63_92a7_ba0c28d124b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircle_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
    pub Radius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocircleFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocircleFactory {
    type Vtable = IGeocircleFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocircleFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafd6531f_72b1_4f7d_87cc_4ed4c9849c05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircleFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocoordinate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinate {
    type Vtable = IGeocoordinate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocoordinate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee21a3aa_976a_4c70_803d_083ea55bcbc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Latitude: usize,
    #[cfg(feature = "deprecated")]
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Longitude: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Altitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Altitude: usize,
    pub Accuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AltitudeAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AltitudeAccuracy: usize,
    #[cfg(feature = "Foundation")]
    pub Heading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Heading: usize,
    #[cfg(feature = "Foundation")]
    pub Speed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Speed: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocoordinateSatelliteData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocoordinateSatelliteData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc32a74d9_2608_474c_912c_06dd490f4af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PositionDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub HorizontalDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HorizontalDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub VerticalDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerticalDilutionOfPrecision: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocoordinateSatelliteData2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateSatelliteData2 {
    type Vtable = IGeocoordinateSatelliteData2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocoordinateSatelliteData2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x761c8cfd_a19d_5a51_80f5_71676115483e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GeometricDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GeometricDilutionOfPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub TimeDilutionOfPrecision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeDilutionOfPrecision: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocoordinateWithPoint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithPoint {
    type Vtable = IGeocoordinateWithPoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocoordinateWithPoint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfeea0525_d22c_4d46_b527_0b96066fc7db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPoint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocoordinateWithPositionData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithPositionData {
    type Vtable = IGeocoordinateWithPositionData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocoordinateWithPositionData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95e634be_dbd6_40ac_b8f2_a65c0340d9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PositionSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionSource) -> ::windows_core::HRESULT,
    pub SatelliteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocoordinateWithPositionSourceTimestamp(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithPositionSourceTimestamp {
    type Vtable = IGeocoordinateWithPositionSourceTimestamp_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocoordinateWithPositionSourceTimestamp {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8543fc02_c9f1_4610_afe0_8bc3a6a87036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionSourceTimestamp_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PositionSourceTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionSourceTimestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeocoordinateWithRemoteSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeocoordinateWithRemoteSource {
    type Vtable = IGeocoordinateWithRemoteSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeocoordinateWithRemoteSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x397cebd7_ee38_5f3b_8900_c4a7bc9cf953);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithRemoteSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsRemoteSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeolocator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocator {
    type Vtable = IGeolocator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeolocator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9c3bf62_4524_4989_8aa9_de019d2e551f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionAccuracy) -> ::windows_core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PositionAccuracy) -> ::windows_core::HRESULT,
    pub MovementThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMovementThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub LocationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetGeopositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGeopositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetGeopositionAsyncWithAgeAndTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGeopositionAsyncWithAgeAndTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeolocator2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocator2 {
    type Vtable = IGeolocator2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeolocator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1b42e6d_8891_43b4_ad36_27c6fe9a97b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowFallbackToConsentlessPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeolocatorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocatorStatics {
    type Vtable = IGeolocatorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeolocatorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a8e7571_2df5_4591_9f87_eb5fd894e9b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGeopositionHistoryWithDurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGeopositionHistoryWithDurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeolocatorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocatorStatics2 {
    type Vtable = IGeolocatorStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeolocatorStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x993011a2_fa1c_4631_a71d_0dbeb1250d9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsDefaultGeopositionRecommended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultGeoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultGeoposition: usize,
    #[cfg(feature = "Foundation")]
    pub DefaultGeoposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DefaultGeoposition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeolocatorWithScalarAccuracy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeolocatorWithScalarAccuracy {
    type Vtable = IGeolocatorWithScalarAccuracy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeolocatorWithScalarAccuracy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96f5d3c1_b80f_460a_994d_a96c47a51aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorWithScalarAccuracy_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredAccuracyInMeters: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredAccuracyInMeters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredAccuracyInMeters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeopath(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopath {
    type Vtable = IGeopath_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeopath {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe53fd7b9_2da4_4714_a652_de8593289898);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopath_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Positions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Positions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeopathFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopathFactory {
    type Vtable = IGeopathFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeopathFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27bea9c8_c7e7_4359_9b9b_fca3e05ef593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopathFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReference: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithAltitudeReferenceAndSpatialReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positions: *mut ::core::ffi::c_void, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithAltitudeReferenceAndSpatialReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeopoint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopoint {
    type Vtable = IGeopoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeopoint {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bfa00eb_e56e_49bb_9caf_cbaa78a8bcef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopoint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeopointFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeopointFactory {
    type Vtable = IGeopointFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeopointFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb6b8d33_76bd_4e30_8af7_a844dc37b7a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopointFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithAltitudeReferenceSystemAndSpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeoposition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoposition {
    type Vtable = IGeoposition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeoposition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc18d0454_7d41_4ff7_a957_9dffb4ef7f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Coordinate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CivicAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeoposition2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeoposition2 {
    type Vtable = IGeoposition2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeoposition2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f62f697_8671_4b0d_86f8_474a8496187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VenueData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeoshape(::windows_core::IUnknown);
impl IGeoshape {
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IGeoshape, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IGeoshape {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IGeoshape {
    type Vtable = IGeoshape_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeoshape {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc99ca2af_c729_43c1_8fab_d6dec914df7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoshape_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GeoshapeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows_core::HRESULT,
    pub SpatialReferenceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AltitudeReferenceSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeovisit(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisit {
    type Vtable = IGeovisit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeovisit {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1877a76_9ef6_41ab_a0dd_793ece76e2de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisit_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisitStateChange) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeovisitMonitor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitMonitor {
    type Vtable = IGeovisitMonitor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeovisitMonitor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80118aaf_5944_4591_83c1_396647f54f2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MonitoringScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VisitMonitoringScope) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VisitMonitoringScope) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VisitStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisitStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisitStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisitStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeovisitMonitorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitMonitorStatics {
    type Vtable = IGeovisitMonitorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeovisitMonitorStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcf976a7_bbf2_4cdd_95cf_554c82edfb87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetLastReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetLastReportAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeovisitStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeovisitStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xceb4d1ff_8b53_4968_beed_4cecd029ce15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Visit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeovisitTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeovisitTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea770d9e_d1c9_454b_99b7_b2f8cdd2482f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPositionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPositionChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x37859ce5_9d1e_46c5_bf3b_6ad8cac1a093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPositionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStatusChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3453d2da_8c93_4111_a205_9aecfc9be5c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVenueData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVenueData {
    type Vtable = IVenueData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVenueData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66f39187_60e3_4b2f_b527_4f53f1c3c677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVenueData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CivicAddress(::windows_core::IUnknown);
impl CivicAddress {
    pub fn Country(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Country)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn City(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).City)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PostalCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CivicAddress {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CivicAddress {
    type Vtable = ICivicAddress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CivicAddress {
    const IID: ::windows_core::GUID = <ICivicAddress as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.CivicAddress";
}
::windows_core::imp::interface_hierarchy!(CivicAddress, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CivicAddress {}
unsafe impl ::core::marker::Sync for CivicAddress {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeoboundingBox(::windows_core::IUnknown);
impl GeoboundingBox {
    pub fn NorthwestCorner(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NorthwestCorner)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SoutheastCorner(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoutheastCorner)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Center(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinAltitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinAltitude)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxAltitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxAltitude)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), northwestcorner, southeastcorner, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReference(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReference)(::windows_core::Interface::as_raw(this), northwestcorner, southeastcorner, altitudereferencesystem, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceAndSpatialReference(northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(::windows_core::Interface::as_raw(this), northwestcorner, southeastcorner, altitudereferencesystem, spatialreferenceid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryCompute<P0>(positions: P0) -> ::windows_core::Result<GeoboundingBox>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCompute)(::windows_core::Interface::as_raw(this), positions.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReference<P0>(positions: P0, altituderefsystem: AltitudeReferenceSystem) -> ::windows_core::Result<GeoboundingBox>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeWithAltitudeReference)(::windows_core::Interface::as_raw(this), positions.try_into_param()?.abi(), altituderefsystem, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReferenceAndSpatialReference<P0>(positions: P0, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<GeoboundingBox>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryComputeWithAltitudeReferenceAndSpatialReference)(::windows_core::Interface::as_raw(this), positions.try_into_param()?.abi(), altituderefsystem, spatialreferenceid, &mut result__).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeoboundingBoxFactory<R, F: FnOnce(&IGeoboundingBoxFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeoboundingBox, IGeoboundingBoxFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeoboundingBoxStatics<R, F: FnOnce(&IGeoboundingBoxStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeoboundingBox, IGeoboundingBoxStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GeoboundingBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GeoboundingBox {
    type Vtable = IGeoboundingBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeoboundingBox {
    const IID: ::windows_core::GUID = <IGeoboundingBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeoboundingBox";
}
::windows_core::imp::interface_hierarchy!(GeoboundingBox, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGeoshape> for GeoboundingBox {}
unsafe impl ::core::marker::Send for GeoboundingBox {}
unsafe impl ::core::marker::Sync for GeoboundingBox {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geocircle(::windows_core::IUnknown);
impl Geocircle {
    pub fn Center(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Center)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Radius(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Radius)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(position: BasicGeoposition, radius: f64) -> ::windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), position, radius, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem(position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), position, radius, altitudereferencesystem, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(::windows_core::Interface::as_raw(this), position, radius, altitudereferencesystem, spatialreferenceid, &mut result__).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeocircleFactory<R, F: FnOnce(&IGeocircleFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Geocircle, IGeocircleFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Geocircle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geocircle {
    type Vtable = IGeocircle_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geocircle {
    const IID: ::windows_core::GUID = <IGeocircle as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocircle";
}
::windows_core::imp::interface_hierarchy!(Geocircle, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGeoshape> for Geocircle {}
unsafe impl ::core::marker::Send for Geocircle {}
unsafe impl ::core::marker::Sync for Geocircle {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geocoordinate(::windows_core::IUnknown);
impl Geocoordinate {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Latitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Latitude)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Longitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Longitude)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Altitude(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Altitude)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Accuracy(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Accuracy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AltitudeAccuracy(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeAccuracy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Heading(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Heading)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Speed(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Speed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Point(&self) -> ::windows_core::Result<Geopoint> {
        let this = &::windows_core::ComInterface::cast::<IGeocoordinateWithPoint>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PositionSource(&self) -> ::windows_core::Result<PositionSource> {
        let this = &::windows_core::ComInterface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SatelliteData(&self) -> ::windows_core::Result<GeocoordinateSatelliteData> {
        let this = &::windows_core::ComInterface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SatelliteData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PositionSourceTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows_core::ComInterface::cast::<IGeocoordinateWithPositionSourceTimestamp>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionSourceTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRemoteSource(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeocoordinateWithRemoteSource>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRemoteSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Geocoordinate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geocoordinate {
    type Vtable = IGeocoordinate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geocoordinate {
    const IID: ::windows_core::GUID = <IGeocoordinate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocoordinate";
}
::windows_core::imp::interface_hierarchy!(Geocoordinate, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Geocoordinate {}
unsafe impl ::core::marker::Sync for Geocoordinate {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeocoordinateSatelliteData(::windows_core::IUnknown);
impl GeocoordinateSatelliteData {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PositionDilutionOfPrecision(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionDilutionOfPrecision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn HorizontalDilutionOfPrecision(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalDilutionOfPrecision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn VerticalDilutionOfPrecision(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticalDilutionOfPrecision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GeometricDilutionOfPrecision(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeometricDilutionOfPrecision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimeDilutionOfPrecision(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeDilutionOfPrecision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GeocoordinateSatelliteData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeocoordinateSatelliteData {
    const IID: ::windows_core::GUID = <IGeocoordinateSatelliteData as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeocoordinateSatelliteData";
}
::windows_core::imp::interface_hierarchy!(GeocoordinateSatelliteData, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GeocoordinateSatelliteData {}
unsafe impl ::core::marker::Sync for GeocoordinateSatelliteData {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geolocator(::windows_core::IUnknown);
impl Geolocator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Geolocator, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DesiredAccuracy(&self) -> ::windows_core::Result<PositionAccuracy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredAccuracy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredAccuracy)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MovementThreshold(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MovementThreshold)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMovementThreshold(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMovementThreshold)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReportInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LocationStatus(&self) -> ::windows_core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetGeopositionAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetGeopositionAsyncWithAgeAndTimeout(&self, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionAsyncWithAgeAndTimeout)(::windows_core::Interface::as_raw(this), maximumage, timeout, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AllowFallbackToConsentlessPositions(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGeolocator2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AllowFallbackToConsentlessPositions)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGeopositionHistoryAsync(starttime: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionHistoryAsync)(::windows_core::Interface::as_raw(this), starttime, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGeopositionHistoryWithDurationAsync(starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGeopositionHistoryWithDurationAsync)(::windows_core::Interface::as_raw(this), starttime, duration, &mut result__).from_abi(result__)
        })
    }
    pub fn IsDefaultGeopositionRecommended() -> ::windows_core::Result<bool> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDefaultGeopositionRecommended)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultGeoposition<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<BasicGeoposition>>,
    {
        Self::IGeolocatorStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetDefaultGeoposition)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DefaultGeoposition() -> ::windows_core::Result<super::super::Foundation::IReference<BasicGeoposition>> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultGeoposition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredAccuracyInMeters(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::ComInterface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredAccuracyInMeters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredAccuracyInMeters<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredAccuracyInMeters)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc(hidden)]
    pub fn IGeolocatorStatics<R, F: FnOnce(&IGeolocatorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Geolocator, IGeolocatorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGeolocatorStatics2<R, F: FnOnce(&IGeolocatorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Geolocator, IGeolocatorStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Geolocator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geolocator {
    type Vtable = IGeolocator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geolocator {
    const IID: ::windows_core::GUID = <IGeolocator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geolocator";
}
::windows_core::imp::interface_hierarchy!(Geolocator, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Geolocator {}
unsafe impl ::core::marker::Sync for Geolocator {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geopath(::windows_core::IUnknown);
impl Geopath {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Positions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Positions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0>(positions: P0) -> ::windows_core::Result<Geopath>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), positions.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReference<P0>(positions: P0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<Geopath>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReference)(::windows_core::Interface::as_raw(this), positions.try_into_param()?.abi(), altitudereferencesystem, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReferenceAndSpatialReference<P0>(positions: P0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<Geopath>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<BasicGeoposition>>,
    {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceAndSpatialReference)(::windows_core::Interface::as_raw(this), positions.try_into_param()?.abi(), altitudereferencesystem, spatialreferenceid, &mut result__).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeopathFactory<R, F: FnOnce(&IGeopathFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Geopath, IGeopathFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Geopath {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geopath {
    type Vtable = IGeopath_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geopath {
    const IID: ::windows_core::GUID = <IGeopath as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopath";
}
::windows_core::imp::interface_hierarchy!(Geopath, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGeoshape> for Geopath {}
unsafe impl ::core::marker::Send for Geopath {}
unsafe impl ::core::marker::Sync for Geopath {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geopoint(::windows_core::IUnknown);
impl Geopoint {
    pub fn Position(&self) -> ::windows_core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(position: BasicGeoposition) -> ::windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem(position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), position, altitudereferencesystem, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows_core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId)(::windows_core::Interface::as_raw(this), position, altitudereferencesystem, spatialreferenceid, &mut result__).from_abi(result__)
        })
    }
    pub fn GeoshapeType(&self) -> ::windows_core::Result<GeoshapeType> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeoshapeType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpatialReferenceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows_core::Result<AltitudeReferenceSystem> {
        let this = &::windows_core::ComInterface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AltitudeReferenceSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeopointFactory<R, F: FnOnce(&IGeopointFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Geopoint, IGeopointFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Geopoint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geopoint {
    type Vtable = IGeopoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geopoint {
    const IID: ::windows_core::GUID = <IGeopoint as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopoint";
}
::windows_core::imp::interface_hierarchy!(Geopoint, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGeoshape> for Geopoint {}
unsafe impl ::core::marker::Send for Geopoint {}
unsafe impl ::core::marker::Sync for Geopoint {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geoposition(::windows_core::IUnknown);
impl Geoposition {
    pub fn Coordinate(&self) -> ::windows_core::Result<Geocoordinate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Coordinate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CivicAddress(&self) -> ::windows_core::Result<CivicAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CivicAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VenueData(&self) -> ::windows_core::Result<VenueData> {
        let this = &::windows_core::ComInterface::cast::<IGeoposition2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VenueData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Geoposition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geoposition {
    type Vtable = IGeoposition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geoposition {
    const IID: ::windows_core::GUID = <IGeoposition as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geoposition";
}
::windows_core::imp::interface_hierarchy!(Geoposition, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Geoposition {}
unsafe impl ::core::marker::Sync for Geoposition {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geovisit(::windows_core::IUnknown);
impl Geovisit {
    pub fn Position(&self) -> ::windows_core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StateChange(&self) -> ::windows_core::Result<VisitStateChange> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Geovisit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geovisit {
    type Vtable = IGeovisit_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geovisit {
    const IID: ::windows_core::GUID = <IGeovisit as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geovisit";
}
::windows_core::imp::interface_hierarchy!(Geovisit, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Geovisit {}
unsafe impl ::core::marker::Sync for Geovisit {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeovisitMonitor(::windows_core::IUnknown);
impl GeovisitMonitor {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeovisitMonitor, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MonitoringScope(&self) -> ::windows_core::Result<VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonitoringScope)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self, value: VisitMonitoringScope) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn VisitStateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VisitStateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisitStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVisitStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetLastReportAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Geovisit>> {
        Self::IGeovisitMonitorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLastReportAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeovisitMonitorStatics<R, F: FnOnce(&IGeovisitMonitorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeovisitMonitor, IGeovisitMonitorStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GeovisitMonitor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GeovisitMonitor {
    type Vtable = IGeovisitMonitor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeovisitMonitor {
    const IID: ::windows_core::GUID = <IGeovisitMonitor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitMonitor";
}
::windows_core::imp::interface_hierarchy!(GeovisitMonitor, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GeovisitMonitor {}
unsafe impl ::core::marker::Sync for GeovisitMonitor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeovisitStateChangedEventArgs(::windows_core::IUnknown);
impl GeovisitStateChangedEventArgs {
    pub fn Visit(&self) -> ::windows_core::Result<Geovisit> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Visit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GeovisitStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeovisitStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IGeovisitStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(GeovisitStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GeovisitStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for GeovisitStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeovisitTriggerDetails(::windows_core::IUnknown);
impl GeovisitTriggerDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<Geovisit>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadReports)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GeovisitTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeovisitTriggerDetails {
    const IID: ::windows_core::GUID = <IGeovisitTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(GeovisitTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GeovisitTriggerDetails {}
unsafe impl ::core::marker::Sync for GeovisitTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PositionChangedEventArgs(::windows_core::IUnknown);
impl PositionChangedEventArgs {
    pub fn Position(&self) -> ::windows_core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PositionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PositionChangedEventArgs {
    const IID: ::windows_core::GUID = <IPositionChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.PositionChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PositionChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PositionChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StatusChangedEventArgs(::windows_core::IUnknown);
impl StatusChangedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for StatusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for StatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StatusChangedEventArgs {
    const IID: ::windows_core::GUID = <IStatusChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.StatusChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(StatusChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for StatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for StatusChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VenueData(::windows_core::IUnknown);
impl VenueData {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Level(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Level)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for VenueData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VenueData {
    type Vtable = IVenueData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VenueData {
    const IID: ::windows_core::GUID = <IVenueData as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.VenueData";
}
::windows_core::imp::interface_hierarchy!(VenueData, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VenueData {}
unsafe impl ::core::marker::Sync for VenueData {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: Self = Self(0i32);
    pub const Terrain: Self = Self(1i32);
    pub const Ellipsoid: Self = Self(2i32);
    pub const Geoid: Self = Self(3i32);
    pub const Surface: Self = Self(4i32);
}
impl ::core::marker::Copy for AltitudeReferenceSystem {}
impl ::core::clone::Clone for AltitudeReferenceSystem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AltitudeReferenceSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AltitudeReferenceSystem {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AltitudeReferenceSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AltitudeReferenceSystem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AltitudeReferenceSystem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.AltitudeReferenceSystem;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for GeolocationAccessStatus {}
impl ::core::clone::Clone for GeolocationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeolocationAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GeolocationAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GeolocationAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeolocationAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeolocationAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeolocationAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: Self = Self(0i32);
    pub const Geocircle: Self = Self(1i32);
    pub const Geopath: Self = Self(2i32);
    pub const GeoboundingBox: Self = Self(3i32);
}
impl ::core::marker::Copy for GeoshapeType {}
impl ::core::clone::Clone for GeoshapeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeoshapeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GeoshapeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GeoshapeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeoshapeType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GeoshapeType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeoshapeType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for PositionAccuracy {}
impl ::core::clone::Clone for PositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PositionAccuracy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PositionAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionAccuracy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PositionAccuracy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionAccuracy;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: Self = Self(0i32);
    pub const Satellite: Self = Self(1i32);
    pub const WiFi: Self = Self(2i32);
    pub const IPAddress: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const Default: Self = Self(5i32);
    pub const Obfuscated: Self = Self(6i32);
}
impl ::core::marker::Copy for PositionSource {}
impl ::core::clone::Clone for PositionSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PositionSource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PositionSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PositionSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionSource;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl ::core::marker::Copy for PositionStatus {}
impl ::core::clone::Clone for PositionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PositionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PositionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PositionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PositionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PositionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: Self = Self(0i32);
    pub const City: Self = Self(1i32);
}
impl ::core::marker::Copy for VisitMonitoringScope {}
impl ::core::clone::Clone for VisitMonitoringScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisitMonitoringScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VisitMonitoringScope {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VisitMonitoringScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitMonitoringScope").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VisitMonitoringScope {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitMonitoringScope;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: Self = Self(0i32);
    pub const Arrived: Self = Self(1i32);
    pub const Departed: Self = Self(2i32);
    pub const OtherMovement: Self = Self(3i32);
}
impl ::core::marker::Copy for VisitStateChange {}
impl ::core::clone::Clone for VisitStateChange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisitStateChange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VisitStateChange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VisitStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisitStateChange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VisitStateChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitStateChange;i4)");
}
#[repr(C)]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl ::core::marker::Copy for BasicGeoposition {}
impl ::core::clone::Clone for BasicGeoposition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BasicGeoposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BasicGeoposition").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).finish()
    }
}
impl ::windows_core::TypeKind for BasicGeoposition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for BasicGeoposition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Geolocation.BasicGeoposition;f8;f8;f8)");
}
impl ::core::cmp::PartialEq for BasicGeoposition {
    fn eq(&self, other: &Self) -> bool {
        self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.Altitude == other.Altitude
    }
}
impl ::core::cmp::Eq for BasicGeoposition {}
impl ::core::default::Default for BasicGeoposition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
