#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaTranscoder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTranscoder {
    type Vtable = IMediaTranscoder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaTranscoder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x190c99d2_a0aa_4d34_86bc_eed1b12c2f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTranscoder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetTrimStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub TrimStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimStopTime: usize,
    #[cfg(feature = "Foundation")]
    pub TrimStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimStopTime: usize,
    pub SetAlwaysReencode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AlwaysReencode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHardwareAccelerationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub HardwareAccelerationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AddAudioEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffectWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, effectrequired: bool, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffectWithSettings: usize,
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffectWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, effectrequired: bool, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffectWithSettings: usize,
    pub ClearEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareFileTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareFileTranscodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareStreamTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareStreamTranscodeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaTranscoder2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTranscoder2 {
    type Vtable = IMediaTranscoder2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaTranscoder2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40531d74_35e0_4f04_8574_ca8bc4e5a082);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTranscoder2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareMediaStreamSourceTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareMediaStreamSourceTranscodeAsync: usize,
    pub SetVideoProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaVideoProcessingAlgorithm) -> ::windows_core::HRESULT,
    pub VideoProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaVideoProcessingAlgorithm) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrepareTranscodeResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrepareTranscodeResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05f25dce_994f_4a34_9d68_97ccce1730d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareTranscodeResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanTranscode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub FailureReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TranscodeFailureReason) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TranscodeAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaTranscoder(::windows_core::IUnknown);
impl MediaTranscoder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaTranscoder, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTrimStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TrimStartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrimStartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStopTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTrimStopTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TrimStopTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrimStopTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlwaysReencode(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysReencode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysReencode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysReencode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHardwareAccelerationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHardwareAccelerationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardwareAccelerationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareAccelerationEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddAudioEffect(&self, activatableclassid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddAudioEffect)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffectWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, effectrequired: bool, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddAudioEffectWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), effectrequired, configuration.try_into_param()?.abi()).ok() }
    }
    pub fn AddVideoEffect(&self, activatableclassid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddVideoEffect)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffectWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, effectrequired: bool, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddVideoEffectWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), effectrequired, configuration.try_into_param()?.abi()).ok() }
    }
    pub fn ClearEffects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearEffects)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareFileTranscodeAsync<P0, P1, P2>(&self, source: P0, destination: P1, profile: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P2: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareFileTranscodeAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), destination.try_into_param()?.abi(), profile.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareStreamTranscodeAsync<P0, P1, P2>(&self, source: P0, destination: P1, profile: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P2: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareStreamTranscodeAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), destination.try_into_param()?.abi(), profile.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Media_Core\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareMediaStreamSourceTranscodeAsync<P0, P1, P2>(&self, source: P0, destination: P1, profile: P2) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: ::windows_core::TryIntoParam<super::Core::IMediaSource>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P2: ::windows_core::IntoParam<super::MediaProperties::MediaEncodingProfile>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrepareMediaStreamSourceTranscodeAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), destination.try_into_param()?.abi(), profile.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaTranscoder2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoProcessingAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoProcessingAlgorithm(&self) -> ::windows_core::Result<MediaVideoProcessingAlgorithm> {
        let this = &::windows_core::ComInterface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoProcessingAlgorithm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaTranscoder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaTranscoder {
    type Vtable = IMediaTranscoder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaTranscoder {
    const IID: ::windows_core::GUID = <IMediaTranscoder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaTranscoder {
    const NAME: &'static str = "Windows.Media.Transcoding.MediaTranscoder";
}
::windows_core::imp::interface_hierarchy!(MediaTranscoder, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaTranscoder {}
unsafe impl ::core::marker::Sync for MediaTranscoder {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrepareTranscodeResult(::windows_core::IUnknown);
impl PrepareTranscodeResult {
    pub fn CanTranscode(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanTranscode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FailureReason(&self) -> ::windows_core::Result<TranscodeFailureReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FailureReason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TranscodeAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncActionWithProgress<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranscodeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrepareTranscodeResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrepareTranscodeResult {
    const IID: ::windows_core::GUID = <IPrepareTranscodeResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrepareTranscodeResult {
    const NAME: &'static str = "Windows.Media.Transcoding.PrepareTranscodeResult";
}
::windows_core::imp::interface_hierarchy!(PrepareTranscodeResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrepareTranscodeResult {}
unsafe impl ::core::marker::Sync for PrepareTranscodeResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaVideoProcessingAlgorithm(pub i32);
impl MediaVideoProcessingAlgorithm {
    pub const Default: Self = Self(0i32);
    pub const MrfCrf444: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaVideoProcessingAlgorithm {}
impl ::core::clone::Clone for MediaVideoProcessingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaVideoProcessingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaVideoProcessingAlgorithm {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaVideoProcessingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaVideoProcessingAlgorithm").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaVideoProcessingAlgorithm {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.MediaVideoProcessingAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidProfile: Self = Self(2i32);
    pub const CodecNotFound: Self = Self(3i32);
}
impl ::core::marker::Copy for TranscodeFailureReason {}
impl ::core::clone::Clone for TranscodeFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TranscodeFailureReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TranscodeFailureReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TranscodeFailureReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TranscodeFailureReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TranscodeFailureReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.TranscodeFailureReason;i4)");
}
