#[inline]
pub unsafe fn CreateAudioReverb() -> ::windows_core::Result<::windows_core::IUnknown> {
    ::windows_targets::link!("xaudio2_8.dll" "system" fn CreateAudioReverb(ppapo : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateAudioReverb(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn CreateAudioVolumeMeter() -> ::windows_core::Result<::windows_core::IUnknown> {
    ::windows_targets::link!("xaudio2_8.dll" "system" fn CreateAudioVolumeMeter(ppapo : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateAudioVolumeMeter(&mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn CreateFX(clsid: *const ::windows_core::GUID, peffect: *mut ::core::option::Option<::windows_core::IUnknown>, pinitdat: ::core::option::Option<*const ::core::ffi::c_void>, initdatabytesize: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("xaudio2_8.dll" "cdecl" fn CreateFX(clsid : *const ::windows_core::GUID, peffect : *mut * mut::core::ffi::c_void, pinitdat : *const ::core::ffi::c_void, initdatabytesize : u32) -> ::windows_core::HRESULT);
    CreateFX(clsid, ::core::mem::transmute(peffect), ::core::mem::transmute(pinitdat.unwrap_or(::std::ptr::null())), initdatabytesize).ok()
}
#[inline]
pub unsafe fn CreateHrtfApo(init: *const HrtfApoInit) -> ::windows_core::Result<IXAPO> {
    ::windows_targets::link!("hrtfapo.dll" "system" fn CreateHrtfApo(init : *const HrtfApoInit, xapo : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateHrtfApo(init, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn XAudio2CreateWithVersionInfo(ppxaudio2: *mut ::core::option::Option<IXAudio2>, flags: u32, xaudio2processor: u32, ntddiversion: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("xaudio2_8.dll" "system" fn XAudio2CreateWithVersionInfo(ppxaudio2 : *mut * mut::core::ffi::c_void, flags : u32, xaudio2processor : u32, ntddiversion : u32) -> ::windows_core::HRESULT);
    XAudio2CreateWithVersionInfo(::core::mem::transmute(ppxaudio2), flags, xaudio2processor, ntddiversion).ok()
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAPO(::windows_core::IUnknown);
impl IXAPO {
    pub unsafe fn GetRegistrationProperties(&self) -> ::windows_core::Result<*mut XAPO_REGISTRATION_PROPERTIES> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetRegistrationProperties(&mut result__)).from_abi(result__)
    }
    pub unsafe fn IsInputFormatSupported(&self, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: ::core::option::Option<*mut *mut super::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.IsInputFormatSupported(poutputformat, prequestedinputformat, ::core::mem::transmute(ppsupportedinputformat.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn IsOutputFormatSupported(&self, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: ::core::option::Option<*mut *mut super::WAVEFORMATEX>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.IsOutputFormatSupported(pinputformat, prequestedoutputformat, ::core::mem::transmute(ppsupportedoutputformat.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Initialize(&self, pdata: ::core::option::Option<*const ::core::ffi::c_void>, databytesize: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Initialize(::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), databytesize)).ok()
    }
    pub unsafe fn Reset(&self) {
        ::windows_core::vcall!(self.Reset())
    }
    pub unsafe fn LockForProcess(&self, pinputlockedparameters: ::core::option::Option<&[XAPO_LOCKFORPROCESS_PARAMETERS]>, poutputlockedparameters: ::core::option::Option<&[XAPO_LOCKFORPROCESS_PARAMETERS]>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.LockForProcess(pinputlockedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pinputlockedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), poutputlockedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(poutputlockedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))).ok()
    }
    pub unsafe fn UnlockForProcess(&self) {
        ::windows_core::vcall!(self.UnlockForProcess())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process<P0>(&self, pinputprocessparameters: ::core::option::Option<&[XAPO_PROCESS_BUFFER_PARAMETERS]>, poutputprocessparameters: ::core::option::Option<&mut [XAPO_PROCESS_BUFFER_PARAMETERS]>, isenabled: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.Process(pinputprocessparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pinputprocessparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), poutputprocessparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(poutputprocessparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), isenabled.into_param().abi()))
    }
    pub unsafe fn CalcInputFrames(&self, outputframecount: u32) -> u32 {
        ::windows_core::vcall!(self.CalcInputFrames(outputframecount))
    }
    pub unsafe fn CalcOutputFrames(&self, inputframecount: u32) -> u32 {
        ::windows_core::vcall!(self.CalcOutputFrames(inputframecount))
    }
}
::windows_core::imp::interface_hierarchy!(IXAPO, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXAPO {
    type Vtable = IXAPO_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXAPO {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa410b984_9839_4819_a0be_2856ae6b3adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPO_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRegistrationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES) -> ::windows_core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> ::windows_core::HRESULT,
    pub IsOutputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> ::windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, databytesize: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub LockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> ::windows_core::HRESULT,
    pub UnlockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
    pub CalcInputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputframecount: u32) -> u32,
    pub CalcOutputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputframecount: u32) -> u32,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAPOHrtfParameters(::windows_core::IUnknown);
impl IXAPOHrtfParameters {
    pub unsafe fn SetSourcePosition(&self, position: *const HrtfPosition) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSourcePosition(position)).ok()
    }
    pub unsafe fn SetSourceOrientation(&self, orientation: *const HrtfOrientation) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSourceOrientation(orientation)).ok()
    }
    pub unsafe fn SetSourceGain(&self, gain: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSourceGain(gain)).ok()
    }
    pub unsafe fn SetEnvironment(&self, environment: HrtfEnvironment) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetEnvironment(environment)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXAPOHrtfParameters, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXAPOHrtfParameters {
    type Vtable = IXAPOHrtfParameters_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXAPOHrtfParameters {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15b3cd66_e9de_4464_b6e6_2bc3cf63d455);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPOHrtfParameters_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetSourcePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: *const HrtfPosition) -> ::windows_core::HRESULT,
    pub SetSourceOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, orientation: *const HrtfOrientation) -> ::windows_core::HRESULT,
    pub SetSourceGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows_core::HRESULT,
    pub SetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: HrtfEnvironment) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAPOParameters(::windows_core::IUnknown);
impl IXAPOParameters {
    pub unsafe fn SetParameters(&self, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32) {
        ::windows_core::vcall!(self.SetParameters(pparameters, parameterbytesize))
    }
    pub unsafe fn GetParameters(&self, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32) {
        ::windows_core::vcall!(self.GetParameters(pparameters, parameterbytesize))
    }
}
::windows_core::imp::interface_hierarchy!(IXAPOParameters, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXAPOParameters {
    type Vtable = IXAPOParameters_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXAPOParameters {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26d95c66_80f2_499a_ad54_5ae7f01c6d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPOParameters_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *const ::core::ffi::c_void, parameterbytesize: u32),
    pub GetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, parameterbytesize: u32),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2(::windows_core::IUnknown);
impl IXAudio2 {
    pub unsafe fn RegisterForCallbacks<P0>(&self, pcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2EngineCallback>,
    {
        ::windows_core::vcall!(self.RegisterForCallbacks(pcallback.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterForCallbacks<P0>(&self, pcallback: P0)
    where
        P0: ::windows_core::IntoParam<IXAudio2EngineCallback>,
    {
        ::windows_core::vcall!(self.UnregisterForCallbacks(pcallback.into_param().abi()))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSourceVoice<P0>(&self, ppsourcevoice: *mut ::core::option::Option<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: P0, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2VoiceCallback>,
    {
        ::windows_core::vcall!(self.CreateSourceVoice(::core::mem::transmute(ppsourcevoice), psourceformat, flags, maxfrequencyratio, pcallback.into_param().abi(), ::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSubmixVoice(&self, ppsubmixvoice: *mut ::core::option::Option<IXAudio2SubmixVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.CreateSubmixVoice(::core::mem::transmute(ppsubmixvoice), inputchannels, inputsamplerate, flags, processingstage, ::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateMasteringVoice<P0>(&self, ppmasteringvoice: *mut ::core::option::Option<IXAudio2MasteringVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: P0, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.CreateMasteringVoice(::core::mem::transmute(ppmasteringvoice), inputchannels, inputsamplerate, flags, szdeviceid.into_param().abi(), ::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null())), streamcategory)).ok()
    }
    pub unsafe fn StartEngine(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.StartEngine()).ok()
    }
    pub unsafe fn StopEngine(&self) {
        ::windows_core::vcall!(self.StopEngine())
    }
    pub unsafe fn CommitChanges(&self, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.CommitChanges(operationset)).ok()
    }
    pub unsafe fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
        ::windows_core::vcall!(self.GetPerformanceData(pperfdata))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDebugConfiguration(&self, pdebugconfiguration: ::core::option::Option<*const XAUDIO2_DEBUG_CONFIGURATION>, preserved: ::core::option::Option<*const ::core::ffi::c_void>) {
        ::windows_core::vcall!(self.SetDebugConfiguration(::core::mem::transmute(pdebugconfiguration.unwrap_or(::std::ptr::null())), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null()))))
    }
}
::windows_core::imp::interface_hierarchy!(IXAudio2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXAudio2 {
    type Vtable = IXAudio2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXAudio2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b02e3cf_2e0b_4ec3_be45_1b2a3fe7210d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterForCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnregisterForCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSourceVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsourcevoice: *mut *mut ::core::ffi::c_void, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: *mut ::core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSourceVoice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSubmixVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubmixvoice: *mut *mut ::core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSubmixVoice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateMasteringVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmasteringvoice: *mut *mut ::core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: ::windows_core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateMasteringVoice: usize,
    pub StartEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub CommitChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows_core::HRESULT,
    pub GetPerformanceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA),
    #[cfg(feature = "Win32_Foundation")]
    pub SetDebugConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *const ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDebugConfiguration: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2EngineCallback(::std::ptr::NonNull<::std::ffi::c_void>);
impl IXAudio2EngineCallback {
    pub unsafe fn OnProcessingPassStart(&self) {
        ::windows_core::vcall!(self.OnProcessingPassStart())
    }
    pub unsafe fn OnProcessingPassEnd(&self) {
        ::windows_core::vcall!(self.OnProcessingPassEnd())
    }
    pub unsafe fn OnCriticalError(&self, error: ::windows_core::HRESULT) {
        ::windows_core::vcall!(self.OnCriticalError(error))
    }
}
unsafe impl ::windows_core::Interface for IXAudio2EngineCallback {
    type Vtable = IXAudio2EngineCallback_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2EngineCallback_Vtbl {
    pub OnProcessingPassStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnProcessingPassEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnCriticalError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: ::windows_core::HRESULT),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2Extension(::windows_core::IUnknown);
impl IXAudio2Extension {
    pub unsafe fn GetProcessingQuantum(&self, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
        ::windows_core::vcall!(self.GetProcessingQuantum(quantumnumerator, quantumdenominator))
    }
    pub unsafe fn GetProcessor(&self, processor: *mut u32) {
        ::windows_core::vcall!(self.GetProcessor(processor))
    }
}
::windows_core::imp::interface_hierarchy!(IXAudio2Extension, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXAudio2Extension {
    type Vtable = IXAudio2Extension_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXAudio2Extension {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84ac29bb_d619_44d2_b197_e4acf7df3ed6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Extension_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetProcessingQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quantumnumerator: *mut u32, quantumdenominator: *mut u32),
    pub GetProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processor: *mut u32),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2MasteringVoice(::std::ptr::NonNull<::std::ffi::c_void>);
impl IXAudio2MasteringVoice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetVoiceDetails(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOutputVoices(::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(&self, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetEffectChain(::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null())))).ok()
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnableEffect(effectindex, operationset)).ok()
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.DisableEffect(effectindex, operationset)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> super::super::super::Foundation::BOOL {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetEffectState(effectindex, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetEffectParameters(effectindex, pparameters, parametersbytesize, operationset)).ok()
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetEffectParameters(effectindex, pparameters, parametersbytesize)).ok()
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFilterParameters(pparameters, operationset)).ok()
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFilterParameters(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.base__.SetOutputFilterParameters(pdestinationvoice.into_param().abi(), pparameters, operationset)).ok()
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOutputFilterParameters(pdestinationvoice.into_param().abi(), &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetVolume(volume, operationset)).ok()
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetVolume(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr()), operationset)).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        ::windows_core::vcall!(self.base__.GetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr())))
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.base__.SetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset)).ok()
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn DestroyVoice(&self) {
        ::windows_core::vcall!(self.base__.DestroyVoice())
    }
    pub unsafe fn GetChannelMask(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetChannelMask(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXAudio2MasteringVoice, IXAudio2Voice);
unsafe impl ::windows_core::Interface for IXAudio2MasteringVoice {
    type Vtable = IXAudio2MasteringVoice_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2MasteringVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
    pub GetChannelMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchannelmask: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2SourceVoice(::std::ptr::NonNull<::std::ffi::c_void>);
impl IXAudio2SourceVoice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetVoiceDetails(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOutputVoices(::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(&self, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetEffectChain(::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null())))).ok()
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnableEffect(effectindex, operationset)).ok()
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.DisableEffect(effectindex, operationset)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> super::super::super::Foundation::BOOL {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetEffectState(effectindex, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetEffectParameters(effectindex, pparameters, parametersbytesize, operationset)).ok()
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetEffectParameters(effectindex, pparameters, parametersbytesize)).ok()
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFilterParameters(pparameters, operationset)).ok()
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFilterParameters(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.base__.SetOutputFilterParameters(pdestinationvoice.into_param().abi(), pparameters, operationset)).ok()
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOutputFilterParameters(pdestinationvoice.into_param().abi(), &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetVolume(volume, operationset)).ok()
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetVolume(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr()), operationset)).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        ::windows_core::vcall!(self.base__.GetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr())))
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.base__.SetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset)).ok()
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn DestroyVoice(&self) {
        ::windows_core::vcall!(self.base__.DestroyVoice())
    }
    pub unsafe fn Start(&self, flags: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Start(flags, operationset)).ok()
    }
    pub unsafe fn Stop(&self, flags: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Stop(flags, operationset)).ok()
    }
    pub unsafe fn SubmitSourceBuffer(&self, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: ::core::option::Option<*const XAUDIO2_BUFFER_WMA>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SubmitSourceBuffer(pbuffer, ::core::mem::transmute(pbufferwma.unwrap_or(::std::ptr::null())))).ok()
    }
    pub unsafe fn FlushSourceBuffers(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.FlushSourceBuffers()).ok()
    }
    pub unsafe fn Discontinuity(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Discontinuity()).ok()
    }
    pub unsafe fn ExitLoop(&self, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.ExitLoop(operationset)).ok()
    }
    pub unsafe fn GetState(&self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
        ::windows_core::vcall!(self.GetState(pvoicestate, flags))
    }
    pub unsafe fn SetFrequencyRatio(&self, ratio: f32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFrequencyRatio(ratio, operationset)).ok()
    }
    pub unsafe fn GetFrequencyRatio(&self) -> f32 {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFrequencyRatio(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetSourceSampleRate(&self, newsourcesamplerate: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSourceSampleRate(newsourcesamplerate)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXAudio2SourceVoice, IXAudio2Voice);
unsafe impl ::windows_core::Interface for IXAudio2SourceVoice {
    type Vtable = IXAudio2SourceVoice_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SourceVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, operationset: u32) -> ::windows_core::HRESULT,
    pub SubmitSourceBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> ::windows_core::HRESULT,
    pub FlushSourceBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExitLoop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationset: u32) -> ::windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32),
    pub SetFrequencyRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratio: f32, operationset: u32) -> ::windows_core::HRESULT,
    pub GetFrequencyRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pratio: *mut f32),
    pub SetSourceSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newsourcesamplerate: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2SubmixVoice(::std::ptr::NonNull<::std::ffi::c_void>);
