#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ADsBuildEnumerator<P0>(padscontainer: P0) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>
where
    P0: ::windows_core::IntoParam<IADsContainer>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ADsBuildEnumerator(padscontainer : * mut::core::ffi::c_void, ppenumvariant : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    ADsBuildEnumerator(padscontainer.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn ADsBuildVarArrayInt(lpdwobjecttypes: *mut u32, dwobjecttypes: u32, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("activeds.dll" "system" fn ADsBuildVarArrayInt(lpdwobjecttypes : *mut u32, dwobjecttypes : u32, pvar : *mut super::super::System::Variant:: VARIANT) -> ::windows_core::HRESULT);
    ADsBuildVarArrayInt(lpdwobjecttypes, dwobjecttypes, pvar).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn ADsBuildVarArrayStr(lpppathnames: &[::windows_core::PCWSTR], pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("activeds.dll" "system" fn ADsBuildVarArrayStr(lpppathnames : *const ::windows_core::PCWSTR, dwpathnames : u32, pvar : *mut super::super::System::Variant:: VARIANT) -> ::windows_core::HRESULT);
    ADsBuildVarArrayStr(::core::mem::transmute(lpppathnames.as_ptr()), lpppathnames.len().try_into().unwrap(), pvar).ok()
}
#[inline]
pub unsafe fn ADsDecodeBinaryData<P0>(szsrcdata: P0, ppbdestdata: *mut *mut u8, pdwdestlen: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ADsDecodeBinaryData(szsrcdata : ::windows_core::PCWSTR, ppbdestdata : *mut *mut u8, pdwdestlen : *mut u32) -> ::windows_core::HRESULT);
    ADsDecodeBinaryData(szsrcdata.into_param().abi(), ppbdestdata, pdwdestlen).ok()
}
#[inline]
pub unsafe fn ADsEncodeBinaryData(pbsrcdata: *mut u8, dwsrclen: u32, ppszdestdata: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    ::windows_targets::link!("activeds.dll" "system" fn ADsEncodeBinaryData(pbsrcdata : *mut u8, dwsrclen : u32, ppszdestdata : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    ADsEncodeBinaryData(pbsrcdata, dwsrclen, ppszdestdata).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn ADsEnumerateNext<P0>(penumvariant: P0, celements: u32, pvar: *mut super::super::System::Variant::VARIANT, pcelementsfetched: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::System::Ole::IEnumVARIANT>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ADsEnumerateNext(penumvariant : * mut::core::ffi::c_void, celements : u32, pvar : *mut super::super::System::Variant:: VARIANT, pcelementsfetched : *mut u32) -> ::windows_core::HRESULT);
    ADsEnumerateNext(penumvariant.into_param().abi(), celements, pvar, pcelementsfetched).ok()
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
#[inline]
pub unsafe fn ADsFreeEnumerator<P0>(penumvariant: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::System::Ole::IEnumVARIANT>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ADsFreeEnumerator(penumvariant : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    ADsFreeEnumerator(penumvariant.into_param().abi()).ok()
}
#[inline]
pub unsafe fn ADsGetLastError(lperror: *mut u32, lperrorbuf: &mut [u16], lpnamebuf: &mut [u16]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("activeds.dll" "system" fn ADsGetLastError(lperror : *mut u32, lperrorbuf : ::windows_core::PWSTR, dwerrorbuflen : u32, lpnamebuf : ::windows_core::PWSTR, dwnamebuflen : u32) -> ::windows_core::HRESULT);
    ADsGetLastError(lperror, ::core::mem::transmute(lperrorbuf.as_ptr()), lperrorbuf.len().try_into().unwrap(), ::core::mem::transmute(lpnamebuf.as_ptr()), lpnamebuf.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn ADsGetObject<P0>(lpszpathname: P0, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ADsGetObject(lpszpathname : ::windows_core::PCWSTR, riid : *const ::windows_core::GUID, ppobject : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ADsGetObject(lpszpathname.into_param().abi(), riid, ppobject).ok()
}
#[inline]
pub unsafe fn ADsOpenObject<P0, P1, P2>(lpszpathname: P0, lpszusername: P1, lpszpassword: P2, dwreserved: ADS_AUTHENTICATION_ENUM, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ADsOpenObject(lpszpathname : ::windows_core::PCWSTR, lpszusername : ::windows_core::PCWSTR, lpszpassword : ::windows_core::PCWSTR, dwreserved : ADS_AUTHENTICATION_ENUM, riid : *const ::windows_core::GUID, ppobject : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    ADsOpenObject(lpszpathname.into_param().abi(), lpszusername.into_param().abi(), lpszpassword.into_param().abi(), dwreserved, riid, ppobject).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropCheckIfWritable<P0>(pwzattr: P0, pwritableattrs: *const ADS_ATTR_INFO) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsprop.dll" "system" fn ADsPropCheckIfWritable(pwzattr : ::windows_core::PCWSTR, pwritableattrs : *const ADS_ATTR_INFO) -> super::super::Foundation:: BOOL);
    ADsPropCheckIfWritable(pwzattr.into_param().abi(), pwritableattrs)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ADsPropCreateNotifyObj<P0, P1>(pappthddataobj: P0, pwzadsobjname: P1, phnotifyobj: *mut super::super::Foundation::HWND) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::System::Com::IDataObject>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsprop.dll" "system" fn ADsPropCreateNotifyObj(pappthddataobj : * mut::core::ffi::c_void, pwzadsobjname : ::windows_core::PCWSTR, phnotifyobj : *mut super::super::Foundation:: HWND) -> ::windows_core::HRESULT);
    ADsPropCreateNotifyObj(pappthddataobj.into_param().abi(), pwzadsobjname.into_param().abi(), phnotifyobj).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropGetInitInfo<P0>(hnotifyobj: P0, pinitparams: *mut ADSPROPINITPARAMS) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("dsprop.dll" "system" fn ADsPropGetInitInfo(hnotifyobj : super::super::Foundation:: HWND, pinitparams : *mut ADSPROPINITPARAMS) -> super::super::Foundation:: BOOL);
    ADsPropGetInitInfo(hnotifyobj.into_param().abi(), pinitparams)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropSendErrorMessage<P0>(hnotifyobj: P0, perror: *mut ADSPROPERROR) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("dsprop.dll" "system" fn ADsPropSendErrorMessage(hnotifyobj : super::super::Foundation:: HWND, perror : *mut ADSPROPERROR) -> super::super::Foundation:: BOOL);
    ADsPropSendErrorMessage(hnotifyobj.into_param().abi(), perror)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropSetHwnd<P0, P1>(hnotifyobj: P0, hpage: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("dsprop.dll" "system" fn ADsPropSetHwnd(hnotifyobj : super::super::Foundation:: HWND, hpage : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ADsPropSetHwnd(hnotifyobj.into_param().abi(), hpage.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropSetHwndWithTitle<P0, P1>(hnotifyobj: P0, hpage: P1, ptztitle: *const i8) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("dsprop.dll" "system" fn ADsPropSetHwndWithTitle(hnotifyobj : super::super::Foundation:: HWND, hpage : super::super::Foundation:: HWND, ptztitle : *const i8) -> super::super::Foundation:: BOOL);
    ADsPropSetHwndWithTitle(hnotifyobj.into_param().abi(), hpage.into_param().abi(), ptztitle)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ADsPropShowErrorDialog<P0, P1>(hnotifyobj: P0, hpage: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("dsprop.dll" "system" fn ADsPropShowErrorDialog(hnotifyobj : super::super::Foundation:: HWND, hpage : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ADsPropShowErrorDialog(hnotifyobj.into_param().abi(), hpage.into_param().abi())
}
#[inline]
pub unsafe fn ADsSetLastError<P0, P1>(dwerr: u32, pszerror: P0, pszprovider: P1)
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ADsSetLastError(dwerr : u32, pszerror : ::windows_core::PCWSTR, pszprovider : ::windows_core::PCWSTR));
    ADsSetLastError(dwerr, pszerror.into_param().abi(), pszprovider.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdsFreeAdsValues(padsvalues: *mut ADSVALUE, dwnumvalues: u32) {
    ::windows_targets::link!("activeds.dll" "system" fn AdsFreeAdsValues(padsvalues : *mut ADSVALUE, dwnumvalues : u32));
    AdsFreeAdsValues(padsvalues, dwnumvalues)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn AdsTypeToPropVariant(padsvalues: *mut ADSVALUE, dwnumvalues: u32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("activeds.dll" "system" fn AdsTypeToPropVariant(padsvalues : *mut ADSVALUE, dwnumvalues : u32, pvariant : *mut super::super::System::Variant:: VARIANT) -> ::windows_core::HRESULT);
    AdsTypeToPropVariant(padsvalues, dwnumvalues, pvariant).ok()
}
#[inline]
pub unsafe fn AllocADsMem(cb: u32) -> *mut ::core::ffi::c_void {
    ::windows_targets::link!("activeds.dll" "system" fn AllocADsMem(cb : u32) -> *mut ::core::ffi::c_void);
    AllocADsMem(cb)
}
#[inline]
pub unsafe fn AllocADsStr<P0>(pstr: P0) -> ::windows_core::PWSTR
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn AllocADsStr(pstr : ::windows_core::PCWSTR) -> ::windows_core::PWSTR);
    AllocADsStr(pstr.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn BinarySDToSecurityDescriptor<P0, P1, P2, P3>(psecuritydescriptor: P0, pvarsec: *mut super::super::System::Variant::VARIANT, pszservername: P1, username: P2, password: P3, dwflags: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Security::PSECURITY_DESCRIPTOR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn BinarySDToSecurityDescriptor(psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, pvarsec : *mut super::super::System::Variant:: VARIANT, pszservername : ::windows_core::PCWSTR, username : ::windows_core::PCWSTR, password : ::windows_core::PCWSTR, dwflags : u32) -> ::windows_core::HRESULT);
    BinarySDToSecurityDescriptor(psecuritydescriptor.into_param().abi(), pvarsec, pszservername.into_param().abi(), username.into_param().abi(), password.into_param().abi(), dwflags).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsAddSidHistoryA<P0, P1, P2, P3, P4, P5>(hds: P0, flags: u32, srcdomain: P1, srcprincipal: P2, srcdomaincontroller: P3, srcdomaincreds: ::core::option::Option<*const ::core::ffi::c_void>, dstdomain: P4, dstprincipal: P5) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P5: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsAddSidHistoryA(hds : super::super::Foundation:: HANDLE, flags : u32, srcdomain : ::windows_core::PCSTR, srcprincipal : ::windows_core::PCSTR, srcdomaincontroller : ::windows_core::PCSTR, srcdomaincreds : *const ::core::ffi::c_void, dstdomain : ::windows_core::PCSTR, dstprincipal : ::windows_core::PCSTR) -> u32);
    DsAddSidHistoryA(hds.into_param().abi(), flags, srcdomain.into_param().abi(), srcprincipal.into_param().abi(), srcdomaincontroller.into_param().abi(), ::core::mem::transmute(srcdomaincreds.unwrap_or(::std::ptr::null())), dstdomain.into_param().abi(), dstprincipal.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsAddSidHistoryW<P0, P1, P2, P3, P4, P5>(hds: P0, flags: u32, srcdomain: P1, srcprincipal: P2, srcdomaincontroller: P3, srcdomaincreds: ::core::option::Option<*const ::core::ffi::c_void>, dstdomain: P4, dstprincipal: P5) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsAddSidHistoryW(hds : super::super::Foundation:: HANDLE, flags : u32, srcdomain : ::windows_core::PCWSTR, srcprincipal : ::windows_core::PCWSTR, srcdomaincontroller : ::windows_core::PCWSTR, srcdomaincreds : *const ::core::ffi::c_void, dstdomain : ::windows_core::PCWSTR, dstprincipal : ::windows_core::PCWSTR) -> u32);
    DsAddSidHistoryW(hds.into_param().abi(), flags, srcdomain.into_param().abi(), srcprincipal.into_param().abi(), srcdomaincontroller.into_param().abi(), ::core::mem::transmute(srcdomaincreds.unwrap_or(::std::ptr::null())), dstdomain.into_param().abi(), dstprincipal.into_param().abi())
}
#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesA<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesA(computername : ::windows_core::PCSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut ::windows_core::PSTR) -> u32);
    DsAddressToSiteNamesA(computername.into_param().abi(), socketaddresses.len().try_into().unwrap(), ::core::mem::transmute(socketaddresses.as_ptr()), sitenames)
}
#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesExA<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows_core::PSTR, subnetnames: *mut *mut ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExA(computername : ::windows_core::PCSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut ::windows_core::PSTR, subnetnames : *mut *mut ::windows_core::PSTR) -> u32);
    DsAddressToSiteNamesExA(computername.into_param().abi(), socketaddresses.len().try_into().unwrap(), ::core::mem::transmute(socketaddresses.as_ptr()), sitenames, subnetnames)
}
#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesExW<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows_core::PWSTR, subnetnames: *mut *mut ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExW(computername : ::windows_core::PCWSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut ::windows_core::PWSTR, subnetnames : *mut *mut ::windows_core::PWSTR) -> u32);
    DsAddressToSiteNamesExW(computername.into_param().abi(), socketaddresses.len().try_into().unwrap(), ::core::mem::transmute(socketaddresses.as_ptr()), sitenames, subnetnames)
}
#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesW<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesW(computername : ::windows_core::PCWSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut ::windows_core::PWSTR) -> u32);
    DsAddressToSiteNamesW(computername.into_param().abi(), socketaddresses.len().try_into().unwrap(), ::core::mem::transmute(socketaddresses.as_ptr()), sitenames)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindA(domaincontrollername : ::windows_core::PCSTR, dnsdomainname : ::windows_core::PCSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindA(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindByInstanceA<P0, P1, P2, P3>(servername: P0, annotation: P1, instanceguid: ::core::option::Option<*const ::windows_core::GUID>, dnsdomainname: P2, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P3, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindByInstanceA(servername : ::windows_core::PCSTR, annotation : ::windows_core::PCSTR, instanceguid : *const ::windows_core::GUID, dnsdomainname : ::windows_core::PCSTR, authidentity : *const ::core::ffi::c_void, serviceprincipalname : ::windows_core::PCSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindByInstanceA(servername.into_param().abi(), annotation.into_param().abi(), ::core::mem::transmute(instanceguid.unwrap_or(::std::ptr::null())), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into_param().abi(), bindflags, phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindByInstanceW<P0, P1, P2, P3>(servername: P0, annotation: P1, instanceguid: ::core::option::Option<*const ::windows_core::GUID>, dnsdomainname: P2, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P3, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindByInstanceW(servername : ::windows_core::PCWSTR, annotation : ::windows_core::PCWSTR, instanceguid : *const ::windows_core::GUID, dnsdomainname : ::windows_core::PCWSTR, authidentity : *const ::core::ffi::c_void, serviceprincipalname : ::windows_core::PCWSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindByInstanceW(servername.into_param().abi(), annotation.into_param().abi(), ::core::mem::transmute(instanceguid.unwrap_or(::std::ptr::null())), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into_param().abi(), bindflags, phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindToISTGA<P0>(sitename: P0, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindToISTGA(sitename : ::windows_core::PCSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindToISTGA(sitename.into_param().abi(), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindToISTGW<P0>(sitename: P0, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindToISTGW(sitename : ::windows_core::PCWSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindToISTGW(sitename.into_param().abi(), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindW(domaincontrollername : ::windows_core::PCWSTR, dnsdomainname : ::windows_core::PCWSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindW(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithCredA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithCredA(domaincontrollername : ::windows_core::PCSTR, dnsdomainname : ::windows_core::PCSTR, authidentity : *const ::core::ffi::c_void, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithCredA(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithCredW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithCredW(domaincontrollername : ::windows_core::PCWSTR, dnsdomainname : ::windows_core::PCWSTR, authidentity : *const ::core::ffi::c_void, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithCredW(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnA<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnA(domaincontrollername : ::windows_core::PCSTR, dnsdomainname : ::windows_core::PCSTR, authidentity : *const ::core::ffi::c_void, serviceprincipalname : ::windows_core::PCSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnA(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into_param().abi(), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnExA<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnExA(domaincontrollername : ::windows_core::PCSTR, dnsdomainname : ::windows_core::PCSTR, authidentity : *const ::core::ffi::c_void, serviceprincipalname : ::windows_core::PCSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnExA(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into_param().abi(), bindflags, phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnExW<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnExW(domaincontrollername : ::windows_core::PCWSTR, dnsdomainname : ::windows_core::PCWSTR, authidentity : *const ::core::ffi::c_void, serviceprincipalname : ::windows_core::PCWSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnExW(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into_param().abi(), bindflags, phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindWithSpnW<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, serviceprincipalname: P2, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnW(domaincontrollername : ::windows_core::PCWSTR, dnsdomainname : ::windows_core::PCWSTR, authidentity : *const ::core::ffi::c_void, serviceprincipalname : ::windows_core::PCWSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnW(domaincontrollername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), serviceprincipalname.into_param().abi(), phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsBindingSetTimeout<P0>(hds: P0, ctimeoutsecs: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsBindingSetTimeout(hds : super::super::Foundation:: HANDLE, ctimeoutsecs : u32) -> u32);
    DsBindingSetTimeout(hds.into_param().abi(), ctimeoutsecs)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[inline]
pub unsafe fn DsBrowseForContainerA(pinfo: *mut DSBROWSEINFOA) -> i32 {
    ::windows_targets::link!("dsuiext.dll" "system" fn DsBrowseForContainerA(pinfo : *mut DSBROWSEINFOA) -> i32);
    DsBrowseForContainerA(pinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
#[inline]
pub unsafe fn DsBrowseForContainerW(pinfo: *mut DSBROWSEINFOW) -> i32 {
    ::windows_targets::link!("dsuiext.dll" "system" fn DsBrowseForContainerW(pinfo : *mut DSBROWSEINFOW) -> i32);
    DsBrowseForContainerW(pinfo)
}
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerA<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerA(serviceclass : ::windows_core::PCSTR, servicename : ::windows_core::PCSTR, pcspnlength : *mut u32, pszspn : ::windows_core::PSTR) -> u32);
    DsClientMakeSpnForTargetServerA(serviceclass.into_param().abi(), servicename.into_param().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerW<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerW(serviceclass : ::windows_core::PCWSTR, servicename : ::windows_core::PCWSTR, pcspnlength : *mut u32, pszspn : ::windows_core::PWSTR) -> u32);
    DsClientMakeSpnForTargetServerW(serviceclass.into_param().abi(), servicename.into_param().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackNamesA<P0>(hds: P0, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[::windows_core::PCSTR], ppresult: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsCrackNamesA(hds : super::super::Foundation:: HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const ::windows_core::PCSTR, ppresult : *mut *mut DS_NAME_RESULTA) -> u32);
    DsCrackNamesA(hds.into_param().abi(), flags, formatoffered, formatdesired, rpnames.len().try_into().unwrap(), ::core::mem::transmute(rpnames.as_ptr()), ppresult)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackNamesW<P0>(hds: P0, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[::windows_core::PCWSTR], ppresult: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsCrackNamesW(hds : super::super::Foundation:: HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const ::windows_core::PCWSTR, ppresult : *mut *mut DS_NAME_RESULTW) -> u32);
    DsCrackNamesW(hds.into_param().abi(), flags, formatoffered, formatdesired, rpnames.len().try_into().unwrap(), ::core::mem::transmute(rpnames.as_ptr()), ppresult)
}
#[inline]
pub unsafe fn DsCrackSpn2A(pszspn: &[u8], pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows_core::PSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows_core::PSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows_core::PSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32 {
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn2A(pszspn : ::windows_core::PCSTR, cspn : u32, pcserviceclass : *mut u32, serviceclass : ::windows_core::PSTR, pcservicename : *mut u32, servicename : ::windows_core::PSTR, pcinstancename : *mut u32, instancename : ::windows_core::PSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpn2A(::core::mem::transmute(pszspn.as_ptr()), pszspn.len().try_into().unwrap(), ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackSpn2W(pszspn: &[u16], pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows_core::PWSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows_core::PWSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows_core::PWSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32 {
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn2W(pszspn : ::windows_core::PCWSTR, cspn : u32, pcserviceclass : *mut u32, serviceclass : ::windows_core::PWSTR, pcservicename : *mut u32, servicename : ::windows_core::PWSTR, pcinstancename : *mut u32, instancename : ::windows_core::PWSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpn2W(::core::mem::transmute(pszspn.as_ptr()), pszspn.len().try_into().unwrap(), ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackSpn3W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: ::windows_core::PWSTR, pcinstancename: *mut u32, instancename: ::windows_core::PWSTR, pportnumber: *mut u16, pcdomainname: *mut u32, domainname: ::windows_core::PWSTR, pcrealmname: *mut u32, realmname: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn3W(pszspn : ::windows_core::PCWSTR, cspn : u32, pchostname : *mut u32, hostname : ::windows_core::PWSTR, pcinstancename : *mut u32, instancename : ::windows_core::PWSTR, pportnumber : *mut u16, pcdomainname : *mut u32, domainname : ::windows_core::PWSTR, pcrealmname : *mut u32, realmname : ::windows_core::PWSTR) -> u32);
    DsCrackSpn3W(pszspn.into_param().abi(), cspn, pchostname, ::core::mem::transmute(hostname), pcinstancename, ::core::mem::transmute(instancename), pportnumber, pcdomainname, ::core::mem::transmute(domainname), pcrealmname, ::core::mem::transmute(realmname))
}
#[inline]
pub unsafe fn DsCrackSpn4W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: ::windows_core::PWSTR, pcinstancename: *mut u32, instancename: ::windows_core::PWSTR, pcportname: *mut u32, portname: ::windows_core::PWSTR, pcdomainname: *mut u32, domainname: ::windows_core::PWSTR, pcrealmname: *mut u32, realmname: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn4W(pszspn : ::windows_core::PCWSTR, cspn : u32, pchostname : *mut u32, hostname : ::windows_core::PWSTR, pcinstancename : *mut u32, instancename : ::windows_core::PWSTR, pcportname : *mut u32, portname : ::windows_core::PWSTR, pcdomainname : *mut u32, domainname : ::windows_core::PWSTR, pcrealmname : *mut u32, realmname : ::windows_core::PWSTR) -> u32);
    DsCrackSpn4W(pszspn.into_param().abi(), cspn, pchostname, ::core::mem::transmute(hostname), pcinstancename, ::core::mem::transmute(instancename), pcportname, ::core::mem::transmute(portname), pcdomainname, ::core::mem::transmute(domainname), pcrealmname, ::core::mem::transmute(realmname))
}
#[inline]
pub unsafe fn DsCrackSpnA<P0>(pszspn: P0, pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows_core::PSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows_core::PSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows_core::PSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackSpnA(pszspn : ::windows_core::PCSTR, pcserviceclass : *mut u32, serviceclass : ::windows_core::PSTR, pcservicename : *mut u32, servicename : ::windows_core::PSTR, pcinstancename : *mut u32, instancename : ::windows_core::PSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpnA(pszspn.into_param().abi(), ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackSpnW<P0>(pszspn: P0, pcserviceclass: ::core::option::Option<*mut u32>, serviceclass: ::windows_core::PWSTR, pcservicename: ::core::option::Option<*mut u32>, servicename: ::windows_core::PWSTR, pcinstancename: ::core::option::Option<*mut u32>, instancename: ::windows_core::PWSTR, pinstanceport: ::core::option::Option<*mut u16>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackSpnW(pszspn : ::windows_core::PCWSTR, pcserviceclass : *mut u32, serviceclass : ::windows_core::PWSTR, pcservicename : *mut u32, servicename : ::windows_core::PWSTR, pcinstancename : *mut u32, instancename : ::windows_core::PWSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpnW(pszspn.into_param().abi(), ::core::mem::transmute(pcserviceclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(serviceclass), ::core::mem::transmute(pcservicename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servicename), ::core::mem::transmute(pcinstancename.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(instancename), ::core::mem::transmute(pinstanceport.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnA(pszrdn: &[u8], pguid: ::core::option::Option<*mut ::windows_core::GUID>, pedsmanglefor: ::core::option::Option<*mut DS_MANGLE_FOR>) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackUnquotedMangledRdnA(pszrdn : ::windows_core::PCSTR, cchrdn : u32, pguid : *mut ::windows_core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsCrackUnquotedMangledRdnA(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), ::core::mem::transmute(pguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pedsmanglefor.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnW(pszrdn: &[u16], pguid: ::core::option::Option<*mut ::windows_core::GUID>, pedsmanglefor: ::core::option::Option<*mut DS_MANGLE_FOR>) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("dsparse.dll" "system" fn DsCrackUnquotedMangledRdnW(pszrdn : ::windows_core::PCWSTR, cchrdn : u32, pguid : *mut ::windows_core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsCrackUnquotedMangledRdnW(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), ::core::mem::transmute(pguid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pedsmanglefor.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsA<P0, P1, P2>(servername: P0, dnsdomainname: P1, domainguid: ::core::option::Option<*const ::windows_core::GUID>, dsaguid: ::core::option::Option<*const ::windows_core::GUID>, dnshostname: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsA(servername : ::windows_core::PCSTR, dnsdomainname : ::windows_core::PCSTR, domainguid : *const ::windows_core::GUID, dsaguid : *const ::windows_core::GUID, dnshostname : ::windows_core::PCSTR) -> u32);
    DsDeregisterDnsHostRecordsA(servername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(dsaguid.unwrap_or(::std::ptr::null())), dnshostname.into_param().abi())
}
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsW<P0, P1, P2>(servername: P0, dnsdomainname: P1, domainguid: ::core::option::Option<*const ::windows_core::GUID>, dsaguid: ::core::option::Option<*const ::windows_core::GUID>, dnshostname: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsW(servername : ::windows_core::PCWSTR, dnsdomainname : ::windows_core::PCWSTR, domainguid : *const ::windows_core::GUID, dsaguid : *const ::windows_core::GUID, dnshostname : ::windows_core::PCWSTR) -> u32);
    DsDeregisterDnsHostRecordsW(servername.into_param().abi(), dnsdomainname.into_param().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(dsaguid.unwrap_or(::std::ptr::null())), dnshostname.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsEnumerateDomainTrustsA<P0>(servername: P0, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSA, domaincount: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsA(servername : ::windows_core::PCSTR, flags : u32, domains : *mut *mut DS_DOMAIN_TRUSTSA, domaincount : *mut u32) -> u32);
    DsEnumerateDomainTrustsA(servername.into_param().abi(), flags, domains, domaincount)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsEnumerateDomainTrustsW<P0>(servername: P0, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSW, domaincount: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsW(servername : ::windows_core::PCWSTR, flags : u32, domains : *mut *mut DS_DOMAIN_TRUSTSW, domaincount : *mut u32) -> u32);
    DsEnumerateDomainTrustsW(servername.into_param().abi(), flags, domains, domaincount)
}
#[inline]
pub unsafe fn DsFreeDomainControllerInfoA(infolevel: u32, pinfo: &[u8]) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoA(infolevel : u32, cinfo : u32, pinfo : *const ::core::ffi::c_void));
    DsFreeDomainControllerInfoA(infolevel, pinfo.len().try_into().unwrap(), ::core::mem::transmute(pinfo.as_ptr()))
}
#[inline]
pub unsafe fn DsFreeDomainControllerInfoW(infolevel: u32, pinfo: &[u8]) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoW(infolevel : u32, cinfo : u32, pinfo : *const ::core::ffi::c_void));
    DsFreeDomainControllerInfoW(infolevel, pinfo.len().try_into().unwrap(), ::core::mem::transmute(pinfo.as_ptr()))
}
#[inline]
pub unsafe fn DsFreeNameResultA(presult: *const DS_NAME_RESULTA) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeNameResultA(presult : *const DS_NAME_RESULTA));
    DsFreeNameResultA(presult)
}
#[inline]
pub unsafe fn DsFreeNameResultW(presult: *const DS_NAME_RESULTW) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeNameResultW(presult : *const DS_NAME_RESULTW));
    DsFreeNameResultW(presult)
}
#[inline]
pub unsafe fn DsFreePasswordCredentials(authidentity: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreePasswordCredentials(authidentity : *const ::core::ffi::c_void));
    DsFreePasswordCredentials(authidentity)
}
#[inline]
pub unsafe fn DsFreeSchemaGuidMapA(pguidmap: *const DS_SCHEMA_GUID_MAPA) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapA(pguidmap : *const DS_SCHEMA_GUID_MAPA));
    DsFreeSchemaGuidMapA(pguidmap)
}
#[inline]
pub unsafe fn DsFreeSchemaGuidMapW(pguidmap: *const DS_SCHEMA_GUID_MAPW) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapW(pguidmap : *const DS_SCHEMA_GUID_MAPW));
    DsFreeSchemaGuidMapW(pguidmap)
}
#[inline]
pub unsafe fn DsFreeSpnArrayA(rpszspn: &mut [::windows_core::PSTR]) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayA(cspn : u32, rpszspn : *mut ::windows_core::PSTR));
    DsFreeSpnArrayA(rpszspn.len().try_into().unwrap(), ::core::mem::transmute(rpszspn.as_ptr()))
}
#[inline]
pub unsafe fn DsFreeSpnArrayW(rpszspn: &mut [::windows_core::PWSTR]) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayW(cspn : u32, rpszspn : *mut ::windows_core::PWSTR));
    DsFreeSpnArrayW(rpszspn.len().try_into().unwrap(), ::core::mem::transmute(rpszspn.as_ptr()))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsGetDcCloseW<P0>(getdccontexthandle: P0)
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcCloseW(getdccontexthandle : super::super::Foundation:: HANDLE));
    DsGetDcCloseW(getdccontexthandle.into_param().abi())
}
#[inline]
pub unsafe fn DsGetDcNameA<P0, P1, P2>(computername: P0, domainname: P1, domainguid: ::core::option::Option<*const ::windows_core::GUID>, sitename: P2, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcNameA(computername : ::windows_core::PCSTR, domainname : ::windows_core::PCSTR, domainguid : *const ::windows_core::GUID, sitename : ::windows_core::PCSTR, flags : u32, domaincontrollerinfo : *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32);
    DsGetDcNameA(computername.into_param().abi(), domainname.into_param().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), sitename.into_param().abi(), flags, domaincontrollerinfo)
}
#[inline]
pub unsafe fn DsGetDcNameW<P0, P1, P2>(computername: P0, domainname: P1, domainguid: ::core::option::Option<*const ::windows_core::GUID>, sitename: P2, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcNameW(computername : ::windows_core::PCWSTR, domainname : ::windows_core::PCWSTR, domainguid : *const ::windows_core::GUID, sitename : ::windows_core::PCWSTR, flags : u32, domaincontrollerinfo : *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32);
    DsGetDcNameW(computername.into_param().abi(), domainname.into_param().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), sitename.into_param().abi(), flags, domaincontrollerinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsGetDcNextA<P0>(getdccontexthandle: P0, sockaddresscount: ::core::option::Option<*mut u32>, sockaddresses: ::core::option::Option<*mut *mut super::WinSock::SOCKET_ADDRESS>, dnshostname: ::core::option::Option<*mut ::windows_core::PSTR>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcNextA(getdccontexthandle : super::super::Foundation:: HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut *mut super::WinSock:: SOCKET_ADDRESS, dnshostname : *mut ::windows_core::PSTR) -> u32);
    DsGetDcNextA(getdccontexthandle.into_param().abi(), ::core::mem::transmute(sockaddresscount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(sockaddresses.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(dnshostname.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn DsGetDcNextW<P0>(getdccontexthandle: P0, sockaddresscount: ::core::option::Option<*mut u32>, sockaddresses: ::core::option::Option<*mut *mut super::WinSock::SOCKET_ADDRESS>, dnshostname: ::core::option::Option<*mut ::windows_core::PWSTR>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcNextW(getdccontexthandle : super::super::Foundation:: HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut *mut super::WinSock:: SOCKET_ADDRESS, dnshostname : *mut ::windows_core::PWSTR) -> u32);
    DsGetDcNextW(getdccontexthandle.into_param().abi(), ::core::mem::transmute(sockaddresscount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(sockaddresses.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(dnshostname.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsGetDcOpenA<P0, P1, P2>(dnsname: P0, optionflags: u32, sitename: P1, domainguid: ::core::option::Option<*const ::windows_core::GUID>, dnsforestname: P2, dcflags: u32, retgetdccontext: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcOpenA(dnsname : ::windows_core::PCSTR, optionflags : u32, sitename : ::windows_core::PCSTR, domainguid : *const ::windows_core::GUID, dnsforestname : ::windows_core::PCSTR, dcflags : u32, retgetdccontext : *mut super::super::Foundation:: HANDLE) -> u32);
    DsGetDcOpenA(dnsname.into_param().abi(), optionflags, sitename.into_param().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), dnsforestname.into_param().abi(), dcflags, retgetdccontext)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsGetDcOpenW<P0, P1, P2>(dnsname: P0, optionflags: u32, sitename: P1, domainguid: ::core::option::Option<*const ::windows_core::GUID>, dnsforestname: P2, dcflags: u32, retgetdccontext: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcOpenW(dnsname : ::windows_core::PCWSTR, optionflags : u32, sitename : ::windows_core::PCWSTR, domainguid : *const ::windows_core::GUID, dnsforestname : ::windows_core::PCWSTR, dcflags : u32, retgetdccontext : *mut super::super::Foundation:: HANDLE) -> u32);
    DsGetDcOpenW(dnsname.into_param().abi(), optionflags, sitename.into_param().abi(), ::core::mem::transmute(domainguid.unwrap_or(::std::ptr::null())), dnsforestname.into_param().abi(), dcflags, retgetdccontext)
}
#[inline]
pub unsafe fn DsGetDcSiteCoverageA<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageA(servername : ::windows_core::PCSTR, entrycount : *mut u32, sitenames : *mut *mut ::windows_core::PSTR) -> u32);
    DsGetDcSiteCoverageA(servername.into_param().abi(), entrycount, sitenames)
}
#[inline]
pub unsafe fn DsGetDcSiteCoverageW<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageW(servername : ::windows_core::PCWSTR, entrycount : *mut u32, sitenames : *mut *mut ::windows_core::PWSTR) -> u32);
    DsGetDcSiteCoverageW(servername.into_param().abi(), entrycount, sitenames)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsGetDomainControllerInfoA<P0, P1>(hds: P0, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoA(hds : super::super::Foundation:: HANDLE, domainname : ::windows_core::PCSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut ::core::ffi::c_void) -> u32);
    DsGetDomainControllerInfoA(hds.into_param().abi(), domainname.into_param().abi(), infolevel, pcout, ppinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsGetDomainControllerInfoW<P0, P1>(hds: P0, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoW(hds : super::super::Foundation:: HANDLE, domainname : ::windows_core::PCWSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut ::core::ffi::c_void) -> u32);
    DsGetDomainControllerInfoW(hds.into_param().abi(), domainname.into_param().abi(), infolevel, pcout, ppinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
#[inline]
pub unsafe fn DsGetForestTrustInformationW<P0, P1>(servername: P0, trusteddomainname: P1, flags: u32, foresttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetForestTrustInformationW(servername : ::windows_core::PCWSTR, trusteddomainname : ::windows_core::PCWSTR, flags : u32, foresttrustinfo : *mut *mut super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION) -> u32);
    DsGetForestTrustInformationW(servername.into_param().abi(), trusteddomainname.into_param().abi(), flags, foresttrustinfo)
}
#[inline]
pub unsafe fn DsGetFriendlyClassName<P0>(pszobjectclass: P0, pszbuffer: &mut [u16]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsuiext.dll" "system" fn DsGetFriendlyClassName(pszobjectclass : ::windows_core::PCWSTR, pszbuffer : ::windows_core::PWSTR, cchbuffer : u32) -> ::windows_core::HRESULT);
    DsGetFriendlyClassName(pszobjectclass.into_param().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap()).ok()
}
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn DsGetIcon<P0>(dwflags: u32, pszobjectclass: P0, cximage: i32, cyimage: i32) -> super::super::UI::WindowsAndMessaging::HICON
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsuiext.dll" "system" fn DsGetIcon(dwflags : u32, pszobjectclass : ::windows_core::PCWSTR, cximage : i32, cyimage : i32) -> super::super::UI::WindowsAndMessaging:: HICON);
    DsGetIcon(dwflags, pszobjectclass.into_param().abi(), cximage, cyimage)
}
#[inline]
pub unsafe fn DsGetRdnW(ppdn: *mut ::windows_core::PWSTR, pcdn: *mut u32, ppkey: *mut ::windows_core::PWSTR, pckey: *mut u32, ppval: *mut ::windows_core::PWSTR, pcval: *mut u32) -> u32 {
    ::windows_targets::link!("dsparse.dll" "system" fn DsGetRdnW(ppdn : *mut ::windows_core::PWSTR, pcdn : *mut u32, ppkey : *mut ::windows_core::PWSTR, pckey : *mut u32, ppval : *mut ::windows_core::PWSTR, pcval : *mut u32) -> u32);
    DsGetRdnW(ppdn, pcdn, ppkey, pckey, ppval, pcval)
}
#[inline]
pub unsafe fn DsGetSiteNameA<P0>(computername: P0, sitename: *mut ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetSiteNameA(computername : ::windows_core::PCSTR, sitename : *mut ::windows_core::PSTR) -> u32);
    DsGetSiteNameA(computername.into_param().abi(), sitename)
}
#[inline]
pub unsafe fn DsGetSiteNameW<P0>(computername: P0, sitename: *mut ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsGetSiteNameW(computername : ::windows_core::PCWSTR, sitename : *mut ::windows_core::PWSTR) -> u32);
    DsGetSiteNameW(computername.into_param().abi(), sitename)
}
#[inline]
pub unsafe fn DsGetSpnA<P0, P1>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P0, servicename: P1, instanceport: u16, cinstancenames: u16, pinstancenames: ::core::option::Option<*const ::windows_core::PCSTR>, pinstanceports: ::core::option::Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsGetSpnA(servicetype : DS_SPN_NAME_TYPE, serviceclass : ::windows_core::PCSTR, servicename : ::windows_core::PCSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const ::windows_core::PCSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut ::windows_core::PSTR) -> u32);
    DsGetSpnA(servicetype, serviceclass.into_param().abi(), servicename.into_param().abi(), instanceport, cinstancenames, ::core::mem::transmute(pinstancenames.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinstanceports.unwrap_or(::std::ptr::null())), pcspn, prpszspn)
}
#[inline]
pub unsafe fn DsGetSpnW<P0, P1>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P0, servicename: P1, instanceport: u16, cinstancenames: u16, pinstancenames: ::core::option::Option<*const ::windows_core::PCWSTR>, pinstanceports: ::core::option::Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsGetSpnW(servicetype : DS_SPN_NAME_TYPE, serviceclass : ::windows_core::PCWSTR, servicename : ::windows_core::PCWSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const ::windows_core::PCWSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut ::windows_core::PWSTR) -> u32);
    DsGetSpnW(servicetype, serviceclass.into_param().abi(), servicename.into_param().abi(), instanceport, cinstancenames, ::core::mem::transmute(pinstancenames.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pinstanceports.unwrap_or(::std::ptr::null())), pcspn, prpszspn)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsInheritSecurityIdentityA<P0, P1, P2>(hds: P0, flags: u32, srcprincipal: P1, dstprincipal: P2) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityA(hds : super::super::Foundation:: HANDLE, flags : u32, srcprincipal : ::windows_core::PCSTR, dstprincipal : ::windows_core::PCSTR) -> u32);
    DsInheritSecurityIdentityA(hds.into_param().abi(), flags, srcprincipal.into_param().abi(), dstprincipal.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsInheritSecurityIdentityW<P0, P1, P2>(hds: P0, flags: u32, srcprincipal: P1, dstprincipal: P2) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityW(hds : super::super::Foundation:: HANDLE, flags : u32, srcprincipal : ::windows_core::PCWSTR, dstprincipal : ::windows_core::PCWSTR) -> u32);
    DsInheritSecurityIdentityW(hds.into_param().abi(), flags, srcprincipal.into_param().abi(), dstprincipal.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledDnA<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsIsMangledDnA(pszdn : ::windows_core::PCSTR, edsmanglefor : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledDnA(pszdn.into_param().abi(), edsmanglefor)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledDnW<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsIsMangledDnW(pszdn : ::windows_core::PCWSTR, edsmanglefor : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledDnW(pszdn.into_param().abi(), edsmanglefor)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledRdnValueA(pszrdn: &[u8], edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("dsparse.dll" "system" fn DsIsMangledRdnValueA(pszrdn : ::windows_core::PCSTR, crdn : u32, edsmanglefordesired : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledRdnValueA(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), edsmanglefordesired)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsIsMangledRdnValueW(pszrdn: &[u16], edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("dsparse.dll" "system" fn DsIsMangledRdnValueW(pszrdn : ::windows_core::PCWSTR, crdn : u32, edsmanglefordesired : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledRdnValueW(::core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), edsmanglefordesired)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListDomainsInSiteA<P0, P1>(hds: P0, site: P1, ppdomains: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteA(hds : super::super::Foundation:: HANDLE, site : ::windows_core::PCSTR, ppdomains : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListDomainsInSiteA(hds.into_param().abi(), site.into_param().abi(), ppdomains)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListDomainsInSiteW<P0, P1>(hds: P0, site: P1, ppdomains: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteW(hds : super::super::Foundation:: HANDLE, site : ::windows_core::PCWSTR, ppdomains : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListDomainsInSiteW(hds.into_param().abi(), site.into_param().abi(), ppdomains)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListInfoForServerA<P0, P1>(hds: P0, server: P1, ppinfo: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListInfoForServerA(hds : super::super::Foundation:: HANDLE, server : ::windows_core::PCSTR, ppinfo : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListInfoForServerA(hds.into_param().abi(), server.into_param().abi(), ppinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListInfoForServerW<P0, P1>(hds: P0, server: P1, ppinfo: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListInfoForServerW(hds : super::super::Foundation:: HANDLE, server : ::windows_core::PCWSTR, ppinfo : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListInfoForServerW(hds.into_param().abi(), server.into_param().abi(), ppinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListRolesA<P0>(hds: P0, pproles: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListRolesA(hds : super::super::Foundation:: HANDLE, pproles : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListRolesA(hds.into_param().abi(), pproles)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListRolesW<P0>(hds: P0, pproles: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListRolesW(hds : super::super::Foundation:: HANDLE, pproles : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListRolesW(hds.into_param().abi(), pproles)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersForDomainInSiteA<P0, P1, P2>(hds: P0, domain: P1, site: P2, ppservers: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteA(hds : super::super::Foundation:: HANDLE, domain : ::windows_core::PCSTR, site : ::windows_core::PCSTR, ppservers : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListServersForDomainInSiteA(hds.into_param().abi(), domain.into_param().abi(), site.into_param().abi(), ppservers)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersForDomainInSiteW<P0, P1, P2>(hds: P0, domain: P1, site: P2, ppservers: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteW(hds : super::super::Foundation:: HANDLE, domain : ::windows_core::PCWSTR, site : ::windows_core::PCWSTR, ppservers : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListServersForDomainInSiteW(hds.into_param().abi(), domain.into_param().abi(), site.into_param().abi(), ppservers)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersInSiteA<P0, P1>(hds: P0, site: P1, ppservers: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListServersInSiteA(hds : super::super::Foundation:: HANDLE, site : ::windows_core::PCSTR, ppservers : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListServersInSiteA(hds.into_param().abi(), site.into_param().abi(), ppservers)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListServersInSiteW<P0, P1>(hds: P0, site: P1, ppservers: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListServersInSiteW(hds : super::super::Foundation:: HANDLE, site : ::windows_core::PCWSTR, ppservers : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListServersInSiteW(hds.into_param().abi(), site.into_param().abi(), ppservers)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListSitesA<P0>(hds: P0, ppsites: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListSitesA(hds : super::super::Foundation:: HANDLE, ppsites : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListSitesA(hds.into_param().abi(), ppsites)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsListSitesW<P0>(hds: P0, ppsites: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsListSitesW(hds : super::super::Foundation:: HANDLE, ppsites : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListSitesW(hds.into_param().abi(), ppsites)
}
#[inline]
pub unsafe fn DsMakePasswordCredentialsA<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsA(user : ::windows_core::PCSTR, domain : ::windows_core::PCSTR, password : ::windows_core::PCSTR, pauthidentity : *mut *mut ::core::ffi::c_void) -> u32);
    DsMakePasswordCredentialsA(user.into_param().abi(), domain.into_param().abi(), password.into_param().abi(), pauthidentity)
}
#[inline]
pub unsafe fn DsMakePasswordCredentialsW<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsW(user : ::windows_core::PCWSTR, domain : ::windows_core::PCWSTR, password : ::windows_core::PCWSTR, pauthidentity : *mut *mut ::core::ffi::c_void) -> u32);
    DsMakePasswordCredentialsW(user.into_param().abi(), domain.into_param().abi(), password.into_param().abi(), pauthidentity)
}
#[inline]
pub unsafe fn DsMakeSpnA<P0, P1, P2, P3>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P3, pcspnlength: *mut u32, pszspn: ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsMakeSpnA(serviceclass : ::windows_core::PCSTR, servicename : ::windows_core::PCSTR, instancename : ::windows_core::PCSTR, instanceport : u16, referrer : ::windows_core::PCSTR, pcspnlength : *mut u32, pszspn : ::windows_core::PSTR) -> u32);
    DsMakeSpnA(serviceclass.into_param().abi(), servicename.into_param().abi(), instancename.into_param().abi(), instanceport, referrer.into_param().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[inline]
pub unsafe fn DsMakeSpnW<P0, P1, P2, P3>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P3, pcspnlength: *mut u32, pszspn: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("dsparse.dll" "system" fn DsMakeSpnW(serviceclass : ::windows_core::PCWSTR, servicename : ::windows_core::PCWSTR, instancename : ::windows_core::PCWSTR, instanceport : u16, referrer : ::windows_core::PCWSTR, pcspnlength : *mut u32, pszspn : ::windows_core::PWSTR) -> u32);
    DsMakeSpnW(serviceclass.into_param().abi(), servicename.into_param().abi(), instancename.into_param().abi(), instanceport, referrer.into_param().abi(), pcspnlength, ::core::mem::transmute(pszspn))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsMapSchemaGuidsA<P0>(hds: P0, rguids: &[::windows_core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPA) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsA(hds : super::super::Foundation:: HANDLE, cguids : u32, rguids : *const ::windows_core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPA) -> u32);
    DsMapSchemaGuidsA(hds.into_param().abi(), rguids.len().try_into().unwrap(), ::core::mem::transmute(rguids.as_ptr()), ppguidmap)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsMapSchemaGuidsW<P0>(hds: P0, rguids: &[::windows_core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPW) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsW(hds : super::super::Foundation:: HANDLE, cguids : u32, rguids : *const ::windows_core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPW) -> u32);
    DsMapSchemaGuidsW(hds.into_param().abi(), rguids.len().try_into().unwrap(), ::core::mem::transmute(rguids.as_ptr()), ppguidmap)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
#[inline]
pub unsafe fn DsMergeForestTrustInformationW<P0>(domainname: P0, newforesttrustinfo: *const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo: ::core::option::Option<*const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION>, mergedforesttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsMergeForestTrustInformationW(domainname : ::windows_core::PCWSTR, newforesttrustinfo : *const super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo : *const super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION, mergedforesttrustinfo : *mut *mut super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION) -> u32);
    DsMergeForestTrustInformationW(domainname.into_param().abi(), newforesttrustinfo, ::core::mem::transmute(oldforesttrustinfo.unwrap_or(::std::ptr::null())), mergedforesttrustinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsQuerySitesByCostA<P0, P1>(hds: P0, pszfromsite: P1, rgsztosites: &[::windows_core::PCSTR], dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostA(hds : super::super::Foundation:: HANDLE, pszfromsite : ::windows_core::PCSTR, rgsztosites : *const ::windows_core::PCSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut *mut DS_SITE_COST_INFO) -> u32);
    DsQuerySitesByCostA(hds.into_param().abi(), pszfromsite.into_param().abi(), ::core::mem::transmute(rgsztosites.as_ptr()), rgsztosites.len().try_into().unwrap(), dwflags, prgsiteinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsQuerySitesByCostW<P0, P1>(hds: P0, pwszfromsite: P1, rgwsztosites: &[::windows_core::PCWSTR], dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostW(hds : super::super::Foundation:: HANDLE, pwszfromsite : ::windows_core::PCWSTR, rgwsztosites : *const ::windows_core::PCWSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut *mut DS_SITE_COST_INFO) -> u32);
    DsQuerySitesByCostW(hds.into_param().abi(), pwszfromsite.into_param().abi(), ::core::mem::transmute(rgwsztosites.as_ptr()), rgwsztosites.len().try_into().unwrap(), dwflags, prgsiteinfo)
}
#[inline]
pub unsafe fn DsQuerySitesFree(rgsiteinfo: *const DS_SITE_COST_INFO) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsQuerySitesFree(rgsiteinfo : *const DS_SITE_COST_INFO));
    DsQuerySitesFree(rgsiteinfo)
}
#[inline]
pub unsafe fn DsQuoteRdnValueA(psunquotedrdnvalue: &[u8], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: ::windows_core::PSTR) -> u32 {
    ::windows_targets::link!("dsparse.dll" "system" fn DsQuoteRdnValueA(cunquotedrdnvaluelength : u32, psunquotedrdnvalue : ::windows_core::PCSTR, pcquotedrdnvaluelength : *mut u32, psquotedrdnvalue : ::windows_core::PSTR) -> u32);
    DsQuoteRdnValueA(psunquotedrdnvalue.len().try_into().unwrap(), ::core::mem::transmute(psunquotedrdnvalue.as_ptr()), pcquotedrdnvaluelength, ::core::mem::transmute(psquotedrdnvalue))
}
#[inline]
pub unsafe fn DsQuoteRdnValueW(psunquotedrdnvalue: &[u16], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: ::windows_core::PWSTR) -> u32 {
    ::windows_targets::link!("dsparse.dll" "system" fn DsQuoteRdnValueW(cunquotedrdnvaluelength : u32, psunquotedrdnvalue : ::windows_core::PCWSTR, pcquotedrdnvaluelength : *mut u32, psquotedrdnvalue : ::windows_core::PWSTR) -> u32);
    DsQuoteRdnValueW(psunquotedrdnvalue.len().try_into().unwrap(), ::core::mem::transmute(psunquotedrdnvalue.as_ptr()), pcquotedrdnvaluelength, ::core::mem::transmute(psquotedrdnvalue))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsDomainA<P0, P1>(hds: P0, domaindn: P1) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainA(hds : super::super::Foundation:: HANDLE, domaindn : ::windows_core::PCSTR) -> u32);
    DsRemoveDsDomainA(hds.into_param().abi(), domaindn.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsDomainW<P0, P1>(hds: P0, domaindn: P1) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainW(hds : super::super::Foundation:: HANDLE, domaindn : ::windows_core::PCWSTR) -> u32);
    DsRemoveDsDomainW(hds.into_param().abi(), domaindn.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsServerA<P0, P1, P2, P3>(hds: P0, serverdn: P1, domaindn: P2, flastdcindomain: ::core::option::Option<*mut super::super::Foundation::BOOL>, fcommit: P3) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsServerA(hds : super::super::Foundation:: HANDLE, serverdn : ::windows_core::PCSTR, domaindn : ::windows_core::PCSTR, flastdcindomain : *mut super::super::Foundation:: BOOL, fcommit : super::super::Foundation:: BOOL) -> u32);
    DsRemoveDsServerA(hds.into_param().abi(), serverdn.into_param().abi(), domaindn.into_param().abi(), ::core::mem::transmute(flastdcindomain.unwrap_or(::std::ptr::null_mut())), fcommit.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsRemoveDsServerW<P0, P1, P2, P3>(hds: P0, serverdn: P1, domaindn: P2, flastdcindomain: ::core::option::Option<*mut super::super::Foundation::BOOL>, fcommit: P3) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsServerW(hds : super::super::Foundation:: HANDLE, serverdn : ::windows_core::PCWSTR, domaindn : ::windows_core::PCWSTR, flastdcindomain : *mut super::super::Foundation:: BOOL, fcommit : super::super::Foundation:: BOOL) -> u32);
    DsRemoveDsServerW(hds.into_param().abi(), serverdn.into_param().abi(), domaindn.into_param().abi(), ::core::mem::transmute(flastdcindomain.unwrap_or(::std::ptr::null_mut())), fcommit.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaAddA<P0, P1, P2, P3, P4>(hds: P0, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: ::core::option::Option<*const SCHEDULE>, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaAddA(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCSTR, sourcedsadn : ::windows_core::PCSTR, transportdn : ::windows_core::PCSTR, sourcedsaaddress : ::windows_core::PCSTR, pschedule : *const SCHEDULE, options : u32) -> u32);
    DsReplicaAddA(hds.into_param().abi(), namecontext.into_param().abi(), sourcedsadn.into_param().abi(), transportdn.into_param().abi(), sourcedsaaddress.into_param().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaAddW<P0, P1, P2, P3, P4>(hds: P0, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: ::core::option::Option<*const SCHEDULE>, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaAddW(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCWSTR, sourcedsadn : ::windows_core::PCWSTR, transportdn : ::windows_core::PCWSTR, sourcedsaaddress : ::windows_core::PCWSTR, pschedule : *const SCHEDULE, options : u32) -> u32);
    DsReplicaAddW(hds.into_param().abi(), namecontext.into_param().abi(), sourcedsadn.into_param().abi(), transportdn.into_param().abi(), sourcedsaaddress.into_param().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaConsistencyCheck<P0>(hds: P0, taskid: DS_KCC_TASKID, dwflags: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaConsistencyCheck(hds : super::super::Foundation:: HANDLE, taskid : DS_KCC_TASKID, dwflags : u32) -> u32);
    DsReplicaConsistencyCheck(hds.into_param().abi(), taskid, dwflags)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaDelA<P0, P1, P2>(hds: P0, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaDelA(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCSTR, dsasrc : ::windows_core::PCSTR, options : u32) -> u32);
    DsReplicaDelA(hds.into_param().abi(), namecontext.into_param().abi(), dsasrc.into_param().abi(), options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaDelW<P0, P1, P2>(hds: P0, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaDelW(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCWSTR, dsasrc : ::windows_core::PCWSTR, options : u32) -> u32);
    DsReplicaDelW(hds.into_param().abi(), namecontext.into_param().abi(), dsasrc.into_param().abi(), options)
}
#[inline]
pub unsafe fn DsReplicaFreeInfo(infotype: DS_REPL_INFO_TYPE, pinfo: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaFreeInfo(infotype : DS_REPL_INFO_TYPE, pinfo : *const ::core::ffi::c_void));
    DsReplicaFreeInfo(infotype, pinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaGetInfo2W<P0, P1, P2, P3>(hds: P0, infotype: DS_REPL_INFO_TYPE, pszobject: P1, puuidforsourcedsaobjguid: ::core::option::Option<*const ::windows_core::GUID>, pszattributename: P2, pszvalue: P3, dwflags: u32, dwenumerationcontext: u32, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaGetInfo2W(hds : super::super::Foundation:: HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : ::windows_core::PCWSTR, puuidforsourcedsaobjguid : *const ::windows_core::GUID, pszattributename : ::windows_core::PCWSTR, pszvalue : ::windows_core::PCWSTR, dwflags : u32, dwenumerationcontext : u32, ppinfo : *mut *mut ::core::ffi::c_void) -> u32);
    DsReplicaGetInfo2W(hds.into_param().abi(), infotype, pszobject.into_param().abi(), ::core::mem::transmute(puuidforsourcedsaobjguid.unwrap_or(::std::ptr::null())), pszattributename.into_param().abi(), pszvalue.into_param().abi(), dwflags, dwenumerationcontext, ppinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaGetInfoW<P0, P1>(hds: P0, infotype: DS_REPL_INFO_TYPE, pszobject: P1, puuidforsourcedsaobjguid: ::core::option::Option<*const ::windows_core::GUID>, ppinfo: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaGetInfoW(hds : super::super::Foundation:: HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : ::windows_core::PCWSTR, puuidforsourcedsaobjguid : *const ::windows_core::GUID, ppinfo : *mut *mut ::core::ffi::c_void) -> u32);
    DsReplicaGetInfoW(hds.into_param().abi(), infotype, pszobject.into_param().abi(), ::core::mem::transmute(puuidforsourcedsaobjguid.unwrap_or(::std::ptr::null())), ppinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaModifyA<P0, P1, P2, P3>(hds: P0, namecontext: P1, puuidsourcedsa: ::core::option::Option<*const ::windows_core::GUID>, transportdn: P2, sourcedsaaddress: P3, pschedule: ::core::option::Option<*const SCHEDULE>, replicaflags: u32, modifyfields: u32, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaModifyA(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCSTR, puuidsourcedsa : *const ::windows_core::GUID, transportdn : ::windows_core::PCSTR, sourcedsaaddress : ::windows_core::PCSTR, pschedule : *const SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
    DsReplicaModifyA(hds.into_param().abi(), namecontext.into_param().abi(), ::core::mem::transmute(puuidsourcedsa.unwrap_or(::std::ptr::null())), transportdn.into_param().abi(), sourcedsaaddress.into_param().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), replicaflags, modifyfields, options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaModifyW<P0, P1, P2, P3>(hds: P0, namecontext: P1, puuidsourcedsa: ::core::option::Option<*const ::windows_core::GUID>, transportdn: P2, sourcedsaaddress: P3, pschedule: ::core::option::Option<*const SCHEDULE>, replicaflags: u32, modifyfields: u32, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaModifyW(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCWSTR, puuidsourcedsa : *const ::windows_core::GUID, transportdn : ::windows_core::PCWSTR, sourcedsaaddress : ::windows_core::PCWSTR, pschedule : *const SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
    DsReplicaModifyW(hds.into_param().abi(), namecontext.into_param().abi(), ::core::mem::transmute(puuidsourcedsa.unwrap_or(::std::ptr::null())), transportdn.into_param().abi(), sourcedsaaddress.into_param().abi(), ::core::mem::transmute(pschedule.unwrap_or(::std::ptr::null())), replicaflags, modifyfields, options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncA<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows_core::GUID, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncA(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCSTR, puuiddsasrc : *const ::windows_core::GUID, options : u32) -> u32);
    DsReplicaSyncA(hds.into_param().abi(), namecontext.into_param().abi(), puuiddsasrc, options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncAllA<P0, P1>(hds: P0, psznamecontext: P1, ulflags: u32, pfncallback: isize, pcallbackdata: ::core::option::Option<*const ::core::ffi::c_void>, perrors: ::core::option::Option<*mut *mut *mut DS_REPSYNCALL_ERRINFOA>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllA(hds : super::super::Foundation:: HANDLE, psznamecontext : ::windows_core::PCSTR, ulflags : u32, pfncallback : isize, pcallbackdata : *const ::core::ffi::c_void, perrors : *mut *mut *mut DS_REPSYNCALL_ERRINFOA) -> u32);
    DsReplicaSyncAllA(hds.into_param().abi(), psznamecontext.into_param().abi(), ulflags, pfncallback, ::core::mem::transmute(pcallbackdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perrors.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncAllW<P0, P1>(hds: P0, psznamecontext: P1, ulflags: u32, pfncallback: isize, pcallbackdata: ::core::option::Option<*const ::core::ffi::c_void>, perrors: ::core::option::Option<*mut *mut *mut DS_REPSYNCALL_ERRINFOW>) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllW(hds : super::super::Foundation:: HANDLE, psznamecontext : ::windows_core::PCWSTR, ulflags : u32, pfncallback : isize, pcallbackdata : *const ::core::ffi::c_void, perrors : *mut *mut *mut DS_REPSYNCALL_ERRINFOW) -> u32);
    DsReplicaSyncAllW(hds.into_param().abi(), psznamecontext.into_param().abi(), ulflags, pfncallback, ::core::mem::transmute(pcallbackdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perrors.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaSyncW<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows_core::GUID, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncW(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCWSTR, puuiddsasrc : *const ::windows_core::GUID, options : u32) -> u32);
    DsReplicaSyncW(hds.into_param().abi(), namecontext.into_param().abi(), puuiddsasrc, options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaUpdateRefsA<P0, P1, P2>(hds: P0, namecontext: P1, dsadest: P2, puuiddsadest: *const ::windows_core::GUID, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsA(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCSTR, dsadest : ::windows_core::PCSTR, puuiddsadest : *const ::windows_core::GUID, options : u32) -> u32);
    DsReplicaUpdateRefsA(hds.into_param().abi(), namecontext.into_param().abi(), dsadest.into_param().abi(), puuiddsadest, options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaUpdateRefsW<P0, P1, P2>(hds: P0, namecontext: P1, dsadest: P2, puuiddsadest: *const ::windows_core::GUID, options: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsW(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCWSTR, dsadest : ::windows_core::PCWSTR, puuiddsadest : *const ::windows_core::GUID, options : u32) -> u32);
    DsReplicaUpdateRefsW(hds.into_param().abi(), namecontext.into_param().abi(), dsadest.into_param().abi(), puuiddsadest, options)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaVerifyObjectsA<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows_core::GUID, uloptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsA(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCSTR, puuiddsasrc : *const ::windows_core::GUID, uloptions : u32) -> u32);
    DsReplicaVerifyObjectsA(hds.into_param().abi(), namecontext.into_param().abi(), puuiddsasrc, uloptions)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsReplicaVerifyObjectsW<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const ::windows_core::GUID, uloptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsW(hds : super::super::Foundation:: HANDLE, namecontext : ::windows_core::PCWSTR, puuiddsasrc : *const ::windows_core::GUID, uloptions : u32) -> u32);
    DsReplicaVerifyObjectsW(hds.into_param().abi(), namecontext.into_param().abi(), puuiddsasrc, uloptions)
}
#[inline]
pub unsafe fn DsRoleFreeMemory(buffer: *mut ::core::ffi::c_void) {
    ::windows_targets::link!("netapi32.dll" "system" fn DsRoleFreeMemory(buffer : *mut ::core::ffi::c_void));
    DsRoleFreeMemory(buffer)
}
#[inline]
pub unsafe fn DsRoleGetPrimaryDomainInformation<P0>(lpserver: P0, infolevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer: *mut *mut u8) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsRoleGetPrimaryDomainInformation(lpserver : ::windows_core::PCWSTR, infolevel : DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer : *mut *mut u8) -> u32);
    DsRoleGetPrimaryDomainInformation(lpserver.into_param().abi(), infolevel, buffer)
}
#[inline]
pub unsafe fn DsServerRegisterSpnA<P0, P1>(operation: DS_SPN_WRITE_OP, serviceclass: P0, userobjectdn: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnA(operation : DS_SPN_WRITE_OP, serviceclass : ::windows_core::PCSTR, userobjectdn : ::windows_core::PCSTR) -> u32);
    DsServerRegisterSpnA(operation, serviceclass.into_param().abi(), userobjectdn.into_param().abi())
}
#[inline]
pub unsafe fn DsServerRegisterSpnW<P0, P1>(operation: DS_SPN_WRITE_OP, serviceclass: P0, userobjectdn: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnW(operation : DS_SPN_WRITE_OP, serviceclass : ::windows_core::PCWSTR, userobjectdn : ::windows_core::PCWSTR) -> u32);
    DsServerRegisterSpnW(operation, serviceclass.into_param().abi(), userobjectdn.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsUnBindA(phds: *const super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsUnBindA(phds : *const super::super::Foundation:: HANDLE) -> u32);
    DsUnBindA(phds)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsUnBindW(phds: *const super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsUnBindW(phds : *const super::super::Foundation:: HANDLE) -> u32);
    DsUnBindW(phds)
}
#[inline]
pub unsafe fn DsUnquoteRdnValueA(psquotedrdnvalue: &[u8], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: ::windows_core::PSTR) -> u32 {
    ::windows_targets::link!("dsparse.dll" "system" fn DsUnquoteRdnValueA(cquotedrdnvaluelength : u32, psquotedrdnvalue : ::windows_core::PCSTR, pcunquotedrdnvaluelength : *mut u32, psunquotedrdnvalue : ::windows_core::PSTR) -> u32);
    DsUnquoteRdnValueA(psquotedrdnvalue.len().try_into().unwrap(), ::core::mem::transmute(psquotedrdnvalue.as_ptr()), pcunquotedrdnvaluelength, ::core::mem::transmute(psunquotedrdnvalue))
}
#[inline]
pub unsafe fn DsUnquoteRdnValueW(psquotedrdnvalue: &[u16], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: ::windows_core::PWSTR) -> u32 {
    ::windows_targets::link!("dsparse.dll" "system" fn DsUnquoteRdnValueW(cquotedrdnvaluelength : u32, psquotedrdnvalue : ::windows_core::PCWSTR, pcunquotedrdnvaluelength : *mut u32, psunquotedrdnvalue : ::windows_core::PWSTR) -> u32);
    DsUnquoteRdnValueW(psquotedrdnvalue.len().try_into().unwrap(), ::core::mem::transmute(psquotedrdnvalue.as_ptr()), pcunquotedrdnvaluelength, ::core::mem::transmute(psunquotedrdnvalue))
}
#[inline]
pub unsafe fn DsValidateSubnetNameA<P0>(subnetname: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsValidateSubnetNameA(subnetname : ::windows_core::PCSTR) -> u32);
    DsValidateSubnetNameA(subnetname.into_param().abi())
}
#[inline]
pub unsafe fn DsValidateSubnetNameW<P0>(subnetname: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netapi32.dll" "system" fn DsValidateSubnetNameW(subnetname : ::windows_core::PCWSTR) -> u32);
    DsValidateSubnetNameW(subnetname.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsWriteAccountSpnA<P0, P1>(hds: P0, operation: DS_SPN_WRITE_OP, pszaccount: P1, rpszspn: &[::windows_core::PCSTR]) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnA(hds : super::super::Foundation:: HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : ::windows_core::PCSTR, cspn : u32, rpszspn : *const ::windows_core::PCSTR) -> u32);
    DsWriteAccountSpnA(hds.into_param().abi(), operation, pszaccount.into_param().abi(), rpszspn.len().try_into().unwrap(), ::core::mem::transmute(rpszspn.as_ptr()))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DsWriteAccountSpnW<P0, P1>(hds: P0, operation: DS_SPN_WRITE_OP, pszaccount: P1, rpszspn: &[::windows_core::PCWSTR]) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnW(hds : super::super::Foundation:: HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : ::windows_core::PCWSTR, cspn : u32, rpszspn : *const ::windows_core::PCWSTR) -> u32);
    DsWriteAccountSpnW(hds.into_param().abi(), operation, pszaccount.into_param().abi(), rpszspn.len().try_into().unwrap(), ::core::mem::transmute(rpszspn.as_ptr()))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeADsMem(pmem: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("activeds.dll" "system" fn FreeADsMem(pmem : *mut ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    FreeADsMem(pmem)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeADsStr<P0>(pstr: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn FreeADsStr(pstr : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    FreeADsStr(pstr.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn PropVariantToAdsType(pvariant: *mut super::super::System::Variant::VARIANT, dwnumvariant: u32, ppadsvalues: *mut *mut ADSVALUE, pdwnumvalues: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("activeds.dll" "system" fn PropVariantToAdsType(pvariant : *mut super::super::System::Variant:: VARIANT, dwnumvariant : u32, ppadsvalues : *mut *mut ADSVALUE, pdwnumvalues : *mut u32) -> ::windows_core::HRESULT);
    PropVariantToAdsType(pvariant, dwnumvariant, ppadsvalues, pdwnumvalues).ok()
}
#[inline]
pub unsafe fn ReallocADsMem(poldmem: *mut ::core::ffi::c_void, cbold: u32, cbnew: u32) -> *mut ::core::ffi::c_void {
    ::windows_targets::link!("activeds.dll" "system" fn ReallocADsMem(poldmem : *mut ::core::ffi::c_void, cbold : u32, cbnew : u32) -> *mut ::core::ffi::c_void);
    ReallocADsMem(poldmem, cbold, cbnew)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReallocADsStr<P0>(ppstr: *mut ::windows_core::PWSTR, pstr: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn ReallocADsStr(ppstr : *mut ::windows_core::PWSTR, pstr : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ReallocADsStr(ppstr, pstr.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SecurityDescriptorToBinarySD<P0, P1, P2>(vvarsecdes: super::super::System::Variant::VARIANT, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, pdwsdlength: *mut u32, pszservername: P0, username: P1, password: P2, dwflags: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("activeds.dll" "system" fn SecurityDescriptorToBinarySD(vvarsecdes : super::super::System::Variant:: VARIANT, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, pdwsdlength : *mut u32, pszservername : ::windows_core::PCWSTR, username : ::windows_core::PCWSTR, password : ::windows_core::PCWSTR, dwflags : u32) -> ::windows_core::HRESULT);
    SecurityDescriptorToBinarySD(::core::mem::transmute(vvarsecdes), ppsecuritydescriptor, pdwsdlength, pszservername.into_param().abi(), username.into_param().abi(), password.into_param().abi(), dwflags).ok()
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADs {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADs, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADs {
    type Vtable = IADs_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd8256d0_fd15_11ce_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Schema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Put: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncontrolcode: i32, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetInfoEx: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsADSystemInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsADSystemInfo {
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComputerName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SiteName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SiteName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DomainShortName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DomainShortName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DomainDNSName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DomainDNSName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ForestDNSName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ForestDNSName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PDCRoleOwner(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PDCRoleOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SchemaRoleOwner(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SchemaRoleOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNativeMode(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsNativeMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAnyDCName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAnyDCName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDCSiteName<P0>(&self, szserver: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDCSiteName)(::windows_core::Interface::as_raw(self), szserver.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RefreshSchemaCache(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RefreshSchemaCache)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetTrees(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTrees)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsADSystemInfo, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsADSystemInfo {
    type Vtable = IADsADSystemInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsADSystemInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bb11929_afd1_11d2_9cb9_0000f87a369e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsADSystemInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DomainShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DomainDNSName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ForestDNSName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PDCRoleOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SchemaRoleOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNativeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNativeMode: usize,
    pub GetAnyDCName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdcname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetDCSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, pszsitename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RefreshSchemaCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetTrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtrees: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetTrees: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsAccessControlEntry(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlEntry {
    pub unsafe fn AccessMask(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AccessMask)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccessMask(&self, lnaccessmask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAccessMask)(::windows_core::Interface::as_raw(self), lnaccessmask).ok()
    }
    pub unsafe fn AceType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AceType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAceType(&self, lnacetype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAceType)(::windows_core::Interface::as_raw(self), lnacetype).ok()
    }
    pub unsafe fn AceFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AceFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAceFlags(&self, lnaceflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAceFlags)(::windows_core::Interface::as_raw(self), lnaceflags).ok()
    }
    pub unsafe fn Flags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Flags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, lnflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), lnflags).ok()
    }
    pub unsafe fn ObjectType(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ObjectType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetObjectType<P0>(&self, bstrobjecttype: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetObjectType)(::windows_core::Interface::as_raw(self), bstrobjecttype.into_param().abi()).ok()
    }
    pub unsafe fn InheritedObjectType(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InheritedObjectType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInheritedObjectType<P0>(&self, bstrinheritedobjecttype: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInheritedObjectType)(::windows_core::Interface::as_raw(self), bstrinheritedobjecttype.into_param().abi()).ok()
    }
    pub unsafe fn Trustee(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Trustee)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTrustee<P0>(&self, bstrtrustee: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTrustee)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsAccessControlEntry, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsAccessControlEntry {
    type Vtable = IADsAccessControlEntry_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsAccessControlEntry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4f3a14c_9bdd_11d0_852c_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlEntry_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AccessMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAccessMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaccessmask: i32) -> ::windows_core::HRESULT,
    pub AceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnacetype: i32) -> ::windows_core::HRESULT,
    pub AceFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAceFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaceflags: i32) -> ::windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows_core::HRESULT,
    pub ObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjecttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub InheritedObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInheritedObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinheritedobjecttype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Trustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsAccessControlList(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlList {
    pub unsafe fn AclRevision(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AclRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAclRevision(&self, lnaclrevision: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAclRevision)(::windows_core::Interface::as_raw(self), lnaclrevision).ok()
    }
    pub unsafe fn AceCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAceCount(&self, lnacecount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAceCount)(::windows_core::Interface::as_raw(self), lnacecount).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAce<P0>(&self, paccesscontrolentry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).AddAce)(::windows_core::Interface::as_raw(self), paccesscontrolentry.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveAce<P0>(&self, paccesscontrolentry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).RemoveAce)(::windows_core::Interface::as_raw(self), paccesscontrolentry.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyAccessList(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CopyAccessList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsAccessControlList, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsAccessControlList {
    type Vtable = IADsAccessControlList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsAccessControlList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7ee91cc_9bdd_11d0_852c_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAccessControlList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AclRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAclRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaclrevision: i32) -> ::windows_core::HRESULT,
    pub AceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnacecount: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAce: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveAce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paccesscontrolentry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveAce: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyAccessList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaccesscontrollist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyAccessList: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsAcl(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsAcl {
    pub unsafe fn ProtectedAttrName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProtectedAttrName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtectedAttrName<P0>(&self, bstrprotectedattrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProtectedAttrName)(::windows_core::Interface::as_raw(self), bstrprotectedattrname.into_param().abi()).ok()
    }
    pub unsafe fn SubjectName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SubjectName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubjectName<P0>(&self, bstrsubjectname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubjectName)(::windows_core::Interface::as_raw(self), bstrsubjectname.into_param().abi()).ok()
    }
    pub unsafe fn Privileges(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Privileges)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrivileges(&self, lnprivileges: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrivileges)(::windows_core::Interface::as_raw(self), lnprivileges).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyAcl(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CopyAcl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsAcl, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsAcl {
    type Vtable = IADsAcl_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsAcl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8452d3ab_0869_11d1_a377_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsAcl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ProtectedAttrName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetProtectedAttrName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotectedattrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SubjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSubjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Privileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPrivileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnprivileges: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppacl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyAcl: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsAggregatee(::windows_core::IUnknown);
impl IADsAggregatee {
    pub unsafe fn ConnectAsAggregatee<P0>(&self, pouterunknown: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).ConnectAsAggregatee)(::windows_core::Interface::as_raw(self), pouterunknown.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectAsAggregatee(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectAsAggregatee)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RelinquishInterface(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RelinquishInterface)(::windows_core::Interface::as_raw(self), riid).ok()
    }
    pub unsafe fn RestoreInterface(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreInterface)(::windows_core::Interface::as_raw(self), riid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IADsAggregatee, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IADsAggregatee {
    type Vtable = IADsAggregatee_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IADsAggregatee {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1346ce8c_9039_11d0_8528_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsAggregatee_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ConnectAsAggregatee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pouterunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisconnectAsAggregatee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RelinquishInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RestoreInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsAggregator(::windows_core::IUnknown);
impl IADsAggregator {
    pub unsafe fn ConnectAsAggregator<P0>(&self, paggregatee: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).ConnectAsAggregator)(::windows_core::Interface::as_raw(self), paggregatee.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectAsAggregator(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectAsAggregator)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IADsAggregator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IADsAggregator {
    type Vtable = IADsAggregator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IADsAggregator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52db5fb0_941f_11d0_8529_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsAggregator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ConnectAsAggregator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paggregatee: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisconnectAsAggregator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsBackLink(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsBackLink {
    pub unsafe fn RemoteID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteID(&self, lnremoteid: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRemoteID)(::windows_core::Interface::as_raw(self), lnremoteid).ok()
    }
    pub unsafe fn ObjectName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ObjectName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetObjectName<P0>(&self, bstrobjectname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetObjectName)(::windows_core::Interface::as_raw(self), bstrobjectname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsBackLink, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsBackLink {
    type Vtable = IADsBackLink_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsBackLink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd1302bd_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsBackLink_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RemoteID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetRemoteID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnremoteid: i32) -> ::windows_core::HRESULT,
    pub ObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsCaseIgnoreList(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsCaseIgnoreList {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CaseIgnoreList(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CaseIgnoreList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetCaseIgnoreList(&self, vcaseignorelist: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCaseIgnoreList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vcaseignorelist)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsCaseIgnoreList, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsCaseIgnoreList {
    type Vtable = IADsCaseIgnoreList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsCaseIgnoreList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b66b533_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCaseIgnoreList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CaseIgnoreList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CaseIgnoreList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetCaseIgnoreList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcaseignorelist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetCaseIgnoreList: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsClass(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsClass {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn PrimaryInterface(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrimaryInterface)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CLSID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CLSID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCLSID<P0>(&self, bstrclsid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCLSID)(::windows_core::Interface::as_raw(self), bstrclsid.into_param().abi()).ok()
    }
    pub unsafe fn OID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOID<P0>(&self, bstroid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOID)(::windows_core::Interface::as_raw(self), bstroid.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abstract(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Abstract)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAbstract<P0>(&self, fabstract: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAbstract)(::windows_core::Interface::as_raw(self), fabstract.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Auxiliary(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Auxiliary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuxiliary<P0>(&self, fauxiliary: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAuxiliary)(::windows_core::Interface::as_raw(self), fauxiliary.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MandatoryProperties(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MandatoryProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetMandatoryProperties(&self, vmandatoryproperties: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMandatoryProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vmandatoryproperties)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OptionalProperties(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OptionalProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetOptionalProperties(&self, voptionalproperties: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOptionalProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(voptionalproperties)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn NamingProperties(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NamingProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetNamingProperties(&self, vnamingproperties: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNamingProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vnamingproperties)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn DerivedFrom(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DerivedFrom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDerivedFrom(&self, vderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDerivedFrom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vderivedfrom)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AuxDerivedFrom(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AuxDerivedFrom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetAuxDerivedFrom(&self, vauxderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuxDerivedFrom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vauxderivedfrom)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PossibleSuperiors(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PossibleSuperiors)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPossibleSuperiors(&self, vpossiblesuperiors: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPossibleSuperiors)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vpossiblesuperiors)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Containment(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Containment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetContainment(&self, vcontainment: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContainment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vcontainment)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Container(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Container)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContainer<P0>(&self, fcontainer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetContainer)(::windows_core::Interface::as_raw(self), fcontainer.into_param().abi()).ok()
    }
    pub unsafe fn HelpFileName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HelpFileName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHelpFileName<P0>(&self, bstrhelpfilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetHelpFileName)(::windows_core::Interface::as_raw(self), bstrhelpfilename.into_param().abi()).ok()
    }
    pub unsafe fn HelpFileContext(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HelpFileContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHelpFileContext(&self, lnhelpfilecontext: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHelpFileContext)(::windows_core::Interface::as_raw(self), lnhelpfilecontext).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers(&self) -> ::windows_core::Result<IADsCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Qualifiers)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsClass, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsClass {
    type Vtable = IADsClass_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsClass {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8f93dd0_4ae0_11cf_9e73_00aa004a5691);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsClass_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrimaryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Abstract: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Abstract: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAbstract: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fabstract: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAbstract: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Auxiliary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Auxiliary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuxiliary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fauxiliary: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuxiliary: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MandatoryProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MandatoryProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetMandatoryProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vmandatoryproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetMandatoryProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OptionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OptionalProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetOptionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voptionalproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetOptionalProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub NamingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    NamingProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetNamingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vnamingproperties: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetNamingProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub DerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    DerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AuxDerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AuxDerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetAuxDerivedFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vauxderivedfrom: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetAuxDerivedFrom: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PossibleSuperiors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PossibleSuperiors: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPossibleSuperiors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpossiblesuperiors: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPossibleSuperiors: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Containment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Containment: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetContainment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcontainment: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetContainment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Container: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcontainer: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContainer: usize,
    pub HelpFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhelpfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub HelpFileContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetHelpFileContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnhelpfilecontext: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add<P0>(&self, bstrname: P0, vitem: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vitem)).ok()
    }
    pub unsafe fn Remove<P0>(&self, bstritemtoberemoved: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), bstritemtoberemoved.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetObject<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetObject)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsCollection {
    type Vtable = IADsCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72b945e0_253b_11cf_a988_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, vitem: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvitem: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetObject: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsComputer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsComputer {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn ComputerID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComputerID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Site(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Site)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn Location(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Location)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocation<P0>(&self, bstrlocation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocation)(::windows_core::Interface::as_raw(self), bstrlocation.into_param().abi()).ok()
    }
    pub unsafe fn PrimaryUser(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrimaryUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrimaryUser<P0>(&self, bstrprimaryuser: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPrimaryUser)(::windows_core::Interface::as_raw(self), bstrprimaryuser.into_param().abi()).ok()
    }
    pub unsafe fn Owner(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Owner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOwner<P0>(&self, bstrowner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwner)(::windows_core::Interface::as_raw(self), bstrowner.into_param().abi()).ok()
    }
    pub unsafe fn Division(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Division)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDivision<P0>(&self, bstrdivision: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDivision)(::windows_core::Interface::as_raw(self), bstrdivision.into_param().abi()).ok()
    }
    pub unsafe fn Department(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Department)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDepartment<P0>(&self, bstrdepartment: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDepartment)(::windows_core::Interface::as_raw(self), bstrdepartment.into_param().abi()).ok()
    }
    pub unsafe fn Role(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Role)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRole<P0>(&self, bstrrole: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRole)(::windows_core::Interface::as_raw(self), bstrrole.into_param().abi()).ok()
    }
    pub unsafe fn OperatingSystem(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OperatingSystem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOperatingSystem<P0>(&self, bstroperatingsystem: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOperatingSystem)(::windows_core::Interface::as_raw(self), bstroperatingsystem.into_param().abi()).ok()
    }
    pub unsafe fn OperatingSystemVersion(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OperatingSystemVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOperatingSystemVersion<P0>(&self, bstroperatingsystemversion: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOperatingSystemVersion)(::windows_core::Interface::as_raw(self), bstroperatingsystemversion.into_param().abi()).ok()
    }
    pub unsafe fn Model(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Model)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetModel<P0>(&self, bstrmodel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetModel)(::windows_core::Interface::as_raw(self), bstrmodel.into_param().abi()).ok()
    }
    pub unsafe fn Processor(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Processor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProcessor<P0>(&self, bstrprocessor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProcessor)(::windows_core::Interface::as_raw(self), bstrprocessor.into_param().abi()).ok()
    }
    pub unsafe fn ProcessorCount(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProcessorCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProcessorCount<P0>(&self, bstrprocessorcount: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProcessorCount)(::windows_core::Interface::as_raw(self), bstrprocessorcount.into_param().abi()).ok()
    }
    pub unsafe fn MemorySize(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MemorySize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMemorySize<P0>(&self, bstrmemorysize: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMemorySize)(::windows_core::Interface::as_raw(self), bstrmemorysize.into_param().abi()).ok()
    }
    pub unsafe fn StorageCapacity(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StorageCapacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStorageCapacity<P0>(&self, bstrstoragecapacity: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetStorageCapacity)(::windows_core::Interface::as_raw(self), bstrstoragecapacity.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn NetAddresses(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetNetAddresses(&self, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vnetaddresses)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsComputer, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsComputer {
    type Vtable = IADsComputer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsComputer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefe3cc70_1d9f_11cf_b1f3_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputer_Vtbl {
    pub base__: IADs_Vtbl,
    pub ComputerID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Site: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PrimaryUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPrimaryUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprimaryuser: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Division: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdivision: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOperatingSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperatingsystem: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OperatingSystemVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOperatingSystemVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperatingsystemversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmodel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Processor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprocessor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ProcessorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetProcessorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprocessorcount: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MemorySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetMemorySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmemorysize: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StorageCapacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetStorageCapacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstoragecapacity: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub NetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    NetAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetNetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetNetAddresses: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsComputerOperations(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsComputerOperations {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Shutdown<P0>(&self, breboot: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Shutdown)(::windows_core::Interface::as_raw(self), breboot.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsComputerOperations, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsComputerOperations {
    type Vtable = IADsComputerOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsComputerOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef497680_1d9f_11cf_b1f3_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsComputerOperations_Vtbl {
    pub base__: IADs_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, breboot: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Shutdown: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsContainer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsContainer {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Filter(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Filter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetFilter(&self, var: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(var)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Hints(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Hints)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetHints(&self, vhints: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vhints)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObject<P0, P1>(&self, classname: P0, relativename: P1) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetObject)(::windows_core::Interface::as_raw(self), classname.into_param().abi(), relativename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1>(&self, classname: P0, relativename: P1) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Create)(::windows_core::Interface::as_raw(self), classname.into_param().abi(), relativename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete<P0, P1>(&self, bstrclassname: P0, bstrrelativename: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), bstrclassname.into_param().abi(), bstrrelativename.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyHere<P0, P1>(&self, sourcename: P0, newname: P1) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CopyHere)(::windows_core::Interface::as_raw(self), sourcename.into_param().abi(), newname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveHere<P0, P1>(&self, sourcename: P0, newname: P1) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveHere)(::windows_core::Interface::as_raw(self), sourcename.into_param().abi(), newname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsContainer, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsContainer {
    type Vtable = IADsContainer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsContainer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x001677d0_fd16_11ce_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsContainer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Filter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Hints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Hints: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vhints: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetHints: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classname: ::std::mem::MaybeUninit<::windows_core::BSTR>, relativename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classname: ::std::mem::MaybeUninit<::windows_core::BSTR>, relativename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrclassname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrelativename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyHere: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveHere: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsDNWithBinary(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithBinary {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn BinaryValue(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BinaryValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetBinaryValue(&self, vbinaryvalue: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBinaryValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbinaryvalue)).ok()
    }
    pub unsafe fn DNString(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DNString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDNString<P0>(&self, bstrdnstring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDNString)(::windows_core::Interface::as_raw(self), bstrdnstring.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsDNWithBinary, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsDNWithBinary {
    type Vtable = IADsDNWithBinary_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsDNWithBinary {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e99c0a2_f935_11d2_ba96_00c04fb6d0d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithBinary_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub BinaryValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    BinaryValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetBinaryValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbinaryvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetBinaryValue: usize,
    pub DNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsDNWithString(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithString {
    pub unsafe fn StringValue(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StringValue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStringValue<P0>(&self, bstrstringvalue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetStringValue)(::windows_core::Interface::as_raw(self), bstrstringvalue.into_param().abi()).ok()
    }
    pub unsafe fn DNString(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DNString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDNString<P0>(&self, bstrdnstring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDNString)(::windows_core::Interface::as_raw(self), bstrdnstring.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsDNWithString, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsDNWithString {
    type Vtable = IADsDNWithString_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsDNWithString {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x370df02e_f934_11d2_ba96_00c04fb6d0d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDNWithString_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstringvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsDeleteOps(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDeleteOps {
    pub unsafe fn DeleteObject(&self, lnflags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteObject)(::windows_core::Interface::as_raw(self), lnflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsDeleteOps, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsDeleteOps {
    type Vtable = IADsDeleteOps_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsDeleteOps {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2bd0902_8878_11d1_8c21_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDeleteOps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DeleteObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnflags: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsDomain(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsDomain {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkgroup(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsWorkgroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MinPasswordLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinPasswordLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinPasswordLength(&self, lnminpasswordlength: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinPasswordLength)(::windows_core::Interface::as_raw(self), lnminpasswordlength).ok()
    }
    pub unsafe fn MinPasswordAge(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinPasswordAge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinPasswordAge(&self, lnminpasswordage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinPasswordAge)(::windows_core::Interface::as_raw(self), lnminpasswordage).ok()
    }
    pub unsafe fn MaxPasswordAge(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxPasswordAge)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxPasswordAge(&self, lnmaxpasswordage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxPasswordAge)(::windows_core::Interface::as_raw(self), lnmaxpasswordage).ok()
    }
    pub unsafe fn MaxBadPasswordsAllowed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxBadPasswordsAllowed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxBadPasswordsAllowed)(::windows_core::Interface::as_raw(self), lnmaxbadpasswordsallowed).ok()
    }
    pub unsafe fn PasswordHistoryLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PasswordHistoryLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPasswordHistoryLength)(::windows_core::Interface::as_raw(self), lnpasswordhistorylength).ok()
    }
    pub unsafe fn PasswordAttributes(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PasswordAttributes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPasswordAttributes(&self, lnpasswordattributes: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPasswordAttributes)(::windows_core::Interface::as_raw(self), lnpasswordattributes).ok()
    }
    pub unsafe fn AutoUnlockInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AutoUnlockInterval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAutoUnlockInterval(&self, lnautounlockinterval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoUnlockInterval)(::windows_core::Interface::as_raw(self), lnautounlockinterval).ok()
    }
    pub unsafe fn LockoutObservationInterval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LockoutObservationInterval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLockoutObservationInterval)(::windows_core::Interface::as_raw(self), lnlockoutobservationinterval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsDomain, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsDomain {
    type Vtable = IADsDomain_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsDomain {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00e4c220_fd16_11ce_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsDomain_Vtbl {
    pub base__: IADs_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWorkgroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWorkgroup: usize,
    pub MinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinPasswordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnminpasswordlength: i32) -> ::windows_core::HRESULT,
    pub MinPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnminpasswordage: i32) -> ::windows_core::HRESULT,
    pub MaxPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxPasswordAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxpasswordage: i32) -> ::windows_core::HRESULT,
    pub MaxBadPasswordsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxBadPasswordsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxbadpasswordsallowed: i32) -> ::windows_core::HRESULT,
    pub PasswordHistoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPasswordHistoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpasswordhistorylength: i32) -> ::windows_core::HRESULT,
    pub PasswordAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPasswordAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpasswordattributes: i32) -> ::windows_core::HRESULT,
    pub AutoUnlockInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAutoUnlockInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnautounlockinterval: i32) -> ::windows_core::HRESULT,
    pub LockoutObservationInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetLockoutObservationInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnlockoutobservationinterval: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsEmail(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsEmail {
    pub unsafe fn Type(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, lntype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), lntype).ok()
    }
    pub unsafe fn Address(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Address)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAddress<P0>(&self, bstraddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAddress)(::windows_core::Interface::as_raw(self), bstraddress.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsEmail, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsEmail {
    type Vtable = IADsEmail_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsEmail {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97af011a_478e_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsEmail_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsExtension(::windows_core::IUnknown);
impl IADsExtension {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Operate(&self, dwcode: u32, vardata1: super::super::System::Variant::VARIANT, vardata2: super::super::System::Variant::VARIANT, vardata3: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Operate)(::windows_core::Interface::as_raw(self), dwcode, ::core::mem::transmute(vardata1), ::core::mem::transmute(vardata2), ::core::mem::transmute(vardata3)).ok()
    }
    pub unsafe fn PrivateGetIDsOfNames(&self, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrivateGetIDsOfNames)(::windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PrivateInvoke(&self, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrivateInvoke)(::windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IADsExtension, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IADsExtension {
    type Vtable = IADsExtension_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IADsExtension {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d35553c_d2b0_11d1_b17b_0000f87593a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADsExtension_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Operate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcode: u32, vardata1: super::super::System::Variant::VARIANT, vardata2: super::super::System::Variant::VARIANT, vardata3: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Operate: usize,
    pub PrivateGetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PrivateInvoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PrivateInvoke: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsFaxNumber(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFaxNumber {
    pub unsafe fn TelephoneNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TelephoneNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTelephoneNumber<P0>(&self, bstrtelephonenumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTelephoneNumber)(::windows_core::Interface::as_raw(self), bstrtelephonenumber.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Parameters(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Parameters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetParameters(&self, vparameters: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vparameters)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsFaxNumber, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsFaxNumber {
    type Vtable = IADsFaxNumber_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsFaxNumber {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa910dea9_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFaxNumber_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Parameters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vparameters: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetParameters: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsFileService(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileService {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn HostComputer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.HostComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHostComputer<P0>(&self, bstrhostcomputer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetHostComputer)(::windows_core::Interface::as_raw(self), bstrhostcomputer.into_param().abi()).ok()
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, bstrdisplayname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDisplayName)(::windows_core::Interface::as_raw(self), bstrdisplayname.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Version)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVersion<P0>(&self, bstrversion: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetVersion)(::windows_core::Interface::as_raw(self), bstrversion.into_param().abi()).ok()
    }
    pub unsafe fn ServiceType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceType(&self, lnservicetype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetServiceType)(::windows_core::Interface::as_raw(self), lnservicetype).ok()
    }
    pub unsafe fn StartType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartType(&self, lnstarttype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStartType)(::windows_core::Interface::as_raw(self), lnstarttype).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, bstrpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPath)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi()).ok()
    }
    pub unsafe fn StartupParameters(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartupParameters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartupParameters<P0>(&self, bstrstartupparameters: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetStartupParameters)(::windows_core::Interface::as_raw(self), bstrstartupparameters.into_param().abi()).ok()
    }
    pub unsafe fn ErrorControl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ErrorControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetErrorControl(&self, lnerrorcontrol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetErrorControl)(::windows_core::Interface::as_raw(self), lnerrorcontrol).ok()
    }
    pub unsafe fn LoadOrderGroup(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LoadOrderGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLoadOrderGroup<P0>(&self, bstrloadordergroup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLoadOrderGroup)(::windows_core::Interface::as_raw(self), bstrloadordergroup.into_param().abi()).ok()
    }
    pub unsafe fn ServiceAccountName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceAccountName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountName<P0>(&self, bstrserviceaccountname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetServiceAccountName)(::windows_core::Interface::as_raw(self), bstrserviceaccountname.into_param().abi()).ok()
    }
    pub unsafe fn ServiceAccountPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceAccountPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountPath<P0>(&self, bstrserviceaccountpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetServiceAccountPath)(::windows_core::Interface::as_raw(self), bstrserviceaccountpath.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Dependencies(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Dependencies)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDependencies(&self, vdependencies: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDependencies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vdependencies)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn MaxUserCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxUserCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxUserCount)(::windows_core::Interface::as_raw(self), lnmaxusercount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsFileService, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs, IADsService);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsFileService {
    type Vtable = IADsFileService_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsFileService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa89d1900_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileService_Vtbl {
    pub base__: IADsService_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsFileServiceOperations(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileServiceOperations {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Continue(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Continue)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, bstrnewpassword: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPassword)(::windows_core::Interface::as_raw(self), bstrnewpassword.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sessions(&self) -> ::windows_core::Result<IADsCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Sessions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> ::windows_core::Result<IADsCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Resources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsFileServiceOperations, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs, IADsServiceOperations);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsFileServiceOperations {
    type Vtable = IADsFileServiceOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsFileServiceOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa02ded10_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileServiceOperations_Vtbl {
    pub base__: IADsServiceOperations_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Sessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsessions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sessions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsFileShare(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileShare {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn CurrentUserCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUserCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn HostComputer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HostComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHostComputer<P0>(&self, bstrhostcomputer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetHostComputer)(::windows_core::Interface::as_raw(self), bstrhostcomputer.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, bstrpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi()).ok()
    }
    pub unsafe fn MaxUserCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxUserCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxUserCount)(::windows_core::Interface::as_raw(self), lnmaxusercount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsFileShare, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsFileShare {
    type Vtable = IADsFileShare_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsFileShare {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb6dcaf0_4b83_11cf_a995_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsFileShare_Vtbl {
    pub base__: IADs_Vtbl,
    pub CurrentUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub HostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxusercount: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsGroup(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsGroup {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> ::windows_core::Result<IADsMembers> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Members)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMember<P0>(&self, bstrmember: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsMember)(::windows_core::Interface::as_raw(self), bstrmember.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Add<P0>(&self, bstrnewitem: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), bstrnewitem.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, bstritemtoberemoved: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), bstritemtoberemoved.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsGroup, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsGroup {
    type Vtable = IADsGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27636b00_410f_11cf_b1ff_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsGroup_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmembers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Members: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmember: ::std::mem::MaybeUninit<::windows_core::BSTR>, bmember: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMember: usize,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnewitem: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstritemtoberemoved: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsHold(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsHold {
    pub unsafe fn ObjectName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ObjectName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetObjectName<P0>(&self, bstrobjectname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetObjectName)(::windows_core::Interface::as_raw(self), bstrobjectname.into_param().abi()).ok()
    }
    pub unsafe fn Amount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Amount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAmount(&self, lnamount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAmount)(::windows_core::Interface::as_raw(self), lnamount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsHold, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsHold {
    type Vtable = IADsHold_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsHold {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3eb3b37_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsHold_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnamount: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsLargeInteger(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsLargeInteger {
    pub unsafe fn HighPart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HighPart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHighPart(&self, lnhighpart: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHighPart)(::windows_core::Interface::as_raw(self), lnhighpart).ok()
    }
    pub unsafe fn LowPart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LowPart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLowPart(&self, lnlowpart: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLowPart)(::windows_core::Interface::as_raw(self), lnlowpart).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsLargeInteger, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsLargeInteger {
    type Vtable = IADsLargeInteger_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsLargeInteger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9068270b_0939_11d1_8be1_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLargeInteger_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub HighPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetHighPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnhighpart: i32) -> ::windows_core::HRESULT,
    pub LowPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetLowPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnlowpart: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsLocality(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsLocality {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn LocalityName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalityName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalityName<P0>(&self, bstrlocalityname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalityName)(::windows_core::Interface::as_raw(self), bstrlocalityname.into_param().abi()).ok()
    }
    pub unsafe fn PostalAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostalAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPostalAddress<P0>(&self, bstrpostaladdress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPostalAddress)(::windows_core::Interface::as_raw(self), bstrpostaladdress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SeeAlso)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSeeAlso)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsLocality, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsLocality {
    type Vtable = IADsLocality_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsLocality {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa05e03a2_effe_11cf_8abc_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsLocality_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSeeAlso: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsMembers(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsMembers {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Filter(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Filter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetFilter(&self, pvfilter: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvfilter)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsMembers, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsMembers {
    type Vtable = IADsMembers_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsMembers {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x451a0030_72ec_11cf_b03b_00aa006e0975);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsMembers_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvfilter: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Filter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvfilter: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetFilter: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsNameTranslate(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsNameTranslate {
    pub unsafe fn SetChaseReferral(&self, lnchasereferral: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetChaseReferral)(::windows_core::Interface::as_raw(self), lnchasereferral).ok()
    }
    pub unsafe fn Init<P0>(&self, lnsettype: i32, bstradspath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Init)(::windows_core::Interface::as_raw(self), lnsettype, bstradspath.into_param().abi()).ok()
    }
    pub unsafe fn InitEx<P0, P1, P2, P3>(&self, lnsettype: i32, bstradspath: P0, bstruserid: P1, bstrdomain: P2, bstrpassword: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).InitEx)(::windows_core::Interface::as_raw(self), lnsettype, bstradspath.into_param().abi(), bstruserid.into_param().abi(), bstrdomain.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    pub unsafe fn Set<P0>(&self, lnsettype: i32, bstradspath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Set)(::windows_core::Interface::as_raw(self), lnsettype, bstradspath.into_param().abi()).ok()
    }
    pub unsafe fn Get(&self, lnformattype: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Get)(::windows_core::Interface::as_raw(self), lnformattype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetEx(&self, lnformattype: i32, pvar: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEx)(::windows_core::Interface::as_raw(self), lnformattype, ::core::mem::transmute(pvar)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx(&self, lnformattype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEx)(::windows_core::Interface::as_raw(self), lnformattype, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsNameTranslate, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsNameTranslate {
    type Vtable = IADsNameTranslate_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsNameTranslate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1b272a3_3625_11d1_a3a4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNameTranslate_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetChaseReferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnchasereferral: i32) -> ::windows_core::HRESULT,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub InitEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruserid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsettype: i32, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pvar: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetEx: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsNamespaces(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsNamespaces {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn DefaultContainer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DefaultContainer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultContainer<P0>(&self, bstrdefaultcontainer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDefaultContainer)(::windows_core::Interface::as_raw(self), bstrdefaultcontainer.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsNamespaces, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsNamespaces {
    type Vtable = IADsNamespaces_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsNamespaces {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28b96ba0_b330_11cf_a9ad_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNamespaces_Vtbl {
    pub base__: IADs_Vtbl,
    pub DefaultContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDefaultContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdefaultcontainer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsNetAddress(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsNetAddress {
    pub unsafe fn AddressType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddressType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAddressType(&self, lnaddresstype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddressType)(::windows_core::Interface::as_raw(self), lnaddresstype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Address(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Address)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetAddress(&self, vaddress: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vaddress)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsNetAddress, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsNetAddress {
    type Vtable = IADsNetAddress_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsNetAddress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb21a50a9_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsNetAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetAddressType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnaddresstype: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Address: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vaddress: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetAddress: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsO(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsO {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn LocalityName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalityName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalityName<P0>(&self, bstrlocalityname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalityName)(::windows_core::Interface::as_raw(self), bstrlocalityname.into_param().abi()).ok()
    }
    pub unsafe fn PostalAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostalAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPostalAddress<P0>(&self, bstrpostaladdress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPostalAddress)(::windows_core::Interface::as_raw(self), bstrpostaladdress.into_param().abi()).ok()
    }
    pub unsafe fn TelephoneNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TelephoneNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTelephoneNumber<P0>(&self, bstrtelephonenumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTelephoneNumber)(::windows_core::Interface::as_raw(self), bstrtelephonenumber.into_param().abi()).ok()
    }
    pub unsafe fn FaxNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FaxNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFaxNumber<P0>(&self, bstrfaxnumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFaxNumber)(::windows_core::Interface::as_raw(self), bstrfaxnumber.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SeeAlso)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSeeAlso)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsO, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsO {
    type Vtable = IADsO_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsO {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1cd2dc6_effe_11cf_8abc_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsO_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSeeAlso: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsOU(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsOU {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn LocalityName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalityName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalityName<P0>(&self, bstrlocalityname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalityName)(::windows_core::Interface::as_raw(self), bstrlocalityname.into_param().abi()).ok()
    }
    pub unsafe fn PostalAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostalAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPostalAddress<P0>(&self, bstrpostaladdress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPostalAddress)(::windows_core::Interface::as_raw(self), bstrpostaladdress.into_param().abi()).ok()
    }
    pub unsafe fn TelephoneNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TelephoneNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTelephoneNumber<P0>(&self, bstrtelephonenumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTelephoneNumber)(::windows_core::Interface::as_raw(self), bstrtelephonenumber.into_param().abi()).ok()
    }
    pub unsafe fn FaxNumber(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FaxNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFaxNumber<P0>(&self, bstrfaxnumber: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFaxNumber)(::windows_core::Interface::as_raw(self), bstrfaxnumber.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SeeAlso)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSeeAlso)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
    pub unsafe fn BusinessCategory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BusinessCategory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBusinessCategory<P0>(&self, bstrbusinesscategory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetBusinessCategory)(::windows_core::Interface::as_raw(self), bstrbusinesscategory.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsOU, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsOU {
    type Vtable = IADsOU_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsOU {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2f733b8_effe_11cf_8abc_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOU_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocalityname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpostaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtelephonenumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfaxnumber: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSeeAlso: usize,
    pub BusinessCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetBusinessCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbusinesscategory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsObjectOptions(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsObjectOptions {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetOption(&self, lnoption: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOption)(::windows_core::Interface::as_raw(self), lnoption, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetOption(&self, lnoption: i32, vvalue: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOption)(::windows_core::Interface::as_raw(self), lnoption, ::core::mem::transmute(vvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsObjectOptions, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsObjectOptions {
    type Vtable = IADsObjectOptions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsObjectOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46f14fda_232b_11d1_a808_00c04fd8d5a8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsObjectOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnoption: i32, pvvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetOption: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnoption: i32, vvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetOption: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsOctetList(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsOctetList {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OctetList(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OctetList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetOctetList(&self, voctetlist: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOctetList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(voctetlist)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsOctetList, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsOctetList {
    type Vtable = IADsOctetList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsOctetList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b28b80f_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOctetList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OctetList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OctetList: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetOctetList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voctetlist: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetOctetList: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsOpenDSObject(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsOpenDSObject {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenDSObject<P0, P1, P2>(&self, lpszdnname: P0, lpszusername: P1, lpszpassword: P2, lnreserved: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OpenDSObject)(::windows_core::Interface::as_raw(self), lpszdnname.into_param().abi(), lpszusername.into_param().abi(), lpszpassword.into_param().abi(), lnreserved, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsOpenDSObject, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsOpenDSObject {
    type Vtable = IADsOpenDSObject_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsOpenDSObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddf2891e_0f9c_11d0_8ad4_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsOpenDSObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenDSObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszdnname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnreserved: i32, ppoledsobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenDSObject: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPath(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPath {
    pub unsafe fn Type(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, lntype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), lntype).ok()
    }
    pub unsafe fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).VolumeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVolumeName<P0>(&self, bstrvolumename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetVolumeName)(::windows_core::Interface::as_raw(self), bstrvolumename.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, bstrpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPath, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPath {
    type Vtable = IADsPath_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPath {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb287fcd5_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPath_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lntype: i32) -> ::windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrvolumename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPathname(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPathname {
    pub unsafe fn Set<P0>(&self, bstradspath: P0, lnsettype: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Set)(::windows_core::Interface::as_raw(self), bstradspath.into_param().abi(), lnsettype).ok()
    }
    pub unsafe fn SetDisplayType(&self, lndisplaytype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayType)(::windows_core::Interface::as_raw(self), lndisplaytype).ok()
    }
    pub unsafe fn Retrieve(&self, lnformattype: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Retrieve)(::windows_core::Interface::as_raw(self), lnformattype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNumElements(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNumElements)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetElement(&self, lnelementindex: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetElement)(::windows_core::Interface::as_raw(self), lnelementindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddLeafElement<P0>(&self, bstrleafelement: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddLeafElement)(::windows_core::Interface::as_raw(self), bstrleafelement.into_param().abi()).ok()
    }
    pub unsafe fn RemoveLeafElement(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveLeafElement)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyPath(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CopyPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEscapedElement<P0>(&self, lnreserved: i32, bstrinstr: P0) -> ::windows_core::Result<::windows_core::BSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEscapedElement)(::windows_core::Interface::as_raw(self), lnreserved, bstrinstr.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EscapedMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EscapedMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEscapedMode(&self, lnescapedmode: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEscapedMode)(::windows_core::Interface::as_raw(self), lnescapedmode).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPathname, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPathname {
    type Vtable = IADsPathname_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPathname {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd592aed4_f420_11d0_a36e_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPathname_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradspath: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnsettype: i32) -> ::windows_core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lndisplaytype: i32) -> ::windows_core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnformattype: i32, pbstradspath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetNumElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plnnumpathelements: *mut i32) -> ::windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnelementindex: i32, pbstrelement: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AddLeafElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrleafelement: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemoveLeafElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppadspath: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyPath: usize,
    pub GetEscapedElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnreserved: i32, bstrinstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstroutstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EscapedMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetEscapedMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnescapedmode: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPostalAddress(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPostalAddress {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PostalAddress(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostalAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPostalAddress(&self, vpostaladdress: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPostalAddress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vpostaladdress)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPostalAddress, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPostalAddress {
    type Vtable = IADsPostalAddress_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPostalAddress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7adecf29_4680_11d1_a3b4_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPostalAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PostalAddress: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPostalAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpostaladdress: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPostalAddress: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPrintJob(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJob {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn HostPrintQueue(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HostPrintQueue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn User(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).User)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TimeSubmitted(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TimeSubmitted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TotalPages(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TotalPages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Size(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Size)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), lnpriority).ok()
    }
    pub unsafe fn StartTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartTime)(::windows_core::Interface::as_raw(self), dastarttime).ok()
    }
    pub unsafe fn UntilTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UntilTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUntilTime)(::windows_core::Interface::as_raw(self), dauntiltime).ok()
    }
    pub unsafe fn Notify(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNotify<P0>(&self, bstrnotify: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNotify)(::windows_core::Interface::as_raw(self), bstrnotify.into_param().abi()).ok()
    }
    pub unsafe fn NotifyPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NotifyPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNotifyPath<P0>(&self, bstrnotifypath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNotifyPath)(::windows_core::Interface::as_raw(self), bstrnotifypath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPrintJob, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPrintJob {
    type Vtable = IADsPrintJob_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPrintJob {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32fb6780_1ed0_11cf_a988_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJob_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostPrintQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TimeSubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub TotalPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows_core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnotify: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub NotifyPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetNotifyPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnotifypath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPrintJobOperations(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJobOperations {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TimeElapsed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TimeElapsed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PagesPrinted(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PagesPrinted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Position(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Position)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPosition(&self, lnposition: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPosition)(::windows_core::Interface::as_raw(self), lnposition).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPrintJobOperations, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPrintJobOperations {
    type Vtable = IADsPrintJobOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPrintJobOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a52db30_1ecf_11cf_a988_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintJobOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub TimeElapsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub PagesPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnposition: i32) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPrintQueue(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueue {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn PrinterPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrinterPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrinterPath<P0>(&self, bstrprinterpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPrinterPath)(::windows_core::Interface::as_raw(self), bstrprinterpath.into_param().abi()).ok()
    }
    pub unsafe fn Model(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Model)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetModel<P0>(&self, bstrmodel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetModel)(::windows_core::Interface::as_raw(self), bstrmodel.into_param().abi()).ok()
    }
    pub unsafe fn Datatype(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Datatype)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDatatype<P0>(&self, bstrdatatype: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDatatype)(::windows_core::Interface::as_raw(self), bstrdatatype.into_param().abi()).ok()
    }
    pub unsafe fn PrintProcessor(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrintProcessor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrintProcessor<P0>(&self, bstrprintprocessor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPrintProcessor)(::windows_core::Interface::as_raw(self), bstrprintprocessor.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn Location(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Location)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocation<P0>(&self, bstrlocation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocation)(::windows_core::Interface::as_raw(self), bstrlocation.into_param().abi()).ok()
    }
    pub unsafe fn StartTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartTime)(::windows_core::Interface::as_raw(self), dastarttime).ok()
    }
    pub unsafe fn UntilTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UntilTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUntilTime)(::windows_core::Interface::as_raw(self), dauntiltime).ok()
    }
    pub unsafe fn DefaultJobPriority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DefaultJobPriority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultJobPriority(&self, lndefaultjobpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultJobPriority)(::windows_core::Interface::as_raw(self), lndefaultjobpriority).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), lnpriority).ok()
    }
    pub unsafe fn BannerPage(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BannerPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBannerPage<P0>(&self, bstrbannerpage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetBannerPage)(::windows_core::Interface::as_raw(self), bstrbannerpage.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PrintDevices(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrintDevices)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPrintDevices(&self, vprintdevices: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPrintDevices)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vprintdevices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn NetAddresses(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NetAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetNetAddresses(&self, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNetAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vnetaddresses)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPrintQueue, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPrintQueue {
    type Vtable = IADsPrintQueue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPrintQueue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb15160d0_1226_11cf_a985_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueue_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrinterPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPrinterPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprinterpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmodel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Datatype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDatatype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdatatype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PrintProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPrintProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprintprocessor: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dastarttime: f64) -> ::windows_core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dauntiltime: f64) -> ::windows_core::HRESULT,
    pub DefaultJobPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetDefaultJobPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lndefaultjobpriority: i32) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpriority: i32) -> ::windows_core::HRESULT,
    pub BannerPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetBannerPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbannerpage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PrintDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PrintDevices: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPrintDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vprintdevices: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPrintDevices: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub NetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    NetAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetNetAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vnetaddresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetNetAddresses: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPrintQueueOperations(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueueOperations {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrintJobs(&self) -> ::windows_core::Result<IADsCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrintJobs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Purge(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Purge)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPrintQueueOperations, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPrintQueueOperations {
    type Vtable = IADsPrintQueueOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPrintQueueOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x124be5c0_156e_11cf_a986_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPrintQueueOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrintJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrintJobs: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsProperty(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsProperty {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn OID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOID<P0>(&self, bstroid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOID)(::windows_core::Interface::as_raw(self), bstroid.into_param().abi()).ok()
    }
    pub unsafe fn Syntax(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Syntax)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSyntax<P0>(&self, bstrsyntax: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSyntax)(::windows_core::Interface::as_raw(self), bstrsyntax.into_param().abi()).ok()
    }
    pub unsafe fn MaxRange(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxRange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxRange(&self, lnmaxrange: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxRange)(::windows_core::Interface::as_raw(self), lnmaxrange).ok()
    }
    pub unsafe fn MinRange(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MinRange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinRange(&self, lnminrange: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMinRange)(::windows_core::Interface::as_raw(self), lnminrange).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MultiValued(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MultiValued)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMultiValued<P0>(&self, fmultivalued: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetMultiValued)(::windows_core::Interface::as_raw(self), fmultivalued.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers(&self) -> ::windows_core::Result<IADsCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Qualifiers)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsProperty, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsProperty {
    type Vtable = IADsProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsProperty {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8f93dd3_4ae0_11cf_9e73_00aa004a5691);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsProperty_Vtbl {
    pub base__: IADs_Vtbl,
    pub OID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Syntax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetSyntax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsyntax: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MaxRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxrange: i32) -> ::windows_core::HRESULT,
    pub MinRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnminrange: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MultiValued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MultiValued: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMultiValued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmultivalued: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMultiValued: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualifiers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPropertyEntry(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyEntry {
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn ADsType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ADsType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetADsType)(::windows_core::Interface::as_raw(self), lnadstype).ok()
    }
    pub unsafe fn ControlCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ControlCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetControlCode(&self, lncontrolcode: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetControlCode)(::windows_core::Interface::as_raw(self), lncontrolcode).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Values(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Values)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValues(&self, vvalues: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValues)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vvalues)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPropertyEntry, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPropertyEntry {
    type Vtable = IADsPropertyEntry_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPropertyEntry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05792c8e_941f_11d0_8529_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyEntry_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows_core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetControlCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncontrolcode: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Values: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vvalues: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValues: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPropertyList(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyList {
    pub unsafe fn PropertyCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PropertyCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), pvariant)
    }
    pub unsafe fn Skip(&self, celements: i32) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celements)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Item(&self, varindex: super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varindex), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPropertyItem<P0>(&self, bstrname: P0, lnadstype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyItem)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), lnadstype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutPropertyItem(&self, vardata: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vardata)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ResetPropertyItem(&self, varentry: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResetPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varentry)).ok()
    }
    pub unsafe fn PurgePropertyList(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PurgePropertyList)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPropertyList, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPropertyList {
    type Vtable = IADsPropertyList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPropertyList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6f602b6_8f69_11d0_8528_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celements: i32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varindex: super::super::System::Variant::VARIANT, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnadstype: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardata: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutPropertyItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ResetPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varentry: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ResetPropertyItem: usize,
    pub PurgePropertyList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPropertyValue(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue {
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ADsType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ADsType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetADsType)(::windows_core::Interface::as_raw(self), lnadstype).ok()
    }
    pub unsafe fn DNString(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DNString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDNString<P0>(&self, bstrdnstring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDNString)(::windows_core::Interface::as_raw(self), bstrdnstring.into_param().abi()).ok()
    }
    pub unsafe fn CaseExactString(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CaseExactString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCaseExactString<P0>(&self, bstrcaseexactstring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCaseExactString)(::windows_core::Interface::as_raw(self), bstrcaseexactstring.into_param().abi()).ok()
    }
    pub unsafe fn CaseIgnoreString(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CaseIgnoreString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCaseIgnoreString<P0>(&self, bstrcaseignorestring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCaseIgnoreString)(::windows_core::Interface::as_raw(self), bstrcaseignorestring.into_param().abi()).ok()
    }
    pub unsafe fn PrintableString(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PrintableString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrintableString<P0>(&self, bstrprintablestring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPrintableString)(::windows_core::Interface::as_raw(self), bstrprintablestring.into_param().abi()).ok()
    }
    pub unsafe fn NumericString(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NumericString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNumericString<P0>(&self, bstrnumericstring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNumericString)(::windows_core::Interface::as_raw(self), bstrnumericstring.into_param().abi()).ok()
    }
    pub unsafe fn Boolean(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Boolean)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBoolean(&self, lnboolean: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBoolean)(::windows_core::Interface::as_raw(self), lnboolean).ok()
    }
    pub unsafe fn Integer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Integer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInteger(&self, lninteger: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInteger)(::windows_core::Interface::as_raw(self), lninteger).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OctetString(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OctetString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetOctetString(&self, voctetstring: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOctetString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(voctetstring)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityDescriptor(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SecurityDescriptor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, psecuritydescriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), psecuritydescriptor.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LargeInteger(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LargeInteger)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetLargeInteger<P0>(&self, plargeinteger: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).SetLargeInteger)(::windows_core::Interface::as_raw(self), plargeinteger.into_param().abi()).ok()
    }
    pub unsafe fn UTCTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UTCTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUTCTime(&self, dautctime: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUTCTime)(::windows_core::Interface::as_raw(self), dautctime).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPropertyValue, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPropertyValue {
    type Vtable = IADsPropertyValue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPropertyValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79fa9ad0_a97c_11d0_8534_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: i32) -> ::windows_core::HRESULT,
    pub DNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CaseExactString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCaseExactString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcaseexactstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CaseIgnoreString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetCaseIgnoreString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcaseignorestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PrintableString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPrintableString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprintablestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub NumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetNumericString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnumericstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Boolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnboolean: i32) -> ::windows_core::HRESULT,
    pub Integer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lninteger: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OctetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OctetString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetOctetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voctetstring: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetOctetString: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecuritydescriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LargeInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LargeInteger: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetLargeInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plargeinteger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetLargeInteger: usize,
    pub UTCTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub SetUTCTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dautctime: f64) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsPropertyValue2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue2 {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetObjectProperty(&self, lnadstype: *mut i32, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectProperty)(::windows_core::Interface::as_raw(self), lnadstype, pvprop).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutObjectProperty(&self, lnadstype: i32, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PutObjectProperty)(::windows_core::Interface::as_raw(self), lnadstype, ::core::mem::transmute(vprop)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsPropertyValue2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsPropertyValue2 {
    type Vtable = IADsPropertyValue2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsPropertyValue2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x306e831c_5bc7_11d1_a3b8_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsPropertyValue2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: *mut i32, pvprop: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetObjectProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutObjectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnadstype: i32, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutObjectProperty: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsReplicaPointer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsReplicaPointer {
    pub unsafe fn ServerName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServerName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServerName<P0>(&self, bstrservername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServerName)(::windows_core::Interface::as_raw(self), bstrservername.into_param().abi()).ok()
    }
    pub unsafe fn ReplicaType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReplicaType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReplicaType(&self, lnreplicatype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReplicaType)(::windows_core::Interface::as_raw(self), lnreplicatype).ok()
    }
    pub unsafe fn ReplicaNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReplicaNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReplicaNumber(&self, lnreplicanumber: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReplicaNumber)(::windows_core::Interface::as_raw(self), lnreplicanumber).ok()
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCount(&self, lncount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCount)(::windows_core::Interface::as_raw(self), lncount).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReplicaAddressHints(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReplicaAddressHints)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetReplicaAddressHints(&self, vreplicaaddresshints: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReplicaAddressHints)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vreplicaaddresshints)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsReplicaPointer, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsReplicaPointer {
    type Vtable = IADsReplicaPointer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsReplicaPointer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf60fb803_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsReplicaPointer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ServerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetServerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ReplicaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetReplicaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnreplicatype: i32) -> ::windows_core::HRESULT,
    pub ReplicaNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetReplicaNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnreplicanumber: i32) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncount: i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReplicaAddressHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReplicaAddressHints: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetReplicaAddressHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vreplicaaddresshints: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetReplicaAddressHints: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsResource(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsResource {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn User(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).User)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LockCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LockCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsResource, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsResource {
    type Vtable = IADsResource_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x34a05b20_4aab_11cf_ae2c_00aa006ebfb9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsResource_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsSecurityDescriptor(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityDescriptor {
    pub unsafe fn Revision(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Revision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRevision(&self, lnrevision: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRevision)(::windows_core::Interface::as_raw(self), lnrevision).ok()
    }
    pub unsafe fn Control(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Control)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetControl(&self, lncontrol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetControl)(::windows_core::Interface::as_raw(self), lncontrol).ok()
    }
    pub unsafe fn Owner(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Owner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOwner<P0>(&self, bstrowner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOwner)(::windows_core::Interface::as_raw(self), bstrowner.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OwnerDefaulted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOwnerDefaulted<P0>(&self, fownerdefaulted: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetOwnerDefaulted)(::windows_core::Interface::as_raw(self), fownerdefaulted.into_param().abi()).ok()
    }
    pub unsafe fn Group(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Group)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGroup<P0>(&self, bstrgroup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetGroup)(::windows_core::Interface::as_raw(self), bstrgroup.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GroupDefaulted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGroupDefaulted<P0>(&self, fgroupdefaulted: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetGroupDefaulted)(::windows_core::Interface::as_raw(self), fgroupdefaulted.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DiscretionaryAcl(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DiscretionaryAcl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDiscretionaryAcl<P0>(&self, pdiscretionaryacl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).SetDiscretionaryAcl)(::windows_core::Interface::as_raw(self), pdiscretionaryacl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DaclDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DaclDefaulted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDaclDefaulted<P0>(&self, fdacldefaulted: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetDaclDefaulted)(::windows_core::Interface::as_raw(self), fdacldefaulted.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SystemAcl(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SystemAcl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSystemAcl<P0>(&self, psystemacl: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).SetSystemAcl)(::windows_core::Interface::as_raw(self), psystemacl.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaclDefaulted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SaclDefaulted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSaclDefaulted<P0>(&self, fsacldefaulted: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSaclDefaulted)(::windows_core::Interface::as_raw(self), fsacldefaulted.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopySecurityDescriptor(&self) -> ::windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CopySecurityDescriptor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsSecurityDescriptor, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsSecurityDescriptor {
    type Vtable = IADsSecurityDescriptor_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsSecurityDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8c787ca_9bdd_11d0_852c_00c04fd8d503);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityDescriptor_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnrevision: i32) -> ::windows_core::HRESULT,
    pub Control: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lncontrol: i32) -> ::windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fownerdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerDefaulted: usize,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGroupDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgroupdefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGroupDefaulted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DiscretionaryAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DiscretionaryAcl: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDiscretionaryAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdiscretionaryacl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDiscretionaryAcl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DaclDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDaclDefaulted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SystemAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SystemAcl: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSystemAcl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psystemacl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSystemAcl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaclDefaulted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSaclDefaulted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsacldefaulted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSaclDefaulted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopySecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopySecurityDescriptor: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsSecurityUtility(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityUtility {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetSecurityDescriptor(&self, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, lformat: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varpath), lpathformat, lformat, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSecurityDescriptor(&self, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, vardata: super::super::System::Variant::VARIANT, ldataformat: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varpath), lpathformat, ::core::mem::transmute(vardata), ldataformat).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ConvertSecurityDescriptor(&self, varsd: super::super::System::Variant::VARIANT, ldataformat: i32, loutformat: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConvertSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varsd), ldataformat, loutformat, &mut result__).from_abi(result__)
    }
    pub unsafe fn SecurityMask(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SecurityMask)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSecurityMask(&self, lnsecuritymask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityMask)(::windows_core::Interface::as_raw(self), lnsecuritymask).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsSecurityUtility, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsSecurityUtility {
    type Vtable = IADsSecurityUtility_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsSecurityUtility {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa63251b2_5f21_474b_ab52_4a8efad10895);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSecurityUtility_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, lformat: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetSecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varpath: super::super::System::Variant::VARIANT, lpathformat: i32, vardata: super::super::System::Variant::VARIANT, ldataformat: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ConvertSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsd: super::super::System::Variant::VARIANT, ldataformat: i32, loutformat: i32, presult: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ConvertSecurityDescriptor: usize,
    pub SecurityMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetSecurityMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnsecuritymask: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsService(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsService {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn HostComputer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HostComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHostComputer<P0>(&self, bstrhostcomputer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetHostComputer)(::windows_core::Interface::as_raw(self), bstrhostcomputer.into_param().abi()).ok()
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, bstrdisplayname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), bstrdisplayname.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVersion<P0>(&self, bstrversion: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetVersion)(::windows_core::Interface::as_raw(self), bstrversion.into_param().abi()).ok()
    }
    pub unsafe fn ServiceType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServiceType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceType(&self, lnservicetype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceType)(::windows_core::Interface::as_raw(self), lnservicetype).ok()
    }
    pub unsafe fn StartType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartType(&self, lnstarttype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartType)(::windows_core::Interface::as_raw(self), lnstarttype).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, bstrpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPath)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi()).ok()
    }
    pub unsafe fn StartupParameters(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartupParameters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartupParameters<P0>(&self, bstrstartupparameters: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetStartupParameters)(::windows_core::Interface::as_raw(self), bstrstartupparameters.into_param().abi()).ok()
    }
    pub unsafe fn ErrorControl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ErrorControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetErrorControl(&self, lnerrorcontrol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetErrorControl)(::windows_core::Interface::as_raw(self), lnerrorcontrol).ok()
    }
    pub unsafe fn LoadOrderGroup(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoadOrderGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLoadOrderGroup<P0>(&self, bstrloadordergroup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLoadOrderGroup)(::windows_core::Interface::as_raw(self), bstrloadordergroup.into_param().abi()).ok()
    }
    pub unsafe fn ServiceAccountName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServiceAccountName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountName<P0>(&self, bstrserviceaccountname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServiceAccountName)(::windows_core::Interface::as_raw(self), bstrserviceaccountname.into_param().abi()).ok()
    }
    pub unsafe fn ServiceAccountPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServiceAccountPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceAccountPath<P0>(&self, bstrserviceaccountpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServiceAccountPath)(::windows_core::Interface::as_raw(self), bstrserviceaccountpath.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Dependencies(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Dependencies)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDependencies(&self, vdependencies: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDependencies)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vdependencies)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsService, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsService {
    type Vtable = IADsService_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68af66e0_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsService_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhostcomputer: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdisplayname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ServiceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetServiceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnservicetype: i32) -> ::windows_core::HRESULT,
    pub StartType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetStartType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnstarttype: i32) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StartupParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetStartupParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrstartupparameters: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ErrorControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetErrorControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnerrorcontrol: i32) -> ::windows_core::HRESULT,
    pub LoadOrderGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLoadOrderGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrloadordergroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ServiceAccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetServiceAccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceaccountname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ServiceAccountPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetServiceAccountPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrserviceaccountpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Dependencies: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vdependencies: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDependencies: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsServiceOperations(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsServiceOperations {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Start(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Start)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Stop)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Continue(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Continue)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, bstrnewpassword: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPassword)(::windows_core::Interface::as_raw(self), bstrnewpassword.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsServiceOperations, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsServiceOperations {
    type Vtable = IADsServiceOperations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsServiceOperations {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d7b33f0_31ca_11cf_a98a_00aa006bc149);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsServiceOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Continue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnewpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsSession(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSession {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn User(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).User)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Computer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Computer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComputerPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConnectTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ConnectTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IdleTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IdleTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsSession, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsSession {
    type Vtable = IADsSession_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x398b7da0_4aab_11cf_ae2c_00aa006ebfb9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSession_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Computer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ComputerPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ConnectTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub IdleTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsSyntax(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsSyntax {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn OleAutoDataType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OleAutoDataType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOleAutoDataType(&self, lnoleautodatatype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOleAutoDataType)(::windows_core::Interface::as_raw(self), lnoleautodatatype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsSyntax, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsSyntax {
    type Vtable = IADsSyntax_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsSyntax {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8f93dd2_4ae0_11cf_9e73_00aa004a5691);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsSyntax_Vtbl {
    pub base__: IADs_Vtbl,
    pub OleAutoDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetOleAutoDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnoleautodatatype: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsTimestamp(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsTimestamp {
    pub unsafe fn WholeSeconds(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).WholeSeconds)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWholeSeconds(&self, lnwholeseconds: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWholeSeconds)(::windows_core::Interface::as_raw(self), lnwholeseconds).ok()
    }
    pub unsafe fn EventID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EventID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventID(&self, lneventid: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEventID)(::windows_core::Interface::as_raw(self), lneventid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsTimestamp, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsTimestamp {
    type Vtable = IADsTimestamp_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsTimestamp {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2f5a901_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTimestamp_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WholeSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetWholeSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnwholeseconds: i32) -> ::windows_core::HRESULT,
    pub EventID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetEventID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lneventid: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsTypedName(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsTypedName {
    pub unsafe fn ObjectName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ObjectName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetObjectName<P0>(&self, bstrobjectname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetObjectName)(::windows_core::Interface::as_raw(self), bstrobjectname.into_param().abi()).ok()
    }
    pub unsafe fn Level(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Level)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLevel(&self, lnlevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLevel)(::windows_core::Interface::as_raw(self), lnlevel).ok()
    }
    pub unsafe fn Interval(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Interval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterval(&self, lninterval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterval)(::windows_core::Interface::as_raw(self), lninterval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsTypedName, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsTypedName {
    type Vtable = IADsTypedName_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsTypedName {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb371a349_4080_11d1_a3ac_00c04fb950dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTypedName_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnlevel: i32) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lninterval: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsUser(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsUser {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Class(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Class)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GUID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GUID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ADsPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ADsPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Parent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Schema(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Schema)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInfo)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Get)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Put<P0>(&self, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Put)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> ::windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEx)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutEx<P0>(&self, lncontrolcode: i32, bstrname: P0, vprop: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PutEx)(::windows_core::Interface::as_raw(self), lncontrolcode, bstrname.into_param().abi(), ::core::mem::transmute(vprop)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetInfoEx(&self, vproperties: super::super::System::Variant::VARIANT, lnreserved: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vproperties), lnreserved).ok()
    }
    pub unsafe fn BadLoginAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BadLoginAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BadLoginCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BadLoginCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastLogin(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastLogin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastLogoff(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastLogoff)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastFailedLogin(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastFailedLogin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PasswordLastChanged(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PasswordLastChanged)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn Division(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Division)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDivision<P0>(&self, bstrdivision: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDivision)(::windows_core::Interface::as_raw(self), bstrdivision.into_param().abi()).ok()
    }
    pub unsafe fn Department(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Department)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDepartment<P0>(&self, bstrdepartment: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDepartment)(::windows_core::Interface::as_raw(self), bstrdepartment.into_param().abi()).ok()
    }
    pub unsafe fn EmployeeID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EmployeeID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEmployeeID<P0>(&self, bstremployeeid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEmployeeID)(::windows_core::Interface::as_raw(self), bstremployeeid.into_param().abi()).ok()
    }
    pub unsafe fn FullName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FullName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFullName<P0>(&self, bstrfullname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFullName)(::windows_core::Interface::as_raw(self), bstrfullname.into_param().abi()).ok()
    }
    pub unsafe fn FirstName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FirstName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFirstName<P0>(&self, bstrfirstname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFirstName)(::windows_core::Interface::as_raw(self), bstrfirstname.into_param().abi()).ok()
    }
    pub unsafe fn LastName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LastName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastName<P0>(&self, bstrlastname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLastName)(::windows_core::Interface::as_raw(self), bstrlastname.into_param().abi()).ok()
    }
    pub unsafe fn OtherName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OtherName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOtherName<P0>(&self, bstrothername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOtherName)(::windows_core::Interface::as_raw(self), bstrothername.into_param().abi()).ok()
    }
    pub unsafe fn NamePrefix(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NamePrefix)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNamePrefix<P0>(&self, bstrnameprefix: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNamePrefix)(::windows_core::Interface::as_raw(self), bstrnameprefix.into_param().abi()).ok()
    }
    pub unsafe fn NameSuffix(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NameSuffix)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNameSuffix<P0>(&self, bstrnamesuffix: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetNameSuffix)(::windows_core::Interface::as_raw(self), bstrnamesuffix.into_param().abi()).ok()
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, bstrtitle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTitle)(::windows_core::Interface::as_raw(self), bstrtitle.into_param().abi()).ok()
    }
    pub unsafe fn Manager(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Manager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetManager<P0>(&self, bstrmanager: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetManager)(::windows_core::Interface::as_raw(self), bstrmanager.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TelephoneHome(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TelephoneHome)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTelephoneHome(&self, vtelephonehome: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTelephoneHome)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vtelephonehome)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TelephoneMobile(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TelephoneMobile)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTelephoneMobile(&self, vtelephonemobile: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTelephoneMobile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vtelephonemobile)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TelephoneNumber(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TelephoneNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTelephoneNumber(&self, vtelephonenumber: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTelephoneNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vtelephonenumber)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn TelephonePager(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TelephonePager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTelephonePager(&self, vtelephonepager: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTelephonePager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vtelephonepager)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn FaxNumber(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FaxNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetFaxNumber(&self, vfaxnumber: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFaxNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vfaxnumber)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OfficeLocations(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OfficeLocations)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetOfficeLocations(&self, vofficelocations: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOfficeLocations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vofficelocations)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PostalAddresses(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostalAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPostalAddresses(&self, vpostaladdresses: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPostalAddresses)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vpostaladdresses)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PostalCodes(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostalCodes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPostalCodes(&self, vpostalcodes: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPostalCodes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vpostalcodes)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SeeAlso(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SeeAlso)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSeeAlso(&self, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSeeAlso)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vseealso)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AccountDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AccountDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAccountDisabled<P0>(&self, faccountdisabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAccountDisabled)(::windows_core::Interface::as_raw(self), faccountdisabled.into_param().abi()).ok()
    }
    pub unsafe fn AccountExpirationDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AccountExpirationDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccountExpirationDate(&self, daaccountexpirationdate: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAccountExpirationDate)(::windows_core::Interface::as_raw(self), daaccountexpirationdate).ok()
    }
    pub unsafe fn GraceLoginsAllowed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GraceLoginsAllowed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGraceLoginsAllowed)(::windows_core::Interface::as_raw(self), lngraceloginsallowed).ok()
    }
    pub unsafe fn GraceLoginsRemaining(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GraceLoginsRemaining)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGraceLoginsRemaining)(::windows_core::Interface::as_raw(self), lngraceloginsremaining).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAccountLocked(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsAccountLocked)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsAccountLocked<P0>(&self, fisaccountlocked: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsAccountLocked)(::windows_core::Interface::as_raw(self), fisaccountlocked.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LoginHours(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoginHours)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetLoginHours(&self, vloginhours: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLoginHours)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vloginhours)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LoginWorkstations(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoginWorkstations)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetLoginWorkstations(&self, vloginworkstations: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLoginWorkstations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vloginworkstations)).ok()
    }
    pub unsafe fn MaxLogins(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxLogins)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxLogins(&self, lnmaxlogins: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxLogins)(::windows_core::Interface::as_raw(self), lnmaxlogins).ok()
    }
    pub unsafe fn MaxStorage(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMaxStorage(&self, lnmaxstorage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxStorage)(::windows_core::Interface::as_raw(self), lnmaxstorage).ok()
    }
    pub unsafe fn PasswordExpirationDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PasswordExpirationDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPasswordExpirationDate(&self, dapasswordexpirationdate: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPasswordExpirationDate)(::windows_core::Interface::as_raw(self), dapasswordexpirationdate).ok()
    }
    pub unsafe fn PasswordMinimumLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PasswordMinimumLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPasswordMinimumLength)(::windows_core::Interface::as_raw(self), lnpasswordminimumlength).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PasswordRequired(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PasswordRequired)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPasswordRequired<P0>(&self, fpasswordrequired: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetPasswordRequired)(::windows_core::Interface::as_raw(self), fpasswordrequired.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequireUniquePassword(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RequireUniquePassword)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRequireUniquePassword<P0>(&self, frequireuniquepassword: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetRequireUniquePassword)(::windows_core::Interface::as_raw(self), frequireuniquepassword.into_param().abi()).ok()
    }
    pub unsafe fn EmailAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EmailAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEmailAddress<P0>(&self, bstremailaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetEmailAddress)(::windows_core::Interface::as_raw(self), bstremailaddress.into_param().abi()).ok()
    }
    pub unsafe fn HomeDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HomeDirectory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHomeDirectory<P0>(&self, bstrhomedirectory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetHomeDirectory)(::windows_core::Interface::as_raw(self), bstrhomedirectory.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Languages)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetLanguages(&self, vlanguages: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLanguages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vlanguages)).ok()
    }
    pub unsafe fn Profile(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Profile)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProfile<P0>(&self, bstrprofile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProfile)(::windows_core::Interface::as_raw(self), bstrprofile.into_param().abi()).ok()
    }
    pub unsafe fn LoginScript(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoginScript)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLoginScript<P0>(&self, bstrloginscript: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLoginScript)(::windows_core::Interface::as_raw(self), bstrloginscript.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Picture(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Picture)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPicture(&self, vpicture: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPicture)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vpicture)).ok()
    }
    pub unsafe fn HomePage(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HomePage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHomePage<P0>(&self, bstrhomepage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetHomePage)(::windows_core::Interface::as_raw(self), bstrhomepage.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> ::windows_core::Result<IADsMembers> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Groups)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPassword<P0>(&self, newpassword: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPassword)(::windows_core::Interface::as_raw(self), newpassword.into_param().abi()).ok()
    }
    pub unsafe fn ChangePassword<P0, P1>(&self, bstroldpassword: P0, bstrnewpassword: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ChangePassword)(::windows_core::Interface::as_raw(self), bstroldpassword.into_param().abi(), bstrnewpassword.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsUser, ::windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsUser {
    type Vtable = IADsUser_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e37e320_17e2_11cf_abc4_02608c9e7553);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsUser_Vtbl {
    pub base__: IADs_Vtbl,
    pub BadLoginAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BadLoginCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub LastLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub LastLogoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub LastFailedLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub PasswordLastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Division: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdivision: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdepartment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EmployeeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEmployeeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstremployeeid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfullname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfirstname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlastname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OtherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetOtherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrothername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub NamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnameprefix: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub NameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetNameSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnamesuffix: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Manager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmanager: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TelephoneHome: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TelephoneHome: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTelephoneHome: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonehome: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTelephoneHome: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TelephoneMobile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TelephoneMobile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTelephoneMobile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonemobile: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTelephoneMobile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TelephoneNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTelephoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonenumber: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTelephoneNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub TelephonePager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    TelephonePager: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTelephonePager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vtelephonepager: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTelephonePager: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub FaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    FaxNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetFaxNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vfaxnumber: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetFaxNumber: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OfficeLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OfficeLocations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetOfficeLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vofficelocations: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetOfficeLocations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PostalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PostalAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPostalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpostaladdresses: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPostalAddresses: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PostalCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PostalCodes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPostalCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpostalcodes: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPostalCodes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SeeAlso: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSeeAlso: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vseealso: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSeeAlso: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AccountDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AccountDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAccountDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, faccountdisabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAccountDisabled: usize,
    pub AccountExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub SetAccountExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, daaccountexpirationdate: f64) -> ::windows_core::HRESULT,
    pub GraceLoginsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetGraceLoginsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lngraceloginsallowed: i32) -> ::windows_core::HRESULT,
    pub GraceLoginsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetGraceLoginsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lngraceloginsremaining: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAccountLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAccountLocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsAccountLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fisaccountlocked: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsAccountLocked: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LoginHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LoginHours: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetLoginHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vloginhours: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetLoginHours: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LoginWorkstations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LoginWorkstations: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetLoginWorkstations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vloginworkstations: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetLoginWorkstations: usize,
    pub MaxLogins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxLogins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxlogins: i32) -> ::windows_core::HRESULT,
    pub MaxStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnmaxstorage: i32) -> ::windows_core::HRESULT,
    pub PasswordExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub SetPasswordExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dapasswordexpirationdate: f64) -> ::windows_core::HRESULT,
    pub PasswordMinimumLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPasswordMinimumLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnpasswordminimumlength: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PasswordRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PasswordRequired: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPasswordRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fpasswordrequired: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPasswordRequired: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequireUniquePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequireUniquePassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRequireUniquePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequireuniquepassword: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRequireUniquePassword: usize,
    pub EmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstremailaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub HomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetHomeDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhomedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Languages: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vlanguages: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetLanguages: usize,
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprofile: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LoginScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLoginScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrloginscript: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Picture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Picture: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vpicture: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPicture: usize,
    pub HomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetHomePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrhomepage: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroldpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IADsWinNTSystemInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IADsWinNTSystemInfo {
    pub unsafe fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComputerName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DomainName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PDC(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IADsWinNTSystemInfo, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IADsWinNTSystemInfo {
    type Vtable = IADsWinNTSystemInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IADsWinNTSystemInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c6d65dc_afd1_11d2_9cb9_0000f87a369e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsWinNTSystemInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICommonQuery(::windows_core::IUnknown);
impl ICommonQuery {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OpenQueryWindow<P0>(&self, hwndparent: P0, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut ::core::option::Option<super::super::System::Com::IDataObject>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).OpenQueryWindow)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), pquerywnd, ::core::mem::transmute(ppdataobject)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ICommonQuery, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommonQuery {
    type Vtable = ICommonQuery_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommonQuery {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab50dec0_6f1d_11d0_a1c4_00aa00c16e65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonQuery_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub OpenQueryWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    OpenQueryWindow: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectoryObject(::windows_core::IUnknown);
impl IDirectoryObject {
    pub unsafe fn GetObjectInformation(&self) -> ::windows_core::Result<*mut ADS_OBJECT_INFO> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetObjectInformation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectAttributes(&self, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectAttributes)(::windows_core::Interface::as_raw(self), pattributenames, dwnumberattributes, ppattributeentries, pdwnumattributesreturned).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetObjectAttributes(&self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetObjectAttributes)(::windows_core::Interface::as_raw(self), pattributeentries, dwnumattributes, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDSObject<P0>(&self, pszrdnname: P0, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> ::windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDSObject)(::windows_core::Interface::as_raw(self), pszrdnname.into_param().abi(), pattributeentries, dwnumattributes, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteDSObject<P0>(&self, pszrdnname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteDSObject)(::windows_core::Interface::as_raw(self), pszrdnname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectoryObject, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectoryObject {
    type Vtable = IDirectoryObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectoryObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe798de2c_22e4_11d0_84fe_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectoryObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetObjectInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobjinfo: *mut *mut ADS_OBJECT_INFO) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetObjectAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, pdwnumattributesmodified: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetObjectAttributes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDSObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrdnname: ::windows_core::PCWSTR, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDSObject: usize,
    pub DeleteDSObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrdnname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectorySchemaMgmt(::windows_core::IUnknown);
impl IDirectorySchemaMgmt {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAttributes(&self, ppszattrnames: *const ::windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumAttributes)(::windows_core::Interface::as_raw(self), ppszattrnames, dwnumattributes, ppattrdefinition, pdwnumattributes).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateAttributeDefinition)(::windows_core::Interface::as_raw(self), pszattributename.into_param().abi(), pattributedefinition).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteAttributeDefinition)(::windows_core::Interface::as_raw(self), pszattributename.into_param().abi(), pattributedefinition).ok()
    }
    pub unsafe fn DeleteAttributeDefinition<P0>(&self, pszattributename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteAttributeDefinition)(::windows_core::Interface::as_raw(self), pszattributename.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumClasses(&self, ppszclassnames: *const ::windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnumClasses)(::windows_core::Interface::as_raw(self), ppszclassnames, dwnumclasses, ppclassdefinition, pdwnumclasses).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteClassDefinition)(::windows_core::Interface::as_raw(self), pszclassname.into_param().abi(), pclassdefinition).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateClassDefinition)(::windows_core::Interface::as_raw(self), pszclassname.into_param().abi(), pclassdefinition).ok()
    }
    pub unsafe fn DeleteClassDefinition<P0>(&self, pszclassname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteClassDefinition)(::windows_core::Interface::as_raw(self), pszclassname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectorySchemaMgmt, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectorySchemaMgmt {
    type Vtable = IDirectorySchemaMgmt_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectorySchemaMgmt {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75db3b9c_a4d8_11d0_a79c_00c04fd8d5a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectorySchemaMgmt_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszattrnames: *const ::windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateAttributeDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateAttributeDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributeDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR, pattributedefinition: *const ADS_ATTR_DEF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributeDefinition: usize,
    pub DeleteAttributeDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszclassnames: *const ::windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumClasses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteClassDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteClassDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateClassDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR, pclassdefinition: *const ADS_CLASS_DEF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateClassDefinition: usize,
    pub DeleteClassDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszclassname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDirectorySearch(::windows_core::IUnknown);
impl IDirectorySearch {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSearchPreference(&self, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSearchPreference)(::windows_core::Interface::as_raw(self), psearchprefs, dwnumprefs).ok()
    }
    pub unsafe fn ExecuteSearch<P0>(&self, pszsearchfilter: P0, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32) -> ::windows_core::Result<ADS_SEARCH_HANDLE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExecuteSearch)(::windows_core::Interface::as_raw(self), pszsearchfilter.into_param().abi(), pattributenames, dwnumberattributes, &mut result__).from_abi(result__)
    }
    pub unsafe fn AbandonSearch<P0>(&self, phsearchresult: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ADS_SEARCH_HANDLE>,
    {
        (::windows_core::Interface::vtable(self).AbandonSearch)(::windows_core::Interface::as_raw(self), phsearchresult.into_param().abi()).ok()
    }
    pub unsafe fn GetFirstRow<P0>(&self, hsearchresult: P0) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<ADS_SEARCH_HANDLE>,
    {
        (::windows_core::Interface::vtable(self).GetFirstRow)(::windows_core::Interface::as_raw(self), hsearchresult.into_param().abi())
    }
    pub unsafe fn GetNextRow<P0>(&self, hsearchresult: P0) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<ADS_SEARCH_HANDLE>,
    {
        (::windows_core::Interface::vtable(self).GetNextRow)(::windows_core::Interface::as_raw(self), hsearchresult.into_param().abi())
    }
    pub unsafe fn GetPreviousRow<P0>(&self, hsearchresult: P0) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<ADS_SEARCH_HANDLE>,
    {
        (::windows_core::Interface::vtable(self).GetPreviousRow)(::windows_core::Interface::as_raw(self), hsearchresult.into_param().abi())
    }
    pub unsafe fn GetNextColumnName<P0>(&self, hsearchhandle: P0, ppszcolumnname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT
    where
        P0: ::windows_core::IntoParam<ADS_SEARCH_HANDLE>,
    {
        (::windows_core::Interface::vtable(self).GetNextColumnName)(::windows_core::Interface::as_raw(self), hsearchhandle.into_param().abi(), ppszcolumnname)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetColumn<P0, P1>(&self, hsearchresult: P0, szcolumnname: P1, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ADS_SEARCH_HANDLE>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetColumn)(::windows_core::Interface::as_raw(self), hsearchresult.into_param().abi(), szcolumnname.into_param().abi(), psearchcolumn).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FreeColumn(&self, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FreeColumn)(::windows_core::Interface::as_raw(self), psearchcolumn).ok()
    }
    pub unsafe fn CloseSearchHandle<P0>(&self, hsearchresult: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ADS_SEARCH_HANDLE>,
    {
        (::windows_core::Interface::vtable(self).CloseSearchHandle)(::windows_core::Interface::as_raw(self), hsearchresult.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDirectorySearch, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectorySearch {
    type Vtable = IDirectorySearch_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDirectorySearch {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x109ba8ec_92f0_11d0_a790_00c04fd8d5a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectorySearch_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSearchPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSearchPreference: usize,
    pub ExecuteSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsearchfilter: ::windows_core::PCWSTR, pattributenames: *const ::windows_core::PCWSTR, dwnumberattributes: u32, phsearchresult: *mut ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT,
    pub AbandonSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT,
    pub GetFirstRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT,
    pub GetNextRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT,
    pub GetPreviousRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT,
    pub GetNextColumnName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchhandle: ADS_SEARCH_HANDLE, ppszcolumnname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE, szcolumnname: ::windows_core::PCWSTR, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetColumn: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FreeColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psearchcolumn: *const ADS_SEARCH_COLUMN) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FreeColumn: usize,
    pub CloseSearchHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsearchresult: ADS_SEARCH_HANDLE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsAdminCreateObj(::windows_core::IUnknown);
impl IDsAdminCreateObj {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, padscontainerobj: P0, padscopysource: P1, lpszclassname: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IADsContainer>,
        P1: ::windows_core::IntoParam<IADs>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), padscontainerobj.into_param().abi(), padscopysource.into_param().abi(), lpszclassname.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateModal<P0>(&self, hwndparent: P0) -> ::windows_core::Result<IADs>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateModal)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDsAdminCreateObj, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsAdminCreateObj {
    type Vtable = IDsAdminCreateObj_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsAdminCreateObj {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53554a38_f902_11d2_82b9_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminCreateObj_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppadsobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateModal: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsAdminNewObj(::windows_core::IUnknown);
impl IDsAdminNewObj {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtons<P0>(&self, ncurrindex: u32, bvalid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetButtons)(::windows_core::Interface::as_raw(self), ncurrindex, bvalid.into_param().abi()).ok()
    }
    pub unsafe fn GetPageCounts(&self, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPageCounts)(::windows_core::Interface::as_raw(self), pntotal, pnstartindex).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDsAdminNewObj, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsAdminNewObj {
    type Vtable = IDsAdminNewObj_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsAdminNewObj {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2573587_e6fc_11d2_82af_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNewObj_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncurrindex: u32, bvalid: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetButtons: usize,
    pub GetPageCounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntotal: *mut i32, pnstartindex: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsAdminNewObjExt(::windows_core::IUnknown);
impl IDsAdminNewObjExt {
    #[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, padscontainerobj: P0, padscopysource: P1, lpszclassname: P2, pdsadminnewobj: P3, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IADsContainer>,
        P1: ::windows_core::IntoParam<IADs>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<IDsAdminNewObj>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), padscontainerobj.into_param().abi(), padscopysource.into_param().abi(), lpszclassname.into_param().abi(), pdsadminnewobj.into_param().abi(), pdispinfo).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn AddPages<P0>(&self, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).AddPages)(::windows_core::Interface::as_raw(self), lpfnaddpage, lparam.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetObject<P0>(&self, padsobj: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IADs>,
    {
        (::windows_core::Interface::vtable(self).SetObject)(::windows_core::Interface::as_raw(self), padsobj.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteData<P0>(&self, hwnd: P0, ucontext: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).WriteData)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), ucontext).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnError<P0>(&self, hwnd: P0, hr: ::windows_core::HRESULT, ucontext: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).OnError)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), hr, ucontext).ok()
    }
    pub unsafe fn GetSummaryInfo(&self, pbstrtext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSummaryInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrtext)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDsAdminNewObjExt, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsAdminNewObjExt {
    type Vtable = IDsAdminNewObjExt_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsAdminNewObjExt {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6088eae2_e7bf_11d2_82af_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNewObjExt_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padscontainerobj: *mut ::core::ffi::c_void, padscopysource: *mut ::core::ffi::c_void, lpszclassname: ::windows_core::PCWSTR, pdsadminnewobj: *mut ::core::ffi::c_void, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging")))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub AddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls")))]
    AddPages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padsobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, ucontext: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, hr: ::windows_core::HRESULT, ucontext: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnError: usize,
    pub GetSummaryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsAdminNewObjPrimarySite(::windows_core::IUnknown);
impl IDsAdminNewObjPrimarySite {
    pub unsafe fn CreateNew<P0>(&self, pszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateNew)(::windows_core::Interface::as_raw(self), pszname.into_param().abi()).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDsAdminNewObjPrimarySite, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsAdminNewObjPrimarySite {
    type Vtable = IDsAdminNewObjPrimarySite_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsAdminNewObjPrimarySite {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe2b487e_f904_11d2_82b9_00c04f68928b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNewObjPrimarySite_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsAdminNotifyHandler(::windows_core::IUnknown);
impl IDsAdminNotifyHandler {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pextrainfo: P0, pueventflags: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pextrainfo.into_param().abi(), pueventflags).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Begin<P0, P1>(&self, uevent: u32, parg1: P0, parg2: P1, puflags: *mut u32, pbstr: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IDataObject>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows_core::Interface::vtable(self).Begin)(::windows_core::Interface::as_raw(self), uevent, parg1.into_param().abi(), parg2.into_param().abi(), puflags, ::core::mem::transmute(pbstr)).ok()
    }
    pub unsafe fn Notify(&self, nitem: u32, uflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), nitem, uflags).ok()
    }
    pub unsafe fn End(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDsAdminNotifyHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsAdminNotifyHandler {
    type Vtable = IDsAdminNotifyHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsAdminNotifyHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4a2b8b3_5a18_11d2_97c1_00a0c9a06d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsAdminNotifyHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextrainfo: *mut ::core::ffi::c_void, pueventflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uevent: u32, parg1: *mut ::core::ffi::c_void, parg2: *mut ::core::ffi::c_void, puflags: *mut u32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin: usize,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitem: u32, uflags: u32) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsBrowseDomainTree(::windows_core::IUnknown);
impl IDsBrowseDomainTree {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrowseTo<P0>(&self, hwndparent: P0, ppsztargetpath: *mut ::windows_core::PWSTR, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).BrowseTo)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ppsztargetpath, dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDomains)(::windows_core::Interface::as_raw(self), ppdomaintree, dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FreeDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FreeDomains)(::windows_core::Interface::as_raw(self), ppdomaintree).ok()
    }
    pub unsafe fn FlushCachedDomains(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FlushCachedDomains)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetComputer<P0, P1, P2>(&self, pszcomputername: P0, pszusername: P1, pszpassword: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetComputer)(::windows_core::Interface::as_raw(self), pszcomputername.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDsBrowseDomainTree, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsBrowseDomainTree {
    type Vtable = IDsBrowseDomainTree_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsBrowseDomainTree {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cabcf1e_78f5_11d2_960c_00c04fa31a86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsBrowseDomainTree_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BrowseTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppsztargetpath: *mut ::windows_core::PWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BrowseTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDomains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDomains: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FreeDomains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdomaintree: *mut *mut DOMAIN_TREE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FreeDomains: usize,
    pub FlushCachedDomains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcomputername: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsDisplaySpecifier(::windows_core::IUnknown);
impl IDsDisplaySpecifier {
    pub unsafe fn SetServer<P0, P1, P2>(&self, pszserver: P0, pszusername: P1, pszpassword: P2, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServer)(::windows_core::Interface::as_raw(self), pszserver.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn SetLanguageID(&self, langid: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLanguageID)(::windows_core::Interface::as_raw(self), langid).ok()
    }
    pub unsafe fn GetDisplaySpecifier<P0>(&self, pszobjectclass: P0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetDisplaySpecifier)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), riid, ppv).ok()
    }
    pub unsafe fn GetIconLocation<P0>(&self, pszobjectclass: P0, dwflags: u32, pszbuffer: &mut [u16], presid: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetIconLocation)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), dwflags, ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), presid).ok()
    }
    #[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon<P0>(&self, pszobjectclass: P0, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetIcon)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), dwflags, cxicon, cyicon)
    }
    pub unsafe fn GetFriendlyClassName<P0>(&self, pszobjectclass: P0, pszbuffer: &mut [u16]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetFriendlyClassName)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFriendlyAttributeName<P0, P1>(&self, pszobjectclass: P0, pszattributename: P1, pszbuffer: &mut [u16]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetFriendlyAttributeName)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), pszattributename.into_param().abi(), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClassContainer<P0, P1>(&self, pszobjectclass: P0, pszadspath: P1, dwflags: u32) -> super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).IsClassContainer)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), pszadspath.into_param().abi(), dwflags)
    }
    pub unsafe fn GetClassCreationInfo<P0>(&self, pszobjectclass: P0, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetClassCreationInfo)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), ppdscci).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumClassAttributes<P0, P1>(&self, pszobjectclass: P0, pcbenum: LPDSENUMATTRIBUTES, lparam: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).EnumClassAttributes)(::windows_core::Interface::as_raw(self), pszobjectclass.into_param().abi(), pcbenum, lparam.into_param().abi()).ok()
    }
    pub unsafe fn GetAttributeADsType<P0>(&self, pszattributename: P0) -> ADSTYPE
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetAttributeADsType)(::windows_core::Interface::as_raw(self), pszattributename.into_param().abi())
    }
}
::windows_core::imp::interface_hierarchy!(IDsDisplaySpecifier, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsDisplaySpecifier {
    type Vtable = IDsDisplaySpecifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsDisplaySpecifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ab4a8c0_6a0b_11d2_ad49_00c04fa31a86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsDisplaySpecifier_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszserver: ::windows_core::PCWSTR, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub SetLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows_core::HRESULT,
    pub GetDisplaySpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetIconLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, dwflags: u32, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32, presid: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    pub GetFriendlyClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::HRESULT,
    pub GetFriendlyAttributeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszattributename: ::windows_core::PCWSTR, pszbuffer: ::windows_core::PWSTR, cchbuffer: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClassContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pszadspath: ::windows_core::PCWSTR, dwflags: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClassContainer: usize,
    pub GetClassCreationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumClassAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszobjectclass: ::windows_core::PCWSTR, pcbenum: LPDSENUMATTRIBUTES, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumClassAttributes: usize,
    pub GetAttributeADsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributename: ::windows_core::PCWSTR) -> ADSTYPE,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsObjectPicker(::windows_core::IUnknown);
impl IDsObjectPicker {
    pub unsafe fn Initialize(&self, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pinitinfo).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InvokeDialog<P0>(&self, hwndparent: P0) -> ::windows_core::Result<super::super::System::Com::IDataObject>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InvokeDialog)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDsObjectPicker, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDsObjectPicker {
    type Vtable = IDsObjectPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsObjectPicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c87e64e_3b7a_11d2_b9e0_00c04fd8dbf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsObjectPicker_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InvokeDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppdoselections: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InvokeDialog: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDsObjectPickerCredentials(::windows_core::IUnknown);
impl IDsObjectPickerCredentials {
    pub unsafe fn Initialize(&self, pinitinfo: *mut DSOP_INIT_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), pinitinfo).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InvokeDialog<P0>(&self, hwndparent: P0) -> ::windows_core::Result<super::super::System::Com::IDataObject>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InvokeDialog)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCredentials<P0, P1>(&self, szusername: P0, szpassword: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCredentials)(::windows_core::Interface::as_raw(self), szusername.into_param().abi(), szpassword.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDsObjectPickerCredentials, ::windows_core::IUnknown, IDsObjectPicker);
unsafe impl ::windows_core::Interface for IDsObjectPickerCredentials {
    type Vtable = IDsObjectPickerCredentials_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDsObjectPickerCredentials {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2d3ec9b_d041_445a_8f16_4748de8fb1cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDsObjectPickerCredentials_Vtbl {
    pub base__: IDsObjectPicker_Vtbl,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szusername: ::windows_core::PCWSTR, szpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPersistQuery(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPersistQuery {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClassID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteString<P0, P1, P2>(&self, psection: P0, pvaluename: P1, pvalue: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteString)(::windows_core::Interface::as_raw(self), psection.into_param().abi(), pvaluename.into_param().abi(), pvalue.into_param().abi()).ok()
    }
    pub unsafe fn ReadString<P0, P1>(&self, psection: P0, pvaluename: P1, pbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ReadString)(::windows_core::Interface::as_raw(self), psection.into_param().abi(), pvaluename.into_param().abi(), ::core::mem::transmute(pbuffer), cchbuffer).ok()
    }
    pub unsafe fn WriteInt<P0, P1>(&self, psection: P0, pvaluename: P1, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteInt)(::windows_core::Interface::as_raw(self), psection.into_param().abi(), pvaluename.into_param().abi(), value).ok()
    }
    pub unsafe fn ReadInt<P0, P1>(&self, psection: P0, pvaluename: P1, pvalue: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ReadInt)(::windows_core::Interface::as_raw(self), psection.into_param().abi(), pvaluename.into_param().abi(), pvalue).ok()
    }
    pub unsafe fn WriteStruct<P0, P1>(&self, psection: P0, pvaluename: P1, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).WriteStruct)(::windows_core::Interface::as_raw(self), psection.into_param().abi(), pvaluename.into_param().abi(), pstruct, cbstruct).ok()
    }
    pub unsafe fn ReadStruct<P0, P1>(&self, psection: P0, pvaluename: P1, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ReadStruct)(::windows_core::Interface::as_raw(self), psection.into_param().abi(), pvaluename.into_param().abi(), pstruct, cbstruct).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IPersistQuery, ::windows_core::IUnknown, super::super::System::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IPersistQuery {
    type Vtable = IPersistQuery_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IPersistQuery {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a3114b8_a62e_11d0_a6c5_00a0c906af45);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistQuery_Vtbl {
    pub base__: super::super::System::Com::IPersist_Vtbl,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub ReadString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pbuffer: ::windows_core::PWSTR, cchbuffer: i32) -> ::windows_core::HRESULT,
    pub WriteInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, value: i32) -> ::windows_core::HRESULT,
    pub ReadInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub WriteStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::HRESULT,
    pub ReadStruct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psection: ::windows_core::PCWSTR, pvaluename: ::windows_core::PCWSTR, pstruct: *mut ::core::ffi::c_void, cbstruct: u32) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrivateDispatch(::windows_core::IUnknown);
impl IPrivateDispatch {
    pub unsafe fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ADSIInitializeDispatchManager)(::windows_core::Interface::as_raw(self), dwextensionid).ok()
    }
    pub unsafe fn ADSIGetTypeInfoCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ADSIGetTypeInfoCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ADSIGetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows_core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ADSIGetTypeInfo)(::windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ADSIGetIDsOfNames(&self, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ADSIGetIDsOfNames)(::windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ADSIInvoke(&self, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ADSIInvoke)(::windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, pvarresult, pexcepinfo, puargerr).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPrivateDispatch, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrivateDispatch {
    type Vtable = IPrivateDispatch_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrivateDispatch {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86ab4bbe_65f6_11d1_8c13_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrivateDispatch_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ADSIInitializeDispatchManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwextensionid: i32) -> ::windows_core::HRESULT,
    pub ADSIGetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ADSIGetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ADSIGetTypeInfo: usize,
    pub ADSIGetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ADSIInvoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Variant::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ADSIInvoke: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrivateUnknown(::windows_core::IUnknown);
impl IPrivateUnknown {
    pub unsafe fn ADSIInitializeObject<P0, P1>(&self, lpszusername: P0, lpszpassword: P1, lnreserved: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ADSIInitializeObject)(::windows_core::Interface::as_raw(self), lpszusername.into_param().abi(), lpszpassword.into_param().abi(), lnreserved).ok()
    }
    pub unsafe fn ADSIReleaseObject(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ADSIReleaseObject)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPrivateUnknown, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrivateUnknown {
    type Vtable = IPrivateUnknown_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrivateUnknown {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89126bab_6ead_11d1_8c18_00c04fd8d503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrivateUnknown_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ADSIInitializeObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpszusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpszpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, lnreserved: i32) -> ::windows_core::HRESULT,
    pub ADSIReleaseObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IQueryForm(::windows_core::IUnknown);
impl IQueryForm {
    #[doc = "Required features: `\"Win32_System_Registry\"`"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0>(&self, hkform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Registry::HKEY>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), hkform.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn AddForms<P0>(&self, paddformsproc: LPCQADDFORMSPROC, lparam: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).AddForms)(::windows_core::Interface::as_raw(self), paddformsproc, lparam.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn AddPages<P0>(&self, paddpagesproc: LPCQADDPAGESPROC, lparam: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).AddPages)(::windows_core::Interface::as_raw(self), paddpagesproc, lparam.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IQueryForm, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQueryForm {
    type Vtable = IQueryForm_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IQueryForm {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cfcee30_39bd_11d0_b8d1_00a024ab2dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryForm_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkform: super::super::System::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub AddForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddformsproc: LPCQADDFORMSPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    AddForms: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub AddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddpagesproc: LPCQADDPAGESPROC, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    AddPages: usize,
}
pub const ACTRL_DS_CONTROL_ACCESS: u32 = 256u32;
pub const ACTRL_DS_CREATE_CHILD: u32 = 1u32;
pub const ACTRL_DS_DELETE_CHILD: u32 = 2u32;
pub const ACTRL_DS_DELETE_TREE: u32 = 64u32;
pub const ACTRL_DS_LIST: u32 = 4u32;
pub const ACTRL_DS_LIST_OBJECT: u32 = 128u32;
pub const ACTRL_DS_OPEN: u32 = 0u32;
pub const ACTRL_DS_READ_PROP: u32 = 16u32;
pub const ACTRL_DS_SELF: u32 = 8u32;
pub const ACTRL_DS_WRITE_PROP: u32 = 32u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED: u32 = 2u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE: u32 = 1u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH: u32 = 0u32;
pub const ADAM_SCP_FSMO_NAMING_STRING: ::windows_core::PCSTR = ::windows_core::s!("naming");
pub const ADAM_SCP_FSMO_NAMING_STRING_W: ::windows_core::PCWSTR = ::windows_core::w!("naming");
pub const ADAM_SCP_FSMO_SCHEMA_STRING: ::windows_core::PCSTR = ::windows_core::s!("schema");
pub const ADAM_SCP_FSMO_SCHEMA_STRING_W: ::windows_core::PCWSTR = ::windows_core::w!("schema");
pub const ADAM_SCP_FSMO_STRING: ::windows_core::PCSTR = ::windows_core::s!("fsmo:");
pub const ADAM_SCP_FSMO_STRING_W: ::windows_core::PCWSTR = ::windows_core::w!("fsmo:");
pub const ADAM_SCP_INSTANCE_NAME_STRING: ::windows_core::PCSTR = ::windows_core::s!("instance:");
pub const ADAM_SCP_INSTANCE_NAME_STRING_W: ::windows_core::PCWSTR = ::windows_core::w!("instance:");
pub const ADAM_SCP_PARTITION_STRING: ::windows_core::PCSTR = ::windows_core::s!("partition:");
pub const ADAM_SCP_PARTITION_STRING_W: ::windows_core::PCWSTR = ::windows_core::w!("partition:");
pub const ADAM_SCP_SITE_NAME_STRING: ::windows_core::PCSTR = ::windows_core::s!("site:");
pub const ADAM_SCP_SITE_NAME_STRING_W: ::windows_core::PCWSTR = ::windows_core::w!("site:");
pub const ADSIPROP_ADSIFLAG: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(12i32);
pub const ADSIPROP_ASYNCHRONOUS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(0i32);
pub const ADSIPROP_ATTRIBTYPES_ONLY: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(4i32);
pub const ADSIPROP_CACHE_RESULTS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(11i32);
pub const ADSIPROP_CHASE_REFERRALS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(9i32);
pub const ADSIPROP_DEREF_ALIASES: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(1i32);
pub const ADSIPROP_PAGED_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(8i32);
pub const ADSIPROP_PAGESIZE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(7i32);
pub const ADSIPROP_SEARCH_SCOPE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(5i32);
pub const ADSIPROP_SIZE_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(2i32);
pub const ADSIPROP_SORT_ON: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(10i32);
pub const ADSIPROP_TIMEOUT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(6i32);
pub const ADSIPROP_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(3i32);
pub const ADSI_DIALECT_LDAP: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(0i32);
pub const ADSI_DIALECT_SQL: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(1i32);
pub const ADSTYPE_BACKLINK: ADSTYPE = ADSTYPE(18i32);
pub const ADSTYPE_BOOLEAN: ADSTYPE = ADSTYPE(6i32);
pub const ADSTYPE_CASEIGNORE_LIST: ADSTYPE = ADSTYPE(13i32);
pub const ADSTYPE_CASE_EXACT_STRING: ADSTYPE = ADSTYPE(2i32);
pub const ADSTYPE_CASE_IGNORE_STRING: ADSTYPE = ADSTYPE(3i32);
pub const ADSTYPE_DN_STRING: ADSTYPE = ADSTYPE(1i32);
pub const ADSTYPE_DN_WITH_BINARY: ADSTYPE = ADSTYPE(27i32);
pub const ADSTYPE_DN_WITH_STRING: ADSTYPE = ADSTYPE(28i32);
pub const ADSTYPE_EMAIL: ADSTYPE = ADSTYPE(24i32);
pub const ADSTYPE_FAXNUMBER: ADSTYPE = ADSTYPE(23i32);
pub const ADSTYPE_HOLD: ADSTYPE = ADSTYPE(20i32);
pub const ADSTYPE_INTEGER: ADSTYPE = ADSTYPE(7i32);
pub const ADSTYPE_INVALID: ADSTYPE = ADSTYPE(0i32);
pub const ADSTYPE_LARGE_INTEGER: ADSTYPE = ADSTYPE(10i32);
pub const ADSTYPE_NETADDRESS: ADSTYPE = ADSTYPE(21i32);
pub const ADSTYPE_NT_SECURITY_DESCRIPTOR: ADSTYPE = ADSTYPE(25i32);
pub const ADSTYPE_NUMERIC_STRING: ADSTYPE = ADSTYPE(5i32);
pub const ADSTYPE_OBJECT_CLASS: ADSTYPE = ADSTYPE(12i32);
pub const ADSTYPE_OCTET_LIST: ADSTYPE = ADSTYPE(14i32);
pub const ADSTYPE_OCTET_STRING: ADSTYPE = ADSTYPE(8i32);
pub const ADSTYPE_PATH: ADSTYPE = ADSTYPE(15i32);
pub const ADSTYPE_POSTALADDRESS: ADSTYPE = ADSTYPE(16i32);
pub const ADSTYPE_PRINTABLE_STRING: ADSTYPE = ADSTYPE(4i32);
pub const ADSTYPE_PROV_SPECIFIC: ADSTYPE = ADSTYPE(11i32);
pub const ADSTYPE_REPLICAPOINTER: ADSTYPE = ADSTYPE(22i32);
pub const ADSTYPE_TIMESTAMP: ADSTYPE = ADSTYPE(17i32);
pub const ADSTYPE_TYPEDNAME: ADSTYPE = ADSTYPE(19i32);
pub const ADSTYPE_UNKNOWN: ADSTYPE = ADSTYPE(26i32);
pub const ADSTYPE_UTC_TIME: ADSTYPE = ADSTYPE(9i32);
pub const ADS_ACEFLAG_FAILED_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(128i32);
pub const ADS_ACEFLAG_INHERITED_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(16i32);
pub const ADS_ACEFLAG_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(2i32);
pub const ADS_ACEFLAG_INHERIT_ONLY_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(8i32);
pub const ADS_ACEFLAG_NO_PROPAGATE_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(4i32);
pub const ADS_ACEFLAG_SUCCESSFUL_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(64i32);
pub const ADS_ACEFLAG_VALID_INHERIT_FLAGS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(31i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(0i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(9i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(11i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(5i32);
pub const ADS_ACETYPE_ACCESS_DENIED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(1i32);
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(10i32);
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(12i32);
pub const ADS_ACETYPE_ACCESS_DENIED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(6i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(14i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(16i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(8i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(2i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(13i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(15i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(7i32);
pub const ADS_ATTR_APPEND: u32 = 3u32;
pub const ADS_ATTR_CLEAR: u32 = 1u32;
pub const ADS_ATTR_DELETE: u32 = 4u32;
pub const ADS_ATTR_UPDATE: u32 = 2u32;
pub const ADS_AUTH_RESERVED: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2147483648u32);
pub const ADS_CHASE_REFERRALS_ALWAYS: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(96i32);
pub const ADS_CHASE_REFERRALS_EXTERNAL: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(64i32);
pub const ADS_CHASE_REFERRALS_NEVER: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(0i32);
pub const ADS_CHASE_REFERRALS_SUBORDINATE: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(32i32);
pub const ADS_DEREF_ALWAYS: ADS_DEREFENUM = ADS_DEREFENUM(3i32);
pub const ADS_DEREF_FINDING: ADS_DEREFENUM = ADS_DEREFENUM(2i32);
pub const ADS_DEREF_NEVER: ADS_DEREFENUM = ADS_DEREFENUM(0i32);
pub const ADS_DEREF_SEARCHING: ADS_DEREFENUM = ADS_DEREFENUM(1i32);
pub const ADS_DISPLAY_FULL: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(1i32);
pub const ADS_DISPLAY_VALUE_ONLY: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(2i32);
pub const ADS_ESCAPEDMODE_DEFAULT: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(1i32);
pub const ADS_ESCAPEDMODE_OFF: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(3i32);
pub const ADS_ESCAPEDMODE_OFF_EX: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(4i32);
pub const ADS_ESCAPEDMODE_ON: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(2i32);
pub const ADS_EXT_INITCREDENTIALS: u32 = 1u32;
pub const ADS_EXT_INITIALIZE_COMPLETE: u32 = 2u32;
pub const ADS_EXT_MAXEXTDISPID: u32 = 16777215u32;
pub const ADS_EXT_MINEXTDISPID: u32 = 1u32;
pub const ADS_FAST_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(32u32);
pub const ADS_FLAG_INHERITED_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(2i32);
pub const ADS_FLAG_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(1i32);
pub const ADS_FORMAT_LEAF: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(11i32);
pub const ADS_FORMAT_PROVIDER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(10i32);
pub const ADS_FORMAT_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(9i32);
pub const ADS_FORMAT_WINDOWS: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(1i32);
pub const ADS_FORMAT_WINDOWS_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(3i32);
pub const ADS_FORMAT_WINDOWS_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(2i32);
pub const ADS_FORMAT_WINDOWS_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(4i32);
pub const ADS_FORMAT_X500: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(5i32);
pub const ADS_FORMAT_X500_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(7i32);
pub const ADS_FORMAT_X500_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(6i32);
pub const ADS_FORMAT_X500_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(8i32);
pub const ADS_GROUP_TYPE_DOMAIN_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
pub const ADS_GROUP_TYPE_GLOBAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(2i32);
pub const ADS_GROUP_TYPE_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
pub const ADS_GROUP_TYPE_SECURITY_ENABLED: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(-2147483648i32);
pub const ADS_GROUP_TYPE_UNIVERSAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(8i32);
pub const ADS_NAME_INITTYPE_DOMAIN: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(1i32);
pub const ADS_NAME_INITTYPE_GC: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(3i32);
pub const ADS_NAME_INITTYPE_SERVER: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(2i32);
pub const ADS_NAME_TYPE_1779: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(1i32);
pub const ADS_NAME_TYPE_CANONICAL: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(2i32);
pub const ADS_NAME_TYPE_CANONICAL_EX: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(10i32);
pub const ADS_NAME_TYPE_DISPLAY: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(4i32);
pub const ADS_NAME_TYPE_DOMAIN_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(5i32);
pub const ADS_NAME_TYPE_ENTERPRISE_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(6i32);
pub const ADS_NAME_TYPE_GUID: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(7i32);
pub const ADS_NAME_TYPE_NT4: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(3i32);
pub const ADS_NAME_TYPE_SERVICE_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(11i32);
pub const ADS_NAME_TYPE_SID_OR_SID_HISTORY_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(12i32);
pub const ADS_NAME_TYPE_UNKNOWN: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(8i32);
pub const ADS_NAME_TYPE_USER_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(9i32);
pub const ADS_NO_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(16u32);
pub const ADS_NO_REFERRAL_CHASING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1024u32);
pub const ADS_OPTION_ACCUMULATIVE_MODIFICATION: ADS_OPTION_ENUM = ADS_OPTION_ENUM(8i32);
pub const ADS_OPTION_MUTUAL_AUTH_STATUS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(4i32);
pub const ADS_OPTION_PAGE_SIZE: ADS_OPTION_ENUM = ADS_OPTION_ENUM(2i32);
pub const ADS_OPTION_PASSWORD_METHOD: ADS_OPTION_ENUM = ADS_OPTION_ENUM(7i32);
pub const ADS_OPTION_PASSWORD_PORTNUMBER: ADS_OPTION_ENUM = ADS_OPTION_ENUM(6i32);
pub const ADS_OPTION_QUOTA: ADS_OPTION_ENUM = ADS_OPTION_ENUM(5i32);
pub const ADS_OPTION_REFERRALS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(1i32);
pub const ADS_OPTION_SECURITY_MASK: ADS_OPTION_ENUM = ADS_OPTION_ENUM(3i32);
pub const ADS_OPTION_SERVERNAME: ADS_OPTION_ENUM = ADS_OPTION_ENUM(0i32);
pub const ADS_OPTION_SKIP_SID_LOOKUP: ADS_OPTION_ENUM = ADS_OPTION_ENUM(9i32);
pub const ADS_PASSWORD_ENCODE_CLEAR: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(1i32);
pub const ADS_PASSWORD_ENCODE_REQUIRE_SSL: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(0i32);
pub const ADS_PATH_FILE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(1i32);
pub const ADS_PATH_FILESHARE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(2i32);
pub const ADS_PATH_REGISTRY: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(3i32);
pub const ADS_PROMPT_CREDENTIALS: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(8u32);
pub const ADS_PROPERTY_APPEND: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(3i32);
pub const ADS_PROPERTY_CLEAR: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(1i32);
pub const ADS_PROPERTY_DELETE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(4i32);
pub const ADS_PROPERTY_UPDATE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(2i32);
pub const ADS_READONLY_SERVER: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(4u32);
pub const ADS_RIGHT_ACCESS_SYSTEM_SECURITY: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16777216i32);
pub const ADS_RIGHT_ACTRL_DS_LIST: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(4i32);
pub const ADS_RIGHT_DELETE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(65536i32);
pub const ADS_RIGHT_DS_CONTROL_ACCESS: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(256i32);
pub const ADS_RIGHT_DS_CREATE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1i32);
pub const ADS_RIGHT_DS_DELETE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(2i32);
pub const ADS_RIGHT_DS_DELETE_TREE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(64i32);
pub const ADS_RIGHT_DS_LIST_OBJECT: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(128i32);
pub const ADS_RIGHT_DS_READ_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16i32);
pub const ADS_RIGHT_DS_SELF: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(8i32);
pub const ADS_RIGHT_DS_WRITE_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(32i32);
pub const ADS_RIGHT_GENERIC_ALL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(268435456i32);
pub const ADS_RIGHT_GENERIC_EXECUTE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(536870912i32);
pub const ADS_RIGHT_GENERIC_READ: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(-2147483648i32);
pub const ADS_RIGHT_GENERIC_WRITE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1073741824i32);
pub const ADS_RIGHT_READ_CONTROL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(131072i32);
pub const ADS_RIGHT_SYNCHRONIZE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1048576i32);
pub const ADS_RIGHT_WRITE_DAC: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(262144i32);
pub const ADS_RIGHT_WRITE_OWNER: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(524288i32);
pub const ADS_SCOPE_BASE: ADS_SCOPEENUM = ADS_SCOPEENUM(0i32);
pub const ADS_SCOPE_ONELEVEL: ADS_SCOPEENUM = ADS_SCOPEENUM(1i32);
pub const ADS_SCOPE_SUBTREE: ADS_SCOPEENUM = ADS_SCOPEENUM(2i32);
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1024i32);
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(256i32);
pub const ADS_SD_CONTROL_SE_DACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8i32);
pub const ADS_SD_CONTROL_SE_DACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4i32);
pub const ADS_SD_CONTROL_SE_DACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4096i32);
pub const ADS_SD_CONTROL_SE_GROUP_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2i32);
pub const ADS_SD_CONTROL_SE_OWNER_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1i32);
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2048i32);
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(512i32);
pub const ADS_SD_CONTROL_SE_SACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32i32);
pub const ADS_SD_CONTROL_SE_SACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(16i32);
pub const ADS_SD_CONTROL_SE_SACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8192i32);
pub const ADS_SD_CONTROL_SE_SELF_RELATIVE: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32768i32);
pub const ADS_SD_FORMAT_HEXSTRING: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(3i32);
pub const ADS_SD_FORMAT_IID: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(1i32);
pub const ADS_SD_FORMAT_RAW: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(2i32);
pub const ADS_SD_REVISION_DS: ADS_SD_REVISION_ENUM = ADS_SD_REVISION_ENUM(4i32);
pub const ADS_SEARCHPREF_ASYNCHRONOUS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(0i32);
pub const ADS_SEARCHPREF_ATTRIBTYPES_ONLY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(4i32);
pub const ADS_SEARCHPREF_ATTRIBUTE_QUERY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(15i32);
pub const ADS_SEARCHPREF_CACHE_RESULTS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(11i32);
pub const ADS_SEARCHPREF_CHASE_REFERRALS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(9i32);
pub const ADS_SEARCHPREF_DEREF_ALIASES: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(1i32);
pub const ADS_SEARCHPREF_DIRSYNC: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(12i32);
pub const ADS_SEARCHPREF_DIRSYNC_FLAG: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(17i32);
pub const ADS_SEARCHPREF_EXTENDED_DN: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(18i32);
pub const ADS_SEARCHPREF_PAGED_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(8i32);
pub const ADS_SEARCHPREF_PAGESIZE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(7i32);
pub const ADS_SEARCHPREF_SEARCH_SCOPE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(5i32);
pub const ADS_SEARCHPREF_SECURITY_MASK: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(16i32);
pub const ADS_SEARCHPREF_SIZE_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(2i32);
pub const ADS_SEARCHPREF_SORT_ON: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(10i32);
pub const ADS_SEARCHPREF_TIMEOUT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(6i32);
pub const ADS_SEARCHPREF_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(3i32);
pub const ADS_SEARCHPREF_TOMBSTONE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(13i32);
pub const ADS_SEARCHPREF_VLV: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(14i32);
pub const ADS_SECURE_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1u32);
pub const ADS_SECURITY_INFO_DACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(4i32);
pub const ADS_SECURITY_INFO_GROUP: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(2i32);
pub const ADS_SECURITY_INFO_OWNER: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(1i32);
pub const ADS_SECURITY_INFO_SACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(8i32);
pub const ADS_SERVER_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(512u32);
pub const ADS_SETTYPE_DN: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(4i32);
pub const ADS_SETTYPE_FULL: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(1i32);
pub const ADS_SETTYPE_PROVIDER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(2i32);
pub const ADS_SETTYPE_SERVER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(3i32);
pub const ADS_STATUS_INVALID_SEARCHPREF: ADS_STATUSENUM = ADS_STATUSENUM(1i32);
pub const ADS_STATUS_INVALID_SEARCHPREFVALUE: ADS_STATUSENUM = ADS_STATUSENUM(2i32);
pub const ADS_STATUS_S_OK: ADS_STATUSENUM = ADS_STATUSENUM(0i32);
pub const ADS_SYSTEMFLAG_ATTR_IS_CONSTRUCTED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(4i32);
pub const ADS_SYSTEMFLAG_ATTR_NOT_REPLICATED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_LIMITED_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(268435456i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(536870912i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1073741824i32);
pub const ADS_SYSTEMFLAG_CR_NTDS_DOMAIN: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(2i32);
pub const ADS_SYSTEMFLAG_CR_NTDS_NC: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
pub const ADS_SYSTEMFLAG_DISALLOW_DELETE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(-2147483648i32);
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(67108864i32);
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(134217728i32);
pub const ADS_UF_ACCOUNTDISABLE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2i32);
pub const ADS_UF_DONT_EXPIRE_PASSWD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(65536i32);
pub const ADS_UF_DONT_REQUIRE_PREAUTH: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4194304i32);
pub const ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(128i32);
pub const ADS_UF_HOMEDIR_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8i32);
pub const ADS_UF_INTERDOMAIN_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2048i32);
pub const ADS_UF_LOCKOUT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16i32);
pub const ADS_UF_MNS_LOGON_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(131072i32);
pub const ADS_UF_NORMAL_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(512i32);
pub const ADS_UF_NOT_DELEGATED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1048576i32);
pub const ADS_UF_PASSWD_CANT_CHANGE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(64i32);
pub const ADS_UF_PASSWD_NOTREQD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(32i32);
pub const ADS_UF_PASSWORD_EXPIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8388608i32);
pub const ADS_UF_SCRIPT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1i32);
pub const ADS_UF_SERVER_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8192i32);
pub const ADS_UF_SMARTCARD_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(262144i32);
pub const ADS_UF_TEMP_DUPLICATE_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(256i32);
pub const ADS_UF_TRUSTED_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(524288i32);
pub const ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16777216i32);
pub const ADS_UF_USE_DES_KEY_ONLY: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2097152i32);
pub const ADS_UF_WORKSTATION_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4096i32);
pub const ADS_USE_DELEGATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(256u32);
pub const ADS_USE_ENCRYPTION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
pub const ADS_USE_SEALING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(128u32);
pub const ADS_USE_SIGNING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(64u32);
pub const ADS_USE_SSL: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
pub const ADSystemInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50b6327f_afd1_11d2_9cb9_0000f87a369e);
pub const ADsSecurityUtility: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf270c64a_ffb8_4ae4_85fe_3a75e5347966);
pub const AccessControlEntry: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb75ac000_9bdd_11d0_852c_00c04fd8d503);
pub const AccessControlList: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb85ea052_9bdd_11d0_852c_00c04fd8d503);
pub const BackLink: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcbf906f_4080_11d1_a3ac_00c04fb950dc);
pub const CFSTR_DSDISPLAYSPECOPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("DsDisplaySpecOptions");
pub const CFSTR_DSOBJECTNAMES: ::windows_core::PCWSTR = ::windows_core::w!("DsObjectNames");
pub const CFSTR_DSOP_DS_SELECTION_LIST: ::windows_core::PCWSTR = ::windows_core::w!("CFSTR_DSOP_DS_SELECTION_LIST");
pub const CFSTR_DSPROPERTYPAGEINFO: ::windows_core::PCWSTR = ::windows_core::w!("DsPropPageInfo");
pub const CFSTR_DSQUERYPARAMS: ::windows_core::PCWSTR = ::windows_core::w!("DsQueryParameters");
pub const CFSTR_DSQUERYSCOPE: ::windows_core::PCWSTR = ::windows_core::w!("DsQueryScope");
pub const CFSTR_DS_DISPLAY_SPEC_OPTIONS: ::windows_core::PCWSTR = ::windows_core::w!("DsDisplaySpecOptions");
pub const CLSID_CommonQuery: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83bc5ec0_6f2a_11d0_a1c4_00aa00c16e65);
pub const CLSID_DsAdminCreateObj: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe301a009_f901_11d2_82b9_00c04f68928b);
pub const CLSID_DsDisplaySpecifier: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ab4a8c0_6a0b_11d2_ad49_00c04fa31a86);
pub const CLSID_DsDomainTreeBrowser: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1698790a_e2b4_11d0_b0b1_00c04fd8dca6);
pub const CLSID_DsFindAdvanced: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83ee3fe3_57d9_11d0_b932_00a024ab2dbb);
pub const CLSID_DsFindComputer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16006700_87ad_11d0_9140_00aa00c16e65);
pub const CLSID_DsFindContainer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1b3cbf2_886a_11d0_9140_00aa00c16e65);
pub const CLSID_DsFindDomainController: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x538c7b7e_d25e_11d0_9742_00a0c906af45);
pub const CLSID_DsFindFrsMembers: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94ce4b18_b3d3_11d1_b9b4_00c04fd8d5b0);
pub const CLSID_DsFindObjects: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83ee3fe1_57d9_11d0_b932_00a024ab2dbb);
pub const CLSID_DsFindPeople: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83ee3fe2_57d9_11d0_b932_00a024ab2dbb);
pub const CLSID_DsFindPrinter: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb577f070_7ee2_11d0_913f_00aa00c16e65);
pub const CLSID_DsFindVolume: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1b3cbf1_886a_11d0_9140_00aa00c16e65);
pub const CLSID_DsFindWriteableDomainController: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cbef079_aa84_444b_bc70_68e41283eabc);
pub const CLSID_DsFolderProperties: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e51e0d0_6e0f_11d2_9601_00c04fa31a86);
pub const CLSID_DsObjectPicker: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17d6ccd8_3b7b_11d2_b9e0_00c04fd8dbf7);
pub const CLSID_DsPropertyPages: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d45d530_764b_11d0_a1ca_00aa00c16e65);
pub const CLSID_DsQuery: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a23e65e_31c2_11d0_891c_00a024ab2dbb);
pub const CLSID_MicrosoftDS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe1290f0_cfbd_11cf_a330_00aa00c16e65);
pub const CQFF_ISOPTIONAL: u32 = 2u32;
pub const CQFF_NOGLOBALPAGES: u32 = 1u32;
pub const CQPM_CLEARFORM: u32 = 6u32;
pub const CQPM_ENABLE: u32 = 3u32;
pub const CQPM_GETPARAMETERS: u32 = 5u32;
pub const CQPM_HANDLERSPECIFIC: u32 = 268435456u32;
pub const CQPM_HELP: u32 = 8u32;
pub const CQPM_INITIALIZE: u32 = 1u32;
pub const CQPM_PERSIST: u32 = 7u32;
pub const CQPM_RELEASE: u32 = 2u32;
pub const CQPM_SETDEFAULTPARAMETERS: u32 = 9u32;
pub const CaseIgnoreList: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15f88a55_4680_11d1_a3b4_00c04fb950dc);
pub const DBDTF_RETURNEXTERNAL: u32 = 4u32;
pub const DBDTF_RETURNFQDN: u32 = 1u32;
pub const DBDTF_RETURNINBOUND: u32 = 8u32;
pub const DBDTF_RETURNINOUTBOUND: u32 = 16u32;
pub const DBDTF_RETURNMIXEDDOMAINS: u32 = 2u32;
pub const DNWithBinary: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e99c0a3_f935_11d2_ba96_00c04fb6d0d1);
pub const DNWithString: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x334857cc_f934_11d2_ba96_00c04fb6d0d1);
pub const DSA_NEWOBJ_CTX_CLEANUP: u32 = 4u32;
pub const DSA_NEWOBJ_CTX_COMMIT: u32 = 2u32;
pub const DSA_NEWOBJ_CTX_POSTCOMMIT: u32 = 3u32;
pub const DSA_NEWOBJ_CTX_PRECOMMIT: u32 = 1u32;
pub const DSA_NOTIFY_DEL: u32 = 1u32;
pub const DSA_NOTIFY_FLAG_ADDITIONAL_DATA: u32 = 2u32;
pub const DSA_NOTIFY_FLAG_FORCE_ADDITIONAL_DATA: u32 = 1u32;
pub const DSA_NOTIFY_MOV: u32 = 4u32;
pub const DSA_NOTIFY_PROP: u32 = 8u32;
pub const DSA_NOTIFY_REN: u32 = 2u32;
pub const DSBF_DISPLAYNAME: u32 = 4u32;
pub const DSBF_ICONLOCATION: u32 = 2u32;
pub const DSBF_STATE: u32 = 1u32;
pub const DSBID_BANNER: u32 = 256u32;
pub const DSBID_CONTAINERLIST: u32 = 257u32;
pub const DSBI_CHECKBOXES: u32 = 256u32;
pub const DSBI_DONTSIGNSEAL: u32 = 33554432u32;
pub const DSBI_ENTIREDIRECTORY: u32 = 589824u32;
pub const DSBI_EXPANDONOPEN: u32 = 262144u32;
pub const DSBI_HASCREDENTIALS: u32 = 2097152u32;
pub const DSBI_IGNORETREATASLEAF: u32 = 4194304u32;
pub const DSBI_INCLUDEHIDDEN: u32 = 131072u32;
pub const DSBI_NOBUTTONS: u32 = 1u32;
pub const DSBI_NOLINES: u32 = 2u32;
pub const DSBI_NOLINESATROOT: u32 = 4u32;
pub const DSBI_NOROOT: u32 = 65536u32;
pub const DSBI_RETURNOBJECTCLASS: u32 = 16777216u32;
pub const DSBI_RETURN_FORMAT: u32 = 1048576u32;
pub const DSBI_SIMPLEAUTHENTICATE: u32 = 8388608u32;
pub const DSBM_CHANGEIMAGESTATE: u32 = 102u32;
pub const DSBM_CONTEXTMENU: u32 = 104u32;
pub const DSBM_HELP: u32 = 103u32;
pub const DSBM_QUERYINSERT: u32 = 100u32;
pub const DSBM_QUERYINSERTA: u32 = 101u32;
pub const DSBM_QUERYINSERTW: u32 = 100u32;
pub const DSBS_CHECKED: u32 = 1u32;
pub const DSBS_HIDDEN: u32 = 2u32;
pub const DSBS_ROOT: u32 = 4u32;
pub const DSB_MAX_DISPLAYNAME_CHARS: u32 = 64u32;
pub const DSCCIF_HASWIZARDDIALOG: u32 = 1u32;
pub const DSCCIF_HASWIZARDPRIMARYPAGE: u32 = 2u32;
pub const DSDSOF_DONTSIGNSEAL: u32 = 4u32;
pub const DSDSOF_DSAVAILABLE: u32 = 1073741824u32;
pub const DSDSOF_HASUSERANDSERVERINFO: u32 = 1u32;
pub const DSDSOF_SIMPLEAUTHENTICATE: u32 = 2u32;
pub const DSECAF_NOTLISTED: u32 = 1u32;
pub const DSGIF_DEFAULTISCONTAINER: u32 = 32u32;
pub const DSGIF_GETDEFAULTICON: u32 = 16u32;
pub const DSGIF_ISDISABLED: u32 = 2u32;
pub const DSGIF_ISMASK: u32 = 15u32;
pub const DSGIF_ISNORMAL: u32 = 0u32;
pub const DSGIF_ISOPEN: u32 = 1u32;
pub const DSICCF_IGNORETREATASLEAF: u32 = 1u32;
pub const DSOBJECT_ISCONTAINER: u32 = 1u32;
pub const DSOBJECT_READONLYPAGES: u32 = 2147483648u32;
pub const DSOP_DOWNLEVEL_FILTER_ALL_APP_PACKAGES: u32 = 2281701376u32;
pub const DSOP_DOWNLEVEL_FILTER_ALL_WELLKNOWN_SIDS: u32 = 2147614720u32;
pub const DSOP_DOWNLEVEL_FILTER_ANONYMOUS: u32 = 2147483712u32;
pub const DSOP_DOWNLEVEL_FILTER_AUTHENTICATED_USER: u32 = 2147483680u32;
pub const DSOP_DOWNLEVEL_FILTER_BATCH: u32 = 2147483776u32;
pub const DSOP_DOWNLEVEL_FILTER_COMPUTERS: u32 = 2147483656u32;
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_GROUP: u32 = 2147484160u32;
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_OWNER: u32 = 2147483904u32;
pub const DSOP_DOWNLEVEL_FILTER_DIALUP: u32 = 2147484672u32;
pub const DSOP_DOWNLEVEL_FILTER_EXCLUDE_BUILTIN_GROUPS: u32 = 2147516416u32;
pub const DSOP_DOWNLEVEL_FILTER_GLOBAL_GROUPS: u32 = 2147483652u32;
pub const DSOP_DOWNLEVEL_FILTER_IIS_APP_POOL: u32 = 2214592512u32;
pub const DSOP_DOWNLEVEL_FILTER_INTERACTIVE: u32 = 2147485696u32;
pub const DSOP_DOWNLEVEL_FILTER_INTERNET_USER: u32 = 2149580800u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_ACCOUNTS: u32 = 2415919104u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_GROUPS: u32 = 2147483650u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_LOGON: u32 = 2164260864u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_SERVICE: u32 = 2147745792u32;
pub const DSOP_DOWNLEVEL_FILTER_NETWORK: u32 = 2147487744u32;
pub const DSOP_DOWNLEVEL_FILTER_NETWORK_SERVICE: u32 = 2148007936u32;
pub const DSOP_DOWNLEVEL_FILTER_OWNER_RIGHTS: u32 = 2151677952u32;
pub const DSOP_DOWNLEVEL_FILTER_REMOTE_LOGON: u32 = 2148532224u32;
pub const DSOP_DOWNLEVEL_FILTER_SERVICE: u32 = 2147491840u32;
pub const DSOP_DOWNLEVEL_FILTER_SERVICES: u32 = 2155872256u32;
pub const DSOP_DOWNLEVEL_FILTER_SYSTEM: u32 = 2147500032u32;
pub const DSOP_DOWNLEVEL_FILTER_TERMINAL_SERVER: u32 = 2147549184u32;
pub const DSOP_DOWNLEVEL_FILTER_THIS_ORG_CERT: u32 = 2181038080u32;
pub const DSOP_DOWNLEVEL_FILTER_USERS: u32 = 2147483649u32;
pub const DSOP_DOWNLEVEL_FILTER_WORLD: u32 = 2147483664u32;
pub const DSOP_FILTER_BUILTIN_GROUPS: u32 = 4u32;
pub const DSOP_FILTER_COMPUTERS: u32 = 2048u32;
pub const DSOP_FILTER_CONTACTS: u32 = 1024u32;
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_DL: u32 = 256u32;
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_SE: u32 = 512u32;
pub const DSOP_FILTER_GLOBAL_GROUPS_DL: u32 = 64u32;
pub const DSOP_FILTER_GLOBAL_GROUPS_SE: u32 = 128u32;
pub const DSOP_FILTER_INCLUDE_ADVANCED_VIEW: u32 = 1u32;
pub const DSOP_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 8192u32;
pub const DSOP_FILTER_SERVICE_ACCOUNTS: u32 = 4096u32;
pub const DSOP_FILTER_UNIVERSAL_GROUPS_DL: u32 = 16u32;
pub const DSOP_FILTER_UNIVERSAL_GROUPS_SE: u32 = 32u32;
pub const DSOP_FILTER_USERS: u32 = 2u32;
pub const DSOP_FILTER_WELL_KNOWN_PRINCIPALS: u32 = 8u32;
pub const DSOP_FLAG_MULTISELECT: u32 = 1u32;
pub const DSOP_FLAG_SKIP_TARGET_COMPUTER_DC_CHECK: u32 = 2u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_COMPUTERS: u32 = 256u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_CONTACTS: u32 = 512u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_GROUPS: u32 = 128u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 2048u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_SERVICE_ACCOUNTS: u32 = 1024u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_USERS: u32 = 64u32;
pub const DSOP_SCOPE_FLAG_STARTING_SCOPE: u32 = 1u32;
pub const DSOP_SCOPE_FLAG_WANT_DOWNLEVEL_BUILTIN_PATH: u32 = 32u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_GC: u32 = 8u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_LDAP: u32 = 4u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_WINNT: u32 = 2u32;
pub const DSOP_SCOPE_FLAG_WANT_SID_PATH: u32 = 16u32;
pub const DSOP_SCOPE_TYPE_DOWNLEVEL_JOINED_DOMAIN: u32 = 4u32;
pub const DSOP_SCOPE_TYPE_ENTERPRISE_DOMAIN: u32 = 8u32;
pub const DSOP_SCOPE_TYPE_EXTERNAL_DOWNLEVEL_DOMAIN: u32 = 64u32;
pub const DSOP_SCOPE_TYPE_EXTERNAL_UPLEVEL_DOMAIN: u32 = 32u32;
pub const DSOP_SCOPE_TYPE_GLOBAL_CATALOG: u32 = 16u32;
pub const DSOP_SCOPE_TYPE_TARGET_COMPUTER: u32 = 1u32;
pub const DSOP_SCOPE_TYPE_UPLEVEL_JOINED_DOMAIN: u32 = 2u32;
pub const DSOP_SCOPE_TYPE_USER_ENTERED_DOWNLEVEL_SCOPE: u32 = 512u32;
pub const DSOP_SCOPE_TYPE_USER_ENTERED_UPLEVEL_SCOPE: u32 = 256u32;
pub const DSOP_SCOPE_TYPE_WORKGROUP: u32 = 128u32;
pub const DSPROP_ATTRCHANGED_MSG: ::windows_core::PCWSTR = ::windows_core::w!("DsPropAttrChanged");
pub const DSPROVIDER_ADVANCED: u32 = 16u32;
pub const DSPROVIDER_AD_LDS: u32 = 32u32;
pub const DSPROVIDER_UNUSED_0: u32 = 1u32;
pub const DSPROVIDER_UNUSED_1: u32 = 2u32;
pub const DSPROVIDER_UNUSED_2: u32 = 4u32;
pub const DSPROVIDER_UNUSED_3: u32 = 8u32;
pub const DSQPF_ENABLEADMINFEATURES: u32 = 8u32;
pub const DSQPF_ENABLEADVANCEDFEATURES: u32 = 16u32;
pub const DSQPF_HASCREDENTIALS: u32 = 32u32;
pub const DSQPF_NOCHOOSECOLUMNS: u32 = 64u32;
pub const DSQPF_NOSAVE: u32 = 1u32;
pub const DSQPF_SAVELOCATION: u32 = 2u32;
pub const DSQPF_SHOWHIDDENOBJECTS: u32 = 4u32;
pub const DSQPM_GETCLASSLIST: u32 = 268435456u32;
pub const DSQPM_HELPTOPICS: u32 = 268435457u32;
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: u32 = 16777216u32;
pub const DSROLE_PRIMARY_DS_MIXED_MODE: u32 = 2u32;
pub const DSROLE_PRIMARY_DS_READONLY: u32 = 8u32;
pub const DSROLE_PRIMARY_DS_RUNNING: u32 = 1u32;
pub const DSROLE_UPGRADE_IN_PROGRESS: u32 = 4u32;
pub const DSSSF_DONTSIGNSEAL: u32 = 2u32;
pub const DSSSF_DSAVAILABLE: u32 = 2147483648u32;
pub const DSSSF_SIMPLEAUTHENTICATE: u32 = 1u32;
pub const DS_AVOID_SELF: u32 = 16384u32;
pub const DS_BACKGROUND_ONLY: u32 = 256u32;
pub const DS_BEHAVIOR_LONGHORN: u32 = 3u32;
pub const DS_BEHAVIOR_WIN2000: u32 = 0u32;
pub const DS_BEHAVIOR_WIN2003: u32 = 2u32;
pub const DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS: u32 = 1u32;
pub const DS_BEHAVIOR_WIN2008: u32 = 3u32;
pub const DS_BEHAVIOR_WIN2008R2: u32 = 4u32;
pub const DS_BEHAVIOR_WIN2012: u32 = 5u32;
pub const DS_BEHAVIOR_WIN2012R2: u32 = 6u32;
pub const DS_BEHAVIOR_WIN2016: u32 = 7u32;
pub const DS_BEHAVIOR_WIN7: u32 = 4u32;
pub const DS_BEHAVIOR_WIN8: u32 = 5u32;
pub const DS_BEHAVIOR_WINBLUE: u32 = 6u32;
pub const DS_BEHAVIOR_WINTHRESHOLD: u32 = 7u32;
pub const DS_CANONICAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(7i32);
pub const DS_CANONICAL_NAME_EX: DS_NAME_FORMAT = DS_NAME_FORMAT(9i32);
pub const DS_CLOSEST_FLAG: u32 = 128u32;
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: u32 = 8388608u32;
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: u32 = 524288u32;
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: u32 = 2097152u32;
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: u32 = 4194304u32;
pub const DS_DIRECTORY_SERVICE_PREFERRED: u32 = 32u32;
pub const DS_DIRECTORY_SERVICE_REQUIRED: u32 = 16u32;
pub const DS_DISPLAY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(3i32);
pub const DS_DNS_CONTROLLER_FLAG: u32 = 536870912u32;
pub const DS_DNS_DOMAIN_FLAG: u32 = 1073741824u32;
pub const DS_DNS_DOMAIN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(12i32);
pub const DS_DNS_FOREST_FLAG: u32 = 2147483648u32;
pub const DS_DOMAIN_DIRECT_INBOUND: u32 = 32u32;
pub const DS_DOMAIN_DIRECT_OUTBOUND: u32 = 2u32;
pub const DS_DOMAIN_IN_FOREST: u32 = 1u32;
pub const DS_DOMAIN_NATIVE_MODE: u32 = 16u32;
pub const DS_DOMAIN_PRIMARY: u32 = 8u32;
pub const DS_DOMAIN_TREE_ROOT: u32 = 4u32;
pub const DS_DS_10_FLAG: u32 = 65536u32;
pub const DS_DS_8_FLAG: u32 = 16384u32;
pub const DS_DS_9_FLAG: u32 = 32768u32;
pub const DS_DS_FLAG: u32 = 16u32;
pub const DS_EXIST_ADVISORY_MODE: u32 = 1u32;
pub const DS_FORCE_REDISCOVERY: u32 = 1u32;
pub const DS_FQDN_1779_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(1i32);
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: u32 = 4096u32;
pub const DS_GC_FLAG: u32 = 4u32;
pub const DS_GC_SERVER_REQUIRED: u32 = 64u32;
pub const DS_GFTI_UPDATE_TDO: u32 = 1u32;
pub const DS_GFTI_VALID_FLAGS: u32 = 1u32;
pub const DS_GOOD_TIMESERV_FLAG: u32 = 512u32;
pub const DS_GOOD_TIMESERV_PREFERRED: u32 = 8192u32;
pub const DS_INSTANCETYPE_IS_NC_HEAD: u32 = 1u32;
pub const DS_INSTANCETYPE_NC_COMING: u32 = 16u32;
pub const DS_INSTANCETYPE_NC_GOING: u32 = 32u32;
pub const DS_INSTANCETYPE_NC_IS_WRITEABLE: u32 = 4u32;
pub const DS_IP_REQUIRED: u32 = 512u32;
pub const DS_IS_DNS_NAME: u32 = 131072u32;
pub const DS_IS_FLAT_NAME: u32 = 65536u32;
pub const DS_KCC_FLAG_ASYNC_OP: u32 = 1u32;
pub const DS_KCC_FLAG_DAMPED: u32 = 2u32;
pub const DS_KCC_TASKID_UPDATE_TOPOLOGY: DS_KCC_TASKID = DS_KCC_TASKID(0i32);
pub const DS_KDC_FLAG: u32 = 32u32;
pub const DS_KDC_REQUIRED: u32 = 1024u32;
pub const DS_KEY_LIST_FLAG: u32 = 131072u32;
pub const DS_KEY_LIST_SUPPORT_REQUIRED: u32 = 16777216u32;
pub const DS_LDAP_FLAG: u32 = 8u32;
pub const DS_LIST_ACCOUNT_OBJECT_FOR_SERVER: u32 = 2u32;
pub const DS_LIST_DNS_HOST_NAME_FOR_SERVER: u32 = 1u32;
pub const DS_LIST_DSA_OBJECT_FOR_SERVER: u32 = 0u32;
pub const DS_MANGLE_OBJECT_RDN_FOR_DELETION: DS_MANGLE_FOR = DS_MANGLE_FOR(1i32);
pub const DS_MANGLE_OBJECT_RDN_FOR_NAME_CONFLICT: DS_MANGLE_FOR = DS_MANGLE_FOR(2i32);
pub const DS_MANGLE_UNKNOWN: DS_MANGLE_FOR = DS_MANGLE_FOR(0i32);
pub const DS_NAME_ERROR_DOMAIN_ONLY: DS_NAME_ERROR = DS_NAME_ERROR(5i32);
pub const DS_NAME_ERROR_NOT_FOUND: DS_NAME_ERROR = DS_NAME_ERROR(2i32);
pub const DS_NAME_ERROR_NOT_UNIQUE: DS_NAME_ERROR = DS_NAME_ERROR(3i32);
pub const DS_NAME_ERROR_NO_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(4i32);
pub const DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(6i32);
pub const DS_NAME_ERROR_RESOLVING: DS_NAME_ERROR = DS_NAME_ERROR(1i32);
pub const DS_NAME_ERROR_TRUST_REFERRAL: DS_NAME_ERROR = DS_NAME_ERROR(7i32);
pub const DS_NAME_FLAG_EVAL_AT_DC: DS_NAME_FLAGS = DS_NAME_FLAGS(2i32);
pub const DS_NAME_FLAG_GCVERIFY: DS_NAME_FLAGS = DS_NAME_FLAGS(4i32);
pub const DS_NAME_FLAG_SYNTACTICAL_ONLY: DS_NAME_FLAGS = DS_NAME_FLAGS(1i32);
pub const DS_NAME_FLAG_TRUST_REFERRAL: DS_NAME_FLAGS = DS_NAME_FLAGS(8i32);
pub const DS_NAME_NO_ERROR: DS_NAME_ERROR = DS_NAME_ERROR(0i32);
pub const DS_NAME_NO_FLAGS: DS_NAME_FLAGS = DS_NAME_FLAGS(0i32);
pub const DS_NDNC_FLAG: u32 = 1024u32;
pub const DS_NOTIFY_AFTER_SITE_RECORDS: u32 = 2u32;
pub const DS_NT4_ACCOUNT_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(2i32);
pub const DS_ONLY_DO_SITE_NAME: u32 = 1u32;
pub const DS_ONLY_LDAP_NEEDED: u32 = 32768u32;
pub const DS_PDC_FLAG: u32 = 1u32;
pub const DS_PDC_REQUIRED: u32 = 128u32;
pub const DS_PING_FLAGS: u32 = 1048575u32;
pub const DS_PROP_ADMIN_PREFIX: ::windows_core::PCWSTR = ::windows_core::w!("admin");
pub const DS_PROP_SHELL_PREFIX: ::windows_core::PCWSTR = ::windows_core::w!("shell");
pub const DS_REPADD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPADD_ASYNCHRONOUS_REPLICA: u32 = 32u32;
pub const DS_REPADD_CRITICAL: u32 = 2048u32;
pub const DS_REPADD_DISABLE_NOTIFICATION: u32 = 64u32;
pub const DS_REPADD_DISABLE_PERIODIC: u32 = 128u32;
pub const DS_REPADD_INITIAL: u32 = 4u32;
pub const DS_REPADD_INTERSITE_MESSAGING: u32 = 16u32;
pub const DS_REPADD_NEVER_NOTIFY: u32 = 512u32;
pub const DS_REPADD_NONGC_RO_REPLICA: u32 = 16777216u32;
pub const DS_REPADD_PERIODIC: u32 = 8u32;
pub const DS_REPADD_SELECT_SECRETS: u32 = 4096u32;
pub const DS_REPADD_TWO_WAY: u32 = 1024u32;
pub const DS_REPADD_USE_COMPRESSION: u32 = 256u32;
pub const DS_REPADD_WRITEABLE: u32 = 2u32;
pub const DS_REPDEL_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPDEL_IGNORE_ERRORS: u32 = 8u32;
pub const DS_REPDEL_INTERSITE_MESSAGING: u32 = 4u32;
pub const DS_REPDEL_LOCAL_ONLY: u32 = 16u32;
pub const DS_REPDEL_NO_SOURCE: u32 = 32u32;
pub const DS_REPDEL_REF_OK: u32 = 64u32;
pub const DS_REPDEL_WRITEABLE: u32 = 2u32;
pub const DS_REPL_INFO_CURSORS_2_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(7i32);
pub const DS_REPL_INFO_CURSORS_3_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(8i32);
pub const DS_REPL_INFO_CURSORS_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(1i32);
pub const DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS: u32 = 1u32;
pub const DS_REPL_INFO_KCC_DSA_CONNECT_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(3i32);
pub const DS_REPL_INFO_KCC_DSA_LINK_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(4i32);
pub const DS_REPL_INFO_METADATA_2_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(10i32);
pub const DS_REPL_INFO_METADATA_2_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(9i32);
pub const DS_REPL_INFO_METADATA_EXT_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(11i32);
pub const DS_REPL_INFO_METADATA_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(6i32);
pub const DS_REPL_INFO_METADATA_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(2i32);
pub const DS_REPL_INFO_NEIGHBORS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(0i32);
pub const DS_REPL_INFO_PENDING_OPS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(5i32);
pub const DS_REPL_INFO_TYPE_MAX: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(12i32);
pub const DS_REPL_NBR_COMPRESS_CHANGES: u32 = 268435456u32;
pub const DS_REPL_NBR_DISABLE_SCHEDULED_SYNC: u32 = 134217728u32;
pub const DS_REPL_NBR_DO_SCHEDULED_SYNCS: u32 = 64u32;
pub const DS_REPL_NBR_FULL_SYNC_IN_PROGRESS: u32 = 65536u32;
pub const DS_REPL_NBR_FULL_SYNC_NEXT_PACKET: u32 = 131072u32;
pub const DS_REPL_NBR_GCSPN: u32 = 1048576u32;
pub const DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS: u32 = 67108864u32;
pub const DS_REPL_NBR_NEVER_SYNCED: u32 = 2097152u32;
pub const DS_REPL_NBR_NONGC_RO_REPLICA: u32 = 1024u32;
pub const DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS: u32 = 536870912u32;
pub const DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET: u32 = 1073741824u32;
pub const DS_REPL_NBR_PREEMPTED: u32 = 16777216u32;
pub const DS_REPL_NBR_RETURN_OBJECT_PARENTS: u32 = 2048u32;
pub const DS_REPL_NBR_SELECT_SECRETS: u32 = 4096u32;
pub const DS_REPL_NBR_SYNC_ON_STARTUP: u32 = 32u32;
pub const DS_REPL_NBR_TWO_WAY_SYNC: u32 = 512u32;
pub const DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT: u32 = 128u32;
pub const DS_REPL_NBR_WRITEABLE: u32 = 16u32;
pub const DS_REPL_OP_TYPE_ADD: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(1i32);
pub const DS_REPL_OP_TYPE_DELETE: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(2i32);
pub const DS_REPL_OP_TYPE_MODIFY: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(3i32);
pub const DS_REPL_OP_TYPE_SYNC: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(0i32);
pub const DS_REPL_OP_TYPE_UPDATE_REFS: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(4i32);
pub const DS_REPMOD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPMOD_UPDATE_ADDRESS: u32 = 2u32;
pub const DS_REPMOD_UPDATE_FLAGS: u32 = 1u32;
pub const DS_REPMOD_UPDATE_INSTANCE: u32 = 2u32;
pub const DS_REPMOD_UPDATE_RESULT: u32 = 8u32;
pub const DS_REPMOD_UPDATE_SCHEDULE: u32 = 4u32;
pub const DS_REPMOD_UPDATE_TRANSPORT: u32 = 16u32;
pub const DS_REPMOD_WRITEABLE: u32 = 2u32;
pub const DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE: u32 = 1u32;
pub const DS_REPSYNCALL_CROSS_SITE_BOUNDARIES: u32 = 64u32;
pub const DS_REPSYNCALL_DO_NOT_SYNC: u32 = 8u32;
pub const DS_REPSYNCALL_EVENT_ERROR: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(0i32);
pub const DS_REPSYNCALL_EVENT_FINISHED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(3i32);
pub const DS_REPSYNCALL_EVENT_SYNC_COMPLETED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(2i32);
pub const DS_REPSYNCALL_EVENT_SYNC_STARTED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(1i32);
pub const DS_REPSYNCALL_ID_SERVERS_BY_DN: u32 = 4u32;
pub const DS_REPSYNCALL_NO_OPTIONS: u32 = 0u32;
pub const DS_REPSYNCALL_PUSH_CHANGES_OUTWARD: u32 = 32u32;
pub const DS_REPSYNCALL_SERVER_UNREACHABLE: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(2i32);
pub const DS_REPSYNCALL_SKIP_INITIAL_CHECK: u32 = 16u32;
pub const DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY: u32 = 2u32;
pub const DS_REPSYNCALL_WIN32_ERROR_CONTACTING_SERVER: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(0i32);
pub const DS_REPSYNCALL_WIN32_ERROR_REPLICATING: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(1i32);
pub const DS_REPSYNC_ABANDONED: u32 = 32768u32;
pub const DS_REPSYNC_ADD_REFERENCE: u32 = 512u32;
pub const DS_REPSYNC_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPSYNC_ASYNCHRONOUS_REPLICA: u32 = 1048576u32;
pub const DS_REPSYNC_CRITICAL: u32 = 2097152u32;
pub const DS_REPSYNC_FORCE: u32 = 256u32;
pub const DS_REPSYNC_FULL: u32 = 32u32;
pub const DS_REPSYNC_FULL_IN_PROGRESS: u32 = 4194304u32;
pub const DS_REPSYNC_INITIAL: u32 = 8192u32;
pub const DS_REPSYNC_INITIAL_IN_PROGRESS: u32 = 65536u32;
pub const DS_REPSYNC_INTERSITE_MESSAGING: u32 = 8u32;
pub const DS_REPSYNC_NEVER_COMPLETED: u32 = 1024u32;
pub const DS_REPSYNC_NEVER_NOTIFY: u32 = 4096u32;
pub const DS_REPSYNC_NONGC_RO_REPLICA: u32 = 16777216u32;
pub const DS_REPSYNC_NOTIFICATION: u32 = 524288u32;
pub const DS_REPSYNC_NO_DISCARD: u32 = 128u32;
pub const DS_REPSYNC_PARTIAL_ATTRIBUTE_SET: u32 = 131072u32;
pub const DS_REPSYNC_PERIODIC: u32 = 4u32;
pub const DS_REPSYNC_PREEMPTED: u32 = 8388608u32;
pub const DS_REPSYNC_REQUEUE: u32 = 262144u32;
pub const DS_REPSYNC_SELECT_SECRETS: u32 = 32768u32;
pub const DS_REPSYNC_TWO_WAY: u32 = 2048u32;
pub const DS_REPSYNC_URGENT: u32 = 64u32;
pub const DS_REPSYNC_USE_COMPRESSION: u32 = 16384u32;
pub const DS_REPSYNC_WRITEABLE: u32 = 2u32;
pub const DS_REPUPD_ADD_REFERENCE: u32 = 4u32;
pub const DS_REPUPD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPUPD_DELETE_REFERENCE: u32 = 8u32;
pub const DS_REPUPD_REFERENCE_GCSPN: u32 = 16u32;
pub const DS_REPUPD_WRITEABLE: u32 = 2u32;
pub const DS_RETURN_DNS_NAME: u32 = 1073741824u32;
pub const DS_RETURN_FLAT_NAME: u32 = 2147483648u32;
pub const DS_ROLE_DOMAIN_OWNER: u32 = 1u32;
pub const DS_ROLE_INFRASTRUCTURE_OWNER: u32 = 4u32;
pub const DS_ROLE_PDC_OWNER: u32 = 2u32;
pub const DS_ROLE_RID_OWNER: u32 = 3u32;
pub const DS_ROLE_SCHEMA_OWNER: u32 = 0u32;
pub const DS_SCHEMA_GUID_ATTR: u32 = 1u32;
pub const DS_SCHEMA_GUID_ATTR_SET: u32 = 2u32;
pub const DS_SCHEMA_GUID_CLASS: u32 = 3u32;
pub const DS_SCHEMA_GUID_CONTROL_RIGHT: u32 = 4u32;
pub const DS_SCHEMA_GUID_NOT_FOUND: u32 = 0u32;
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: u32 = 2048u32;
pub const DS_SERVICE_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(10i32);
pub const DS_SID_OR_SID_HISTORY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(11i32);
pub const DS_SPN_ADD_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(0i32);
pub const DS_SPN_DELETE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(2i32);
pub const DS_SPN_DNS_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(0i32);
pub const DS_SPN_DN_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(1i32);
pub const DS_SPN_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(3i32);
pub const DS_SPN_NB_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(4i32);
pub const DS_SPN_NB_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(2i32);
pub const DS_SPN_REPLACE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(1i32);
pub const DS_SPN_SERVICE: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(5i32);
pub const DS_SYNCED_EVENT_NAME: ::windows_core::PCSTR = ::windows_core::s!("NTDSInitialSyncsCompleted");
pub const DS_SYNCED_EVENT_NAME_W: ::windows_core::PCWSTR = ::windows_core::w!("NTDSInitialSyncsCompleted");
pub const DS_TIMESERV_FLAG: u32 = 64u32;
pub const DS_TIMESERV_REQUIRED: u32 = 2048u32;
pub const DS_TRY_NEXTCLOSEST_SITE: u32 = 262144u32;
pub const DS_UNIQUE_ID_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(6i32);
pub const DS_UNKNOWN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(0i32);
pub const DS_USER_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(8i32);
pub const DS_WEB_SERVICE_REQUIRED: u32 = 1048576u32;
pub const DS_WRITABLE_FLAG: u32 = 256u32;
pub const DS_WRITABLE_REQUIRED: u32 = 4096u32;
pub const DS_WS_FLAG: u32 = 8192u32;
pub const DsRoleOperationActive: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(1i32);
pub const DsRoleOperationIdle: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(0i32);
pub const DsRoleOperationNeedReboot: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(2i32);
pub const DsRoleOperationState: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(3i32);
pub const DsRolePrimaryDomainInfoBasic: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(1i32);
pub const DsRoleServerBackup: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(2i32);
pub const DsRoleServerPrimary: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(1i32);
pub const DsRoleServerUnknown: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(0i32);
pub const DsRoleUpgradeStatus: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(2i32);
pub const DsRole_RoleBackupDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(4i32);
pub const DsRole_RoleMemberServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(3i32);
pub const DsRole_RoleMemberWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(1i32);
pub const DsRole_RolePrimaryDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(5i32);
pub const DsRole_RoleStandaloneServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(2i32);
pub const DsRole_RoleStandaloneWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(0i32);
pub const Email: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f92a857_478e_11d1_a3b4_00c04fb950dc);
pub const FACILITY_BACKUP: u32 = 2047u32;
pub const FACILITY_NTDSB: u32 = 2048u32;
pub const FACILITY_SYSTEM: u32 = 0u32;
pub const FLAG_DISABLABLE_OPTIONAL_FEATURE: u32 = 4u32;
pub const FLAG_DOMAIN_OPTIONAL_FEATURE: u32 = 2u32;
pub const FLAG_FOREST_OPTIONAL_FEATURE: u32 = 1u32;
pub const FLAG_SERVER_OPTIONAL_FEATURE: u32 = 8u32;
pub const FRSCONN_MAX_PRIORITY: u32 = 8u32;
pub const FRSCONN_PRIORITY_MASK: u32 = 1879048192u32;
pub const FaxNumber: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5062215_4681_11d1_a3b4_00c04fb950dc);
pub const GUID_COMPUTRS_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_COMPUTRS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_DELETED_OBJECTS_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DELETED_OBJECTS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_INFRASTRUCTURE_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_INFRASTRUCTURE_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_KEYS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("683A24E2E8164BD3AF86AC3C2CF3F981");
pub const GUID_LOSTANDFOUND_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_LOSTANDFOUND_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_MANAGED_SERVICE_ACCOUNTS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("1EB93889E40C45DF9F0C64D23BBB6237");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("f4be92a4c777485e878e9421d53087db");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("f4be92a4c777485e878e9421d53087db");
pub const GUID_NTDS_QUOTAS_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_NTDS_QUOTAS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_A: ::windows_core::PCSTR = ::windows_core::s!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_W: ::windows_core::PCWSTR = ::windows_core::w!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PROGRAM_DATA_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_PROGRAM_DATA_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_A: ::windows_core::PCSTR = ::windows_core::s!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_W: ::windows_core::PCWSTR = ::windows_core::w!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_SYSTEMS_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_SYSTEMS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_A: ::windows_core::PCSTR = ::windows_core::s!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_W: ::windows_core::PCWSTR = ::windows_core::w!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const Hold: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3ad3e13_4080_11d1_a3ac_00c04fb950dc);
pub const LargeInteger: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x927971f5_0939_11d1_8be1_00c04fd8d503);
pub const NTDSAPI_BIND_ALLOW_DELEGATION: u32 = 1u32;
pub const NTDSAPI_BIND_FIND_BINDING: u32 = 2u32;
pub const NTDSAPI_BIND_FORCE_KERBEROS: u32 = 4u32;
pub const NTDSCONN_KCC_GC_TOPOLOGY: u32 = 1u32;
pub const NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY: u32 = 32u32;
pub const NTDSCONN_KCC_INTERSITE_TOPOLOGY: u32 = 64u32;
pub const NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY: u32 = 4u32;
pub const NTDSCONN_KCC_NO_REASON: u32 = 0u32;
pub const NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY: u32 = 16u32;
pub const NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY: u32 = 512u32;
pub const NTDSCONN_KCC_RING_TOPOLOGY: u32 = 2u32;
pub const NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY: u32 = 128u32;
pub const NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY: u32 = 256u32;
pub const NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY: u32 = 8u32;
pub const NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION: u32 = 16u32;
pub const NTDSCONN_OPT_IGNORE_SCHEDULE_MASK: u32 = 2147483648u32;
pub const NTDSCONN_OPT_IS_GENERATED: u32 = 1u32;
pub const NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT: u32 = 4u32;
pub const NTDSCONN_OPT_RODC_TOPOLOGY: u32 = 64u32;
pub const NTDSCONN_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSCONN_OPT_USER_OWNED_SCHEDULE: u32 = 32u32;
pub const NTDSCONN_OPT_USE_NOTIFY: u32 = 8u32;
pub const NTDSDSA_OPT_BLOCK_RPC: u32 = 64u32;
pub const NTDSDSA_OPT_DISABLE_INBOUND_REPL: u32 = 2u32;
pub const NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE: u32 = 8u32;
pub const NTDSDSA_OPT_DISABLE_OUTBOUND_REPL: u32 = 4u32;
pub const NTDSDSA_OPT_DISABLE_SPN_REGISTRATION: u32 = 16u32;
pub const NTDSDSA_OPT_GENERATE_OWN_TOPO: u32 = 32u32;
pub const NTDSDSA_OPT_IS_GC: u32 = 1u32;
pub const NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY: u32 = 2u32;
pub const NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION: u32 = 128u32;
pub const NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR: u32 = 64u32;
pub const NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED: u32 = 1u32;
pub const NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED: u32 = 32u32;
pub const NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED: u32 = 16u32;
pub const NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED: u32 = 256u32;
pub const NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED: u32 = 1024u32;
pub const NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED: u32 = 512u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED: u32 = 2u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED: u32 = 8u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED: u32 = 4u32;
pub const NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED: u32 = 4096u32;
pub const NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES: u32 = 2048u32;
pub const NTDSSITECONN_OPT_DISABLE_COMPRESSION: u32 = 4u32;
pub const NTDSSITECONN_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSSITECONN_OPT_USE_NOTIFY: u32 = 1u32;
pub const NTDSSITELINK_OPT_DISABLE_COMPRESSION: u32 = 4u32;
pub const NTDSSITELINK_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSSITELINK_OPT_USE_NOTIFY: u32 = 1u32;
pub const NTDSTRANSPORT_OPT_BRIDGES_REQUIRED: u32 = 2u32;
pub const NTDSTRANSPORT_OPT_IGNORE_SCHEDULES: u32 = 1u32;
pub const NameTranslate: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x274fae1f_3626_11d1_a3a4_00c04fb950dc);
pub const NetAddress: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0b71247_4080_11d1_a3ac_00c04fb950dc);
pub const OQWF_DEFAULTFORM: u32 = 2u32;
pub const OQWF_HIDEMENUS: u32 = 1024u32;
pub const OQWF_HIDESEARCHUI: u32 = 2048u32;
pub const OQWF_ISSUEONOPEN: u32 = 64u32;
pub const OQWF_LOADQUERY: u32 = 8u32;
pub const OQWF_OKCANCEL: u32 = 1u32;
pub const OQWF_PARAMISPROPERTYBAG: u32 = 2147483648u32;
pub const OQWF_REMOVEFORMS: u32 = 32u32;
pub const OQWF_REMOVESCOPES: u32 = 16u32;
pub const OQWF_SAVEQUERYONOK: u32 = 512u32;
pub const OQWF_SHOWOPTIONAL: u32 = 128u32;
pub const OQWF_SINGLESELECT: u32 = 4u32;
pub const OctetList: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1241400f_4680_11d1_a3b4_00c04fb950dc);
pub const Path: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2538919_4080_11d1_a3ac_00c04fb950dc);
pub const Pathname: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x080d0d78_f421_11d0_a36e_00c04fb950dc);
pub const PostalAddress: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a75afcd_4680_11d1_a3b4_00c04fb950dc);
pub const PropertyEntry: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72d3edc2_a4c4_11d0_8533_00c04fd8d503);
pub const PropertyValue: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b9e38b0_a97c_11d0_8534_00c04fd8d503);
pub const QUERYFORM_CHANGESFORMLIST: u64 = 1u64;
pub const QUERYFORM_CHANGESOPTFORMLIST: u64 = 2u64;
pub const ReplicaPointer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5d1badf_4080_11d1_a3ac_00c04fb950dc);
pub const SCHEDULE_BANDWIDTH: u32 = 1u32;
pub const SCHEDULE_INTERVAL: u32 = 0u32;
pub const SCHEDULE_PRIORITY: u32 = 2u32;
pub const SecurityDescriptor: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb958f73c_9bdd_11d0_852c_00c04fd8d503);
pub const Timestamp: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2bed2eb_4080_11d1_a3ac_00c04fb950dc);
pub const TypedName: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb33143cb_4080_11d1_a3ac_00c04fb950dc);
pub const WM_ADSPROP_NOTIFY_APPLY: u32 = 2128u32;
pub const WM_ADSPROP_NOTIFY_CHANGE: u32 = 2127u32;
pub const WM_ADSPROP_NOTIFY_ERROR: u32 = 2134u32;
pub const WM_ADSPROP_NOTIFY_EXIT: u32 = 2131u32;
pub const WM_ADSPROP_NOTIFY_FOREGROUND: u32 = 2130u32;
pub const WM_ADSPROP_NOTIFY_PAGEHWND: u32 = 2126u32;
pub const WM_ADSPROP_NOTIFY_PAGEINIT: u32 = 2125u32;
pub const WM_ADSPROP_NOTIFY_SETFOCUS: u32 = 2129u32;
pub const WinNTSystemInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66182ec4_afd1_11d2_9cb9_0000f87a369e);
pub const hrAccessDenied: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522189i32);
pub const hrAfterInitialization: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522246i32);
pub const hrAlreadyInitialized: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523066i32);
pub const hrAlreadyOpen: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589627i32);
pub const hrAlreadyPrepared: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522489i32);
pub const hrBFInUse: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523894i32);
pub const hrBFNotSynchronous: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013265720i32);
pub const hrBFPageNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013265719i32);
pub const hrBackupDirectoryNotEmpty: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523592i32);
pub const hrBackupInProgress: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523591i32);
pub const hrBackupNotAllowedYet: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523573i32);
pub const hrBadBackupDatabaseSize: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523535i32);
pub const hrBadCheckpointSignature: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523564i32);
pub const hrBadColumnId: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522579i32);
pub const hrBadDbSignature: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523565i32);
pub const hrBadItagSequence: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522578i32);
pub const hrBadLogSignature: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523566i32);
pub const hrBadLogVersion: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523582i32);
pub const hrBufferTooSmall: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523058i32);
pub const hrBufferTruncated: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264914i32);
pub const hrCannotBeTagged: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522575i32);
pub const hrCannotRename: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522790i32);
pub const hrCheckpointCorrupt: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523563i32);
pub const hrCircularLogging: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589621i32);
pub const hrColumn2ndSysMaint: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522586i32);
pub const hrColumnCannotIndex: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522583i32);
pub const hrColumnDoesNotFit: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522593i32);
pub const hrColumnDuplicate: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522588i32);
pub const hrColumnInUse: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523050i32);
pub const hrColumnIndexed: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522591i32);
pub const hrColumnLong: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522595i32);
pub const hrColumnMaxTruncated: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264408i32);
pub const hrColumnNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522589i32);
pub const hrColumnNotUpdatable: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523048i32);
pub const hrColumnNull: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264916i32);
pub const hrColumnSetNull: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264852i32);
pub const hrColumnTooBig: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522590i32);
pub const hrCommunicationError: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589619i32);
pub const hrConsistentTimeMismatch: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523545i32);
pub const hrContainerNotEmpty: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523053i32);
pub const hrContentsExpired: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589615i32);
pub const hrCouldNotConnect: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589625i32);
pub const hrCreateIndexFailed: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264511i32);
pub const hrCurrencyStackOutOfMemory: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523026i32);
pub const hrDatabaseAttached: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264913i32);
pub const hrDatabaseCorrupted: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522890i32);
pub const hrDatabaseDuplicate: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522895i32);
pub const hrDatabaseInUse: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522894i32);
pub const hrDatabaseInconsistent: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523546i32);
pub const hrDatabaseInvalidName: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522892i32);
pub const hrDatabaseInvalidPages: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522891i32);
pub const hrDatabaseLocked: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522889i32);
pub const hrDatabaseNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522893i32);
pub const hrDeleteBackupFileFail: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523572i32);
pub const hrDensityInvalid: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522789i32);
pub const hrDiskFull: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522288i32);
pub const hrDiskIO: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523074i32);
pub const hrError: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589630i32);
pub const hrExistingLogFileHasBadSignature: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013265362i32);
pub const hrExistingLogFileIsNotContiguous: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013265361i32);
pub const hrFLDKeyTooBig: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013265520i32);
pub const hrFLDNullKey: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013265518i32);
pub const hrFLDTooManySegments: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523695i32);
pub const hrFeatureNotAvailable: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523095i32);
pub const hrFileAccessDenied: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523064i32);
pub const hrFileClose: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523994i32);
pub const hrFileNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522285i32);
pub const hrFileOpenReadOnly: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264107i32);
pub const hrFullBackupNotTaken: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589618i32);
pub const hrGivenLogFileHasBadSignature: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523541i32);
pub const hrGivenLogFileIsNotContiguous: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523540i32);
pub const hrIllegalOperation: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522784i32);
pub const hrInTransaction: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522988i32);
pub const hrIncrementalBackupDisabled: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589623i32);
pub const hrIndexCantBuild: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522695i32);
pub const hrIndexDuplicate: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522693i32);
pub const hrIndexHasClustered: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522688i32);
pub const hrIndexHasPrimary: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522694i32);
pub const hrIndexInUse: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523045i32);
pub const hrIndexInvalidDef: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522690i32);
pub const hrIndexMustStay: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522691i32);
pub const hrIndexNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522692i32);
pub const hrInvalidBackup: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523570i32);
pub const hrInvalidBackupSequence: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523575i32);
pub const hrInvalidBookmark: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523051i32);
pub const hrInvalidBufferSize: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523049i32);
pub const hrInvalidCodePage: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523033i32);
pub const hrInvalidColumnType: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522585i32);
pub const hrInvalidCountry: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523035i32);
pub const hrInvalidDatabase: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523068i32);
pub const hrInvalidDatabaseId: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523086i32);
pub const hrInvalidFilename: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523052i32);
pub const hrInvalidHandle: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589629i32);
pub const hrInvalidLanguageId: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523034i32);
pub const hrInvalidLogSequence: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523581i32);
pub const hrInvalidName: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523094i32);
pub const hrInvalidObject: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522780i32);
pub const hrInvalidOnSort: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522394i32);
pub const hrInvalidOperation: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522190i32);
pub const hrInvalidParam: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589631i32);
pub const hrInvalidParameter: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523093i32);
pub const hrInvalidPath: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523073i32);
pub const hrInvalidRecips: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589626i32);
pub const hrInvalidSesid: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522992i32);
pub const hrInvalidTableId: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522786i32);
pub const hrKeyChanged: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264302i32);
pub const hrKeyDuplicate: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522491i32);
pub const hrKeyIsMade: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522580i32);
pub const hrKeyNotMade: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522488i32);
pub const hrLogBufferTooSmall: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523579i32);
pub const hrLogCorrupted: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522244i32);
pub const hrLogDiskFull: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523567i32);
pub const hrLogFileCorrupt: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523595i32);
pub const hrLogFileNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589622i32);
pub const hrLogSequenceEnd: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523577i32);
pub const hrLogWriteFail: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523586i32);
pub const hrLoggingDisabled: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523580i32);
pub const hrMakeBackupDirectoryFail: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523571i32);
pub const hrMissingExpiryToken: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589617i32);
pub const hrMissingFullBackup: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523536i32);
pub const hrMissingLogFile: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523568i32);
pub const hrMissingPreviousLogFile: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523587i32);
pub const hrMissingRestoreLogFiles: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523539i32);
pub const hrNoBackup: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523576i32);
pub const hrNoBackupDirectory: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523593i32);
pub const hrNoCurrentIndex: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522581i32);
pub const hrNoCurrentRecord: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522493i32);
pub const hrNoFullRestore: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589620i32);
pub const hrNoIdleActivity: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264862i32);
pub const hrNoWriteLock: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264853i32);
pub const hrNone: ::windows_core::HRESULT = ::windows_core::HRESULT(0i32);
pub const hrNotInTransaction: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523042i32);
pub const hrNotInitialized: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523067i32);
pub const hrNullInvalid: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522592i32);
pub const hrNullKeyDisallowed: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523043i32);
pub const hrNyi: ::windows_core::HRESULT = ::windows_core::HRESULT(-1073741823i32);
pub const hrObjectDuplicate: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522782i32);
pub const hrObjectNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522791i32);
pub const hrOutOfBuffers: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523082i32);
pub const hrOutOfCursors: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523083i32);
pub const hrOutOfDatabaseSpace: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523084i32);
pub const hrOutOfFileHandles: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523076i32);
pub const hrOutOfMemory: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523085i32);
pub const hrOutOfSessions: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522995i32);
pub const hrOutOfThreads: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523993i32);
pub const hrPMRecDeleted: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523794i32);
pub const hrPatchFileMismatch: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523544i32);
pub const hrPermissionDenied: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522287i32);
pub const hrReadVerifyFailure: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523078i32);
pub const hrRecordClusteredChanged: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522492i32);
pub const hrRecordDeleted: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523079i32);
pub const hrRecordNotFound: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522495i32);
pub const hrRecordTooBig: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523070i32);
pub const hrRecoveredWithErrors: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523569i32);
pub const hrRemainingVersions: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013265599i32);
pub const hrRestoreInProgress: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589628i32);
pub const hrRestoreLogTooHigh: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523542i32);
pub const hrRestoreLogTooLow: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523543i32);
pub const hrRestoreMapExists: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589624i32);
pub const hrSeekNotEqual: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264881i32);
pub const hrSessionWriteConflict: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522989i32);
pub const hrTableDuplicate: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522793i32);
pub const hrTableEmpty: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264619i32);
pub const hrTableInUse: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522792i32);
pub const hrTableLocked: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522794i32);
pub const hrTableNotEmpty: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522788i32);
pub const hrTaggedNotNULL: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522582i32);
pub const hrTempFileOpenError: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522293i32);
pub const hrTermInProgress: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523096i32);
pub const hrTooManyActiveUsers: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523037i32);
pub const hrTooManyAttachedDatabases: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522291i32);
pub const hrTooManyColumns: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523056i32);
pub const hrTooManyIO: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523991i32);
pub const hrTooManyIndexes: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523081i32);
pub const hrTooManyKeys: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523080i32);
pub const hrTooManyOpenDatabases: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523069i32);
pub const hrTooManyOpenIndexes: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522686i32);
pub const hrTooManyOpenTables: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522785i32);
pub const hrTooManySorts: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522395i32);
pub const hrTransTooDeep: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522993i32);
pub const hrUnknownExpiryTokenFormat: ::windows_core::HRESULT = ::windows_core::HRESULT(-939589616i32);
pub const hrUpdateNotPrepared: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522487i32);
pub const hrVersionStoreOutOfMemory: ::windows_core::HRESULT = ::windows_core::HRESULT(-939523027i32);
pub const hrWriteConflict: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522994i32);
pub const hrerrDataHasChanged: ::windows_core::HRESULT = ::windows_core::HRESULT(-939522485i32);
pub const hrwrnDataHasChanged: ::windows_core::HRESULT = ::windows_core::HRESULT(-2013264310i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADSI_DIALECT_ENUM(pub i32);
impl ::core::marker::Copy for ADSI_DIALECT_ENUM {}
impl ::core::clone::Clone for ADSI_DIALECT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADSI_DIALECT_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADSI_DIALECT_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADSI_DIALECT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADSI_DIALECT_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADSTYPE(pub i32);
impl ::core::marker::Copy for ADSTYPE {}
impl ::core::clone::Clone for ADSTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADSTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADSTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_ACEFLAG_ENUM(pub i32);
impl ::core::marker::Copy for ADS_ACEFLAG_ENUM {}
impl ::core::clone::Clone for ADS_ACEFLAG_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_ACEFLAG_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_ACEFLAG_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_ACEFLAG_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_ACEFLAG_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_ACETYPE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_ACETYPE_ENUM {}
impl ::core::clone::Clone for ADS_ACETYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_ACETYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_ACETYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_ACETYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_ACETYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_AUTHENTICATION_ENUM(pub u32);
impl ::core::marker::Copy for ADS_AUTHENTICATION_ENUM {}
impl ::core::clone::Clone for ADS_AUTHENTICATION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_AUTHENTICATION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_AUTHENTICATION_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_AUTHENTICATION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_AUTHENTICATION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_CHASE_REFERRALS_ENUM(pub i32);
impl ::core::marker::Copy for ADS_CHASE_REFERRALS_ENUM {}
impl ::core::clone::Clone for ADS_CHASE_REFERRALS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_CHASE_REFERRALS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_CHASE_REFERRALS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_CHASE_REFERRALS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_CHASE_REFERRALS_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_DEREFENUM(pub i32);
impl ::core::marker::Copy for ADS_DEREFENUM {}
impl ::core::clone::Clone for ADS_DEREFENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_DEREFENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_DEREFENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_DEREFENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_DEREFENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_DISPLAY_ENUM(pub i32);
impl ::core::marker::Copy for ADS_DISPLAY_ENUM {}
impl ::core::clone::Clone for ADS_DISPLAY_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_DISPLAY_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_DISPLAY_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_DISPLAY_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_DISPLAY_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_ESCAPE_MODE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_ESCAPE_MODE_ENUM {}
impl ::core::clone::Clone for ADS_ESCAPE_MODE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_ESCAPE_MODE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_ESCAPE_MODE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_ESCAPE_MODE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_ESCAPE_MODE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_FLAGTYPE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_FLAGTYPE_ENUM {}
impl ::core::clone::Clone for ADS_FLAGTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_FLAGTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_FLAGTYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_FLAGTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_FLAGTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_FORMAT_ENUM(pub i32);
impl ::core::marker::Copy for ADS_FORMAT_ENUM {}
impl ::core::clone::Clone for ADS_FORMAT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_FORMAT_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_FORMAT_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_FORMAT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_FORMAT_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_GROUP_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_GROUP_TYPE_ENUM {}
impl ::core::clone::Clone for ADS_GROUP_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_GROUP_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_GROUP_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_GROUP_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_GROUP_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_NAME_INITTYPE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_NAME_INITTYPE_ENUM {}
impl ::core::clone::Clone for ADS_NAME_INITTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_NAME_INITTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_NAME_INITTYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_NAME_INITTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_NAME_INITTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_NAME_TYPE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_NAME_TYPE_ENUM {}
impl ::core::clone::Clone for ADS_NAME_TYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_NAME_TYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_NAME_TYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_NAME_TYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_NAME_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_OPTION_ENUM(pub i32);
impl ::core::marker::Copy for ADS_OPTION_ENUM {}
impl ::core::clone::Clone for ADS_OPTION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_OPTION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_OPTION_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_OPTION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_OPTION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PASSWORD_ENCODING_ENUM(pub i32);
impl ::core::marker::Copy for ADS_PASSWORD_ENCODING_ENUM {}
impl ::core::clone::Clone for ADS_PASSWORD_ENCODING_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_PASSWORD_ENCODING_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_PASSWORD_ENCODING_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_PASSWORD_ENCODING_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PASSWORD_ENCODING_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PATHTYPE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_PATHTYPE_ENUM {}
impl ::core::clone::Clone for ADS_PATHTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_PATHTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_PATHTYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_PATHTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PATHTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PREFERENCES_ENUM(pub i32);
impl ::core::marker::Copy for ADS_PREFERENCES_ENUM {}
impl ::core::clone::Clone for ADS_PREFERENCES_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_PREFERENCES_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_PREFERENCES_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_PREFERENCES_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PREFERENCES_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_PROPERTY_OPERATION_ENUM(pub i32);
impl ::core::marker::Copy for ADS_PROPERTY_OPERATION_ENUM {}
impl ::core::clone::Clone for ADS_PROPERTY_OPERATION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_PROPERTY_OPERATION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_PROPERTY_OPERATION_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_PROPERTY_OPERATION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_PROPERTY_OPERATION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_RIGHTS_ENUM(pub i32);
impl ::core::marker::Copy for ADS_RIGHTS_ENUM {}
impl ::core::clone::Clone for ADS_RIGHTS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_RIGHTS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_RIGHTS_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_RIGHTS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_RIGHTS_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SCOPEENUM(pub i32);
impl ::core::marker::Copy for ADS_SCOPEENUM {}
impl ::core::clone::Clone for ADS_SCOPEENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SCOPEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SCOPEENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SCOPEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SCOPEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SD_CONTROL_ENUM(pub i32);
impl ::core::marker::Copy for ADS_SD_CONTROL_ENUM {}
impl ::core::clone::Clone for ADS_SD_CONTROL_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SD_CONTROL_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SD_CONTROL_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SD_CONTROL_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SD_CONTROL_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SD_FORMAT_ENUM(pub i32);
impl ::core::marker::Copy for ADS_SD_FORMAT_ENUM {}
impl ::core::clone::Clone for ADS_SD_FORMAT_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SD_FORMAT_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SD_FORMAT_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SD_FORMAT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SD_FORMAT_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SD_REVISION_ENUM(pub i32);
impl ::core::marker::Copy for ADS_SD_REVISION_ENUM {}
impl ::core::clone::Clone for ADS_SD_REVISION_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SD_REVISION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SD_REVISION_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SD_REVISION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SD_REVISION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SEARCHPREF_ENUM(pub i32);
impl ::core::marker::Copy for ADS_SEARCHPREF_ENUM {}
impl ::core::clone::Clone for ADS_SEARCHPREF_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SEARCHPREF_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SEARCHPREF_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SEARCHPREF_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SEARCHPREF_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SECURITY_INFO_ENUM(pub i32);
impl ::core::marker::Copy for ADS_SECURITY_INFO_ENUM {}
impl ::core::clone::Clone for ADS_SECURITY_INFO_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SECURITY_INFO_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SECURITY_INFO_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SECURITY_INFO_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SECURITY_INFO_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SETTYPE_ENUM(pub i32);
impl ::core::marker::Copy for ADS_SETTYPE_ENUM {}
impl ::core::clone::Clone for ADS_SETTYPE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SETTYPE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SETTYPE_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SETTYPE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SETTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_STATUSENUM(pub i32);
impl ::core::marker::Copy for ADS_STATUSENUM {}
impl ::core::clone::Clone for ADS_STATUSENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_STATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_STATUSENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_STATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_STATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SYSTEMFLAG_ENUM(pub i32);
impl ::core::marker::Copy for ADS_SYSTEMFLAG_ENUM {}
impl ::core::clone::Clone for ADS_SYSTEMFLAG_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_SYSTEMFLAG_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_SYSTEMFLAG_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_SYSTEMFLAG_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SYSTEMFLAG_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_USER_FLAG_ENUM(pub i32);
impl ::core::marker::Copy for ADS_USER_FLAG_ENUM {}
impl ::core::clone::Clone for ADS_USER_FLAG_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADS_USER_FLAG_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADS_USER_FLAG_ENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADS_USER_FLAG_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_USER_FLAG_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_MACHINE_ROLE(pub i32);
impl ::core::marker::Copy for DSROLE_MACHINE_ROLE {}
impl ::core::clone::Clone for DSROLE_MACHINE_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSROLE_MACHINE_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DSROLE_MACHINE_ROLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DSROLE_MACHINE_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_MACHINE_ROLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_OPERATION_STATE(pub i32);
impl ::core::marker::Copy for DSROLE_OPERATION_STATE {}
impl ::core::clone::Clone for DSROLE_OPERATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSROLE_OPERATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DSROLE_OPERATION_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DSROLE_OPERATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_OPERATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(pub i32);
impl ::core::marker::Copy for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {}
impl ::core::clone::Clone for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_PRIMARY_DOMAIN_INFO_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSROLE_SERVER_STATE(pub i32);
impl ::core::marker::Copy for DSROLE_SERVER_STATE {}
impl ::core::clone::Clone for DSROLE_SERVER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSROLE_SERVER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DSROLE_SERVER_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DSROLE_SERVER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSROLE_SERVER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_KCC_TASKID(pub i32);
impl ::core::marker::Copy for DS_KCC_TASKID {}
impl ::core::clone::Clone for DS_KCC_TASKID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_KCC_TASKID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_KCC_TASKID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_KCC_TASKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_KCC_TASKID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_MANGLE_FOR(pub i32);
impl ::core::marker::Copy for DS_MANGLE_FOR {}
impl ::core::clone::Clone for DS_MANGLE_FOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_MANGLE_FOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_MANGLE_FOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_MANGLE_FOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_MANGLE_FOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_NAME_ERROR(pub i32);
impl ::core::marker::Copy for DS_NAME_ERROR {}
impl ::core::clone::Clone for DS_NAME_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_NAME_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_NAME_ERROR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_NAME_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_NAME_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_NAME_FLAGS(pub i32);
impl ::core::marker::Copy for DS_NAME_FLAGS {}
impl ::core::clone::Clone for DS_NAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_NAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_NAME_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_NAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_NAME_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_NAME_FORMAT(pub i32);
impl ::core::marker::Copy for DS_NAME_FORMAT {}
impl ::core::clone::Clone for DS_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_NAME_FORMAT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_NAME_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPL_INFO_TYPE(pub i32);
impl ::core::marker::Copy for DS_REPL_INFO_TYPE {}
impl ::core::clone::Clone for DS_REPL_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_REPL_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_REPL_INFO_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_REPL_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPL_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPL_OP_TYPE(pub i32);
impl ::core::marker::Copy for DS_REPL_OP_TYPE {}
impl ::core::clone::Clone for DS_REPL_OP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_REPL_OP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_REPL_OP_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_REPL_OP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPL_OP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPSYNCALL_ERROR(pub i32);
impl ::core::marker::Copy for DS_REPSYNCALL_ERROR {}
impl ::core::clone::Clone for DS_REPSYNCALL_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_REPSYNCALL_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_ERROR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_REPSYNCALL_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPSYNCALL_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_REPSYNCALL_EVENT(pub i32);
impl ::core::marker::Copy for DS_REPSYNCALL_EVENT {}
impl ::core::clone::Clone for DS_REPSYNCALL_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_REPSYNCALL_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_REPSYNCALL_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_REPSYNCALL_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_SPN_NAME_TYPE(pub i32);
impl ::core::marker::Copy for DS_SPN_NAME_TYPE {}
impl ::core::clone::Clone for DS_SPN_NAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_SPN_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_SPN_NAME_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_SPN_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_SPN_NAME_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DS_SPN_WRITE_OP(pub i32);
impl ::core::marker::Copy for DS_SPN_WRITE_OP {}
impl ::core::clone::Clone for DS_SPN_WRITE_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DS_SPN_WRITE_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DS_SPN_WRITE_OP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DS_SPN_WRITE_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DS_SPN_WRITE_OP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADSPROPERROR {
    pub hwndPage: super::super::Foundation::HWND,
    pub pszPageTitle: ::windows_core::PWSTR,
    pub pszObjPath: ::windows_core::PWSTR,
    pub pszObjClass: ::windows_core::PWSTR,
    pub hr: ::windows_core::HRESULT,
    pub pszError: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADSPROPERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSPROPERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADSPROPERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADSPROPERROR").field("hwndPage", &self.hwndPage).field("pszPageTitle", &self.pszPageTitle).field("pszObjPath", &self.pszObjPath).field("pszObjClass", &self.pszObjClass).field("hr", &self.hr).field("pszError", &self.pszError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADSPROPERROR {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADSPROPERROR {
    fn eq(&self, other: &Self) -> bool {
        self.hwndPage == other.hwndPage && self.pszPageTitle == other.pszPageTitle && self.pszObjPath == other.pszObjPath && self.pszObjClass == other.pszObjClass && self.hr == other.hr && self.pszError == other.pszError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADSPROPERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADSPROPERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADSPROPINITPARAMS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hr: ::windows_core::HRESULT,
    pub pDsObj: ::std::mem::ManuallyDrop<::core::option::Option<IDirectoryObject>>,
    pub pwzCN: ::windows_core::PWSTR,
    pub pWritableAttrs: *mut ADS_ATTR_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSPROPINITPARAMS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADSPROPINITPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADSPROPINITPARAMS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("hr", &self.hr).field("pDsObj", &self.pDsObj).field("pwzCN", &self.pwzCN).field("pWritableAttrs", &self.pWritableAttrs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADSPROPINITPARAMS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADSPROPINITPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.hr == other.hr && self.pDsObj == other.pDsObj && self.pwzCN == other.pwzCN && self.pWritableAttrs == other.pWritableAttrs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADSPROPINITPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADSPROPINITPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADSVALUE {
    pub dwType: ADSTYPE,
    pub Anonymous: ADSVALUE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADSVALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADSVALUE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADSVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union ADSVALUE_0 {
    pub DNString: *mut u16,
    pub CaseExactString: *mut u16,
    pub CaseIgnoreString: *mut u16,
    pub PrintableString: *mut u16,
    pub NumericString: *mut u16,
    pub Boolean: u32,
    pub Integer: u32,
    pub OctetString: ADS_OCTET_STRING,
    pub UTCTime: super::super::Foundation::SYSTEMTIME,
    pub LargeInteger: i64,
    pub ClassName: *mut u16,
    pub ProviderSpecific: ADS_PROV_SPECIFIC,
    pub pCaseIgnoreList: *mut ADS_CASEIGNORE_LIST,
    pub pOctetList: *mut ADS_OCTET_LIST,
    pub pPath: *mut ADS_PATH,
    pub pPostalAddress: *mut ADS_POSTALADDRESS,
    pub Timestamp: ADS_TIMESTAMP,
    pub BackLink: ADS_BACKLINK,
    pub pTypedName: *mut ADS_TYPEDNAME,
    pub Hold: ADS_HOLD,
    pub pNetAddress: *mut ADS_NETADDRESS,
    pub pReplicaPointer: *mut ADS_REPLICAPOINTER,
    pub pFaxNumber: *mut ADS_FAXNUMBER,
    pub Email: ADS_EMAIL,
    pub SecurityDescriptor: ADS_NT_SECURITY_DESCRIPTOR,
    pub pDNWithBinary: *mut ADS_DN_WITH_BINARY,
    pub pDNWithString: *mut ADS_DN_WITH_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADSVALUE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADSVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADSVALUE_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADSVALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_ATTR_DEF {
    pub pszAttrName: ::windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub dwMinRange: u32,
    pub dwMaxRange: u32,
    pub fMultiValued: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_ATTR_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_ATTR_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_ATTR_DEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_ATTR_DEF").field("pszAttrName", &self.pszAttrName).field("dwADsType", &self.dwADsType).field("dwMinRange", &self.dwMinRange).field("dwMaxRange", &self.dwMaxRange).field("fMultiValued", &self.fMultiValued).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADS_ATTR_DEF {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_ATTR_DEF {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrName == other.pszAttrName && self.dwADsType == other.dwADsType && self.dwMinRange == other.dwMinRange && self.dwMaxRange == other.dwMaxRange && self.fMultiValued == other.fMultiValued
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_ATTR_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_ATTR_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_ATTR_INFO {
    pub pszAttrName: ::windows_core::PWSTR,
    pub dwControlCode: u32,
    pub dwADsType: ADSTYPE,
    pub pADsValues: *mut ADSVALUE,
    pub dwNumValues: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_ATTR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_ATTR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_ATTR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_ATTR_INFO").field("pszAttrName", &self.pszAttrName).field("dwControlCode", &self.dwControlCode).field("dwADsType", &self.dwADsType).field("pADsValues", &self.pADsValues).field("dwNumValues", &self.dwNumValues).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADS_ATTR_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_ATTR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrName == other.pszAttrName && self.dwControlCode == other.dwControlCode && self.dwADsType == other.dwADsType && self.pADsValues == other.pADsValues && self.dwNumValues == other.dwNumValues
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_ATTR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_ATTR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_BACKLINK {
    pub RemoteID: u32,
    pub ObjectName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ADS_BACKLINK {}
impl ::core::clone::Clone for ADS_BACKLINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_BACKLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_BACKLINK").field("RemoteID", &self.RemoteID).field("ObjectName", &self.ObjectName).finish()
    }
}
impl ::windows_core::TypeKind for ADS_BACKLINK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_BACKLINK {
    fn eq(&self, other: &Self) -> bool {
        self.RemoteID == other.RemoteID && self.ObjectName == other.ObjectName
    }
}
impl ::core::cmp::Eq for ADS_BACKLINK {}
impl ::core::default::Default for ADS_BACKLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_CASEIGNORE_LIST {
    pub Next: *mut ADS_CASEIGNORE_LIST,
    pub String: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ADS_CASEIGNORE_LIST {}
impl ::core::clone::Clone for ADS_CASEIGNORE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_CASEIGNORE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_CASEIGNORE_LIST").field("Next", &self.Next).field("String", &self.String).finish()
    }
}
impl ::windows_core::TypeKind for ADS_CASEIGNORE_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_CASEIGNORE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.String == other.String
    }
}
impl ::core::cmp::Eq for ADS_CASEIGNORE_LIST {}
impl ::core::default::Default for ADS_CASEIGNORE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_CLASS_DEF {
    pub pszClassName: ::windows_core::PWSTR,
    pub dwMandatoryAttrs: u32,
    pub ppszMandatoryAttrs: *mut ::windows_core::PWSTR,
    pub optionalAttrs: u32,
    pub ppszOptionalAttrs: *mut *mut ::windows_core::PWSTR,
    pub dwNamingAttrs: u32,
    pub ppszNamingAttrs: *mut *mut ::windows_core::PWSTR,
    pub dwSuperClasses: u32,
    pub ppszSuperClasses: *mut *mut ::windows_core::PWSTR,
    pub fIsContainer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_CLASS_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_CLASS_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_CLASS_DEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_CLASS_DEF")
            .field("pszClassName", &self.pszClassName)
            .field("dwMandatoryAttrs", &self.dwMandatoryAttrs)
            .field("ppszMandatoryAttrs", &self.ppszMandatoryAttrs)
            .field("optionalAttrs", &self.optionalAttrs)
            .field("ppszOptionalAttrs", &self.ppszOptionalAttrs)
            .field("dwNamingAttrs", &self.dwNamingAttrs)
            .field("ppszNamingAttrs", &self.ppszNamingAttrs)
            .field("dwSuperClasses", &self.dwSuperClasses)
            .field("ppszSuperClasses", &self.ppszSuperClasses)
            .field("fIsContainer", &self.fIsContainer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADS_CLASS_DEF {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_CLASS_DEF {
    fn eq(&self, other: &Self) -> bool {
        self.pszClassName == other.pszClassName && self.dwMandatoryAttrs == other.dwMandatoryAttrs && self.ppszMandatoryAttrs == other.ppszMandatoryAttrs && self.optionalAttrs == other.optionalAttrs && self.ppszOptionalAttrs == other.ppszOptionalAttrs && self.dwNamingAttrs == other.dwNamingAttrs && self.ppszNamingAttrs == other.ppszNamingAttrs && self.dwSuperClasses == other.dwSuperClasses && self.ppszSuperClasses == other.ppszSuperClasses && self.fIsContainer == other.fIsContainer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_CLASS_DEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_CLASS_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_DN_WITH_BINARY {
    pub dwLength: u32,
    pub lpBinaryValue: *mut u8,
    pub pszDNString: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ADS_DN_WITH_BINARY {}
impl ::core::clone::Clone for ADS_DN_WITH_BINARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_DN_WITH_BINARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_DN_WITH_BINARY").field("dwLength", &self.dwLength).field("lpBinaryValue", &self.lpBinaryValue).field("pszDNString", &self.pszDNString).finish()
    }
}
impl ::windows_core::TypeKind for ADS_DN_WITH_BINARY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_DN_WITH_BINARY {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpBinaryValue == other.lpBinaryValue && self.pszDNString == other.pszDNString
    }
}
impl ::core::cmp::Eq for ADS_DN_WITH_BINARY {}
impl ::core::default::Default for ADS_DN_WITH_BINARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_DN_WITH_STRING {
    pub pszStringValue: ::windows_core::PWSTR,
    pub pszDNString: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ADS_DN_WITH_STRING {}
impl ::core::clone::Clone for ADS_DN_WITH_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_DN_WITH_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_DN_WITH_STRING").field("pszStringValue", &self.pszStringValue).field("pszDNString", &self.pszDNString).finish()
    }
}
impl ::windows_core::TypeKind for ADS_DN_WITH_STRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_DN_WITH_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.pszStringValue == other.pszStringValue && self.pszDNString == other.pszDNString
    }
}
impl ::core::cmp::Eq for ADS_DN_WITH_STRING {}
impl ::core::default::Default for ADS_DN_WITH_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_EMAIL {
    pub Address: ::windows_core::PWSTR,
    pub Type: u32,
}
impl ::core::marker::Copy for ADS_EMAIL {}
impl ::core::clone::Clone for ADS_EMAIL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_EMAIL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_EMAIL").field("Address", &self.Address).field("Type", &self.Type).finish()
    }
}
impl ::windows_core::TypeKind for ADS_EMAIL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_EMAIL {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for ADS_EMAIL {}
impl ::core::default::Default for ADS_EMAIL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_FAXNUMBER {
    pub TelephoneNumber: ::windows_core::PWSTR,
    pub NumberOfBits: u32,
    pub Parameters: *mut u8,
}
impl ::core::marker::Copy for ADS_FAXNUMBER {}
impl ::core::clone::Clone for ADS_FAXNUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_FAXNUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_FAXNUMBER").field("TelephoneNumber", &self.TelephoneNumber).field("NumberOfBits", &self.NumberOfBits).field("Parameters", &self.Parameters).finish()
    }
}
impl ::windows_core::TypeKind for ADS_FAXNUMBER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_FAXNUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.TelephoneNumber == other.TelephoneNumber && self.NumberOfBits == other.NumberOfBits && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for ADS_FAXNUMBER {}
impl ::core::default::Default for ADS_FAXNUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_HOLD {
    pub ObjectName: ::windows_core::PWSTR,
    pub Amount: u32,
}
impl ::core::marker::Copy for ADS_HOLD {}
impl ::core::clone::Clone for ADS_HOLD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_HOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_HOLD").field("ObjectName", &self.ObjectName).field("Amount", &self.Amount).finish()
    }
}
impl ::windows_core::TypeKind for ADS_HOLD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_HOLD {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectName == other.ObjectName && self.Amount == other.Amount
    }
}
impl ::core::cmp::Eq for ADS_HOLD {}
impl ::core::default::Default for ADS_HOLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_NETADDRESS {
    pub AddressType: u32,
    pub AddressLength: u32,
    pub Address: *mut u8,
}
impl ::core::marker::Copy for ADS_NETADDRESS {}
impl ::core::clone::Clone for ADS_NETADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_NETADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_NETADDRESS").field("AddressType", &self.AddressType).field("AddressLength", &self.AddressLength).field("Address", &self.Address).finish()
    }
}
impl ::windows_core::TypeKind for ADS_NETADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_NETADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType && self.AddressLength == other.AddressLength && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for ADS_NETADDRESS {}
impl ::core::default::Default for ADS_NETADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_NT_SECURITY_DESCRIPTOR {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for ADS_NT_SECURITY_DESCRIPTOR {}
impl ::core::clone::Clone for ADS_NT_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_NT_SECURITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_NT_SECURITY_DESCRIPTOR").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::windows_core::TypeKind for ADS_NT_SECURITY_DESCRIPTOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_NT_SECURITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for ADS_NT_SECURITY_DESCRIPTOR {}
impl ::core::default::Default for ADS_NT_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_OBJECT_INFO {
    pub pszRDN: ::windows_core::PWSTR,
    pub pszObjectDN: ::windows_core::PWSTR,
    pub pszParentDN: ::windows_core::PWSTR,
    pub pszSchemaDN: ::windows_core::PWSTR,
    pub pszClassName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ADS_OBJECT_INFO {}
impl ::core::clone::Clone for ADS_OBJECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_OBJECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_OBJECT_INFO").field("pszRDN", &self.pszRDN).field("pszObjectDN", &self.pszObjectDN).field("pszParentDN", &self.pszParentDN).field("pszSchemaDN", &self.pszSchemaDN).field("pszClassName", &self.pszClassName).finish()
    }
}
impl ::windows_core::TypeKind for ADS_OBJECT_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszRDN == other.pszRDN && self.pszObjectDN == other.pszObjectDN && self.pszParentDN == other.pszParentDN && self.pszSchemaDN == other.pszSchemaDN && self.pszClassName == other.pszClassName
    }
}
impl ::core::cmp::Eq for ADS_OBJECT_INFO {}
impl ::core::default::Default for ADS_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_OCTET_LIST {
    pub Next: *mut ADS_OCTET_LIST,
    pub Length: u32,
    pub Data: *mut u8,
}
impl ::core::marker::Copy for ADS_OCTET_LIST {}
impl ::core::clone::Clone for ADS_OCTET_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_OCTET_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_OCTET_LIST").field("Next", &self.Next).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::windows_core::TypeKind for ADS_OCTET_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_OCTET_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for ADS_OCTET_LIST {}
impl ::core::default::Default for ADS_OCTET_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for ADS_OCTET_STRING {}
impl ::core::clone::Clone for ADS_OCTET_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_OCTET_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_OCTET_STRING").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::windows_core::TypeKind for ADS_OCTET_STRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_OCTET_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for ADS_OCTET_STRING {}
impl ::core::default::Default for ADS_OCTET_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_PATH {
    pub Type: u32,
    pub VolumeName: ::windows_core::PWSTR,
    pub Path: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ADS_PATH {}
impl ::core::clone::Clone for ADS_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_PATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_PATH").field("Type", &self.Type).field("VolumeName", &self.VolumeName).field("Path", &self.Path).finish()
    }
}
impl ::windows_core::TypeKind for ADS_PATH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_PATH {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.VolumeName == other.VolumeName && self.Path == other.Path
    }
}
impl ::core::cmp::Eq for ADS_PATH {}
impl ::core::default::Default for ADS_PATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_POSTALADDRESS {
    pub PostalAddress: [::windows_core::PWSTR; 6],
}
impl ::core::marker::Copy for ADS_POSTALADDRESS {}
impl ::core::clone::Clone for ADS_POSTALADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_POSTALADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_POSTALADDRESS").field("PostalAddress", &self.PostalAddress).finish()
    }
}
impl ::windows_core::TypeKind for ADS_POSTALADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_POSTALADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.PostalAddress == other.PostalAddress
    }
}
impl ::core::cmp::Eq for ADS_POSTALADDRESS {}
impl ::core::default::Default for ADS_POSTALADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_PROV_SPECIFIC {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for ADS_PROV_SPECIFIC {}
impl ::core::clone::Clone for ADS_PROV_SPECIFIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_PROV_SPECIFIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_PROV_SPECIFIC").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::windows_core::TypeKind for ADS_PROV_SPECIFIC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_PROV_SPECIFIC {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for ADS_PROV_SPECIFIC {}
impl ::core::default::Default for ADS_PROV_SPECIFIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_REPLICAPOINTER {
    pub ServerName: ::windows_core::PWSTR,
    pub ReplicaType: u32,
    pub ReplicaNumber: u32,
    pub Count: u32,
    pub ReplicaAddressHints: *mut ADS_NETADDRESS,
}
impl ::core::marker::Copy for ADS_REPLICAPOINTER {}
impl ::core::clone::Clone for ADS_REPLICAPOINTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_REPLICAPOINTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_REPLICAPOINTER").field("ServerName", &self.ServerName).field("ReplicaType", &self.ReplicaType).field("ReplicaNumber", &self.ReplicaNumber).field("Count", &self.Count).field("ReplicaAddressHints", &self.ReplicaAddressHints).finish()
    }
}
impl ::windows_core::TypeKind for ADS_REPLICAPOINTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_REPLICAPOINTER {
    fn eq(&self, other: &Self) -> bool {
        self.ServerName == other.ServerName && self.ReplicaType == other.ReplicaType && self.ReplicaNumber == other.ReplicaNumber && self.Count == other.Count && self.ReplicaAddressHints == other.ReplicaAddressHints
    }
}
impl ::core::cmp::Eq for ADS_REPLICAPOINTER {}
impl ::core::default::Default for ADS_REPLICAPOINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_SEARCHPREF_INFO {
    pub dwSearchPref: ADS_SEARCHPREF_ENUM,
    pub vValue: ADSVALUE,
    pub dwStatus: ADS_STATUSENUM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_SEARCHPREF_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_SEARCHPREF_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADS_SEARCHPREF_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_SEARCHPREF_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_SEARCH_COLUMN {
    pub pszAttrName: ::windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub pADsValues: *mut ADSVALUE,
    pub dwNumValues: u32,
    pub hReserved: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_SEARCH_COLUMN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_SEARCH_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_SEARCH_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_SEARCH_COLUMN").field("pszAttrName", &self.pszAttrName).field("dwADsType", &self.dwADsType).field("pADsValues", &self.pADsValues).field("dwNumValues", &self.dwNumValues).field("hReserved", &self.hReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADS_SEARCH_COLUMN {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_SEARCH_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrName == other.pszAttrName && self.dwADsType == other.dwADsType && self.pADsValues == other.pADsValues && self.dwNumValues == other.dwNumValues && self.hReserved == other.hReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_SEARCH_COLUMN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_SEARCH_COLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADS_SEARCH_HANDLE(pub isize);
impl ADS_SEARCH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for ADS_SEARCH_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for ADS_SEARCH_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for ADS_SEARCH_HANDLE {}
impl ::core::fmt::Debug for ADS_SEARCH_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADS_SEARCH_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for ADS_SEARCH_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct ADS_SORTKEY {
    pub pszAttrType: ::windows_core::PWSTR,
    pub pszReserved: ::windows_core::PWSTR,
    pub fReverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ADS_SORTKEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ADS_SORTKEY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADS_SORTKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_SORTKEY").field("pszAttrType", &self.pszAttrType).field("pszReserved", &self.pszReserved).field("fReverseorder", &self.fReverseorder).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ADS_SORTKEY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADS_SORTKEY {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttrType == other.pszAttrType && self.pszReserved == other.pszReserved && self.fReverseorder == other.fReverseorder
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADS_SORTKEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADS_SORTKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_TIMESTAMP {
    pub WholeSeconds: u32,
    pub EventID: u32,
}
impl ::core::marker::Copy for ADS_TIMESTAMP {}
impl ::core::clone::Clone for ADS_TIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_TIMESTAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_TIMESTAMP").field("WholeSeconds", &self.WholeSeconds).field("EventID", &self.EventID).finish()
    }
}
impl ::windows_core::TypeKind for ADS_TIMESTAMP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_TIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        self.WholeSeconds == other.WholeSeconds && self.EventID == other.EventID
    }
}
impl ::core::cmp::Eq for ADS_TIMESTAMP {}
impl ::core::default::Default for ADS_TIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_TYPEDNAME {
    pub ObjectName: ::windows_core::PWSTR,
    pub Level: u32,
    pub Interval: u32,
}
impl ::core::marker::Copy for ADS_TYPEDNAME {}
impl ::core::clone::Clone for ADS_TYPEDNAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_TYPEDNAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_TYPEDNAME").field("ObjectName", &self.ObjectName).field("Level", &self.Level).field("Interval", &self.Interval).finish()
    }
}
impl ::windows_core::TypeKind for ADS_TYPEDNAME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_TYPEDNAME {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectName == other.ObjectName && self.Level == other.Level && self.Interval == other.Interval
    }
}
impl ::core::cmp::Eq for ADS_TYPEDNAME {}
impl ::core::default::Default for ADS_TYPEDNAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ADS_VLV {
    pub dwBeforeCount: u32,
    pub dwAfterCount: u32,
    pub dwOffset: u32,
    pub dwContentCount: u32,
    pub pszTarget: ::windows_core::PWSTR,
    pub dwContextIDLength: u32,
    pub lpContextID: *mut u8,
}
impl ::core::marker::Copy for ADS_VLV {}
impl ::core::clone::Clone for ADS_VLV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADS_VLV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADS_VLV").field("dwBeforeCount", &self.dwBeforeCount).field("dwAfterCount", &self.dwAfterCount).field("dwOffset", &self.dwOffset).field("dwContentCount", &self.dwContentCount).field("pszTarget", &self.pszTarget).field("dwContextIDLength", &self.dwContextIDLength).field("lpContextID", &self.lpContextID).finish()
    }
}
impl ::windows_core::TypeKind for ADS_VLV {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ADS_VLV {
    fn eq(&self, other: &Self) -> bool {
        self.dwBeforeCount == other.dwBeforeCount && self.dwAfterCount == other.dwAfterCount && self.dwOffset == other.dwOffset && self.dwContentCount == other.dwContentCount && self.pszTarget == other.pszTarget && self.dwContextIDLength == other.dwContextIDLength && self.lpContextID == other.lpContextID
    }
}
impl ::core::cmp::Eq for ADS_VLV {}
impl ::core::default::Default for ADS_VLV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct CQFORM {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub clsid: ::windows_core::GUID,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pszTitle: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for CQFORM {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for CQFORM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for CQFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CQFORM").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("clsid", &self.clsid).field("hIcon", &self.hIcon).field("pszTitle", &self.pszTitle).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::TypeKind for CQFORM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for CQFORM {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.clsid == other.clsid && self.hIcon == other.hIcon && self.pszTitle == other.pszTitle
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for CQFORM {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for CQFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct CQPAGE {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub pPageProc: LPCQPAGEPROC,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub idPageName: i32,
    pub idPageTemplate: i32,
    pub pDlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for CQPAGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for CQPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for CQPAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CQPAGE").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hInstance", &self.hInstance).field("idPageName", &self.idPageName).field("idPageTemplate", &self.idPageTemplate).field("lParam", &self.lParam).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::TypeKind for CQPAGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for CQPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAINDESC {
    pub pszName: ::windows_core::PWSTR,
    pub pszPath: ::windows_core::PWSTR,
    pub pszNCName: ::windows_core::PWSTR,
    pub pszTrustParent: ::windows_core::PWSTR,
    pub pszObjectClass: ::windows_core::PWSTR,
    pub ulFlags: u32,
    pub fDownLevel: super::super::Foundation::BOOL,
    pub pdChildList: *mut DOMAINDESC,
    pub pdNextSibling: *mut DOMAINDESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOMAINDESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOMAINDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOMAINDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAINDESC").field("pszName", &self.pszName).field("pszPath", &self.pszPath).field("pszNCName", &self.pszNCName).field("pszTrustParent", &self.pszTrustParent).field("pszObjectClass", &self.pszObjectClass).field("ulFlags", &self.ulFlags).field("fDownLevel", &self.fDownLevel).field("pdChildList", &self.pdChildList).field("pdNextSibling", &self.pdNextSibling).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DOMAINDESC {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOMAINDESC {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszPath == other.pszPath && self.pszNCName == other.pszNCName && self.pszTrustParent == other.pszTrustParent && self.pszObjectClass == other.pszObjectClass && self.ulFlags == other.ulFlags && self.fDownLevel == other.fDownLevel && self.pdChildList == other.pdChildList && self.pdNextSibling == other.pdNextSibling
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOMAINDESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOMAINDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOMAIN_CONTROLLER_INFOA {
    pub DomainControllerName: ::windows_core::PSTR,
    pub DomainControllerAddress: ::windows_core::PSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: ::windows_core::GUID,
    pub DomainName: ::windows_core::PSTR,
    pub DnsForestName: ::windows_core::PSTR,
    pub Flags: u32,
    pub DcSiteName: ::windows_core::PSTR,
    pub ClientSiteName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for DOMAIN_CONTROLLER_INFOA {}
impl ::core::clone::Clone for DOMAIN_CONTROLLER_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOMAIN_CONTROLLER_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAIN_CONTROLLER_INFOA").field("DomainControllerName", &self.DomainControllerName).field("DomainControllerAddress", &self.DomainControllerAddress).field("DomainControllerAddressType", &self.DomainControllerAddressType).field("DomainGuid", &self.DomainGuid).field("DomainName", &self.DomainName).field("DnsForestName", &self.DnsForestName).field("Flags", &self.Flags).field("DcSiteName", &self.DcSiteName).field("ClientSiteName", &self.ClientSiteName).finish()
    }
}
impl ::windows_core::TypeKind for DOMAIN_CONTROLLER_INFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DOMAIN_CONTROLLER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.DomainControllerName == other.DomainControllerName && self.DomainControllerAddress == other.DomainControllerAddress && self.DomainControllerAddressType == other.DomainControllerAddressType && self.DomainGuid == other.DomainGuid && self.DomainName == other.DomainName && self.DnsForestName == other.DnsForestName && self.Flags == other.Flags && self.DcSiteName == other.DcSiteName && self.ClientSiteName == other.ClientSiteName
    }
}
impl ::core::cmp::Eq for DOMAIN_CONTROLLER_INFOA {}
impl ::core::default::Default for DOMAIN_CONTROLLER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOMAIN_CONTROLLER_INFOW {
    pub DomainControllerName: ::windows_core::PWSTR,
    pub DomainControllerAddress: ::windows_core::PWSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: ::windows_core::GUID,
    pub DomainName: ::windows_core::PWSTR,
    pub DnsForestName: ::windows_core::PWSTR,
    pub Flags: u32,
    pub DcSiteName: ::windows_core::PWSTR,
    pub ClientSiteName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for DOMAIN_CONTROLLER_INFOW {}
impl ::core::clone::Clone for DOMAIN_CONTROLLER_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOMAIN_CONTROLLER_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAIN_CONTROLLER_INFOW").field("DomainControllerName", &self.DomainControllerName).field("DomainControllerAddress", &self.DomainControllerAddress).field("DomainControllerAddressType", &self.DomainControllerAddressType).field("DomainGuid", &self.DomainGuid).field("DomainName", &self.DomainName).field("DnsForestName", &self.DnsForestName).field("Flags", &self.Flags).field("DcSiteName", &self.DcSiteName).field("ClientSiteName", &self.ClientSiteName).finish()
    }
}
impl ::windows_core::TypeKind for DOMAIN_CONTROLLER_INFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DOMAIN_CONTROLLER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.DomainControllerName == other.DomainControllerName && self.DomainControllerAddress == other.DomainControllerAddress && self.DomainControllerAddressType == other.DomainControllerAddressType && self.DomainGuid == other.DomainGuid && self.DomainName == other.DomainName && self.DnsForestName == other.DnsForestName && self.Flags == other.Flags && self.DcSiteName == other.DcSiteName && self.ClientSiteName == other.ClientSiteName
    }
}
impl ::core::cmp::Eq for DOMAIN_CONTROLLER_INFOW {}
impl ::core::default::Default for DOMAIN_CONTROLLER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOMAIN_TREE {
    pub dsSize: u32,
    pub dwCount: u32,
    pub aDomains: [DOMAINDESC; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOMAIN_TREE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOMAIN_TREE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOMAIN_TREE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOMAIN_TREE").field("dsSize", &self.dsSize).field("dwCount", &self.dwCount).field("aDomains", &self.aDomains).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DOMAIN_TREE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOMAIN_TREE {
    fn eq(&self, other: &Self) -> bool {
        self.dsSize == other.dsSize && self.dwCount == other.dwCount && self.aDomains == other.aDomains
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOMAIN_TREE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOMAIN_TREE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct DSA_NEWOBJ_DISPINFO {
    pub dwSize: u32,
    pub hObjClassIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub lpszWizTitle: ::windows_core::PWSTR,
    pub lpszContDisplayName: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for DSA_NEWOBJ_DISPINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for DSA_NEWOBJ_DISPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for DSA_NEWOBJ_DISPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSA_NEWOBJ_DISPINFO").field("dwSize", &self.dwSize).field("hObjClassIcon", &self.hObjClassIcon).field("lpszWizTitle", &self.lpszWizTitle).field("lpszContDisplayName", &self.lpszContDisplayName).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::TypeKind for DSA_NEWOBJ_DISPINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for DSA_NEWOBJ_DISPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hObjClassIcon == other.hObjClassIcon && self.lpszWizTitle == other.lpszWizTitle && self.lpszContDisplayName == other.lpszContDisplayName
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for DSA_NEWOBJ_DISPINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for DSA_NEWOBJ_DISPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSBITEMA {
    pub cbStruct: u32,
    pub pszADsPath: ::windows_core::PCWSTR,
    pub pszClass: ::windows_core::PCWSTR,
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szDisplayName: [u8; 64],
    pub szIconLocation: [u8; 260],
    pub iIconResID: i32,
}
impl ::core::marker::Copy for DSBITEMA {}
impl ::core::clone::Clone for DSBITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSBITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBITEMA").field("cbStruct", &self.cbStruct).field("pszADsPath", &self.pszADsPath).field("pszClass", &self.pszClass).field("dwMask", &self.dwMask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("szDisplayName", &self.szDisplayName).field("szIconLocation", &self.szIconLocation).field("iIconResID", &self.iIconResID).finish()
    }
}
impl ::windows_core::TypeKind for DSBITEMA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSBITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszADsPath == other.pszADsPath && self.pszClass == other.pszClass && self.dwMask == other.dwMask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.szDisplayName == other.szDisplayName && self.szIconLocation == other.szIconLocation && self.iIconResID == other.iIconResID
    }
}
impl ::core::cmp::Eq for DSBITEMA {}
impl ::core::default::Default for DSBITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSBITEMW {
    pub cbStruct: u32,
    pub pszADsPath: ::windows_core::PCWSTR,
    pub pszClass: ::windows_core::PCWSTR,
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szDisplayName: [u16; 64],
    pub szIconLocation: [u16; 260],
    pub iIconResID: i32,
}
impl ::core::marker::Copy for DSBITEMW {}
impl ::core::clone::Clone for DSBITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSBITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBITEMW").field("cbStruct", &self.cbStruct).field("pszADsPath", &self.pszADsPath).field("pszClass", &self.pszClass).field("dwMask", &self.dwMask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("szDisplayName", &self.szDisplayName).field("szIconLocation", &self.szIconLocation).field("iIconResID", &self.iIconResID).finish()
    }
}
impl ::windows_core::TypeKind for DSBITEMW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSBITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszADsPath == other.pszADsPath && self.pszClass == other.pszClass && self.dwMask == other.dwMask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.szDisplayName == other.szDisplayName && self.szIconLocation == other.szIconLocation && self.iIconResID == other.iIconResID
    }
}
impl ::core::cmp::Eq for DSBITEMW {}
impl ::core::default::Default for DSBITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct DSBROWSEINFOA {
    pub cbStruct: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pszCaption: ::windows_core::PCSTR,
    pub pszTitle: ::windows_core::PCSTR,
    pub pszRoot: ::windows_core::PCWSTR,
    pub pszPath: ::windows_core::PWSTR,
    pub cchPath: u32,
    pub dwFlags: u32,
    pub pfnCallback: super::super::UI::Shell::BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReturnFormat: u32,
    pub pUserName: ::windows_core::PCWSTR,
    pub pPassword: ::windows_core::PCWSTR,
    pub pszObjectClass: ::windows_core::PWSTR,
    pub cchObjectClass: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::marker::Copy for DSBROWSEINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::clone::Clone for DSBROWSEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::fmt::Debug for DSBROWSEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBROWSEINFOA")
            .field("cbStruct", &self.cbStruct)
            .field("hwndOwner", &self.hwndOwner)
            .field("pszCaption", &self.pszCaption)
            .field("pszTitle", &self.pszTitle)
            .field("pszRoot", &self.pszRoot)
            .field("pszPath", &self.pszPath)
            .field("cchPath", &self.cchPath)
            .field("dwFlags", &self.dwFlags)
            .field("lParam", &self.lParam)
            .field("dwReturnFormat", &self.dwReturnFormat)
            .field("pUserName", &self.pUserName)
            .field("pPassword", &self.pPassword)
            .field("pszObjectClass", &self.pszObjectClass)
            .field("cchObjectClass", &self.cchObjectClass)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::windows_core::TypeKind for DSBROWSEINFOA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for DSBROWSEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub struct DSBROWSEINFOW {
    pub cbStruct: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pszCaption: ::windows_core::PCWSTR,
    pub pszTitle: ::windows_core::PCWSTR,
    pub pszRoot: ::windows_core::PCWSTR,
    pub pszPath: ::windows_core::PWSTR,
    pub cchPath: u32,
    pub dwFlags: u32,
    pub pfnCallback: super::super::UI::Shell::BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReturnFormat: u32,
    pub pUserName: ::windows_core::PCWSTR,
    pub pPassword: ::windows_core::PCWSTR,
    pub pszObjectClass: ::windows_core::PWSTR,
    pub cchObjectClass: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::marker::Copy for DSBROWSEINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::clone::Clone for DSBROWSEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::fmt::Debug for DSBROWSEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBROWSEINFOW")
            .field("cbStruct", &self.cbStruct)
            .field("hwndOwner", &self.hwndOwner)
            .field("pszCaption", &self.pszCaption)
            .field("pszTitle", &self.pszTitle)
            .field("pszRoot", &self.pszRoot)
            .field("pszPath", &self.pszPath)
            .field("cchPath", &self.cchPath)
            .field("dwFlags", &self.dwFlags)
            .field("lParam", &self.lParam)
            .field("dwReturnFormat", &self.dwReturnFormat)
            .field("pUserName", &self.pUserName)
            .field("pPassword", &self.pPassword)
            .field("pszObjectClass", &self.pszObjectClass)
            .field("cchObjectClass", &self.cchObjectClass)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::windows_core::TypeKind for DSBROWSEINFOW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for DSBROWSEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSCLASSCREATIONINFO {
    pub dwFlags: u32,
    pub clsidWizardDialog: ::windows_core::GUID,
    pub clsidWizardPrimaryPage: ::windows_core::GUID,
    pub cWizardExtensions: u32,
    pub aWizardExtensions: [::windows_core::GUID; 1],
}
impl ::core::marker::Copy for DSCLASSCREATIONINFO {}
impl ::core::clone::Clone for DSCLASSCREATIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCLASSCREATIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCLASSCREATIONINFO").field("dwFlags", &self.dwFlags).field("clsidWizardDialog", &self.clsidWizardDialog).field("clsidWizardPrimaryPage", &self.clsidWizardPrimaryPage).field("cWizardExtensions", &self.cWizardExtensions).field("aWizardExtensions", &self.aWizardExtensions).finish()
    }
}
impl ::windows_core::TypeKind for DSCLASSCREATIONINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSCLASSCREATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.clsidWizardDialog == other.clsidWizardDialog && self.clsidWizardPrimaryPage == other.clsidWizardPrimaryPage && self.cWizardExtensions == other.cWizardExtensions && self.aWizardExtensions == other.aWizardExtensions
    }
}
impl ::core::cmp::Eq for DSCLASSCREATIONINFO {}
impl ::core::default::Default for DSCLASSCREATIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSCOLUMN {
    pub dwFlags: u32,
    pub fmt: i32,
    pub cx: i32,
    pub idsName: i32,
    pub offsetProperty: i32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DSCOLUMN {}
impl ::core::clone::Clone for DSCOLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCOLUMN").field("dwFlags", &self.dwFlags).field("fmt", &self.fmt).field("cx", &self.cx).field("idsName", &self.idsName).field("offsetProperty", &self.offsetProperty).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::windows_core::TypeKind for DSCOLUMN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.fmt == other.fmt && self.cx == other.cx && self.idsName == other.idsName && self.offsetProperty == other.offsetProperty && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DSCOLUMN {}
impl ::core::default::Default for DSCOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSDISPLAYSPECOPTIONS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub offsetAttribPrefix: u32,
    pub offsetUserName: u32,
    pub offsetPassword: u32,
    pub offsetServer: u32,
    pub offsetServerConfigPath: u32,
}
impl ::core::marker::Copy for DSDISPLAYSPECOPTIONS {}
impl ::core::clone::Clone for DSDISPLAYSPECOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSDISPLAYSPECOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSDISPLAYSPECOPTIONS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("offsetAttribPrefix", &self.offsetAttribPrefix).field("offsetUserName", &self.offsetUserName).field("offsetPassword", &self.offsetPassword).field("offsetServer", &self.offsetServer).field("offsetServerConfigPath", &self.offsetServerConfigPath).finish()
    }
}
impl ::windows_core::TypeKind for DSDISPLAYSPECOPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSDISPLAYSPECOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.offsetAttribPrefix == other.offsetAttribPrefix && self.offsetUserName == other.offsetUserName && self.offsetPassword == other.offsetPassword && self.offsetServer == other.offsetServer && self.offsetServerConfigPath == other.offsetServerConfigPath
    }
}
impl ::core::cmp::Eq for DSDISPLAYSPECOPTIONS {}
impl ::core::default::Default for DSDISPLAYSPECOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSOBJECT {
    pub dwFlags: u32,
    pub dwProviderFlags: u32,
    pub offsetName: u32,
    pub offsetClass: u32,
}
impl ::core::marker::Copy for DSOBJECT {}
impl ::core::clone::Clone for DSOBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOBJECT").field("dwFlags", &self.dwFlags).field("dwProviderFlags", &self.dwProviderFlags).field("offsetName", &self.offsetName).field("offsetClass", &self.offsetClass).finish()
    }
}
impl ::windows_core::TypeKind for DSOBJECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwProviderFlags == other.dwProviderFlags && self.offsetName == other.offsetName && self.offsetClass == other.offsetClass
    }
}
impl ::core::cmp::Eq for DSOBJECT {}
impl ::core::default::Default for DSOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSOBJECTNAMES {
    pub clsidNamespace: ::windows_core::GUID,
    pub cItems: u32,
    pub aObjects: [DSOBJECT; 1],
}
impl ::core::marker::Copy for DSOBJECTNAMES {}
impl ::core::clone::Clone for DSOBJECTNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSOBJECTNAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOBJECTNAMES").field("clsidNamespace", &self.clsidNamespace).field("cItems", &self.cItems).field("aObjects", &self.aObjects).finish()
    }
}
impl ::windows_core::TypeKind for DSOBJECTNAMES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSOBJECTNAMES {
    fn eq(&self, other: &Self) -> bool {
        self.clsidNamespace == other.clsidNamespace && self.cItems == other.cItems && self.aObjects == other.aObjects
    }
}
impl ::core::cmp::Eq for DSOBJECTNAMES {}
impl ::core::default::Default for DSOBJECTNAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSOP_FILTER_FLAGS {
    pub Uplevel: DSOP_UPLEVEL_FILTER_FLAGS,
    pub flDownlevel: u32,
}
impl ::core::marker::Copy for DSOP_FILTER_FLAGS {}
impl ::core::clone::Clone for DSOP_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSOP_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_FILTER_FLAGS").field("Uplevel", &self.Uplevel).field("flDownlevel", &self.flDownlevel).finish()
    }
}
impl ::windows_core::TypeKind for DSOP_FILTER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSOP_FILTER_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.Uplevel == other.Uplevel && self.flDownlevel == other.flDownlevel
    }
}
impl ::core::cmp::Eq for DSOP_FILTER_FLAGS {}
impl ::core::default::Default for DSOP_FILTER_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSOP_INIT_INFO {
    pub cbSize: u32,
    pub pwzTargetComputer: ::windows_core::PCWSTR,
    pub cDsScopeInfos: u32,
    pub aDsScopeInfos: *mut DSOP_SCOPE_INIT_INFO,
    pub flOptions: u32,
    pub cAttributesToFetch: u32,
    pub apwzAttributeNames: *const ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for DSOP_INIT_INFO {}
impl ::core::clone::Clone for DSOP_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSOP_INIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_INIT_INFO").field("cbSize", &self.cbSize).field("pwzTargetComputer", &self.pwzTargetComputer).field("cDsScopeInfos", &self.cDsScopeInfos).field("aDsScopeInfos", &self.aDsScopeInfos).field("flOptions", &self.flOptions).field("cAttributesToFetch", &self.cAttributesToFetch).field("apwzAttributeNames", &self.apwzAttributeNames).finish()
    }
}
impl ::windows_core::TypeKind for DSOP_INIT_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSOP_INIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pwzTargetComputer == other.pwzTargetComputer && self.cDsScopeInfos == other.cDsScopeInfos && self.aDsScopeInfos == other.aDsScopeInfos && self.flOptions == other.flOptions && self.cAttributesToFetch == other.cAttributesToFetch && self.apwzAttributeNames == other.apwzAttributeNames
    }
}
impl ::core::cmp::Eq for DSOP_INIT_INFO {}
impl ::core::default::Default for DSOP_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSOP_SCOPE_INIT_INFO {
    pub cbSize: u32,
    pub flType: u32,
    pub flScope: u32,
    pub FilterFlags: DSOP_FILTER_FLAGS,
    pub pwzDcName: ::windows_core::PCWSTR,
    pub pwzADsPath: ::windows_core::PCWSTR,
    pub hr: ::windows_core::HRESULT,
}
impl ::core::marker::Copy for DSOP_SCOPE_INIT_INFO {}
impl ::core::clone::Clone for DSOP_SCOPE_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSOP_SCOPE_INIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_SCOPE_INIT_INFO").field("cbSize", &self.cbSize).field("flType", &self.flType).field("flScope", &self.flScope).field("FilterFlags", &self.FilterFlags).field("pwzDcName", &self.pwzDcName).field("pwzADsPath", &self.pwzADsPath).field("hr", &self.hr).finish()
    }
}
impl ::windows_core::TypeKind for DSOP_SCOPE_INIT_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSOP_SCOPE_INIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.flType == other.flType && self.flScope == other.flScope && self.FilterFlags == other.FilterFlags && self.pwzDcName == other.pwzDcName && self.pwzADsPath == other.pwzADsPath && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for DSOP_SCOPE_INIT_INFO {}
impl ::core::default::Default for DSOP_SCOPE_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSOP_UPLEVEL_FILTER_FLAGS {
    pub flBothModes: u32,
    pub flMixedModeOnly: u32,
    pub flNativeModeOnly: u32,
}
impl ::core::marker::Copy for DSOP_UPLEVEL_FILTER_FLAGS {}
impl ::core::clone::Clone for DSOP_UPLEVEL_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSOP_UPLEVEL_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSOP_UPLEVEL_FILTER_FLAGS").field("flBothModes", &self.flBothModes).field("flMixedModeOnly", &self.flMixedModeOnly).field("flNativeModeOnly", &self.flNativeModeOnly).finish()
    }
}
impl ::windows_core::TypeKind for DSOP_UPLEVEL_FILTER_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSOP_UPLEVEL_FILTER_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.flBothModes == other.flBothModes && self.flMixedModeOnly == other.flMixedModeOnly && self.flNativeModeOnly == other.flNativeModeOnly
    }
}
impl ::core::cmp::Eq for DSOP_UPLEVEL_FILTER_FLAGS {}
impl ::core::default::Default for DSOP_UPLEVEL_FILTER_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSPROPERTYPAGEINFO {
    pub offsetString: u32,
}
impl ::core::marker::Copy for DSPROPERTYPAGEINFO {}
impl ::core::clone::Clone for DSPROPERTYPAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSPROPERTYPAGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTYPAGEINFO").field("offsetString", &self.offsetString).finish()
    }
}
impl ::windows_core::TypeKind for DSPROPERTYPAGEINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSPROPERTYPAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.offsetString == other.offsetString
    }
}
impl ::core::cmp::Eq for DSPROPERTYPAGEINFO {}
impl ::core::default::Default for DSPROPERTYPAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSQUERYCLASSLIST {
    pub cbStruct: u32,
    pub cClasses: i32,
    pub offsetClass: [u32; 1],
}
impl ::core::marker::Copy for DSQUERYCLASSLIST {}
impl ::core::clone::Clone for DSQUERYCLASSLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSQUERYCLASSLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSQUERYCLASSLIST").field("cbStruct", &self.cbStruct).field("cClasses", &self.cClasses).field("offsetClass", &self.offsetClass).finish()
    }
}
impl ::windows_core::TypeKind for DSQUERYCLASSLIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSQUERYCLASSLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cClasses == other.cClasses && self.offsetClass == other.offsetClass
    }
}
impl ::core::cmp::Eq for DSQUERYCLASSLIST {}
impl ::core::default::Default for DSQUERYCLASSLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSQUERYINITPARAMS {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub pDefaultScope: ::windows_core::PWSTR,
    pub pDefaultSaveLocation: ::windows_core::PWSTR,
    pub pUserName: ::windows_core::PWSTR,
    pub pPassword: ::windows_core::PWSTR,
    pub pServer: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for DSQUERYINITPARAMS {}
impl ::core::clone::Clone for DSQUERYINITPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSQUERYINITPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSQUERYINITPARAMS").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("pDefaultScope", &self.pDefaultScope).field("pDefaultSaveLocation", &self.pDefaultSaveLocation).field("pUserName", &self.pUserName).field("pPassword", &self.pPassword).field("pServer", &self.pServer).finish()
    }
}
impl ::windows_core::TypeKind for DSQUERYINITPARAMS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSQUERYINITPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.pDefaultScope == other.pDefaultScope && self.pDefaultSaveLocation == other.pDefaultSaveLocation && self.pUserName == other.pUserName && self.pPassword == other.pPassword && self.pServer == other.pServer
    }
}
impl ::core::cmp::Eq for DSQUERYINITPARAMS {}
impl ::core::default::Default for DSQUERYINITPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSQUERYPARAMS {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub offsetQuery: i32,
    pub iColumns: i32,
    pub dwReserved: u32,
    pub aColumns: [DSCOLUMN; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSQUERYPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSQUERYPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSQUERYPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSQUERYPARAMS").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hInstance", &self.hInstance).field("offsetQuery", &self.offsetQuery).field("iColumns", &self.iColumns).field("dwReserved", &self.dwReserved).field("aColumns", &self.aColumns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DSQUERYPARAMS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSQUERYPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.hInstance == other.hInstance && self.offsetQuery == other.offsetQuery && self.iColumns == other.iColumns && self.dwReserved == other.dwReserved && self.aColumns == other.aColumns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSQUERYPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSQUERYPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSROLE_OPERATION_STATE_INFO {
    pub OperationState: DSROLE_OPERATION_STATE,
}
impl ::core::marker::Copy for DSROLE_OPERATION_STATE_INFO {}
impl ::core::clone::Clone for DSROLE_OPERATION_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSROLE_OPERATION_STATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSROLE_OPERATION_STATE_INFO").field("OperationState", &self.OperationState).finish()
    }
}
impl ::windows_core::TypeKind for DSROLE_OPERATION_STATE_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSROLE_OPERATION_STATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationState == other.OperationState
    }
}
impl ::core::cmp::Eq for DSROLE_OPERATION_STATE_INFO {}
impl ::core::default::Default for DSROLE_OPERATION_STATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    pub MachineRole: DSROLE_MACHINE_ROLE,
    pub Flags: u32,
    pub DomainNameFlat: ::windows_core::PWSTR,
    pub DomainNameDns: ::windows_core::PWSTR,
    pub DomainForestName: ::windows_core::PWSTR,
    pub DomainGuid: ::windows_core::GUID,
}
impl ::core::marker::Copy for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {}
impl ::core::clone::Clone for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSROLE_PRIMARY_DOMAIN_INFO_BASIC").field("MachineRole", &self.MachineRole).field("Flags", &self.Flags).field("DomainNameFlat", &self.DomainNameFlat).field("DomainNameDns", &self.DomainNameDns).field("DomainForestName", &self.DomainForestName).field("DomainGuid", &self.DomainGuid).finish()
    }
}
impl ::windows_core::TypeKind for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn eq(&self, other: &Self) -> bool {
        self.MachineRole == other.MachineRole && self.Flags == other.Flags && self.DomainNameFlat == other.DomainNameFlat && self.DomainNameDns == other.DomainNameDns && self.DomainForestName == other.DomainForestName && self.DomainGuid == other.DomainGuid
    }
}
impl ::core::cmp::Eq for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {}
impl ::core::default::Default for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DSROLE_UPGRADE_STATUS_INFO {
    pub OperationState: u32,
    pub PreviousServerState: DSROLE_SERVER_STATE,
}
impl ::core::marker::Copy for DSROLE_UPGRADE_STATUS_INFO {}
impl ::core::clone::Clone for DSROLE_UPGRADE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSROLE_UPGRADE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSROLE_UPGRADE_STATUS_INFO").field("OperationState", &self.OperationState).field("PreviousServerState", &self.PreviousServerState).finish()
    }
}
impl ::windows_core::TypeKind for DSROLE_UPGRADE_STATUS_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DSROLE_UPGRADE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.OperationState == other.OperationState && self.PreviousServerState == other.PreviousServerState
    }
}
impl ::core::cmp::Eq for DSROLE_UPGRADE_STATUS_INFO {}
impl ::core::default::Default for DSROLE_UPGRADE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_1A {
    pub NetbiosName: ::windows_core::PSTR,
    pub DnsHostName: ::windows_core::PSTR,
    pub SiteName: ::windows_core::PSTR,
    pub ComputerObjectName: ::windows_core::PSTR,
    pub ServerObjectName: ::windows_core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_1A").field("NetbiosName", &self.NetbiosName).field("DnsHostName", &self.DnsHostName).field("SiteName", &self.SiteName).field("ComputerObjectName", &self.ComputerObjectName).field("ServerObjectName", &self.ServerObjectName).field("fIsPdc", &self.fIsPdc).field("fDsEnabled", &self.fDsEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_1A {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_1A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_1W {
    pub NetbiosName: ::windows_core::PWSTR,
    pub DnsHostName: ::windows_core::PWSTR,
    pub SiteName: ::windows_core::PWSTR,
    pub ComputerObjectName: ::windows_core::PWSTR,
    pub ServerObjectName: ::windows_core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_1W").field("NetbiosName", &self.NetbiosName).field("DnsHostName", &self.DnsHostName).field("SiteName", &self.SiteName).field("ComputerObjectName", &self.ComputerObjectName).field("ServerObjectName", &self.ServerObjectName).field("fIsPdc", &self.fIsPdc).field("fDsEnabled", &self.fDsEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_1W {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_1W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_2A {
    pub NetbiosName: ::windows_core::PSTR,
    pub DnsHostName: ::windows_core::PSTR,
    pub SiteName: ::windows_core::PSTR,
    pub SiteObjectName: ::windows_core::PSTR,
    pub ComputerObjectName: ::windows_core::PSTR,
    pub ServerObjectName: ::windows_core::PSTR,
    pub NtdsDsaObjectName: ::windows_core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows_core::GUID,
    pub ComputerObjectGuid: ::windows_core::GUID,
    pub ServerObjectGuid: ::windows_core::GUID,
    pub NtdsDsaObjectGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_2A")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_2A {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_2W {
    pub NetbiosName: ::windows_core::PWSTR,
    pub DnsHostName: ::windows_core::PWSTR,
    pub SiteName: ::windows_core::PWSTR,
    pub SiteObjectName: ::windows_core::PWSTR,
    pub ComputerObjectName: ::windows_core::PWSTR,
    pub ServerObjectName: ::windows_core::PWSTR,
    pub NtdsDsaObjectName: ::windows_core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows_core::GUID,
    pub ComputerObjectGuid: ::windows_core::GUID,
    pub ServerObjectGuid: ::windows_core::GUID,
    pub NtdsDsaObjectGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_2W")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_2W {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_3A {
    pub NetbiosName: ::windows_core::PSTR,
    pub DnsHostName: ::windows_core::PSTR,
    pub SiteName: ::windows_core::PSTR,
    pub SiteObjectName: ::windows_core::PSTR,
    pub ComputerObjectName: ::windows_core::PSTR,
    pub ServerObjectName: ::windows_core::PSTR,
    pub NtdsDsaObjectName: ::windows_core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub fIsRodc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows_core::GUID,
    pub ComputerObjectGuid: ::windows_core::GUID,
    pub ServerObjectGuid: ::windows_core::GUID,
    pub NtdsDsaObjectGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_3A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_3A")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("fIsRodc", &self.fIsRodc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_3A {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.fIsRodc == other.fIsRodc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_3A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_CONTROLLER_INFO_3W {
    pub NetbiosName: ::windows_core::PWSTR,
    pub DnsHostName: ::windows_core::PWSTR,
    pub SiteName: ::windows_core::PWSTR,
    pub SiteObjectName: ::windows_core::PWSTR,
    pub ComputerObjectName: ::windows_core::PWSTR,
    pub ServerObjectName: ::windows_core::PWSTR,
    pub NtdsDsaObjectName: ::windows_core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub fIsRodc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: ::windows_core::GUID,
    pub ComputerObjectGuid: ::windows_core::GUID,
    pub ServerObjectGuid: ::windows_core::GUID,
    pub NtdsDsaObjectGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_CONTROLLER_INFO_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_CONTROLLER_INFO_3W")
            .field("NetbiosName", &self.NetbiosName)
            .field("DnsHostName", &self.DnsHostName)
            .field("SiteName", &self.SiteName)
            .field("SiteObjectName", &self.SiteObjectName)
            .field("ComputerObjectName", &self.ComputerObjectName)
            .field("ServerObjectName", &self.ServerObjectName)
            .field("NtdsDsaObjectName", &self.NtdsDsaObjectName)
            .field("fIsPdc", &self.fIsPdc)
            .field("fDsEnabled", &self.fDsEnabled)
            .field("fIsGc", &self.fIsGc)
            .field("fIsRodc", &self.fIsRodc)
            .field("SiteObjectGuid", &self.SiteObjectGuid)
            .field("ComputerObjectGuid", &self.ComputerObjectGuid)
            .field("ServerObjectGuid", &self.ServerObjectGuid)
            .field("NtdsDsaObjectGuid", &self.NtdsDsaObjectGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_3W {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosName == other.NetbiosName && self.DnsHostName == other.DnsHostName && self.SiteName == other.SiteName && self.SiteObjectName == other.SiteObjectName && self.ComputerObjectName == other.ComputerObjectName && self.ServerObjectName == other.ServerObjectName && self.NtdsDsaObjectName == other.NtdsDsaObjectName && self.fIsPdc == other.fIsPdc && self.fDsEnabled == other.fDsEnabled && self.fIsGc == other.fIsGc && self.fIsRodc == other.fIsRodc && self.SiteObjectGuid == other.SiteObjectGuid && self.ComputerObjectGuid == other.ComputerObjectGuid && self.ServerObjectGuid == other.ServerObjectGuid && self.NtdsDsaObjectGuid == other.NtdsDsaObjectGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_CONTROLLER_INFO_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_TRUSTSA {
    pub NetbiosDomainName: ::windows_core::PSTR,
    pub DnsDomainName: ::windows_core::PSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::super::Foundation::PSID,
    pub DomainGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_TRUSTSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_TRUSTSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_TRUSTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_TRUSTSA").field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("Flags", &self.Flags).field("ParentIndex", &self.ParentIndex).field("TrustType", &self.TrustType).field("TrustAttributes", &self.TrustAttributes).field("DomainSid", &self.DomainSid).field("DomainGuid", &self.DomainGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_TRUSTSA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_TRUSTSA {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.Flags == other.Flags && self.ParentIndex == other.ParentIndex && self.TrustType == other.TrustType && self.TrustAttributes == other.TrustAttributes && self.DomainSid == other.DomainSid && self.DomainGuid == other.DomainGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_TRUSTSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_TRUSTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_DOMAIN_TRUSTSW {
    pub NetbiosDomainName: ::windows_core::PWSTR,
    pub DnsDomainName: ::windows_core::PWSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::super::Foundation::PSID,
    pub DomainGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_DOMAIN_TRUSTSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_DOMAIN_TRUSTSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_DOMAIN_TRUSTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_DOMAIN_TRUSTSW").field("NetbiosDomainName", &self.NetbiosDomainName).field("DnsDomainName", &self.DnsDomainName).field("Flags", &self.Flags).field("ParentIndex", &self.ParentIndex).field("TrustType", &self.TrustType).field("TrustAttributes", &self.TrustAttributes).field("DomainSid", &self.DomainSid).field("DomainGuid", &self.DomainGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_DOMAIN_TRUSTSW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_DOMAIN_TRUSTSW {
    fn eq(&self, other: &Self) -> bool {
        self.NetbiosDomainName == other.NetbiosDomainName && self.DnsDomainName == other.DnsDomainName && self.Flags == other.Flags && self.ParentIndex == other.ParentIndex && self.TrustType == other.TrustType && self.TrustAttributes == other.TrustAttributes && self.DomainSid == other.DomainSid && self.DomainGuid == other.DomainGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_DOMAIN_TRUSTSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_DOMAIN_TRUSTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_NAME_RESULTA {
    pub cItems: u32,
    pub rItems: *mut DS_NAME_RESULT_ITEMA,
}
impl ::core::marker::Copy for DS_NAME_RESULTA {}
impl ::core::clone::Clone for DS_NAME_RESULTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_NAME_RESULTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULTA").field("cItems", &self.cItems).field("rItems", &self.rItems).finish()
    }
}
impl ::windows_core::TypeKind for DS_NAME_RESULTA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_NAME_RESULTA {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.rItems == other.rItems
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULTA {}
impl ::core::default::Default for DS_NAME_RESULTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_NAME_RESULTW {
    pub cItems: u32,
    pub rItems: *mut DS_NAME_RESULT_ITEMW,
}
impl ::core::marker::Copy for DS_NAME_RESULTW {}
impl ::core::clone::Clone for DS_NAME_RESULTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_NAME_RESULTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULTW").field("cItems", &self.cItems).field("rItems", &self.rItems).finish()
    }
}
impl ::windows_core::TypeKind for DS_NAME_RESULTW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_NAME_RESULTW {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.rItems == other.rItems
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULTW {}
impl ::core::default::Default for DS_NAME_RESULTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_NAME_RESULT_ITEMA {
    pub status: u32,
    pub pDomain: ::windows_core::PSTR,
    pub pName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for DS_NAME_RESULT_ITEMA {}
impl ::core::clone::Clone for DS_NAME_RESULT_ITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_NAME_RESULT_ITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULT_ITEMA").field("status", &self.status).field("pDomain", &self.pDomain).field("pName", &self.pName).finish()
    }
}
impl ::windows_core::TypeKind for DS_NAME_RESULT_ITEMA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_NAME_RESULT_ITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.pDomain == other.pDomain && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULT_ITEMA {}
impl ::core::default::Default for DS_NAME_RESULT_ITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_NAME_RESULT_ITEMW {
    pub status: u32,
    pub pDomain: ::windows_core::PWSTR,
    pub pName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for DS_NAME_RESULT_ITEMW {}
impl ::core::clone::Clone for DS_NAME_RESULT_ITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_NAME_RESULT_ITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_NAME_RESULT_ITEMW").field("status", &self.status).field("pDomain", &self.pDomain).field("pName", &self.pName).finish()
    }
}
impl ::windows_core::TypeKind for DS_NAME_RESULT_ITEMW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_NAME_RESULT_ITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.pDomain == other.pDomain && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_NAME_RESULT_ITEMW {}
impl ::core::default::Default for DS_NAME_RESULT_ITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA {
    pub pszAttributeName: ::windows_core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_META_DATA").field("pszAttributeName", &self.pszAttributeName).field("dwVersion", &self.dwVersion).field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange).field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID).field("usnOriginatingChange", &self.usnOriginatingChange).field("usnLocalChange", &self.usnLocalChange).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_ATTR_META_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA_2 {
    pub pszAttributeName: ::windows_core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_META_DATA_2").field("pszAttributeName", &self.pszAttributeName).field("dwVersion", &self.dwVersion).field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange).field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID).field("usnOriginatingChange", &self.usnOriginatingChange).field("usnLocalChange", &self.usnLocalChange).field("pszLastOriginatingDsaDN", &self.pszLastOriginatingDsaDN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_ATTR_META_DATA_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.pszLastOriginatingDsaDN == other.pszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_META_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_META_DATA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_META_DATA_BLOB").field("oszAttributeName", &self.oszAttributeName).field("dwVersion", &self.dwVersion).field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange).field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID).field("usnOriginatingChange", &self.usnOriginatingChange).field("usnLocalChange", &self.usnLocalChange).field("oszLastOriginatingDsaDN", &self.oszLastOriginatingDsaDN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_ATTR_META_DATA_BLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_META_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszAttributeName == other.oszAttributeName && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.oszLastOriginatingDsaDN == other.oszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_META_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_VALUE_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_VALUE_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_VALUE_META_DATA").field("cNumEntries", &self.cNumEntries).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_ATTR_VALUE_META_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_VALUE_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwEnumerationContext == other.dwEnumerationContext && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_VALUE_META_DATA_2").field("cNumEntries", &self.cNumEntries).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_ATTR_VALUE_META_DATA_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwEnumerationContext == other.dwEnumerationContext && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_ATTR_VALUE_META_DATA_EXT {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_EXT; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_ATTR_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_ATTR_VALUE_META_DATA_EXT").field("cNumEntries", &self.cNumEntries).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwEnumerationContext == other.dwEnumerationContext && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_ATTR_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPL_CURSOR {
    pub uuidSourceDsaInvocationID: ::windows_core::GUID,
    pub usnAttributeFilter: i64,
}
impl ::core::marker::Copy for DS_REPL_CURSOR {}
impl ::core::clone::Clone for DS_REPL_CURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPL_CURSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPL_CURSOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPL_CURSOR {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter
    }
}
impl ::core::cmp::Eq for DS_REPL_CURSOR {}
impl ::core::default::Default for DS_REPL_CURSOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPL_CURSORS {
    pub cNumCursors: u32,
    pub dwReserved: u32,
    pub rgCursor: [DS_REPL_CURSOR; 1],
}
impl ::core::marker::Copy for DS_REPL_CURSORS {}
impl ::core::clone::Clone for DS_REPL_CURSORS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPL_CURSORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSORS").field("cNumCursors", &self.cNumCursors).field("dwReserved", &self.dwReserved).field("rgCursor", &self.rgCursor).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPL_CURSORS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPL_CURSORS {
    fn eq(&self, other: &Self) -> bool {
        self.cNumCursors == other.cNumCursors && self.dwReserved == other.dwReserved && self.rgCursor == other.rgCursor
    }
}
impl ::core::cmp::Eq for DS_REPL_CURSORS {}
impl ::core::default::Default for DS_REPL_CURSORS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSORS_2 {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSORS_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSORS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSORS_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSORS_2").field("cNumCursors", &self.cNumCursors).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgCursor", &self.rgCursor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_CURSORS_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSORS_2 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumCursors == other.cNumCursors && self.dwEnumerationContext == other.dwEnumerationContext && self.rgCursor == other.rgCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSORS_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSORS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSORS_3W {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_3W; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSORS_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSORS_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSORS_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSORS_3W").field("cNumCursors", &self.cNumCursors).field("dwEnumerationContext", &self.dwEnumerationContext).field("rgCursor", &self.rgCursor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_CURSORS_3W {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSORS_3W {
    fn eq(&self, other: &Self) -> bool {
        self.cNumCursors == other.cNumCursors && self.dwEnumerationContext == other.dwEnumerationContext && self.rgCursor == other.rgCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSORS_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSORS_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_2 {
    pub uuidSourceDsaInvocationID: ::windows_core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSOR_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSOR_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSOR_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR_2").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_CURSOR_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSOR_2 {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSOR_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSOR_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_3W {
    pub uuidSourceDsaInvocationID: ::windows_core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub pszSourceDsaDN: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSOR_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSOR_3W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSOR_3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR_3W").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess).field("pszSourceDsaDN", &self.pszSourceDsaDN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_CURSOR_3W {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSOR_3W {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess && self.pszSourceDsaDN == other.pszSourceDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSOR_3W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSOR_3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_CURSOR_BLOB {
    pub uuidSourceDsaInvocationID: ::windows_core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub oszSourceDsaDN: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_CURSOR_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_CURSOR_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_CURSOR_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_CURSOR_BLOB").field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID).field("usnAttributeFilter", &self.usnAttributeFilter).field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess).field("oszSourceDsaDN", &self.oszSourceDsaDN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_CURSOR_BLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_CURSOR_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID && self.usnAttributeFilter == other.usnAttributeFilter && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess && self.oszSourceDsaDN == other.oszSourceDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_CURSOR_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_CURSOR_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILURESW {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgDsaFailure: [DS_REPL_KCC_DSA_FAILUREW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_KCC_DSA_FAILURESW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_KCC_DSA_FAILURESW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_KCC_DSA_FAILURESW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_KCC_DSA_FAILURESW").field("cNumEntries", &self.cNumEntries).field("dwReserved", &self.dwReserved).field("rgDsaFailure", &self.rgDsaFailure).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_KCC_DSA_FAILURESW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_KCC_DSA_FAILURESW {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwReserved == other.dwReserved && self.rgDsaFailure == other.rgDsaFailure
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_KCC_DSA_FAILURESW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_KCC_DSA_FAILURESW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILUREW {
    pub pszDsaDN: ::windows_core::PWSTR,
    pub uuidDsaObjGuid: ::windows_core::GUID,
    pub ftimeFirstFailure: super::super::Foundation::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_KCC_DSA_FAILUREW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_KCC_DSA_FAILUREW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_KCC_DSA_FAILUREW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_KCC_DSA_FAILUREW").field("pszDsaDN", &self.pszDsaDN).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).field("ftimeFirstFailure", &self.ftimeFirstFailure).field("cNumFailures", &self.cNumFailures).field("dwLastResult", &self.dwLastResult).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_KCC_DSA_FAILUREW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_KCC_DSA_FAILUREW {
    fn eq(&self, other: &Self) -> bool {
        self.pszDsaDN == other.pszDsaDN && self.uuidDsaObjGuid == other.uuidDsaObjGuid && self.ftimeFirstFailure == other.ftimeFirstFailure && self.cNumFailures == other.cNumFailures && self.dwLastResult == other.dwLastResult
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_KCC_DSA_FAILUREW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_KCC_DSA_FAILUREW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_KCC_DSA_FAILUREW_BLOB {
    pub oszDsaDN: u32,
    pub uuidDsaObjGuid: ::windows_core::GUID,
    pub ftimeFirstFailure: super::super::Foundation::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_KCC_DSA_FAILUREW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_KCC_DSA_FAILUREW_BLOB").field("oszDsaDN", &self.oszDsaDN).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).field("ftimeFirstFailure", &self.ftimeFirstFailure).field("cNumFailures", &self.cNumFailures).field("dwLastResult", &self.dwLastResult).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszDsaDN == other.oszDsaDN && self.uuidDsaObjGuid == other.uuidDsaObjGuid && self.ftimeFirstFailure == other.ftimeFirstFailure && self.cNumFailures == other.cNumFailures && self.dwLastResult == other.dwLastResult
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_KCC_DSA_FAILUREW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORSW {
    pub cNumNeighbors: u32,
    pub dwReserved: u32,
    pub rgNeighbor: [DS_REPL_NEIGHBORW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_NEIGHBORSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_NEIGHBORSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_NEIGHBORSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_NEIGHBORSW").field("cNumNeighbors", &self.cNumNeighbors).field("dwReserved", &self.dwReserved).field("rgNeighbor", &self.rgNeighbor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_NEIGHBORSW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_NEIGHBORSW {
    fn eq(&self, other: &Self) -> bool {
        self.cNumNeighbors == other.cNumNeighbors && self.dwReserved == other.dwReserved && self.rgNeighbor == other.rgNeighbor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_NEIGHBORSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_NEIGHBORSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORW {
    pub pszNamingContext: ::windows_core::PWSTR,
    pub pszSourceDsaDN: ::windows_core::PWSTR,
    pub pszSourceDsaAddress: ::windows_core::PWSTR,
    pub pszAsyncIntersiteTransportDN: ::windows_core::PWSTR,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: ::windows_core::GUID,
    pub uuidSourceDsaObjGuid: ::windows_core::GUID,
    pub uuidSourceDsaInvocationID: ::windows_core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: ::windows_core::GUID,
    pub usnLastObjChangeSynced: i64,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub ftimeLastSyncAttempt: super::super::Foundation::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_NEIGHBORW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_NEIGHBORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_NEIGHBORW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_NEIGHBORW")
            .field("pszNamingContext", &self.pszNamingContext)
            .field("pszSourceDsaDN", &self.pszSourceDsaDN)
            .field("pszSourceDsaAddress", &self.pszSourceDsaAddress)
            .field("pszAsyncIntersiteTransportDN", &self.pszAsyncIntersiteTransportDN)
            .field("dwReplicaFlags", &self.dwReplicaFlags)
            .field("dwReserved", &self.dwReserved)
            .field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid)
            .field("uuidSourceDsaObjGuid", &self.uuidSourceDsaObjGuid)
            .field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID)
            .field("uuidAsyncIntersiteTransportObjGuid", &self.uuidAsyncIntersiteTransportObjGuid)
            .field("usnLastObjChangeSynced", &self.usnLastObjChangeSynced)
            .field("usnAttributeFilter", &self.usnAttributeFilter)
            .field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess)
            .field("ftimeLastSyncAttempt", &self.ftimeLastSyncAttempt)
            .field("dwLastSyncResult", &self.dwLastSyncResult)
            .field("cNumConsecutiveSyncFailures", &self.cNumConsecutiveSyncFailures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_NEIGHBORW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_NEIGHBORW {
    fn eq(&self, other: &Self) -> bool {
        self.pszNamingContext == other.pszNamingContext
            && self.pszSourceDsaDN == other.pszSourceDsaDN
            && self.pszSourceDsaAddress == other.pszSourceDsaAddress
            && self.pszAsyncIntersiteTransportDN == other.pszAsyncIntersiteTransportDN
            && self.dwReplicaFlags == other.dwReplicaFlags
            && self.dwReserved == other.dwReserved
            && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid
            && self.uuidSourceDsaObjGuid == other.uuidSourceDsaObjGuid
            && self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID
            && self.uuidAsyncIntersiteTransportObjGuid == other.uuidAsyncIntersiteTransportObjGuid
            && self.usnLastObjChangeSynced == other.usnLastObjChangeSynced
            && self.usnAttributeFilter == other.usnAttributeFilter
            && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess
            && self.ftimeLastSyncAttempt == other.ftimeLastSyncAttempt
            && self.dwLastSyncResult == other.dwLastSyncResult
            && self.cNumConsecutiveSyncFailures == other.cNumConsecutiveSyncFailures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_NEIGHBORW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_NEIGHBORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_NEIGHBORW_BLOB {
    pub oszNamingContext: u32,
    pub oszSourceDsaDN: u32,
    pub oszSourceDsaAddress: u32,
    pub oszAsyncIntersiteTransportDN: u32,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: ::windows_core::GUID,
    pub uuidSourceDsaObjGuid: ::windows_core::GUID,
    pub uuidSourceDsaInvocationID: ::windows_core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: ::windows_core::GUID,
    pub usnLastObjChangeSynced: i64,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub ftimeLastSyncAttempt: super::super::Foundation::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_NEIGHBORW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_NEIGHBORW_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_NEIGHBORW_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_NEIGHBORW_BLOB")
            .field("oszNamingContext", &self.oszNamingContext)
            .field("oszSourceDsaDN", &self.oszSourceDsaDN)
            .field("oszSourceDsaAddress", &self.oszSourceDsaAddress)
            .field("oszAsyncIntersiteTransportDN", &self.oszAsyncIntersiteTransportDN)
            .field("dwReplicaFlags", &self.dwReplicaFlags)
            .field("dwReserved", &self.dwReserved)
            .field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid)
            .field("uuidSourceDsaObjGuid", &self.uuidSourceDsaObjGuid)
            .field("uuidSourceDsaInvocationID", &self.uuidSourceDsaInvocationID)
            .field("uuidAsyncIntersiteTransportObjGuid", &self.uuidAsyncIntersiteTransportObjGuid)
            .field("usnLastObjChangeSynced", &self.usnLastObjChangeSynced)
            .field("usnAttributeFilter", &self.usnAttributeFilter)
            .field("ftimeLastSyncSuccess", &self.ftimeLastSyncSuccess)
            .field("ftimeLastSyncAttempt", &self.ftimeLastSyncAttempt)
            .field("dwLastSyncResult", &self.dwLastSyncResult)
            .field("cNumConsecutiveSyncFailures", &self.cNumConsecutiveSyncFailures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_NEIGHBORW_BLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_NEIGHBORW_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszNamingContext == other.oszNamingContext
            && self.oszSourceDsaDN == other.oszSourceDsaDN
            && self.oszSourceDsaAddress == other.oszSourceDsaAddress
            && self.oszAsyncIntersiteTransportDN == other.oszAsyncIntersiteTransportDN
            && self.dwReplicaFlags == other.dwReplicaFlags
            && self.dwReserved == other.dwReserved
            && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid
            && self.uuidSourceDsaObjGuid == other.uuidSourceDsaObjGuid
            && self.uuidSourceDsaInvocationID == other.uuidSourceDsaInvocationID
            && self.uuidAsyncIntersiteTransportObjGuid == other.uuidAsyncIntersiteTransportObjGuid
            && self.usnLastObjChangeSynced == other.usnLastObjChangeSynced
            && self.usnAttributeFilter == other.usnAttributeFilter
            && self.ftimeLastSyncSuccess == other.ftimeLastSyncSuccess
            && self.ftimeLastSyncAttempt == other.ftimeLastSyncAttempt
            && self.dwLastSyncResult == other.dwLastSyncResult
            && self.cNumConsecutiveSyncFailures == other.cNumConsecutiveSyncFailures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_NEIGHBORW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_NEIGHBORW_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OBJ_META_DATA {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OBJ_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OBJ_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OBJ_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OBJ_META_DATA").field("cNumEntries", &self.cNumEntries).field("dwReserved", &self.dwReserved).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_OBJ_META_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OBJ_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwReserved == other.dwReserved && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OBJ_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OBJ_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OBJ_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA_2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OBJ_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OBJ_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OBJ_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OBJ_META_DATA_2").field("cNumEntries", &self.cNumEntries).field("dwReserved", &self.dwReserved).field("rgMetaData", &self.rgMetaData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_OBJ_META_DATA_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OBJ_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.cNumEntries == other.cNumEntries && self.dwReserved == other.dwReserved && self.rgMetaData == other.rgMetaData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OBJ_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OBJ_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OPW {
    pub ftimeEnqueued: super::super::Foundation::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub pszNamingContext: ::windows_core::PWSTR,
    pub pszDsaDN: ::windows_core::PWSTR,
    pub pszDsaAddress: ::windows_core::PWSTR,
    pub uuidNamingContextObjGuid: ::windows_core::GUID,
    pub uuidDsaObjGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OPW").field("ftimeEnqueued", &self.ftimeEnqueued).field("ulSerialNumber", &self.ulSerialNumber).field("ulPriority", &self.ulPriority).field("OpType", &self.OpType).field("ulOptions", &self.ulOptions).field("pszNamingContext", &self.pszNamingContext).field("pszDsaDN", &self.pszDsaDN).field("pszDsaAddress", &self.pszDsaAddress).field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_OPW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OPW {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeEnqueued == other.ftimeEnqueued && self.ulSerialNumber == other.ulSerialNumber && self.ulPriority == other.ulPriority && self.OpType == other.OpType && self.ulOptions == other.ulOptions && self.pszNamingContext == other.pszNamingContext && self.pszDsaDN == other.pszDsaDN && self.pszDsaAddress == other.pszDsaAddress && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid && self.uuidDsaObjGuid == other.uuidDsaObjGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_OPW_BLOB {
    pub ftimeEnqueued: super::super::Foundation::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub oszNamingContext: u32,
    pub oszDsaDN: u32,
    pub oszDsaAddress: u32,
    pub uuidNamingContextObjGuid: ::windows_core::GUID,
    pub uuidDsaObjGuid: ::windows_core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_OPW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_OPW_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_OPW_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_OPW_BLOB").field("ftimeEnqueued", &self.ftimeEnqueued).field("ulSerialNumber", &self.ulSerialNumber).field("ulPriority", &self.ulPriority).field("OpType", &self.OpType).field("ulOptions", &self.ulOptions).field("oszNamingContext", &self.oszNamingContext).field("oszDsaDN", &self.oszDsaDN).field("oszDsaAddress", &self.oszDsaAddress).field("uuidNamingContextObjGuid", &self.uuidNamingContextObjGuid).field("uuidDsaObjGuid", &self.uuidDsaObjGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_OPW_BLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_OPW_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeEnqueued == other.ftimeEnqueued && self.ulSerialNumber == other.ulSerialNumber && self.ulPriority == other.ulPriority && self.OpType == other.OpType && self.ulOptions == other.ulOptions && self.oszNamingContext == other.oszNamingContext && self.oszDsaDN == other.oszDsaDN && self.oszDsaAddress == other.oszDsaAddress && self.uuidNamingContextObjGuid == other.uuidNamingContextObjGuid && self.uuidDsaObjGuid == other.uuidDsaObjGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_OPW_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_OPW_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_PENDING_OPSW {
    pub ftimeCurrentOpStarted: super::super::Foundation::FILETIME,
    pub cNumPendingOps: u32,
    pub rgPendingOp: [DS_REPL_OPW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_PENDING_OPSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_PENDING_OPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_PENDING_OPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_PENDING_OPSW").field("ftimeCurrentOpStarted", &self.ftimeCurrentOpStarted).field("cNumPendingOps", &self.cNumPendingOps).field("rgPendingOp", &self.rgPendingOp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_PENDING_OPSW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_PENDING_OPSW {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeCurrentOpStarted == other.ftimeCurrentOpStarted && self.cNumPendingOps == other.cNumPendingOps && self.rgPendingOp == other.rgPendingOp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_PENDING_OPSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_PENDING_OPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_QUEUE_STATISTICSW {
    pub ftimeCurrentOpStarted: super::super::Foundation::FILETIME,
    pub cNumPendingOps: u32,
    pub ftimeOldestSync: super::super::Foundation::FILETIME,
    pub ftimeOldestAdd: super::super::Foundation::FILETIME,
    pub ftimeOldestMod: super::super::Foundation::FILETIME,
    pub ftimeOldestDel: super::super::Foundation::FILETIME,
    pub ftimeOldestUpdRefs: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_QUEUE_STATISTICSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_QUEUE_STATISTICSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_QUEUE_STATISTICSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_QUEUE_STATISTICSW").field("ftimeCurrentOpStarted", &self.ftimeCurrentOpStarted).field("cNumPendingOps", &self.cNumPendingOps).field("ftimeOldestSync", &self.ftimeOldestSync).field("ftimeOldestAdd", &self.ftimeOldestAdd).field("ftimeOldestMod", &self.ftimeOldestMod).field("ftimeOldestDel", &self.ftimeOldestDel).field("ftimeOldestUpdRefs", &self.ftimeOldestUpdRefs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_QUEUE_STATISTICSW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_QUEUE_STATISTICSW {
    fn eq(&self, other: &Self) -> bool {
        self.ftimeCurrentOpStarted == other.ftimeCurrentOpStarted && self.cNumPendingOps == other.cNumPendingOps && self.ftimeOldestSync == other.ftimeOldestSync && self.ftimeOldestAdd == other.ftimeOldestAdd && self.ftimeOldestMod == other.ftimeOldestMod && self.ftimeOldestDel == other.ftimeOldestDel && self.ftimeOldestUpdRefs == other.ftimeOldestUpdRefs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_QUEUE_STATISTICSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_QUEUE_STATISTICSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA {
    pub pszAttributeName: ::windows_core::PWSTR,
    pub pszObjectDn: ::windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA")
            .field("pszAttributeName", &self.pszAttributeName)
            .field("pszObjectDn", &self.pszObjectDn)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_VALUE_META_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.pszObjectDn == other.pszObjectDn && self.cbData == other.cbData && self.pbData == other.pbData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_2 {
    pub pszAttributeName: ::windows_core::PWSTR,
    pub pszObjectDn: ::windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_2")
            .field("pszAttributeName", &self.pszAttributeName)
            .field("pszObjectDn", &self.pszObjectDn)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("pszLastOriginatingDsaDN", &self.pszLastOriginatingDsaDN)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_VALUE_META_DATA_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.pszObjectDn == other.pszObjectDn && self.cbData == other.cbData && self.pbData == other.pbData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.pszLastOriginatingDsaDN == other.pszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_BLOB")
            .field("oszAttributeName", &self.oszAttributeName)
            .field("oszObjectDn", &self.oszObjectDn)
            .field("cbData", &self.cbData)
            .field("obData", &self.obData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("oszLastOriginatingDsaDN", &self.oszLastOriginatingDsaDN)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_VALUE_META_DATA_BLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.oszAttributeName == other.oszAttributeName && self.oszObjectDn == other.oszObjectDn && self.cbData == other.cbData && self.obData == other.obData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.oszLastOriginatingDsaDN == other.oszLastOriginatingDsaDN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_BLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_BLOB_EXT {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_BLOB_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_BLOB_EXT")
            .field("oszAttributeName", &self.oszAttributeName)
            .field("oszObjectDn", &self.oszObjectDn)
            .field("cbData", &self.cbData)
            .field("obData", &self.obData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("oszLastOriginatingDsaDN", &self.oszLastOriginatingDsaDN)
            .field("dwUserIdentifier", &self.dwUserIdentifier)
            .field("dwPriorLinkState", &self.dwPriorLinkState)
            .field("dwCurrentLinkState", &self.dwCurrentLinkState)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.oszAttributeName == other.oszAttributeName && self.oszObjectDn == other.oszObjectDn && self.cbData == other.cbData && self.obData == other.obData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.oszLastOriginatingDsaDN == other.oszLastOriginatingDsaDN && self.dwUserIdentifier == other.dwUserIdentifier && self.dwPriorLinkState == other.dwPriorLinkState && self.dwCurrentLinkState == other.dwCurrentLinkState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_BLOB_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DS_REPL_VALUE_META_DATA_EXT {
    pub pszAttributeName: ::windows_core::PWSTR,
    pub pszObjectDn: ::windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: ::windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: ::windows_core::PWSTR,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DS_REPL_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DS_REPL_VALUE_META_DATA_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DS_REPL_VALUE_META_DATA_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPL_VALUE_META_DATA_EXT")
            .field("pszAttributeName", &self.pszAttributeName)
            .field("pszObjectDn", &self.pszObjectDn)
            .field("cbData", &self.cbData)
            .field("pbData", &self.pbData)
            .field("ftimeDeleted", &self.ftimeDeleted)
            .field("ftimeCreated", &self.ftimeCreated)
            .field("dwVersion", &self.dwVersion)
            .field("ftimeLastOriginatingChange", &self.ftimeLastOriginatingChange)
            .field("uuidLastOriginatingDsaInvocationID", &self.uuidLastOriginatingDsaInvocationID)
            .field("usnOriginatingChange", &self.usnOriginatingChange)
            .field("usnLocalChange", &self.usnLocalChange)
            .field("pszLastOriginatingDsaDN", &self.pszLastOriginatingDsaDN)
            .field("dwUserIdentifier", &self.dwUserIdentifier)
            .field("dwPriorLinkState", &self.dwPriorLinkState)
            .field("dwCurrentLinkState", &self.dwCurrentLinkState)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DS_REPL_VALUE_META_DATA_EXT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DS_REPL_VALUE_META_DATA_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.pszAttributeName == other.pszAttributeName && self.pszObjectDn == other.pszObjectDn && self.cbData == other.cbData && self.pbData == other.pbData && self.ftimeDeleted == other.ftimeDeleted && self.ftimeCreated == other.ftimeCreated && self.dwVersion == other.dwVersion && self.ftimeLastOriginatingChange == other.ftimeLastOriginatingChange && self.uuidLastOriginatingDsaInvocationID == other.uuidLastOriginatingDsaInvocationID && self.usnOriginatingChange == other.usnOriginatingChange && self.usnLocalChange == other.usnLocalChange && self.pszLastOriginatingDsaDN == other.pszLastOriginatingDsaDN && self.dwUserIdentifier == other.dwUserIdentifier && self.dwPriorLinkState == other.dwPriorLinkState && self.dwCurrentLinkState == other.dwCurrentLinkState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DS_REPL_VALUE_META_DATA_EXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DS_REPL_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPSYNCALL_ERRINFOA {
    pub pszSvrId: ::windows_core::PSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: ::windows_core::PSTR,
}
impl ::core::marker::Copy for DS_REPSYNCALL_ERRINFOA {}
impl ::core::clone::Clone for DS_REPSYNCALL_ERRINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_ERRINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_ERRINFOA").field("pszSvrId", &self.pszSvrId).field("error", &self.error).field("dwWin32Err", &self.dwWin32Err).field("pszSrcId", &self.pszSrcId).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_ERRINFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_ERRINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pszSvrId == other.pszSvrId && self.error == other.error && self.dwWin32Err == other.dwWin32Err && self.pszSrcId == other.pszSrcId
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_ERRINFOA {}
impl ::core::default::Default for DS_REPSYNCALL_ERRINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPSYNCALL_ERRINFOW {
    pub pszSvrId: ::windows_core::PWSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for DS_REPSYNCALL_ERRINFOW {}
impl ::core::clone::Clone for DS_REPSYNCALL_ERRINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_ERRINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_ERRINFOW").field("pszSvrId", &self.pszSvrId).field("error", &self.error).field("dwWin32Err", &self.dwWin32Err).field("pszSrcId", &self.pszSrcId).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_ERRINFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_ERRINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pszSvrId == other.pszSvrId && self.error == other.error && self.dwWin32Err == other.dwWin32Err && self.pszSrcId == other.pszSrcId
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_ERRINFOW {}
impl ::core::default::Default for DS_REPSYNCALL_ERRINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPSYNCALL_SYNCA {
    pub pszSrcId: ::windows_core::PSTR,
    pub pszDstId: ::windows_core::PSTR,
    pub pszNC: ::windows_core::PSTR,
    pub pguidSrc: *mut ::windows_core::GUID,
    pub pguidDst: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for DS_REPSYNCALL_SYNCA {}
impl ::core::clone::Clone for DS_REPSYNCALL_SYNCA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_SYNCA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_SYNCA").field("pszSrcId", &self.pszSrcId).field("pszDstId", &self.pszDstId).field("pszNC", &self.pszNC).field("pguidSrc", &self.pguidSrc).field("pguidDst", &self.pguidDst).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_SYNCA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_SYNCA {
    fn eq(&self, other: &Self) -> bool {
        self.pszSrcId == other.pszSrcId && self.pszDstId == other.pszDstId && self.pszNC == other.pszNC && self.pguidSrc == other.pguidSrc && self.pguidDst == other.pguidDst
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_SYNCA {}
impl ::core::default::Default for DS_REPSYNCALL_SYNCA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPSYNCALL_SYNCW {
    pub pszSrcId: ::windows_core::PWSTR,
    pub pszDstId: ::windows_core::PWSTR,
    pub pszNC: ::windows_core::PWSTR,
    pub pguidSrc: *mut ::windows_core::GUID,
    pub pguidDst: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for DS_REPSYNCALL_SYNCW {}
impl ::core::clone::Clone for DS_REPSYNCALL_SYNCW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_SYNCW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_SYNCW").field("pszSrcId", &self.pszSrcId).field("pszDstId", &self.pszDstId).field("pszNC", &self.pszNC).field("pguidSrc", &self.pguidSrc).field("pguidDst", &self.pguidDst).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_SYNCW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_SYNCW {
    fn eq(&self, other: &Self) -> bool {
        self.pszSrcId == other.pszSrcId && self.pszDstId == other.pszDstId && self.pszNC == other.pszNC && self.pguidSrc == other.pguidSrc && self.pguidDst == other.pguidDst
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_SYNCW {}
impl ::core::default::Default for DS_REPSYNCALL_SYNCW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPSYNCALL_UPDATEA {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOA,
    pub pSync: *mut DS_REPSYNCALL_SYNCA,
}
impl ::core::marker::Copy for DS_REPSYNCALL_UPDATEA {}
impl ::core::clone::Clone for DS_REPSYNCALL_UPDATEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_UPDATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_UPDATEA").field("event", &self.event).field("pErrInfo", &self.pErrInfo).field("pSync", &self.pSync).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_UPDATEA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_UPDATEA {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.pErrInfo == other.pErrInfo && self.pSync == other.pSync
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_UPDATEA {}
impl ::core::default::Default for DS_REPSYNCALL_UPDATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_REPSYNCALL_UPDATEW {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOW,
    pub pSync: *mut DS_REPSYNCALL_SYNCW,
}
impl ::core::marker::Copy for DS_REPSYNCALL_UPDATEW {}
impl ::core::clone::Clone for DS_REPSYNCALL_UPDATEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_REPSYNCALL_UPDATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_REPSYNCALL_UPDATEW").field("event", &self.event).field("pErrInfo", &self.pErrInfo).field("pSync", &self.pSync).finish()
    }
}
impl ::windows_core::TypeKind for DS_REPSYNCALL_UPDATEW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_REPSYNCALL_UPDATEW {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event && self.pErrInfo == other.pErrInfo && self.pSync == other.pSync
    }
}
impl ::core::cmp::Eq for DS_REPSYNCALL_UPDATEW {}
impl ::core::default::Default for DS_REPSYNCALL_UPDATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_SCHEMA_GUID_MAPA {
    pub guid: ::windows_core::GUID,
    pub guidType: u32,
    pub pName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for DS_SCHEMA_GUID_MAPA {}
impl ::core::clone::Clone for DS_SCHEMA_GUID_MAPA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_SCHEMA_GUID_MAPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SCHEMA_GUID_MAPA").field("guid", &self.guid).field("guidType", &self.guidType).field("pName", &self.pName).finish()
    }
}
impl ::windows_core::TypeKind for DS_SCHEMA_GUID_MAPA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_SCHEMA_GUID_MAPA {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.guidType == other.guidType && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_SCHEMA_GUID_MAPA {}
impl ::core::default::Default for DS_SCHEMA_GUID_MAPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_SCHEMA_GUID_MAPW {
    pub guid: ::windows_core::GUID,
    pub guidType: u32,
    pub pName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for DS_SCHEMA_GUID_MAPW {}
impl ::core::clone::Clone for DS_SCHEMA_GUID_MAPW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_SCHEMA_GUID_MAPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SCHEMA_GUID_MAPW").field("guid", &self.guid).field("guidType", &self.guidType).field("pName", &self.pName).finish()
    }
}
impl ::windows_core::TypeKind for DS_SCHEMA_GUID_MAPW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_SCHEMA_GUID_MAPW {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.guidType == other.guidType && self.pName == other.pName
    }
}
impl ::core::cmp::Eq for DS_SCHEMA_GUID_MAPW {}
impl ::core::default::Default for DS_SCHEMA_GUID_MAPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub struct DS_SELECTION {
    pub pwzName: ::windows_core::PWSTR,
    pub pwzADsPath: ::windows_core::PWSTR,
    pub pwzClass: ::windows_core::PWSTR,
    pub pwzUPN: ::windows_core::PWSTR,
    pub pvarFetchedAttributes: *mut super::super::System::Variant::VARIANT,
    pub flScopeType: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::marker::Copy for DS_SELECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for DS_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::fmt::Debug for DS_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SELECTION").field("pwzName", &self.pwzName).field("pwzADsPath", &self.pwzADsPath).field("pwzClass", &self.pwzClass).field("pwzUPN", &self.pwzUPN).field("pvarFetchedAttributes", &self.pvarFetchedAttributes).field("flScopeType", &self.flScopeType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for DS_SELECTION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::cmp::PartialEq for DS_SELECTION {
    fn eq(&self, other: &Self) -> bool {
        self.pwzName == other.pwzName && self.pwzADsPath == other.pwzADsPath && self.pwzClass == other.pwzClass && self.pwzUPN == other.pwzUPN && self.pvarFetchedAttributes == other.pvarFetchedAttributes && self.flScopeType == other.flScopeType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::cmp::Eq for DS_SELECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::default::Default for DS_SELECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub struct DS_SELECTION_LIST {
    pub cItems: u32,
    pub cFetchedAttributes: u32,
    pub aDsSelection: [DS_SELECTION; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::marker::Copy for DS_SELECTION_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::clone::Clone for DS_SELECTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::fmt::Debug for DS_SELECTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SELECTION_LIST").field("cItems", &self.cItems).field("cFetchedAttributes", &self.cFetchedAttributes).field("aDsSelection", &self.aDsSelection).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::TypeKind for DS_SELECTION_LIST {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::cmp::PartialEq for DS_SELECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.cFetchedAttributes == other.cFetchedAttributes && self.aDsSelection == other.aDsSelection
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::cmp::Eq for DS_SELECTION_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::core::default::Default for DS_SELECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DS_SITE_COST_INFO {
    pub errorCode: u32,
    pub cost: u32,
}
impl ::core::marker::Copy for DS_SITE_COST_INFO {}
impl ::core::clone::Clone for DS_SITE_COST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DS_SITE_COST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS_SITE_COST_INFO").field("errorCode", &self.errorCode).field("cost", &self.cost).finish()
    }
}
impl ::windows_core::TypeKind for DS_SITE_COST_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DS_SITE_COST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.errorCode == other.errorCode && self.cost == other.cost
    }
}
impl ::core::cmp::Eq for DS_SITE_COST_INFO {}
impl ::core::default::Default for DS_SITE_COST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct OPENQUERYWINDOW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub clsidHandler: ::windows_core::GUID,
    pub pHandlerParameters: *mut ::core::ffi::c_void,
    pub clsidDefaultForm: ::windows_core::GUID,
    pub pPersistQuery: ::std::mem::ManuallyDrop<::core::option::Option<IPersistQuery>>,
    pub Anonymous: OPENQUERYWINDOW_0,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for OPENQUERYWINDOW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::TypeKind for OPENQUERYWINDOW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::default::Default for OPENQUERYWINDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub union OPENQUERYWINDOW_0 {
    pub pFormParameters: *mut ::core::ffi::c_void,
    pub ppbFormParameters: ::std::mem::ManuallyDrop<::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag>>,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::clone::Clone for OPENQUERYWINDOW_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::TypeKind for OPENQUERYWINDOW_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::default::Default for OPENQUERYWINDOW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SCHEDULE {
    pub Size: u32,
    pub Bandwidth: u32,
    pub NumberOfSchedules: u32,
    pub Schedules: [SCHEDULE_HEADER; 1],
}
impl ::core::marker::Copy for SCHEDULE {}
impl ::core::clone::Clone for SCHEDULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCHEDULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHEDULE").field("Size", &self.Size).field("Bandwidth", &self.Bandwidth).field("NumberOfSchedules", &self.NumberOfSchedules).field("Schedules", &self.Schedules).finish()
    }
}
impl ::windows_core::TypeKind for SCHEDULE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SCHEDULE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Bandwidth == other.Bandwidth && self.NumberOfSchedules == other.NumberOfSchedules && self.Schedules == other.Schedules
    }
}
impl ::core::cmp::Eq for SCHEDULE {}
impl ::core::default::Default for SCHEDULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SCHEDULE_HEADER {
    pub Type: u32,
    pub Offset: u32,
}
impl ::core::marker::Copy for SCHEDULE_HEADER {}
impl ::core::clone::Clone for SCHEDULE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCHEDULE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCHEDULE_HEADER").field("Type", &self.Type).field("Offset", &self.Offset).finish()
    }
}
impl ::windows_core::TypeKind for SCHEDULE_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SCHEDULE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for SCHEDULE_HEADER {}
impl ::core::default::Default for SCHEDULE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPCQADDFORMSPROC = ::core::option::Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, pform: *mut CQFORM) -> ::windows_core::HRESULT>;
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPCQADDPAGESPROC = ::core::option::Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, clsidform: *const ::windows_core::GUID, ppage: *mut CQPAGE) -> ::windows_core::HRESULT>;
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPCQPAGEPROC = ::core::option::Option<unsafe extern "system" fn(ppage: *mut CQPAGE, hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMATTRIBUTES = ::core::option::Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, pszattributename: ::windows_core::PCWSTR, pszdisplayname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
