#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneNumberFormatter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneNumberFormatter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1556b49e_bad4_4b4a_900d_4407adb7c981);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatWithOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: *mut ::core::ffi::c_void, numberformat: PhoneNumberFormat, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatPartialString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FormatStringWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneNumberFormatterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberFormatterStatics {
    type Vtable = IPhoneNumberFormatterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneNumberFormatterStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ca6f931_84d9_414b_ab4e_a0552c878602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberFormatterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, phonenumber: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCountryCodeForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetNationalDirectDialingPrefixForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, regioncode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, stripnondigit: bool, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WrapWithLeftToRightMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneNumberInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneNumberInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c7ce4dd_c8b4_4ea3_9aef_b342e2c5b417);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CountryCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetLengthOfGeographicalAreaCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetNationalSignificantNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetLengthOfNationalDestinationCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PredictNumberKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PredictedPhoneNumberKind) -> ::windows_core::HRESULT,
    pub GetGeographicRegionCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CheckNumberMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, othernumber: *mut ::core::ffi::c_void, result__: *mut PhoneNumberMatchResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneNumberInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberInfoFactory {
    type Vtable = IPhoneNumberInfoFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneNumberInfoFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8202b964_adaa_4cff_8fcf_17e7516a28ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneNumberInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneNumberInfoStatics {
    type Vtable = IPhoneNumberInfoStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneNumberInfoStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b3f4f6a_86a9_40e9_8649_6d61161928d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneNumberInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows_core::HSTRING>, phonenumber: *mut *mut ::core::ffi::c_void, result__: *mut PhoneNumberParseResult) -> ::windows_core::HRESULT,
    pub TryParseWithRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows_core::HSTRING>, regioncode: ::std::mem::MaybeUninit<::windows_core::HSTRING>, phonenumber: *mut *mut ::core::ffi::c_void, result__: *mut PhoneNumberParseResult) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneNumberFormatter(::windows_core::IUnknown);