impl IXAudio2SubmixVoice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetVoiceDetails(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOutputVoices(::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(&self, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetEffectChain(::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null())))).ok()
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnableEffect(effectindex, operationset)).ok()
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.DisableEffect(effectindex, operationset)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> super::super::super::Foundation::BOOL {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetEffectState(effectindex, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetEffectParameters(effectindex, pparameters, parametersbytesize, operationset)).ok()
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetEffectParameters(effectindex, pparameters, parametersbytesize)).ok()
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFilterParameters(pparameters, operationset)).ok()
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFilterParameters(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.base__.SetOutputFilterParameters(pdestinationvoice.into_param().abi(), pparameters, operationset)).ok()
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOutputFilterParameters(pdestinationvoice.into_param().abi(), &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetVolume(volume, operationset)).ok()
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetVolume(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr()), operationset)).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        ::windows_core::vcall!(self.base__.GetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr())))
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.base__.SetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset)).ok()
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn DestroyVoice(&self) {
        ::windows_core::vcall!(self.base__.DestroyVoice())
    }
}
::windows_core::imp::interface_hierarchy!(IXAudio2SubmixVoice, IXAudio2Voice);
unsafe impl ::windows_core::Interface for IXAudio2SubmixVoice {
    type Vtable = IXAudio2SubmixVoice_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SubmixVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2Voice(::std::ptr::NonNull<::std::ffi::c_void>);
impl IXAudio2Voice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetVoiceDetails(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: ::core::option::Option<*const XAUDIO2_VOICE_SENDS>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOutputVoices(::core::mem::transmute(psendlist.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectChain(&self, peffectchain: ::core::option::Option<*const XAUDIO2_EFFECT_CHAIN>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetEffectChain(::core::mem::transmute(peffectchain.unwrap_or(::std::ptr::null())))).ok()
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnableEffect(effectindex, operationset)).ok()
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.DisableEffect(effectindex, operationset)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> super::super::super::Foundation::BOOL {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetEffectState(effectindex, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetEffectParameters(effectindex, pparameters, parametersbytesize, operationset)).ok()
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetEffectParameters(effectindex, pparameters, parametersbytesize)).ok()
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFilterParameters(pparameters, operationset)).ok()
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFilterParameters(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.SetOutputFilterParameters(pdestinationvoice.into_param().abi(), pparameters, operationset)).ok()
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutputFilterParameters(pdestinationvoice.into_param().abi(), &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetVolume(volume, operationset)).ok()
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetVolume(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr()), operationset)).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        ::windows_core::vcall!(self.GetChannelVolumes(pvolumes.len().try_into().unwrap(), ::core::mem::transmute(pvolumes.as_ptr())))
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        ::windows_core::vcall!(self.SetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset)).ok()
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: ::windows_core::IntoParam<IXAudio2Voice>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutputMatrix(pdestinationvoice.into_param().abi(), sourcechannels, destinationchannels, &mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn DestroyVoice(&self) {
        ::windows_core::vcall!(self.DestroyVoice())
    }
}
unsafe impl ::windows_core::Interface for IXAudio2Voice {
    type Vtable = IXAudio2Voice_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Voice_Vtbl {
    pub GetVoiceDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS),
    pub SetOutputVoices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEffectChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEffectChain: usize,
    pub EnableEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows_core::HRESULT,
    pub DisableEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectindex: u32, operationset: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEffectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectindex: u32, penabled: *mut super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEffectState: usize,
    pub SetEffectParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *const ::core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> ::windows_core::HRESULT,
    pub GetEffectParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectindex: u32, pparameters: *mut ::core::ffi::c_void, parametersbytesize: u32) -> ::windows_core::HRESULT,
    pub SetFilterParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::HRESULT,
    pub GetFilterParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS),
    pub SetOutputFilterParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationvoice: *mut ::core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> ::windows_core::HRESULT,
    pub GetOutputFilterParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationvoice: *mut ::core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS),
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: f32, operationset: u32) -> ::windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvolume: *mut f32),
    pub SetChannelVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *const f32, operationset: u32) -> ::windows_core::HRESULT,
    pub GetChannelVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channels: u32, pvolumes: *mut f32),
    pub SetOutputMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationvoice: *mut ::core::ffi::c_void, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> ::windows_core::HRESULT,
    pub GetOutputMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationvoice: *mut ::core::ffi::c_void, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32),
    pub DestroyVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXAudio2VoiceCallback(::std::ptr::NonNull<::std::ffi::c_void>);
