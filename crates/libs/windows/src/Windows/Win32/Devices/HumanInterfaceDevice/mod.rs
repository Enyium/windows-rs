#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectInput8Create<P0, P1>(hinst: P0, dwversion: u32, riidltf: *const ::windows_core::GUID, ppvout: *mut *mut ::core::ffi::c_void, punkouter: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("dinput8.dll" "system" fn DirectInput8Create(hinst : super::super::Foundation:: HINSTANCE, dwversion : u32, riidltf : *const ::windows_core::GUID, ppvout : *mut *mut ::core::ffi::c_void, punkouter : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    DirectInput8Create(hinst.into_param().abi(), dwversion, riidltf, ppvout, punkouter.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_FlushQueue<P0>(hiddeviceobject: P0) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOLEAN);
    HidD_FlushQueue(hiddeviceobject.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_FreePreparsedData<P0>(preparseddata: P0) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation:: BOOLEAN);
    HidD_FreePreparsedData(preparseddata.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetAttributes<P0>(hiddeviceobject: P0, attributes: *mut HIDD_ATTRIBUTES) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::super::Foundation:: HANDLE, attributes : *mut HIDD_ATTRIBUTES) -> super::super::Foundation:: BOOLEAN);
    HidD_GetAttributes(hiddeviceobject.into_param().abi(), attributes)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetConfiguration<P0>(hiddeviceobject: P0, configuration: *mut HIDD_CONFIGURATION, configurationlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::super::Foundation:: HANDLE, configuration : *mut HIDD_CONFIGURATION, configurationlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetConfiguration(hiddeviceobject.into_param().abi(), configuration, configurationlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetFeature<P0>(hiddeviceobject: P0, reportbuffer: *mut ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *mut ::core::ffi::c_void, reportbufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetFeature(hiddeviceobject.into_param().abi(), reportbuffer, reportbufferlength)
}
#[inline]
pub unsafe fn HidD_GetHidGuid() -> ::windows_core::GUID {
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : *mut ::windows_core::GUID));
    let mut result__ = ::std::mem::zeroed();
    HidD_GetHidGuid(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetIndexedString<P0>(hiddeviceobject: P0, stringindex: u32, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::super::Foundation:: HANDLE, stringindex : u32, buffer : *mut ::core::ffi::c_void, bufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetIndexedString(hiddeviceobject.into_param().abi(), stringindex, buffer, bufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetInputReport<P0>(hiddeviceobject: P0, reportbuffer: *mut ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *mut ::core::ffi::c_void, reportbufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetInputReport(hiddeviceobject.into_param().abi(), reportbuffer, reportbufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetManufacturerString<P0>(hiddeviceobject: P0, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut ::core::ffi::c_void, bufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetManufacturerString(hiddeviceobject.into_param().abi(), buffer, bufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetMsGenreDescriptor<P0>(hiddeviceobject: P0, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut ::core::ffi::c_void, bufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetMsGenreDescriptor(hiddeviceobject.into_param().abi(), buffer, bufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetNumInputBuffers<P0>(hiddeviceobject: P0, numberbuffers: *mut u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::super::Foundation:: HANDLE, numberbuffers : *mut u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetNumInputBuffers(hiddeviceobject.into_param().abi(), numberbuffers)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetPhysicalDescriptor<P0>(hiddeviceobject: P0, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut ::core::ffi::c_void, bufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetPhysicalDescriptor(hiddeviceobject.into_param().abi(), buffer, bufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetPreparsedData<P0>(hiddeviceobject: P0, preparseddata: *mut PHIDP_PREPARSED_DATA) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::super::Foundation:: HANDLE, preparseddata : *mut PHIDP_PREPARSED_DATA) -> super::super::Foundation:: BOOLEAN);
    HidD_GetPreparsedData(hiddeviceobject.into_param().abi(), preparseddata)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetProductString<P0>(hiddeviceobject: P0, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut ::core::ffi::c_void, bufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetProductString(hiddeviceobject.into_param().abi(), buffer, bufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetSerialNumberString<P0>(hiddeviceobject: P0, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut ::core::ffi::c_void, bufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_GetSerialNumberString(hiddeviceobject.into_param().abi(), buffer, bufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetConfiguration<P0>(hiddeviceobject: P0, configuration: *const HIDD_CONFIGURATION, configurationlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::super::Foundation:: HANDLE, configuration : *const HIDD_CONFIGURATION, configurationlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_SetConfiguration(hiddeviceobject.into_param().abi(), configuration, configurationlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetFeature<P0>(hiddeviceobject: P0, reportbuffer: *const ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *const ::core::ffi::c_void, reportbufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_SetFeature(hiddeviceobject.into_param().abi(), reportbuffer, reportbufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetNumInputBuffers<P0>(hiddeviceobject: P0, numberbuffers: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::super::Foundation:: HANDLE, numberbuffers : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_SetNumInputBuffers(hiddeviceobject.into_param().abi(), numberbuffers)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetOutputReport<P0>(hiddeviceobject: P0, reportbuffer: *const ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *const ::core::ffi::c_void, reportbufferlength : u32) -> super::super::Foundation:: BOOLEAN);
    HidD_SetOutputReport(hiddeviceobject.into_param().abi(), reportbuffer, reportbufferlength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetButtonArray<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, buttondata: *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength: *mut u16, preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetButtonArray(reporttype, usagepage, linkcollection, usage, buttondata, buttondatalength, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetButtonCaps<P0>(reporttype: HIDP_REPORT_TYPE, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: P0) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetButtonCaps(reporttype : HIDP_REPORT_TYPE, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation:: NTSTATUS);
    HidP_GetButtonCaps(reporttype, buttoncaps, buttoncapslength, preparseddata.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetCaps<P0>(preparseddata: P0, capabilities: *mut HIDP_CAPS) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetCaps(preparseddata : PHIDP_PREPARSED_DATA, capabilities : *mut HIDP_CAPS) -> super::super::Foundation:: NTSTATUS);
    HidP_GetCaps(preparseddata.into_param().abi(), capabilities)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetData<P0>(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: P0, report: &mut [u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetData(reporttype, datalist, datalength, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetExtendedAttributes<P0>(reporttype: HIDP_REPORT_TYPE, dataindex: u16, preparseddata: P0, attributes: *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes: *mut u32) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetExtendedAttributes(reporttype : HIDP_REPORT_TYPE, dataindex : u16, preparseddata : PHIDP_PREPARSED_DATA, attributes : *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes : *mut u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetExtendedAttributes(reporttype, dataindex, preparseddata.into_param().abi(), attributes, lengthattributes)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetLinkCollectionNodes<P0>(linkcollectionnodes: *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength: *mut u32, preparseddata: P0) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetLinkCollectionNodes(linkcollectionnodes : *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation:: NTSTATUS);
    HidP_GetLinkCollectionNodes(linkcollectionnodes, linkcollectionnodeslength, preparseddata.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetScaledUsageValue<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: *mut i32, preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut i32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetScaledUsageValue(reporttype, usagepage, linkcollection, usage, usagevalue, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetSpecificButtonCaps<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: P0) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetSpecificButtonCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation:: NTSTATUS);
    HidP_GetSpecificButtonCaps(reporttype, usagepage, linkcollection, usage, buttoncaps, buttoncapslength, preparseddata.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetSpecificValueCaps<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: P0) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetSpecificValueCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation:: NTSTATUS);
    HidP_GetSpecificValueCaps(reporttype, usagepage, linkcollection, usage, valuecaps, valuecapslength, preparseddata.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsageValue<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: *mut u32, preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetUsageValue(reporttype, usagepage, linkcollection, usage, usagevalue, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsageValueArray<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: &mut [u8], preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : ::windows_core::PSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetUsageValueArray(reporttype, usagepage, linkcollection, usage, ::core::mem::transmute(usagevalue.as_ptr()), usagevalue.len().try_into().unwrap(), preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsages<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usagelist: *mut u16, usagelength: *mut u32, preparseddata: P0, report: &mut [u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetUsages(reporttype, usagepage, linkcollection, usagelist, usagelength, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsagesEx<P0>(reporttype: HIDP_REPORT_TYPE, linkcollection: u16, buttonlist: *mut USAGE_AND_PAGE, usagelength: *mut u32, preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetUsagesEx(reporttype : HIDP_REPORT_TYPE, linkcollection : u16, buttonlist : *mut USAGE_AND_PAGE, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_GetUsagesEx(reporttype, linkcollection, buttonlist, usagelength, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetValueCaps<P0>(reporttype: HIDP_REPORT_TYPE, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: P0) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_GetValueCaps(reporttype : HIDP_REPORT_TYPE, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation:: NTSTATUS);
    HidP_GetValueCaps(reporttype, valuecaps, valuecapslength, preparseddata.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_InitializeReportForID<P0>(reporttype: HIDP_REPORT_TYPE, reportid: u8, preparseddata: P0, report: &mut [u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_InitializeReportForID(reporttype : HIDP_REPORT_TYPE, reportid : u8, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_InitializeReportForID(reporttype, reportid, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[inline]
pub unsafe fn HidP_MaxDataListLength<P0>(reporttype: HIDP_REPORT_TYPE, preparseddata: P0) -> u32
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_MaxDataListLength(reporttype : HIDP_REPORT_TYPE, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
    HidP_MaxDataListLength(reporttype, preparseddata.into_param().abi())
}
#[inline]
pub unsafe fn HidP_MaxUsageListLength<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, preparseddata: P0) -> u32
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_MaxUsageListLength(reporttype : HIDP_REPORT_TYPE, usagepage : u16, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
    HidP_MaxUsageListLength(reporttype, usagepage, preparseddata.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetButtonArray<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, buttondata: &[HIDP_BUTTON_ARRAY_DATA], preparseddata: P0, report: &mut [u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_SetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *const HIDP_BUTTON_ARRAY_DATA, buttondatalength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_SetButtonArray(reporttype, usagepage, linkcollection, usage, ::core::mem::transmute(buttondata.as_ptr()), buttondata.len().try_into().unwrap(), preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetData<P0>(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_SetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_SetData(reporttype, datalist, datalength, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetScaledUsageValue<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: i32, preparseddata: P0, report: &mut [u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_SetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : i32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_SetScaledUsageValue(reporttype, usagepage, linkcollection, usage, usagevalue, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetUsageValue<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: u32, preparseddata: P0, report: &mut [u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_SetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_SetUsageValue(reporttype, usagepage, linkcollection, usage, usagevalue, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetUsageValueArray<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: &[u8], preparseddata: P0, report: &mut [u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_SetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : ::windows_core::PCSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_SetUsageValueArray(reporttype, usagepage, linkcollection, usage, ::core::mem::transmute(usagevalue.as_ptr()), usagevalue.len().try_into().unwrap(), preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetUsages<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usagelist: *mut u16, usagelength: *mut u32, preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_SetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_SetUsages(reporttype, usagepage, linkcollection, usagelist, usagelength, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist: &[u16], keyaction: HIDP_KEYBOARD_DIRECTION, modifierstate: *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure: PHIDP_INSERT_SCANCODES, insertcodescontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::NTSTATUS {
    ::windows_targets::link!("hid.dll" "system" fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist : *const u16, usagelistlength : u32, keyaction : HIDP_KEYBOARD_DIRECTION, modifierstate : *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure : PHIDP_INSERT_SCANCODES, insertcodescontext : *const ::core::ffi::c_void) -> super::super::Foundation:: NTSTATUS);
    HidP_TranslateUsagesToI8042ScanCodes(::core::mem::transmute(changedusagelist.as_ptr()), changedusagelist.len().try_into().unwrap(), keyaction, modifierstate, insertcodesprocedure, ::core::mem::transmute(insertcodescontext.unwrap_or(::std::ptr::null())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_UnsetUsages<P0>(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usagelist: *mut u16, usagelength: *mut u32, preparseddata: P0, report: &[u8]) -> super::super::Foundation::NTSTATUS
where
    P0: ::windows_core::IntoParam<PHIDP_PREPARSED_DATA>,
{
    ::windows_targets::link!("hid.dll" "system" fn HidP_UnsetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : ::windows_core::PCSTR, reportlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_UnsetUsages(reporttype, usagepage, linkcollection, usagelist, usagelength, preparseddata.into_param().abi(), ::core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_UsageListDifference(previoususagelist: *const u16, currentusagelist: *const u16, breakusagelist: *mut u16, makeusagelist: *mut u16, usagelistlength: u32) -> super::super::Foundation::NTSTATUS {
    ::windows_targets::link!("hid.dll" "system" fn HidP_UsageListDifference(previoususagelist : *const u16, currentusagelist : *const u16, breakusagelist : *mut u16, makeusagelist : *mut u16, usagelistlength : u32) -> super::super::Foundation:: NTSTATUS);
    HidP_UsageListDifference(previoususagelist, currentusagelist, breakusagelist, makeusagelist, usagelistlength)
}
#[inline]
pub unsafe fn joyConfigChanged(dwflags: u32) -> u32 {
    ::windows_targets::link!("winmm.dll" "system" fn joyConfigChanged(dwflags : u32) -> u32);
    joyConfigChanged(dwflags)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInput2A(::windows_core::IUnknown);
impl IDirectInput2A {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: P0, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).FindDevice)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInput2A, ::windows_core::IUnknown, IDirectInputA);
unsafe impl ::windows_core::Interface for IDirectInput2A {
    type Vtable = IDirectInput2A_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInput2A {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5944e662_aa8a_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput2A_Vtbl {
    pub base__: IDirectInputA_Vtbl,
    pub FindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInput2W(::windows_core::IUnknown);
impl IDirectInput2W {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: P0, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FindDevice)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInput2W, ::windows_core::IUnknown, IDirectInputW);
unsafe impl ::windows_core::Interface for IDirectInput2W {
    type Vtable = IDirectInput2W_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInput2W {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5944e663_aa8a_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput2W_Vtbl {
    pub base__: IDirectInputW_Vtbl,
    pub FindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCWSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInput7A(::windows_core::IUnknown);
impl IDirectInput7A {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: P0, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.FindDevice)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    pub unsafe fn CreateDeviceEx<P0>(&self, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateDeviceEx)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInput7A, ::windows_core::IUnknown, IDirectInputA, IDirectInput2A);
unsafe impl ::windows_core::Interface for IDirectInput7A {
    type Vtable = IDirectInput7A_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInput7A {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a4cb684_236d_11d3_8e9d_00c04f6844ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput7A_Vtbl {
    pub base__: IDirectInput2A_Vtbl,
    pub CreateDeviceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInput7W(::windows_core::IUnknown);
impl IDirectInput7W {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: P0, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.FindDevice)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    pub unsafe fn CreateDeviceEx<P0>(&self, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateDeviceEx)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInput7W, ::windows_core::IUnknown, IDirectInputW, IDirectInput2W);
unsafe impl ::windows_core::Interface for IDirectInput7W {
    type Vtable = IDirectInput7W_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInput7W {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a4cb685_236d_11d3_8e9d_00c04f6844ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput7W_Vtbl {
    pub base__: IDirectInput2W_Vtbl,
    pub CreateDeviceEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *const ::windows_core::GUID, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInput8A(::windows_core::IUnknown);
impl IDirectInput8A {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8A>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: P0, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).FindDevice)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevicesBySemantics<P0>(&self, param0: P0, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumDevicesBySemantics)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3, param4).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInput8A, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInput8A {
    type Vtable = IDirectInput8A_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInput8A {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf798030_483a_4da2_aa99_5d64ed369700);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput8A_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevices: usize,
    pub GetDeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub FindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevicesBySemantics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevicesBySemantics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigureDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigureDevices: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInput8W(::windows_core::IUnknown);
impl IDirectInput8W {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDevice8W>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: P0, param2: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).FindDevice)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevicesBySemantics<P0>(&self, param0: P0, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumDevicesBySemantics)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3, param4).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConfigureDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInput8W, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInput8W {
    type Vtable = IDirectInput8W_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInput8W {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf798031_483a_4da2_aa99_5d64ed369700);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput8W_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevices: usize,
    pub GetDeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub FindDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: ::windows_core::PCWSTR, param2: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevicesBySemantics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut ::core::ffi::c_void, param4: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevicesBySemantics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigureDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigureDevices: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputA(::windows_core::IUnknown);
impl IDirectInputA {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputA, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputA {
    type Vtable = IDirectInputA_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputA {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89521360_aa8a_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputA_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevices: usize,
    pub GetDeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDevice2A(::windows_core::IUnknown);
impl IDirectInputDevice2A {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateEffect)(::windows_core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumEffects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectInfo)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForceFeedbackState)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendForceFeedbackCommand)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Poll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDevice2A, ::windows_core::IUnknown, IDirectInputDeviceA);
unsafe impl ::windows_core::Interface for IDirectInputDevice2A {
    type Vtable = IDirectInputDevice2A_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDevice2A {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5944e682_c92e_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice2A_Vtbl {
    pub base__: IDirectInputDeviceA_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffects: usize,
    pub GetEffectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumCreatedEffectObjects: usize,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDevice2W(::windows_core::IUnknown);
impl IDirectInputDevice2W {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateEffect)(::windows_core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumEffects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectInfo)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForceFeedbackState)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendForceFeedbackCommand)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Poll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDevice2W, ::windows_core::IUnknown, IDirectInputDeviceW);
unsafe impl ::windows_core::Interface for IDirectInputDevice2W {
    type Vtable = IDirectInputDevice2W_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDevice2W {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5944e683_c92e_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice2W_Vtbl {
    pub base__: IDirectInputDeviceW_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffects: usize,
    pub GetEffectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumCreatedEffectObjects: usize,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDevice7A(::windows_core::IUnknown);
impl IDirectInputDevice7A {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.CreateEffect)(::windows_core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumEffects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEffectInfo)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetForceFeedbackState)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendForceFeedbackCommand)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumCreatedEffectObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Escape)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Poll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumEffectsInFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteEffectToFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDevice7A, ::windows_core::IUnknown, IDirectInputDeviceA, IDirectInputDevice2A);
unsafe impl ::windows_core::Interface for IDirectInputDevice7A {
    type Vtable = IDirectInputDevice7A_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDevice7A {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57d7c6bc_2356_11d3_8e9d_00c04f6844ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice7A_Vtbl {
    pub base__: IDirectInputDevice2A_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffectsInFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffectsInFile: usize,
    pub WriteEffectToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDevice7W(::windows_core::IUnknown);
impl IDirectInputDevice7W {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).base__.CreateEffect)(::windows_core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumEffects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEffectInfo)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetForceFeedbackState)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendForceFeedbackCommand)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnumCreatedEffectObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Escape)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Poll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SendDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumEffectsInFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteEffectToFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDevice7W, ::windows_core::IUnknown, IDirectInputDeviceW, IDirectInputDevice2W);
unsafe impl ::windows_core::Interface for IDirectInputDevice7W {
    type Vtable = IDirectInputDevice7W_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDevice7W {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57d7c6bd_2356_11d3_8e9d_00c04f6844ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice7W_Vtbl {
    pub base__: IDirectInputDevice2W_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffectsInFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffectsInFile: usize,
    pub WriteEffectToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDevice8A(::windows_core::IUnknown);
impl IDirectInputDevice8A {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateEffect)(::windows_core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumEffects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectInfo)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForceFeedbackState)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendForceFeedbackCommand)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Poll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumEffectsInFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteEffectToFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BuildActionMap<P0>(&self, param0: *mut DIACTIONFORMATA, param1: P0, param2: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).BuildActionMap)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActionMap<P0>(&self, param0: *mut DIACTIONFORMATA, param1: P0, param2: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).SetActionMap)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImageInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDevice8A, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputDevice8A {
    type Vtable = IDirectInputDevice8A_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDevice8A {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54d41080_dc15_4833_a41b_748f73a38179);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice8A_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumObjects: usize,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffects: usize,
    pub GetEffectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumCreatedEffectObjects: usize,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffectsInFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffectsInFile: usize,
    pub WriteEffectToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BuildActionMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: ::windows_core::PCSTR, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BuildActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetActionMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: ::windows_core::PCSTR, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetImageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetImageInfo: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDevice8W(::windows_core::IUnknown);
impl IDirectInputDevice8W {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateEffect)(::windows_core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumEffects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectInfo)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForceFeedbackState)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendForceFeedbackCommand)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Poll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).EnumEffectsInFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteEffectToFile)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BuildActionMap<P0>(&self, param0: *mut DIACTIONFORMATW, param1: P0, param2: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).BuildActionMap)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActionMap<P0>(&self, param0: *mut DIACTIONFORMATW, param1: P0, param2: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetActionMap)(::windows_core::Interface::as_raw(self), param0, param1.into_param().abi(), param2).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetImageInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDevice8W, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputDevice8W {
    type Vtable = IDirectInputDevice8W_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDevice8W {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54d41081_dc15_4833_a41b_748f73a38179);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice8W_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumObjects: usize,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffects: usize,
    pub GetEffectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumCreatedEffectObjects: usize,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumEffectsInFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumEffectsInFile: usize,
    pub WriteEffectToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BuildActionMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: ::windows_core::PCWSTR, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BuildActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetActionMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: ::windows_core::PCWSTR, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetImageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetImageInfo: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDeviceA(::windows_core::IUnknown);
impl IDirectInputDeviceA {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDeviceA, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputDeviceA {
    type Vtable = IDirectInputDeviceA_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDeviceA {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5944e680_c92e_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDeviceA_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumObjects: usize,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputDeviceW(::windows_core::IUnknown);
impl IDirectInputDeviceW {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumObjects)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceData)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDataFormat)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetEventNotification)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectInfo)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceInfo)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputDeviceW, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputDeviceW {
    type Vtable = IDirectInputDeviceW_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputDeviceW {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5944e681_c92e_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDeviceW_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVCAPS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumObjects: usize,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut DIPROPHEADER) -> ::windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDATAFORMAT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputEffect(::windows_core::IUnknown);
impl IDirectInputEffect {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn GetEffectGuid(&self, param0: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectGuid)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetParameters)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameters)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Start(&self, param0: u32, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetEffectStatus(&self, param0: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Download(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Download)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unload(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unload)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), param0).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputEffect, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputEffect {
    type Vtable = IDirectInputEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7e1f7c0_88d2_11d0_9ad0_00a0c9a06e35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputEffect_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetEffectGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEffectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIEFFESCAPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputEffectDriver(::windows_core::IUnknown);
impl IDirectInputEffectDriver {
    pub unsafe fn DeviceID(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeviceID)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3, param4).ok()
    }
    pub unsafe fn GetVersions(&self, param0: *mut DIDRIVERVERSIONS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVersions)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Escape(&self, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Escape)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn SetGain(&self, param0: u32, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGain)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendForceFeedbackCommand)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetForceFeedbackState)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn DownloadEffect(&self, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DownloadEffect)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3, param4).ok()
    }
    pub unsafe fn DestroyEffect(&self, param0: u32, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DestroyEffect)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn StartEffect(&self, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartEffect)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn StopEffect(&self, param0: u32, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopEffect)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetEffectStatus(&self, param0: u32, param1: u32, param2: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffectStatus)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputEffectDriver, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputEffectDriver {
    type Vtable = IDirectInputEffectDriver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputEffectDriver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02538130_898f_11d0_9ad0_00a0c9a06e35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputEffectDriver_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub DeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVersions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> ::windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> ::windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> ::windows_core::HRESULT,
    pub DownloadEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> ::windows_core::HRESULT,
    pub DestroyEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT,
    pub StartEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> ::windows_core::HRESULT,
    pub StopEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32) -> ::windows_core::HRESULT,
    pub GetEffectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputJoyConfig(::windows_core::IUnknown);
impl IDirectInputJoyConfig {
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn SendNotify(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendNotify)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumTypes)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetTypeInfo<P0>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetTypeInfo)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn SetTypeInfo<P0>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTypeInfo)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn DeleteType<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteType)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConfig)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConfig)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn DeleteConfig(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteConfig)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserValues)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserValues)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddNewHardware<P0>(&self, param0: P0, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).AddNewHardware)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_System_Registry\"`"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenTypeKey<P0>(&self, param0: P0, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenTypeKey)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    #[doc = "Required features: `\"Win32_System_Registry\"`"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenConfigKey(&self, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenConfigKey)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputJoyConfig, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputJoyConfig {
    type Vtable = IDirectInputJoyConfig_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputJoyConfig {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1de12ab1_c9f5_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputJoyConfig_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub SendNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumTypes: usize,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::HRESULT,
    pub SetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::HRESULT,
    pub DeleteType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT,
    pub SetConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT,
    pub DeleteConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT,
    pub GetUserValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT,
    pub SetUserValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddNewHardware: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddNewHardware: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenTypeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenTypeKey: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenConfigKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenConfigKey: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputJoyConfig8(::windows_core::IUnknown);
impl IDirectInputJoyConfig8 {
    pub unsafe fn Acquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Acquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unacquire)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SetCooperativeLevel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn SendNotify(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SendNotify)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumTypes)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetTypeInfo<P0>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetTypeInfo)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    pub unsafe fn SetTypeInfo<P0, P1>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32, param3: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTypeInfo)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2, param3.into_param().abi()).ok()
    }
    pub unsafe fn DeleteType<P0>(&self, param0: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteType)(::windows_core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConfig)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetConfig)(::windows_core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn DeleteConfig(&self, param0: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteConfig)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUserValues)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserValues)(::windows_core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddNewHardware<P0>(&self, param0: P0, param1: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).AddNewHardware)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_System_Registry\"`"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenTypeKey<P0>(&self, param0: P0, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenTypeKey)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    #[doc = "Required features: `\"Win32_System_Registry\"`"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenAppStatusKey(&self, param0: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenAppStatusKey)(::windows_core::Interface::as_raw(self), param0).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputJoyConfig8, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputJoyConfig8 {
    type Vtable = IDirectInputJoyConfig8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputJoyConfig8 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb0d7dfa_1990_4f27_b4d6_edf2eec4a44c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputJoyConfig8_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub SendNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumTypes: usize,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> ::windows_core::HRESULT,
    pub SetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub DeleteType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT,
    pub SetConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> ::windows_core::HRESULT,
    pub DeleteConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows_core::HRESULT,
    pub GetUserValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT,
    pub SetUserValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddNewHardware: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddNewHardware: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenTypeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: ::windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenTypeKey: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenAppStatusKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenAppStatusKey: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectInputW(::windows_core::IUnknown);
impl IDirectInputW {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows_core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).CreateDevice)(::windows_core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumDevices)(::windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDeviceStatus)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).RunControlPanel)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectInputW, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectInputW {
    type Vtable = IDirectInputW_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectInputW {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89521361_aa8a_11cf_bfc7_444553540000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputW_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevices: usize,
    pub GetDeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
pub const BALLPOINT_I8042_HARDWARE: u32 = 8u32;
pub const BALLPOINT_SERIAL_HARDWARE: u32 = 16u32;
pub const BUTTON_BIT_ALLBUTTONSMASK: u32 = 16383u32;
pub const BUTTON_BIT_BACK: u32 = 32u32;
pub const BUTTON_BIT_CAMERAFOCUS: u32 = 128u32;
pub const BUTTON_BIT_CAMERALENS: u32 = 4096u32;
pub const BUTTON_BIT_CAMERASHUTTER: u32 = 256u32;
pub const BUTTON_BIT_HEADSET: u32 = 1024u32;
pub const BUTTON_BIT_HWKBDEPLOY: u32 = 2048u32;
pub const BUTTON_BIT_OEMCUSTOM: u32 = 8192u32;
pub const BUTTON_BIT_OEMCUSTOM2: u32 = 16384u32;
pub const BUTTON_BIT_OEMCUSTOM3: u32 = 32768u32;
pub const BUTTON_BIT_POWER: u32 = 1u32;
pub const BUTTON_BIT_RINGERTOGGLE: u32 = 512u32;
pub const BUTTON_BIT_ROTATION_LOCK: u32 = 16u32;
pub const BUTTON_BIT_SEARCH: u32 = 64u32;
pub const BUTTON_BIT_VOLUMEDOWN: u32 = 8u32;
pub const BUTTON_BIT_VOLUMEUP: u32 = 4u32;
pub const BUTTON_BIT_WINDOWS: u32 = 2u32;
pub const CLSID_DirectInput: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25e609e0_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInput8: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25e609e4_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25e609e1_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice8: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25e609e5_b259_11cf_bfc7_444553540000);
pub const DD_KEYBOARD_DEVICE_NAME: ::windows_core::PCSTR = ::windows_core::s!("\\Device\\KeyboardClass");
pub const DD_KEYBOARD_DEVICE_NAME_U: ::windows_core::PCWSTR = ::windows_core::w!("\\Device\\KeyboardClass");
pub const DD_MOUSE_DEVICE_NAME: ::windows_core::PCSTR = ::windows_core::s!("\\Device\\PointerClass");
pub const DD_MOUSE_DEVICE_NAME_U: ::windows_core::PCWSTR = ::windows_core::w!("\\Device\\PointerClass");
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_BackgroundAccess: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 8 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_IsReadOnly: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 4 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_ProductId: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 6 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_UsageId: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 3 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_UsagePage: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 2 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_VendorId: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 5 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_VersionNumber: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 7 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_HID_WakeScreenOnInputCapable: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 9 };
pub const DI8DEVCLASS_ALL: u32 = 0u32;
pub const DI8DEVCLASS_DEVICE: u32 = 1u32;
pub const DI8DEVCLASS_GAMECTRL: u32 = 4u32;
pub const DI8DEVCLASS_KEYBOARD: u32 = 3u32;
pub const DI8DEVCLASS_POINTER: u32 = 2u32;
pub const DI8DEVTYPE1STPERSON_LIMITED: u32 = 1u32;
pub const DI8DEVTYPE1STPERSON_SHOOTER: u32 = 4u32;
pub const DI8DEVTYPE1STPERSON_SIXDOF: u32 = 3u32;
pub const DI8DEVTYPE1STPERSON_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION: u32 = 3u32;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION_HARDWIRED: u32 = 4u32;
pub const DI8DEVTYPEDEVICECTRL_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPEDRIVING_COMBINEDPEDALS: u32 = 2u32;
pub const DI8DEVTYPEDRIVING_DUALPEDALS: u32 = 3u32;
pub const DI8DEVTYPEDRIVING_HANDHELD: u32 = 5u32;
pub const DI8DEVTYPEDRIVING_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEDRIVING_THREEPEDALS: u32 = 4u32;
pub const DI8DEVTYPEFLIGHT_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEFLIGHT_RC: u32 = 4u32;
pub const DI8DEVTYPEFLIGHT_STICK: u32 = 2u32;
pub const DI8DEVTYPEFLIGHT_YOKE: u32 = 3u32;
pub const DI8DEVTYPEGAMEPAD_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEGAMEPAD_STANDARD: u32 = 2u32;
pub const DI8DEVTYPEGAMEPAD_TILT: u32 = 3u32;
pub const DI8DEVTYPEJOYSTICK_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEJOYSTICK_STANDARD: u32 = 2u32;
pub const DI8DEVTYPEKEYBOARD_J3100: u32 = 12u32;
pub const DI8DEVTYPEKEYBOARD_JAPAN106: u32 = 10u32;
pub const DI8DEVTYPEKEYBOARD_JAPANAX: u32 = 11u32;
pub const DI8DEVTYPEKEYBOARD_NEC98: u32 = 7u32;
pub const DI8DEVTYPEKEYBOARD_NEC98106: u32 = 9u32;
pub const DI8DEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8u32;
pub const DI8DEVTYPEKEYBOARD_NOKIA1050: u32 = 5u32;
pub const DI8DEVTYPEKEYBOARD_NOKIA9140: u32 = 6u32;
pub const DI8DEVTYPEKEYBOARD_OLIVETTI: u32 = 2u32;
pub const DI8DEVTYPEKEYBOARD_PCAT: u32 = 3u32;
pub const DI8DEVTYPEKEYBOARD_PCENH: u32 = 4u32;
pub const DI8DEVTYPEKEYBOARD_PCXT: u32 = 1u32;
pub const DI8DEVTYPEKEYBOARD_UNKNOWN: u32 = 0u32;
pub const DI8DEVTYPEMOUSE_ABSOLUTE: u32 = 6u32;
pub const DI8DEVTYPEMOUSE_FINGERSTICK: u32 = 3u32;
pub const DI8DEVTYPEMOUSE_TOUCHPAD: u32 = 4u32;
pub const DI8DEVTYPEMOUSE_TRACKBALL: u32 = 5u32;
pub const DI8DEVTYPEMOUSE_TRADITIONAL: u32 = 2u32;
pub const DI8DEVTYPEMOUSE_UNKNOWN: u32 = 1u32;
pub const DI8DEVTYPEREMOTE_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPESCREENPTR_LIGHTGUN: u32 = 3u32;
pub const DI8DEVTYPESCREENPTR_LIGHTPEN: u32 = 4u32;
pub const DI8DEVTYPESCREENPTR_TOUCH: u32 = 5u32;
pub const DI8DEVTYPESCREENPTR_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPESUPPLEMENTAL_2NDHANDCONTROLLER: u32 = 3u32;
pub const DI8DEVTYPESUPPLEMENTAL_COMBINEDPEDALS: u32 = 10u32;
pub const DI8DEVTYPESUPPLEMENTAL_DUALPEDALS: u32 = 11u32;
pub const DI8DEVTYPESUPPLEMENTAL_HANDTRACKER: u32 = 5u32;
pub const DI8DEVTYPESUPPLEMENTAL_HEADTRACKER: u32 = 4u32;
pub const DI8DEVTYPESUPPLEMENTAL_RUDDERPEDALS: u32 = 13u32;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTER: u32 = 7u32;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTSTICKGATE: u32 = 6u32;
pub const DI8DEVTYPESUPPLEMENTAL_SPLITTHROTTLE: u32 = 9u32;
pub const DI8DEVTYPESUPPLEMENTAL_THREEPEDALS: u32 = 12u32;
pub const DI8DEVTYPESUPPLEMENTAL_THROTTLE: u32 = 8u32;
pub const DI8DEVTYPESUPPLEMENTAL_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPE_1STPERSON: u32 = 24u32;
pub const DI8DEVTYPE_DEVICE: u32 = 17u32;
pub const DI8DEVTYPE_DEVICECTRL: u32 = 25u32;
pub const DI8DEVTYPE_DRIVING: u32 = 22u32;
pub const DI8DEVTYPE_FLIGHT: u32 = 23u32;
pub const DI8DEVTYPE_GAMEPAD: u32 = 21u32;
pub const DI8DEVTYPE_JOYSTICK: u32 = 20u32;
pub const DI8DEVTYPE_KEYBOARD: u32 = 19u32;
pub const DI8DEVTYPE_LIMITEDGAMESUBTYPE: u32 = 1u32;
pub const DI8DEVTYPE_MOUSE: u32 = 18u32;
pub const DI8DEVTYPE_REMOTE: u32 = 27u32;
pub const DI8DEVTYPE_SCREENPOINTER: u32 = 26u32;
pub const DI8DEVTYPE_SUPPLEMENTAL: u32 = 28u32;
pub const DIAFTS_NEWDEVICEHIGH: u32 = 4294967295u32;
pub const DIAFTS_NEWDEVICELOW: u32 = 4294967295u32;
pub const DIAFTS_UNUSEDDEVICEHIGH: u32 = 0u32;
pub const DIAFTS_UNUSEDDEVICELOW: u32 = 0u32;
pub const DIAH_APPREQUESTED: u32 = 2u32;
pub const DIAH_DEFAULT: u32 = 32u32;
pub const DIAH_ERROR: u32 = 2147483648u32;
pub const DIAH_HWAPP: u32 = 4u32;
pub const DIAH_HWDEFAULT: u32 = 8u32;
pub const DIAH_UNMAPPED: u32 = 0u32;
pub const DIAH_USERCONFIG: u32 = 1u32;
pub const DIAPPIDFLAG_NOSIZE: u32 = 2u32;
pub const DIAPPIDFLAG_NOTIME: u32 = 1u32;
pub const DIAXIS_2DCONTROL_INOUT: u32 = 587301379u32;
pub const DIAXIS_2DCONTROL_LATERAL: u32 = 587235841u32;
pub const DIAXIS_2DCONTROL_MOVE: u32 = 587268610u32;
pub const DIAXIS_2DCONTROL_ROTATEZ: u32 = 587350532u32;
pub const DIAXIS_3DCONTROL_INOUT: u32 = 604078595u32;
pub const DIAXIS_3DCONTROL_LATERAL: u32 = 604013057u32;
pub const DIAXIS_3DCONTROL_MOVE: u32 = 604045826u32;
pub const DIAXIS_3DCONTROL_ROTATEX: u32 = 604193284u32;
pub const DIAXIS_3DCONTROL_ROTATEY: u32 = 604160517u32;
pub const DIAXIS_3DCONTROL_ROTATEZ: u32 = 604127750u32;
pub const DIAXIS_ANY_1: u32 = 4278206977u32;
pub const DIAXIS_ANY_2: u32 = 4278206978u32;
pub const DIAXIS_ANY_3: u32 = 4278206979u32;
pub const DIAXIS_ANY_4: u32 = 4278206980u32;
pub const DIAXIS_ANY_A_1: u32 = 4278436353u32;
pub const DIAXIS_ANY_A_2: u32 = 4278436354u32;
pub const DIAXIS_ANY_B_1: u32 = 4278469121u32;
pub const DIAXIS_ANY_B_2: u32 = 4278469122u32;
pub const DIAXIS_ANY_C_1: u32 = 4278501889u32;
pub const DIAXIS_ANY_C_2: u32 = 4278501890u32;
pub const DIAXIS_ANY_R_1: u32 = 4278338049u32;
pub const DIAXIS_ANY_R_2: u32 = 4278338050u32;
pub const DIAXIS_ANY_S_1: u32 = 4278534657u32;
pub const DIAXIS_ANY_S_2: u32 = 4278534658u32;
pub const DIAXIS_ANY_U_1: u32 = 4278370817u32;
pub const DIAXIS_ANY_U_2: u32 = 4278370818u32;
pub const DIAXIS_ANY_V_1: u32 = 4278403585u32;
pub const DIAXIS_ANY_V_2: u32 = 4278403586u32;
pub const DIAXIS_ANY_X_1: u32 = 4278239745u32;
pub const DIAXIS_ANY_X_2: u32 = 4278239746u32;
pub const DIAXIS_ANY_Y_1: u32 = 4278272513u32;
pub const DIAXIS_ANY_Y_2: u32 = 4278272514u32;
pub const DIAXIS_ANY_Z_1: u32 = 4278305281u32;
pub const DIAXIS_ANY_Z_2: u32 = 4278305282u32;
pub const DIAXIS_ARCADEP_LATERAL: u32 = 570458625u32;
pub const DIAXIS_ARCADEP_MOVE: u32 = 570491394u32;
pub const DIAXIS_ARCADES_LATERAL: u32 = 553681409u32;
pub const DIAXIS_ARCADES_MOVE: u32 = 553714178u32;
pub const DIAXIS_BASEBALLB_LATERAL: u32 = 251691521u32;
pub const DIAXIS_BASEBALLB_MOVE: u32 = 251724290u32;
pub const DIAXIS_BASEBALLF_LATERAL: u32 = 285245953u32;
pub const DIAXIS_BASEBALLF_MOVE: u32 = 285278722u32;
pub const DIAXIS_BASEBALLP_LATERAL: u32 = 268468737u32;
pub const DIAXIS_BASEBALLP_MOVE: u32 = 268501506u32;
pub const DIAXIS_BBALLD_LATERAL: u32 = 318800385u32;
pub const DIAXIS_BBALLD_MOVE: u32 = 318833154u32;
pub const DIAXIS_BBALLO_LATERAL: u32 = 302023169u32;
pub const DIAXIS_BBALLO_MOVE: u32 = 302055938u32;
pub const DIAXIS_BIKINGM_BRAKE: u32 = 470041091u32;
pub const DIAXIS_BIKINGM_PEDAL: u32 = 469828098u32;
pub const DIAXIS_BIKINGM_TURN: u32 = 469795329u32;
pub const DIAXIS_BROWSER_LATERAL: u32 = 671121921u32;
pub const DIAXIS_BROWSER_MOVE: u32 = 671154690u32;
pub const DIAXIS_BROWSER_VIEW: u32 = 671187459u32;
pub const DIAXIS_CADF_INOUT: u32 = 620855811u32;
pub const DIAXIS_CADF_LATERAL: u32 = 620790273u32;
pub const DIAXIS_CADF_MOVE: u32 = 620823042u32;
pub const DIAXIS_CADF_ROTATEX: u32 = 620970500u32;
pub const DIAXIS_CADF_ROTATEY: u32 = 620937733u32;
pub const DIAXIS_CADF_ROTATEZ: u32 = 620904966u32;
pub const DIAXIS_CADM_INOUT: u32 = 637633027u32;
pub const DIAXIS_CADM_LATERAL: u32 = 637567489u32;
pub const DIAXIS_CADM_MOVE: u32 = 637600258u32;
pub const DIAXIS_CADM_ROTATEX: u32 = 637747716u32;
pub const DIAXIS_CADM_ROTATEY: u32 = 637714949u32;
pub const DIAXIS_CADM_ROTATEZ: u32 = 637682182u32;
pub const DIAXIS_DRIVINGC_ACCELERATE: u32 = 33788418u32;
pub const DIAXIS_DRIVINGC_ACCEL_AND_BRAKE: u32 = 33638916u32;
pub const DIAXIS_DRIVINGC_BRAKE: u32 = 33821187u32;
pub const DIAXIS_DRIVINGC_STEER: u32 = 33589761u32;
pub const DIAXIS_DRIVINGR_ACCELERATE: u32 = 17011202u32;
pub const DIAXIS_DRIVINGR_ACCEL_AND_BRAKE: u32 = 16861700u32;
pub const DIAXIS_DRIVINGR_BRAKE: u32 = 17043971u32;
pub const DIAXIS_DRIVINGR_STEER: u32 = 16812545u32;
pub const DIAXIS_DRIVINGT_ACCELERATE: u32 = 50565635u32;
pub const DIAXIS_DRIVINGT_ACCEL_AND_BRAKE: u32 = 50416134u32;
pub const DIAXIS_DRIVINGT_BARREL: u32 = 50397698u32;
pub const DIAXIS_DRIVINGT_BRAKE: u32 = 50614789u32;
pub const DIAXIS_DRIVINGT_ROTATE: u32 = 50463236u32;
pub const DIAXIS_DRIVINGT_STEER: u32 = 50366977u32;
pub const DIAXIS_FIGHTINGH_LATERAL: u32 = 134251009u32;
pub const DIAXIS_FIGHTINGH_MOVE: u32 = 134283778u32;
pub const DIAXIS_FIGHTINGH_ROTATE: u32 = 134365699u32;
pub const DIAXIS_FISHING_LATERAL: u32 = 234914305u32;
pub const DIAXIS_FISHING_MOVE: u32 = 234947074u32;
pub const DIAXIS_FISHING_ROTATE: u32 = 235028995u32;
pub const DIAXIS_FLYINGC_BANK: u32 = 67144193u32;
pub const DIAXIS_FLYINGC_BRAKE: u32 = 67398148u32;
pub const DIAXIS_FLYINGC_FLAPS: u32 = 67459590u32;
pub const DIAXIS_FLYINGC_PITCH: u32 = 67176962u32;
pub const DIAXIS_FLYINGC_RUDDER: u32 = 67260933u32;
pub const DIAXIS_FLYINGC_THROTTLE: u32 = 67342851u32;
pub const DIAXIS_FLYINGH_BANK: u32 = 100698625u32;
pub const DIAXIS_FLYINGH_COLLECTIVE: u32 = 100764163u32;
pub const DIAXIS_FLYINGH_PITCH: u32 = 100731394u32;
pub const DIAXIS_FLYINGH_THROTTLE: u32 = 100915717u32;
pub const DIAXIS_FLYINGH_TORQUE: u32 = 100817412u32;
pub const DIAXIS_FLYINGM_BANK: u32 = 83921409u32;
pub const DIAXIS_FLYINGM_BRAKE: u32 = 84173317u32;
pub const DIAXIS_FLYINGM_FLAPS: u32 = 84234758u32;
pub const DIAXIS_FLYINGM_PITCH: u32 = 83954178u32;
pub const DIAXIS_FLYINGM_RUDDER: u32 = 84036100u32;
pub const DIAXIS_FLYINGM_THROTTLE: u32 = 84120067u32;
pub const DIAXIS_FOOTBALLD_LATERAL: u32 = 385909249u32;
pub const DIAXIS_FOOTBALLD_MOVE: u32 = 385942018u32;
pub const DIAXIS_FOOTBALLO_LATERAL: u32 = 369132033u32;
pub const DIAXIS_FOOTBALLO_MOVE: u32 = 369164802u32;
pub const DIAXIS_FOOTBALLQ_LATERAL: u32 = 352354817u32;
pub const DIAXIS_FOOTBALLQ_MOVE: u32 = 352387586u32;
pub const DIAXIS_FPS_LOOKUPDOWN: u32 = 151093763u32;
pub const DIAXIS_FPS_MOVE: u32 = 151060994u32;
pub const DIAXIS_FPS_ROTATE: u32 = 151028225u32;
pub const DIAXIS_FPS_SIDESTEP: u32 = 151142916u32;
pub const DIAXIS_GOLF_LATERAL: u32 = 402686465u32;
pub const DIAXIS_GOLF_MOVE: u32 = 402719234u32;
pub const DIAXIS_HOCKEYD_LATERAL: u32 = 436240897u32;
pub const DIAXIS_HOCKEYD_MOVE: u32 = 436273666u32;
pub const DIAXIS_HOCKEYG_LATERAL: u32 = 453018113u32;
pub const DIAXIS_HOCKEYG_MOVE: u32 = 453050882u32;
pub const DIAXIS_HOCKEYO_LATERAL: u32 = 419463681u32;
pub const DIAXIS_HOCKEYO_MOVE: u32 = 419496450u32;
pub const DIAXIS_HUNTING_LATERAL: u32 = 218137089u32;
pub const DIAXIS_HUNTING_MOVE: u32 = 218169858u32;
pub const DIAXIS_HUNTING_ROTATE: u32 = 218251779u32;
pub const DIAXIS_MECHA_ROTATE: u32 = 687997443u32;
pub const DIAXIS_MECHA_STEER: u32 = 687899137u32;
pub const DIAXIS_MECHA_THROTTLE: u32 = 688095748u32;
pub const DIAXIS_MECHA_TORSO: u32 = 687931906u32;
pub const DIAXIS_RACQUET_LATERAL: u32 = 536904193u32;
pub const DIAXIS_RACQUET_MOVE: u32 = 536936962u32;
pub const DIAXIS_REMOTE_SLIDER: u32 = 654639617u32;
pub const DIAXIS_REMOTE_SLIDER2: u32 = 654656002u32;
pub const DIAXIS_SKIING_SPEED: u32 = 486605314u32;
pub const DIAXIS_SKIING_TURN: u32 = 486572545u32;
pub const DIAXIS_SOCCERD_LATERAL: u32 = 520126977u32;
pub const DIAXIS_SOCCERD_MOVE: u32 = 520159746u32;
pub const DIAXIS_SOCCERO_BEND: u32 = 503415299u32;
pub const DIAXIS_SOCCERO_LATERAL: u32 = 503349761u32;
pub const DIAXIS_SOCCERO_MOVE: u32 = 503382530u32;
pub const DIAXIS_SPACESIM_CLIMB: u32 = 117555716u32;
pub const DIAXIS_SPACESIM_LATERAL: u32 = 117473793u32;
pub const DIAXIS_SPACESIM_MOVE: u32 = 117506562u32;
pub const DIAXIS_SPACESIM_ROTATE: u32 = 117588485u32;
pub const DIAXIS_SPACESIM_THROTTLE: u32 = 117670403u32;
pub const DIAXIS_STRATEGYR_LATERAL: u32 = 184582657u32;
pub const DIAXIS_STRATEGYR_MOVE: u32 = 184615426u32;
pub const DIAXIS_STRATEGYR_ROTATE: u32 = 184697347u32;
pub const DIAXIS_STRATEGYT_LATERAL: u32 = 201359873u32;
pub const DIAXIS_STRATEGYT_MOVE: u32 = 201392642u32;
pub const DIAXIS_TPS_MOVE: u32 = 167838210u32;
pub const DIAXIS_TPS_STEP: u32 = 167821827u32;
pub const DIAXIS_TPS_TURN: u32 = 167903745u32;
pub const DIA_APPFIXED: u32 = 16u32;
pub const DIA_APPMAPPED: u32 = 2u32;
pub const DIA_APPNOMAP: u32 = 4u32;
pub const DIA_FORCEFEEDBACK: u32 = 1u32;
pub const DIA_NORANGE: u32 = 8u32;
pub const DIBUTTON_2DCONTROL_DEVICE: u32 = 587220222u32;
pub const DIBUTTON_2DCONTROL_DISPLAY: u32 = 587219973u32;
pub const DIBUTTON_2DCONTROL_MENU: u32 = 587203837u32;
pub const DIBUTTON_2DCONTROL_PAUSE: u32 = 587220220u32;
pub const DIBUTTON_2DCONTROL_SELECT: u32 = 587203585u32;
pub const DIBUTTON_2DCONTROL_SPECIAL: u32 = 587203587u32;
pub const DIBUTTON_2DCONTROL_SPECIAL1: u32 = 587203586u32;
pub const DIBUTTON_2DCONTROL_SPECIAL2: u32 = 587203588u32;
pub const DIBUTTON_3DCONTROL_DEVICE: u32 = 603997438u32;
pub const DIBUTTON_3DCONTROL_DISPLAY: u32 = 603997189u32;
pub const DIBUTTON_3DCONTROL_MENU: u32 = 603981053u32;
pub const DIBUTTON_3DCONTROL_PAUSE: u32 = 603997436u32;
pub const DIBUTTON_3DCONTROL_SELECT: u32 = 603980801u32;
pub const DIBUTTON_3DCONTROL_SPECIAL: u32 = 603980803u32;
pub const DIBUTTON_3DCONTROL_SPECIAL1: u32 = 603980802u32;
pub const DIBUTTON_3DCONTROL_SPECIAL2: u32 = 603980804u32;
pub const DIBUTTON_ARCADEP_BACK_LINK: u32 = 570508520u32;
pub const DIBUTTON_ARCADEP_CROUCH: u32 = 570426371u32;
pub const DIBUTTON_ARCADEP_DEVICE: u32 = 570443006u32;
pub const DIBUTTON_ARCADEP_FIRE: u32 = 570426370u32;
pub const DIBUTTON_ARCADEP_FIRESECONDARY: u32 = 570442758u32;
pub const DIBUTTON_ARCADEP_FORWARD_LINK: u32 = 570508512u32;
pub const DIBUTTON_ARCADEP_JUMP: u32 = 570426369u32;
pub const DIBUTTON_ARCADEP_LEFT_LINK: u32 = 570475748u32;
pub const DIBUTTON_ARCADEP_MENU: u32 = 570426621u32;
pub const DIBUTTON_ARCADEP_PAUSE: u32 = 570443004u32;
pub const DIBUTTON_ARCADEP_RIGHT_LINK: u32 = 570475756u32;
pub const DIBUTTON_ARCADEP_SELECT: u32 = 570426373u32;
pub const DIBUTTON_ARCADEP_SPECIAL: u32 = 570426372u32;
pub const DIBUTTON_ARCADEP_VIEW_DOWN_LINK: u32 = 570934504u32;
pub const DIBUTTON_ARCADEP_VIEW_LEFT_LINK: u32 = 570934500u32;
pub const DIBUTTON_ARCADEP_VIEW_RIGHT_LINK: u32 = 570934508u32;
pub const DIBUTTON_ARCADEP_VIEW_UP_LINK: u32 = 570934496u32;
pub const DIBUTTON_ARCADES_ATTACK: u32 = 553649155u32;
pub const DIBUTTON_ARCADES_BACK_LINK: u32 = 553731304u32;
pub const DIBUTTON_ARCADES_CARRY: u32 = 553649154u32;
pub const DIBUTTON_ARCADES_DEVICE: u32 = 553665790u32;
pub const DIBUTTON_ARCADES_FORWARD_LINK: u32 = 553731296u32;
pub const DIBUTTON_ARCADES_LEFT_LINK: u32 = 553698532u32;
pub const DIBUTTON_ARCADES_MENU: u32 = 553649405u32;
pub const DIBUTTON_ARCADES_PAUSE: u32 = 553665788u32;
pub const DIBUTTON_ARCADES_RIGHT_LINK: u32 = 553698540u32;
pub const DIBUTTON_ARCADES_SELECT: u32 = 553649157u32;
pub const DIBUTTON_ARCADES_SPECIAL: u32 = 553649156u32;
pub const DIBUTTON_ARCADES_THROW: u32 = 553649153u32;
pub const DIBUTTON_ARCADES_VIEW_DOWN_LINK: u32 = 554157288u32;
pub const DIBUTTON_ARCADES_VIEW_LEFT_LINK: u32 = 554157284u32;
pub const DIBUTTON_ARCADES_VIEW_RIGHT_LINK: u32 = 554157292u32;
pub const DIBUTTON_ARCADES_VIEW_UP_LINK: u32 = 554157280u32;
pub const DIBUTTON_BASEBALLB_BACK_LINK: u32 = 251741416u32;
pub const DIBUTTON_BASEBALLB_BOX: u32 = 251675658u32;
pub const DIBUTTON_BASEBALLB_BUNT: u32 = 251659268u32;
pub const DIBUTTON_BASEBALLB_BURST: u32 = 251659270u32;
pub const DIBUTTON_BASEBALLB_CONTACT: u32 = 251659272u32;
pub const DIBUTTON_BASEBALLB_DEVICE: u32 = 251675902u32;
pub const DIBUTTON_BASEBALLB_FORWARD_LINK: u32 = 251741408u32;
pub const DIBUTTON_BASEBALLB_LEFT_LINK: u32 = 251708644u32;
pub const DIBUTTON_BASEBALLB_MENU: u32 = 251659517u32;
pub const DIBUTTON_BASEBALLB_NORMAL: u32 = 251659266u32;
pub const DIBUTTON_BASEBALLB_NOSTEAL: u32 = 251675657u32;
pub const DIBUTTON_BASEBALLB_PAUSE: u32 = 251675900u32;
pub const DIBUTTON_BASEBALLB_POWER: u32 = 251659267u32;
pub const DIBUTTON_BASEBALLB_RIGHT_LINK: u32 = 251708652u32;
pub const DIBUTTON_BASEBALLB_SELECT: u32 = 251659265u32;
pub const DIBUTTON_BASEBALLB_SLIDE: u32 = 251659271u32;
pub const DIBUTTON_BASEBALLB_STEAL: u32 = 251659269u32;
pub const DIBUTTON_BASEBALLF_AIM_LEFT_LINK: u32 = 285263076u32;
pub const DIBUTTON_BASEBALLF_AIM_RIGHT_LINK: u32 = 285263084u32;
pub const DIBUTTON_BASEBALLF_BACK_LINK: u32 = 285295848u32;
pub const DIBUTTON_BASEBALLF_BURST: u32 = 285213700u32;
pub const DIBUTTON_BASEBALLF_DEVICE: u32 = 285230334u32;
pub const DIBUTTON_BASEBALLF_DIVE: u32 = 285213702u32;
pub const DIBUTTON_BASEBALLF_FORWARD_LINK: u32 = 285295840u32;
pub const DIBUTTON_BASEBALLF_JUMP: u32 = 285213701u32;
pub const DIBUTTON_BASEBALLF_MENU: u32 = 285213949u32;
pub const DIBUTTON_BASEBALLF_NEAREST: u32 = 285213697u32;
pub const DIBUTTON_BASEBALLF_PAUSE: u32 = 285230332u32;
pub const DIBUTTON_BASEBALLF_SHIFTIN: u32 = 285230087u32;
pub const DIBUTTON_BASEBALLF_SHIFTOUT: u32 = 285230088u32;
pub const DIBUTTON_BASEBALLF_THROW1: u32 = 285213698u32;
pub const DIBUTTON_BASEBALLF_THROW2: u32 = 285213699u32;
pub const DIBUTTON_BASEBALLP_BACK_LINK: u32 = 268518632u32;
pub const DIBUTTON_BASEBALLP_BASE: u32 = 268436483u32;
pub const DIBUTTON_BASEBALLP_DEVICE: u32 = 268453118u32;
pub const DIBUTTON_BASEBALLP_FAKE: u32 = 268436485u32;
pub const DIBUTTON_BASEBALLP_FORWARD_LINK: u32 = 268518624u32;
pub const DIBUTTON_BASEBALLP_LEFT_LINK: u32 = 268485860u32;
pub const DIBUTTON_BASEBALLP_LOOK: u32 = 268452871u32;
pub const DIBUTTON_BASEBALLP_MENU: u32 = 268436733u32;
pub const DIBUTTON_BASEBALLP_PAUSE: u32 = 268453116u32;
pub const DIBUTTON_BASEBALLP_PITCH: u32 = 268436482u32;
pub const DIBUTTON_BASEBALLP_RIGHT_LINK: u32 = 268485868u32;
pub const DIBUTTON_BASEBALLP_SELECT: u32 = 268436481u32;
pub const DIBUTTON_BASEBALLP_THROW: u32 = 268436484u32;
pub const DIBUTTON_BASEBALLP_WALK: u32 = 268452870u32;
pub const DIBUTTON_BBALLD_BACK_LINK: u32 = 318850280u32;
pub const DIBUTTON_BBALLD_BURST: u32 = 318768134u32;
pub const DIBUTTON_BBALLD_DEVICE: u32 = 318784766u32;
pub const DIBUTTON_BBALLD_FAKE: u32 = 318768131u32;
pub const DIBUTTON_BBALLD_FORWARD_LINK: u32 = 318850272u32;
pub const DIBUTTON_BBALLD_JUMP: u32 = 318768129u32;
pub const DIBUTTON_BBALLD_LEFT_LINK: u32 = 318817508u32;
pub const DIBUTTON_BBALLD_MENU: u32 = 318768381u32;
pub const DIBUTTON_BBALLD_PAUSE: u32 = 318784764u32;
pub const DIBUTTON_BBALLD_PLAY: u32 = 318768135u32;
pub const DIBUTTON_BBALLD_PLAYER: u32 = 318768133u32;
pub const DIBUTTON_BBALLD_RIGHT_LINK: u32 = 318817516u32;
pub const DIBUTTON_BBALLD_SPECIAL: u32 = 318768132u32;
pub const DIBUTTON_BBALLD_STEAL: u32 = 318768130u32;
pub const DIBUTTON_BBALLD_SUBSTITUTE: u32 = 318784521u32;
pub const DIBUTTON_BBALLD_TIMEOUT: u32 = 318784520u32;
pub const DIBUTTON_BBALLO_BACK_LINK: u32 = 302073064u32;
pub const DIBUTTON_BBALLO_BURST: u32 = 301990919u32;
pub const DIBUTTON_BBALLO_CALL: u32 = 301990920u32;
pub const DIBUTTON_BBALLO_DEVICE: u32 = 302007550u32;
pub const DIBUTTON_BBALLO_DUNK: u32 = 301990914u32;
pub const DIBUTTON_BBALLO_FAKE: u32 = 301990916u32;
pub const DIBUTTON_BBALLO_FORWARD_LINK: u32 = 302073056u32;
pub const DIBUTTON_BBALLO_JAB: u32 = 302007307u32;
pub const DIBUTTON_BBALLO_LEFT_LINK: u32 = 302040292u32;
pub const DIBUTTON_BBALLO_MENU: u32 = 301991165u32;
pub const DIBUTTON_BBALLO_PASS: u32 = 301990915u32;
pub const DIBUTTON_BBALLO_PAUSE: u32 = 302007548u32;
pub const DIBUTTON_BBALLO_PLAY: u32 = 302007306u32;
pub const DIBUTTON_BBALLO_PLAYER: u32 = 301990918u32;
pub const DIBUTTON_BBALLO_POST: u32 = 302007308u32;
pub const DIBUTTON_BBALLO_RIGHT_LINK: u32 = 302040300u32;
pub const DIBUTTON_BBALLO_SCREEN: u32 = 302007305u32;
pub const DIBUTTON_BBALLO_SHOOT: u32 = 301990913u32;
pub const DIBUTTON_BBALLO_SPECIAL: u32 = 301990917u32;
pub const DIBUTTON_BBALLO_SUBSTITUTE: u32 = 302007310u32;
pub const DIBUTTON_BBALLO_TIMEOUT: u32 = 302007309u32;
pub const DIBUTTON_BIKINGM_BRAKE_BUTTON_LINK: u32 = 470041832u32;
pub const DIBUTTON_BIKINGM_CAMERA: u32 = 469763074u32;
pub const DIBUTTON_BIKINGM_DEVICE: u32 = 469779710u32;
pub const DIBUTTON_BIKINGM_FASTER_LINK: u32 = 469845216u32;
pub const DIBUTTON_BIKINGM_JUMP: u32 = 469763073u32;
pub const DIBUTTON_BIKINGM_LEFT_LINK: u32 = 469812452u32;
pub const DIBUTTON_BIKINGM_MENU: u32 = 469763325u32;
pub const DIBUTTON_BIKINGM_PAUSE: u32 = 469779708u32;
pub const DIBUTTON_BIKINGM_RIGHT_LINK: u32 = 469812460u32;
pub const DIBUTTON_BIKINGM_SELECT: u32 = 469763076u32;
pub const DIBUTTON_BIKINGM_SLOWER_LINK: u32 = 469845224u32;
pub const DIBUTTON_BIKINGM_SPECIAL1: u32 = 469763075u32;
pub const DIBUTTON_BIKINGM_SPECIAL2: u32 = 469763077u32;
pub const DIBUTTON_BIKINGM_ZOOM: u32 = 469779462u32;
pub const DIBUTTON_BROWSER_DEVICE: u32 = 671106302u32;
pub const DIBUTTON_BROWSER_FAVORITES: u32 = 671106054u32;
pub const DIBUTTON_BROWSER_HISTORY: u32 = 671106057u32;
pub const DIBUTTON_BROWSER_HOME: u32 = 671106053u32;
pub const DIBUTTON_BROWSER_MENU: u32 = 671089917u32;
pub const DIBUTTON_BROWSER_NEXT: u32 = 671106055u32;
pub const DIBUTTON_BROWSER_PAUSE: u32 = 671106300u32;
pub const DIBUTTON_BROWSER_PREVIOUS: u32 = 671106056u32;
pub const DIBUTTON_BROWSER_PRINT: u32 = 671106058u32;
pub const DIBUTTON_BROWSER_REFRESH: u32 = 671089666u32;
pub const DIBUTTON_BROWSER_SEARCH: u32 = 671106051u32;
pub const DIBUTTON_BROWSER_SELECT: u32 = 671089665u32;
pub const DIBUTTON_BROWSER_STOP: u32 = 671106052u32;
pub const DIBUTTON_CADF_DEVICE: u32 = 620774654u32;
pub const DIBUTTON_CADF_DISPLAY: u32 = 620774405u32;
pub const DIBUTTON_CADF_MENU: u32 = 620758269u32;
pub const DIBUTTON_CADF_PAUSE: u32 = 620774652u32;
pub const DIBUTTON_CADF_SELECT: u32 = 620758017u32;
pub const DIBUTTON_CADF_SPECIAL: u32 = 620758019u32;
pub const DIBUTTON_CADF_SPECIAL1: u32 = 620758018u32;
pub const DIBUTTON_CADF_SPECIAL2: u32 = 620758020u32;
pub const DIBUTTON_CADM_DEVICE: u32 = 637551870u32;
pub const DIBUTTON_CADM_DISPLAY: u32 = 637551621u32;
pub const DIBUTTON_CADM_MENU: u32 = 637535485u32;
pub const DIBUTTON_CADM_PAUSE: u32 = 637551868u32;
pub const DIBUTTON_CADM_SELECT: u32 = 637535233u32;
pub const DIBUTTON_CADM_SPECIAL: u32 = 637535235u32;
pub const DIBUTTON_CADM_SPECIAL1: u32 = 637535234u32;
pub const DIBUTTON_CADM_SPECIAL2: u32 = 637535236u32;
pub const DIBUTTON_DRIVINGC_ACCELERATE_LINK: u32 = 33805536u32;
pub const DIBUTTON_DRIVINGC_AIDS: u32 = 33571847u32;
pub const DIBUTTON_DRIVINGC_BRAKE: u32 = 33573896u32;
pub const DIBUTTON_DRIVINGC_DASHBOARD: u32 = 33571846u32;
pub const DIBUTTON_DRIVINGC_DEVICE: u32 = 33572094u32;
pub const DIBUTTON_DRIVINGC_FIRE: u32 = 33557505u32;
pub const DIBUTTON_DRIVINGC_FIRESECONDARY: u32 = 33573897u32;
pub const DIBUTTON_DRIVINGC_GLANCE_LEFT_LINK: u32 = 34063588u32;
pub const DIBUTTON_DRIVINGC_GLANCE_RIGHT_LINK: u32 = 34063596u32;
pub const DIBUTTON_DRIVINGC_MENU: u32 = 33555709u32;
pub const DIBUTTON_DRIVINGC_PAUSE: u32 = 33572092u32;
pub const DIBUTTON_DRIVINGC_SHIFTDOWN: u32 = 33573893u32;
pub const DIBUTTON_DRIVINGC_SHIFTUP: u32 = 33573892u32;
pub const DIBUTTON_DRIVINGC_STEER_LEFT_LINK: u32 = 33606884u32;
pub const DIBUTTON_DRIVINGC_STEER_RIGHT_LINK: u32 = 33606892u32;
pub const DIBUTTON_DRIVINGC_TARGET: u32 = 33557507u32;
pub const DIBUTTON_DRIVINGC_WEAPONS: u32 = 33557506u32;
pub const DIBUTTON_DRIVINGR_ACCELERATE_LINK: u32 = 17028320u32;
pub const DIBUTTON_DRIVINGR_AIDS: u32 = 16794630u32;
pub const DIBUTTON_DRIVINGR_BOOST: u32 = 16794632u32;
pub const DIBUTTON_DRIVINGR_BRAKE: u32 = 16796676u32;
pub const DIBUTTON_DRIVINGR_DASHBOARD: u32 = 16794629u32;
pub const DIBUTTON_DRIVINGR_DEVICE: u32 = 16794878u32;
pub const DIBUTTON_DRIVINGR_GLANCE_LEFT_LINK: u32 = 17286372u32;
pub const DIBUTTON_DRIVINGR_GLANCE_RIGHT_LINK: u32 = 17286380u32;
pub const DIBUTTON_DRIVINGR_MAP: u32 = 16794631u32;
pub const DIBUTTON_DRIVINGR_MENU: u32 = 16778493u32;
pub const DIBUTTON_DRIVINGR_PAUSE: u32 = 16794876u32;
pub const DIBUTTON_DRIVINGR_PIT: u32 = 16794633u32;
pub const DIBUTTON_DRIVINGR_SHIFTDOWN: u32 = 16780290u32;
pub const DIBUTTON_DRIVINGR_SHIFTUP: u32 = 16780289u32;
pub const DIBUTTON_DRIVINGR_STEER_LEFT_LINK: u32 = 16829668u32;
pub const DIBUTTON_DRIVINGR_STEER_RIGHT_LINK: u32 = 16829676u32;
pub const DIBUTTON_DRIVINGR_VIEW: u32 = 16784387u32;
pub const DIBUTTON_DRIVINGT_ACCELERATE_LINK: u32 = 50582752u32;
pub const DIBUTTON_DRIVINGT_BARREL_DOWN_LINK: u32 = 50414824u32;
pub const DIBUTTON_DRIVINGT_BARREL_UP_LINK: u32 = 50414816u32;
pub const DIBUTTON_DRIVINGT_BRAKE: u32 = 50351110u32;
pub const DIBUTTON_DRIVINGT_DASHBOARD: u32 = 50355205u32;
pub const DIBUTTON_DRIVINGT_DEVICE: u32 = 50349310u32;
pub const DIBUTTON_DRIVINGT_FIRE: u32 = 50334721u32;
pub const DIBUTTON_DRIVINGT_FIRESECONDARY: u32 = 50351111u32;
pub const DIBUTTON_DRIVINGT_GLANCE_LEFT_LINK: u32 = 50840804u32;
pub const DIBUTTON_DRIVINGT_GLANCE_RIGHT_LINK: u32 = 50840812u32;
pub const DIBUTTON_DRIVINGT_MENU: u32 = 50332925u32;
pub const DIBUTTON_DRIVINGT_PAUSE: u32 = 50349308u32;
pub const DIBUTTON_DRIVINGT_ROTATE_LEFT_LINK: u32 = 50480356u32;
pub const DIBUTTON_DRIVINGT_ROTATE_RIGHT_LINK: u32 = 50480364u32;
pub const DIBUTTON_DRIVINGT_STEER_LEFT_LINK: u32 = 50384100u32;
pub const DIBUTTON_DRIVINGT_STEER_RIGHT_LINK: u32 = 50384108u32;
pub const DIBUTTON_DRIVINGT_TARGET: u32 = 50334723u32;
pub const DIBUTTON_DRIVINGT_VIEW: u32 = 50355204u32;
pub const DIBUTTON_DRIVINGT_WEAPONS: u32 = 50334722u32;
pub const DIBUTTON_FIGHTINGH_BACKWARD_LINK: u32 = 134300904u32;
pub const DIBUTTON_FIGHTINGH_BLOCK: u32 = 134218755u32;
pub const DIBUTTON_FIGHTINGH_CROUCH: u32 = 134218756u32;
pub const DIBUTTON_FIGHTINGH_DEVICE: u32 = 134235390u32;
pub const DIBUTTON_FIGHTINGH_DISPLAY: u32 = 134235145u32;
pub const DIBUTTON_FIGHTINGH_DODGE: u32 = 134235146u32;
pub const DIBUTTON_FIGHTINGH_FORWARD_LINK: u32 = 134300896u32;
pub const DIBUTTON_FIGHTINGH_JUMP: u32 = 134218757u32;
pub const DIBUTTON_FIGHTINGH_KICK: u32 = 134218754u32;
pub const DIBUTTON_FIGHTINGH_LEFT_LINK: u32 = 134268132u32;
pub const DIBUTTON_FIGHTINGH_MENU: u32 = 134219005u32;
pub const DIBUTTON_FIGHTINGH_PAUSE: u32 = 134235388u32;
pub const DIBUTTON_FIGHTINGH_PUNCH: u32 = 134218753u32;
pub const DIBUTTON_FIGHTINGH_RIGHT_LINK: u32 = 134268140u32;
pub const DIBUTTON_FIGHTINGH_SELECT: u32 = 134235144u32;
pub const DIBUTTON_FIGHTINGH_SPECIAL1: u32 = 134218758u32;
pub const DIBUTTON_FIGHTINGH_SPECIAL2: u32 = 134218759u32;
pub const DIBUTTON_FISHING_BACK_LINK: u32 = 234964200u32;
pub const DIBUTTON_FISHING_BAIT: u32 = 234882052u32;
pub const DIBUTTON_FISHING_BINOCULAR: u32 = 234882051u32;
pub const DIBUTTON_FISHING_CAST: u32 = 234882049u32;
pub const DIBUTTON_FISHING_CROUCH: u32 = 234898439u32;
pub const DIBUTTON_FISHING_DEVICE: u32 = 234898686u32;
pub const DIBUTTON_FISHING_DISPLAY: u32 = 234898438u32;
pub const DIBUTTON_FISHING_FORWARD_LINK: u32 = 234964192u32;
pub const DIBUTTON_FISHING_JUMP: u32 = 234898440u32;
pub const DIBUTTON_FISHING_LEFT_LINK: u32 = 234931428u32;
pub const DIBUTTON_FISHING_MAP: u32 = 234882053u32;
pub const DIBUTTON_FISHING_MENU: u32 = 234882301u32;
pub const DIBUTTON_FISHING_PAUSE: u32 = 234898684u32;
pub const DIBUTTON_FISHING_RIGHT_LINK: u32 = 234931436u32;
pub const DIBUTTON_FISHING_ROTATE_LEFT_LINK: u32 = 235029732u32;
pub const DIBUTTON_FISHING_ROTATE_RIGHT_LINK: u32 = 235029740u32;
pub const DIBUTTON_FISHING_TYPE: u32 = 234882050u32;
pub const DIBUTTON_FLYINGC_BRAKE_LINK: u32 = 67398880u32;
pub const DIBUTTON_FLYINGC_DEVICE: u32 = 67126526u32;
pub const DIBUTTON_FLYINGC_DISPLAY: u32 = 67118082u32;
pub const DIBUTTON_FLYINGC_FASTER_LINK: u32 = 67359968u32;
pub const DIBUTTON_FLYINGC_FLAPSDOWN: u32 = 67134469u32;
pub const DIBUTTON_FLYINGC_FLAPSUP: u32 = 67134468u32;
pub const DIBUTTON_FLYINGC_GEAR: u32 = 67120131u32;
pub const DIBUTTON_FLYINGC_GLANCE_DOWN_LINK: u32 = 67618024u32;
pub const DIBUTTON_FLYINGC_GLANCE_LEFT_LINK: u32 = 67618020u32;
pub const DIBUTTON_FLYINGC_GLANCE_RIGHT_LINK: u32 = 67618028u32;
pub const DIBUTTON_FLYINGC_GLANCE_UP_LINK: u32 = 67618016u32;
pub const DIBUTTON_FLYINGC_MENU: u32 = 67110141u32;
pub const DIBUTTON_FLYINGC_PAUSE: u32 = 67126524u32;
pub const DIBUTTON_FLYINGC_SLOWER_LINK: u32 = 67359976u32;
pub const DIBUTTON_FLYINGC_VIEW: u32 = 67118081u32;
pub const DIBUTTON_FLYINGH_COUNTER: u32 = 100684804u32;
pub const DIBUTTON_FLYINGH_DEVICE: u32 = 100680958u32;
pub const DIBUTTON_FLYINGH_FASTER_LINK: u32 = 100916448u32;
pub const DIBUTTON_FLYINGH_FIRE: u32 = 100668417u32;
pub const DIBUTTON_FLYINGH_FIRESECONDARY: u32 = 100682759u32;
pub const DIBUTTON_FLYINGH_GEAR: u32 = 100688902u32;
pub const DIBUTTON_FLYINGH_GLANCE_DOWN_LINK: u32 = 101172456u32;
pub const DIBUTTON_FLYINGH_GLANCE_LEFT_LINK: u32 = 101172452u32;
pub const DIBUTTON_FLYINGH_GLANCE_RIGHT_LINK: u32 = 101172460u32;
pub const DIBUTTON_FLYINGH_GLANCE_UP_LINK: u32 = 101172448u32;
pub const DIBUTTON_FLYINGH_MENU: u32 = 100664573u32;
pub const DIBUTTON_FLYINGH_PAUSE: u32 = 100680956u32;
pub const DIBUTTON_FLYINGH_SLOWER_LINK: u32 = 100916456u32;
pub const DIBUTTON_FLYINGH_TARGET: u32 = 100668419u32;
pub const DIBUTTON_FLYINGH_VIEW: u32 = 100688901u32;
pub const DIBUTTON_FLYINGH_WEAPONS: u32 = 100668418u32;
pub const DIBUTTON_FLYINGM_BRAKE_LINK: u32 = 84174048u32;
pub const DIBUTTON_FLYINGM_COUNTER: u32 = 83909636u32;
pub const DIBUTTON_FLYINGM_DEVICE: u32 = 83903742u32;
pub const DIBUTTON_FLYINGM_DISPLAY: u32 = 83911686u32;
pub const DIBUTTON_FLYINGM_FASTER_LINK: u32 = 84137184u32;
pub const DIBUTTON_FLYINGM_FIRE: u32 = 83889153u32;
pub const DIBUTTON_FLYINGM_FIRESECONDARY: u32 = 83905545u32;
pub const DIBUTTON_FLYINGM_FLAPSDOWN: u32 = 83907592u32;
pub const DIBUTTON_FLYINGM_FLAPSUP: u32 = 83907591u32;
pub const DIBUTTON_FLYINGM_GEAR: u32 = 83911690u32;
pub const DIBUTTON_FLYINGM_GLANCE_DOWN_LINK: u32 = 84395240u32;
pub const DIBUTTON_FLYINGM_GLANCE_LEFT_LINK: u32 = 84395236u32;
pub const DIBUTTON_FLYINGM_GLANCE_RIGHT_LINK: u32 = 84395244u32;
pub const DIBUTTON_FLYINGM_GLANCE_UP_LINK: u32 = 84395232u32;
pub const DIBUTTON_FLYINGM_MENU: u32 = 83887357u32;
pub const DIBUTTON_FLYINGM_PAUSE: u32 = 83903740u32;
pub const DIBUTTON_FLYINGM_SLOWER_LINK: u32 = 84137192u32;
pub const DIBUTTON_FLYINGM_TARGET: u32 = 83889155u32;
pub const DIBUTTON_FLYINGM_VIEW: u32 = 83911685u32;
pub const DIBUTTON_FLYINGM_WEAPONS: u32 = 83889154u32;
pub const DIBUTTON_FOOTBALLD_AUDIBLE: u32 = 385893387u32;
pub const DIBUTTON_FOOTBALLD_BACK_LINK: u32 = 385959144u32;
pub const DIBUTTON_FOOTBALLD_BULLRUSH: u32 = 385893385u32;
pub const DIBUTTON_FOOTBALLD_DEVICE: u32 = 385893630u32;
pub const DIBUTTON_FOOTBALLD_FAKE: u32 = 385876997u32;
pub const DIBUTTON_FOOTBALLD_FORWARD_LINK: u32 = 385959136u32;
pub const DIBUTTON_FOOTBALLD_JUMP: u32 = 385876995u32;
pub const DIBUTTON_FOOTBALLD_LEFT_LINK: u32 = 385926372u32;
pub const DIBUTTON_FOOTBALLD_MENU: u32 = 385877245u32;
pub const DIBUTTON_FOOTBALLD_PAUSE: u32 = 385893628u32;
pub const DIBUTTON_FOOTBALLD_PLAY: u32 = 385876993u32;
pub const DIBUTTON_FOOTBALLD_RIGHT_LINK: u32 = 385926380u32;
pub const DIBUTTON_FOOTBALLD_RIP: u32 = 385893386u32;
pub const DIBUTTON_FOOTBALLD_SELECT: u32 = 385876994u32;
pub const DIBUTTON_FOOTBALLD_SPIN: u32 = 385893383u32;
pub const DIBUTTON_FOOTBALLD_SUBSTITUTE: u32 = 385893389u32;
pub const DIBUTTON_FOOTBALLD_SUPERTACKLE: u32 = 385876998u32;
pub const DIBUTTON_FOOTBALLD_SWIM: u32 = 385893384u32;
pub const DIBUTTON_FOOTBALLD_TACKLE: u32 = 385876996u32;
pub const DIBUTTON_FOOTBALLD_ZOOM: u32 = 385893388u32;
pub const DIBUTTON_FOOTBALLO_BACK_LINK: u32 = 369181928u32;
pub const DIBUTTON_FOOTBALLO_DEVICE: u32 = 369116414u32;
pub const DIBUTTON_FOOTBALLO_DIVE: u32 = 369116169u32;
pub const DIBUTTON_FOOTBALLO_FORWARD_LINK: u32 = 369181920u32;
pub const DIBUTTON_FOOTBALLO_JUKE: u32 = 369116166u32;
pub const DIBUTTON_FOOTBALLO_JUMP: u32 = 369099777u32;
pub const DIBUTTON_FOOTBALLO_LEFTARM: u32 = 369099778u32;
pub const DIBUTTON_FOOTBALLO_LEFT_LINK: u32 = 369149156u32;
pub const DIBUTTON_FOOTBALLO_MENU: u32 = 369100029u32;
pub const DIBUTTON_FOOTBALLO_PAUSE: u32 = 369116412u32;
pub const DIBUTTON_FOOTBALLO_RIGHTARM: u32 = 369099779u32;
pub const DIBUTTON_FOOTBALLO_RIGHT_LINK: u32 = 369149164u32;
pub const DIBUTTON_FOOTBALLO_SHOULDER: u32 = 369116167u32;
pub const DIBUTTON_FOOTBALLO_SPIN: u32 = 369099781u32;
pub const DIBUTTON_FOOTBALLO_SUBSTITUTE: u32 = 369116171u32;
pub const DIBUTTON_FOOTBALLO_THROW: u32 = 369099780u32;
pub const DIBUTTON_FOOTBALLO_TURBO: u32 = 369116168u32;
pub const DIBUTTON_FOOTBALLO_ZOOM: u32 = 369116170u32;
pub const DIBUTTON_FOOTBALLP_DEVICE: u32 = 335561982u32;
pub const DIBUTTON_FOOTBALLP_HELP: u32 = 335545347u32;
pub const DIBUTTON_FOOTBALLP_MENU: u32 = 335545597u32;
pub const DIBUTTON_FOOTBALLP_PAUSE: u32 = 335561980u32;
pub const DIBUTTON_FOOTBALLP_PLAY: u32 = 335545345u32;
pub const DIBUTTON_FOOTBALLP_SELECT: u32 = 335545346u32;
pub const DIBUTTON_FOOTBALLQ_AUDIBLE: u32 = 352338953u32;
pub const DIBUTTON_FOOTBALLQ_BACK_LINK: u32 = 352404712u32;
pub const DIBUTTON_FOOTBALLQ_DEVICE: u32 = 352339198u32;
pub const DIBUTTON_FOOTBALLQ_FAKE: u32 = 352322566u32;
pub const DIBUTTON_FOOTBALLQ_FAKESNAP: u32 = 352338951u32;
pub const DIBUTTON_FOOTBALLQ_FORWARD_LINK: u32 = 352404704u32;
pub const DIBUTTON_FOOTBALLQ_JUMP: u32 = 352322563u32;
pub const DIBUTTON_FOOTBALLQ_LEFT_LINK: u32 = 352371940u32;
pub const DIBUTTON_FOOTBALLQ_MENU: u32 = 352322813u32;
pub const DIBUTTON_FOOTBALLQ_MOTION: u32 = 352338952u32;
pub const DIBUTTON_FOOTBALLQ_PASS: u32 = 352322565u32;
pub const DIBUTTON_FOOTBALLQ_PAUSE: u32 = 352339196u32;
pub const DIBUTTON_FOOTBALLQ_RIGHT_LINK: u32 = 352371948u32;
pub const DIBUTTON_FOOTBALLQ_SELECT: u32 = 352322561u32;
pub const DIBUTTON_FOOTBALLQ_SLIDE: u32 = 352322564u32;
pub const DIBUTTON_FOOTBALLQ_SNAP: u32 = 352322562u32;
pub const DIBUTTON_FPS_APPLY: u32 = 150995971u32;
pub const DIBUTTON_FPS_BACKWARD_LINK: u32 = 151078120u32;
pub const DIBUTTON_FPS_CROUCH: u32 = 150995973u32;
pub const DIBUTTON_FPS_DEVICE: u32 = 151012606u32;
pub const DIBUTTON_FPS_DISPLAY: u32 = 151012360u32;
pub const DIBUTTON_FPS_DODGE: u32 = 151012361u32;
pub const DIBUTTON_FPS_FIRE: u32 = 150995969u32;
pub const DIBUTTON_FPS_FIRESECONDARY: u32 = 151012364u32;
pub const DIBUTTON_FPS_FORWARD_LINK: u32 = 151078112u32;
pub const DIBUTTON_FPS_GLANCEL: u32 = 151012362u32;
pub const DIBUTTON_FPS_GLANCER: u32 = 151012363u32;
pub const DIBUTTON_FPS_GLANCE_DOWN_LINK: u32 = 151110888u32;
pub const DIBUTTON_FPS_GLANCE_UP_LINK: u32 = 151110880u32;
pub const DIBUTTON_FPS_JUMP: u32 = 150995974u32;
pub const DIBUTTON_FPS_MENU: u32 = 150996221u32;
pub const DIBUTTON_FPS_PAUSE: u32 = 151012604u32;
pub const DIBUTTON_FPS_ROTATE_LEFT_LINK: u32 = 151045348u32;
pub const DIBUTTON_FPS_ROTATE_RIGHT_LINK: u32 = 151045356u32;
pub const DIBUTTON_FPS_SELECT: u32 = 150995972u32;
pub const DIBUTTON_FPS_STEP_LEFT_LINK: u32 = 151143652u32;
pub const DIBUTTON_FPS_STEP_RIGHT_LINK: u32 = 151143660u32;
pub const DIBUTTON_FPS_STRAFE: u32 = 150995975u32;
pub const DIBUTTON_FPS_WEAPONS: u32 = 150995970u32;
pub const DIBUTTON_GOLF_BACK_LINK: u32 = 402736360u32;
pub const DIBUTTON_GOLF_DEVICE: u32 = 402670846u32;
pub const DIBUTTON_GOLF_DOWN: u32 = 402654212u32;
pub const DIBUTTON_GOLF_FLYBY: u32 = 402654214u32;
pub const DIBUTTON_GOLF_FORWARD_LINK: u32 = 402736352u32;
pub const DIBUTTON_GOLF_LEFT_LINK: u32 = 402703588u32;
pub const DIBUTTON_GOLF_MENU: u32 = 402654461u32;
pub const DIBUTTON_GOLF_PAUSE: u32 = 402670844u32;
pub const DIBUTTON_GOLF_RIGHT_LINK: u32 = 402703596u32;
pub const DIBUTTON_GOLF_SELECT: u32 = 402654210u32;
pub const DIBUTTON_GOLF_SUBSTITUTE: u32 = 402670601u32;
pub const DIBUTTON_GOLF_SWING: u32 = 402654209u32;
pub const DIBUTTON_GOLF_TERRAIN: u32 = 402654213u32;
pub const DIBUTTON_GOLF_TIMEOUT: u32 = 402670600u32;
pub const DIBUTTON_GOLF_UP: u32 = 402654211u32;
pub const DIBUTTON_GOLF_ZOOM: u32 = 402670599u32;
pub const DIBUTTON_HOCKEYD_BACK_LINK: u32 = 436290792u32;
pub const DIBUTTON_HOCKEYD_BLOCK: u32 = 436208644u32;
pub const DIBUTTON_HOCKEYD_BURST: u32 = 436208643u32;
pub const DIBUTTON_HOCKEYD_DEVICE: u32 = 436225278u32;
pub const DIBUTTON_HOCKEYD_FAKE: u32 = 436208645u32;
pub const DIBUTTON_HOCKEYD_FORWARD_LINK: u32 = 436290784u32;
pub const DIBUTTON_HOCKEYD_LEFT_LINK: u32 = 436258020u32;
pub const DIBUTTON_HOCKEYD_MENU: u32 = 436208893u32;
pub const DIBUTTON_HOCKEYD_PAUSE: u32 = 436225276u32;
pub const DIBUTTON_HOCKEYD_PLAYER: u32 = 436208641u32;
pub const DIBUTTON_HOCKEYD_RIGHT_LINK: u32 = 436258028u32;
pub const DIBUTTON_HOCKEYD_STEAL: u32 = 436208642u32;
pub const DIBUTTON_HOCKEYD_STRATEGY: u32 = 436225031u32;
pub const DIBUTTON_HOCKEYD_SUBSTITUTE: u32 = 436225033u32;
pub const DIBUTTON_HOCKEYD_TIMEOUT: u32 = 436225032u32;
pub const DIBUTTON_HOCKEYD_ZOOM: u32 = 436225030u32;
pub const DIBUTTON_HOCKEYG_BACK_LINK: u32 = 453068008u32;
pub const DIBUTTON_HOCKEYG_BLOCK: u32 = 452985860u32;
pub const DIBUTTON_HOCKEYG_DEVICE: u32 = 453002494u32;
pub const DIBUTTON_HOCKEYG_FORWARD_LINK: u32 = 453068000u32;
pub const DIBUTTON_HOCKEYG_LEFT_LINK: u32 = 453035236u32;
pub const DIBUTTON_HOCKEYG_MENU: u32 = 452986109u32;
pub const DIBUTTON_HOCKEYG_PASS: u32 = 452985857u32;
pub const DIBUTTON_HOCKEYG_PAUSE: u32 = 453002492u32;
pub const DIBUTTON_HOCKEYG_POKE: u32 = 452985858u32;
pub const DIBUTTON_HOCKEYG_RIGHT_LINK: u32 = 453035244u32;
pub const DIBUTTON_HOCKEYG_STEAL: u32 = 452985859u32;
pub const DIBUTTON_HOCKEYG_STRATEGY: u32 = 453002246u32;
pub const DIBUTTON_HOCKEYG_SUBSTITUTE: u32 = 453002248u32;
pub const DIBUTTON_HOCKEYG_TIMEOUT: u32 = 453002247u32;
pub const DIBUTTON_HOCKEYG_ZOOM: u32 = 453002245u32;
pub const DIBUTTON_HOCKEYO_BACK_LINK: u32 = 419513576u32;
pub const DIBUTTON_HOCKEYO_BURST: u32 = 419431427u32;
pub const DIBUTTON_HOCKEYO_DEVICE: u32 = 419448062u32;
pub const DIBUTTON_HOCKEYO_FAKE: u32 = 419431429u32;
pub const DIBUTTON_HOCKEYO_FORWARD_LINK: u32 = 419513568u32;
pub const DIBUTTON_HOCKEYO_LEFT_LINK: u32 = 419480804u32;
pub const DIBUTTON_HOCKEYO_MENU: u32 = 419431677u32;
pub const DIBUTTON_HOCKEYO_PASS: u32 = 419431426u32;
pub const DIBUTTON_HOCKEYO_PAUSE: u32 = 419448060u32;
pub const DIBUTTON_HOCKEYO_RIGHT_LINK: u32 = 419480812u32;
pub const DIBUTTON_HOCKEYO_SHOOT: u32 = 419431425u32;
pub const DIBUTTON_HOCKEYO_SPECIAL: u32 = 419431428u32;
pub const DIBUTTON_HOCKEYO_STRATEGY: u32 = 419447815u32;
pub const DIBUTTON_HOCKEYO_SUBSTITUTE: u32 = 419447817u32;
pub const DIBUTTON_HOCKEYO_TIMEOUT: u32 = 419447816u32;
pub const DIBUTTON_HOCKEYO_ZOOM: u32 = 419447814u32;
pub const DIBUTTON_HUNTING_AIM: u32 = 218104834u32;
pub const DIBUTTON_HUNTING_BACK_LINK: u32 = 218186984u32;
pub const DIBUTTON_HUNTING_BINOCULAR: u32 = 218104836u32;
pub const DIBUTTON_HUNTING_CALL: u32 = 218104837u32;
pub const DIBUTTON_HUNTING_CROUCH: u32 = 218121225u32;
pub const DIBUTTON_HUNTING_DEVICE: u32 = 218121470u32;
pub const DIBUTTON_HUNTING_DISPLAY: u32 = 218121224u32;
pub const DIBUTTON_HUNTING_FIRE: u32 = 218104833u32;
pub const DIBUTTON_HUNTING_FIRESECONDARY: u32 = 218121227u32;
pub const DIBUTTON_HUNTING_FORWARD_LINK: u32 = 218186976u32;
pub const DIBUTTON_HUNTING_JUMP: u32 = 218121226u32;
pub const DIBUTTON_HUNTING_LEFT_LINK: u32 = 218154212u32;
pub const DIBUTTON_HUNTING_MAP: u32 = 218104838u32;
pub const DIBUTTON_HUNTING_MENU: u32 = 218105085u32;
pub const DIBUTTON_HUNTING_PAUSE: u32 = 218121468u32;
pub const DIBUTTON_HUNTING_RIGHT_LINK: u32 = 218154220u32;
pub const DIBUTTON_HUNTING_ROTATE_LEFT_LINK: u32 = 218252516u32;
pub const DIBUTTON_HUNTING_ROTATE_RIGHT_LINK: u32 = 218252524u32;
pub const DIBUTTON_HUNTING_SPECIAL: u32 = 218104839u32;
pub const DIBUTTON_HUNTING_WEAPON: u32 = 218104835u32;
pub const DIBUTTON_MECHA_BACK_LINK: u32 = 687949032u32;
pub const DIBUTTON_MECHA_CENTER: u32 = 687883271u32;
pub const DIBUTTON_MECHA_DEVICE: u32 = 687883518u32;
pub const DIBUTTON_MECHA_FASTER_LINK: u32 = 688112864u32;
pub const DIBUTTON_MECHA_FIRE: u32 = 687866881u32;
pub const DIBUTTON_MECHA_FIRESECONDARY: u32 = 687883273u32;
pub const DIBUTTON_MECHA_FORWARD_LINK: u32 = 687949024u32;
pub const DIBUTTON_MECHA_JUMP: u32 = 687866886u32;
pub const DIBUTTON_MECHA_LEFT_LINK: u32 = 687916260u32;
pub const DIBUTTON_MECHA_MENU: u32 = 687867133u32;
pub const DIBUTTON_MECHA_PAUSE: u32 = 687883516u32;
pub const DIBUTTON_MECHA_REVERSE: u32 = 687866884u32;
pub const DIBUTTON_MECHA_RIGHT_LINK: u32 = 687916268u32;
pub const DIBUTTON_MECHA_ROTATE_LEFT_LINK: u32 = 688014564u32;
pub const DIBUTTON_MECHA_ROTATE_RIGHT_LINK: u32 = 688014572u32;
pub const DIBUTTON_MECHA_SLOWER_LINK: u32 = 688112872u32;
pub const DIBUTTON_MECHA_TARGET: u32 = 687866883u32;
pub const DIBUTTON_MECHA_VIEW: u32 = 687883272u32;
pub const DIBUTTON_MECHA_WEAPONS: u32 = 687866882u32;
pub const DIBUTTON_MECHA_ZOOM: u32 = 687866885u32;
pub const DIBUTTON_RACQUET_BACKSWING: u32 = 536871938u32;
pub const DIBUTTON_RACQUET_BACK_LINK: u32 = 536954088u32;
pub const DIBUTTON_RACQUET_DEVICE: u32 = 536888574u32;
pub const DIBUTTON_RACQUET_FORWARD_LINK: u32 = 536954080u32;
pub const DIBUTTON_RACQUET_LEFT_LINK: u32 = 536921316u32;
pub const DIBUTTON_RACQUET_MENU: u32 = 536872189u32;
pub const DIBUTTON_RACQUET_PAUSE: u32 = 536888572u32;
pub const DIBUTTON_RACQUET_RIGHT_LINK: u32 = 536921324u32;
pub const DIBUTTON_RACQUET_SELECT: u32 = 536871941u32;
pub const DIBUTTON_RACQUET_SMASH: u32 = 536871939u32;
pub const DIBUTTON_RACQUET_SPECIAL: u32 = 536871940u32;
pub const DIBUTTON_RACQUET_SUBSTITUTE: u32 = 536888327u32;
pub const DIBUTTON_RACQUET_SWING: u32 = 536871937u32;
pub const DIBUTTON_RACQUET_TIMEOUT: u32 = 536888326u32;
pub const DIBUTTON_REMOTE_ADJUST: u32 = 654334990u32;
pub const DIBUTTON_REMOTE_CABLE: u32 = 654334985u32;
pub const DIBUTTON_REMOTE_CD: u32 = 654334986u32;
pub const DIBUTTON_REMOTE_CHANGE: u32 = 654320646u32;
pub const DIBUTTON_REMOTE_CUE: u32 = 654320644u32;
pub const DIBUTTON_REMOTE_DEVICE: u32 = 654329086u32;
pub const DIBUTTON_REMOTE_DIGIT0: u32 = 654332943u32;
pub const DIBUTTON_REMOTE_DIGIT1: u32 = 654332944u32;
pub const DIBUTTON_REMOTE_DIGIT2: u32 = 654332945u32;
pub const DIBUTTON_REMOTE_DIGIT3: u32 = 654332946u32;
pub const DIBUTTON_REMOTE_DIGIT4: u32 = 654332947u32;
pub const DIBUTTON_REMOTE_DIGIT5: u32 = 654332948u32;
pub const DIBUTTON_REMOTE_DIGIT6: u32 = 654332949u32;
pub const DIBUTTON_REMOTE_DIGIT7: u32 = 654332950u32;
pub const DIBUTTON_REMOTE_DIGIT8: u32 = 654332951u32;
pub const DIBUTTON_REMOTE_DIGIT9: u32 = 654332952u32;
pub const DIBUTTON_REMOTE_DVD: u32 = 654334989u32;
pub const DIBUTTON_REMOTE_MENU: u32 = 654312701u32;
pub const DIBUTTON_REMOTE_MUTE: u32 = 654312449u32;
pub const DIBUTTON_REMOTE_PAUSE: u32 = 654329084u32;
pub const DIBUTTON_REMOTE_PLAY: u32 = 654320643u32;
pub const DIBUTTON_REMOTE_RECORD: u32 = 654320647u32;
pub const DIBUTTON_REMOTE_REVIEW: u32 = 654320645u32;
pub const DIBUTTON_REMOTE_SELECT: u32 = 654312450u32;
pub const DIBUTTON_REMOTE_TUNER: u32 = 654334988u32;
pub const DIBUTTON_REMOTE_TV: u32 = 654334984u32;
pub const DIBUTTON_REMOTE_VCR: u32 = 654334987u32;
pub const DIBUTTON_SKIING_CAMERA: u32 = 486540291u32;
pub const DIBUTTON_SKIING_CROUCH: u32 = 486540290u32;
pub const DIBUTTON_SKIING_DEVICE: u32 = 486556926u32;
pub const DIBUTTON_SKIING_FASTER_LINK: u32 = 486622432u32;
pub const DIBUTTON_SKIING_JUMP: u32 = 486540289u32;
pub const DIBUTTON_SKIING_LEFT_LINK: u32 = 486589668u32;
pub const DIBUTTON_SKIING_MENU: u32 = 486540541u32;
pub const DIBUTTON_SKIING_PAUSE: u32 = 486556924u32;
pub const DIBUTTON_SKIING_RIGHT_LINK: u32 = 486589676u32;
pub const DIBUTTON_SKIING_SELECT: u32 = 486540293u32;
pub const DIBUTTON_SKIING_SLOWER_LINK: u32 = 486622440u32;
pub const DIBUTTON_SKIING_SPECIAL1: u32 = 486540292u32;
pub const DIBUTTON_SKIING_SPECIAL2: u32 = 486540294u32;
pub const DIBUTTON_SKIING_ZOOM: u32 = 486556679u32;
pub const DIBUTTON_SOCCERD_BACK_LINK: u32 = 520176872u32;
pub const DIBUTTON_SOCCERD_BLOCK: u32 = 520094721u32;
pub const DIBUTTON_SOCCERD_CLEAR: u32 = 520111114u32;
pub const DIBUTTON_SOCCERD_DEVICE: u32 = 520111358u32;
pub const DIBUTTON_SOCCERD_FAKE: u32 = 520094723u32;
pub const DIBUTTON_SOCCERD_FORWARD_LINK: u32 = 520176864u32;
pub const DIBUTTON_SOCCERD_FOUL: u32 = 520111112u32;
pub const DIBUTTON_SOCCERD_GOALIECHARGE: u32 = 520111115u32;
pub const DIBUTTON_SOCCERD_HEAD: u32 = 520111113u32;
pub const DIBUTTON_SOCCERD_LEFT_LINK: u32 = 520144100u32;
pub const DIBUTTON_SOCCERD_MENU: u32 = 520094973u32;
pub const DIBUTTON_SOCCERD_PAUSE: u32 = 520111356u32;
pub const DIBUTTON_SOCCERD_PLAYER: u32 = 520094724u32;
pub const DIBUTTON_SOCCERD_RIGHT_LINK: u32 = 520144108u32;
pub const DIBUTTON_SOCCERD_SELECT: u32 = 520094726u32;
pub const DIBUTTON_SOCCERD_SLIDE: u32 = 520094727u32;
pub const DIBUTTON_SOCCERD_SPECIAL: u32 = 520094725u32;
pub const DIBUTTON_SOCCERD_STEAL: u32 = 520094722u32;
pub const DIBUTTON_SOCCERD_SUBSTITUTE: u32 = 520111116u32;
pub const DIBUTTON_SOCCERO_BACK_LINK: u32 = 503399656u32;
pub const DIBUTTON_SOCCERO_CONTROL: u32 = 503333900u32;
pub const DIBUTTON_SOCCERO_DEVICE: u32 = 503334142u32;
pub const DIBUTTON_SOCCERO_FAKE: u32 = 503317507u32;
pub const DIBUTTON_SOCCERO_FORWARD_LINK: u32 = 503399648u32;
pub const DIBUTTON_SOCCERO_HEAD: u32 = 503333901u32;
pub const DIBUTTON_SOCCERO_LEFT_LINK: u32 = 503366884u32;
pub const DIBUTTON_SOCCERO_MENU: u32 = 503317757u32;
pub const DIBUTTON_SOCCERO_PASS: u32 = 503317506u32;
pub const DIBUTTON_SOCCERO_PASSTHRU: u32 = 503333898u32;
pub const DIBUTTON_SOCCERO_PAUSE: u32 = 503334140u32;
pub const DIBUTTON_SOCCERO_PLAYER: u32 = 503317508u32;
pub const DIBUTTON_SOCCERO_RIGHT_LINK: u32 = 503366892u32;
pub const DIBUTTON_SOCCERO_SELECT: u32 = 503317510u32;
pub const DIBUTTON_SOCCERO_SHOOT: u32 = 503317505u32;
pub const DIBUTTON_SOCCERO_SHOOTHIGH: u32 = 503333897u32;
pub const DIBUTTON_SOCCERO_SHOOTLOW: u32 = 503333896u32;
pub const DIBUTTON_SOCCERO_SPECIAL1: u32 = 503317509u32;
pub const DIBUTTON_SOCCERO_SPRINT: u32 = 503333899u32;
pub const DIBUTTON_SOCCERO_SUBSTITUTE: u32 = 503333895u32;
pub const DIBUTTON_SPACESIM_BACKWARD_LINK: u32 = 117523688u32;
pub const DIBUTTON_SPACESIM_DEVICE: u32 = 117458174u32;
pub const DIBUTTON_SPACESIM_DISPLAY: u32 = 117457925u32;
pub const DIBUTTON_SPACESIM_FASTER_LINK: u32 = 117687520u32;
pub const DIBUTTON_SPACESIM_FIRE: u32 = 117441537u32;
pub const DIBUTTON_SPACESIM_FIRESECONDARY: u32 = 117457929u32;
pub const DIBUTTON_SPACESIM_FORWARD_LINK: u32 = 117523680u32;
pub const DIBUTTON_SPACESIM_GEAR: u32 = 117457928u32;
pub const DIBUTTON_SPACESIM_GLANCE_DOWN_LINK: u32 = 117949672u32;
pub const DIBUTTON_SPACESIM_GLANCE_LEFT_LINK: u32 = 117949668u32;
pub const DIBUTTON_SPACESIM_GLANCE_RIGHT_LINK: u32 = 117949676u32;
pub const DIBUTTON_SPACESIM_GLANCE_UP_LINK: u32 = 117949664u32;
pub const DIBUTTON_SPACESIM_LEFT_LINK: u32 = 117490916u32;
pub const DIBUTTON_SPACESIM_LOWER: u32 = 117457927u32;
pub const DIBUTTON_SPACESIM_MENU: u32 = 117441789u32;
pub const DIBUTTON_SPACESIM_PAUSE: u32 = 117458172u32;
pub const DIBUTTON_SPACESIM_RAISE: u32 = 117457926u32;
pub const DIBUTTON_SPACESIM_RIGHT_LINK: u32 = 117490924u32;
pub const DIBUTTON_SPACESIM_SLOWER_LINK: u32 = 117687528u32;
pub const DIBUTTON_SPACESIM_TARGET: u32 = 117441539u32;
pub const DIBUTTON_SPACESIM_TURN_LEFT_LINK: u32 = 117589220u32;
pub const DIBUTTON_SPACESIM_TURN_RIGHT_LINK: u32 = 117589228u32;
pub const DIBUTTON_SPACESIM_VIEW: u32 = 117457924u32;
pub const DIBUTTON_SPACESIM_WEAPONS: u32 = 117441538u32;
pub const DIBUTTON_STRATEGYR_APPLY: u32 = 184550402u32;
pub const DIBUTTON_STRATEGYR_ATTACK: u32 = 184550404u32;
pub const DIBUTTON_STRATEGYR_BACK_LINK: u32 = 184632552u32;
pub const DIBUTTON_STRATEGYR_CAST: u32 = 184550405u32;
pub const DIBUTTON_STRATEGYR_CROUCH: u32 = 184550406u32;
pub const DIBUTTON_STRATEGYR_DEVICE: u32 = 184567038u32;
pub const DIBUTTON_STRATEGYR_DISPLAY: u32 = 184566793u32;
pub const DIBUTTON_STRATEGYR_FORWARD_LINK: u32 = 184632544u32;
pub const DIBUTTON_STRATEGYR_GET: u32 = 184550401u32;
pub const DIBUTTON_STRATEGYR_JUMP: u32 = 184550407u32;
pub const DIBUTTON_STRATEGYR_LEFT_LINK: u32 = 184599780u32;
pub const DIBUTTON_STRATEGYR_MAP: u32 = 184566792u32;
pub const DIBUTTON_STRATEGYR_MENU: u32 = 184550653u32;
pub const DIBUTTON_STRATEGYR_PAUSE: u32 = 184567036u32;
pub const DIBUTTON_STRATEGYR_RIGHT_LINK: u32 = 184599788u32;
pub const DIBUTTON_STRATEGYR_ROTATE_LEFT_LINK: u32 = 184698084u32;
pub const DIBUTTON_STRATEGYR_ROTATE_RIGHT_LINK: u32 = 184698092u32;
pub const DIBUTTON_STRATEGYR_SELECT: u32 = 184550403u32;
pub const DIBUTTON_STRATEGYT_APPLY: u32 = 201327619u32;
pub const DIBUTTON_STRATEGYT_BACK_LINK: u32 = 201409768u32;
pub const DIBUTTON_STRATEGYT_DEVICE: u32 = 201344254u32;
pub const DIBUTTON_STRATEGYT_DISPLAY: u32 = 201344008u32;
pub const DIBUTTON_STRATEGYT_FORWARD_LINK: u32 = 201409760u32;
pub const DIBUTTON_STRATEGYT_INSTRUCT: u32 = 201327618u32;
pub const DIBUTTON_STRATEGYT_LEFT_LINK: u32 = 201376996u32;
pub const DIBUTTON_STRATEGYT_MAP: u32 = 201344007u32;
pub const DIBUTTON_STRATEGYT_MENU: u32 = 201327869u32;
pub const DIBUTTON_STRATEGYT_PAUSE: u32 = 201344252u32;
pub const DIBUTTON_STRATEGYT_RIGHT_LINK: u32 = 201377004u32;
pub const DIBUTTON_STRATEGYT_SELECT: u32 = 201327617u32;
pub const DIBUTTON_STRATEGYT_TEAM: u32 = 201327620u32;
pub const DIBUTTON_STRATEGYT_TURN: u32 = 201327621u32;
pub const DIBUTTON_STRATEGYT_ZOOM: u32 = 201344006u32;
pub const DIBUTTON_TPS_ACTION: u32 = 167773186u32;
pub const DIBUTTON_TPS_BACKWARD_LINK: u32 = 167855336u32;
pub const DIBUTTON_TPS_DEVICE: u32 = 167789822u32;
pub const DIBUTTON_TPS_DODGE: u32 = 167789577u32;
pub const DIBUTTON_TPS_FORWARD_LINK: u32 = 167855328u32;
pub const DIBUTTON_TPS_GLANCE_DOWN_LINK: u32 = 168281320u32;
pub const DIBUTTON_TPS_GLANCE_LEFT_LINK: u32 = 168281316u32;
pub const DIBUTTON_TPS_GLANCE_RIGHT_LINK: u32 = 168281324u32;
pub const DIBUTTON_TPS_GLANCE_UP_LINK: u32 = 168281312u32;
pub const DIBUTTON_TPS_INVENTORY: u32 = 167789578u32;
pub const DIBUTTON_TPS_JUMP: u32 = 167773189u32;
pub const DIBUTTON_TPS_MENU: u32 = 167773437u32;
pub const DIBUTTON_TPS_PAUSE: u32 = 167789820u32;
pub const DIBUTTON_TPS_RUN: u32 = 167773185u32;
pub const DIBUTTON_TPS_SELECT: u32 = 167773187u32;
pub const DIBUTTON_TPS_STEPLEFT: u32 = 167789575u32;
pub const DIBUTTON_TPS_STEPRIGHT: u32 = 167789576u32;
pub const DIBUTTON_TPS_TURN_LEFT_LINK: u32 = 167920868u32;
pub const DIBUTTON_TPS_TURN_RIGHT_LINK: u32 = 167920876u32;
pub const DIBUTTON_TPS_USE: u32 = 167773188u32;
pub const DIBUTTON_TPS_VIEW: u32 = 167789574u32;
pub const DICD_DEFAULT: u32 = 0u32;
pub const DICD_EDIT: u32 = 1u32;
pub const DIDAL_BOTTOMALIGNED: u32 = 8u32;
pub const DIDAL_CENTERED: u32 = 0u32;
pub const DIDAL_LEFTALIGNED: u32 = 1u32;
pub const DIDAL_MIDDLE: u32 = 0u32;
pub const DIDAL_RIGHTALIGNED: u32 = 2u32;
pub const DIDAL_TOPALIGNED: u32 = 4u32;
pub const DIDBAM_DEFAULT: u32 = 0u32;
pub const DIDBAM_HWDEFAULTS: u32 = 4u32;
pub const DIDBAM_INITIALIZE: u32 = 2u32;
pub const DIDBAM_PRESERVE: u32 = 1u32;
pub const DIDC_ALIAS: u32 = 65536u32;
pub const DIDC_ATTACHED: u32 = 1u32;
pub const DIDC_DEADBAND: u32 = 16384u32;
pub const DIDC_EMULATED: u32 = 4u32;
pub const DIDC_FFATTACK: u32 = 512u32;
pub const DIDC_FFFADE: u32 = 1024u32;
pub const DIDC_FORCEFEEDBACK: u32 = 256u32;
pub const DIDC_HIDDEN: u32 = 262144u32;
pub const DIDC_PHANTOM: u32 = 131072u32;
pub const DIDC_POLLEDDATAFORMAT: u32 = 8u32;
pub const DIDC_POLLEDDEVICE: u32 = 2u32;
pub const DIDC_POSNEGCOEFFICIENTS: u32 = 4096u32;
pub const DIDC_POSNEGSATURATION: u32 = 8192u32;
pub const DIDC_SATURATION: u32 = 2048u32;
pub const DIDC_STARTDELAY: u32 = 32768u32;
pub const DIDEVTYPEJOYSTICK_FLIGHTSTICK: u32 = 3u32;
pub const DIDEVTYPEJOYSTICK_GAMEPAD: u32 = 4u32;
pub const DIDEVTYPEJOYSTICK_HEADTRACKER: u32 = 7u32;
pub const DIDEVTYPEJOYSTICK_RUDDER: u32 = 5u32;
pub const DIDEVTYPEJOYSTICK_TRADITIONAL: u32 = 2u32;
pub const DIDEVTYPEJOYSTICK_UNKNOWN: u32 = 1u32;
pub const DIDEVTYPEJOYSTICK_WHEEL: u32 = 6u32;
pub const DIDEVTYPEKEYBOARD_J3100: u32 = 12u32;
pub const DIDEVTYPEKEYBOARD_JAPAN106: u32 = 10u32;
pub const DIDEVTYPEKEYBOARD_JAPANAX: u32 = 11u32;
pub const DIDEVTYPEKEYBOARD_NEC98: u32 = 7u32;
pub const DIDEVTYPEKEYBOARD_NEC98106: u32 = 9u32;
pub const DIDEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8u32;
pub const DIDEVTYPEKEYBOARD_NOKIA1050: u32 = 5u32;
pub const DIDEVTYPEKEYBOARD_NOKIA9140: u32 = 6u32;
pub const DIDEVTYPEKEYBOARD_OLIVETTI: u32 = 2u32;
pub const DIDEVTYPEKEYBOARD_PCAT: u32 = 3u32;
pub const DIDEVTYPEKEYBOARD_PCENH: u32 = 4u32;
pub const DIDEVTYPEKEYBOARD_PCXT: u32 = 1u32;
pub const DIDEVTYPEKEYBOARD_UNKNOWN: u32 = 0u32;
pub const DIDEVTYPEMOUSE_FINGERSTICK: u32 = 3u32;
pub const DIDEVTYPEMOUSE_TOUCHPAD: u32 = 4u32;
pub const DIDEVTYPEMOUSE_TRACKBALL: u32 = 5u32;
pub const DIDEVTYPEMOUSE_TRADITIONAL: u32 = 2u32;
pub const DIDEVTYPEMOUSE_UNKNOWN: u32 = 1u32;
pub const DIDEVTYPE_DEVICE: u32 = 1u32;
pub const DIDEVTYPE_HID: u32 = 65536u32;
pub const DIDEVTYPE_JOYSTICK: u32 = 4u32;
pub const DIDEVTYPE_KEYBOARD: u32 = 3u32;
pub const DIDEVTYPE_MOUSE: u32 = 2u32;
pub const DIDFT_ABSAXIS: u32 = 2u32;
pub const DIDFT_ALIAS: u32 = 134217728u32;
pub const DIDFT_ALL: u32 = 0u32;
pub const DIDFT_ANYINSTANCE: u32 = 16776960u32;
pub const DIDFT_AXIS: u32 = 3u32;
pub const DIDFT_BUTTON: u32 = 12u32;
pub const DIDFT_COLLECTION: u32 = 64u32;
pub const DIDFT_FFACTUATOR: u32 = 16777216u32;
pub const DIDFT_FFEFFECTTRIGGER: u32 = 33554432u32;
pub const DIDFT_INSTANCEMASK: u32 = 16776960u32;
pub const DIDFT_NOCOLLECTION: u32 = 16776960u32;
pub const DIDFT_NODATA: u32 = 128u32;
pub const DIDFT_OUTPUT: u32 = 268435456u32;
pub const DIDFT_POV: u32 = 16u32;
pub const DIDFT_PSHBUTTON: u32 = 4u32;
pub const DIDFT_RELAXIS: u32 = 1u32;
pub const DIDFT_TGLBUTTON: u32 = 8u32;
pub const DIDFT_VENDORDEFINED: u32 = 67108864u32;
pub const DIDF_ABSAXIS: u32 = 1u32;
pub const DIDF_RELAXIS: u32 = 2u32;
pub const DIDIFT_CONFIGURATION: u32 = 1u32;
pub const DIDIFT_DELETE: u32 = 16777216u32;
pub const DIDIFT_OVERLAY: u32 = 2u32;
pub const DIDOI_ASPECTACCEL: u32 = 768u32;
pub const DIDOI_ASPECTFORCE: u32 = 1024u32;
pub const DIDOI_ASPECTMASK: u32 = 3840u32;
pub const DIDOI_ASPECTPOSITION: u32 = 256u32;
pub const DIDOI_ASPECTVELOCITY: u32 = 512u32;
pub const DIDOI_FFACTUATOR: u32 = 1u32;
pub const DIDOI_FFEFFECTTRIGGER: u32 = 2u32;
pub const DIDOI_GUIDISUSAGE: u32 = 65536u32;
pub const DIDOI_POLLED: u32 = 32768u32;
pub const DIDSAM_DEFAULT: u32 = 0u32;
pub const DIDSAM_FORCESAVE: u32 = 2u32;
pub const DIDSAM_NOUSER: u32 = 1u32;
pub const DIEB_NOTRIGGER: u32 = 4294967295u32;
pub const DIEDBSFL_ATTACHEDONLY: u32 = 0u32;
pub const DIEDBSFL_AVAILABLEDEVICES: u32 = 4096u32;
pub const DIEDBSFL_FORCEFEEDBACK: u32 = 256u32;
pub const DIEDBSFL_MULTIMICEKEYBOARDS: u32 = 8192u32;
pub const DIEDBSFL_NONGAMINGDEVICES: u32 = 16384u32;
pub const DIEDBSFL_THISUSER: u32 = 16u32;
pub const DIEDBSFL_VALID: u32 = 28944u32;
pub const DIEDBS_MAPPEDPRI1: u32 = 1u32;
pub const DIEDBS_MAPPEDPRI2: u32 = 2u32;
pub const DIEDBS_NEWDEVICE: u32 = 32u32;
pub const DIEDBS_RECENTDEVICE: u32 = 16u32;
pub const DIEDFL_ALLDEVICES: u32 = 0u32;
pub const DIEDFL_ATTACHEDONLY: u32 = 1u32;
pub const DIEDFL_FORCEFEEDBACK: u32 = 256u32;
pub const DIEDFL_INCLUDEALIASES: u32 = 65536u32;
pub const DIEDFL_INCLUDEHIDDEN: u32 = 262144u32;
pub const DIEDFL_INCLUDEPHANTOMS: u32 = 131072u32;
pub const DIEFF_CARTESIAN: u32 = 16u32;
pub const DIEFF_OBJECTIDS: u32 = 1u32;
pub const DIEFF_OBJECTOFFSETS: u32 = 2u32;
pub const DIEFF_POLAR: u32 = 32u32;
pub const DIEFF_SPHERICAL: u32 = 64u32;
pub const DIEFT_ALL: u32 = 0u32;
pub const DIEFT_CONDITION: u32 = 4u32;
pub const DIEFT_CONSTANTFORCE: u32 = 1u32;
pub const DIEFT_CUSTOMFORCE: u32 = 5u32;
pub const DIEFT_DEADBAND: u32 = 16384u32;
pub const DIEFT_FFATTACK: u32 = 512u32;
pub const DIEFT_FFFADE: u32 = 1024u32;
pub const DIEFT_HARDWARE: u32 = 255u32;
pub const DIEFT_PERIODIC: u32 = 3u32;
pub const DIEFT_POSNEGCOEFFICIENTS: u32 = 4096u32;
pub const DIEFT_POSNEGSATURATION: u32 = 8192u32;
pub const DIEFT_RAMPFORCE: u32 = 2u32;
pub const DIEFT_SATURATION: u32 = 2048u32;
pub const DIEFT_STARTDELAY: u32 = 32768u32;
pub const DIEGES_EMULATED: u32 = 2u32;
pub const DIEGES_PLAYING: u32 = 1u32;
pub const DIENUM_CONTINUE: u32 = 1u32;
pub const DIENUM_STOP: u32 = 0u32;
pub const DIEP_ALLPARAMS: u32 = 1023u32;
pub const DIEP_ALLPARAMS_DX5: u32 = 511u32;
pub const DIEP_AXES: u32 = 32u32;
pub const DIEP_DIRECTION: u32 = 64u32;
pub const DIEP_DURATION: u32 = 1u32;
pub const DIEP_ENVELOPE: u32 = 128u32;
pub const DIEP_GAIN: u32 = 4u32;
pub const DIEP_NODOWNLOAD: u32 = 2147483648u32;
pub const DIEP_NORESTART: u32 = 1073741824u32;
pub const DIEP_SAMPLEPERIOD: u32 = 2u32;
pub const DIEP_START: u32 = 536870912u32;
pub const DIEP_STARTDELAY: u32 = 512u32;
pub const DIEP_TRIGGERBUTTON: u32 = 8u32;
pub const DIEP_TRIGGERREPEATINTERVAL: u32 = 16u32;
pub const DIEP_TYPESPECIFICPARAMS: u32 = 256u32;
pub const DIERR_ACQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024726i32);
pub const DIERR_ALREADYINITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147023649i32);
pub const DIERR_BADDRIVERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024777i32);
pub const DIERR_BADINF: i32 = -2147220478i32;
pub const DIERR_BETADIRECTINPUTVERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147023743i32);
pub const DIERR_CANCELLED: i32 = -2147220479i32;
pub const DIERR_DEVICEFULL: i32 = -2147220991i32;
pub const DIERR_DEVICENOTREG: i32 = -2147221164i32;
pub const DIERR_DRIVERFIRST: i32 = -2147220736i32;
pub const DIERR_DRIVERLAST: i32 = -2147220481i32;
pub const DIERR_EFFECTPLAYING: i32 = -2147220984i32;
pub const DIERR_GENERIC: i32 = -2147467259i32;
pub const DIERR_HANDLEEXISTS: i32 = -2147024891i32;
pub const DIERR_HASEFFECTS: i32 = -2147220988i32;
pub const DIERR_INCOMPLETEEFFECT: i32 = -2147220986i32;
pub const DIERR_INPUTLOST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024866i32);
pub const DIERR_INSUFFICIENTPRIVS: i32 = -2147220992i32;
pub const DIERR_INVALIDCLASSINSTALLER: i32 = -2147220480i32;
pub const DIERR_INVALIDPARAM: i32 = -2147024809i32;
pub const DIERR_MAPFILEFAIL: i32 = -2147220981i32;
pub const DIERR_MOREDATA: i32 = -2147220990i32;
pub const DIERR_NOAGGREGATION: i32 = -2147221232i32;
pub const DIERR_NOINTERFACE: i32 = -2147467262i32;
pub const DIERR_NOMOREITEMS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024637i32);
pub const DIERR_NOTACQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024884i32);
pub const DIERR_NOTBUFFERED: i32 = -2147220985i32;
pub const DIERR_NOTDOWNLOADED: i32 = -2147220989i32;
pub const DIERR_NOTEXCLUSIVEACQUIRED: i32 = -2147220987i32;
pub const DIERR_NOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024894i32);
pub const DIERR_NOTINITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024875i32);
pub const DIERR_OBJECTNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147024894i32);
pub const DIERR_OLDDIRECTINPUTVERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147023746i32);
pub const DIERR_OTHERAPPHASPRIO: i32 = -2147024891i32;
pub const DIERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const DIERR_READONLY: i32 = -2147024891i32;
pub const DIERR_REPORTFULL: i32 = -2147220982i32;
pub const DIERR_UNPLUGGED: i32 = -2147220983i32;
pub const DIERR_UNSUPPORTED: i32 = -2147467263i32;
pub const DIES_NODOWNLOAD: u32 = 2147483648u32;
pub const DIES_SOLO: u32 = 1u32;
pub const DIFEF_DEFAULT: u32 = 0u32;
pub const DIFEF_INCLUDENONSTANDARD: u32 = 1u32;
pub const DIFEF_MODIFYIFNEEDED: u32 = 16u32;
pub const DIGDD_PEEK: u32 = 1u32;
pub const DIGFFS_ACTUATORSOFF: u32 = 32u32;
pub const DIGFFS_ACTUATORSON: u32 = 16u32;
pub const DIGFFS_DEVICELOST: u32 = 2147483648u32;
pub const DIGFFS_EMPTY: u32 = 1u32;
pub const DIGFFS_PAUSED: u32 = 4u32;
pub const DIGFFS_POWEROFF: u32 = 128u32;
pub const DIGFFS_POWERON: u32 = 64u32;
pub const DIGFFS_SAFETYSWITCHOFF: u32 = 512u32;
pub const DIGFFS_SAFETYSWITCHON: u32 = 256u32;
pub const DIGFFS_STOPPED: u32 = 2u32;
pub const DIGFFS_USERFFSWITCHOFF: u32 = 2048u32;
pub const DIGFFS_USERFFSWITCHON: u32 = 1024u32;
pub const DIHATSWITCH_2DCONTROL_HATSWITCH: u32 = 587220481u32;
pub const DIHATSWITCH_3DCONTROL_HATSWITCH: u32 = 603997697u32;
pub const DIHATSWITCH_ARCADEP_VIEW: u32 = 570443265u32;
pub const DIHATSWITCH_ARCADES_VIEW: u32 = 553666049u32;
pub const DIHATSWITCH_BBALLD_GLANCE: u32 = 318785025u32;
pub const DIHATSWITCH_BBALLO_GLANCE: u32 = 302007809u32;
pub const DIHATSWITCH_BIKINGM_SCROLL: u32 = 469779969u32;
pub const DIHATSWITCH_CADF_HATSWITCH: u32 = 620774913u32;
pub const DIHATSWITCH_CADM_HATSWITCH: u32 = 637552129u32;
pub const DIHATSWITCH_DRIVINGC_GLANCE: u32 = 33572353u32;
pub const DIHATSWITCH_DRIVINGR_GLANCE: u32 = 16795137u32;
pub const DIHATSWITCH_DRIVINGT_GLANCE: u32 = 50349569u32;
pub const DIHATSWITCH_FIGHTINGH_SLIDE: u32 = 134235649u32;
pub const DIHATSWITCH_FISHING_GLANCE: u32 = 234898945u32;
pub const DIHATSWITCH_FLYINGC_GLANCE: u32 = 67126785u32;
pub const DIHATSWITCH_FLYINGH_GLANCE: u32 = 100681217u32;
pub const DIHATSWITCH_FLYINGM_GLANCE: u32 = 83904001u32;
pub const DIHATSWITCH_FPS_GLANCE: u32 = 151012865u32;
pub const DIHATSWITCH_GOLF_SCROLL: u32 = 402671105u32;
pub const DIHATSWITCH_HOCKEYD_SCROLL: u32 = 436225537u32;
pub const DIHATSWITCH_HOCKEYG_SCROLL: u32 = 453002753u32;
pub const DIHATSWITCH_HOCKEYO_SCROLL: u32 = 419448321u32;
pub const DIHATSWITCH_HUNTING_GLANCE: u32 = 218121729u32;
pub const DIHATSWITCH_MECHA_GLANCE: u32 = 687883777u32;
pub const DIHATSWITCH_RACQUET_GLANCE: u32 = 536888833u32;
pub const DIHATSWITCH_SKIING_GLANCE: u32 = 486557185u32;
pub const DIHATSWITCH_SOCCERD_GLANCE: u32 = 520111617u32;
pub const DIHATSWITCH_SOCCERO_GLANCE: u32 = 503334401u32;
pub const DIHATSWITCH_SPACESIM_GLANCE: u32 = 117458433u32;
pub const DIHATSWITCH_STRATEGYR_GLANCE: u32 = 184567297u32;
pub const DIHATSWITCH_TPS_GLANCE: u32 = 167790081u32;
pub const DIJC_CALLOUT: u32 = 8u32;
pub const DIJC_GAIN: u32 = 4u32;
pub const DIJC_GUIDINSTANCE: u32 = 1u32;
pub const DIJC_REGHWCONFIGTYPE: u32 = 2u32;
pub const DIJC_WDMGAMEPORT: u32 = 16u32;
pub const DIJU_GAMEPORTEMULATOR: u32 = 4u32;
pub const DIJU_GLOBALDRIVER: u32 = 2u32;
pub const DIJU_USERVALUES: u32 = 1u32;
pub const DIKEYBOARD_0: u32 = 2164261899u32;
pub const DIKEYBOARD_1: u32 = 2164261890u32;
pub const DIKEYBOARD_2: u32 = 2164261891u32;
pub const DIKEYBOARD_3: u32 = 2164261892u32;
pub const DIKEYBOARD_4: u32 = 2164261893u32;
pub const DIKEYBOARD_5: u32 = 2164261894u32;
pub const DIKEYBOARD_6: u32 = 2164261895u32;
pub const DIKEYBOARD_7: u32 = 2164261896u32;
pub const DIKEYBOARD_8: u32 = 2164261897u32;
pub const DIKEYBOARD_9: u32 = 2164261898u32;
pub const DIKEYBOARD_A: u32 = 2164261918u32;
pub const DIKEYBOARD_ABNT_C1: u32 = 2164262003u32;
pub const DIKEYBOARD_ABNT_C2: u32 = 2164262014u32;
pub const DIKEYBOARD_ADD: u32 = 2164261966u32;
pub const DIKEYBOARD_APOSTROPHE: u32 = 2164261928u32;
pub const DIKEYBOARD_APPS: u32 = 2164262109u32;
pub const DIKEYBOARD_AT: u32 = 2164262033u32;
pub const DIKEYBOARD_AX: u32 = 2164262038u32;
pub const DIKEYBOARD_B: u32 = 2164261936u32;
pub const DIKEYBOARD_BACK: u32 = 2164261902u32;
pub const DIKEYBOARD_BACKSLASH: u32 = 2164261931u32;
pub const DIKEYBOARD_C: u32 = 2164261934u32;
pub const DIKEYBOARD_CALCULATOR: u32 = 2164262049u32;
pub const DIKEYBOARD_CAPITAL: u32 = 2164261946u32;
pub const DIKEYBOARD_COLON: u32 = 2164262034u32;
pub const DIKEYBOARD_COMMA: u32 = 2164261939u32;
pub const DIKEYBOARD_CONVERT: u32 = 2164262009u32;
pub const DIKEYBOARD_D: u32 = 2164261920u32;
pub const DIKEYBOARD_DECIMAL: u32 = 2164261971u32;
pub const DIKEYBOARD_DELETE: u32 = 2164262099u32;
pub const DIKEYBOARD_DIVIDE: u32 = 2164262069u32;
pub const DIKEYBOARD_DOWN: u32 = 2164262096u32;
pub const DIKEYBOARD_E: u32 = 2164261906u32;
pub const DIKEYBOARD_END: u32 = 2164262095u32;
pub const DIKEYBOARD_EQUALS: u32 = 2164261901u32;
pub const DIKEYBOARD_ESCAPE: u32 = 2164261889u32;
pub const DIKEYBOARD_F: u32 = 2164261921u32;
pub const DIKEYBOARD_F1: u32 = 2164261947u32;
pub const DIKEYBOARD_F10: u32 = 2164261956u32;
pub const DIKEYBOARD_F11: u32 = 2164261975u32;
pub const DIKEYBOARD_F12: u32 = 2164261976u32;
pub const DIKEYBOARD_F13: u32 = 2164261988u32;
pub const DIKEYBOARD_F14: u32 = 2164261989u32;
pub const DIKEYBOARD_F15: u32 = 2164261990u32;
pub const DIKEYBOARD_F2: u32 = 2164261948u32;
pub const DIKEYBOARD_F3: u32 = 2164261949u32;
pub const DIKEYBOARD_F4: u32 = 2164261950u32;
pub const DIKEYBOARD_F5: u32 = 2164261951u32;
pub const DIKEYBOARD_F6: u32 = 2164261952u32;
pub const DIKEYBOARD_F7: u32 = 2164261953u32;
pub const DIKEYBOARD_F8: u32 = 2164261954u32;
pub const DIKEYBOARD_F9: u32 = 2164261955u32;
pub const DIKEYBOARD_G: u32 = 2164261922u32;
pub const DIKEYBOARD_GRAVE: u32 = 2164261929u32;
pub const DIKEYBOARD_H: u32 = 2164261923u32;
pub const DIKEYBOARD_HOME: u32 = 2164262087u32;
pub const DIKEYBOARD_I: u32 = 2164261911u32;
pub const DIKEYBOARD_INSERT: u32 = 2164262098u32;
pub const DIKEYBOARD_J: u32 = 2164261924u32;
pub const DIKEYBOARD_K: u32 = 2164261925u32;
pub const DIKEYBOARD_KANA: u32 = 2164262000u32;
pub const DIKEYBOARD_KANJI: u32 = 2164262036u32;
pub const DIKEYBOARD_L: u32 = 2164261926u32;
pub const DIKEYBOARD_LBRACKET: u32 = 2164261914u32;
pub const DIKEYBOARD_LCONTROL: u32 = 2164261917u32;
pub const DIKEYBOARD_LEFT: u32 = 2164262091u32;
pub const DIKEYBOARD_LMENU: u32 = 2164261944u32;
pub const DIKEYBOARD_LSHIFT: u32 = 2164261930u32;
pub const DIKEYBOARD_LWIN: u32 = 2164262107u32;
pub const DIKEYBOARD_M: u32 = 2164261938u32;
pub const DIKEYBOARD_MAIL: u32 = 2164262124u32;
pub const DIKEYBOARD_MEDIASELECT: u32 = 2164262125u32;
pub const DIKEYBOARD_MEDIASTOP: u32 = 2164262052u32;
pub const DIKEYBOARD_MINUS: u32 = 2164261900u32;
pub const DIKEYBOARD_MULTIPLY: u32 = 2164261943u32;
pub const DIKEYBOARD_MUTE: u32 = 2164262048u32;
pub const DIKEYBOARD_MYCOMPUTER: u32 = 2164262123u32;
pub const DIKEYBOARD_N: u32 = 2164261937u32;
pub const DIKEYBOARD_NEXT: u32 = 2164262097u32;
pub const DIKEYBOARD_NEXTTRACK: u32 = 2164262041u32;
pub const DIKEYBOARD_NOCONVERT: u32 = 2164262011u32;
pub const DIKEYBOARD_NUMLOCK: u32 = 2164261957u32;
pub const DIKEYBOARD_NUMPAD0: u32 = 2164261970u32;
pub const DIKEYBOARD_NUMPAD1: u32 = 2164261967u32;
pub const DIKEYBOARD_NUMPAD2: u32 = 2164261968u32;
pub const DIKEYBOARD_NUMPAD3: u32 = 2164261969u32;
pub const DIKEYBOARD_NUMPAD4: u32 = 2164261963u32;
pub const DIKEYBOARD_NUMPAD5: u32 = 2164261964u32;
pub const DIKEYBOARD_NUMPAD6: u32 = 2164261965u32;
pub const DIKEYBOARD_NUMPAD7: u32 = 2164261959u32;
pub const DIKEYBOARD_NUMPAD8: u32 = 2164261960u32;
pub const DIKEYBOARD_NUMPAD9: u32 = 2164261961u32;
pub const DIKEYBOARD_NUMPADCOMMA: u32 = 2164262067u32;
pub const DIKEYBOARD_NUMPADENTER: u32 = 2164262044u32;
pub const DIKEYBOARD_NUMPADEQUALS: u32 = 2164262029u32;
pub const DIKEYBOARD_O: u32 = 2164261912u32;
pub const DIKEYBOARD_OEM_102: u32 = 2164261974u32;
pub const DIKEYBOARD_P: u32 = 2164261913u32;
pub const DIKEYBOARD_PAUSE: u32 = 2164262085u32;
pub const DIKEYBOARD_PERIOD: u32 = 2164261940u32;
pub const DIKEYBOARD_PLAYPAUSE: u32 = 2164262050u32;
pub const DIKEYBOARD_POWER: u32 = 2164262110u32;
pub const DIKEYBOARD_PREVTRACK: u32 = 2164262032u32;
pub const DIKEYBOARD_PRIOR: u32 = 2164262089u32;
pub const DIKEYBOARD_Q: u32 = 2164261904u32;
pub const DIKEYBOARD_R: u32 = 2164261907u32;
pub const DIKEYBOARD_RBRACKET: u32 = 2164261915u32;
pub const DIKEYBOARD_RCONTROL: u32 = 2164262045u32;
pub const DIKEYBOARD_RETURN: u32 = 2164261916u32;
pub const DIKEYBOARD_RIGHT: u32 = 2164262093u32;
pub const DIKEYBOARD_RMENU: u32 = 2164262072u32;
pub const DIKEYBOARD_RSHIFT: u32 = 2164261942u32;
pub const DIKEYBOARD_RWIN: u32 = 2164262108u32;
pub const DIKEYBOARD_S: u32 = 2164261919u32;
pub const DIKEYBOARD_SCROLL: u32 = 2164261958u32;
pub const DIKEYBOARD_SEMICOLON: u32 = 2164261927u32;
pub const DIKEYBOARD_SLASH: u32 = 2164261941u32;
pub const DIKEYBOARD_SLEEP: u32 = 2164262111u32;
pub const DIKEYBOARD_SPACE: u32 = 2164261945u32;
pub const DIKEYBOARD_STOP: u32 = 2164262037u32;
pub const DIKEYBOARD_SUBTRACT: u32 = 2164261962u32;
pub const DIKEYBOARD_SYSRQ: u32 = 2164262071u32;
pub const DIKEYBOARD_T: u32 = 2164261908u32;
pub const DIKEYBOARD_TAB: u32 = 2164261903u32;
pub const DIKEYBOARD_U: u32 = 2164261910u32;
pub const DIKEYBOARD_UNDERLINE: u32 = 2164262035u32;
pub const DIKEYBOARD_UNLABELED: u32 = 2164262039u32;
pub const DIKEYBOARD_UP: u32 = 2164262088u32;
pub const DIKEYBOARD_V: u32 = 2164261935u32;
pub const DIKEYBOARD_VOLUMEDOWN: u32 = 2164262062u32;
pub const DIKEYBOARD_VOLUMEUP: u32 = 2164262064u32;
pub const DIKEYBOARD_W: u32 = 2164261905u32;
pub const DIKEYBOARD_WAKE: u32 = 2164262115u32;
pub const DIKEYBOARD_WEBBACK: u32 = 2164262122u32;
pub const DIKEYBOARD_WEBFAVORITES: u32 = 2164262118u32;
pub const DIKEYBOARD_WEBFORWARD: u32 = 2164262121u32;
pub const DIKEYBOARD_WEBHOME: u32 = 2164262066u32;
pub const DIKEYBOARD_WEBREFRESH: u32 = 2164262119u32;
pub const DIKEYBOARD_WEBSEARCH: u32 = 2164262117u32;
pub const DIKEYBOARD_WEBSTOP: u32 = 2164262120u32;
pub const DIKEYBOARD_X: u32 = 2164261933u32;
pub const DIKEYBOARD_Y: u32 = 2164261909u32;
pub const DIKEYBOARD_YEN: u32 = 2164262013u32;
pub const DIKEYBOARD_Z: u32 = 2164261932u32;
pub const DIK_0: u32 = 11u32;
pub const DIK_1: u32 = 2u32;
pub const DIK_2: u32 = 3u32;
pub const DIK_3: u32 = 4u32;
pub const DIK_4: u32 = 5u32;
pub const DIK_5: u32 = 6u32;
pub const DIK_6: u32 = 7u32;
pub const DIK_7: u32 = 8u32;
pub const DIK_8: u32 = 9u32;
pub const DIK_9: u32 = 10u32;
pub const DIK_A: u32 = 30u32;
pub const DIK_ABNT_C1: u32 = 115u32;
pub const DIK_ABNT_C2: u32 = 126u32;
pub const DIK_ADD: u32 = 78u32;
pub const DIK_APOSTROPHE: u32 = 40u32;
pub const DIK_APPS: u32 = 221u32;
pub const DIK_AT: u32 = 145u32;
pub const DIK_AX: u32 = 150u32;
pub const DIK_B: u32 = 48u32;
pub const DIK_BACK: u32 = 14u32;
pub const DIK_BACKSLASH: u32 = 43u32;
pub const DIK_BACKSPACE: u32 = 14u32;
pub const DIK_C: u32 = 46u32;
pub const DIK_CALCULATOR: u32 = 161u32;
pub const DIK_CAPITAL: u32 = 58u32;
pub const DIK_CAPSLOCK: u32 = 58u32;
pub const DIK_CIRCUMFLEX: u32 = 144u32;
pub const DIK_COLON: u32 = 146u32;
pub const DIK_COMMA: u32 = 51u32;
pub const DIK_CONVERT: u32 = 121u32;
pub const DIK_D: u32 = 32u32;
pub const DIK_DECIMAL: u32 = 83u32;
pub const DIK_DELETE: u32 = 211u32;
pub const DIK_DIVIDE: u32 = 181u32;
pub const DIK_DOWN: u32 = 208u32;
pub const DIK_DOWNARROW: u32 = 208u32;
pub const DIK_E: u32 = 18u32;
pub const DIK_END: u32 = 207u32;
pub const DIK_EQUALS: u32 = 13u32;
pub const DIK_ESCAPE: u32 = 1u32;
pub const DIK_F: u32 = 33u32;
pub const DIK_F1: u32 = 59u32;
pub const DIK_F10: u32 = 68u32;
pub const DIK_F11: u32 = 87u32;
pub const DIK_F12: u32 = 88u32;
pub const DIK_F13: u32 = 100u32;
pub const DIK_F14: u32 = 101u32;
pub const DIK_F15: u32 = 102u32;
pub const DIK_F2: u32 = 60u32;
pub const DIK_F3: u32 = 61u32;
pub const DIK_F4: u32 = 62u32;
pub const DIK_F5: u32 = 63u32;
pub const DIK_F6: u32 = 64u32;
pub const DIK_F7: u32 = 65u32;
pub const DIK_F8: u32 = 66u32;
pub const DIK_F9: u32 = 67u32;
pub const DIK_G: u32 = 34u32;
pub const DIK_GRAVE: u32 = 41u32;
pub const DIK_H: u32 = 35u32;
pub const DIK_HOME: u32 = 199u32;
pub const DIK_I: u32 = 23u32;
pub const DIK_INSERT: u32 = 210u32;
pub const DIK_J: u32 = 36u32;
pub const DIK_K: u32 = 37u32;
pub const DIK_KANA: u32 = 112u32;
pub const DIK_KANJI: u32 = 148u32;
pub const DIK_L: u32 = 38u32;
pub const DIK_LALT: u32 = 56u32;
pub const DIK_LBRACKET: u32 = 26u32;
pub const DIK_LCONTROL: u32 = 29u32;
pub const DIK_LEFT: u32 = 203u32;
pub const DIK_LEFTARROW: u32 = 203u32;
pub const DIK_LMENU: u32 = 56u32;
pub const DIK_LSHIFT: u32 = 42u32;
pub const DIK_LWIN: u32 = 219u32;
pub const DIK_M: u32 = 50u32;
pub const DIK_MAIL: u32 = 236u32;
pub const DIK_MEDIASELECT: u32 = 237u32;
pub const DIK_MEDIASTOP: u32 = 164u32;
pub const DIK_MINUS: u32 = 12u32;
pub const DIK_MULTIPLY: u32 = 55u32;
pub const DIK_MUTE: u32 = 160u32;
pub const DIK_MYCOMPUTER: u32 = 235u32;
pub const DIK_N: u32 = 49u32;
pub const DIK_NEXT: u32 = 209u32;
pub const DIK_NEXTTRACK: u32 = 153u32;
pub const DIK_NOCONVERT: u32 = 123u32;
pub const DIK_NUMLOCK: u32 = 69u32;
pub const DIK_NUMPAD0: u32 = 82u32;
pub const DIK_NUMPAD1: u32 = 79u32;
pub const DIK_NUMPAD2: u32 = 80u32;
pub const DIK_NUMPAD3: u32 = 81u32;
pub const DIK_NUMPAD4: u32 = 75u32;
pub const DIK_NUMPAD5: u32 = 76u32;
pub const DIK_NUMPAD6: u32 = 77u32;
pub const DIK_NUMPAD7: u32 = 71u32;
pub const DIK_NUMPAD8: u32 = 72u32;
pub const DIK_NUMPAD9: u32 = 73u32;
pub const DIK_NUMPADCOMMA: u32 = 179u32;
pub const DIK_NUMPADENTER: u32 = 156u32;
pub const DIK_NUMPADEQUALS: u32 = 141u32;
pub const DIK_NUMPADMINUS: u32 = 74u32;
pub const DIK_NUMPADPERIOD: u32 = 83u32;
pub const DIK_NUMPADPLUS: u32 = 78u32;
pub const DIK_NUMPADSLASH: u32 = 181u32;
pub const DIK_NUMPADSTAR: u32 = 55u32;
pub const DIK_O: u32 = 24u32;
pub const DIK_OEM_102: u32 = 86u32;
pub const DIK_P: u32 = 25u32;
pub const DIK_PAUSE: u32 = 197u32;
pub const DIK_PERIOD: u32 = 52u32;
pub const DIK_PGDN: u32 = 209u32;
pub const DIK_PGUP: u32 = 201u32;
pub const DIK_PLAYPAUSE: u32 = 162u32;
pub const DIK_POWER: u32 = 222u32;
pub const DIK_PREVTRACK: u32 = 144u32;
pub const DIK_PRIOR: u32 = 201u32;
pub const DIK_Q: u32 = 16u32;
pub const DIK_R: u32 = 19u32;
pub const DIK_RALT: u32 = 184u32;
pub const DIK_RBRACKET: u32 = 27u32;
pub const DIK_RCONTROL: u32 = 157u32;
pub const DIK_RETURN: u32 = 28u32;
pub const DIK_RIGHT: u32 = 205u32;
pub const DIK_RIGHTARROW: u32 = 205u32;
pub const DIK_RMENU: u32 = 184u32;
pub const DIK_RSHIFT: u32 = 54u32;
pub const DIK_RWIN: u32 = 220u32;
pub const DIK_S: u32 = 31u32;
pub const DIK_SCROLL: u32 = 70u32;
pub const DIK_SEMICOLON: u32 = 39u32;
pub const DIK_SLASH: u32 = 53u32;
pub const DIK_SLEEP: u32 = 223u32;
pub const DIK_SPACE: u32 = 57u32;
pub const DIK_STOP: u32 = 149u32;
pub const DIK_SUBTRACT: u32 = 74u32;
pub const DIK_SYSRQ: u32 = 183u32;
pub const DIK_T: u32 = 20u32;
pub const DIK_TAB: u32 = 15u32;
pub const DIK_U: u32 = 22u32;
pub const DIK_UNDERLINE: u32 = 147u32;
pub const DIK_UNLABELED: u32 = 151u32;
pub const DIK_UP: u32 = 200u32;
pub const DIK_UPARROW: u32 = 200u32;
pub const DIK_V: u32 = 47u32;
pub const DIK_VOLUMEDOWN: u32 = 174u32;
pub const DIK_VOLUMEUP: u32 = 176u32;
pub const DIK_W: u32 = 17u32;
pub const DIK_WAKE: u32 = 227u32;
pub const DIK_WEBBACK: u32 = 234u32;
pub const DIK_WEBFAVORITES: u32 = 230u32;
pub const DIK_WEBFORWARD: u32 = 233u32;
pub const DIK_WEBHOME: u32 = 178u32;
pub const DIK_WEBREFRESH: u32 = 231u32;
pub const DIK_WEBSEARCH: u32 = 229u32;
pub const DIK_WEBSTOP: u32 = 232u32;
pub const DIK_X: u32 = 45u32;
pub const DIK_Y: u32 = 21u32;
pub const DIK_YEN: u32 = 125u32;
pub const DIK_Z: u32 = 44u32;
pub const DIMSGWP_DX8APPSTART: u32 = 2u32;
pub const DIMSGWP_DX8MAPPERAPPSTART: u32 = 3u32;
pub const DIMSGWP_NEWAPPSTART: u32 = 1u32;
pub const DIPH_BYID: u32 = 2u32;
pub const DIPH_BYOFFSET: u32 = 1u32;
pub const DIPH_BYUSAGE: u32 = 3u32;
pub const DIPH_DEVICE: u32 = 0u32;
pub const DIPOV_ANY_1: u32 = 4278208001u32;
pub const DIPOV_ANY_2: u32 = 4278208002u32;
pub const DIPOV_ANY_3: u32 = 4278208003u32;
pub const DIPOV_ANY_4: u32 = 4278208004u32;
pub const DIPROPAUTOCENTER_OFF: u32 = 0u32;
pub const DIPROPAUTOCENTER_ON: u32 = 1u32;
pub const DIPROPAXISMODE_ABS: u32 = 0u32;
pub const DIPROPAXISMODE_REL: u32 = 1u32;
pub const DIPROPCALIBRATIONMODE_COOKED: u32 = 0u32;
pub const DIPROPCALIBRATIONMODE_RAW: u32 = 1u32;
pub const DIPROP_APPDATA: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000016);
pub const DIPROP_AUTOCENTER: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000009);
pub const DIPROP_AXISMODE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000002);
pub const DIPROP_BUFFERSIZE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000001);
pub const DIPROP_CALIBRATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000b);
pub const DIPROP_CALIBRATIONMODE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000a);
pub const DIPROP_CPOINTS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000015);
pub const DIPROP_DEADZONE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000005);
pub const DIPROP_FFGAIN: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000007);
pub const DIPROP_FFLOAD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000008);
pub const DIPROP_GETPORTDISPLAYNAME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000010);
pub const DIPROP_GRANULARITY: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000003);
pub const DIPROP_GUIDANDPATH: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000c);
pub const DIPROP_INSTANCENAME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000d);
pub const DIPROP_JOYSTICKID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000f);
pub const DIPROP_KEYNAME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000014);
pub const DIPROP_LOGICALRANGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000013);
pub const DIPROP_PHYSICALRANGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000012);
pub const DIPROP_PRODUCTNAME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000e);
pub const DIPROP_RANGE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000004);
pub const DIPROP_SATURATION: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000006);
pub const DIPROP_SCANCODE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000017);
pub const DIPROP_TYPENAME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000001a);
pub const DIPROP_USERNAME: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000019);
pub const DIPROP_VIDPID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000018);
pub const DIRECTINPUT_HEADER_VERSION: u32 = 2048u32;
pub const DIRECTINPUT_NOTIFICATION_MSGSTRING: ::windows_core::PCWSTR = ::windows_core::w!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_NOTIFICATION_MSGSTRINGA: ::windows_core::PCSTR = ::windows_core::s!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_NOTIFICATION_MSGSTRINGW: ::windows_core::PCWSTR = ::windows_core::w!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPP: ::windows_core::PCWSTR = ::windows_core::w!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPPA: ::windows_core::PCSTR = ::windows_core::s!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPPW: ::windows_core::PCWSTR = ::windows_core::w!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPP: ::windows_core::PCWSTR = ::windows_core::w!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPPA: ::windows_core::PCSTR = ::windows_core::s!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPPW: ::windows_core::PCWSTR = ::windows_core::w!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAG: ::windows_core::PCWSTR = ::windows_core::w!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAGA: ::windows_core::PCSTR = ::windows_core::s!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAGW: ::windows_core::PCWSTR = ::windows_core::w!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_ID: ::windows_core::PCWSTR = ::windows_core::w!("Id");
pub const DIRECTINPUT_REGSTR_VAL_IDA: ::windows_core::PCSTR = ::windows_core::s!("Id");
pub const DIRECTINPUT_REGSTR_VAL_IDW: ::windows_core::PCWSTR = ::windows_core::w!("Id");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTART: ::windows_core::PCWSTR = ::windows_core::w!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTARTA: ::windows_core::PCSTR = ::windows_core::s!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTARTW: ::windows_core::PCWSTR = ::windows_core::w!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_MAPPER: ::windows_core::PCWSTR = ::windows_core::w!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_MAPPERA: ::windows_core::PCSTR = ::windows_core::s!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_MAPPERW: ::windows_core::PCWSTR = ::windows_core::w!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_NAME: ::windows_core::PCWSTR = ::windows_core::w!("Name");
pub const DIRECTINPUT_REGSTR_VAL_NAMEA: ::windows_core::PCSTR = ::windows_core::s!("Name");
pub const DIRECTINPUT_REGSTR_VAL_NAMEW: ::windows_core::PCWSTR = ::windows_core::w!("Name");
pub const DIRECTINPUT_REGSTR_VAL_VERSION: ::windows_core::PCWSTR = ::windows_core::w!("Version");
pub const DIRECTINPUT_REGSTR_VAL_VERSIONA: ::windows_core::PCSTR = ::windows_core::s!("Version");
pub const DIRECTINPUT_REGSTR_VAL_VERSIONW: ::windows_core::PCWSTR = ::windows_core::w!("Version");
pub const DIRECTINPUT_VERSION: u32 = 2048u32;
pub const DISCL_BACKGROUND: u32 = 8u32;
pub const DISCL_EXCLUSIVE: u32 = 1u32;
pub const DISCL_FOREGROUND: u32 = 4u32;
pub const DISCL_NONEXCLUSIVE: u32 = 2u32;
pub const DISCL_NOWINKEY: u32 = 16u32;
pub const DISDD_CONTINUE: u32 = 1u32;
pub const DISFFC_CONTINUE: u32 = 8u32;
pub const DISFFC_PAUSE: u32 = 4u32;
pub const DISFFC_RESET: u32 = 1u32;
pub const DISFFC_SETACTUATORSOFF: u32 = 32u32;
pub const DISFFC_SETACTUATORSON: u32 = 16u32;
pub const DISFFC_STOPALL: u32 = 2u32;
pub const DITC_CALLOUT: u32 = 8u32;
pub const DITC_CLSIDCONFIG: u32 = 2u32;
pub const DITC_DISPLAYNAME: u32 = 4u32;
pub const DITC_FLAGS1: u32 = 32u32;
pub const DITC_FLAGS2: u32 = 64u32;
pub const DITC_HARDWAREID: u32 = 16u32;
pub const DITC_MAPFILE: u32 = 128u32;
pub const DITC_REGHWSETTINGS: u32 = 1u32;
pub const DIVIRTUAL_ARCADE_PLATFORM: u32 = 570425344u32;
pub const DIVIRTUAL_ARCADE_SIDE2SIDE: u32 = 553648128u32;
pub const DIVIRTUAL_BROWSER_CONTROL: u32 = 671088640u32;
pub const DIVIRTUAL_CAD_2DCONTROL: u32 = 587202560u32;
pub const DIVIRTUAL_CAD_3DCONTROL: u32 = 603979776u32;
pub const DIVIRTUAL_CAD_FLYBY: u32 = 620756992u32;
pub const DIVIRTUAL_CAD_MODEL: u32 = 637534208u32;
pub const DIVIRTUAL_DRIVING_COMBAT: u32 = 33554432u32;
pub const DIVIRTUAL_DRIVING_MECHA: u32 = 687865856u32;
pub const DIVIRTUAL_DRIVING_RACE: u32 = 16777216u32;
pub const DIVIRTUAL_DRIVING_TANK: u32 = 50331648u32;
pub const DIVIRTUAL_FIGHTING_FPS: u32 = 150994944u32;
pub const DIVIRTUAL_FIGHTING_HAND2HAND: u32 = 134217728u32;
pub const DIVIRTUAL_FIGHTING_THIRDPERSON: u32 = 167772160u32;
pub const DIVIRTUAL_FLYING_CIVILIAN: u32 = 67108864u32;
pub const DIVIRTUAL_FLYING_HELICOPTER: u32 = 100663296u32;
pub const DIVIRTUAL_FLYING_MILITARY: u32 = 83886080u32;
pub const DIVIRTUAL_REMOTE_CONTROL: u32 = 654311424u32;
pub const DIVIRTUAL_SPACESIM: u32 = 117440512u32;
pub const DIVIRTUAL_SPORTS_BASEBALL_BAT: u32 = 251658240u32;
pub const DIVIRTUAL_SPORTS_BASEBALL_FIELD: u32 = 285212672u32;
pub const DIVIRTUAL_SPORTS_BASEBALL_PITCH: u32 = 268435456u32;
pub const DIVIRTUAL_SPORTS_BASKETBALL_DEFENSE: u32 = 318767104u32;
pub const DIVIRTUAL_SPORTS_BASKETBALL_OFFENSE: u32 = 301989888u32;
pub const DIVIRTUAL_SPORTS_BIKING_MOUNTAIN: u32 = 469762048u32;
pub const DIVIRTUAL_SPORTS_FISHING: u32 = 234881024u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_DEFENSE: u32 = 385875968u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_FIELD: u32 = 335544320u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_OFFENSE: u32 = 369098752u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_QBCK: u32 = 352321536u32;
pub const DIVIRTUAL_SPORTS_GOLF: u32 = 402653184u32;
pub const DIVIRTUAL_SPORTS_HOCKEY_DEFENSE: u32 = 436207616u32;
pub const DIVIRTUAL_SPORTS_HOCKEY_GOALIE: u32 = 452984832u32;
pub const DIVIRTUAL_SPORTS_HOCKEY_OFFENSE: u32 = 419430400u32;
pub const DIVIRTUAL_SPORTS_HUNTING: u32 = 218103808u32;
pub const DIVIRTUAL_SPORTS_RACQUET: u32 = 536870912u32;
pub const DIVIRTUAL_SPORTS_SKIING: u32 = 486539264u32;
pub const DIVIRTUAL_SPORTS_SOCCER_DEFENSE: u32 = 520093696u32;
pub const DIVIRTUAL_SPORTS_SOCCER_OFFENSE: u32 = 503316480u32;
pub const DIVIRTUAL_STRATEGY_ROLEPLAYING: u32 = 184549376u32;
pub const DIVIRTUAL_STRATEGY_TURN: u32 = 201326592u32;
pub const DIVOICE_ALL: u32 = 2197816330u32;
pub const DIVOICE_CHANNEL1: u32 = 2197816321u32;
pub const DIVOICE_CHANNEL2: u32 = 2197816322u32;
pub const DIVOICE_CHANNEL3: u32 = 2197816323u32;
pub const DIVOICE_CHANNEL4: u32 = 2197816324u32;
pub const DIVOICE_CHANNEL5: u32 = 2197816325u32;
pub const DIVOICE_CHANNEL6: u32 = 2197816326u32;
pub const DIVOICE_CHANNEL7: u32 = 2197816327u32;
pub const DIVOICE_CHANNEL8: u32 = 2197816328u32;
pub const DIVOICE_PLAYBACKMUTE: u32 = 2197816332u32;
pub const DIVOICE_RECORDMUTE: u32 = 2197816331u32;
pub const DIVOICE_TEAM: u32 = 2197816329u32;
pub const DIVOICE_TRANSMIT: u32 = 2197816333u32;
pub const DIVOICE_VOICECOMMAND: u32 = 2197816336u32;
pub const DI_BUFFEROVERFLOW: i32 = 1i32;
pub const DI_DEGREES: u32 = 100u32;
pub const DI_DOWNLOADSKIPPED: ::windows_core::HRESULT = ::windows_core::HRESULT(3i32);
pub const DI_EFFECTRESTARTED: ::windows_core::HRESULT = ::windows_core::HRESULT(4i32);
pub const DI_FFNOMINALMAX: u32 = 10000u32;
pub const DI_NOEFFECT: i32 = 1i32;
pub const DI_NOTATTACHED: i32 = 1i32;
pub const DI_OK: i32 = 0i32;
pub const DI_POLLEDDEVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(2i32);
pub const DI_PROPNOEFFECT: i32 = 1i32;
pub const DI_SECONDS: u32 = 1000000u32;
pub const DI_SETTINGSNOTSAVED: ::windows_core::HRESULT = ::windows_core::HRESULT(11i32);
pub const DI_TRUNCATED: ::windows_core::HRESULT = ::windows_core::HRESULT(8i32);
pub const DI_TRUNCATEDANDRESTARTED: ::windows_core::HRESULT = ::windows_core::HRESULT(12i32);
pub const DI_WRITEPROTECT: ::windows_core::HRESULT = ::windows_core::HRESULT(19i32);
pub const GPIO_BUTTON_BACK: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(5i32);
pub const GPIO_BUTTON_CAMERA_FOCUS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(7i32);
pub const GPIO_BUTTON_CAMERA_LENS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(12i32);
pub const GPIO_BUTTON_CAMERA_SHUTTER: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(8i32);
pub const GPIO_BUTTON_COUNT: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(16i32);
pub const GPIO_BUTTON_COUNT_MIN: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(5i32);
pub const GPIO_BUTTON_HEADSET: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(10i32);
pub const GPIO_BUTTON_HWKB_DEPLOY: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(11i32);
pub const GPIO_BUTTON_OEM_CUSTOM: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(13i32);
pub const GPIO_BUTTON_OEM_CUSTOM2: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(14i32);
pub const GPIO_BUTTON_OEM_CUSTOM3: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(15i32);
pub const GPIO_BUTTON_POWER: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(0i32);
pub const GPIO_BUTTON_RINGER_TOGGLE: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(9i32);
pub const GPIO_BUTTON_ROTATION_LOCK: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(4i32);
pub const GPIO_BUTTON_SEARCH: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(6i32);
pub const GPIO_BUTTON_VOLUME_DOWN: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(3i32);
pub const GPIO_BUTTON_VOLUME_UP: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(2i32);
pub const GPIO_BUTTON_WINDOWS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(1i32);
pub const GUID_Button: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02f0_c9f3_11cf_bfc7_444553540000);
pub const GUID_ConstantForce: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c20_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_CustomForce: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c2b_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_DEVINTERFACE_HID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d1e55b2_f16f_11cf_88cb_001111000030);
pub const GUID_DEVINTERFACE_KEYBOARD: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x884b96c3_56ef_11d1_bc8c_00a0c91405dd);
pub const GUID_DEVINTERFACE_MOUSE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x378de44c_56ef_11d1_bc8c_00a0c91405dd);
pub const GUID_Damper: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c28_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Friction: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c2a_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_HIDClass: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x745a17a0_74d3_11d0_b6fe_00a0c90f57da);
pub const GUID_HID_INTERFACE_HIDPARSE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5c315a5_69ac_4bc2_9279_d0b64576f44b);
pub const GUID_HID_INTERFACE_NOTIFY: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c4e2e88_25e6_4c33_882f_3d82e6073681);
pub const GUID_Inertia: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c29_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Joystick: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1d2b70_d5a0_11cf_bfc7_444553540000);
pub const GUID_Key: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55728220_d33c_11cf_bfc7_444553540000);
pub const GUID_KeyboardClass: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d36e96b_e325_11ce_bfc1_08002be10318);
pub const GUID_MediaClass: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d36e96c_e325_11ce_bfc1_08002be10318);
pub const GUID_MouseClass: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d36e96f_e325_11ce_bfc1_08002be10318);
pub const GUID_POV: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02f2_c9f3_11cf_bfc7_444553540000);
pub const GUID_RampForce: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c21_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_RxAxis: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02f4_c9f3_11cf_bfc7_444553540000);
pub const GUID_RyAxis: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02f5_c9f3_11cf_bfc7_444553540000);
pub const GUID_RzAxis: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02e3_c9f3_11cf_bfc7_444553540000);
pub const GUID_SawtoothDown: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c26_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_SawtoothUp: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c25_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Sine: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c23_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Slider: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02e4_c9f3_11cf_bfc7_444553540000);
pub const GUID_Spring: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c27_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Square: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c22_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_SysKeyboard: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1d2b61_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysKeyboardEm: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1d2b82_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysKeyboardEm2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1d2b83_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouse: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1d2b60_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouseEm: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1d2b80_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouseEm2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f1d2b81_d5a0_11cf_bfc7_444553540000);
pub const GUID_Triangle: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13541c24_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Unknown: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02f3_c9f3_11cf_bfc7_444553540000);
pub const GUID_XAxis: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02e0_c9f3_11cf_bfc7_444553540000);
pub const GUID_YAxis: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02e1_c9f3_11cf_bfc7_444553540000);
pub const GUID_ZAxis: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36d02e2_c9f3_11cf_bfc7_444553540000);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627706i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_BUFFER_TOO_SMALL: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627705i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627697i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627699i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627698i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627703i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_I8242_TRANS_UNKNOWN: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627703i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627702i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INTERNAL_ERROR: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627704i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627711i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627709i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INVALID_REPORT_TYPE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627710i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_IS_VALUE_ARRAY: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627700i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NOT_BUTTON_ARRAY: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627679i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NOT_IMPLEMENTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627680i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NOT_VALUE_ARRAY: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627701i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NULL: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-2146369535i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627696i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_SUCCESS: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(1114112i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_USAGE_NOT_FOUND: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627708i32);
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(-1072627707i32);
pub const HID_REVISION: u32 = 1u32;
pub const HID_USAGE_ALPHANUMERIC_14_SEGMENT_DIRECT_MAP: u16 = 69u16;
pub const HID_USAGE_ALPHANUMERIC_7_SEGMENT_DIRECT_MAP: u16 = 67u16;
pub const HID_USAGE_ALPHANUMERIC_ALPHANUMERIC_DISPLAY: u16 = 1u16;
pub const HID_USAGE_ALPHANUMERIC_ASCII_CHARACTER_SET: u16 = 33u16;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_DATA: u16 = 74u16;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_READBACK: u16 = 73u16;
pub const HID_USAGE_ALPHANUMERIC_BITMAPPED_DISPLAY: u16 = 2u16;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_X: u16 = 128u16;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_Y: u16 = 129u16;
pub const HID_USAGE_ALPHANUMERIC_BIT_DEPTH_FORMAT: u16 = 131u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_DATA: u16 = 143u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X1: u16 = 139u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X2: u16 = 141u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y1: u16 = 140u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y2: u16 = 142u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_REPORT: u16 = 138u16;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_ATTRIBUTE: u16 = 72u16;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_REPORT: u16 = 43u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_BLINK: u16 = 77u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_ENHANCE: u16 = 75u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_UNDERLINE: u16 = 76u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_HEIGHT: u16 = 62u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_HORIZONTAL: u16 = 63u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_VERTICAL: u16 = 64u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_WIDTH: u16 = 61u16;
pub const HID_USAGE_ALPHANUMERIC_CLEAR_DISPLAY: u16 = 37u16;
pub const HID_USAGE_ALPHANUMERIC_COLUMN: u16 = 52u16;
pub const HID_USAGE_ALPHANUMERIC_COLUMNS: u16 = 54u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_BLINK: u16 = 58u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_ENABLE: u16 = 57u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_MODE: u16 = 56u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_PIXEL_POSITIONING: u16 = 55u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_POSITION_REPORT: u16 = 50u16;
pub const HID_USAGE_ALPHANUMERIC_DATA_READ_BACK: u16 = 34u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ATTRIBUTES_REPORT: u16 = 32u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_BRIGHTNESS: u16 = 70u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTRAST: u16 = 71u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTROL_REPORT: u16 = 36u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_DATA: u16 = 44u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ENABLE: u16 = 38u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ORIENTATION: u16 = 132u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_STATUS: u16 = 45u16;
pub const HID_USAGE_ALPHANUMERIC_ERR_FONT_DATA_CANNOT_BE_READ: u16 = 49u16;
pub const HID_USAGE_ALPHANUMERIC_ERR_NOT_A_LOADABLE_CHARACTER: u16 = 48u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_14_SEGMENT: u16 = 68u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_7_SEGMENT: u16 = 66u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_DATA: u16 = 60u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_READ_BACK: u16 = 35u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_REPORT: u16 = 59u16;
pub const HID_USAGE_ALPHANUMERIC_HORIZONTAL_SCROLL: u16 = 42u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA: u16 = 136u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_OFFSET: u16 = 135u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_SIZE: u16 = 134u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_REPORT: u16 = 133u16;
pub const HID_USAGE_ALPHANUMERIC_ROW: u16 = 51u16;
pub const HID_USAGE_ALPHANUMERIC_ROWS: u16 = 53u16;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_DELAY: u16 = 39u16;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_ENABLE: u16 = 40u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON: u16 = 144u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_ID: u16 = 145u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET1: u16 = 147u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET2: u16 = 148u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_REPORT: u16 = 149u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_SIDE: u16 = 146u16;
pub const HID_USAGE_ALPHANUMERIC_STATUS_NOT_READY: u16 = 46u16;
pub const HID_USAGE_ALPHANUMERIC_STATUS_READY: u16 = 47u16;
pub const HID_USAGE_ALPHANUMERIC_UNICODE_CHAR_SET: u16 = 65u16;
pub const HID_USAGE_ALPHANUMERIC_VERTICAL_SCROLL: u16 = 41u16;
pub const HID_USAGE_CAMERA_AUTO_FOCUS: u16 = 32u16;
pub const HID_USAGE_CAMERA_SHUTTER: u16 = 33u16;
pub const HID_USAGE_CONSUMERCTRL: u16 = 1u16;
pub const HID_USAGE_CONSUMER_AC_BACK: u16 = 548u16;
pub const HID_USAGE_CONSUMER_AC_BOOKMARKS: u16 = 554u16;
pub const HID_USAGE_CONSUMER_AC_FORWARD: u16 = 549u16;
pub const HID_USAGE_CONSUMER_AC_GOTO: u16 = 546u16;
pub const HID_USAGE_CONSUMER_AC_HOME: u16 = 547u16;
pub const HID_USAGE_CONSUMER_AC_NEXT: u16 = 553u16;
pub const HID_USAGE_CONSUMER_AC_PAN: u16 = 568u16;
pub const HID_USAGE_CONSUMER_AC_PREVIOUS: u16 = 552u16;
pub const HID_USAGE_CONSUMER_AC_REFRESH: u16 = 551u16;
pub const HID_USAGE_CONSUMER_AC_SEARCH: u16 = 545u16;
pub const HID_USAGE_CONSUMER_AC_STOP: u16 = 550u16;
pub const HID_USAGE_CONSUMER_AL_BROWSER: u16 = 404u16;
pub const HID_USAGE_CONSUMER_AL_CALCULATOR: u16 = 402u16;
pub const HID_USAGE_CONSUMER_AL_CONFIGURATION: u16 = 387u16;
pub const HID_USAGE_CONSUMER_AL_EMAIL: u16 = 394u16;
pub const HID_USAGE_CONSUMER_AL_SEARCH: u16 = 454u16;
pub const HID_USAGE_CONSUMER_BALANCE: u16 = 225u16;
pub const HID_USAGE_CONSUMER_BASS: u16 = 227u16;
pub const HID_USAGE_CONSUMER_BASS_BOOST: u16 = 229u16;
pub const HID_USAGE_CONSUMER_BASS_DECREMENT: u16 = 339u16;
pub const HID_USAGE_CONSUMER_BASS_INCREMENT: u16 = 338u16;
pub const HID_USAGE_CONSUMER_CHANNEL_DECREMENT: u16 = 157u16;
pub const HID_USAGE_CONSUMER_CHANNEL_INCREMENT: u16 = 156u16;
pub const HID_USAGE_CONSUMER_EXTENDED_KEYBOARD_ATTRIBUTES_COLLECTION: u16 = 704u16;
pub const HID_USAGE_CONSUMER_FAST_FORWARD: u16 = 179u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_OPEN_GAMEBAR: u16 = 208u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_RECORD_CLIP: u16 = 210u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_SCREENSHOT: u16 = 211u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_BROADCAST: u16 = 215u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_CAMERA: u16 = 214u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_INDICATOR: u16 = 212u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_MICROPHONE: u16 = 213u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_RECORD: u16 = 209u16;
pub const HID_USAGE_CONSUMER_IMPLEMENTED_KEYBOARD_INPUT_ASSIST_CONTROLS: u16 = 710u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_FORM_FACTOR: u16 = 705u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_IETF_LANGUAGE_TAG_INDEX: u16 = 709u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_KEY_TYPE: u16 = 706u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_PHYSICAL_LAYOUT: u16 = 707u16;
pub const HID_USAGE_CONSUMER_LOUDNESS: u16 = 231u16;
pub const HID_USAGE_CONSUMER_MPX: u16 = 232u16;
pub const HID_USAGE_CONSUMER_MUTE: u16 = 226u16;
pub const HID_USAGE_CONSUMER_PAUSE: u16 = 177u16;
pub const HID_USAGE_CONSUMER_PLAY: u16 = 176u16;
pub const HID_USAGE_CONSUMER_PLAY_PAUSE: u16 = 205u16;
pub const HID_USAGE_CONSUMER_RECORD: u16 = 178u16;
pub const HID_USAGE_CONSUMER_REWIND: u16 = 180u16;
pub const HID_USAGE_CONSUMER_SCAN_NEXT_TRACK: u16 = 181u16;
pub const HID_USAGE_CONSUMER_SCAN_PREV_TRACK: u16 = 182u16;
pub const HID_USAGE_CONSUMER_STOP: u16 = 183u16;
pub const HID_USAGE_CONSUMER_SURROUND_MODE: u16 = 230u16;
pub const HID_USAGE_CONSUMER_TREBLE: u16 = 228u16;
pub const HID_USAGE_CONSUMER_TREBLE_DECREMENT: u16 = 341u16;
pub const HID_USAGE_CONSUMER_TREBLE_INCREMENT: u16 = 340u16;
pub const HID_USAGE_CONSUMER_VENDOR_SPECIFIC_KEYBOARD_PHYSICAL_LAYOUT: u16 = 708u16;
pub const HID_USAGE_CONSUMER_VOLUME: u16 = 224u16;
pub const HID_USAGE_CONSUMER_VOLUME_DECREMENT: u16 = 234u16;
pub const HID_USAGE_CONSUMER_VOLUME_INCREMENT: u16 = 233u16;
pub const HID_USAGE_DIGITIZER_3D_DIGITIZER: u16 = 8u16;
pub const HID_USAGE_DIGITIZER_ALTITUDE: u16 = 64u16;
pub const HID_USAGE_DIGITIZER_ARMATURE: u16 = 11u16;
pub const HID_USAGE_DIGITIZER_ARTICULATED_ARM: u16 = 10u16;
pub const HID_USAGE_DIGITIZER_AZIMUTH: u16 = 63u16;
pub const HID_USAGE_DIGITIZER_BARREL_PRESSURE: u16 = 49u16;
pub const HID_USAGE_DIGITIZER_BARREL_SWITCH: u16 = 68u16;
pub const HID_USAGE_DIGITIZER_BATTERY_STRENGTH: u16 = 59u16;
pub const HID_USAGE_DIGITIZER_COORD_MEASURING: u16 = 7u16;
pub const HID_USAGE_DIGITIZER_DATA_VALID: u16 = 55u16;
pub const HID_USAGE_DIGITIZER_DIGITIZER: u16 = 1u16;
pub const HID_USAGE_DIGITIZER_ERASER: u16 = 69u16;
pub const HID_USAGE_DIGITIZER_FINGER: u16 = 34u16;
pub const HID_USAGE_DIGITIZER_FREE_SPACE_WAND: u16 = 13u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP: u16 = 15u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_FRAME_DATA: u16 = 108u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VENDOR_ID: u16 = 106u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VERSION: u16 = 107u16;
pub const HID_USAGE_DIGITIZER_INVERT: u16 = 60u16;
pub const HID_USAGE_DIGITIZER_IN_RANGE: u16 = 50u16;
pub const HID_USAGE_DIGITIZER_LIGHT_PEN: u16 = 3u16;
pub const HID_USAGE_DIGITIZER_MULTI_POINT: u16 = 12u16;
pub const HID_USAGE_DIGITIZER_PEN: u16 = 2u16;
pub const HID_USAGE_DIGITIZER_PROG_CHANGE_KEYS: u16 = 58u16;
pub const HID_USAGE_DIGITIZER_PUCK: u16 = 33u16;
pub const HID_USAGE_DIGITIZER_QUALITY: u16 = 54u16;
pub const HID_USAGE_DIGITIZER_SECONDARY_TIP_SWITCH: u16 = 67u16;
pub const HID_USAGE_DIGITIZER_STEREO_PLOTTER: u16 = 9u16;
pub const HID_USAGE_DIGITIZER_STYLUS: u16 = 32u16;
pub const HID_USAGE_DIGITIZER_TABLET_FUNC_KEYS: u16 = 57u16;
pub const HID_USAGE_DIGITIZER_TABLET_PICK: u16 = 70u16;
pub const HID_USAGE_DIGITIZER_TAP: u16 = 53u16;
pub const HID_USAGE_DIGITIZER_TIP_PRESSURE: u16 = 48u16;
pub const HID_USAGE_DIGITIZER_TIP_SWITCH: u16 = 66u16;
pub const HID_USAGE_DIGITIZER_TOUCH: u16 = 51u16;
pub const HID_USAGE_DIGITIZER_TOUCH_PAD: u16 = 5u16;
pub const HID_USAGE_DIGITIZER_TOUCH_SCREEN: u16 = 4u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_CONNECTED: u16 = 162u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_INDEX: u16 = 56u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_PRODUCT: u16 = 146u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL: u16 = 91u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL_PART2: u16 = 110u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_VENDOR: u16 = 145u16;
pub const HID_USAGE_DIGITIZER_TWIST: u16 = 65u16;
pub const HID_USAGE_DIGITIZER_UNTOUCH: u16 = 52u16;
pub const HID_USAGE_DIGITIZER_WHITE_BOARD: u16 = 6u16;
pub const HID_USAGE_DIGITIZER_X_TILT: u16 = 61u16;
pub const HID_USAGE_DIGITIZER_Y_TILT: u16 = 62u16;
pub const HID_USAGE_GAME_3D_GAME_CONTROLLER: u16 = 1u16;
pub const HID_USAGE_GAME_BUMP: u16 = 44u16;
pub const HID_USAGE_GAME_FLIPPER: u16 = 42u16;
pub const HID_USAGE_GAME_GAMEPAD_FIRE_JUMP: u16 = 55u16;
pub const HID_USAGE_GAME_GAMEPAD_TRIGGER: u16 = 57u16;
pub const HID_USAGE_GAME_GUN_AUTOMATIC: u16 = 53u16;
pub const HID_USAGE_GAME_GUN_BOLT: u16 = 48u16;
pub const HID_USAGE_GAME_GUN_BURST: u16 = 52u16;
pub const HID_USAGE_GAME_GUN_CLIP: u16 = 49u16;
pub const HID_USAGE_GAME_GUN_DEVICE: u16 = 3u16;
pub const HID_USAGE_GAME_GUN_SAFETY: u16 = 54u16;
pub const HID_USAGE_GAME_GUN_SELECTOR: u16 = 50u16;
pub const HID_USAGE_GAME_GUN_SINGLE_SHOT: u16 = 51u16;
pub const HID_USAGE_GAME_LEAN_FORWARD_BACK: u16 = 40u16;
pub const HID_USAGE_GAME_LEAN_RIGHT_LEFT: u16 = 39u16;
pub const HID_USAGE_GAME_MOVE_FORWARD_BACK: u16 = 37u16;
pub const HID_USAGE_GAME_MOVE_RIGHT_LEFT: u16 = 36u16;
pub const HID_USAGE_GAME_MOVE_UP_DOWN: u16 = 38u16;
pub const HID_USAGE_GAME_NEW_GAME: u16 = 45u16;
pub const HID_USAGE_GAME_PINBALL_DEVICE: u16 = 2u16;
pub const HID_USAGE_GAME_PITCH_FORWARD_BACK: u16 = 34u16;
pub const HID_USAGE_GAME_PLAYER: u16 = 47u16;
pub const HID_USAGE_GAME_POINT_OF_VIEW: u16 = 32u16;
pub const HID_USAGE_GAME_POV_HEIGHT: u16 = 41u16;
pub const HID_USAGE_GAME_ROLL_RIGHT_LEFT: u16 = 35u16;
pub const HID_USAGE_GAME_SECONDARY_FLIPPER: u16 = 43u16;
pub const HID_USAGE_GAME_SHOOT_BALL: u16 = 46u16;
pub const HID_USAGE_GAME_TURN_RIGHT_LEFT: u16 = 33u16;
pub const HID_USAGE_GENERIC_BYTE_COUNT: u16 = 59u16;
pub const HID_USAGE_GENERIC_CONTROL_ENABLE: u16 = 203u16;
pub const HID_USAGE_GENERIC_COUNTED_BUFFER: u16 = 58u16;
pub const HID_USAGE_GENERIC_DEVICE_BATTERY_STRENGTH: u16 = 32u16;
pub const HID_USAGE_GENERIC_DEVICE_DISCOVER_WIRELESS_CONTROL: u16 = 35u16;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ENTERED: u16 = 36u16;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ERASED: u16 = 37u16;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CLEARED: u16 = 38u16;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_CHANNEL: u16 = 33u16;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_ID: u16 = 34u16;
pub const HID_USAGE_GENERIC_DIAL: u16 = 55u16;
pub const HID_USAGE_GENERIC_DPAD_DOWN: u16 = 145u16;
pub const HID_USAGE_GENERIC_DPAD_LEFT: u16 = 147u16;
pub const HID_USAGE_GENERIC_DPAD_RIGHT: u16 = 146u16;
pub const HID_USAGE_GENERIC_DPAD_UP: u16 = 144u16;
pub const HID_USAGE_GENERIC_FEATURE_NOTIFICATION: u16 = 71u16;
pub const HID_USAGE_GENERIC_GAMEPAD: u16 = 5u16;
pub const HID_USAGE_GENERIC_HATSWITCH: u16 = 57u16;
pub const HID_USAGE_GENERIC_INTERACTIVE_CONTROL: u16 = 14u16;
pub const HID_USAGE_GENERIC_JOYSTICK: u16 = 4u16;
pub const HID_USAGE_GENERIC_KEYBOARD: u16 = 6u16;
pub const HID_USAGE_GENERIC_KEYPAD: u16 = 7u16;
pub const HID_USAGE_GENERIC_MOTION_WAKEUP: u16 = 60u16;
pub const HID_USAGE_GENERIC_MOUSE: u16 = 2u16;
pub const HID_USAGE_GENERIC_MULTI_AXIS_CONTROLLER: u16 = 8u16;
pub const HID_USAGE_GENERIC_POINTER: u16 = 1u16;
pub const HID_USAGE_GENERIC_PORTABLE_DEVICE_CONTROL: u16 = 13u16;
pub const HID_USAGE_GENERIC_RESOLUTION_MULTIPLIER: u16 = 72u16;
pub const HID_USAGE_GENERIC_RX: u16 = 51u16;
pub const HID_USAGE_GENERIC_RY: u16 = 52u16;
pub const HID_USAGE_GENERIC_RZ: u16 = 53u16;
pub const HID_USAGE_GENERIC_SELECT: u16 = 62u16;
pub const HID_USAGE_GENERIC_SLIDER: u16 = 54u16;
pub const HID_USAGE_GENERIC_START: u16 = 61u16;
pub const HID_USAGE_GENERIC_SYSCTL_APP_BREAK: u16 = 165u16;
pub const HID_USAGE_GENERIC_SYSCTL_APP_DBG_BREAK: u16 = 166u16;
pub const HID_USAGE_GENERIC_SYSCTL_APP_MENU: u16 = 134u16;
pub const HID_USAGE_GENERIC_SYSCTL_COLD_RESTART: u16 = 142u16;
pub const HID_USAGE_GENERIC_SYSCTL_CONTEXT_MENU: u16 = 132u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISMISS_NOTIFICATION: u16 = 154u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_AUTOSCALE: u16 = 183u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_BOTH: u16 = 179u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_DUAL: u16 = 180u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_EXTERNAL: u16 = 178u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INTERNAL: u16 = 177u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INVERT: u16 = 176u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_SWAP: u16 = 182u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_TOGGLE: u16 = 181u16;
pub const HID_USAGE_GENERIC_SYSCTL_DOCK: u16 = 160u16;
pub const HID_USAGE_GENERIC_SYSCTL_FN: u16 = 151u16;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK: u16 = 152u16;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK_INDICATOR: u16 = 153u16;
pub const HID_USAGE_GENERIC_SYSCTL_HELP_MENU: u16 = 135u16;
pub const HID_USAGE_GENERIC_SYSCTL_HIBERNATE: u16 = 168u16;
pub const HID_USAGE_GENERIC_SYSCTL_MAIN_MENU: u16 = 133u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_DOWN: u16 = 141u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_EXIT: u16 = 136u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_LEFT: u16 = 139u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_RIGHT: u16 = 138u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_SELECT: u16 = 137u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_UP: u16 = 140u16;
pub const HID_USAGE_GENERIC_SYSCTL_MUTE: u16 = 167u16;
pub const HID_USAGE_GENERIC_SYSCTL_POWER: u16 = 129u16;
pub const HID_USAGE_GENERIC_SYSCTL_SETUP: u16 = 162u16;
pub const HID_USAGE_GENERIC_SYSCTL_SLEEP: u16 = 130u16;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_BREAK: u16 = 163u16;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_DBG_BREAK: u16 = 164u16;
pub const HID_USAGE_GENERIC_SYSCTL_UNDOCK: u16 = 161u16;
pub const HID_USAGE_GENERIC_SYSCTL_WAKE: u16 = 131u16;
pub const HID_USAGE_GENERIC_SYSCTL_WARM_RESTART: u16 = 143u16;
pub const HID_USAGE_GENERIC_SYSTEM_CTL: u16 = 128u16;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_BUTTON: u16 = 201u16;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_SLIDER_SWITCH: u16 = 202u16;
pub const HID_USAGE_GENERIC_TABLET_PC_SYSTEM_CTL: u16 = 9u16;
pub const HID_USAGE_GENERIC_VBRX: u16 = 67u16;
pub const HID_USAGE_GENERIC_VBRY: u16 = 68u16;
pub const HID_USAGE_GENERIC_VBRZ: u16 = 69u16;
pub const HID_USAGE_GENERIC_VNO: u16 = 70u16;
pub const HID_USAGE_GENERIC_VX: u16 = 64u16;
pub const HID_USAGE_GENERIC_VY: u16 = 65u16;
pub const HID_USAGE_GENERIC_VZ: u16 = 66u16;
pub const HID_USAGE_GENERIC_WHEEL: u16 = 56u16;
pub const HID_USAGE_GENERIC_X: u16 = 48u16;
pub const HID_USAGE_GENERIC_Y: u16 = 49u16;
pub const HID_USAGE_GENERIC_Z: u16 = 50u16;
pub const HID_USAGE_HAPTICS_AUTO_ASSOCIATED_CONTROL: u16 = 34u16;
pub const HID_USAGE_HAPTICS_AUTO_TRIGGER: u16 = 32u16;
pub const HID_USAGE_HAPTICS_DURATION_LIST: u16 = 17u16;
pub const HID_USAGE_HAPTICS_INTENSITY: u16 = 35u16;
pub const HID_USAGE_HAPTICS_MANUAL_TRIGGER: u16 = 33u16;
pub const HID_USAGE_HAPTICS_REPEAT_COUNT: u16 = 36u16;
pub const HID_USAGE_HAPTICS_RETRIGGER_PERIOD: u16 = 37u16;
pub const HID_USAGE_HAPTICS_SIMPLE_CONTROLLER: u16 = 1u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_BEGIN: u16 = 4096u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_BUZZ: u16 = 4100u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_CLICK: u16 = 4099u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_CUTOFF_TIME: u16 = 40u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_END: u16 = 8191u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_LIST: u16 = 16u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_NULL: u16 = 4098u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_PRESS: u16 = 4102u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_RELEASE: u16 = 4103u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_RUMBLE: u16 = 4101u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_STOP: u16 = 4097u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_BEGIN: u16 = 8192u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_END: u16 = 12287u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_ID: u16 = 39u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_PAGE: u16 = 38u16;
pub const HID_USAGE_KEYBOARD_CAPS_LOCK: u16 = 57u16;
pub const HID_USAGE_KEYBOARD_DELETE: u16 = 42u16;
pub const HID_USAGE_KEYBOARD_DELETE_FORWARD: u16 = 76u16;
pub const HID_USAGE_KEYBOARD_ESCAPE: u16 = 41u16;
pub const HID_USAGE_KEYBOARD_F1: u16 = 58u16;
pub const HID_USAGE_KEYBOARD_F10: u16 = 67u16;
pub const HID_USAGE_KEYBOARD_F11: u16 = 68u16;
pub const HID_USAGE_KEYBOARD_F12: u16 = 69u16;
pub const HID_USAGE_KEYBOARD_F13: u16 = 104u16;
pub const HID_USAGE_KEYBOARD_F14: u16 = 105u16;
pub const HID_USAGE_KEYBOARD_F15: u16 = 106u16;
pub const HID_USAGE_KEYBOARD_F16: u16 = 107u16;
pub const HID_USAGE_KEYBOARD_F17: u16 = 108u16;
pub const HID_USAGE_KEYBOARD_F18: u16 = 109u16;
pub const HID_USAGE_KEYBOARD_F19: u16 = 110u16;
pub const HID_USAGE_KEYBOARD_F2: u16 = 59u16;
pub const HID_USAGE_KEYBOARD_F20: u16 = 111u16;
pub const HID_USAGE_KEYBOARD_F21: u16 = 112u16;
pub const HID_USAGE_KEYBOARD_F22: u16 = 113u16;
pub const HID_USAGE_KEYBOARD_F23: u16 = 114u16;
pub const HID_USAGE_KEYBOARD_F24: u16 = 115u16;
pub const HID_USAGE_KEYBOARD_F3: u16 = 60u16;
pub const HID_USAGE_KEYBOARD_F4: u16 = 61u16;
pub const HID_USAGE_KEYBOARD_F5: u16 = 62u16;
pub const HID_USAGE_KEYBOARD_F6: u16 = 63u16;
pub const HID_USAGE_KEYBOARD_F7: u16 = 64u16;
pub const HID_USAGE_KEYBOARD_F8: u16 = 65u16;
pub const HID_USAGE_KEYBOARD_F9: u16 = 66u16;
pub const HID_USAGE_KEYBOARD_LALT: u16 = 226u16;
pub const HID_USAGE_KEYBOARD_LCTRL: u16 = 224u16;
pub const HID_USAGE_KEYBOARD_LGUI: u16 = 227u16;
pub const HID_USAGE_KEYBOARD_LSHFT: u16 = 225u16;
pub const HID_USAGE_KEYBOARD_NOEVENT: u16 = 0u16;
pub const HID_USAGE_KEYBOARD_NUM_LOCK: u16 = 83u16;
pub const HID_USAGE_KEYBOARD_ONE: u16 = 30u16;
pub const HID_USAGE_KEYBOARD_POSTFAIL: u16 = 2u16;
pub const HID_USAGE_KEYBOARD_PRINT_SCREEN: u16 = 70u16;
pub const HID_USAGE_KEYBOARD_RALT: u16 = 230u16;
pub const HID_USAGE_KEYBOARD_RCTRL: u16 = 228u16;
pub const HID_USAGE_KEYBOARD_RETURN: u16 = 40u16;
pub const HID_USAGE_KEYBOARD_RGUI: u16 = 231u16;
pub const HID_USAGE_KEYBOARD_ROLLOVER: u16 = 1u16;
pub const HID_USAGE_KEYBOARD_RSHFT: u16 = 229u16;
pub const HID_USAGE_KEYBOARD_SCROLL_LOCK: u16 = 71u16;
pub const HID_USAGE_KEYBOARD_UNDEFINED: u16 = 3u16;
pub const HID_USAGE_KEYBOARD_ZERO: u16 = 39u16;
pub const HID_USAGE_KEYBOARD_aA: u16 = 4u16;
pub const HID_USAGE_KEYBOARD_zZ: u16 = 29u16;
pub const HID_USAGE_LAMPARRAY: u16 = 1u16;
pub const HID_USAGE_LAMPARRAY_ATTRBIUTES_REPORT: u16 = 2u16;
pub const HID_USAGE_LAMPARRAY_AUTONOMOUS_MODE: u16 = 113u16;
pub const HID_USAGE_LAMPARRAY_BLUE_LEVEL_COUNT: u16 = 42u16;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_DEPTH_IN_MICROMETERS: u16 = 6u16;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_HEIGHT_IN_MICROMETERS: u16 = 5u16;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_WIDTH_IN_MICROMETERS: u16 = 4u16;
pub const HID_USAGE_LAMPARRAY_CONTROL_REPORT: u16 = 112u16;
pub const HID_USAGE_LAMPARRAY_GREEN_LEVEL_COUNT: u16 = 41u16;
pub const HID_USAGE_LAMPARRAY_INPUT_BINDING: u16 = 45u16;
pub const HID_USAGE_LAMPARRAY_INTENSITY_LEVEL_COUNT: u16 = 43u16;
pub const HID_USAGE_LAMPARRAY_IS_PROGRAMMABLE: u16 = 44u16;
pub const HID_USAGE_LAMPARRAY_KIND: u16 = 7u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_REQUEST_REPORT: u16 = 32u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_RESPONSE_REPORT: u16 = 34u16;
pub const HID_USAGE_LAMPARRAY_LAMP_BLUE_UPDATE_CHANNEL: u16 = 83u16;
pub const HID_USAGE_LAMPARRAY_LAMP_COUNT: u16 = 3u16;
pub const HID_USAGE_LAMPARRAY_LAMP_GREEN_UPDATE_CHANNEL: u16 = 82u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ID: u16 = 33u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_END: u16 = 98u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_START: u16 = 97u16;
pub const HID_USAGE_LAMPARRAY_LAMP_INTENSITY_UPDATE_CHANNEL: u16 = 84u16;
pub const HID_USAGE_LAMPARRAY_LAMP_MULTI_UPDATE_REPORT: u16 = 80u16;
pub const HID_USAGE_LAMPARRAY_LAMP_PURPOSES: u16 = 38u16;
pub const HID_USAGE_LAMPARRAY_LAMP_RANGE_UPDATE_REPORT: u16 = 96u16;
pub const HID_USAGE_LAMPARRAY_LAMP_RED_UPDATE_CHANNEL: u16 = 81u16;
pub const HID_USAGE_LAMPARRAY_LAMP_UPDATE_FLAGS: u16 = 85u16;
pub const HID_USAGE_LAMPARRAY_MIN_UPDATE_INTERVAL_IN_MICROSECONDS: u16 = 8u16;
pub const HID_USAGE_LAMPARRAY_POSITION_X_IN_MICROMETERS: u16 = 35u16;
pub const HID_USAGE_LAMPARRAY_POSITION_Y_IN_MICROMETERS: u16 = 36u16;
pub const HID_USAGE_LAMPARRAY_POSITION_Z_IN_MICROMETERS: u16 = 37u16;
pub const HID_USAGE_LAMPARRAY_RED_LEVEL_COUNT: u16 = 40u16;
pub const HID_USAGE_LAMPARRAY_UPDATE_LATENCY_IN_MICROSECONDS: u16 = 39u16;
pub const HID_USAGE_LED_AMBER: u16 = 74u16;
pub const HID_USAGE_LED_BATTERY_LOW: u16 = 29u16;
pub const HID_USAGE_LED_BATTERY_OK: u16 = 28u16;
pub const HID_USAGE_LED_BATTERY_OPERATION: u16 = 27u16;
pub const HID_USAGE_LED_BUSY: u16 = 44u16;
pub const HID_USAGE_LED_CALL_PICKUP: u16 = 37u16;
pub const HID_USAGE_LED_CAMERA_OFF: u16 = 41u16;
pub const HID_USAGE_LED_CAMERA_ON: u16 = 40u16;
pub const HID_USAGE_LED_CAPS_LOCK: u16 = 2u16;
pub const HID_USAGE_LED_CAV: u16 = 20u16;
pub const HID_USAGE_LED_CLV: u16 = 21u16;
pub const HID_USAGE_LED_COMPOSE: u16 = 4u16;
pub const HID_USAGE_LED_CONFERENCE: u16 = 38u16;
pub const HID_USAGE_LED_COVERAGE: u16 = 34u16;
pub const HID_USAGE_LED_DATA_MODE: u16 = 26u16;
pub const HID_USAGE_LED_DO_NOT_DISTURB: u16 = 8u16;
pub const HID_USAGE_LED_EQUALIZER_ENABLE: u16 = 13u16;
pub const HID_USAGE_LED_ERROR: u16 = 57u16;
pub const HID_USAGE_LED_EXTERNAL_POWER: u16 = 77u16;
pub const HID_USAGE_LED_FAST_BLINK_OFF_TIME: u16 = 70u16;
pub const HID_USAGE_LED_FAST_BLINK_ON_TIME: u16 = 69u16;
pub const HID_USAGE_LED_FAST_FORWARD: u16 = 53u16;
pub const HID_USAGE_LED_FLASH_ON_TIME: u16 = 66u16;
pub const HID_USAGE_LED_FORWARD: u16 = 49u16;
pub const HID_USAGE_LED_GENERIC_INDICATOR: u16 = 75u16;
pub const HID_USAGE_LED_GREEN: u16 = 73u16;
pub const HID_USAGE_LED_HEAD_SET: u16 = 31u16;
pub const HID_USAGE_LED_HIGH_CUT_FILTER: u16 = 11u16;
pub const HID_USAGE_LED_HOLD: u16 = 32u16;
pub const HID_USAGE_LED_INDICATOR_COLOR: u16 = 71u16;
pub const HID_USAGE_LED_INDICATOR_FAST_BLINK: u16 = 64u16;
pub const HID_USAGE_LED_INDICATOR_FLASH: u16 = 62u16;
pub const HID_USAGE_LED_INDICATOR_OFF: u16 = 65u16;
pub const HID_USAGE_LED_INDICATOR_ON: u16 = 61u16;
pub const HID_USAGE_LED_INDICATOR_SLOW_BLINK: u16 = 63u16;
pub const HID_USAGE_LED_IN_USE_INDICATOR: u16 = 59u16;
pub const HID_USAGE_LED_KANA: u16 = 5u16;
pub const HID_USAGE_LED_LOW_CUT_FILTER: u16 = 12u16;
pub const HID_USAGE_LED_MESSAGE_WAITING: u16 = 25u16;
pub const HID_USAGE_LED_MICROPHONE: u16 = 33u16;
pub const HID_USAGE_LED_MULTI_MODE_INDICATOR: u16 = 60u16;
pub const HID_USAGE_LED_MUTE: u16 = 9u16;
pub const HID_USAGE_LED_NIGHT_MODE: u16 = 35u16;
pub const HID_USAGE_LED_NUM_LOCK: u16 = 1u16;
pub const HID_USAGE_LED_OFF_HOOK: u16 = 23u16;
pub const HID_USAGE_LED_OFF_LINE: u16 = 43u16;
pub const HID_USAGE_LED_ON_LINE: u16 = 42u16;
pub const HID_USAGE_LED_PAPER_JAM: u16 = 47u16;
pub const HID_USAGE_LED_PAPER_OUT: u16 = 46u16;
pub const HID_USAGE_LED_PAUSE: u16 = 55u16;
pub const HID_USAGE_LED_PLAY: u16 = 54u16;
pub const HID_USAGE_LED_POWER: u16 = 6u16;
pub const HID_USAGE_LED_READY: u16 = 45u16;
pub const HID_USAGE_LED_RECORD: u16 = 56u16;
pub const HID_USAGE_LED_RECORDING_FORMAT_DET: u16 = 22u16;
pub const HID_USAGE_LED_RED: u16 = 72u16;
pub const HID_USAGE_LED_REMOTE: u16 = 48u16;
pub const HID_USAGE_LED_REPEAT: u16 = 16u16;
pub const HID_USAGE_LED_REVERSE: u16 = 50u16;
pub const HID_USAGE_LED_REWIND: u16 = 52u16;
pub const HID_USAGE_LED_RING: u16 = 24u16;
pub const HID_USAGE_LED_SAMPLING_RATE_DETECT: u16 = 18u16;
pub const HID_USAGE_LED_SCROLL_LOCK: u16 = 3u16;
pub const HID_USAGE_LED_SELECTED_INDICATOR: u16 = 58u16;
pub const HID_USAGE_LED_SEND_CALLS: u16 = 36u16;
pub const HID_USAGE_LED_SHIFT: u16 = 7u16;
pub const HID_USAGE_LED_SLOW_BLINK_OFF_TIME: u16 = 68u16;
pub const HID_USAGE_LED_SLOW_BLINK_ON_TIME: u16 = 67u16;
pub const HID_USAGE_LED_SOUND_FIELD_ON: u16 = 14u16;
pub const HID_USAGE_LED_SPEAKER: u16 = 30u16;
pub const HID_USAGE_LED_SPINNING: u16 = 19u16;
pub const HID_USAGE_LED_STAND_BY: u16 = 39u16;
pub const HID_USAGE_LED_STEREO: u16 = 17u16;
pub const HID_USAGE_LED_STOP: u16 = 51u16;
pub const HID_USAGE_LED_SURROUND_FIELD_ON: u16 = 15u16;
pub const HID_USAGE_LED_SYSTEM_SUSPEND: u16 = 76u16;
pub const HID_USAGE_LED_TONE_ENABLE: u16 = 10u16;
pub const HID_USAGE_MS_BTH_HF_DIALMEMORY: u16 = 34u16;
pub const HID_USAGE_MS_BTH_HF_DIALNUMBER: u16 = 33u16;
pub const HID_USAGE_PAGE_ALPHANUMERIC: u16 = 20u16;
pub const HID_USAGE_PAGE_ARCADE: u16 = 145u16;
pub const HID_USAGE_PAGE_BARCODE_SCANNER: u16 = 140u16;
pub const HID_USAGE_PAGE_BUTTON: u16 = 9u16;
pub const HID_USAGE_PAGE_CAMERA_CONTROL: u16 = 144u16;
pub const HID_USAGE_PAGE_CONSUMER: u16 = 12u16;
pub const HID_USAGE_PAGE_DIGITIZER: u16 = 13u16;
pub const HID_USAGE_PAGE_GAME: u16 = 5u16;
pub const HID_USAGE_PAGE_GENERIC: u16 = 1u16;
pub const HID_USAGE_PAGE_GENERIC_DEVICE: u16 = 6u16;
pub const HID_USAGE_PAGE_HAPTICS: u16 = 14u16;
pub const HID_USAGE_PAGE_KEYBOARD: u16 = 7u16;
pub const HID_USAGE_PAGE_LED: u16 = 8u16;
pub const HID_USAGE_PAGE_LIGHTING_ILLUMINATION: u16 = 89u16;
pub const HID_USAGE_PAGE_MAGNETIC_STRIPE_READER: u16 = 142u16;
pub const HID_USAGE_PAGE_MICROSOFT_BLUETOOTH_HANDSFREE: u16 = 65523u16;
pub const HID_USAGE_PAGE_ORDINAL: u16 = 10u16;
pub const HID_USAGE_PAGE_PID: u16 = 15u16;
pub const HID_USAGE_PAGE_SENSOR: u16 = 32u16;
pub const HID_USAGE_PAGE_SIMULATION: u16 = 2u16;
pub const HID_USAGE_PAGE_SPORT: u16 = 4u16;
pub const HID_USAGE_PAGE_TELEPHONY: u16 = 11u16;
pub const HID_USAGE_PAGE_UNDEFINED: u16 = 0u16;
pub const HID_USAGE_PAGE_UNICODE: u16 = 16u16;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_BEGIN: u16 = 65280u16;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_END: u16 = 65535u16;
pub const HID_USAGE_PAGE_VR: u16 = 3u16;
pub const HID_USAGE_PAGE_WEIGHING_DEVICE: u16 = 141u16;
pub const HID_USAGE_SIMULATION_ACCELLERATOR: u16 = 196u16;
pub const HID_USAGE_SIMULATION_AILERON: u16 = 176u16;
pub const HID_USAGE_SIMULATION_AILERON_TRIM: u16 = 177u16;
pub const HID_USAGE_SIMULATION_AIRPLANE_SIMULATION_DEVICE: u16 = 9u16;
pub const HID_USAGE_SIMULATION_ANTI_TORQUE_CONTROL: u16 = 178u16;
pub const HID_USAGE_SIMULATION_AUTOMOBILE_SIMULATION_DEVICE: u16 = 2u16;
pub const HID_USAGE_SIMULATION_AUTOPIOLOT_ENABLE: u16 = 179u16;
pub const HID_USAGE_SIMULATION_BALLAST: u16 = 204u16;
pub const HID_USAGE_SIMULATION_BARREL_ELEVATION: u16 = 202u16;
pub const HID_USAGE_SIMULATION_BICYCLE_CRANK: u16 = 205u16;
pub const HID_USAGE_SIMULATION_BICYCLE_SIMULATION_DEVICE: u16 = 12u16;
pub const HID_USAGE_SIMULATION_BRAKE: u16 = 197u16;
pub const HID_USAGE_SIMULATION_CHAFF_RELEASE: u16 = 180u16;
pub const HID_USAGE_SIMULATION_CLUTCH: u16 = 198u16;
pub const HID_USAGE_SIMULATION_COLLECTIVE_CONTROL: u16 = 181u16;
pub const HID_USAGE_SIMULATION_CYCLIC_CONTROL: u16 = 34u16;
pub const HID_USAGE_SIMULATION_CYCLIC_TRIM: u16 = 35u16;
pub const HID_USAGE_SIMULATION_DIVE_BRAKE: u16 = 182u16;
pub const HID_USAGE_SIMULATION_DIVE_PLANE: u16 = 203u16;
pub const HID_USAGE_SIMULATION_ELECTRONIC_COUNTERMEASURES: u16 = 183u16;
pub const HID_USAGE_SIMULATION_ELEVATOR: u16 = 184u16;
pub const HID_USAGE_SIMULATION_ELEVATOR_TRIM: u16 = 185u16;
pub const HID_USAGE_SIMULATION_FLARE_RELEASE: u16 = 189u16;
pub const HID_USAGE_SIMULATION_FLIGHT_COMMUNICATIONS: u16 = 188u16;
pub const HID_USAGE_SIMULATION_FLIGHT_CONTROL_STICK: u16 = 32u16;
pub const HID_USAGE_SIMULATION_FLIGHT_SIMULATION_DEVICE: u16 = 1u16;
pub const HID_USAGE_SIMULATION_FLIGHT_STICK: u16 = 33u16;
pub const HID_USAGE_SIMULATION_FLIGHT_YOKE: u16 = 36u16;
pub const HID_USAGE_SIMULATION_FRONT_BRAKE: u16 = 207u16;
pub const HID_USAGE_SIMULATION_HANDLE_BARS: u16 = 206u16;
pub const HID_USAGE_SIMULATION_HELICOPTER_SIMULATION_DEVICE: u16 = 10u16;
pub const HID_USAGE_SIMULATION_LANDING_GEAR: u16 = 190u16;
pub const HID_USAGE_SIMULATION_MAGIC_CARPET_SIMULATION_DEVICE: u16 = 11u16;
pub const HID_USAGE_SIMULATION_MOTORCYCLE_SIMULATION_DEVICE: u16 = 7u16;
pub const HID_USAGE_SIMULATION_REAR_BRAKE: u16 = 208u16;
pub const HID_USAGE_SIMULATION_RUDDER: u16 = 186u16;
pub const HID_USAGE_SIMULATION_SAILING_SIMULATION_DEVICE: u16 = 6u16;
pub const HID_USAGE_SIMULATION_SHIFTER: u16 = 199u16;
pub const HID_USAGE_SIMULATION_SPACESHIP_SIMULATION_DEVICE: u16 = 4u16;
pub const HID_USAGE_SIMULATION_SPORTS_SIMULATION_DEVICE: u16 = 8u16;
pub const HID_USAGE_SIMULATION_STEERING: u16 = 200u16;
pub const HID_USAGE_SIMULATION_SUBMARINE_SIMULATION_DEVICE: u16 = 5u16;
pub const HID_USAGE_SIMULATION_TANK_SIMULATION_DEVICE: u16 = 3u16;
pub const HID_USAGE_SIMULATION_THROTTLE: u16 = 187u16;
pub const HID_USAGE_SIMULATION_TOE_BRAKE: u16 = 191u16;
pub const HID_USAGE_SIMULATION_TRACK_CONTROL: u16 = 37u16;
pub const HID_USAGE_SIMULATION_TRIGGER: u16 = 192u16;
pub const HID_USAGE_SIMULATION_TURRET_DIRECTION: u16 = 201u16;
pub const HID_USAGE_SIMULATION_WEAPONS_ARM: u16 = 193u16;
pub const HID_USAGE_SIMULATION_WEAPONS_SELECT: u16 = 194u16;
pub const HID_USAGE_SIMULATION_WING_FLAPS: u16 = 195u16;
pub const HID_USAGE_SPORT_10_IRON: u16 = 90u16;
pub const HID_USAGE_SPORT_11_IRON: u16 = 91u16;
pub const HID_USAGE_SPORT_1_IRON: u16 = 81u16;
pub const HID_USAGE_SPORT_1_WOOD: u16 = 95u16;
pub const HID_USAGE_SPORT_2_IRON: u16 = 82u16;
pub const HID_USAGE_SPORT_3_IRON: u16 = 83u16;
pub const HID_USAGE_SPORT_3_WOOD: u16 = 96u16;
pub const HID_USAGE_SPORT_4_IRON: u16 = 84u16;
pub const HID_USAGE_SPORT_5_IRON: u16 = 85u16;
pub const HID_USAGE_SPORT_5_WOOD: u16 = 97u16;
pub const HID_USAGE_SPORT_6_IRON: u16 = 86u16;
pub const HID_USAGE_SPORT_7_IRON: u16 = 87u16;
pub const HID_USAGE_SPORT_7_WOOD: u16 = 98u16;
pub const HID_USAGE_SPORT_8_IRON: u16 = 88u16;
pub const HID_USAGE_SPORT_9_IRON: u16 = 89u16;
pub const HID_USAGE_SPORT_9_WOOD: u16 = 99u16;
pub const HID_USAGE_SPORT_BASEBALL_BAT: u16 = 1u16;
pub const HID_USAGE_SPORT_FOLLOW_THROUGH: u16 = 54u16;
pub const HID_USAGE_SPORT_GOLF_CLUB: u16 = 2u16;
pub const HID_USAGE_SPORT_HEEL_TOE: u16 = 53u16;
pub const HID_USAGE_SPORT_HEIGHT: u16 = 57u16;
pub const HID_USAGE_SPORT_LOFT_WEDGE: u16 = 93u16;
pub const HID_USAGE_SPORT_OAR: u16 = 48u16;
pub const HID_USAGE_SPORT_POWER_WEDGE: u16 = 94u16;
pub const HID_USAGE_SPORT_PUTTER: u16 = 80u16;
pub const HID_USAGE_SPORT_RATE: u16 = 50u16;
pub const HID_USAGE_SPORT_ROWING_MACHINE: u16 = 3u16;
pub const HID_USAGE_SPORT_SAND_WEDGE: u16 = 92u16;
pub const HID_USAGE_SPORT_SLOPE: u16 = 49u16;
pub const HID_USAGE_SPORT_STICK_FACE_ANGLE: u16 = 52u16;
pub const HID_USAGE_SPORT_STICK_SPEED: u16 = 51u16;
pub const HID_USAGE_SPORT_STICK_TYPE: u16 = 56u16;
pub const HID_USAGE_SPORT_TEMPO: u16 = 55u16;
pub const HID_USAGE_SPORT_TREADMILL: u16 = 4u16;
pub const HID_USAGE_TELEPHONY_ANSWERING_MACHINE: u16 = 2u16;
pub const HID_USAGE_TELEPHONY_DROP: u16 = 38u16;
pub const HID_USAGE_TELEPHONY_HANDSET: u16 = 4u16;
pub const HID_USAGE_TELEPHONY_HEADSET: u16 = 5u16;
pub const HID_USAGE_TELEPHONY_HOST_AVAILABLE: u16 = 241u16;
pub const HID_USAGE_TELEPHONY_KEYPAD: u16 = 6u16;
pub const HID_USAGE_TELEPHONY_KEYPAD_0: u16 = 176u16;
pub const HID_USAGE_TELEPHONY_KEYPAD_D: u16 = 191u16;
pub const HID_USAGE_TELEPHONY_LINE: u16 = 42u16;
pub const HID_USAGE_TELEPHONY_MESSAGE_CONTROLS: u16 = 3u16;
pub const HID_USAGE_TELEPHONY_PHONE: u16 = 1u16;
pub const HID_USAGE_TELEPHONY_PROGRAMMABLE_BUTTON: u16 = 7u16;
pub const HID_USAGE_TELEPHONY_REDIAL: u16 = 36u16;
pub const HID_USAGE_TELEPHONY_RING_ENABLE: u16 = 45u16;
pub const HID_USAGE_TELEPHONY_SEND: u16 = 49u16;
pub const HID_USAGE_TELEPHONY_TRANSFER: u16 = 37u16;
pub const HID_USAGE_VR_ANIMATRONIC_DEVICE: u16 = 10u16;
pub const HID_USAGE_VR_BELT: u16 = 1u16;
pub const HID_USAGE_VR_BODY_SUIT: u16 = 2u16;
pub const HID_USAGE_VR_DISPLAY_ENABLE: u16 = 33u16;
pub const HID_USAGE_VR_FLEXOR: u16 = 3u16;
pub const HID_USAGE_VR_GLOVE: u16 = 4u16;
pub const HID_USAGE_VR_HAND_TRACKER: u16 = 7u16;
pub const HID_USAGE_VR_HEAD_MOUNTED_DISPLAY: u16 = 6u16;
pub const HID_USAGE_VR_HEAD_TRACKER: u16 = 5u16;
pub const HID_USAGE_VR_OCULOMETER: u16 = 8u16;
pub const HID_USAGE_VR_STEREO_ENABLE: u16 = 32u16;
pub const HID_USAGE_VR_VEST: u16 = 9u16;
pub const HORIZONTAL_WHEEL_PRESENT: u32 = 32768u32;
pub const HidP_Feature: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(2i32);
pub const HidP_Input: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(0i32);
pub const HidP_Keyboard_Break: HIDP_KEYBOARD_DIRECTION = HIDP_KEYBOARD_DIRECTION(0i32);
pub const HidP_Keyboard_Make: HIDP_KEYBOARD_DIRECTION = HIDP_KEYBOARD_DIRECTION(1i32);
pub const HidP_Output: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(1i32);
pub const IOCTL_BUTTON_GET_ENABLED_ON_IDLE: u32 = 721580u32;
pub const IOCTL_BUTTON_SET_ENABLED_ON_IDLE: u32 = 721576u32;
pub const IOCTL_KEYBOARD_INSERT_DATA: u32 = 721152u32;
pub const IOCTL_KEYBOARD_QUERY_ATTRIBUTES: u32 = 720896u32;
pub const IOCTL_KEYBOARD_QUERY_EXTENDED_ATTRIBUTES: u32 = 721408u32;
pub const IOCTL_KEYBOARD_QUERY_IME_STATUS: u32 = 724992u32;
pub const IOCTL_KEYBOARD_QUERY_INDICATORS: u32 = 720960u32;
pub const IOCTL_KEYBOARD_QUERY_INDICATOR_TRANSLATION: u32 = 721024u32;
pub const IOCTL_KEYBOARD_QUERY_TYPEMATIC: u32 = 720928u32;
pub const IOCTL_KEYBOARD_SET_IME_STATUS: u32 = 724996u32;
pub const IOCTL_KEYBOARD_SET_INDICATORS: u32 = 720904u32;
pub const IOCTL_KEYBOARD_SET_TYPEMATIC: u32 = 720900u32;
pub const IOCTL_MOUSE_INSERT_DATA: u32 = 983044u32;
pub const IOCTL_MOUSE_QUERY_ATTRIBUTES: u32 = 983040u32;
pub const JOYTYPE_ANALOGCOMPAT: i32 = 8i32;
pub const JOYTYPE_DEFAULTPROPSHEET: i32 = -2147483648i32;
pub const JOYTYPE_DEVICEHIDE: i32 = 65536i32;
pub const JOYTYPE_ENABLEINPUTREPORT: i32 = 16777216i32;
pub const JOYTYPE_GAMEHIDE: i32 = 524288i32;
pub const JOYTYPE_HIDEACTIVE: i32 = 1048576i32;
pub const JOYTYPE_INFODEFAULT: i32 = 0i32;
pub const JOYTYPE_INFOMASK: i32 = 14680064i32;
pub const JOYTYPE_INFOYRPEDALS: i32 = 6291456i32;
pub const JOYTYPE_INFOYYPEDALS: i32 = 2097152i32;
pub const JOYTYPE_INFOZISSLIDER: i32 = 2097152i32;
pub const JOYTYPE_INFOZISZ: i32 = 4194304i32;
pub const JOYTYPE_INFOZRPEDALS: i32 = 8388608i32;
pub const JOYTYPE_INFOZYPEDALS: i32 = 4194304i32;
pub const JOYTYPE_KEYBHIDE: i32 = 262144i32;
pub const JOYTYPE_MOUSEHIDE: i32 = 131072i32;
pub const JOYTYPE_NOAUTODETECTGAMEPORT: i32 = 2i32;
pub const JOYTYPE_NOHIDDIRECT: i32 = 4i32;
pub const JOYTYPE_ZEROGAMEENUMOEMDATA: i32 = 1i32;
pub const JOY_HWS_AUTOLOAD: i32 = 268435456i32;
pub const JOY_HWS_GAMEPORTBUSBUSY: i32 = 1i32;
pub const JOY_HWS_HASPOV: i32 = 2i32;
pub const JOY_HWS_HASR: i32 = 524288i32;
pub const JOY_HWS_HASU: i32 = 8388608i32;
pub const JOY_HWS_HASV: i32 = 16777216i32;
pub const JOY_HWS_HASZ: i32 = 1i32;
pub const JOY_HWS_ISANALOGPORTDRIVER: i32 = 134217728i32;
pub const JOY_HWS_ISCARCTRL: i32 = 64i32;
pub const JOY_HWS_ISGAMEPAD: i32 = 32i32;
pub const JOY_HWS_ISGAMEPORTBUS: i32 = -2147483648i32;
pub const JOY_HWS_ISGAMEPORTDRIVER: i32 = 67108864i32;
pub const JOY_HWS_ISHEADTRACKER: i32 = 33554432i32;
pub const JOY_HWS_ISYOKE: i32 = 16i32;
pub const JOY_HWS_NODEVNODE: i32 = 536870912i32;
pub const JOY_HWS_POVISBUTTONCOMBOS: i32 = 4i32;
pub const JOY_HWS_POVISJ1X: i32 = 65536i32;
pub const JOY_HWS_POVISJ1Y: i32 = 131072i32;
pub const JOY_HWS_POVISJ2X: i32 = 262144i32;
pub const JOY_HWS_POVISPOLL: i32 = 8i32;
pub const JOY_HWS_RISJ1X: i32 = 1048576i32;
pub const JOY_HWS_RISJ1Y: i32 = 2097152i32;
pub const JOY_HWS_RISJ2Y: i32 = 4194304i32;
pub const JOY_HWS_XISJ1Y: i32 = 128i32;
pub const JOY_HWS_XISJ2X: i32 = 256i32;
pub const JOY_HWS_XISJ2Y: i32 = 512i32;
pub const JOY_HWS_YISJ1X: i32 = 1024i32;
pub const JOY_HWS_YISJ2X: i32 = 2048i32;
pub const JOY_HWS_YISJ2Y: i32 = 4096i32;
pub const JOY_HWS_ZISJ1X: i32 = 8192i32;
pub const JOY_HWS_ZISJ1Y: i32 = 16384i32;
pub const JOY_HWS_ZISJ2X: i32 = 32768i32;
pub const JOY_HW_2A_2B_GENERIC: u32 = 2u32;
pub const JOY_HW_2A_4B_GENERIC: u32 = 3u32;
pub const JOY_HW_2B_FLIGHTYOKE: u32 = 5u32;
pub const JOY_HW_2B_FLIGHTYOKETHROTTLE: u32 = 6u32;
pub const JOY_HW_2B_GAMEPAD: u32 = 4u32;
pub const JOY_HW_3A_2B_GENERIC: u32 = 7u32;
pub const JOY_HW_3A_4B_GENERIC: u32 = 8u32;
pub const JOY_HW_4B_FLIGHTYOKE: u32 = 10u32;
pub const JOY_HW_4B_FLIGHTYOKETHROTTLE: u32 = 11u32;
pub const JOY_HW_4B_GAMEPAD: u32 = 9u32;
pub const JOY_HW_CUSTOM: u32 = 1u32;
pub const JOY_HW_LASTENTRY: u32 = 13u32;
pub const JOY_HW_NONE: u32 = 0u32;
pub const JOY_HW_TWO_2A_2B_WITH_Y: u32 = 12u32;
pub const JOY_ISCAL_POV: i32 = 32i32;
pub const JOY_ISCAL_R: i32 = 4i32;
pub const JOY_ISCAL_U: i32 = 8i32;
pub const JOY_ISCAL_V: i32 = 16i32;
pub const JOY_ISCAL_XY: i32 = 1i32;
pub const JOY_ISCAL_Z: i32 = 2i32;
pub const JOY_OEMPOLL_PASSDRIVERDATA: u32 = 7u32;
pub const JOY_PASSDRIVERDATA: i32 = 268435456i32;
pub const JOY_POVVAL_BACKWARD: u32 = 1u32;
pub const JOY_POVVAL_FORWARD: u32 = 0u32;
pub const JOY_POVVAL_LEFT: u32 = 2u32;
pub const JOY_POVVAL_RIGHT: u32 = 3u32;
pub const JOY_POV_NUMDIRS: u32 = 4u32;
pub const JOY_US_HASRUDDER: i32 = 1i32;
pub const JOY_US_ISOEM: i32 = 4i32;
pub const JOY_US_PRESENT: i32 = 2i32;
pub const JOY_US_RESERVED: i32 = -2147483648i32;
pub const JOY_US_VOLATILE: i32 = 8i32;
pub const KEYBOARD_CAPS_LOCK_ON: u32 = 4u32;
pub const KEYBOARD_ERROR_VALUE_BASE: u32 = 10000u32;
pub const KEYBOARD_EXTENDED_ATTRIBUTES_STRUCT_VERSION_1: u32 = 1u32;
pub const KEYBOARD_KANA_LOCK_ON: u32 = 8u32;
pub const KEYBOARD_LED_INJECTED: u32 = 32768u32;
pub const KEYBOARD_NUM_LOCK_ON: u32 = 2u32;
pub const KEYBOARD_OVERRUN_MAKE_CODE: u32 = 255u32;
pub const KEYBOARD_SCROLL_LOCK_ON: u32 = 1u32;
pub const KEYBOARD_SHADOW: u32 = 16384u32;
pub const KEY_BREAK: u32 = 1u32;
pub const KEY_E0: u32 = 2u32;
pub const KEY_E1: u32 = 4u32;
pub const KEY_FROM_KEYBOARD_OVERRIDER: u32 = 128u32;
pub const KEY_MAKE: u32 = 0u32;
pub const KEY_RIM_VKEY: u32 = 64u32;
pub const KEY_TERMSRV_SET_LED: u32 = 8u32;
pub const KEY_TERMSRV_SHADOW: u32 = 16u32;
pub const KEY_TERMSRV_VKPACKET: u32 = 32u32;
pub const KEY_UNICODE_SEQUENCE_END: u32 = 512u32;
pub const KEY_UNICODE_SEQUENCE_ITEM: u32 = 256u32;
pub const MAXCPOINTSNUM: u32 = 8u32;
pub const MAX_JOYSTICKOEMVXDNAME: u32 = 260u32;
pub const MAX_JOYSTRING: u32 = 256u32;
pub const MOUSE_ATTRIBUTES_CHANGED: u32 = 4u32;
pub const MOUSE_BUTTON_1_DOWN: u32 = 1u32;
pub const MOUSE_BUTTON_1_UP: u32 = 2u32;
pub const MOUSE_BUTTON_2_DOWN: u32 = 4u32;
pub const MOUSE_BUTTON_2_UP: u32 = 8u32;
pub const MOUSE_BUTTON_3_DOWN: u32 = 16u32;
pub const MOUSE_BUTTON_3_UP: u32 = 32u32;
pub const MOUSE_BUTTON_4_DOWN: u32 = 64u32;
pub const MOUSE_BUTTON_4_UP: u32 = 128u32;
pub const MOUSE_BUTTON_5_DOWN: u32 = 256u32;
pub const MOUSE_BUTTON_5_UP: u32 = 512u32;
pub const MOUSE_ERROR_VALUE_BASE: u32 = 20000u32;
pub const MOUSE_HID_HARDWARE: u32 = 128u32;
pub const MOUSE_HWHEEL: u32 = 2048u32;
pub const MOUSE_I8042_HARDWARE: u32 = 2u32;
pub const MOUSE_INPORT_HARDWARE: u32 = 1u32;
pub const MOUSE_LEFT_BUTTON_DOWN: u32 = 1u32;
pub const MOUSE_LEFT_BUTTON_UP: u32 = 2u32;
pub const MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16u32;
pub const MOUSE_MIDDLE_BUTTON_UP: u32 = 32u32;
pub const MOUSE_MOVE_ABSOLUTE: u32 = 1u32;
pub const MOUSE_MOVE_NOCOALESCE: u32 = 8u32;
pub const MOUSE_MOVE_RELATIVE: u32 = 0u32;
pub const MOUSE_RIGHT_BUTTON_DOWN: u32 = 4u32;
pub const MOUSE_RIGHT_BUTTON_UP: u32 = 8u32;
pub const MOUSE_SERIAL_HARDWARE: u32 = 4u32;
pub const MOUSE_TERMSRV_SRC_SHADOW: u32 = 256u32;
pub const MOUSE_VIRTUAL_DESKTOP: u32 = 2u32;
pub const MOUSE_WHEEL: u32 = 1024u32;
pub const WHEELMOUSE_HID_HARDWARE: u32 = 256u32;
pub const WHEELMOUSE_I8042_HARDWARE: u32 = 32u32;
pub const WHEELMOUSE_SERIAL_HARDWARE: u32 = 64u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPIOBUTTONS_BUTTON_TYPE(pub i32);
impl ::core::marker::Copy for GPIOBUTTONS_BUTTON_TYPE {}
impl ::core::clone::Clone for GPIOBUTTONS_BUTTON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPIOBUTTONS_BUTTON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPIOBUTTONS_BUTTON_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPIOBUTTONS_BUTTON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPIOBUTTONS_BUTTON_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HIDP_KEYBOARD_DIRECTION(pub i32);
impl ::core::marker::Copy for HIDP_KEYBOARD_DIRECTION {}
impl ::core::clone::Clone for HIDP_KEYBOARD_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HIDP_KEYBOARD_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HIDP_KEYBOARD_DIRECTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HIDP_KEYBOARD_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIDP_KEYBOARD_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HIDP_REPORT_TYPE(pub i32);
impl ::core::marker::Copy for HIDP_REPORT_TYPE {}
impl ::core::clone::Clone for HIDP_REPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HIDP_REPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HIDP_REPORT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HIDP_REPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIDP_REPORT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CPOINT {
    pub lP: i32,
    pub dwLog: u32,
}
impl ::core::marker::Copy for CPOINT {}
impl ::core::clone::Clone for CPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPOINT").field("lP", &self.lP).field("dwLog", &self.dwLog).finish()
    }
}
impl ::windows_core::TypeKind for CPOINT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.lP == other.lP && self.dwLog == other.dwLog
    }
}
impl ::core::cmp::Eq for CPOINT {}
impl ::core::default::Default for CPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIACTIONA {
    pub uAppData: usize,
    pub dwSemantic: u32,
    pub dwFlags: u32,
    pub Anonymous: DIACTIONA_0,
    pub guidInstance: ::windows_core::GUID,
    pub dwObjID: u32,
    pub dwHow: u32,
}
impl ::core::marker::Copy for DIACTIONA {}
impl ::core::clone::Clone for DIACTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for DIACTIONA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for DIACTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DIACTIONA_0 {
    pub lptszActionName: ::windows_core::PCSTR,
    pub uResIdString: u32,
}
impl ::core::marker::Copy for DIACTIONA_0 {}
impl ::core::clone::Clone for DIACTIONA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for DIACTIONA_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for DIACTIONA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIACTIONFORMATA {
    pub dwSize: u32,
    pub dwActionSize: u32,
    pub dwDataSize: u32,
    pub dwNumActions: u32,
    pub rgoAction: *mut DIACTIONA,
    pub guidActionMap: ::windows_core::GUID,
    pub dwGenre: u32,
    pub dwBufferSize: u32,
    pub lAxisMin: i32,
    pub lAxisMax: i32,
    pub hInstString: super::super::Foundation::HINSTANCE,
    pub ftTimeStamp: super::super::Foundation::FILETIME,
    pub dwCRC: u32,
    pub tszActionMap: [u8; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIACTIONFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIACTIONFORMATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIACTIONFORMATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIACTIONFORMATA")
            .field("dwSize", &self.dwSize)
            .field("dwActionSize", &self.dwActionSize)
            .field("dwDataSize", &self.dwDataSize)
            .field("dwNumActions", &self.dwNumActions)
            .field("rgoAction", &self.rgoAction)
            .field("guidActionMap", &self.guidActionMap)
            .field("dwGenre", &self.dwGenre)
            .field("dwBufferSize", &self.dwBufferSize)
            .field("lAxisMin", &self.lAxisMin)
            .field("lAxisMax", &self.lAxisMax)
            .field("hInstString", &self.hInstString)
            .field("ftTimeStamp", &self.ftTimeStamp)
            .field("dwCRC", &self.dwCRC)
            .field("tszActionMap", &self.tszActionMap)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DIACTIONFORMATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIACTIONFORMATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwActionSize == other.dwActionSize && self.dwDataSize == other.dwDataSize && self.dwNumActions == other.dwNumActions && self.rgoAction == other.rgoAction && self.guidActionMap == other.guidActionMap && self.dwGenre == other.dwGenre && self.dwBufferSize == other.dwBufferSize && self.lAxisMin == other.lAxisMin && self.lAxisMax == other.lAxisMax && self.hInstString == other.hInstString && self.ftTimeStamp == other.ftTimeStamp && self.dwCRC == other.dwCRC && self.tszActionMap == other.tszActionMap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIACTIONFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIACTIONFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIACTIONFORMATW {
    pub dwSize: u32,
    pub dwActionSize: u32,
    pub dwDataSize: u32,
    pub dwNumActions: u32,
    pub rgoAction: *mut DIACTIONW,
    pub guidActionMap: ::windows_core::GUID,
    pub dwGenre: u32,
    pub dwBufferSize: u32,
    pub lAxisMin: i32,
    pub lAxisMax: i32,
    pub hInstString: super::super::Foundation::HINSTANCE,
    pub ftTimeStamp: super::super::Foundation::FILETIME,
    pub dwCRC: u32,
    pub tszActionMap: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIACTIONFORMATW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIACTIONFORMATW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIACTIONFORMATW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIACTIONFORMATW")
            .field("dwSize", &self.dwSize)
            .field("dwActionSize", &self.dwActionSize)
            .field("dwDataSize", &self.dwDataSize)
            .field("dwNumActions", &self.dwNumActions)
            .field("rgoAction", &self.rgoAction)
            .field("guidActionMap", &self.guidActionMap)
            .field("dwGenre", &self.dwGenre)
            .field("dwBufferSize", &self.dwBufferSize)
            .field("lAxisMin", &self.lAxisMin)
            .field("lAxisMax", &self.lAxisMax)
            .field("hInstString", &self.hInstString)
            .field("ftTimeStamp", &self.ftTimeStamp)
            .field("dwCRC", &self.dwCRC)
            .field("tszActionMap", &self.tszActionMap)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DIACTIONFORMATW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIACTIONFORMATW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwActionSize == other.dwActionSize && self.dwDataSize == other.dwDataSize && self.dwNumActions == other.dwNumActions && self.rgoAction == other.rgoAction && self.guidActionMap == other.guidActionMap && self.dwGenre == other.dwGenre && self.dwBufferSize == other.dwBufferSize && self.lAxisMin == other.lAxisMin && self.lAxisMax == other.lAxisMax && self.hInstString == other.hInstString && self.ftTimeStamp == other.ftTimeStamp && self.dwCRC == other.dwCRC && self.tszActionMap == other.tszActionMap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIACTIONFORMATW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIACTIONFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIACTIONW {
    pub uAppData: usize,
    pub dwSemantic: u32,
    pub dwFlags: u32,
    pub Anonymous: DIACTIONW_0,
    pub guidInstance: ::windows_core::GUID,
    pub dwObjID: u32,
    pub dwHow: u32,
}
impl ::core::marker::Copy for DIACTIONW {}
impl ::core::clone::Clone for DIACTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for DIACTIONW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for DIACTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DIACTIONW_0 {
    pub lptszActionName: ::windows_core::PCWSTR,
    pub uResIdString: u32,
}
impl ::core::marker::Copy for DIACTIONW_0 {}
impl ::core::clone::Clone for DIACTIONW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for DIACTIONW_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for DIACTIONW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DICOLORSET {
    pub dwSize: u32,
    pub cTextFore: u32,
    pub cTextHighlight: u32,
    pub cCalloutLine: u32,
    pub cCalloutHighlight: u32,
    pub cBorder: u32,
    pub cControlFill: u32,
    pub cHighlightFill: u32,
    pub cAreaFill: u32,
}
impl ::core::marker::Copy for DICOLORSET {}
impl ::core::clone::Clone for DICOLORSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DICOLORSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICOLORSET").field("dwSize", &self.dwSize).field("cTextFore", &self.cTextFore).field("cTextHighlight", &self.cTextHighlight).field("cCalloutLine", &self.cCalloutLine).field("cCalloutHighlight", &self.cCalloutHighlight).field("cBorder", &self.cBorder).field("cControlFill", &self.cControlFill).field("cHighlightFill", &self.cHighlightFill).field("cAreaFill", &self.cAreaFill).finish()
    }
}
impl ::windows_core::TypeKind for DICOLORSET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DICOLORSET {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cTextFore == other.cTextFore && self.cTextHighlight == other.cTextHighlight && self.cCalloutLine == other.cCalloutLine && self.cCalloutHighlight == other.cCalloutHighlight && self.cBorder == other.cBorder && self.cControlFill == other.cControlFill && self.cHighlightFill == other.cHighlightFill && self.cAreaFill == other.cAreaFill
    }
}
impl ::core::cmp::Eq for DICOLORSET {}
impl ::core::default::Default for DICOLORSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DICONDITION {
    pub lOffset: i32,
    pub lPositiveCoefficient: i32,
    pub lNegativeCoefficient: i32,
    pub dwPositiveSaturation: u32,
    pub dwNegativeSaturation: u32,
    pub lDeadBand: i32,
}
impl ::core::marker::Copy for DICONDITION {}
impl ::core::clone::Clone for DICONDITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DICONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONDITION").field("lOffset", &self.lOffset).field("lPositiveCoefficient", &self.lPositiveCoefficient).field("lNegativeCoefficient", &self.lNegativeCoefficient).field("dwPositiveSaturation", &self.dwPositiveSaturation).field("dwNegativeSaturation", &self.dwNegativeSaturation).field("lDeadBand", &self.lDeadBand).finish()
    }
}
impl ::windows_core::TypeKind for DICONDITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DICONDITION {
    fn eq(&self, other: &Self) -> bool {
        self.lOffset == other.lOffset && self.lPositiveCoefficient == other.lPositiveCoefficient && self.lNegativeCoefficient == other.lNegativeCoefficient && self.dwPositiveSaturation == other.dwPositiveSaturation && self.dwNegativeSaturation == other.dwNegativeSaturation && self.lDeadBand == other.lDeadBand
    }
}
impl ::core::cmp::Eq for DICONDITION {}
impl ::core::default::Default for DICONDITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DICONFIGUREDEVICESPARAMSA {
    pub dwSize: u32,
    pub dwcUsers: u32,
    pub lptszUserNames: ::windows_core::PSTR,
    pub dwcFormats: u32,
    pub lprgFormats: *mut DIACTIONFORMATA,
    pub hwnd: super::super::Foundation::HWND,
    pub dics: DICOLORSET,
    pub lpUnkDDSTarget: ::std::mem::ManuallyDrop<::core::option::Option<::windows_core::IUnknown>>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DICONFIGUREDEVICESPARAMSA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DICONFIGUREDEVICESPARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONFIGUREDEVICESPARAMSA").field("dwSize", &self.dwSize).field("dwcUsers", &self.dwcUsers).field("lptszUserNames", &self.lptszUserNames).field("dwcFormats", &self.dwcFormats).field("lprgFormats", &self.lprgFormats).field("hwnd", &self.hwnd).field("dics", &self.dics).field("lpUnkDDSTarget", &self.lpUnkDDSTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DICONFIGUREDEVICESPARAMSA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DICONFIGUREDEVICESPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwcUsers == other.dwcUsers && self.lptszUserNames == other.lptszUserNames && self.dwcFormats == other.dwcFormats && self.lprgFormats == other.lprgFormats && self.hwnd == other.hwnd && self.dics == other.dics && self.lpUnkDDSTarget == other.lpUnkDDSTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DICONFIGUREDEVICESPARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DICONFIGUREDEVICESPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DICONFIGUREDEVICESPARAMSW {
    pub dwSize: u32,
    pub dwcUsers: u32,
    pub lptszUserNames: ::windows_core::PWSTR,
    pub dwcFormats: u32,
    pub lprgFormats: *mut DIACTIONFORMATW,
    pub hwnd: super::super::Foundation::HWND,
    pub dics: DICOLORSET,
    pub lpUnkDDSTarget: ::std::mem::ManuallyDrop<::core::option::Option<::windows_core::IUnknown>>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DICONFIGUREDEVICESPARAMSW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DICONFIGUREDEVICESPARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONFIGUREDEVICESPARAMSW").field("dwSize", &self.dwSize).field("dwcUsers", &self.dwcUsers).field("lptszUserNames", &self.lptszUserNames).field("dwcFormats", &self.dwcFormats).field("lprgFormats", &self.lprgFormats).field("hwnd", &self.hwnd).field("dics", &self.dics).field("lpUnkDDSTarget", &self.lpUnkDDSTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DICONFIGUREDEVICESPARAMSW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DICONFIGUREDEVICESPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwcUsers == other.dwcUsers && self.lptszUserNames == other.lptszUserNames && self.dwcFormats == other.dwcFormats && self.lprgFormats == other.lprgFormats && self.hwnd == other.hwnd && self.dics == other.dics && self.lpUnkDDSTarget == other.lpUnkDDSTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DICONFIGUREDEVICESPARAMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DICONFIGUREDEVICESPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DICONSTANTFORCE {
    pub lMagnitude: i32,
}
impl ::core::marker::Copy for DICONSTANTFORCE {}
impl ::core::clone::Clone for DICONSTANTFORCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DICONSTANTFORCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONSTANTFORCE").field("lMagnitude", &self.lMagnitude).finish()
    }
}
impl ::windows_core::TypeKind for DICONSTANTFORCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DICONSTANTFORCE {
    fn eq(&self, other: &Self) -> bool {
        self.lMagnitude == other.lMagnitude
    }
}
impl ::core::cmp::Eq for DICONSTANTFORCE {}
impl ::core::default::Default for DICONSTANTFORCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DICUSTOMFORCE {
    pub cChannels: u32,
    pub dwSamplePeriod: u32,
    pub cSamples: u32,
    pub rglForceData: *mut i32,
}
impl ::core::marker::Copy for DICUSTOMFORCE {}
impl ::core::clone::Clone for DICUSTOMFORCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DICUSTOMFORCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICUSTOMFORCE").field("cChannels", &self.cChannels).field("dwSamplePeriod", &self.dwSamplePeriod).field("cSamples", &self.cSamples).field("rglForceData", &self.rglForceData).finish()
    }
}
impl ::windows_core::TypeKind for DICUSTOMFORCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DICUSTOMFORCE {
    fn eq(&self, other: &Self) -> bool {
        self.cChannels == other.cChannels && self.dwSamplePeriod == other.dwSamplePeriod && self.cSamples == other.cSamples && self.rglForceData == other.rglForceData
    }
}
impl ::core::cmp::Eq for DICUSTOMFORCE {}
impl ::core::default::Default for DICUSTOMFORCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDATAFORMAT {
    pub dwSize: u32,
    pub dwObjSize: u32,
    pub dwFlags: u32,
    pub dwDataSize: u32,
    pub dwNumObjs: u32,
    pub rgodf: *mut DIOBJECTDATAFORMAT,
}
impl ::core::marker::Copy for DIDATAFORMAT {}
impl ::core::clone::Clone for DIDATAFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDATAFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDATAFORMAT").field("dwSize", &self.dwSize).field("dwObjSize", &self.dwObjSize).field("dwFlags", &self.dwFlags).field("dwDataSize", &self.dwDataSize).field("dwNumObjs", &self.dwNumObjs).field("rgodf", &self.rgodf).finish()
    }
}
impl ::windows_core::TypeKind for DIDATAFORMAT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDATAFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwObjSize == other.dwObjSize && self.dwFlags == other.dwFlags && self.dwDataSize == other.dwDataSize && self.dwNumObjs == other.dwNumObjs && self.rgodf == other.rgodf
    }
}
impl ::core::cmp::Eq for DIDATAFORMAT {}
impl ::core::default::Default for DIDATAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDevType: u32,
    pub dwAxes: u32,
    pub dwButtons: u32,
    pub dwPOVs: u32,
    pub dwFFSamplePeriod: u32,
    pub dwFFMinTimeResolution: u32,
    pub dwFirmwareRevision: u32,
    pub dwHardwareRevision: u32,
    pub dwFFDriverVersion: u32,
}
impl ::core::marker::Copy for DIDEVCAPS {}
impl ::core::clone::Clone for DIDEVCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwDevType", &self.dwDevType)
            .field("dwAxes", &self.dwAxes)
            .field("dwButtons", &self.dwButtons)
            .field("dwPOVs", &self.dwPOVs)
            .field("dwFFSamplePeriod", &self.dwFFSamplePeriod)
            .field("dwFFMinTimeResolution", &self.dwFFMinTimeResolution)
            .field("dwFirmwareRevision", &self.dwFirmwareRevision)
            .field("dwHardwareRevision", &self.dwHardwareRevision)
            .field("dwFFDriverVersion", &self.dwFFDriverVersion)
            .finish()
    }
}
impl ::windows_core::TypeKind for DIDEVCAPS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDevType == other.dwDevType && self.dwAxes == other.dwAxes && self.dwButtons == other.dwButtons && self.dwPOVs == other.dwPOVs && self.dwFFSamplePeriod == other.dwFFSamplePeriod && self.dwFFMinTimeResolution == other.dwFFMinTimeResolution && self.dwFirmwareRevision == other.dwFirmwareRevision && self.dwHardwareRevision == other.dwHardwareRevision && self.dwFFDriverVersion == other.dwFFDriverVersion
    }
}
impl ::core::cmp::Eq for DIDEVCAPS {}
impl ::core::default::Default for DIDEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVCAPS_DX3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDevType: u32,
    pub dwAxes: u32,
    pub dwButtons: u32,
    pub dwPOVs: u32,
}
impl ::core::marker::Copy for DIDEVCAPS_DX3 {}
impl ::core::clone::Clone for DIDEVCAPS_DX3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVCAPS_DX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVCAPS_DX3").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDevType", &self.dwDevType).field("dwAxes", &self.dwAxes).field("dwButtons", &self.dwButtons).field("dwPOVs", &self.dwPOVs).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVCAPS_DX3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVCAPS_DX3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDevType == other.dwDevType && self.dwAxes == other.dwAxes && self.dwButtons == other.dwButtons && self.dwPOVs == other.dwPOVs
    }
}
impl ::core::cmp::Eq for DIDEVCAPS_DX3 {}
impl ::core::default::Default for DIDEVCAPS_DX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIDEVICEIMAGEINFOA {
    pub tszImagePath: [u8; 260],
    pub dwFlags: u32,
    pub dwViewID: u32,
    pub rcOverlay: super::super::Foundation::RECT,
    pub dwObjID: u32,
    pub dwcValidPts: u32,
    pub rgptCalloutLine: [super::super::Foundation::POINT; 5],
    pub rcCalloutRect: super::super::Foundation::RECT,
    pub dwTextAlign: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIDEVICEIMAGEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIDEVICEIMAGEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOA").field("tszImagePath", &self.tszImagePath).field("dwFlags", &self.dwFlags).field("dwViewID", &self.dwViewID).field("rcOverlay", &self.rcOverlay).field("dwObjID", &self.dwObjID).field("dwcValidPts", &self.dwcValidPts).field("rgptCalloutLine", &self.rgptCalloutLine).field("rcCalloutRect", &self.rcCalloutRect).field("dwTextAlign", &self.dwTextAlign).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DIDEVICEIMAGEINFOA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.tszImagePath == other.tszImagePath && self.dwFlags == other.dwFlags && self.dwViewID == other.dwViewID && self.rcOverlay == other.rcOverlay && self.dwObjID == other.dwObjID && self.dwcValidPts == other.dwcValidPts && self.rgptCalloutLine == other.rgptCalloutLine && self.rcCalloutRect == other.rcCalloutRect && self.dwTextAlign == other.dwTextAlign
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIDEVICEIMAGEINFOHEADERA {
    pub dwSize: u32,
    pub dwSizeImageInfo: u32,
    pub dwcViews: u32,
    pub dwcButtons: u32,
    pub dwcAxes: u32,
    pub dwcPOVs: u32,
    pub dwBufferSize: u32,
    pub dwBufferUsed: u32,
    pub lprgImageInfoArray: *mut DIDEVICEIMAGEINFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIDEVICEIMAGEINFOHEADERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIDEVICEIMAGEINFOHEADERA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOHEADERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOHEADERA").field("dwSize", &self.dwSize).field("dwSizeImageInfo", &self.dwSizeImageInfo).field("dwcViews", &self.dwcViews).field("dwcButtons", &self.dwcButtons).field("dwcAxes", &self.dwcAxes).field("dwcPOVs", &self.dwcPOVs).field("dwBufferSize", &self.dwBufferSize).field("dwBufferUsed", &self.dwBufferUsed).field("lprgImageInfoArray", &self.lprgImageInfoArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DIDEVICEIMAGEINFOHEADERA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOHEADERA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwSizeImageInfo == other.dwSizeImageInfo && self.dwcViews == other.dwcViews && self.dwcButtons == other.dwcButtons && self.dwcAxes == other.dwcAxes && self.dwcPOVs == other.dwcPOVs && self.dwBufferSize == other.dwBufferSize && self.dwBufferUsed == other.dwBufferUsed && self.lprgImageInfoArray == other.lprgImageInfoArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOHEADERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOHEADERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIDEVICEIMAGEINFOHEADERW {
    pub dwSize: u32,
    pub dwSizeImageInfo: u32,
    pub dwcViews: u32,
    pub dwcButtons: u32,
    pub dwcAxes: u32,
    pub dwcPOVs: u32,
    pub dwBufferSize: u32,
    pub dwBufferUsed: u32,
    pub lprgImageInfoArray: *mut DIDEVICEIMAGEINFOW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIDEVICEIMAGEINFOHEADERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIDEVICEIMAGEINFOHEADERW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOHEADERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOHEADERW").field("dwSize", &self.dwSize).field("dwSizeImageInfo", &self.dwSizeImageInfo).field("dwcViews", &self.dwcViews).field("dwcButtons", &self.dwcButtons).field("dwcAxes", &self.dwcAxes).field("dwcPOVs", &self.dwcPOVs).field("dwBufferSize", &self.dwBufferSize).field("dwBufferUsed", &self.dwBufferUsed).field("lprgImageInfoArray", &self.lprgImageInfoArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DIDEVICEIMAGEINFOHEADERW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOHEADERW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwSizeImageInfo == other.dwSizeImageInfo && self.dwcViews == other.dwcViews && self.dwcButtons == other.dwcButtons && self.dwcAxes == other.dwcAxes && self.dwcPOVs == other.dwcPOVs && self.dwBufferSize == other.dwBufferSize && self.dwBufferUsed == other.dwBufferUsed && self.lprgImageInfoArray == other.lprgImageInfoArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOHEADERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOHEADERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIDEVICEIMAGEINFOW {
    pub tszImagePath: [u16; 260],
    pub dwFlags: u32,
    pub dwViewID: u32,
    pub rcOverlay: super::super::Foundation::RECT,
    pub dwObjID: u32,
    pub dwcValidPts: u32,
    pub rgptCalloutLine: [super::super::Foundation::POINT; 5],
    pub rcCalloutRect: super::super::Foundation::RECT,
    pub dwTextAlign: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIDEVICEIMAGEINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIDEVICEIMAGEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOW").field("tszImagePath", &self.tszImagePath).field("dwFlags", &self.dwFlags).field("dwViewID", &self.dwViewID).field("rcOverlay", &self.rcOverlay).field("dwObjID", &self.dwObjID).field("dwcValidPts", &self.dwcValidPts).field("rgptCalloutLine", &self.rgptCalloutLine).field("rcCalloutRect", &self.rcCalloutRect).field("dwTextAlign", &self.dwTextAlign).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DIDEVICEIMAGEINFOW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.tszImagePath == other.tszImagePath && self.dwFlags == other.dwFlags && self.dwViewID == other.dwViewID && self.rcOverlay == other.rcOverlay && self.dwObjID == other.dwObjID && self.dwcValidPts == other.dwcValidPts && self.rgptCalloutLine == other.rgptCalloutLine && self.rcCalloutRect == other.rcCalloutRect && self.dwTextAlign == other.dwTextAlign
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEINSTANCEA {
    pub dwSize: u32,
    pub guidInstance: ::windows_core::GUID,
    pub guidProduct: ::windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u8; 260],
    pub tszProductName: [u8; 260],
    pub guidFFDriver: ::windows_core::GUID,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
impl ::core::marker::Copy for DIDEVICEINSTANCEA {}
impl ::core::clone::Clone for DIDEVICEINSTANCEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEINSTANCEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCEA").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).field("guidFFDriver", &self.guidFFDriver).field("wUsagePage", &self.wUsagePage).field("wUsage", &self.wUsage).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEINSTANCEA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEINSTANCEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName && self.guidFFDriver == other.guidFFDriver && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage
    }
}
impl ::core::cmp::Eq for DIDEVICEINSTANCEA {}
impl ::core::default::Default for DIDEVICEINSTANCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEINSTANCEW {
    pub dwSize: u32,
    pub guidInstance: ::windows_core::GUID,
    pub guidProduct: ::windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u16; 260],
    pub tszProductName: [u16; 260],
    pub guidFFDriver: ::windows_core::GUID,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
impl ::core::marker::Copy for DIDEVICEINSTANCEW {}
impl ::core::clone::Clone for DIDEVICEINSTANCEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEINSTANCEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCEW").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).field("guidFFDriver", &self.guidFFDriver).field("wUsagePage", &self.wUsagePage).field("wUsage", &self.wUsage).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEINSTANCEW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEINSTANCEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName && self.guidFFDriver == other.guidFFDriver && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage
    }
}
impl ::core::cmp::Eq for DIDEVICEINSTANCEW {}
impl ::core::default::Default for DIDEVICEINSTANCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEINSTANCE_DX3A {
    pub dwSize: u32,
    pub guidInstance: ::windows_core::GUID,
    pub guidProduct: ::windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u8; 260],
    pub tszProductName: [u8; 260],
}
impl ::core::marker::Copy for DIDEVICEINSTANCE_DX3A {}
impl ::core::clone::Clone for DIDEVICEINSTANCE_DX3A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEINSTANCE_DX3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCE_DX3A").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEINSTANCE_DX3A {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEINSTANCE_DX3A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName
    }
}
impl ::core::cmp::Eq for DIDEVICEINSTANCE_DX3A {}
impl ::core::default::Default for DIDEVICEINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEINSTANCE_DX3W {
    pub dwSize: u32,
    pub guidInstance: ::windows_core::GUID,
    pub guidProduct: ::windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u16; 260],
    pub tszProductName: [u16; 260],
}
impl ::core::marker::Copy for DIDEVICEINSTANCE_DX3W {}
impl ::core::clone::Clone for DIDEVICEINSTANCE_DX3W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEINSTANCE_DX3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCE_DX3W").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEINSTANCE_DX3W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEINSTANCE_DX3W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName
    }
}
impl ::core::cmp::Eq for DIDEVICEINSTANCE_DX3W {}
impl ::core::default::Default for DIDEVICEINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEOBJECTDATA {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
    pub uAppData: usize,
}
impl ::core::marker::Copy for DIDEVICEOBJECTDATA {}
impl ::core::clone::Clone for DIDEVICEOBJECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEOBJECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTDATA").field("dwOfs", &self.dwOfs).field("dwData", &self.dwData).field("dwTimeStamp", &self.dwTimeStamp).field("dwSequence", &self.dwSequence).field("uAppData", &self.uAppData).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEOBJECTDATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOfs == other.dwOfs && self.dwData == other.dwData && self.dwTimeStamp == other.dwTimeStamp && self.dwSequence == other.dwSequence && self.uAppData == other.uAppData
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTDATA {}
impl ::core::default::Default for DIDEVICEOBJECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEOBJECTDATA_DX3 {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
}
impl ::core::marker::Copy for DIDEVICEOBJECTDATA_DX3 {}
impl ::core::clone::Clone for DIDEVICEOBJECTDATA_DX3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEOBJECTDATA_DX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTDATA_DX3").field("dwOfs", &self.dwOfs).field("dwData", &self.dwData).field("dwTimeStamp", &self.dwTimeStamp).field("dwSequence", &self.dwSequence).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEOBJECTDATA_DX3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTDATA_DX3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwOfs == other.dwOfs && self.dwData == other.dwData && self.dwTimeStamp == other.dwTimeStamp && self.dwSequence == other.dwSequence
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTDATA_DX3 {}
impl ::core::default::Default for DIDEVICEOBJECTDATA_DX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEOBJECTINSTANCEA {
    pub dwSize: u32,
    pub guidType: ::windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u8; 260],
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
    pub wCollectionNumber: u16,
    pub wDesignatorIndex: u16,
    pub wUsagePage: u16,
    pub wUsage: u16,
    pub dwDimension: u32,
    pub wExponent: u16,
    pub wReportId: u16,
}
impl ::core::marker::Copy for DIDEVICEOBJECTINSTANCEA {}
impl ::core::clone::Clone for DIDEVICEOBJECTINSTANCEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCEA")
            .field("dwSize", &self.dwSize)
            .field("guidType", &self.guidType)
            .field("dwOfs", &self.dwOfs)
            .field("dwType", &self.dwType)
            .field("dwFlags", &self.dwFlags)
            .field("tszName", &self.tszName)
            .field("dwFFMaxForce", &self.dwFFMaxForce)
            .field("dwFFForceResolution", &self.dwFFForceResolution)
            .field("wCollectionNumber", &self.wCollectionNumber)
            .field("wDesignatorIndex", &self.wDesignatorIndex)
            .field("wUsagePage", &self.wUsagePage)
            .field("wUsage", &self.wUsage)
            .field("dwDimension", &self.dwDimension)
            .field("wExponent", &self.wExponent)
            .field("wReportId", &self.wReportId)
            .finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEOBJECTINSTANCEA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName && self.dwFFMaxForce == other.dwFFMaxForce && self.dwFFForceResolution == other.dwFFForceResolution && self.wCollectionNumber == other.wCollectionNumber && self.wDesignatorIndex == other.wDesignatorIndex && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage && self.dwDimension == other.dwDimension && self.wExponent == other.wExponent && self.wReportId == other.wReportId
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCEA {}
impl ::core::default::Default for DIDEVICEOBJECTINSTANCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEOBJECTINSTANCEW {
    pub dwSize: u32,
    pub guidType: ::windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u16; 260],
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
    pub wCollectionNumber: u16,
    pub wDesignatorIndex: u16,
    pub wUsagePage: u16,
    pub wUsage: u16,
    pub dwDimension: u32,
    pub wExponent: u16,
    pub wReportId: u16,
}
impl ::core::marker::Copy for DIDEVICEOBJECTINSTANCEW {}
impl ::core::clone::Clone for DIDEVICEOBJECTINSTANCEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCEW")
            .field("dwSize", &self.dwSize)
            .field("guidType", &self.guidType)
            .field("dwOfs", &self.dwOfs)
            .field("dwType", &self.dwType)
            .field("dwFlags", &self.dwFlags)
            .field("tszName", &self.tszName)
            .field("dwFFMaxForce", &self.dwFFMaxForce)
            .field("dwFFForceResolution", &self.dwFFForceResolution)
            .field("wCollectionNumber", &self.wCollectionNumber)
            .field("wDesignatorIndex", &self.wDesignatorIndex)
            .field("wUsagePage", &self.wUsagePage)
            .field("wUsage", &self.wUsage)
            .field("dwDimension", &self.dwDimension)
            .field("wExponent", &self.wExponent)
            .field("wReportId", &self.wReportId)
            .finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEOBJECTINSTANCEW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName && self.dwFFMaxForce == other.dwFFMaxForce && self.dwFFForceResolution == other.dwFFForceResolution && self.wCollectionNumber == other.wCollectionNumber && self.wDesignatorIndex == other.wDesignatorIndex && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage && self.dwDimension == other.dwDimension && self.wExponent == other.wExponent && self.wReportId == other.wReportId
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCEW {}
impl ::core::default::Default for DIDEVICEOBJECTINSTANCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEOBJECTINSTANCE_DX3A {
    pub dwSize: u32,
    pub guidType: ::windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u8; 260],
}
impl ::core::marker::Copy for DIDEVICEOBJECTINSTANCE_DX3A {}
impl ::core::clone::Clone for DIDEVICEOBJECTINSTANCE_DX3A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCE_DX3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCE_DX3A").field("dwSize", &self.dwSize).field("guidType", &self.guidType).field("dwOfs", &self.dwOfs).field("dwType", &self.dwType).field("dwFlags", &self.dwFlags).field("tszName", &self.tszName).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEOBJECTINSTANCE_DX3A {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCE_DX3A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCE_DX3A {}
impl ::core::default::Default for DIDEVICEOBJECTINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICEOBJECTINSTANCE_DX3W {
    pub dwSize: u32,
    pub guidType: ::windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u16; 260],
}
impl ::core::marker::Copy for DIDEVICEOBJECTINSTANCE_DX3W {}
impl ::core::clone::Clone for DIDEVICEOBJECTINSTANCE_DX3W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCE_DX3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCE_DX3W").field("dwSize", &self.dwSize).field("guidType", &self.guidType).field("dwOfs", &self.dwOfs).field("dwType", &self.dwType).field("dwFlags", &self.dwFlags).field("tszName", &self.tszName).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICEOBJECTINSTANCE_DX3W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCE_DX3W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCE_DX3W {}
impl ::core::default::Default for DIDEVICEOBJECTINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDEVICESTATE {
    pub dwSize: u32,
    pub dwState: u32,
    pub dwLoad: u32,
}
impl ::core::marker::Copy for DIDEVICESTATE {}
impl ::core::clone::Clone for DIDEVICESTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDEVICESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICESTATE").field("dwSize", &self.dwSize).field("dwState", &self.dwState).field("dwLoad", &self.dwLoad).finish()
    }
}
impl ::windows_core::TypeKind for DIDEVICESTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDEVICESTATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwState == other.dwState && self.dwLoad == other.dwLoad
    }
}
impl ::core::cmp::Eq for DIDEVICESTATE {}
impl ::core::default::Default for DIDEVICESTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIDRIVERVERSIONS {
    pub dwSize: u32,
    pub dwFirmwareRevision: u32,
    pub dwHardwareRevision: u32,
    pub dwFFDriverVersion: u32,
}
impl ::core::marker::Copy for DIDRIVERVERSIONS {}
impl ::core::clone::Clone for DIDRIVERVERSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIDRIVERVERSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDRIVERVERSIONS").field("dwSize", &self.dwSize).field("dwFirmwareRevision", &self.dwFirmwareRevision).field("dwHardwareRevision", &self.dwHardwareRevision).field("dwFFDriverVersion", &self.dwFFDriverVersion).finish()
    }
}
impl ::windows_core::TypeKind for DIDRIVERVERSIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIDRIVERVERSIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFirmwareRevision == other.dwFirmwareRevision && self.dwHardwareRevision == other.dwHardwareRevision && self.dwFFDriverVersion == other.dwFFDriverVersion
    }
}
impl ::core::cmp::Eq for DIDRIVERVERSIONS {}
impl ::core::default::Default for DIDRIVERVERSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIEFFECT {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDuration: u32,
    pub dwSamplePeriod: u32,
    pub dwGain: u32,
    pub dwTriggerButton: u32,
    pub dwTriggerRepeatInterval: u32,
    pub cAxes: u32,
    pub rgdwAxes: *mut u32,
    pub rglDirection: *mut i32,
    pub lpEnvelope: *mut DIENVELOPE,
    pub cbTypeSpecificParams: u32,
    pub lpvTypeSpecificParams: *mut ::core::ffi::c_void,
    pub dwStartDelay: u32,
}
impl ::core::marker::Copy for DIEFFECT {}
impl ::core::clone::Clone for DIEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECT")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwDuration", &self.dwDuration)
            .field("dwSamplePeriod", &self.dwSamplePeriod)
            .field("dwGain", &self.dwGain)
            .field("dwTriggerButton", &self.dwTriggerButton)
            .field("dwTriggerRepeatInterval", &self.dwTriggerRepeatInterval)
            .field("cAxes", &self.cAxes)
            .field("rgdwAxes", &self.rgdwAxes)
            .field("rglDirection", &self.rglDirection)
            .field("lpEnvelope", &self.lpEnvelope)
            .field("cbTypeSpecificParams", &self.cbTypeSpecificParams)
            .field("lpvTypeSpecificParams", &self.lpvTypeSpecificParams)
            .field("dwStartDelay", &self.dwStartDelay)
            .finish()
    }
}
impl ::windows_core::TypeKind for DIEFFECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIEFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDuration == other.dwDuration && self.dwSamplePeriod == other.dwSamplePeriod && self.dwGain == other.dwGain && self.dwTriggerButton == other.dwTriggerButton && self.dwTriggerRepeatInterval == other.dwTriggerRepeatInterval && self.cAxes == other.cAxes && self.rgdwAxes == other.rgdwAxes && self.rglDirection == other.rglDirection && self.lpEnvelope == other.lpEnvelope && self.cbTypeSpecificParams == other.cbTypeSpecificParams && self.lpvTypeSpecificParams == other.lpvTypeSpecificParams && self.dwStartDelay == other.dwStartDelay
    }
}
impl ::core::cmp::Eq for DIEFFECT {}
impl ::core::default::Default for DIEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIEFFECTATTRIBUTES {
    pub dwEffectId: u32,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub dwCoords: u32,
}
impl ::core::marker::Copy for DIEFFECTATTRIBUTES {}
impl ::core::clone::Clone for DIEFFECTATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIEFFECTATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECTATTRIBUTES").field("dwEffectId", &self.dwEffectId).field("dwEffType", &self.dwEffType).field("dwStaticParams", &self.dwStaticParams).field("dwDynamicParams", &self.dwDynamicParams).field("dwCoords", &self.dwCoords).finish()
    }
}
impl ::windows_core::TypeKind for DIEFFECTATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIEFFECTATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwEffectId == other.dwEffectId && self.dwEffType == other.dwEffType && self.dwStaticParams == other.dwStaticParams && self.dwDynamicParams == other.dwDynamicParams && self.dwCoords == other.dwCoords
    }
}
impl ::core::cmp::Eq for DIEFFECTATTRIBUTES {}
impl ::core::default::Default for DIEFFECTATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIEFFECTINFOA {
    pub dwSize: u32,
    pub guid: ::windows_core::GUID,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub tszName: [u8; 260],
}
impl ::core::marker::Copy for DIEFFECTINFOA {}
impl ::core::clone::Clone for DIEFFECTINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIEFFECTINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECTINFOA").field("dwSize", &self.dwSize).field("guid", &self.guid).field("dwEffType", &self.dwEffType).field("dwStaticParams", &self.dwStaticParams).field("dwDynamicParams", &self.dwDynamicParams).field("tszName", &self.tszName).finish()
    }
}
impl ::windows_core::TypeKind for DIEFFECTINFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIEFFECTINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guid == other.guid && self.dwEffType == other.dwEffType && self.dwStaticParams == other.dwStaticParams && self.dwDynamicParams == other.dwDynamicParams && self.tszName == other.tszName
    }
}
impl ::core::cmp::Eq for DIEFFECTINFOA {}
impl ::core::default::Default for DIEFFECTINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIEFFECTINFOW {
    pub dwSize: u32,
    pub guid: ::windows_core::GUID,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub tszName: [u16; 260],
}
impl ::core::marker::Copy for DIEFFECTINFOW {}
impl ::core::clone::Clone for DIEFFECTINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIEFFECTINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECTINFOW").field("dwSize", &self.dwSize).field("guid", &self.guid).field("dwEffType", &self.dwEffType).field("dwStaticParams", &self.dwStaticParams).field("dwDynamicParams", &self.dwDynamicParams).field("tszName", &self.tszName).finish()
    }
}
impl ::windows_core::TypeKind for DIEFFECTINFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIEFFECTINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guid == other.guid && self.dwEffType == other.dwEffType && self.dwStaticParams == other.dwStaticParams && self.dwDynamicParams == other.dwDynamicParams && self.tszName == other.tszName
    }
}
impl ::core::cmp::Eq for DIEFFECTINFOW {}
impl ::core::default::Default for DIEFFECTINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIEFFECT_DX5 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDuration: u32,
    pub dwSamplePeriod: u32,
    pub dwGain: u32,
    pub dwTriggerButton: u32,
    pub dwTriggerRepeatInterval: u32,
    pub cAxes: u32,
    pub rgdwAxes: *mut u32,
    pub rglDirection: *mut i32,
    pub lpEnvelope: *mut DIENVELOPE,
    pub cbTypeSpecificParams: u32,
    pub lpvTypeSpecificParams: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DIEFFECT_DX5 {}
impl ::core::clone::Clone for DIEFFECT_DX5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIEFFECT_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECT_DX5")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwDuration", &self.dwDuration)
            .field("dwSamplePeriod", &self.dwSamplePeriod)
            .field("dwGain", &self.dwGain)
            .field("dwTriggerButton", &self.dwTriggerButton)
            .field("dwTriggerRepeatInterval", &self.dwTriggerRepeatInterval)
            .field("cAxes", &self.cAxes)
            .field("rgdwAxes", &self.rgdwAxes)
            .field("rglDirection", &self.rglDirection)
            .field("lpEnvelope", &self.lpEnvelope)
            .field("cbTypeSpecificParams", &self.cbTypeSpecificParams)
            .field("lpvTypeSpecificParams", &self.lpvTypeSpecificParams)
            .finish()
    }
}
impl ::windows_core::TypeKind for DIEFFECT_DX5 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIEFFECT_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDuration == other.dwDuration && self.dwSamplePeriod == other.dwSamplePeriod && self.dwGain == other.dwGain && self.dwTriggerButton == other.dwTriggerButton && self.dwTriggerRepeatInterval == other.dwTriggerRepeatInterval && self.cAxes == other.cAxes && self.rgdwAxes == other.rgdwAxes && self.rglDirection == other.rglDirection && self.lpEnvelope == other.lpEnvelope && self.cbTypeSpecificParams == other.cbTypeSpecificParams && self.lpvTypeSpecificParams == other.lpvTypeSpecificParams
    }
}
impl ::core::cmp::Eq for DIEFFECT_DX5 {}
impl ::core::default::Default for DIEFFECT_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIEFFESCAPE {
    pub dwSize: u32,
    pub dwCommand: u32,
    pub lpvInBuffer: *mut ::core::ffi::c_void,
    pub cbInBuffer: u32,
    pub lpvOutBuffer: *mut ::core::ffi::c_void,
    pub cbOutBuffer: u32,
}
impl ::core::marker::Copy for DIEFFESCAPE {}
impl ::core::clone::Clone for DIEFFESCAPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIEFFESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFESCAPE").field("dwSize", &self.dwSize).field("dwCommand", &self.dwCommand).field("lpvInBuffer", &self.lpvInBuffer).field("cbInBuffer", &self.cbInBuffer).field("lpvOutBuffer", &self.lpvOutBuffer).field("cbOutBuffer", &self.cbOutBuffer).finish()
    }
}
impl ::windows_core::TypeKind for DIEFFESCAPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIEFFESCAPE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCommand == other.dwCommand && self.lpvInBuffer == other.lpvInBuffer && self.cbInBuffer == other.cbInBuffer && self.lpvOutBuffer == other.lpvOutBuffer && self.cbOutBuffer == other.cbOutBuffer
    }
}
impl ::core::cmp::Eq for DIEFFESCAPE {}
impl ::core::default::Default for DIEFFESCAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIENVELOPE {
    pub dwSize: u32,
    pub dwAttackLevel: u32,
    pub dwAttackTime: u32,
    pub dwFadeLevel: u32,
    pub dwFadeTime: u32,
}
impl ::core::marker::Copy for DIENVELOPE {}
impl ::core::clone::Clone for DIENVELOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIENVELOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIENVELOPE").field("dwSize", &self.dwSize).field("dwAttackLevel", &self.dwAttackLevel).field("dwAttackTime", &self.dwAttackTime).field("dwFadeLevel", &self.dwFadeLevel).field("dwFadeTime", &self.dwFadeTime).finish()
    }
}
impl ::windows_core::TypeKind for DIENVELOPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIENVELOPE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwAttackLevel == other.dwAttackLevel && self.dwAttackTime == other.dwAttackTime && self.dwFadeLevel == other.dwFadeLevel && self.dwFadeTime == other.dwFadeTime
    }
}
impl ::core::cmp::Eq for DIENVELOPE {}
impl ::core::default::Default for DIENVELOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIFFDEVICEATTRIBUTES {
    pub dwFlags: u32,
    pub dwFFSamplePeriod: u32,
    pub dwFFMinTimeResolution: u32,
}
impl ::core::marker::Copy for DIFFDEVICEATTRIBUTES {}
impl ::core::clone::Clone for DIFFDEVICEATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIFFDEVICEATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIFFDEVICEATTRIBUTES").field("dwFlags", &self.dwFlags).field("dwFFSamplePeriod", &self.dwFFSamplePeriod).field("dwFFMinTimeResolution", &self.dwFFMinTimeResolution).finish()
    }
}
impl ::windows_core::TypeKind for DIFFDEVICEATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIFFDEVICEATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwFFSamplePeriod == other.dwFFSamplePeriod && self.dwFFMinTimeResolution == other.dwFFMinTimeResolution
    }
}
impl ::core::cmp::Eq for DIFFDEVICEATTRIBUTES {}
impl ::core::default::Default for DIFFDEVICEATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIFFOBJECTATTRIBUTES {
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
}
impl ::core::marker::Copy for DIFFOBJECTATTRIBUTES {}
impl ::core::clone::Clone for DIFFOBJECTATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIFFOBJECTATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIFFOBJECTATTRIBUTES").field("dwFFMaxForce", &self.dwFFMaxForce).field("dwFFForceResolution", &self.dwFFForceResolution).finish()
    }
}
impl ::windows_core::TypeKind for DIFFOBJECTATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIFFOBJECTATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFFMaxForce == other.dwFFMaxForce && self.dwFFForceResolution == other.dwFFForceResolution
    }
}
impl ::core::cmp::Eq for DIFFOBJECTATTRIBUTES {}
impl ::core::default::Default for DIFFOBJECTATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIFILEEFFECT {
    pub dwSize: u32,
    pub GuidEffect: ::windows_core::GUID,
    pub lpDiEffect: *mut DIEFFECT,
    pub szFriendlyName: [u8; 260],
}
impl ::core::marker::Copy for DIFILEEFFECT {}
impl ::core::clone::Clone for DIFILEEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIFILEEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIFILEEFFECT").field("dwSize", &self.dwSize).field("GuidEffect", &self.GuidEffect).field("lpDiEffect", &self.lpDiEffect).field("szFriendlyName", &self.szFriendlyName).finish()
    }
}
impl ::windows_core::TypeKind for DIFILEEFFECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIFILEEFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.GuidEffect == other.GuidEffect && self.lpDiEffect == other.lpDiEffect && self.szFriendlyName == other.szFriendlyName
    }
}
impl ::core::cmp::Eq for DIFILEEFFECT {}
impl ::core::default::Default for DIFILEEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIHIDFFINITINFO {
    pub dwSize: u32,
    pub pwszDeviceInterface: ::windows_core::PWSTR,
    pub GuidInstance: ::windows_core::GUID,
}
impl ::core::marker::Copy for DIHIDFFINITINFO {}
impl ::core::clone::Clone for DIHIDFFINITINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIHIDFFINITINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIHIDFFINITINFO").field("dwSize", &self.dwSize).field("pwszDeviceInterface", &self.pwszDeviceInterface).field("GuidInstance", &self.GuidInstance).finish()
    }
}
impl ::windows_core::TypeKind for DIHIDFFINITINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIHIDFFINITINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pwszDeviceInterface == other.pwszDeviceInterface && self.GuidInstance == other.GuidInstance
    }
}
impl ::core::cmp::Eq for DIHIDFFINITINFO {}
impl ::core::default::Default for DIHIDFFINITINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYCONFIG {
    pub dwSize: u32,
    pub guidInstance: ::windows_core::GUID,
    pub hwc: JOYREGHWCONFIG,
    pub dwGain: u32,
    pub wszType: [u16; 256],
    pub wszCallout: [u16; 256],
    pub guidGameport: ::windows_core::GUID,
}
impl ::core::marker::Copy for DIJOYCONFIG {}
impl ::core::clone::Clone for DIJOYCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYCONFIG").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("hwc", &self.hwc).field("dwGain", &self.dwGain).field("wszType", &self.wszType).field("wszCallout", &self.wszCallout).field("guidGameport", &self.guidGameport).finish()
    }
}
impl ::windows_core::TypeKind for DIJOYCONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.hwc == other.hwc && self.dwGain == other.dwGain && self.wszType == other.wszType && self.wszCallout == other.wszCallout && self.guidGameport == other.guidGameport
    }
}
impl ::core::cmp::Eq for DIJOYCONFIG {}
impl ::core::default::Default for DIJOYCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYCONFIG_DX5 {
    pub dwSize: u32,
    pub guidInstance: ::windows_core::GUID,
    pub hwc: JOYREGHWCONFIG,
    pub dwGain: u32,
    pub wszType: [u16; 256],
    pub wszCallout: [u16; 256],
}
impl ::core::marker::Copy for DIJOYCONFIG_DX5 {}
impl ::core::clone::Clone for DIJOYCONFIG_DX5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYCONFIG_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYCONFIG_DX5").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("hwc", &self.hwc).field("dwGain", &self.dwGain).field("wszType", &self.wszType).field("wszCallout", &self.wszCallout).finish()
    }
}
impl ::windows_core::TypeKind for DIJOYCONFIG_DX5 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYCONFIG_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.hwc == other.hwc && self.dwGain == other.dwGain && self.wszType == other.wszType && self.wszCallout == other.wszCallout
    }
}
impl ::core::cmp::Eq for DIJOYCONFIG_DX5 {}
impl ::core::default::Default for DIJOYCONFIG_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYSTATE {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub lRx: i32,
    pub lRy: i32,
    pub lRz: i32,
    pub rglSlider: [i32; 2],
    pub rgdwPOV: [u32; 4],
    pub rgbButtons: [u8; 32],
}
impl ::core::marker::Copy for DIJOYSTATE {}
impl ::core::clone::Clone for DIJOYSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYSTATE").field("lX", &self.lX).field("lY", &self.lY).field("lZ", &self.lZ).field("lRx", &self.lRx).field("lRy", &self.lRy).field("lRz", &self.lRz).field("rglSlider", &self.rglSlider).field("rgdwPOV", &self.rgdwPOV).field("rgbButtons", &self.rgbButtons).finish()
    }
}
impl ::windows_core::TypeKind for DIJOYSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX && self.lY == other.lY && self.lZ == other.lZ && self.lRx == other.lRx && self.lRy == other.lRy && self.lRz == other.lRz && self.rglSlider == other.rglSlider && self.rgdwPOV == other.rgdwPOV && self.rgbButtons == other.rgbButtons
    }
}
impl ::core::cmp::Eq for DIJOYSTATE {}
impl ::core::default::Default for DIJOYSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYSTATE2 {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub lRx: i32,
    pub lRy: i32,
    pub lRz: i32,
    pub rglSlider: [i32; 2],
    pub rgdwPOV: [u32; 4],
    pub rgbButtons: [u8; 128],
    pub lVX: i32,
    pub lVY: i32,
    pub lVZ: i32,
    pub lVRx: i32,
    pub lVRy: i32,
    pub lVRz: i32,
    pub rglVSlider: [i32; 2],
    pub lAX: i32,
    pub lAY: i32,
    pub lAZ: i32,
    pub lARx: i32,
    pub lARy: i32,
    pub lARz: i32,
    pub rglASlider: [i32; 2],
    pub lFX: i32,
    pub lFY: i32,
    pub lFZ: i32,
    pub lFRx: i32,
    pub lFRy: i32,
    pub lFRz: i32,
    pub rglFSlider: [i32; 2],
}
impl ::core::marker::Copy for DIJOYSTATE2 {}
impl ::core::clone::Clone for DIJOYSTATE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYSTATE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYSTATE2")
            .field("lX", &self.lX)
            .field("lY", &self.lY)
            .field("lZ", &self.lZ)
            .field("lRx", &self.lRx)
            .field("lRy", &self.lRy)
            .field("lRz", &self.lRz)
            .field("rglSlider", &self.rglSlider)
            .field("rgdwPOV", &self.rgdwPOV)
            .field("rgbButtons", &self.rgbButtons)
            .field("lVX", &self.lVX)
            .field("lVY", &self.lVY)
            .field("lVZ", &self.lVZ)
            .field("lVRx", &self.lVRx)
            .field("lVRy", &self.lVRy)
            .field("lVRz", &self.lVRz)
            .field("rglVSlider", &self.rglVSlider)
            .field("lAX", &self.lAX)
            .field("lAY", &self.lAY)
            .field("lAZ", &self.lAZ)
            .field("lARx", &self.lARx)
            .field("lARy", &self.lARy)
            .field("lARz", &self.lARz)
            .field("rglASlider", &self.rglASlider)
            .field("lFX", &self.lFX)
            .field("lFY", &self.lFY)
            .field("lFZ", &self.lFZ)
            .field("lFRx", &self.lFRx)
            .field("lFRy", &self.lFRy)
            .field("lFRz", &self.lFRz)
            .field("rglFSlider", &self.rglFSlider)
            .finish()
    }
}
impl ::windows_core::TypeKind for DIJOYSTATE2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYSTATE2 {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX
            && self.lY == other.lY
            && self.lZ == other.lZ
            && self.lRx == other.lRx
            && self.lRy == other.lRy
            && self.lRz == other.lRz
            && self.rglSlider == other.rglSlider
            && self.rgdwPOV == other.rgdwPOV
            && self.rgbButtons == other.rgbButtons
            && self.lVX == other.lVX
            && self.lVY == other.lVY
            && self.lVZ == other.lVZ
            && self.lVRx == other.lVRx
            && self.lVRy == other.lVRy
            && self.lVRz == other.lVRz
            && self.rglVSlider == other.rglVSlider
            && self.lAX == other.lAX
            && self.lAY == other.lAY
            && self.lAZ == other.lAZ
            && self.lARx == other.lARx
            && self.lARy == other.lARy
            && self.lARz == other.lARz
            && self.rglASlider == other.rglASlider
            && self.lFX == other.lFX
            && self.lFY == other.lFY
            && self.lFZ == other.lFZ
            && self.lFRx == other.lFRx
            && self.lFRy == other.lFRy
            && self.lFRz == other.lFRz
            && self.rglFSlider == other.rglFSlider
    }
}
impl ::core::cmp::Eq for DIJOYSTATE2 {}
impl ::core::default::Default for DIJOYSTATE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYTYPEINFO {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: ::windows_core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
    pub wszHardwareId: [u16; 256],
    pub dwFlags1: u32,
    pub dwFlags2: u32,
    pub wszMapFile: [u16; 256],
}
impl ::core::marker::Copy for DIJOYTYPEINFO {}
impl ::core::clone::Clone for DIJOYTYPEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYTYPEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYTYPEINFO").field("dwSize", &self.dwSize).field("hws", &self.hws).field("clsidConfig", &self.clsidConfig).field("wszDisplayName", &self.wszDisplayName).field("wszCallout", &self.wszCallout).field("wszHardwareId", &self.wszHardwareId).field("dwFlags1", &self.dwFlags1).field("dwFlags2", &self.dwFlags2).field("wszMapFile", &self.wszMapFile).finish()
    }
}
impl ::windows_core::TypeKind for DIJOYTYPEINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYTYPEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hws == other.hws && self.clsidConfig == other.clsidConfig && self.wszDisplayName == other.wszDisplayName && self.wszCallout == other.wszCallout && self.wszHardwareId == other.wszHardwareId && self.dwFlags1 == other.dwFlags1 && self.dwFlags2 == other.dwFlags2 && self.wszMapFile == other.wszMapFile
    }
}
impl ::core::cmp::Eq for DIJOYTYPEINFO {}
impl ::core::default::Default for DIJOYTYPEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYTYPEINFO_DX5 {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: ::windows_core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
}
impl ::core::marker::Copy for DIJOYTYPEINFO_DX5 {}
impl ::core::clone::Clone for DIJOYTYPEINFO_DX5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYTYPEINFO_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYTYPEINFO_DX5").field("dwSize", &self.dwSize).field("hws", &self.hws).field("clsidConfig", &self.clsidConfig).field("wszDisplayName", &self.wszDisplayName).field("wszCallout", &self.wszCallout).finish()
    }
}
impl ::windows_core::TypeKind for DIJOYTYPEINFO_DX5 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYTYPEINFO_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hws == other.hws && self.clsidConfig == other.clsidConfig && self.wszDisplayName == other.wszDisplayName && self.wszCallout == other.wszCallout
    }
}
impl ::core::cmp::Eq for DIJOYTYPEINFO_DX5 {}
impl ::core::default::Default for DIJOYTYPEINFO_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYTYPEINFO_DX6 {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: ::windows_core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
    pub wszHardwareId: [u16; 256],
    pub dwFlags1: u32,
}
impl ::core::marker::Copy for DIJOYTYPEINFO_DX6 {}
impl ::core::clone::Clone for DIJOYTYPEINFO_DX6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYTYPEINFO_DX6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYTYPEINFO_DX6").field("dwSize", &self.dwSize).field("hws", &self.hws).field("clsidConfig", &self.clsidConfig).field("wszDisplayName", &self.wszDisplayName).field("wszCallout", &self.wszCallout).field("wszHardwareId", &self.wszHardwareId).field("dwFlags1", &self.dwFlags1).finish()
    }
}
impl ::windows_core::TypeKind for DIJOYTYPEINFO_DX6 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYTYPEINFO_DX6 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hws == other.hws && self.clsidConfig == other.clsidConfig && self.wszDisplayName == other.wszDisplayName && self.wszCallout == other.wszCallout && self.wszHardwareId == other.wszHardwareId && self.dwFlags1 == other.dwFlags1
    }
}
impl ::core::cmp::Eq for DIJOYTYPEINFO_DX6 {}
impl ::core::default::Default for DIJOYTYPEINFO_DX6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIJOYUSERVALUES {
    pub dwSize: u32,
    pub ruv: JOYREGUSERVALUES,
    pub wszGlobalDriver: [u16; 256],
    pub wszGameportEmulator: [u16; 256],
}
impl ::core::marker::Copy for DIJOYUSERVALUES {}
impl ::core::clone::Clone for DIJOYUSERVALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIJOYUSERVALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYUSERVALUES").field("dwSize", &self.dwSize).field("ruv", &self.ruv).field("wszGlobalDriver", &self.wszGlobalDriver).field("wszGameportEmulator", &self.wszGameportEmulator).finish()
    }
}
impl ::windows_core::TypeKind for DIJOYUSERVALUES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIJOYUSERVALUES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ruv == other.ruv && self.wszGlobalDriver == other.wszGlobalDriver && self.wszGameportEmulator == other.wszGameportEmulator
    }
}
impl ::core::cmp::Eq for DIJOYUSERVALUES {}
impl ::core::default::Default for DIJOYUSERVALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIMOUSESTATE {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub rgbButtons: [u8; 4],
}
impl ::core::marker::Copy for DIMOUSESTATE {}
impl ::core::clone::Clone for DIMOUSESTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIMOUSESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIMOUSESTATE").field("lX", &self.lX).field("lY", &self.lY).field("lZ", &self.lZ).field("rgbButtons", &self.rgbButtons).finish()
    }
}
impl ::windows_core::TypeKind for DIMOUSESTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIMOUSESTATE {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX && self.lY == other.lY && self.lZ == other.lZ && self.rgbButtons == other.rgbButtons
    }
}
impl ::core::cmp::Eq for DIMOUSESTATE {}
impl ::core::default::Default for DIMOUSESTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIMOUSESTATE2 {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub rgbButtons: [u8; 8],
}
impl ::core::marker::Copy for DIMOUSESTATE2 {}
impl ::core::clone::Clone for DIMOUSESTATE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIMOUSESTATE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIMOUSESTATE2").field("lX", &self.lX).field("lY", &self.lY).field("lZ", &self.lZ).field("rgbButtons", &self.rgbButtons).finish()
    }
}
impl ::windows_core::TypeKind for DIMOUSESTATE2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIMOUSESTATE2 {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX && self.lY == other.lY && self.lZ == other.lZ && self.rgbButtons == other.rgbButtons
    }
}
impl ::core::cmp::Eq for DIMOUSESTATE2 {}
impl ::core::default::Default for DIMOUSESTATE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIOBJECTATTRIBUTES {
    pub dwFlags: u32,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
impl ::core::marker::Copy for DIOBJECTATTRIBUTES {}
impl ::core::clone::Clone for DIOBJECTATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIOBJECTATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIOBJECTATTRIBUTES").field("dwFlags", &self.dwFlags).field("wUsagePage", &self.wUsagePage).field("wUsage", &self.wUsage).finish()
    }
}
impl ::windows_core::TypeKind for DIOBJECTATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIOBJECTATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage
    }
}
impl ::core::cmp::Eq for DIOBJECTATTRIBUTES {}
impl ::core::default::Default for DIOBJECTATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIOBJECTCALIBRATION {
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
impl ::core::marker::Copy for DIOBJECTCALIBRATION {}
impl ::core::clone::Clone for DIOBJECTCALIBRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIOBJECTCALIBRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIOBJECTCALIBRATION").field("lMin", &self.lMin).field("lCenter", &self.lCenter).field("lMax", &self.lMax).finish()
    }
}
impl ::windows_core::TypeKind for DIOBJECTCALIBRATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIOBJECTCALIBRATION {
    fn eq(&self, other: &Self) -> bool {
        self.lMin == other.lMin && self.lCenter == other.lCenter && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIOBJECTCALIBRATION {}
impl ::core::default::Default for DIOBJECTCALIBRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIOBJECTDATAFORMAT {
    pub pguid: *const ::windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DIOBJECTDATAFORMAT {}
impl ::core::clone::Clone for DIOBJECTDATAFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIOBJECTDATAFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIOBJECTDATAFORMAT").field("pguid", &self.pguid).field("dwOfs", &self.dwOfs).field("dwType", &self.dwType).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows_core::TypeKind for DIOBJECTDATAFORMAT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIOBJECTDATAFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DIOBJECTDATAFORMAT {}
impl ::core::default::Default for DIOBJECTDATAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPERIODIC {
    pub dwMagnitude: u32,
    pub lOffset: i32,
    pub dwPhase: u32,
    pub dwPeriod: u32,
}
impl ::core::marker::Copy for DIPERIODIC {}
impl ::core::clone::Clone for DIPERIODIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPERIODIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPERIODIC").field("dwMagnitude", &self.dwMagnitude).field("lOffset", &self.lOffset).field("dwPhase", &self.dwPhase).field("dwPeriod", &self.dwPeriod).finish()
    }
}
impl ::windows_core::TypeKind for DIPERIODIC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPERIODIC {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagnitude == other.dwMagnitude && self.lOffset == other.lOffset && self.dwPhase == other.dwPhase && self.dwPeriod == other.dwPeriod
    }
}
impl ::core::cmp::Eq for DIPERIODIC {}
impl ::core::default::Default for DIPERIODIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPOVCALIBRATION {
    pub lMin: [i32; 5],
    pub lMax: [i32; 5],
}
impl ::core::marker::Copy for DIPOVCALIBRATION {}
impl ::core::clone::Clone for DIPOVCALIBRATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPOVCALIBRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPOVCALIBRATION").field("lMin", &self.lMin).field("lMax", &self.lMax).finish()
    }
}
impl ::windows_core::TypeKind for DIPOVCALIBRATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPOVCALIBRATION {
    fn eq(&self, other: &Self) -> bool {
        self.lMin == other.lMin && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPOVCALIBRATION {}
impl ::core::default::Default for DIPOVCALIBRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPCAL {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
impl ::core::marker::Copy for DIPROPCAL {}
impl ::core::clone::Clone for DIPROPCAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPCAL").field("diph", &self.diph).field("lMin", &self.lMin).field("lCenter", &self.lCenter).field("lMax", &self.lMax).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPCAL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPCAL {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.lMin == other.lMin && self.lCenter == other.lCenter && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPROPCAL {}
impl ::core::default::Default for DIPROPCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPCALPOV {
    pub diph: DIPROPHEADER,
    pub lMin: [i32; 5],
    pub lMax: [i32; 5],
}
impl ::core::marker::Copy for DIPROPCALPOV {}
impl ::core::clone::Clone for DIPROPCALPOV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPCALPOV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPCALPOV").field("diph", &self.diph).field("lMin", &self.lMin).field("lMax", &self.lMax).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPCALPOV {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPCALPOV {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.lMin == other.lMin && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPROPCALPOV {}
impl ::core::default::Default for DIPROPCALPOV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPCPOINTS {
    pub diph: DIPROPHEADER,
    pub dwCPointsNum: u32,
    pub cp: [CPOINT; 8],
}
impl ::core::marker::Copy for DIPROPCPOINTS {}
impl ::core::clone::Clone for DIPROPCPOINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPCPOINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPCPOINTS").field("diph", &self.diph).field("dwCPointsNum", &self.dwCPointsNum).field("cp", &self.cp).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPCPOINTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPCPOINTS {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.dwCPointsNum == other.dwCPointsNum && self.cp == other.cp
    }
}
impl ::core::cmp::Eq for DIPROPCPOINTS {}
impl ::core::default::Default for DIPROPCPOINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPDWORD {
    pub diph: DIPROPHEADER,
    pub dwData: u32,
}
impl ::core::marker::Copy for DIPROPDWORD {}
impl ::core::clone::Clone for DIPROPDWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPDWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPDWORD").field("diph", &self.diph).field("dwData", &self.dwData).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPDWORD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPDWORD {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.dwData == other.dwData
    }
}
impl ::core::cmp::Eq for DIPROPDWORD {}
impl ::core::default::Default for DIPROPDWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPGUIDANDPATH {
    pub diph: DIPROPHEADER,
    pub guidClass: ::windows_core::GUID,
    pub wszPath: [u16; 260],
}
impl ::core::marker::Copy for DIPROPGUIDANDPATH {}
impl ::core::clone::Clone for DIPROPGUIDANDPATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPGUIDANDPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPGUIDANDPATH").field("diph", &self.diph).field("guidClass", &self.guidClass).field("wszPath", &self.wszPath).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPGUIDANDPATH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPGUIDANDPATH {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.guidClass == other.guidClass && self.wszPath == other.wszPath
    }
}
impl ::core::cmp::Eq for DIPROPGUIDANDPATH {}
impl ::core::default::Default for DIPROPGUIDANDPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPHEADER {
    pub dwSize: u32,
    pub dwHeaderSize: u32,
    pub dwObj: u32,
    pub dwHow: u32,
}
impl ::core::marker::Copy for DIPROPHEADER {}
impl ::core::clone::Clone for DIPROPHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPHEADER").field("dwSize", &self.dwSize).field("dwHeaderSize", &self.dwHeaderSize).field("dwObj", &self.dwObj).field("dwHow", &self.dwHow).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPHEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeaderSize == other.dwHeaderSize && self.dwObj == other.dwObj && self.dwHow == other.dwHow
    }
}
impl ::core::cmp::Eq for DIPROPHEADER {}
impl ::core::default::Default for DIPROPHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPPOINTER {
    pub diph: DIPROPHEADER,
    pub uData: usize,
}
impl ::core::marker::Copy for DIPROPPOINTER {}
impl ::core::clone::Clone for DIPROPPOINTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPPOINTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPPOINTER").field("diph", &self.diph).field("uData", &self.uData).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPPOINTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPPOINTER {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.uData == other.uData
    }
}
impl ::core::cmp::Eq for DIPROPPOINTER {}
impl ::core::default::Default for DIPROPPOINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPRANGE {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lMax: i32,
}
impl ::core::marker::Copy for DIPROPRANGE {}
impl ::core::clone::Clone for DIPROPRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPRANGE").field("diph", &self.diph).field("lMin", &self.lMin).field("lMax", &self.lMax).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPRANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.lMin == other.lMin && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPROPRANGE {}
impl ::core::default::Default for DIPROPRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIPROPSTRING {
    pub diph: DIPROPHEADER,
    pub wsz: [u16; 260],
}
impl ::core::marker::Copy for DIPROPSTRING {}
impl ::core::clone::Clone for DIPROPSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIPROPSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPSTRING").field("diph", &self.diph).field("wsz", &self.wsz).finish()
    }
}
impl ::windows_core::TypeKind for DIPROPSTRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIPROPSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.wsz == other.wsz
    }
}
impl ::core::cmp::Eq for DIPROPSTRING {}
impl ::core::default::Default for DIPROPSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DIRAMPFORCE {
    pub lStart: i32,
    pub lEnd: i32,
}
impl ::core::marker::Copy for DIRAMPFORCE {}
impl ::core::clone::Clone for DIRAMPFORCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIRAMPFORCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIRAMPFORCE").field("lStart", &self.lStart).field("lEnd", &self.lEnd).finish()
    }
}
impl ::windows_core::TypeKind for DIRAMPFORCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DIRAMPFORCE {
    fn eq(&self, other: &Self) -> bool {
        self.lStart == other.lStart && self.lEnd == other.lEnd
    }
}
impl ::core::cmp::Eq for DIRAMPFORCE {}
impl ::core::default::Default for DIRAMPFORCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HIDD_ATTRIBUTES {
    pub Size: u32,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
impl ::core::marker::Copy for HIDD_ATTRIBUTES {}
impl ::core::clone::Clone for HIDD_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HIDD_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDD_ATTRIBUTES").field("Size", &self.Size).field("VendorID", &self.VendorID).field("ProductID", &self.ProductID).field("VersionNumber", &self.VersionNumber).finish()
    }
}
impl ::windows_core::TypeKind for HIDD_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HIDD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.VendorID == other.VendorID && self.ProductID == other.ProductID && self.VersionNumber == other.VersionNumber
    }
}
impl ::core::cmp::Eq for HIDD_ATTRIBUTES {}
impl ::core::default::Default for HIDD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
pub struct HIDD_CONFIGURATION {
    pub cookie: *mut ::core::ffi::c_void,
    pub size: u32,
    pub RingBufferSize: u32,
}
impl ::core::marker::Copy for HIDD_CONFIGURATION {}
impl ::core::clone::Clone for HIDD_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for HIDD_CONFIGURATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for HIDD_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_BUTTON_ARRAY_DATA {
    pub ArrayIndex: u16,
    pub On: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_BUTTON_ARRAY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_BUTTON_ARRAY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIDP_BUTTON_ARRAY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_BUTTON_ARRAY_DATA").field("ArrayIndex", &self.ArrayIndex).field("On", &self.On).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_BUTTON_ARRAY_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIDP_BUTTON_ARRAY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ArrayIndex == other.ArrayIndex && self.On == other.On
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIDP_BUTTON_ARRAY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_BUTTON_ARRAY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_BUTTON_CAPS {
    pub UsagePage: u16,
    pub ReportID: u8,
    pub IsAlias: super::super::Foundation::BOOLEAN,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub IsRange: super::super::Foundation::BOOLEAN,
    pub IsStringRange: super::super::Foundation::BOOLEAN,
    pub IsDesignatorRange: super::super::Foundation::BOOLEAN,
    pub IsAbsolute: super::super::Foundation::BOOLEAN,
    pub ReportCount: u16,
    pub Reserved2: u16,
    pub Reserved: [u32; 9],
    pub Anonymous: HIDP_BUTTON_CAPS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_BUTTON_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_BUTTON_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_BUTTON_CAPS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_BUTTON_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union HIDP_BUTTON_CAPS_0 {
    pub Range: HIDP_BUTTON_CAPS_0_1,
    pub NotRange: HIDP_BUTTON_CAPS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_BUTTON_CAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_BUTTON_CAPS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_BUTTON_CAPS_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_BUTTON_CAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_BUTTON_CAPS_0_0 {
    pub Usage: u16,
    pub Reserved1: u16,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_BUTTON_CAPS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_BUTTON_CAPS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIDP_BUTTON_CAPS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_BUTTON_CAPS_0_0").field("Usage", &self.Usage).field("Reserved1", &self.Reserved1).field("StringIndex", &self.StringIndex).field("Reserved2", &self.Reserved2).field("DesignatorIndex", &self.DesignatorIndex).field("Reserved3", &self.Reserved3).field("DataIndex", &self.DataIndex).field("Reserved4", &self.Reserved4).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_BUTTON_CAPS_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIDP_BUTTON_CAPS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Usage == other.Usage && self.Reserved1 == other.Reserved1 && self.StringIndex == other.StringIndex && self.Reserved2 == other.Reserved2 && self.DesignatorIndex == other.DesignatorIndex && self.Reserved3 == other.Reserved3 && self.DataIndex == other.DataIndex && self.Reserved4 == other.Reserved4
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIDP_BUTTON_CAPS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_BUTTON_CAPS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_BUTTON_CAPS_0_1 {
    pub UsageMin: u16,
    pub UsageMax: u16,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_BUTTON_CAPS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_BUTTON_CAPS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIDP_BUTTON_CAPS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_BUTTON_CAPS_0_1").field("UsageMin", &self.UsageMin).field("UsageMax", &self.UsageMax).field("StringMin", &self.StringMin).field("StringMax", &self.StringMax).field("DesignatorMin", &self.DesignatorMin).field("DesignatorMax", &self.DesignatorMax).field("DataIndexMin", &self.DataIndexMin).field("DataIndexMax", &self.DataIndexMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_BUTTON_CAPS_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIDP_BUTTON_CAPS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.UsageMin == other.UsageMin && self.UsageMax == other.UsageMax && self.StringMin == other.StringMin && self.StringMax == other.StringMax && self.DesignatorMin == other.DesignatorMin && self.DesignatorMax == other.DesignatorMax && self.DataIndexMin == other.DataIndexMin && self.DataIndexMax == other.DataIndexMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIDP_BUTTON_CAPS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_BUTTON_CAPS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HIDP_CAPS {
    pub Usage: u16,
    pub UsagePage: u16,
    pub InputReportByteLength: u16,
    pub OutputReportByteLength: u16,
    pub FeatureReportByteLength: u16,
    pub Reserved: [u16; 17],
    pub NumberLinkCollectionNodes: u16,
    pub NumberInputButtonCaps: u16,
    pub NumberInputValueCaps: u16,
    pub NumberInputDataIndices: u16,
    pub NumberOutputButtonCaps: u16,
    pub NumberOutputValueCaps: u16,
    pub NumberOutputDataIndices: u16,
    pub NumberFeatureButtonCaps: u16,
    pub NumberFeatureValueCaps: u16,
    pub NumberFeatureDataIndices: u16,
}
impl ::core::marker::Copy for HIDP_CAPS {}
impl ::core::clone::Clone for HIDP_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HIDP_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_CAPS")
            .field("Usage", &self.Usage)
            .field("UsagePage", &self.UsagePage)
            .field("InputReportByteLength", &self.InputReportByteLength)
            .field("OutputReportByteLength", &self.OutputReportByteLength)
            .field("FeatureReportByteLength", &self.FeatureReportByteLength)
            .field("Reserved", &self.Reserved)
            .field("NumberLinkCollectionNodes", &self.NumberLinkCollectionNodes)
            .field("NumberInputButtonCaps", &self.NumberInputButtonCaps)
            .field("NumberInputValueCaps", &self.NumberInputValueCaps)
            .field("NumberInputDataIndices", &self.NumberInputDataIndices)
            .field("NumberOutputButtonCaps", &self.NumberOutputButtonCaps)
            .field("NumberOutputValueCaps", &self.NumberOutputValueCaps)
            .field("NumberOutputDataIndices", &self.NumberOutputDataIndices)
            .field("NumberFeatureButtonCaps", &self.NumberFeatureButtonCaps)
            .field("NumberFeatureValueCaps", &self.NumberFeatureValueCaps)
            .field("NumberFeatureDataIndices", &self.NumberFeatureDataIndices)
            .finish()
    }
}
impl ::windows_core::TypeKind for HIDP_CAPS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HIDP_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Usage == other.Usage
            && self.UsagePage == other.UsagePage
            && self.InputReportByteLength == other.InputReportByteLength
            && self.OutputReportByteLength == other.OutputReportByteLength
            && self.FeatureReportByteLength == other.FeatureReportByteLength
            && self.Reserved == other.Reserved
            && self.NumberLinkCollectionNodes == other.NumberLinkCollectionNodes
            && self.NumberInputButtonCaps == other.NumberInputButtonCaps
            && self.NumberInputValueCaps == other.NumberInputValueCaps
            && self.NumberInputDataIndices == other.NumberInputDataIndices
            && self.NumberOutputButtonCaps == other.NumberOutputButtonCaps
            && self.NumberOutputValueCaps == other.NumberOutputValueCaps
            && self.NumberOutputDataIndices == other.NumberOutputDataIndices
            && self.NumberFeatureButtonCaps == other.NumberFeatureButtonCaps
            && self.NumberFeatureValueCaps == other.NumberFeatureValueCaps
            && self.NumberFeatureDataIndices == other.NumberFeatureDataIndices
    }
}
impl ::core::cmp::Eq for HIDP_CAPS {}
impl ::core::default::Default for HIDP_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_DATA {
    pub DataIndex: u16,
    pub Reserved: u16,
    pub Anonymous: HIDP_DATA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union HIDP_DATA_0 {
    pub RawValue: u32,
    pub On: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_DATA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_DATA_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: u8,
    pub Reserved: [u8; 3],
    pub GlobalUnknowns: *mut HIDP_UNKNOWN_TOKEN,
    pub Data: [u32; 1],
}
impl ::core::marker::Copy for HIDP_EXTENDED_ATTRIBUTES {}
impl ::core::clone::Clone for HIDP_EXTENDED_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for HIDP_EXTENDED_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for HIDP_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0,
}
impl ::core::marker::Copy for HIDP_KEYBOARD_MODIFIER_STATE {}
impl ::core::clone::Clone for HIDP_KEYBOARD_MODIFIER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for HIDP_KEYBOARD_MODIFIER_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for HIDP_KEYBOARD_MODIFIER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union HIDP_KEYBOARD_MODIFIER_STATE_0 {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0_0,
    pub ul: u32,
}
impl ::core::marker::Copy for HIDP_KEYBOARD_MODIFIER_STATE_0 {}
impl ::core::clone::Clone for HIDP_KEYBOARD_MODIFIER_STATE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for HIDP_KEYBOARD_MODIFIER_STATE_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for HIDP_KEYBOARD_MODIFIER_STATE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for HIDP_KEYBOARD_MODIFIER_STATE_0_0 {}
impl ::core::clone::Clone for HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_KEYBOARD_MODIFIER_STATE_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for HIDP_KEYBOARD_MODIFIER_STATE_0_0 {}
impl ::core::default::Default for HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub Parent: u16,
    pub NumberOfChildren: u16,
    pub NextSibling: u16,
    pub FirstChild: u16,
    pub _bitfield: u32,
    pub UserContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HIDP_LINK_COLLECTION_NODE {}
impl ::core::clone::Clone for HIDP_LINK_COLLECTION_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for HIDP_LINK_COLLECTION_NODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for HIDP_LINK_COLLECTION_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HIDP_UNKNOWN_TOKEN {
    pub Token: u8,
    pub Reserved: [u8; 3],
    pub BitField: u32,
}
impl ::core::marker::Copy for HIDP_UNKNOWN_TOKEN {}
impl ::core::clone::Clone for HIDP_UNKNOWN_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HIDP_UNKNOWN_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_UNKNOWN_TOKEN").field("Token", &self.Token).field("Reserved", &self.Reserved).field("BitField", &self.BitField).finish()
    }
}
impl ::windows_core::TypeKind for HIDP_UNKNOWN_TOKEN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HIDP_UNKNOWN_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token && self.Reserved == other.Reserved && self.BitField == other.BitField
    }
}
impl ::core::cmp::Eq for HIDP_UNKNOWN_TOKEN {}
impl ::core::default::Default for HIDP_UNKNOWN_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: u16,
    pub ReportID: u8,
    pub IsAlias: super::super::Foundation::BOOLEAN,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub IsRange: super::super::Foundation::BOOLEAN,
    pub IsStringRange: super::super::Foundation::BOOLEAN,
    pub IsDesignatorRange: super::super::Foundation::BOOLEAN,
    pub IsAbsolute: super::super::Foundation::BOOLEAN,
    pub HasNull: super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub BitSize: u16,
    pub ReportCount: u16,
    pub Reserved2: [u16; 5],
    pub UnitsExp: u32,
    pub Units: u32,
    pub LogicalMin: i32,
    pub LogicalMax: i32,
    pub PhysicalMin: i32,
    pub PhysicalMax: i32,
    pub Anonymous: HIDP_VALUE_CAPS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_VALUE_CAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_VALUE_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_VALUE_CAPS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_VALUE_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union HIDP_VALUE_CAPS_0 {
    pub Range: HIDP_VALUE_CAPS_0_1,
    pub NotRange: HIDP_VALUE_CAPS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_VALUE_CAPS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_VALUE_CAPS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_VALUE_CAPS_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_VALUE_CAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_VALUE_CAPS_0_0 {
    pub Usage: u16,
    pub Reserved1: u16,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_VALUE_CAPS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_VALUE_CAPS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIDP_VALUE_CAPS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_VALUE_CAPS_0_0").field("Usage", &self.Usage).field("Reserved1", &self.Reserved1).field("StringIndex", &self.StringIndex).field("Reserved2", &self.Reserved2).field("DesignatorIndex", &self.DesignatorIndex).field("Reserved3", &self.Reserved3).field("DataIndex", &self.DataIndex).field("Reserved4", &self.Reserved4).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_VALUE_CAPS_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIDP_VALUE_CAPS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Usage == other.Usage && self.Reserved1 == other.Reserved1 && self.StringIndex == other.StringIndex && self.Reserved2 == other.Reserved2 && self.DesignatorIndex == other.DesignatorIndex && self.Reserved3 == other.Reserved3 && self.DataIndex == other.DataIndex && self.Reserved4 == other.Reserved4
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIDP_VALUE_CAPS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_VALUE_CAPS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HIDP_VALUE_CAPS_0_1 {
    pub UsageMin: u16,
    pub UsageMax: u16,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HIDP_VALUE_CAPS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HIDP_VALUE_CAPS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIDP_VALUE_CAPS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_VALUE_CAPS_0_1").field("UsageMin", &self.UsageMin).field("UsageMax", &self.UsageMax).field("StringMin", &self.StringMin).field("StringMax", &self.StringMax).field("DesignatorMin", &self.DesignatorMin).field("DesignatorMax", &self.DesignatorMax).field("DataIndexMin", &self.DataIndexMin).field("DataIndexMax", &self.DataIndexMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HIDP_VALUE_CAPS_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIDP_VALUE_CAPS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.UsageMin == other.UsageMin && self.UsageMax == other.UsageMax && self.StringMin == other.StringMin && self.StringMax == other.StringMax && self.DesignatorMin == other.DesignatorMin && self.DesignatorMax == other.DesignatorMax && self.DataIndexMin == other.DataIndexMin && self.DataIndexMax == other.DataIndexMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIDP_VALUE_CAPS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_VALUE_CAPS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct HID_COLLECTION_INFORMATION {
    pub DescriptorSize: u32,
    pub Polled: super::super::Foundation::BOOLEAN,
    pub Reserved1: [u8; 1],
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HID_COLLECTION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HID_COLLECTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HID_COLLECTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HID_COLLECTION_INFORMATION").field("DescriptorSize", &self.DescriptorSize).field("Polled", &self.Polled).field("Reserved1", &self.Reserved1).field("VendorID", &self.VendorID).field("ProductID", &self.ProductID).field("VersionNumber", &self.VersionNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for HID_COLLECTION_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HID_COLLECTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.DescriptorSize == other.DescriptorSize && self.Polled == other.Polled && self.Reserved1 == other.Reserved1 && self.VendorID == other.VendorID && self.ProductID == other.ProductID && self.VersionNumber == other.VersionNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HID_COLLECTION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HID_COLLECTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HID_DRIVER_CONFIG {
    pub Size: u32,
    pub RingBufferSize: u32,
}
impl ::core::marker::Copy for HID_DRIVER_CONFIG {}
impl ::core::clone::Clone for HID_DRIVER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HID_DRIVER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HID_DRIVER_CONFIG").field("Size", &self.Size).field("RingBufferSize", &self.RingBufferSize).finish()
    }
}
impl ::windows_core::TypeKind for HID_DRIVER_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HID_DRIVER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.RingBufferSize == other.RingBufferSize
    }
}
impl ::core::cmp::Eq for HID_DRIVER_CONFIG {}
impl ::core::default::Default for HID_DRIVER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HID_XFER_PACKET {
    pub reportBuffer: *mut u8,
    pub reportBufferLen: u32,
    pub reportId: u8,
}
impl ::core::marker::Copy for HID_XFER_PACKET {}
impl ::core::clone::Clone for HID_XFER_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HID_XFER_PACKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HID_XFER_PACKET").field("reportBuffer", &self.reportBuffer).field("reportBufferLen", &self.reportBufferLen).field("reportId", &self.reportId).finish()
    }
}
impl ::windows_core::TypeKind for HID_XFER_PACKET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HID_XFER_PACKET {
    fn eq(&self, other: &Self) -> bool {
        self.reportBuffer == other.reportBuffer && self.reportBufferLen == other.reportBufferLen && self.reportId == other.reportId
    }
}
impl ::core::cmp::Eq for HID_XFER_PACKET {}
impl ::core::default::Default for HID_XFER_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INDICATOR_LIST {
    pub MakeCode: u16,
    pub IndicatorFlags: u16,
}
impl ::core::marker::Copy for INDICATOR_LIST {}
impl ::core::clone::Clone for INDICATOR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INDICATOR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INDICATOR_LIST").field("MakeCode", &self.MakeCode).field("IndicatorFlags", &self.IndicatorFlags).finish()
    }
}
impl ::windows_core::TypeKind for INDICATOR_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for INDICATOR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.MakeCode == other.MakeCode && self.IndicatorFlags == other.IndicatorFlags
    }
}
impl ::core::cmp::Eq for INDICATOR_LIST {}
impl ::core::default::Default for INDICATOR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct INPUT_BUTTON_ENABLE_INFO {
    pub ButtonType: GPIOBUTTONS_BUTTON_TYPE,
    pub Enabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INPUT_BUTTON_ENABLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INPUT_BUTTON_ENABLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INPUT_BUTTON_ENABLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_BUTTON_ENABLE_INFO").field("ButtonType", &self.ButtonType).field("Enabled", &self.Enabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for INPUT_BUTTON_ENABLE_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INPUT_BUTTON_ENABLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ButtonType == other.ButtonType && self.Enabled == other.Enabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INPUT_BUTTON_ENABLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INPUT_BUTTON_ENABLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JOYCALIBRATE {
    pub wXbase: u32,
    pub wXdelta: u32,
    pub wYbase: u32,
    pub wYdelta: u32,
    pub wZbase: u32,
    pub wZdelta: u32,
}
impl ::core::marker::Copy for JOYCALIBRATE {}
impl ::core::clone::Clone for JOYCALIBRATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYCALIBRATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYCALIBRATE").field("wXbase", &self.wXbase).field("wXdelta", &self.wXdelta).field("wYbase", &self.wYbase).field("wYdelta", &self.wYdelta).field("wZbase", &self.wZbase).field("wZdelta", &self.wZdelta).finish()
    }
}
impl ::windows_core::TypeKind for JOYCALIBRATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for JOYCALIBRATE {
    fn eq(&self, other: &Self) -> bool {
        self.wXbase == other.wXbase && self.wXdelta == other.wXdelta && self.wYbase == other.wYbase && self.wYdelta == other.wYdelta && self.wZbase == other.wZbase && self.wZdelta == other.wZdelta
    }
}
impl ::core::cmp::Eq for JOYCALIBRATE {}
impl ::core::default::Default for JOYCALIBRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JOYPOS {
    pub dwX: u32,
    pub dwY: u32,
    pub dwZ: u32,
    pub dwR: u32,
    pub dwU: u32,
    pub dwV: u32,
}
impl ::core::marker::Copy for JOYPOS {}
impl ::core::clone::Clone for JOYPOS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYPOS").field("dwX", &self.dwX).field("dwY", &self.dwY).field("dwZ", &self.dwZ).field("dwR", &self.dwR).field("dwU", &self.dwU).field("dwV", &self.dwV).finish()
    }
}
impl ::windows_core::TypeKind for JOYPOS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for JOYPOS {
    fn eq(&self, other: &Self) -> bool {
        self.dwX == other.dwX && self.dwY == other.dwY && self.dwZ == other.dwZ && self.dwR == other.dwR && self.dwU == other.dwU && self.dwV == other.dwV
    }
}
impl ::core::cmp::Eq for JOYPOS {}
impl ::core::default::Default for JOYPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JOYRANGE {
    pub jpMin: JOYPOS,
    pub jpMax: JOYPOS,
    pub jpCenter: JOYPOS,
}
impl ::core::marker::Copy for JOYRANGE {}
impl ::core::clone::Clone for JOYRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYRANGE").field("jpMin", &self.jpMin).field("jpMax", &self.jpMax).field("jpCenter", &self.jpCenter).finish()
    }
}
impl ::windows_core::TypeKind for JOYRANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for JOYRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.jpMin == other.jpMin && self.jpMax == other.jpMax && self.jpCenter == other.jpCenter
    }
}
impl ::core::cmp::Eq for JOYRANGE {}
impl ::core::default::Default for JOYRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JOYREGHWCONFIG {
    pub hws: JOYREGHWSETTINGS,
    pub dwUsageSettings: u32,
    pub hwv: JOYREGHWVALUES,
    pub dwType: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for JOYREGHWCONFIG {}
impl ::core::clone::Clone for JOYREGHWCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYREGHWCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGHWCONFIG").field("hws", &self.hws).field("dwUsageSettings", &self.dwUsageSettings).field("hwv", &self.hwv).field("dwType", &self.dwType).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::windows_core::TypeKind for JOYREGHWCONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for JOYREGHWCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.hws == other.hws && self.dwUsageSettings == other.dwUsageSettings && self.hwv == other.hwv && self.dwType == other.dwType && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for JOYREGHWCONFIG {}
impl ::core::default::Default for JOYREGHWCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JOYREGHWSETTINGS {
    pub dwFlags: u32,
    pub dwNumButtons: u32,
}
impl ::core::marker::Copy for JOYREGHWSETTINGS {}
impl ::core::clone::Clone for JOYREGHWSETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYREGHWSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGHWSETTINGS").field("dwFlags", &self.dwFlags).field("dwNumButtons", &self.dwNumButtons).finish()
    }
}
impl ::windows_core::TypeKind for JOYREGHWSETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for JOYREGHWSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwNumButtons == other.dwNumButtons
    }
}
impl ::core::cmp::Eq for JOYREGHWSETTINGS {}
impl ::core::default::Default for JOYREGHWSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JOYREGHWVALUES {
    pub jrvHardware: JOYRANGE,
    pub dwPOVValues: [u32; 4],
    pub dwCalFlags: u32,
}
impl ::core::marker::Copy for JOYREGHWVALUES {}
impl ::core::clone::Clone for JOYREGHWVALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYREGHWVALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGHWVALUES").field("jrvHardware", &self.jrvHardware).field("dwPOVValues", &self.dwPOVValues).field("dwCalFlags", &self.dwCalFlags).finish()
    }
}
impl ::windows_core::TypeKind for JOYREGHWVALUES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for JOYREGHWVALUES {
    fn eq(&self, other: &Self) -> bool {
        self.jrvHardware == other.jrvHardware && self.dwPOVValues == other.dwPOVValues && self.dwCalFlags == other.dwCalFlags
    }
}
impl ::core::cmp::Eq for JOYREGHWVALUES {}
impl ::core::default::Default for JOYREGHWVALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct JOYREGUSERVALUES {
    pub dwTimeOut: u32,
    pub jrvRanges: JOYRANGE,
    pub jpDeadZone: JOYPOS,
}
impl ::core::marker::Copy for JOYREGUSERVALUES {}
impl ::core::clone::Clone for JOYREGUSERVALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOYREGUSERVALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGUSERVALUES").field("dwTimeOut", &self.dwTimeOut).field("jrvRanges", &self.jrvRanges).field("jpDeadZone", &self.jpDeadZone).finish()
    }
}
impl ::windows_core::TypeKind for JOYREGUSERVALUES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for JOYREGUSERVALUES {
    fn eq(&self, other: &Self) -> bool {
        self.dwTimeOut == other.dwTimeOut && self.jrvRanges == other.jrvRanges && self.jpDeadZone == other.jpDeadZone
    }
}
impl ::core::cmp::Eq for JOYREGUSERVALUES {}
impl ::core::default::Default for JOYREGUSERVALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_ATTRIBUTES {
    pub KeyboardIdentifier: KEYBOARD_ID,
    pub KeyboardMode: u16,
    pub NumberOfFunctionKeys: u16,
    pub NumberOfIndicators: u16,
    pub NumberOfKeysTotal: u16,
    pub InputDataQueueLength: u32,
    pub KeyRepeatMinimum: KEYBOARD_TYPEMATIC_PARAMETERS,
    pub KeyRepeatMaximum: KEYBOARD_TYPEMATIC_PARAMETERS,
}
impl ::core::marker::Copy for KEYBOARD_ATTRIBUTES {}
impl ::core::clone::Clone for KEYBOARD_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_ATTRIBUTES").field("KeyboardIdentifier", &self.KeyboardIdentifier).field("KeyboardMode", &self.KeyboardMode).field("NumberOfFunctionKeys", &self.NumberOfFunctionKeys).field("NumberOfIndicators", &self.NumberOfIndicators).field("NumberOfKeysTotal", &self.NumberOfKeysTotal).field("InputDataQueueLength", &self.InputDataQueueLength).field("KeyRepeatMinimum", &self.KeyRepeatMinimum).field("KeyRepeatMaximum", &self.KeyRepeatMaximum).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.KeyboardIdentifier == other.KeyboardIdentifier && self.KeyboardMode == other.KeyboardMode && self.NumberOfFunctionKeys == other.NumberOfFunctionKeys && self.NumberOfIndicators == other.NumberOfIndicators && self.NumberOfKeysTotal == other.NumberOfKeysTotal && self.InputDataQueueLength == other.InputDataQueueLength && self.KeyRepeatMinimum == other.KeyRepeatMinimum && self.KeyRepeatMaximum == other.KeyRepeatMaximum
    }
}
impl ::core::cmp::Eq for KEYBOARD_ATTRIBUTES {}
impl ::core::default::Default for KEYBOARD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_EXTENDED_ATTRIBUTES {
    pub Version: u8,
    pub FormFactor: u8,
    pub KeyType: u8,
    pub PhysicalLayout: u8,
    pub VendorSpecificPhysicalLayout: u8,
    pub IETFLanguageTagIndex: u8,
    pub ImplementedInputAssistControls: u8,
}
impl ::core::marker::Copy for KEYBOARD_EXTENDED_ATTRIBUTES {}
impl ::core::clone::Clone for KEYBOARD_EXTENDED_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_EXTENDED_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_EXTENDED_ATTRIBUTES").field("Version", &self.Version).field("FormFactor", &self.FormFactor).field("KeyType", &self.KeyType).field("PhysicalLayout", &self.PhysicalLayout).field("VendorSpecificPhysicalLayout", &self.VendorSpecificPhysicalLayout).field("IETFLanguageTagIndex", &self.IETFLanguageTagIndex).field("ImplementedInputAssistControls", &self.ImplementedInputAssistControls).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_EXTENDED_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_EXTENDED_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.FormFactor == other.FormFactor && self.KeyType == other.KeyType && self.PhysicalLayout == other.PhysicalLayout && self.VendorSpecificPhysicalLayout == other.VendorSpecificPhysicalLayout && self.IETFLanguageTagIndex == other.IETFLanguageTagIndex && self.ImplementedInputAssistControls == other.ImplementedInputAssistControls
    }
}
impl ::core::cmp::Eq for KEYBOARD_EXTENDED_ATTRIBUTES {}
impl ::core::default::Default for KEYBOARD_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_ID {
    pub Type: u8,
    pub Subtype: u8,
}
impl ::core::marker::Copy for KEYBOARD_ID {}
impl ::core::clone::Clone for KEYBOARD_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_ID").field("Type", &self.Type).field("Subtype", &self.Subtype).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Subtype == other.Subtype
    }
}
impl ::core::cmp::Eq for KEYBOARD_ID {}
impl ::core::default::Default for KEYBOARD_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_IME_STATUS {
    pub UnitId: u16,
    pub ImeOpen: u32,
    pub ImeConvMode: u32,
}
impl ::core::marker::Copy for KEYBOARD_IME_STATUS {}
impl ::core::clone::Clone for KEYBOARD_IME_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_IME_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_IME_STATUS").field("UnitId", &self.UnitId).field("ImeOpen", &self.ImeOpen).field("ImeConvMode", &self.ImeConvMode).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_IME_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_IME_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.ImeOpen == other.ImeOpen && self.ImeConvMode == other.ImeConvMode
    }
}
impl ::core::cmp::Eq for KEYBOARD_IME_STATUS {}
impl ::core::default::Default for KEYBOARD_IME_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_INDICATOR_PARAMETERS {
    pub UnitId: u16,
    pub LedFlags: u16,
}
impl ::core::marker::Copy for KEYBOARD_INDICATOR_PARAMETERS {}
impl ::core::clone::Clone for KEYBOARD_INDICATOR_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_INDICATOR_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_INDICATOR_PARAMETERS").field("UnitId", &self.UnitId).field("LedFlags", &self.LedFlags).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_INDICATOR_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_INDICATOR_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.LedFlags == other.LedFlags
    }
}
impl ::core::cmp::Eq for KEYBOARD_INDICATOR_PARAMETERS {}
impl ::core::default::Default for KEYBOARD_INDICATOR_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_INDICATOR_TRANSLATION {
    pub NumberOfIndicatorKeys: u16,
    pub IndicatorList: [INDICATOR_LIST; 1],
}
impl ::core::marker::Copy for KEYBOARD_INDICATOR_TRANSLATION {}
impl ::core::clone::Clone for KEYBOARD_INDICATOR_TRANSLATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_INDICATOR_TRANSLATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_INDICATOR_TRANSLATION").field("NumberOfIndicatorKeys", &self.NumberOfIndicatorKeys).field("IndicatorList", &self.IndicatorList).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_INDICATOR_TRANSLATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_INDICATOR_TRANSLATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfIndicatorKeys == other.NumberOfIndicatorKeys && self.IndicatorList == other.IndicatorList
    }
}
impl ::core::cmp::Eq for KEYBOARD_INDICATOR_TRANSLATION {}
impl ::core::default::Default for KEYBOARD_INDICATOR_TRANSLATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_INPUT_DATA {
    pub UnitId: u16,
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub ExtraInformation: u32,
}
impl ::core::marker::Copy for KEYBOARD_INPUT_DATA {}
impl ::core::clone::Clone for KEYBOARD_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_INPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_INPUT_DATA").field("UnitId", &self.UnitId).field("MakeCode", &self.MakeCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ExtraInformation", &self.ExtraInformation).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_INPUT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.MakeCode == other.MakeCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ExtraInformation == other.ExtraInformation
    }
}
impl ::core::cmp::Eq for KEYBOARD_INPUT_DATA {}
impl ::core::default::Default for KEYBOARD_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_TYPEMATIC_PARAMETERS {
    pub UnitId: u16,
    pub Rate: u16,
    pub Delay: u16,
}
impl ::core::marker::Copy for KEYBOARD_TYPEMATIC_PARAMETERS {}
impl ::core::clone::Clone for KEYBOARD_TYPEMATIC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_TYPEMATIC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_TYPEMATIC_PARAMETERS").field("UnitId", &self.UnitId).field("Rate", &self.Rate).field("Delay", &self.Delay).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_TYPEMATIC_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_TYPEMATIC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.Rate == other.Rate && self.Delay == other.Delay
    }
}
impl ::core::cmp::Eq for KEYBOARD_TYPEMATIC_PARAMETERS {}
impl ::core::default::Default for KEYBOARD_TYPEMATIC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct KEYBOARD_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
impl ::core::marker::Copy for KEYBOARD_UNIT_ID_PARAMETER {}
impl ::core::clone::Clone for KEYBOARD_UNIT_ID_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KEYBOARD_UNIT_ID_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_UNIT_ID_PARAMETER").field("UnitId", &self.UnitId).finish()
    }
}
impl ::windows_core::TypeKind for KEYBOARD_UNIT_ID_PARAMETER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for KEYBOARD_UNIT_ID_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId
    }
}
impl ::core::cmp::Eq for KEYBOARD_UNIT_ID_PARAMETER {}
impl ::core::default::Default for KEYBOARD_UNIT_ID_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MOUSE_ATTRIBUTES {
    pub MouseIdentifier: u16,
    pub NumberOfButtons: u16,
    pub SampleRate: u16,
    pub InputDataQueueLength: u32,
}
impl ::core::marker::Copy for MOUSE_ATTRIBUTES {}
impl ::core::clone::Clone for MOUSE_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_ATTRIBUTES").field("MouseIdentifier", &self.MouseIdentifier).field("NumberOfButtons", &self.NumberOfButtons).field("SampleRate", &self.SampleRate).field("InputDataQueueLength", &self.InputDataQueueLength).finish()
    }
}
impl ::windows_core::TypeKind for MOUSE_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MOUSE_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.MouseIdentifier == other.MouseIdentifier && self.NumberOfButtons == other.NumberOfButtons && self.SampleRate == other.SampleRate && self.InputDataQueueLength == other.InputDataQueueLength
    }
}
impl ::core::cmp::Eq for MOUSE_ATTRIBUTES {}
impl ::core::default::Default for MOUSE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MOUSE_INPUT_DATA {
    pub UnitId: u16,
    pub Flags: u16,
    pub Anonymous: MOUSE_INPUT_DATA_0,
    pub RawButtons: u32,
    pub LastX: i32,
    pub LastY: i32,
    pub ExtraInformation: u32,
}
impl ::core::marker::Copy for MOUSE_INPUT_DATA {}
impl ::core::clone::Clone for MOUSE_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MOUSE_INPUT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MOUSE_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union MOUSE_INPUT_DATA_0 {
    pub Buttons: u32,
    pub Anonymous: MOUSE_INPUT_DATA_0_0,
}
impl ::core::marker::Copy for MOUSE_INPUT_DATA_0 {}
impl ::core::clone::Clone for MOUSE_INPUT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for MOUSE_INPUT_DATA_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for MOUSE_INPUT_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MOUSE_INPUT_DATA_0_0 {
    pub ButtonFlags: u16,
    pub ButtonData: u16,
}
impl ::core::marker::Copy for MOUSE_INPUT_DATA_0_0 {}
impl ::core::clone::Clone for MOUSE_INPUT_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSE_INPUT_DATA_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_INPUT_DATA_0_0").field("ButtonFlags", &self.ButtonFlags).field("ButtonData", &self.ButtonData).finish()
    }
}
impl ::windows_core::TypeKind for MOUSE_INPUT_DATA_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MOUSE_INPUT_DATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ButtonFlags == other.ButtonFlags && self.ButtonData == other.ButtonData
    }
}
impl ::core::cmp::Eq for MOUSE_INPUT_DATA_0_0 {}
impl ::core::default::Default for MOUSE_INPUT_DATA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MOUSE_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
impl ::core::marker::Copy for MOUSE_UNIT_ID_PARAMETER {}
impl ::core::clone::Clone for MOUSE_UNIT_ID_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSE_UNIT_ID_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_UNIT_ID_PARAMETER").field("UnitId", &self.UnitId).finish()
    }
}
impl ::windows_core::TypeKind for MOUSE_UNIT_ID_PARAMETER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MOUSE_UNIT_ID_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId
    }
}
impl ::core::cmp::Eq for MOUSE_UNIT_ID_PARAMETER {}
impl ::core::default::Default for MOUSE_UNIT_ID_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHIDP_PREPARSED_DATA(pub isize);
impl ::core::default::Default for PHIDP_PREPARSED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PHIDP_PREPARSED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PHIDP_PREPARSED_DATA {}
impl ::core::fmt::Debug for PHIDP_PREPARSED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHIDP_PREPARSED_DATA").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for PHIDP_PREPARSED_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct USAGE_AND_PAGE {
    pub Usage: u16,
    pub UsagePage: u16,
}
impl ::core::marker::Copy for USAGE_AND_PAGE {}
impl ::core::clone::Clone for USAGE_AND_PAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USAGE_AND_PAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USAGE_AND_PAGE").field("Usage", &self.Usage).field("UsagePage", &self.UsagePage).finish()
    }
}
impl ::windows_core::TypeKind for USAGE_AND_PAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for USAGE_AND_PAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Usage == other.Usage && self.UsagePage == other.UsagePage
    }
}
impl ::core::cmp::Eq for USAGE_AND_PAGE {}
impl ::core::default::Default for USAGE_AND_PAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDICONFIGUREDEVICESCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<::windows_core::IUnknown>, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMCREATEDEFFECTOBJECTSCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<IDirectInputEffect>, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMDEVICEOBJECTSCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIDEVICEOBJECTINSTANCEA, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMDEVICEOBJECTSCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIDEVICEOBJECTINSTANCEW, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMDEVICESBYSEMANTICSCBA = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEA, param1: ::core::option::Option<IDirectInputDevice8A>, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMDEVICESBYSEMANTICSCBW = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEW, param1: ::core::option::Option<IDirectInputDevice8W>, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMDEVICESCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEA, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMDEVICESCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEW, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMEFFECTSCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIEFFECTINFOA, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMEFFECTSCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIEFFECTINFOW, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIENUMEFFECTSINFILECALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DIFILEEFFECT, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDIJOYTYPECALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNSHOWJOYCPL = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND)>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_HidP_GetVersionInternal = ::core::option::Option<unsafe extern "system" fn(version: *mut u32) -> super::super::Foundation::NTSTATUS>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PHIDP_INSERT_SCANCODES = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, newscancodes: ::windows_core::PCSTR, length: u32) -> super::super::Foundation::BOOLEAN>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
