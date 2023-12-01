#[inline]
pub unsafe fn DMOEnum(guidcategory: *const ::windows_core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows_core::Result<IEnumDMO> {
    ::windows_targets::link!("msdmo.dll" "system" fn DMOEnum(guidcategory : *const ::windows_core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE, ppenum : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    DMOEnum(guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn DMOGetName(clsiddmo: *const ::windows_core::GUID, szname: &mut [u16; 80]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn DMOGetName(clsiddmo : *const ::windows_core::GUID, szname : ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    DMOGetName(clsiddmo, ::core::mem::transmute(szname.as_ptr())).ok()
}
#[inline]
pub unsafe fn DMOGetTypes(clsiddmo: *const ::windows_core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn DMOGetTypes(clsiddmo : *const ::windows_core::GUID, ulinputtypesrequested : u32, pulinputtypessupplied : *mut u32, pinputtypes : *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested : u32, puloutputtypessupplied : *mut u32, poutputtypes : *mut DMO_PARTIAL_MEDIATYPE) -> ::windows_core::HRESULT);
    DMOGetTypes(clsiddmo, ulinputtypesrequested, pulinputtypessupplied, pinputtypes, uloutputtypesrequested, puloutputtypessupplied, poutputtypes).ok()
}
#[inline]
pub unsafe fn DMORegister<P0>(szname: P0, clsiddmo: *const ::windows_core::GUID, guidcategory: *const ::windows_core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msdmo.dll" "system" fn DMORegister(szname : ::windows_core::PCWSTR, clsiddmo : *const ::windows_core::GUID, guidcategory : *const ::windows_core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE) -> ::windows_core::HRESULT);
    DMORegister(szname.into_param().abi(), clsiddmo, guidcategory, dwflags, cintypes, pintypes, couttypes, pouttypes).ok()
}
#[inline]
pub unsafe fn DMOUnregister(clsiddmo: *const ::windows_core::GUID, guidcategory: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn DMOUnregister(clsiddmo : *const ::windows_core::GUID, guidcategory : *const ::windows_core::GUID) -> ::windows_core::HRESULT);
    DMOUnregister(clsiddmo, guidcategory).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> ::windows_core::HRESULT);
    MoCopyMediaType(pmtdest, pmtsrc).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut DMO_MEDIA_TYPE, cbformat : u32) -> ::windows_core::HRESULT);
    MoCreateMediaType(ppmt, cbformat).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT);
    MoDeleteMediaType(pmt).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> ::windows_core::HRESULT);
    MoDuplicateMediaType(ppmtdest, pmtsrc).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT);
    MoFreeMediaType(pmt).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut DMO_MEDIA_TYPE, cbformat : u32) -> ::windows_core::HRESULT);
    MoInitMediaType(pmt, cbformat).ok()
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDMOQualityControl(::windows_core::IUnknown);
impl IDMOQualityControl {
    pub unsafe fn SetNow(&self, rtnow: i64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetNow(rtnow)).ok()
    }
    pub unsafe fn SetStatus(&self, dwflags: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetStatus(dwflags)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStatus(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDMOQualityControl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDMOQualityControl {
    type Vtable = IDMOQualityControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDMOQualityControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65abea96_cf36_453f_af8a_705e98f16260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOQualityControl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDMOVideoOutputOptimizations(::windows_core::IUnknown);
impl IDMOVideoOutputOptimizations {
    pub unsafe fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.QueryOperationModePreferences(uloutputstreamindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOperationMode(uloutputstreamindex, dwenabledfeatures)).ok()
    }
    pub unsafe fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCurrentOperationMode(uloutputstreamindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCurrentSampleRequirements(uloutputstreamindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDMOVideoOutputOptimizations, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDMOVideoOutputOptimizations {
    type Vtable = IDMOVideoOutputOptimizations_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDMOVideoOutputOptimizations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe8f4f4e_5b16_4d29_b350_7f6b5d9298ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMOVideoOutputOptimizations_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub QueryOperationModePreferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows_core::HRESULT,
    pub SetOperationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows_core::HRESULT,
    pub GetCurrentOperationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows_core::HRESULT,
    pub GetCurrentSampleRequirements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumDMO(::windows_core::IUnknown);
impl IEnumDMO {
    pub unsafe fn Next(&self, citemstofetch: u32, pclsid: *mut ::windows_core::GUID, names: *mut ::windows_core::PWSTR, pcitemsfetched: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Next(citemstofetch, pclsid, names, pcitemsfetched)).ok()
    }
    pub unsafe fn Skip(&self, citemstoskip: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Skip(citemstoskip)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Reset()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumDMO> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.Clone(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumDMO, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumDMO {
    type Vtable = IEnumDMO_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumDMO {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c3cd98a_2bfa_4a53_9c27_5249ba64ba0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDMO_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, citemstofetch: u32, pclsid: *mut ::windows_core::GUID, names: *mut ::windows_core::PWSTR, pcitemsfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, citemstoskip: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaBuffer(::windows_core::IUnknown);
impl IMediaBuffer {
    pub unsafe fn SetLength(&self, cblength: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLength(cblength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMaxLength(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppbuffer: ::core::option::Option<*mut *mut u8>, pcblength: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetBufferAndLength(::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcblength.unwrap_or(::std::ptr::null_mut())))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMediaBuffer, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBuffer {
    type Vtable = IMediaBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59eff8b9_938c_4a26_82f2_95cb84cdc837);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBuffer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows_core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaObject(::windows_core::IUnknown);
impl IMediaObject {
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetStreamCount(pcinputstreams, pcoutputstreams)).ok()
    }
    pub unsafe fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInputStreamInfo(dwinputstreamindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutputStreamInfo(dwoutputstreamindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32, pmt: ::core::option::Option<*mut DMO_MEDIA_TYPE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetInputType(dwinputstreamindex, dwtypeindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: ::core::option::Option<*mut DMO_MEDIA_TYPE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetOutputType(dwoutputstreamindex, dwtypeindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInputType(&self, dwinputstreamindex: u32, pmt: ::core::option::Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInputType(dwinputstreamindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null())), dwflags)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: ::core::option::Option<*const DMO_MEDIA_TYPE>, dwflags: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOutputType(dwoutputstreamindex, ::core::mem::transmute(pmt.unwrap_or(::std::ptr::null())), dwflags)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetInputCurrentType(dwinputstreamindex, pmt)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetOutputCurrentType(dwoutputstreamindex, pmt)).ok()
    }
    pub unsafe fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetInputSizeInfo(dwinputstreamindex, pcbsize, pcbmaxlookahead, pcbalignment)).ok()
    }
    pub unsafe fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetOutputSizeInfo(dwoutputstreamindex, pcbsize, pcbalignment)).ok()
    }
    pub unsafe fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInputMaxLatency(dwinputstreamindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInputMaxLatency(dwinputstreamindex, rtmaxlatency)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Flush()).ok()
    }
    pub unsafe fn Discontinuity(&self, dwinputstreamindex: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Discontinuity(dwinputstreamindex)).ok()
    }
    pub unsafe fn AllocateStreamingResources(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AllocateStreamingResources()).ok()
    }
    pub unsafe fn FreeStreamingResources(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.FreeStreamingResources()).ok()
    }
    pub unsafe fn GetInputStatus(&self, dwinputstreamindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInputStatus(dwinputstreamindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn ProcessInput<P0>(&self, dwinputstreamindex: u32, pbuffer: P0, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMediaBuffer>,
    {
        ::windows_core::vcall!(self.ProcessInput(dwinputstreamindex, pbuffer.into_param().abi(), dwflags, rttimestamp, rttimelength)).ok()
    }
    pub unsafe fn ProcessOutput(&self, dwflags: u32, poutputbuffers: &mut [DMO_OUTPUT_DATA_BUFFER], pdwstatus: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.ProcessOutput(dwflags, poutputbuffers.len().try_into().unwrap(), ::core::mem::transmute(poutputbuffers.as_ptr()), pdwstatus)).ok()
    }
    pub unsafe fn Lock(&self, block: i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Lock(block)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMediaObject, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaObject {
    type Vtable = IMediaObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8ad0f58_5494_4102_97c5_ec798e59bcf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows_core::HRESULT,
    pub GetInputStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetOutputStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInputCurrentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInputCurrentType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputCurrentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputCurrentType: usize,
    pub GetInputSizeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows_core::HRESULT,
    pub GetOutputSizeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows_core::HRESULT,
    pub GetInputMaxLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows_core::HRESULT,
    pub SetInputMaxLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32) -> ::windows_core::HRESULT,
    pub AllocateStreamingResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FreeStreamingResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetInputStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows_core::HRESULT,
    pub ProcessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pbuffer: *mut ::core::ffi::c_void, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows_core::HRESULT,
    pub ProcessOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows_core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaObjectInPlace(::windows_core::IUnknown);
impl IMediaObjectInPlace {
    pub unsafe fn Process(&self, pdata: &mut [u8], reftimestart: i64, dwflags: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Process(pdata.len().try_into().unwrap(), ::core::mem::transmute(pdata.as_ptr()), reftimestart, dwflags)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IMediaObjectInPlace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.Clone(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLatency(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLatency(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMediaObjectInPlace, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaObjectInPlace {
    type Vtable = IMediaObjectInPlace_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaObjectInPlace {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x651b9ad0_0fc7_4aa9_9538_d89931010741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaObjectInPlace_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmediaobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows_core::HRESULT,
}
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AGC: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf665aaba_3e09_4920_aa5f_219811148f09);
pub const DMOCATEGORY_AUDIO_DECODER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57f2db8b_e6bb_4513_9d43_dcd2a6593125);
pub const DMOCATEGORY_AUDIO_EFFECT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3602b3f_0592_48df_a4cd_674721e7ebeb);
pub const DMOCATEGORY_AUDIO_ENCODER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const DMOCATEGORY_VIDEO_DECODER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a69b442_28be_4991_969c_b500adf5d8a8);
pub const DMOCATEGORY_VIDEO_EFFECT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd990ee14_776c_4723_be46_3da2f56f10b9);
pub const DMOCATEGORY_VIDEO_ENCODER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = DMO_ENUM_FLAGS(1i32);
pub const DMO_E_INVALIDSTREAMINDEX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220991i32);
pub const DMO_E_INVALIDTYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220990i32);
pub const DMO_E_NOTACCEPTING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220988i32);
pub const DMO_E_NO_MORE_ITEMS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220986i32);
pub const DMO_E_TYPE_NOT_ACCEPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220987i32);
pub const DMO_E_TYPE_NOT_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220989i32);
pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(0i32);
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = _DMO_INPLACE_PROCESS_FLAGS(1i32);
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(8i32);
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(1i32);
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(2i32);
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = _DMO_INPUT_DATA_BUFFER_FLAGS(4i32);
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = _DMO_INPUT_STATUS_FLAGS(1i32);
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(4i32);
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(8i32);
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(2i32);
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = _DMO_INPUT_STREAM_INFO_FLAGS(1i32);
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(8i32);
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(16777216i32);
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(1i32);
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(2i32);
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = _DMO_OUTPUT_DATA_BUFFER_FLAGS(4i32);
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(8i32);
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(4i32);
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(16i32);
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(2i32);
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = _DMO_OUTPUT_STREAM_INFO_FLAGS(1i32);
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = _DMO_PROCESS_OUTPUT_FLAGS(1i32);
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = _DMO_QUALITY_STATUS_FLAGS(1i32);
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = DMO_REGISTER_FLAGS(1i32);
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(2i32);
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = _DMO_SET_TYPE_FLAGS(1i32);
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = _DMO_VIDEO_OUTPUT_STREAM_FLAGS(1i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DMO_ENUM_FLAGS(pub i32);
impl ::core::marker::Copy for DMO_ENUM_FLAGS {}
impl ::core::clone::Clone for DMO_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DMO_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DMO_ENUM_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DMO_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DMO_REGISTER_FLAGS(pub i32);
impl ::core::marker::Copy for DMO_REGISTER_FLAGS {}
impl ::core::clone::Clone for DMO_REGISTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DMO_REGISTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DMO_REGISTER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DMO_REGISTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_REGISTER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPLACE_PROCESS_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_INPLACE_PROCESS_FLAGS {}
impl ::core::clone::Clone for _DMO_INPLACE_PROCESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPLACE_PROCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_INPLACE_PROCESS_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_INPLACE_PROCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPLACE_PROCESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPUT_DATA_BUFFER_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_INPUT_DATA_BUFFER_FLAGS {}
impl ::core::clone::Clone for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_INPUT_DATA_BUFFER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPUT_STATUS_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_INPUT_STATUS_FLAGS {}
impl ::core::clone::Clone for _DMO_INPUT_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPUT_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_INPUT_STATUS_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_INPUT_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_INPUT_STREAM_INFO_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_INPUT_STREAM_INFO_FLAGS {}
impl ::core::clone::Clone for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_INPUT_STREAM_INFO_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_OUTPUT_DATA_BUFFER_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_OUTPUT_DATA_BUFFER_FLAGS {}
impl ::core::clone::Clone for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_OUTPUT_STREAM_INFO_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_OUTPUT_STREAM_INFO_FLAGS {}
impl ::core::clone::Clone for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_PROCESS_OUTPUT_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_PROCESS_OUTPUT_FLAGS {}
impl ::core::clone::Clone for _DMO_PROCESS_OUTPUT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_PROCESS_OUTPUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_PROCESS_OUTPUT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_PROCESS_OUTPUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_PROCESS_OUTPUT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_QUALITY_STATUS_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_QUALITY_STATUS_FLAGS {}
impl ::core::clone::Clone for _DMO_QUALITY_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_QUALITY_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_QUALITY_STATUS_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_QUALITY_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_QUALITY_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_SET_TYPE_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_SET_TYPE_FLAGS {}
impl ::core::clone::Clone for _DMO_SET_TYPE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_SET_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_SET_TYPE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_SET_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_SET_TYPE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DMO_VIDEO_OUTPUT_STREAM_FLAGS(pub i32);
impl ::core::marker::Copy for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {}
impl ::core::clone::Clone for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_VIDEO_OUTPUT_STREAM_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DMO_MEDIA_TYPE {
    pub majortype: ::windows_core::GUID,
    pub subtype: ::windows_core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows_core::GUID,
    pub pUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows_core::IUnknown>>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DMO_MEDIA_TYPE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMO_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DMO_MEDIA_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMO_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMO_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: ::std::mem::ManuallyDrop<::core::option::Option<IMediaBuffer>>,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl ::core::clone::Clone for DMO_OUTPUT_DATA_BUFFER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for DMO_OUTPUT_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_OUTPUT_DATA_BUFFER").field("pBuffer", &self.pBuffer).field("dwStatus", &self.dwStatus).field("rtTimestamp", &self.rtTimestamp).field("rtTimelength", &self.rtTimelength).finish()
    }
}
impl ::windows_core::TypeKind for DMO_OUTPUT_DATA_BUFFER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DMO_OUTPUT_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.dwStatus == other.dwStatus && self.rtTimestamp == other.rtTimestamp && self.rtTimelength == other.rtTimelength
    }
}
impl ::core::cmp::Eq for DMO_OUTPUT_DATA_BUFFER {}
impl ::core::default::Default for DMO_OUTPUT_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: ::windows_core::GUID,
    pub subtype: ::windows_core::GUID,
}
impl ::core::marker::Copy for DMO_PARTIAL_MEDIATYPE {}
impl ::core::clone::Clone for DMO_PARTIAL_MEDIATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DMO_PARTIAL_MEDIATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_PARTIAL_MEDIATYPE").field("type", &self.r#type).field("subtype", &self.subtype).finish()
    }
}
impl ::windows_core::TypeKind for DMO_PARTIAL_MEDIATYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DMO_PARTIAL_MEDIATYPE {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.subtype == other.subtype
    }
}
impl ::core::cmp::Eq for DMO_PARTIAL_MEDIATYPE {}
impl ::core::default::Default for DMO_PARTIAL_MEDIATYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