impl IXAudio2VoiceCallback {
    pub unsafe fn OnVoiceProcessingPassStart(&self, bytesrequired: u32) {
        ::windows_core::vcall!(self.OnVoiceProcessingPassStart(bytesrequired))
    }
    pub unsafe fn OnVoiceProcessingPassEnd(&self) {
        ::windows_core::vcall!(self.OnVoiceProcessingPassEnd())
    }
    pub unsafe fn OnStreamEnd(&self) {
        ::windows_core::vcall!(self.OnStreamEnd())
    }
    pub unsafe fn OnBufferStart(&self, pbuffercontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.OnBufferStart(pbuffercontext))
    }
    pub unsafe fn OnBufferEnd(&self, pbuffercontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.OnBufferEnd(pbuffercontext))
    }
    pub unsafe fn OnLoopEnd(&self, pbuffercontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.OnLoopEnd(pbuffercontext))
    }
    pub unsafe fn OnVoiceError(&self, pbuffercontext: *mut ::core::ffi::c_void, error: ::windows_core::HRESULT) {
        ::windows_core::vcall!(self.OnVoiceError(pbuffercontext, error))
    }
}
unsafe impl ::windows_core::Interface for IXAudio2VoiceCallback {
    type Vtable = IXAudio2VoiceCallback_Vtbl;
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2VoiceCallback_Vtbl {
    pub OnVoiceProcessingPassStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytesrequired: u32),
    pub OnVoiceProcessingPassEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnStreamEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnBufferStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void),
    pub OnBufferEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void),
    pub OnLoopEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void),
    pub OnVoiceError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffercontext: *mut ::core::ffi::c_void, error: ::windows_core::HRESULT),
}
pub const AudioReverb: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2633b16_471b_4498_b8c5_4f0959e2ec09);
pub const AudioVolumeMeter: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fc3b166_972a_40cf_bc37_7db03db2fba3);
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(1i32);
pub const Cardioid: HrtfDirectivityType = HrtfDirectivityType(1i32);
pub const Cone: HrtfDirectivityType = HrtfDirectivityType(2i32);
pub const CustomDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(1i32);
pub const FACILITY_XAPO: u32 = 2199u32;
pub const FACILITY_XAUDIO2: u32 = 2198u32;
pub const FXECHO_DEFAULT_DELAY: f32 = 500f32;
pub const FXECHO_DEFAULT_FEEDBACK: f32 = 0.5f32;
pub const FXECHO_DEFAULT_WETDRYMIX: f32 = 0.5f32;
pub const FXECHO_MAX_DELAY: f32 = 2000f32;
pub const FXECHO_MAX_FEEDBACK: f32 = 1f32;
pub const FXECHO_MAX_WETDRYMIX: f32 = 1f32;
pub const FXECHO_MIN_DELAY: f32 = 1f32;
pub const FXECHO_MIN_FEEDBACK: f32 = 0f32;
pub const FXECHO_MIN_WETDRYMIX: f32 = 0f32;
pub const FXEQ: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5e01117_d6c4_485a_a3f5_695196f3dbfa);
pub const FXEQ_DEFAULT_BANDWIDTH: f32 = 1f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_0: f32 = 100f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_1: f32 = 800f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_2: f32 = 2000f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_3: f32 = 10000f32;
pub const FXEQ_DEFAULT_GAIN: f32 = 1f32;
pub const FXEQ_MAX_BANDWIDTH: f32 = 2f32;
pub const FXEQ_MAX_FRAMERATE: u32 = 48000u32;
pub const FXEQ_MAX_FREQUENCY_CENTER: f32 = 20000f32;
pub const FXEQ_MAX_GAIN: f32 = 7.94f32;
pub const FXEQ_MIN_BANDWIDTH: f32 = 0.1f32;
pub const FXEQ_MIN_FRAMERATE: u32 = 22000u32;
pub const FXEQ_MIN_FREQUENCY_CENTER: f32 = 20f32;
pub const FXEQ_MIN_GAIN: f32 = 0.126f32;
pub const FXEcho: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5039d740_f736_449a_84d3_a56202557b87);
pub const FXLOUDNESS_DEFAULT_MOMENTARY_MS: u32 = 400u32;
pub const FXLOUDNESS_DEFAULT_SHORTTERM_MS: u32 = 3000u32;
pub const FXMASTERINGLIMITER_DEFAULT_LOUDNESS: u32 = 1000u32;
pub const FXMASTERINGLIMITER_DEFAULT_RELEASE: u32 = 6u32;
pub const FXMASTERINGLIMITER_MAX_LOUDNESS: u32 = 1800u32;
pub const FXMASTERINGLIMITER_MAX_RELEASE: u32 = 20u32;
pub const FXMASTERINGLIMITER_MIN_LOUDNESS: u32 = 1u32;
pub const FXMASTERINGLIMITER_MIN_RELEASE: u32 = 1u32;
pub const FXMasteringLimiter: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4137916_2be1_46fd_8599_441536f49856);
pub const FXREVERB_DEFAULT_DIFFUSION: f32 = 0.9f32;
pub const FXREVERB_DEFAULT_ROOMSIZE: f32 = 0.6f32;
pub const FXREVERB_MAX_DIFFUSION: f32 = 1f32;
pub const FXREVERB_MAX_ROOMSIZE: f32 = 1f32;
pub const FXREVERB_MIN_DIFFUSION: f32 = 0f32;
pub const FXREVERB_MIN_ROOMSIZE: f32 = 0.0001f32;
pub const FXReverb: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d9aca56_cb68_4807_b632_b137352e8596);
pub const HRTF_DEFAULT_UNITY_GAIN_DISTANCE: f32 = 1f32;
pub const HRTF_MAX_GAIN_LIMIT: f32 = 12f32;
pub const HRTF_MIN_GAIN_LIMIT: f32 = -96f32;
pub const HRTF_MIN_UNITY_GAIN_DISTANCE: f32 = 0.05f32;
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(2i32);
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(5i32);
pub const Large: HrtfEnvironment = HrtfEnvironment(2i32);
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(0i32);
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(4i32);
pub const Medium: HrtfEnvironment = HrtfEnvironment(1i32);
pub const NaturalDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(0i32);
pub const NotchFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(3i32);
pub const OmniDirectional: HrtfDirectivityType = HrtfDirectivityType(0i32);
pub const Outdoors: HrtfEnvironment = HrtfEnvironment(3i32);
pub const Processor1: u32 = 1u32;
pub const Processor10: u32 = 512u32;
pub const Processor11: u32 = 1024u32;
pub const Processor12: u32 = 2048u32;
pub const Processor13: u32 = 4096u32;
pub const Processor14: u32 = 8192u32;
pub const Processor15: u32 = 16384u32;
pub const Processor16: u32 = 32768u32;
pub const Processor17: u32 = 65536u32;
pub const Processor18: u32 = 131072u32;
pub const Processor19: u32 = 262144u32;
pub const Processor2: u32 = 2u32;
pub const Processor20: u32 = 524288u32;
pub const Processor21: u32 = 1048576u32;
pub const Processor22: u32 = 2097152u32;
pub const Processor23: u32 = 4194304u32;
pub const Processor24: u32 = 8388608u32;
pub const Processor25: u32 = 16777216u32;
pub const Processor26: u32 = 33554432u32;
pub const Processor27: u32 = 67108864u32;
pub const Processor28: u32 = 134217728u32;
pub const Processor29: u32 = 268435456u32;
pub const Processor3: u32 = 4u32;
pub const Processor30: u32 = 536870912u32;
pub const Processor31: u32 = 1073741824u32;
pub const Processor32: u32 = 2147483648u32;
pub const Processor4: u32 = 8u32;
pub const Processor5: u32 = 16u32;
pub const Processor6: u32 = 32u32;
pub const Processor7: u32 = 64u32;
pub const Processor8: u32 = 128u32;
pub const Processor9: u32 = 256u32;
pub const SPEAKER_MONO: u32 = 4u32;
pub const Small: HrtfEnvironment = HrtfEnvironment(0i32);
pub const X3DAUDIO_2PI: f32 = 6.2831855f32;
pub const X3DAUDIO_CALCULATE_DELAY: u32 = 2u32;
pub const X3DAUDIO_CALCULATE_DOPPLER: u32 = 32u32;
pub const X3DAUDIO_CALCULATE_EMITTER_ANGLE: u32 = 64u32;
pub const X3DAUDIO_CALCULATE_LPF_DIRECT: u32 = 4u32;
pub const X3DAUDIO_CALCULATE_LPF_REVERB: u32 = 8u32;
pub const X3DAUDIO_CALCULATE_MATRIX: u32 = 1u32;
pub const X3DAUDIO_CALCULATE_REDIRECT_TO_LFE: u32 = 131072u32;
pub const X3DAUDIO_CALCULATE_REVERB: u32 = 16u32;
pub const X3DAUDIO_CALCULATE_ZEROCENTER: u32 = 65536u32;
pub const X3DAUDIO_HANDLE_BYTESIZE: u32 = 20u32;
pub const X3DAUDIO_PI: f32 = 3.1415927f32;
pub const X3DAUDIO_SPEED_OF_SOUND: f32 = 343.5f32;
pub const XAPO_BUFFER_SILENT: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(0i32);
pub const XAPO_BUFFER_VALID: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(1i32);
pub const XAPO_E_FORMAT_UNSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003369983i32);
pub const XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH: u32 = 4u32;
pub const XAPO_FLAG_BUFFERCOUNT_MUST_MATCH: u32 = 8u32;
pub const XAPO_FLAG_CHANNELS_MUST_MATCH: u32 = 1u32;
pub const XAPO_FLAG_FRAMERATE_MUST_MATCH: u32 = 2u32;
pub const XAPO_FLAG_INPLACE_REQUIRED: u32 = 32u32;
pub const XAPO_FLAG_INPLACE_SUPPORTED: u32 = 16u32;
pub const XAPO_MAX_CHANNELS: u32 = 64u32;
pub const XAPO_MAX_FRAMERATE: u32 = 200000u32;
pub const XAPO_MIN_CHANNELS: u32 = 1u32;
pub const XAPO_MIN_FRAMERATE: u32 = 1000u32;
pub const XAPO_REGISTRATION_STRING_LENGTH: u32 = 256u32;
pub const XAUDIO2D_DLL: ::windows_core::PCWSTR = ::windows_core::w!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_A: ::windows_core::PCSTR = ::windows_core::s!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_W: ::windows_core::PCWSTR = ::windows_core::w!("xaudio2_9d.dll");
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_DECAY_TIME: f32 = 1f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DISABLE_LATE_FIELD: u32 = 0u32;
pub const XAUDIO2FX_REVERB_DEFAULT_EARLY_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LATE_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION: u32 = 6u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX: u32 = 27u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_FREQ: f32 = 5000f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_WET_DRY_MIX: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_MAX_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_DIFFUSION: u32 = 15u32;
pub const XAUDIO2FX_REVERB_MAX_FRAMERATE: u32 = 48000u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_CUTOFF: u32 = 14u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_CUTOFF: u32 = 9u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_GAIN: u32 = 12u32;
pub const XAUDIO2FX_REVERB_MAX_POSITION: u32 = 30u32;
pub const XAUDIO2FX_REVERB_MAX_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY: u32 = 300u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_DELAY: u32 = 85u32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_FREQ: f32 = 20000f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_WET_DRY_MIX: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_SIDE_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_DECAY_TIME: f32 = 0.1f32;
pub const XAUDIO2FX_REVERB_MIN_DENSITY: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_DIFFUSION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_FRAMERATE: u32 = 20000u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_POSITION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_FREQ: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_HF: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_MAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_SIZE: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_WET_DRY_MIX: f32 = 0f32;
pub const XAUDIO2_1024_QUANTUM: u32 = 32768u32;
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295u32;
pub const XAUDIO2_COMMIT_ALL: u32 = 0u32;
pub const XAUDIO2_COMMIT_NOW: u32 = 0u32;
pub const XAUDIO2_DEBUG_ENGINE: u32 = 1u32;
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0u32;
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2f32;
pub const XAUDIO2_DEFAULT_PROCESSOR: u32 = 1u32;
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0u32;
pub const XAUDIO2_DLL: ::windows_core::PCWSTR = ::windows_core::w!("xaudio2_9.dll");
pub const XAUDIO2_DLL_A: ::windows_core::PCSTR = ::windows_core::s!("xaudio2_9.dll");
pub const XAUDIO2_DLL_W: ::windows_core::PCWSTR = ::windows_core::w!("xaudio2_9.dll");
pub const XAUDIO2_END_OF_STREAM: u32 = 64u32;
pub const XAUDIO2_E_DEVICE_INVALIDATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003435516i32);
pub const XAUDIO2_E_INVALID_CALL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003435519i32);
pub const XAUDIO2_E_XAPO_CREATION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003435517i32);
pub const XAUDIO2_E_XMA_DECODER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003435518i32);
pub const XAUDIO2_LOG_API_CALLS: u32 = 16u32;
pub const XAUDIO2_LOG_DETAIL: u32 = 8u32;
pub const XAUDIO2_LOG_ERRORS: u32 = 1u32;
pub const XAUDIO2_LOG_FUNC_CALLS: u32 = 32u32;
pub const XAUDIO2_LOG_INFO: u32 = 4u32;
pub const XAUDIO2_LOG_LOCKS: u32 = 128u32;
pub const XAUDIO2_LOG_MEMORY: u32 = 256u32;
pub const XAUDIO2_LOG_STREAMING: u32 = 4096u32;
pub const XAUDIO2_LOG_TIMING: u32 = 64u32;
pub const XAUDIO2_LOG_WARNINGS: u32 = 2u32;
pub const XAUDIO2_LOOP_INFINITE: u32 = 255u32;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64u32;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: u32 = 2u32;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648u32;
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5f32;
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024f32;
pub const XAUDIO2_MAX_INSTANCES: u32 = 8u32;
pub const XAUDIO2_MAX_LOOP_COUNT: u32 = 254u32;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: u32 = 64u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: u32 = 600000u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: u32 = 300000u32;
pub const XAUDIO2_MAX_SAMPLE_RATE: u32 = 200000u32;
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216f32;
pub const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1000u32;
pub const XAUDIO2_NO_LOOP_REGION: u32 = 0u32;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: u32 = 65536u32;
pub const XAUDIO2_PLAY_TAILS: u32 = 32u32;
pub const XAUDIO2_QUANTUM_DENOMINATOR: u32 = 100u32;
pub const XAUDIO2_QUANTUM_NUMERATOR: u32 = 1u32;
pub const XAUDIO2_SEND_USEFILTER: u32 = 128u32;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: u32 = 8192u32;
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: u32 = 0u32;
pub const XAUDIO2_VOICE_NOPITCH: u32 = 2u32;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 256u32;
pub const XAUDIO2_VOICE_NOSRC: u32 = 4u32;
pub const XAUDIO2_VOICE_USEFILTER: u32 = 8u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HrtfDirectivityType(pub i32);
impl ::core::marker::Copy for HrtfDirectivityType {}
impl ::core::clone::Clone for HrtfDirectivityType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HrtfDirectivityType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HrtfDirectivityType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HrtfDirectivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HrtfDirectivityType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HrtfDistanceDecayType(pub i32);
impl ::core::marker::Copy for HrtfDistanceDecayType {}
impl ::core::clone::Clone for HrtfDistanceDecayType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HrtfDistanceDecayType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HrtfDistanceDecayType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HrtfDistanceDecayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HrtfDistanceDecayType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HrtfEnvironment(pub i32);
impl ::core::marker::Copy for HrtfEnvironment {}
impl ::core::clone::Clone for HrtfEnvironment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HrtfEnvironment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HrtfEnvironment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HrtfEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HrtfEnvironment").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XAPO_BUFFER_FLAGS(pub i32);
impl ::core::marker::Copy for XAPO_BUFFER_FLAGS {}
impl ::core::clone::Clone for XAPO_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XAPO_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XAPO_BUFFER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XAPO_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XAPO_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XAUDIO2_FILTER_TYPE(pub i32);
impl ::core::marker::Copy for XAUDIO2_FILTER_TYPE {}
impl ::core::clone::Clone for XAUDIO2_FILTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XAUDIO2_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XAUDIO2_FILTER_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XAUDIO2_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XAUDIO2_FILTER_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
pub struct FXECHO_INITDATA {
    pub MaxDelay: f32,
}
impl ::core::marker::Copy for FXECHO_INITDATA {}
impl ::core::clone::Clone for FXECHO_INITDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for FXECHO_INITDATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for FXECHO_INITDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct FXECHO_PARAMETERS {
    pub WetDryMix: f32,
    pub Feedback: f32,
    pub Delay: f32,
}
impl ::core::marker::Copy for FXECHO_PARAMETERS {}
impl ::core::clone::Clone for FXECHO_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for FXECHO_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for FXECHO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct FXEQ_PARAMETERS {
    pub FrequencyCenter0: f32,
    pub Gain0: f32,
    pub Bandwidth0: f32,
    pub FrequencyCenter1: f32,
    pub Gain1: f32,
    pub Bandwidth1: f32,
    pub FrequencyCenter2: f32,
    pub Gain2: f32,
    pub Bandwidth2: f32,
    pub FrequencyCenter3: f32,
    pub Gain3: f32,
    pub Bandwidth3: f32,
}
impl ::core::marker::Copy for FXEQ_PARAMETERS {}
impl ::core::clone::Clone for FXEQ_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for FXEQ_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for FXEQ_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct FXMASTERINGLIMITER_PARAMETERS {
    pub Release: u32,
    pub Loudness: u32,
}
impl ::core::marker::Copy for FXMASTERINGLIMITER_PARAMETERS {}
impl ::core::clone::Clone for FXMASTERINGLIMITER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for FXMASTERINGLIMITER_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for FXMASTERINGLIMITER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct FXREVERB_PARAMETERS {
    pub Diffusion: f32,
    pub RoomSize: f32,
}
impl ::core::marker::Copy for FXREVERB_PARAMETERS {}
impl ::core::clone::Clone for FXREVERB_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for FXREVERB_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for FXREVERB_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HrtfApoInit {
    pub distanceDecay: *mut HrtfDistanceDecay,
    pub directivity: *mut HrtfDirectivity,
}
impl ::core::marker::Copy for HrtfApoInit {}
impl ::core::clone::Clone for HrtfApoInit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HrtfApoInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfApoInit").field("distanceDecay", &self.distanceDecay).field("directivity", &self.directivity).finish()
    }
}
impl ::windows_core::TypeKind for HrtfApoInit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HrtfApoInit {
    fn eq(&self, other: &Self) -> bool {
        self.distanceDecay == other.distanceDecay && self.directivity == other.directivity
    }
}
impl ::core::cmp::Eq for HrtfApoInit {}
impl ::core::default::Default for HrtfApoInit {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HrtfDirectivity {
    pub r#type: HrtfDirectivityType,
    pub scaling: f32,
}
impl ::core::marker::Copy for HrtfDirectivity {}
impl ::core::clone::Clone for HrtfDirectivity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HrtfDirectivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDirectivity").field("type", &self.r#type).field("scaling", &self.scaling).finish()
    }
}
impl ::windows_core::TypeKind for HrtfDirectivity {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HrtfDirectivity {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.scaling == other.scaling
    }
}
impl ::core::cmp::Eq for HrtfDirectivity {}
impl ::core::default::Default for HrtfDirectivity {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HrtfDirectivityCardioid {
    pub directivity: HrtfDirectivity,
    pub order: f32,
}
impl ::core::marker::Copy for HrtfDirectivityCardioid {}
impl ::core::clone::Clone for HrtfDirectivityCardioid {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HrtfDirectivityCardioid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDirectivityCardioid").field("directivity", &self.directivity).field("order", &self.order).finish()
    }
}
impl ::windows_core::TypeKind for HrtfDirectivityCardioid {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HrtfDirectivityCardioid {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity && self.order == other.order
    }
}
impl ::core::cmp::Eq for HrtfDirectivityCardioid {}
impl ::core::default::Default for HrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HrtfDirectivityCone {
    pub directivity: HrtfDirectivity,
    pub innerAngle: f32,
    pub outerAngle: f32,
}
impl ::core::marker::Copy for HrtfDirectivityCone {}
impl ::core::clone::Clone for HrtfDirectivityCone {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HrtfDirectivityCone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDirectivityCone").field("directivity", &self.directivity).field("innerAngle", &self.innerAngle).field("outerAngle", &self.outerAngle).finish()
    }
}
impl ::windows_core::TypeKind for HrtfDirectivityCone {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HrtfDirectivityCone {
    fn eq(&self, other: &Self) -> bool {
        self.directivity == other.directivity && self.innerAngle == other.innerAngle && self.outerAngle == other.outerAngle
    }
}
impl ::core::cmp::Eq for HrtfDirectivityCone {}
impl ::core::default::Default for HrtfDirectivityCone {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HrtfDistanceDecay {
    pub r#type: HrtfDistanceDecayType,
    pub maxGain: f32,
    pub minGain: f32,
    pub unityGainDistance: f32,
    pub cutoffDistance: f32,
}
impl ::core::marker::Copy for HrtfDistanceDecay {}
impl ::core::clone::Clone for HrtfDistanceDecay {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HrtfDistanceDecay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfDistanceDecay").field("type", &self.r#type).field("maxGain", &self.maxGain).field("minGain", &self.minGain).field("unityGainDistance", &self.unityGainDistance).field("cutoffDistance", &self.cutoffDistance).finish()
    }
}
impl ::windows_core::TypeKind for HrtfDistanceDecay {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HrtfDistanceDecay {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.maxGain == other.maxGain && self.minGain == other.minGain && self.unityGainDistance == other.unityGainDistance && self.cutoffDistance == other.cutoffDistance
    }
}
impl ::core::cmp::Eq for HrtfDistanceDecay {}
impl ::core::default::Default for HrtfDistanceDecay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HrtfOrientation {
    pub element: [f32; 9],
}
impl ::core::marker::Copy for HrtfOrientation {}
impl ::core::clone::Clone for HrtfOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HrtfOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfOrientation").field("element", &self.element).finish()
    }
}
impl ::windows_core::TypeKind for HrtfOrientation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HrtfOrientation {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}
impl ::core::cmp::Eq for HrtfOrientation {}
impl ::core::default::Default for HrtfOrientation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HrtfPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ::core::marker::Copy for HrtfPosition {}
impl ::core::clone::Clone for HrtfPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HrtfPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HrtfPosition").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl ::windows_core::TypeKind for HrtfPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HrtfPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for HrtfPosition {}
impl ::core::default::Default for HrtfPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAPO_LOCKFORPROCESS_PARAMETERS {
    pub pFormat: *const super::WAVEFORMATEX,
    pub MaxFrameCount: u32,
}
impl ::core::marker::Copy for XAPO_LOCKFORPROCESS_PARAMETERS {}
impl ::core::clone::Clone for XAPO_LOCKFORPROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAPO_LOCKFORPROCESS_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAPO_LOCKFORPROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAPO_PROCESS_BUFFER_PARAMETERS {
    pub pBuffer: *mut ::core::ffi::c_void,
    pub BufferFlags: XAPO_BUFFER_FLAGS,
    pub ValidFrameCount: u32,
}
impl ::core::marker::Copy for XAPO_PROCESS_BUFFER_PARAMETERS {}
impl ::core::clone::Clone for XAPO_PROCESS_BUFFER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAPO_PROCESS_BUFFER_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAPO_PROCESS_BUFFER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAPO_REGISTRATION_PROPERTIES {
    pub clsid: ::windows_core::GUID,
    pub FriendlyName: [u16; 256],
    pub CopyrightInfo: [u16; 256],
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub Flags: u32,
    pub MinInputBufferCount: u32,
    pub MaxInputBufferCount: u32,
    pub MinOutputBufferCount: u32,
    pub MaxOutputBufferCount: u32,
}
impl ::core::marker::Copy for XAPO_REGISTRATION_PROPERTIES {}
impl ::core::clone::Clone for XAPO_REGISTRATION_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAPO_REGISTRATION_PROPERTIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAPO_REGISTRATION_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    pub WetDryMix: f32,
    pub Room: i32,
    pub RoomHF: i32,
    pub RoomRolloffFactor: f32,
    pub DecayTime: f32,
    pub DecayHFRatio: f32,
    pub Reflections: i32,
    pub ReflectionsDelay: f32,
    pub Reverb: i32,
    pub ReverbDelay: f32,
    pub Diffusion: f32,
    pub Density: f32,
    pub HFReference: f32,
}
impl ::core::marker::Copy for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {}
impl ::core::clone::Clone for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2FX_REVERB_PARAMETERS {
    pub WetDryMix: f32,
    pub ReflectionsDelay: u32,
    pub ReverbDelay: u8,
    pub RearDelay: u8,
    pub SideDelay: u8,
    pub PositionLeft: u8,
    pub PositionRight: u8,
    pub PositionMatrixLeft: u8,
    pub PositionMatrixRight: u8,
    pub EarlyDiffusion: u8,
    pub LateDiffusion: u8,
    pub LowEQGain: u8,
    pub LowEQCutoff: u8,
    pub HighEQGain: u8,
    pub HighEQCutoff: u8,
    pub RoomFilterFreq: f32,
    pub RoomFilterMain: f32,
    pub RoomFilterHF: f32,
    pub ReflectionsGain: f32,
    pub ReverbGain: f32,
    pub DecayTime: f32,
    pub Density: f32,
    pub RoomSize: f32,
    pub DisableLateField: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XAUDIO2FX_REVERB_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XAUDIO2FX_REVERB_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for XAUDIO2FX_REVERB_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2FX_REVERB_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2FX_VOLUMEMETER_LEVELS {
    pub pPeakLevels: *mut f32,
    pub pRMSLevels: *mut f32,
    pub ChannelCount: u32,
}
impl ::core::marker::Copy for XAUDIO2FX_VOLUMEMETER_LEVELS {}
impl ::core::clone::Clone for XAUDIO2FX_VOLUMEMETER_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2FX_VOLUMEMETER_LEVELS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2FX_VOLUMEMETER_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_BUFFER {
    pub Flags: u32,
    pub AudioBytes: u32,
    pub pAudioData: *const u8,
    pub PlayBegin: u32,
    pub PlayLength: u32,
    pub LoopBegin: u32,
    pub LoopLength: u32,
    pub LoopCount: u32,
    pub pContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for XAUDIO2_BUFFER {}
impl ::core::clone::Clone for XAUDIO2_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2_BUFFER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_BUFFER_WMA {
    pub pDecodedPacketCumulativeBytes: *const u32,
    pub PacketCount: u32,
}
impl ::core::marker::Copy for XAUDIO2_BUFFER_WMA {}
impl ::core::clone::Clone for XAUDIO2_BUFFER_WMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2_BUFFER_WMA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_BUFFER_WMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_DEBUG_CONFIGURATION {
    pub TraceMask: u32,
    pub BreakMask: u32,
    pub LogThreadID: super::super::super::Foundation::BOOL,
    pub LogFileline: super::super::super::Foundation::BOOL,
    pub LogFunctionName: super::super::super::Foundation::BOOL,
    pub LogTiming: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XAUDIO2_DEBUG_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XAUDIO2_DEBUG_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for XAUDIO2_DEBUG_CONFIGURATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2_DEBUG_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount: u32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XAUDIO2_EFFECT_CHAIN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XAUDIO2_EFFECT_CHAIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for XAUDIO2_EFFECT_CHAIN {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2_EFFECT_CHAIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect: ::std::mem::ManuallyDrop<::core::option::Option<::windows_core::IUnknown>>,
    pub InitialState: super::super::super::Foundation::BOOL,
    pub OutputChannels: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for XAUDIO2_EFFECT_DESCRIPTOR {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XAUDIO2_EFFECT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_FILTER_PARAMETERS {
    pub Type: XAUDIO2_FILTER_TYPE,
    pub Frequency: f32,
    pub OneOverQ: f32,
}
impl ::core::marker::Copy for XAUDIO2_FILTER_PARAMETERS {}
impl ::core::clone::Clone for XAUDIO2_FILTER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2_FILTER_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_FILTER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_PERFORMANCE_DATA {
    pub AudioCyclesSinceLastQuery: u64,
    pub TotalCyclesSinceLastQuery: u64,
    pub MinimumCyclesPerQuantum: u32,
    pub MaximumCyclesPerQuantum: u32,
    pub MemoryUsageInBytes: u32,
    pub CurrentLatencyInSamples: u32,
    pub GlitchesSinceEngineStarted: u32,
    pub ActiveSourceVoiceCount: u32,
    pub TotalSourceVoiceCount: u32,
    pub ActiveSubmixVoiceCount: u32,
    pub ActiveResamplerCount: u32,
    pub ActiveMatrixMixCount: u32,
    pub ActiveXmaSourceVoices: u32,
    pub ActiveXmaStreams: u32,
}
impl ::core::marker::Copy for XAUDIO2_PERFORMANCE_DATA {}
impl ::core::clone::Clone for XAUDIO2_PERFORMANCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2_PERFORMANCE_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: u32,
    pub pOutputVoice: ::std::mem::ManuallyDrop<::core::option::Option<IXAudio2Voice>>,
}
impl ::windows_core::TypeKind for XAUDIO2_SEND_DESCRIPTOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_SEND_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: u32,
    pub ActiveFlags: u32,
    pub InputChannels: u32,
    pub InputSampleRate: u32,
}
impl ::core::marker::Copy for XAUDIO2_VOICE_DETAILS {}
impl ::core::clone::Clone for XAUDIO2_VOICE_DETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2_VOICE_DETAILS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_VOICE_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: u32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}
impl ::core::marker::Copy for XAUDIO2_VOICE_SENDS {}
impl ::core::clone::Clone for XAUDIO2_VOICE_SENDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2_VOICE_SENDS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_VOICE_SENDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut ::core::ffi::c_void,
    pub BuffersQueued: u32,
    pub SamplesPlayed: u64,
}
impl ::core::marker::Copy for XAUDIO2_VOICE_STATE {}
impl ::core::clone::Clone for XAUDIO2_VOICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for XAUDIO2_VOICE_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XAUDIO2_VOICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