impl PhoneNumberFormatter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneNumberFormatter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Format<P0>(&self, number: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<PhoneNumberInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Format)(::windows_core::Interface::as_raw(this), number.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn FormatWithOutputFormat<P0>(&self, number: P0, numberformat: PhoneNumberFormat) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<PhoneNumberInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatWithOutputFormat)(::windows_core::Interface::as_raw(this), number.into_param().abi(), numberformat, &mut result__).from_abi(result__)
        }
    }
    pub fn FormatPartialString(&self, number: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatPartialString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn FormatString(&self, number: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn FormatStringWithLeftToRightMarkers(&self, number: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormatStringWithLeftToRightMarkers)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        }
    }
    pub fn TryCreate(regioncode: &::windows_core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberFormatter>) -> ::windows_core::Result<()> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe { (::windows_core::Interface::vtable(this).TryCreate)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(regioncode), phonenumber as *mut _ as _).ok() })
    }
    pub fn GetCountryCodeForRegion(regioncode: &::windows_core::HSTRING) -> ::windows_core::Result<i32> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCountryCodeForRegion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(regioncode), &mut result__).from_abi(result__)
        })
    }
    pub fn GetNationalDirectDialingPrefixForRegion(regioncode: &::windows_core::HSTRING, stripnondigit: bool) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNationalDirectDialingPrefixForRegion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(regioncode), stripnondigit, &mut result__).from_abi(result__)
        })
    }
    pub fn WrapWithLeftToRightMarkers(number: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPhoneNumberFormatterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WrapWithLeftToRightMarkers)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneNumberFormatterStatics<R, F: FnOnce(&IPhoneNumberFormatterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneNumberFormatter, IPhoneNumberFormatterStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PhoneNumberFormatter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PhoneNumberFormatter {
    type Vtable = IPhoneNumberFormatter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneNumberFormatter {
    const IID: ::windows_core::GUID = <IPhoneNumberFormatter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNumberFormatter {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormatter";
}
::windows_core::imp::interface_hierarchy!(PhoneNumberFormatter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PhoneNumberFormatter {}
unsafe impl ::core::marker::Sync for PhoneNumberFormatter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneNumberInfo(::windows_core::IUnknown);
impl PhoneNumberInfo {
    pub fn CountryCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CountryCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhoneNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetLengthOfGeographicalAreaCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLengthOfGeographicalAreaCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetNationalSignificantNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNationalSignificantNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetLengthOfNationalDestinationCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLengthOfNationalDestinationCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PredictNumberKind(&self) -> ::windows_core::Result<PredictedPhoneNumberKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PredictNumberKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetGeographicRegionCode(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGeographicRegionCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CheckNumberMatch<P0>(&self, othernumber: P0) -> ::windows_core::Result<PhoneNumberMatchResult>
    where
        P0: ::windows_core::IntoParam<PhoneNumberInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckNumberMatch)(::windows_core::Interface::as_raw(this), othernumber.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(number: &::windows_core::HSTRING) -> ::windows_core::Result<PhoneNumberInfo> {
        Self::IPhoneNumberInfoFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(number), &mut result__).from_abi(result__)
        })
    }
    pub fn TryParse(input: &::windows_core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows_core::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(input), phonenumber as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    pub fn TryParseWithRegion(input: &::windows_core::HSTRING, regioncode: &::windows_core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows_core::Result<PhoneNumberParseResult> {
        Self::IPhoneNumberInfoStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryParseWithRegion)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(input), ::core::mem::transmute_copy(regioncode), phonenumber as *mut _ as _, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IPhoneNumberInfoFactory<R, F: FnOnce(&IPhoneNumberInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPhoneNumberInfoStatics<R, F: FnOnce(&IPhoneNumberInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneNumberInfo, IPhoneNumberInfoStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PhoneNumberInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PhoneNumberInfo {
    type Vtable = IPhoneNumberInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneNumberInfo {
    const IID: ::windows_core::GUID = <IPhoneNumberInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneNumberInfo {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.PhoneNumberInfo";
}
::windows_core::imp::interface_hierarchy!(PhoneNumberInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IStringable> for PhoneNumberInfo {}
unsafe impl ::core::marker::Send for PhoneNumberInfo {}
unsafe impl ::core::marker::Sync for PhoneNumberInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneNumberFormat(pub i32);
impl PhoneNumberFormat {
    pub const E164: Self = Self(0i32);
    pub const International: Self = Self(1i32);
    pub const National: Self = Self(2i32);
    pub const Rfc3966: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberFormat {}
impl ::core::clone::Clone for PhoneNumberFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneNumberFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneNumberFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneNumberFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberFormat;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneNumberMatchResult(pub i32);
impl PhoneNumberMatchResult {
    pub const NoMatch: Self = Self(0i32);
    pub const ShortNationalSignificantNumberMatch: Self = Self(1i32);
    pub const NationalSignificantNumberMatch: Self = Self(2i32);
    pub const ExactMatch: Self = Self(3i32);
}
impl ::core::marker::Copy for PhoneNumberMatchResult {}
impl ::core::clone::Clone for PhoneNumberMatchResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberMatchResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneNumberMatchResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneNumberMatchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberMatchResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneNumberMatchResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberMatchResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhoneNumberParseResult(pub i32);
impl PhoneNumberParseResult {
    pub const Valid: Self = Self(0i32);
    pub const NotANumber: Self = Self(1i32);
    pub const InvalidCountryCode: Self = Self(2i32);
    pub const TooShort: Self = Self(3i32);
    pub const TooLong: Self = Self(4i32);
}
impl ::core::marker::Copy for PhoneNumberParseResult {}
impl ::core::clone::Clone for PhoneNumberParseResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhoneNumberParseResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PhoneNumberParseResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PhoneNumberParseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberParseResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PhoneNumberParseResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PhoneNumberParseResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PredictedPhoneNumberKind(pub i32);
impl PredictedPhoneNumberKind {
    pub const FixedLine: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const FixedLineOrMobile: Self = Self(2i32);
    pub const TollFree: Self = Self(3i32);
    pub const PremiumRate: Self = Self(4i32);
    pub const SharedCost: Self = Self(5i32);
    pub const Voip: Self = Self(6i32);
    pub const PersonalNumber: Self = Self(7i32);
    pub const Pager: Self = Self(8i32);
    pub const UniversalAccountNumber: Self = Self(9i32);
    pub const Voicemail: Self = Self(10i32);
    pub const Unknown: Self = Self(11i32);
}
impl ::core::marker::Copy for PredictedPhoneNumberKind {}
impl ::core::clone::Clone for PredictedPhoneNumberKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PredictedPhoneNumberKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PredictedPhoneNumberKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PredictedPhoneNumberKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PredictedPhoneNumberKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PredictedPhoneNumberKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Globalization.PhoneNumberFormatting.PredictedPhoneNumberKind;i4)");
}
