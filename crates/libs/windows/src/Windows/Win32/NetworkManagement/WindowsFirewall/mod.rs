#[inline]
pub unsafe fn NcFreeNetconProperties(pprops: *mut NETCON_PROPERTIES) {
    ::windows_targets::link!("netshell.dll" "system" fn NcFreeNetconProperties(pprops : *mut NETCON_PROPERTIES));
    NcFreeNetconProperties(pprops)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NcIsValidConnectionName<P0>(pszwname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("netshell.dll" "system" fn NcIsValidConnectionName(pszwname : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    NcIsValidConnectionName(pszwname.into_param().abi())
}
#[inline]
pub unsafe fn NetworkIsolationDiagnoseConnectFailureAndGetInfo<P0>(wszservername: P0, netisoerror: *mut NETISO_ERROR_TYPE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername : ::windows_core::PCWSTR, netisoerror : *mut NETISO_ERROR_TYPE) -> u32);
    NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername.into_param().abi(), netisoerror)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32 {
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationEnumAppContainers(flags : u32, pdwnumpublicappcs : *mut u32, pppublicappcs : *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32);
    NetworkIsolationEnumAppContainers(flags, pdwnumpublicappcs, pppublicappcs)
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
#[inline]
pub unsafe fn NetworkIsolationEnumerateAppContainerRules() -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT> {
    ::windows_targets::link!("firewallapi.dll" "system" fn NetworkIsolationEnumerateAppContainerRules(newenum : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    NetworkIsolationEnumerateAppContainerRules(&mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32 {
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationFreeAppContainers(ppublicappcs : *const INET_FIREWALL_APP_CONTAINER) -> u32);
    NetworkIsolationFreeAppContainers(ppublicappcs)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32 {
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs : *mut u32, appcontainersids : *mut *mut super::super::Security:: SID_AND_ATTRIBUTES) -> u32);
    NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs, appcontainersids)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetworkIsolationGetEnterpriseIdAsync<P0>(wszservername: P0, dwflags: u32, context: ::core::option::Option<*const ::core::ffi::c_void>, callback: PNETISO_EDP_ID_CALLBACK_FN, hoperation: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdAsync(wszservername : ::windows_core::PCWSTR, dwflags : u32, context : *const ::core::ffi::c_void, callback : PNETISO_EDP_ID_CALLBACK_FN, hoperation : *mut super::super::Foundation:: HANDLE) -> u32);
    NetworkIsolationGetEnterpriseIdAsync(wszservername.into_param().abi(), dwflags, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), callback, hoperation)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetworkIsolationGetEnterpriseIdClose<P0, P1>(hoperation: P0, bwaitforoperation: P1) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdClose(hoperation : super::super::Foundation:: HANDLE, bwaitforoperation : super::super::Foundation:: BOOL) -> u32);
    NetworkIsolationGetEnterpriseIdClose(hoperation.into_param().abi(), bwaitforoperation.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: ::core::option::Option<*const ::core::ffi::c_void>, registrationobject: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationRegisterForAppContainerChanges(flags : u32, callback : PAC_CHANGES_CALLBACK_FN, context : *const ::core::ffi::c_void, registrationobject : *mut super::super::Foundation:: HANDLE) -> u32);
    NetworkIsolationRegisterForAppContainerChanges(flags, callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), registrationobject)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetworkIsolationSetAppContainerConfig(appcontainersids: &[super::super::Security::SID_AND_ATTRIBUTES]) -> u32 {
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs : u32, appcontainersids : *const super::super::Security:: SID_AND_ATTRIBUTES) -> u32);
    NetworkIsolationSetAppContainerConfig(appcontainersids.len().try_into().unwrap(), ::core::mem::transmute(appcontainersids.as_ptr()))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetworkIsolationSetupAppContainerBinaries<P0, P1, P2, P3, P4>(applicationcontainersid: P0, packagefullname: P1, packagefolder: P2, displayname: P3, bbinariesfullycomputed: P4, binaries: &[::windows_core::PCWSTR]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::PSID>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid : super::super::Foundation:: PSID, packagefullname : ::windows_core::PCWSTR, packagefolder : ::windows_core::PCWSTR, displayname : ::windows_core::PCWSTR, bbinariesfullycomputed : super::super::Foundation:: BOOL, binaries : *const ::windows_core::PCWSTR, binariescount : u32) -> ::windows_core::HRESULT);
    NetworkIsolationSetupAppContainerBinaries(applicationcontainersid.into_param().abi(), packagefullname.into_param().abi(), packagefolder.into_param().abi(), displayname.into_param().abi(), bbinariesfullycomputed.into_param().abi(), ::core::mem::transmute(binaries.as_ptr()), binaries.len().try_into().unwrap()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetworkIsolationUnregisterForAppContainerChanges<P0>(registrationobject: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject : super::super::Foundation:: HANDLE) -> u32);
    NetworkIsolationUnregisterForAppContainerChanges(registrationobject.into_param().abi())
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDynamicPortMapping(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMapping {
    pub unsafe fn ExternalIPAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExternalIPAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoteHost(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteHost)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExternalPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InternalPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InternalClient(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InternalClient)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LeaseDuration(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LeaseDuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RenewLease(&self, lleasedurationdesired: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RenewLease)(::windows_core::Interface::as_raw(self), lleasedurationdesired, &mut result__).from_abi(result__)
    }
    pub unsafe fn EditInternalClient<P0>(&self, bstrinternalclient: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EditInternalClient)(::windows_core::Interface::as_raw(self), bstrinternalclient.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, vb: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self), vb.into_param().abi()).ok()
    }
    pub unsafe fn EditDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EditDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditInternalPort)(::windows_core::Interface::as_raw(self), linternalport).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDynamicPortMapping, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDynamicPortMapping {
    type Vtable = IDynamicPortMapping_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDynamicPortMapping {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fc80282_23b6_4378_9a27_cd8f17c9400c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemoteHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LeaseDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RenewLease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> ::windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
    pub EditDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDynamicPortMappingCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0, P1>(&self, bstrremotehost: P0, lexternalport: i32, bstrprotocol: P1) -> ::windows_core::Result<IDynamicPortMapping>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), bstrremotehost.into_param().abi(), lexternalport, bstrprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0, P1>(&self, bstrremotehost: P0, lexternalport: i32, bstrprotocol: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), bstrremotehost.into_param().abi(), lexternalport, bstrprotocol.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<P0, P1, P2, P3, P4>(&self, bstrremotehost: P0, lexternalport: i32, bstrprotocol: P1, linternalport: i32, bstrinternalclient: P2, benabled: P3, bstrdescription: P4, lleaseduration: i32) -> ::windows_core::Result<IDynamicPortMapping>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), bstrremotehost.into_param().abi(), lexternalport, bstrprotocol.into_param().abi(), linternalport, bstrinternalclient.into_param().abi(), benabled.into_param().abi(), bstrdescription.into_param().abi(), lleaseduration, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDynamicPortMappingCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDynamicPortMappingCollection {
    type Vtable = IDynamicPortMappingCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDynamicPortMappingCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb60de00f_156e_4e8d_9ec1_3a2342c10899);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdpm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrremotehost: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternalport: i32, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, lleaseduration: i32, ppdpm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumNetConnection(::windows_core::IUnknown);
impl IEnumNetConnection {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<INetConnection>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumNetConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumNetConnection {
    type Vtable = IEnumNetConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumNetConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a0_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumNetSharingEveryConnection(::windows_core::IUnknown);
impl IEnumNetSharingEveryConnection {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), ::core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingEveryConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumNetSharingEveryConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumNetSharingEveryConnection {
    type Vtable = IEnumNetSharingEveryConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumNetSharingEveryConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b8_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingEveryConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumNetSharingPortMapping(::windows_core::IUnknown);
impl IEnumNetSharingPortMapping {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), ::core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingPortMapping> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumNetSharingPortMapping, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumNetSharingPortMapping {
    type Vtable = IEnumNetSharingPortMapping_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumNetSharingPortMapping {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b0_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPortMapping_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumNetSharingPrivateConnection(::windows_core::IUnknown);
impl IEnumNetSharingPrivateConnection {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), ::core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingPrivateConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumNetSharingPrivateConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumNetSharingPrivateConnection {
    type Vtable = IEnumNetSharingPrivateConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumNetSharingPrivateConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b5_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPrivateConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumNetSharingPublicConnection(::windows_core::IUnknown);
impl IEnumNetSharingPublicConnection {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::super::System::Variant::VARIANT], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), ::core::mem::transmute(rgvar.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumNetSharingPublicConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumNetSharingPublicConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumNetSharingPublicConnection {
    type Vtable = IEnumNetSharingPublicConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumNetSharingPublicConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b4_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumNetSharingPublicConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::super::System::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INATEventManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INATEventManager {
    pub unsafe fn SetExternalIPAddressCallback<P0>(&self, punk: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetExternalIPAddressCallback)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn SetNumberOfEntriesCallback<P0>(&self, punk: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetNumberOfEntriesCallback)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INATEventManager, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INATEventManager {
    type Vtable = INATEventManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INATEventManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x624bd588_9060_4109_b0b0_1adbbcac32df);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INATEventManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetExternalIPAddressCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetNumberOfEntriesCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INATExternalIPAddressCallback(::windows_core::IUnknown);
impl INATExternalIPAddressCallback {
    pub unsafe fn NewExternalIPAddress<P0>(&self, bstrnewexternalipaddress: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).NewExternalIPAddress)(::windows_core::Interface::as_raw(self), bstrnewexternalipaddress.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(INATExternalIPAddressCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INATExternalIPAddressCallback {
    type Vtable = INATExternalIPAddressCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INATExternalIPAddressCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c416740_a34e_446f_ba06_abd04c3149ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATExternalIPAddressCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NewExternalIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrnewexternalipaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INATNumberOfEntriesCallback(::windows_core::IUnknown);
impl INATNumberOfEntriesCallback {
    pub unsafe fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NewNumberOfEntries)(::windows_core::Interface::as_raw(self), lnewnumberofentries).ok()
    }
}
::windows_core::imp::interface_hierarchy!(INATNumberOfEntriesCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INATNumberOfEntriesCallback {
    type Vtable = INATNumberOfEntriesCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INATNumberOfEntriesCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc83a0a74_91ee_41b6_b67a_67e0f00bbd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct INATNumberOfEntriesCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NewNumberOfEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewnumberofentries: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetConnection(::windows_core::IUnknown);
impl INetConnection {
    pub unsafe fn Connect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Duplicate<P0>(&self, pszwduplicatename: P0) -> ::windows_core::Result<INetConnection>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Duplicate)(::windows_core::Interface::as_raw(self), pszwduplicatename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows_core::Result<*mut NETCON_PROPERTIES> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUiObjectClassId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUiObjectClassId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Rename<P0>(&self, pszwnewname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Rename)(::windows_core::Interface::as_raw(self), pszwnewname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(INetConnection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetConnection {
    type Vtable = INetConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a1_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Duplicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwduplicatename: ::windows_core::PCWSTR, ppcon: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> ::windows_core::HRESULT,
    pub GetUiObjectClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Rename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszwnewname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetConnectionConnectUi(::windows_core::IUnknown);
impl INetConnectionConnectUi {
    pub unsafe fn SetConnection<P0>(&self, pcon: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<INetConnection>,
    {
        (::windows_core::Interface::vtable(self).SetConnection)(::windows_core::Interface::as_raw(self), pcon.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<P0>(&self, hwndparent: P0, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Disconnect<P0>(&self, hwndparent: P0, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), dwflags).ok()
    }
}
::windows_core::imp::interface_hierarchy!(INetConnectionConnectUi, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetConnectionConnectUi {
    type Vtable = INetConnectionConnectUi_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetConnectionConnectUi {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a3_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionConnectUi_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcon: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Disconnect: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetConnectionManager(::windows_core::IUnknown);
impl INetConnectionManager {
    pub unsafe fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> ::windows_core::Result<IEnumNetConnection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumConnections)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(INetConnectionManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetConnectionManager {
    type Vtable = INetConnectionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetConnectionManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956a2_1cd3_11d1_b1c5_00805fc1270e);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub EnumConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetConnectionProps(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetConnectionProps {
    pub unsafe fn Guid(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Guid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeviceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<NETCON_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MediaType(&self) -> ::windows_core::Result<NETCON_MEDIATYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MediaType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Characteristics(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Characteristics)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetConnectionProps, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetConnectionProps {
    type Vtable = INetConnectionProps_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetConnectionProps {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4277c95_ce5b_463d_8167_5662d9bcaa72);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetConnectionProps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> ::windows_core::HRESULT,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> ::windows_core::HRESULT,
    pub Characteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwAuthorizedApplication(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplication {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn ProcessImageFileName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProcessImageFileName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProcessImageFileName<P0>(&self, imagefilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetProcessImageFileName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwAuthorizedApplication, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwAuthorizedApplication {
    type Vtable = INetFwAuthorizedApplication_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwAuthorizedApplication {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5e64ffa_c2c5_444e_a301_fb5e00018050);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplication_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ProcessImageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetProcessImageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwAuthorizedApplications(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplications {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, app: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<INetFwAuthorizedApplication>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), app.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, imagefilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, imagefilename: P0) -> ::windows_core::Result<INetFwAuthorizedApplication>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwAuthorizedApplications, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwAuthorizedApplications {
    type Vtable = INetFwAuthorizedApplications_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwAuthorizedApplications {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x644efd52_ccf9_486c_97a2_39f352570b30);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwAuthorizedApplications_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, app: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, app: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwIcmpSettings(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwIcmpSettings {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowOutboundDestinationUnreachable(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundDestinationUnreachable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowOutboundDestinationUnreachable<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowOutboundDestinationUnreachable)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowRedirect(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowRedirect)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowRedirect<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowRedirect)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInboundEchoRequest(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundEchoRequest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInboundEchoRequest<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInboundEchoRequest)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowOutboundTimeExceeded(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundTimeExceeded)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowOutboundTimeExceeded<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowOutboundTimeExceeded)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowOutboundParameterProblem(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundParameterProblem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowOutboundParameterProblem<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowOutboundParameterProblem)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowOutboundSourceQuench(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundSourceQuench)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowOutboundSourceQuench<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowOutboundSourceQuench)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInboundRouterRequest(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundRouterRequest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInboundRouterRequest<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInboundRouterRequest)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInboundTimestampRequest(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundTimestampRequest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInboundTimestampRequest<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInboundTimestampRequest)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInboundMaskRequest(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowInboundMaskRequest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInboundMaskRequest<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowInboundMaskRequest)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowOutboundPacketTooBig(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllowOutboundPacketTooBig)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowOutboundPacketTooBig<P0>(&self, allow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetAllowOutboundPacketTooBig)(::windows_core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwIcmpSettings, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwIcmpSettings {
    type Vtable = INetFwIcmpSettings_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwIcmpSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6207b2e_7cdd_426a_951e_5e1cbc5afead);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwIcmpSettings_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowOutboundDestinationUnreachable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowOutboundDestinationUnreachable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowOutboundDestinationUnreachable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowOutboundDestinationUnreachable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowRedirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowRedirect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInboundEchoRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInboundEchoRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInboundEchoRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInboundEchoRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowOutboundTimeExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowOutboundTimeExceeded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowOutboundTimeExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowOutboundTimeExceeded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowOutboundParameterProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowOutboundParameterProblem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowOutboundParameterProblem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowOutboundParameterProblem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowOutboundSourceQuench: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowOutboundSourceQuench: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowOutboundSourceQuench: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowOutboundSourceQuench: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInboundRouterRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInboundRouterRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInboundRouterRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInboundRouterRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInboundTimestampRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInboundTimestampRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInboundTimestampRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInboundTimestampRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInboundMaskRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInboundMaskRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInboundMaskRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInboundMaskRequest: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowOutboundPacketTooBig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowOutboundPacketTooBig: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowOutboundPacketTooBig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowOutboundPacketTooBig: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwMgr(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwMgr {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LocalPolicy(&self) -> ::windows_core::Result<INetFwPolicy> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentProfileType(&self) -> ::windows_core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentProfileType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RestoreDefaults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreDefaults)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn IsPortAllowed<P0, P1>(&self, imagefilename: P0, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: P1, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).IsPortAllowed)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi(), ipversion, portnumber, localaddress.into_param().abi(), ipprotocol, allowed, restricted).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn IsIcmpTypeAllowed<P0>(&self, ipversion: NET_FW_IP_VERSION, localaddress: P0, r#type: u8, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).IsIcmpTypeAllowed)(::windows_core::Interface::as_raw(self), ipversion, localaddress.into_param().abi(), r#type, allowed, restricted).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwMgr, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwMgr {
    type Vtable = INetFwMgr_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwMgr {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7898af5_cac4_4632_a2ec_da06e5111af2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwMgr_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LocalPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LocalPolicy: usize,
    pub CurrentProfileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> ::windows_core::HRESULT,
    pub RestoreDefaults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub IsPortAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    IsPortAllowed: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub IsIcmpTypeAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#type: u8, allowed: *mut super::super::System::Variant::VARIANT, restricted: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    IsIcmpTypeAllowed: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwOpenPort(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPort {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<NET_FW_IP_PROTOCOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtocol)(::windows_core::Interface::as_raw(self), ipprotocol).ok()
    }
    pub unsafe fn Port(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Port)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, portnumber: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPort)(::windows_core::Interface::as_raw(self), portnumber).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BuiltIn(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BuiltIn)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwOpenPort, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwOpenPort {
    type Vtable = INetFwOpenPort_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwOpenPort {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0483ba0_47ff_4d9c_a6d6_7741d0b195f7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPort_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: *mut i32) -> ::windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BuiltIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, builtin: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BuiltIn: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwOpenPorts(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPorts {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, port: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<INetFwOpenPort>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), port.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), portnumber, ipprotocol).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::Result<INetFwOpenPort> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), portnumber, ipprotocol, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwOpenPorts, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwOpenPorts {
    type Vtable = INetFwOpenPorts_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwOpenPorts {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0e9d7fa_e07e_430a_b19a_090ce82d92e2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwOpenPorts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, port: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwPolicy(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicy {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentProfile(&self) -> ::windows_core::Result<INetFwProfile> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentProfile)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> ::windows_core::Result<INetFwProfile> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProfileByType)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwPolicy, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwPolicy {
    type Vtable = INetFwPolicy_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwPolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd46d2478_9ac9_4008_9dc7_5563ce5536cc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentProfile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProfileByType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProfileByType: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwPolicy2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicy2 {
    pub unsafe fn CurrentProfileTypes(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CurrentProfileTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_FirewallEnabled)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_FirewallEnabled<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).put_FirewallEnabled)(::windows_core::Interface::as_raw(self), profiletype, enabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_ExcludedInterfaces)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn put_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_ExcludedInterfaces)(::windows_core::Interface::as_raw(self), profiletype, ::core::mem::transmute(interfaces)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_BlockAllInboundTraffic)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_BlockAllInboundTraffic<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, block: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).put_BlockAllInboundTraffic)(::windows_core::Interface::as_raw(self), profiletype, block.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_NotificationsDisabled)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_NotificationsDisabled<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).put_NotificationsDisabled)(::windows_core::Interface::as_raw(self), profiletype, disabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_UnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_UnicastResponsesToMulticastBroadcastDisabled<P0>(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).put_UnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), profiletype, disabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rules(&self) -> ::windows_core::Result<INetFwRules> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Rules)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ServiceRestriction(&self) -> ::windows_core::Result<INetFwServiceRestriction> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServiceRestriction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableRuleGroup<P0, P1>(&self, profiletypesbitmask: i32, group: P0, enable: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).EnableRuleGroup)(::windows_core::Interface::as_raw(self), profiletypesbitmask, group.into_param().abi(), enable.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRuleGroupEnabled<P0>(&self, profiletypesbitmask: i32, group: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRuleGroupEnabled)(::windows_core::Interface::as_raw(self), profiletypesbitmask, group.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RestoreLocalFirewallDefaults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreLocalFirewallDefaults)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn get_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_DefaultInboundAction)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_DefaultInboundAction)(::windows_core::Interface::as_raw(self), profiletype, action).ok()
    }
    pub unsafe fn get_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_DefaultOutboundAction)(::windows_core::Interface::as_raw(self), profiletype, &mut result__).from_abi(result__)
    }
    pub unsafe fn put_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_DefaultOutboundAction)(::windows_core::Interface::as_raw(self), profiletype, action).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_IsRuleGroupCurrentlyEnabled<P0>(&self, group: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_IsRuleGroupCurrentlyEnabled)(::windows_core::Interface::as_raw(self), group.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn LocalPolicyModifyState(&self) -> ::windows_core::Result<NET_FW_MODIFY_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalPolicyModifyState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwPolicy2, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwPolicy2 {
    type Vtable = INetFwPolicy2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwPolicy2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98325047_c671_4174_8d81_defcd3f03186);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwPolicy2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CurrentProfileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_FirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_FirewallEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_FirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_FirewallEnabled: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_ExcludedInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_ExcludedInterfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub put_ExcludedInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    put_ExcludedInterfaces: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_BlockAllInboundTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_BlockAllInboundTraffic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_BlockAllInboundTraffic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_BlockAllInboundTraffic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_NotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_NotificationsDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_NotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_NotificationsDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_UnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_UnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ServiceRestriction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicerestriction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ServiceRestriction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableRuleGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::std::mem::MaybeUninit<::windows_core::BSTR>, enable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableRuleGroup: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRuleGroupEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32, group: ::std::mem::MaybeUninit<::windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRuleGroupEnabled: usize,
    pub RestoreLocalFirewallDefaults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_DefaultInboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub put_DefaultInboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub get_DefaultOutboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub put_DefaultOutboundAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_IsRuleGroupCurrentlyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::std::mem::MaybeUninit<::windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_IsRuleGroupCurrentlyEnabled: usize,
    pub LocalPolicyModifyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwProduct(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwProduct {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RuleCategories(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RuleCategories)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetRuleCategories(&self, rulecategories: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRuleCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rulecategories)).ok()
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, displayname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), displayname.into_param().abi()).ok()
    }
    pub unsafe fn PathToSignedProductExe(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PathToSignedProductExe)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwProduct, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwProduct {
    type Vtable = INetFwProduct_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwProduct {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71881699_18f4_458b_b892_3ffce5e07f75);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProduct_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RuleCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulecategories: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RuleCategories: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetRuleCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulecategories: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetRuleCategories: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PathToSignedProductExe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwProducts(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwProducts {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Register<P0>(&self, product: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<INetFwProduct>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Register)(::windows_core::Interface::as_raw(self), product.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<INetFwProduct> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwProducts, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwProducts {
    type Vtable = INetFwProducts_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwProducts {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39eb36e0_2097_40bd_8af2_63a13b525362);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProducts_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, product: *mut ::core::ffi::c_void, registration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Register: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, product: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwProfile(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwProfile {
    pub unsafe fn Type(&self) -> ::windows_core::Result<NET_FW_PROFILE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FirewallEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FirewallEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFirewallEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetFirewallEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExceptionsNotAllowed(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExceptionsNotAllowed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExceptionsNotAllowed<P0>(&self, notallowed: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetExceptionsNotAllowed)(::windows_core::Interface::as_raw(self), notallowed.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotificationsDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NotificationsDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotificationsDisabled<P0>(&self, disabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetNotificationsDisabled)(::windows_core::Interface::as_raw(self), disabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnicastResponsesToMulticastBroadcastDisabled<P0>(&self, disabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUnicastResponsesToMulticastBroadcastDisabled)(::windows_core::Interface::as_raw(self), disabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoteAdminSettings(&self) -> ::windows_core::Result<INetFwRemoteAdminSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAdminSettings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IcmpSettings(&self) -> ::windows_core::Result<INetFwIcmpSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IcmpSettings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows_core::Result<INetFwOpenPorts> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GloballyOpenPorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> ::windows_core::Result<INetFwServices> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Services)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AuthorizedApplications(&self) -> ::windows_core::Result<INetFwAuthorizedApplications> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AuthorizedApplications)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwProfile, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwProfile {
    type Vtable = INetFwProfile_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x174a0dda_e9f9_449d_993b_21ab667ca456);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwProfile_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FirewallEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFirewallEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExceptionsNotAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExceptionsNotAllowed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExceptionsNotAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExceptionsNotAllowed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotificationsDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotificationsDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotificationsDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnicastResponsesToMulticastBroadcastDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnicastResponsesToMulticastBroadcastDisabled: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoteAdminSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteadminsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoteAdminSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IcmpSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmpsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IcmpSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GloballyOpenPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openports: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GloballyOpenPorts: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, services: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AuthorizedApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AuthorizedApplications: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwRemoteAdminSettings(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRemoteAdminSettings {
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwRemoteAdminSettings, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwRemoteAdminSettings {
    type Vtable = INetFwRemoteAdminSettings_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwRemoteAdminSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4becddf_6f73_4a83_b832_9c66874cd20e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRemoteAdminSettings_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwRule(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, desc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), desc.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationName<P0>(&self, imagefilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetApplicationName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn ServiceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServiceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceName<P0>(&self, servicename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServiceName)(::windows_core::Interface::as_raw(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtocol)(::windows_core::Interface::as_raw(self), protocol).ok()
    }
    pub unsafe fn LocalPorts(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalPorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalPorts<P0>(&self, portnumbers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalPorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn RemotePorts(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemotePorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemotePorts<P0>(&self, portnumbers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemotePorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn LocalAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalAddresses<P0>(&self, localaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalAddresses)(::windows_core::Interface::as_raw(self), localaddrs.into_param().abi()).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIcmpTypesAndCodes<P0>(&self, icmptypesandcodes: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetIcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows_core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Direction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDirection)(::windows_core::Interface::as_raw(self), dir).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Interfaces(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Interfaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetInterfaces(&self, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInterfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interfaces)).ok()
    }
    pub unsafe fn InterfaceTypes(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InterfaceTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterfaceTypes<P0>(&self, interfacetypes: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInterfaceTypes)(::windows_core::Interface::as_raw(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    pub unsafe fn Grouping(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Grouping)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGrouping<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetGrouping)(::windows_core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Profiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProfiles)(::windows_core::Interface::as_raw(self), profiletypesbitmask).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EdgeTraversal(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EdgeTraversal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEdgeTraversal<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEdgeTraversal)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    pub unsafe fn Action(&self) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Action)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAction)(::windows_core::Interface::as_raw(self), action).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwRule, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwRule {
    type Vtable = INetFwRule_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwRule {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf230d27_baba_4e42_aced_f524f22cfce2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: *mut i32) -> ::windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: i32) -> ::windows_core::HRESULT,
    pub LocalPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemotePorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemotePorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portnumbers: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LocalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IcmpTypesAndCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetIcmpTypesAndCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icmptypesandcodes: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> ::windows_core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Interfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Interfaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetInterfaces: usize,
    pub InterfaceTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInterfaceTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interfacetypes: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    pub Grouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetGrouping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Profiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: *mut i32) -> ::windows_core::HRESULT,
    pub SetProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profiletypesbitmask: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EdgeTraversal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EdgeTraversal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEdgeTraversal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEdgeTraversal: usize,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut NET_FW_ACTION) -> ::windows_core::HRESULT,
    pub SetAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: NET_FW_ACTION) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwRule2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, desc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), desc.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationName<P0>(&self, imagefilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetApplicationName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn ServiceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceName<P0>(&self, servicename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetServiceName)(::windows_core::Interface::as_raw(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Protocol)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProtocol)(::windows_core::Interface::as_raw(self), protocol).ok()
    }
    pub unsafe fn LocalPorts(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalPorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalPorts<P0>(&self, portnumbers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLocalPorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn RemotePorts(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RemotePorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemotePorts<P0>(&self, portnumbers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetRemotePorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn LocalAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.LocalAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalAddresses<P0>(&self, localaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLocalAddresses)(::windows_core::Interface::as_raw(self), localaddrs.into_param().abi()).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RemoteAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIcmpTypesAndCodes<P0>(&self, icmptypesandcodes: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows_core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Direction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDirection)(::windows_core::Interface::as_raw(self), dir).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Interfaces(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Interfaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetInterfaces(&self, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInterfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interfaces)).ok()
    }
    pub unsafe fn InterfaceTypes(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InterfaceTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterfaceTypes<P0>(&self, interfacetypes: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetInterfaceTypes)(::windows_core::Interface::as_raw(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    pub unsafe fn Grouping(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Grouping)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGrouping<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetGrouping)(::windows_core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Profiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProfiles)(::windows_core::Interface::as_raw(self), profiletypesbitmask).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EdgeTraversal(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EdgeTraversal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEdgeTraversal<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetEdgeTraversal)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    pub unsafe fn Action(&self) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Action)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAction)(::windows_core::Interface::as_raw(self), action).ok()
    }
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EdgeTraversalOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEdgeTraversalOptions)(::windows_core::Interface::as_raw(self), loptions).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwRule2, ::windows_core::IUnknown, super::super::System::Com::IDispatch, INetFwRule);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwRule2 {
    type Vtable = INetFwRule2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwRule2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c27c8da_189b_4dde_89f7_8b39a316782c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule2_Vtbl {
    pub base__: INetFwRule_Vtbl,
    pub EdgeTraversalOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows_core::HRESULT,
    pub SetEdgeTraversalOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwRule3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule3 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, desc: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), desc.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplicationName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetApplicationName<P0>(&self, imagefilename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetApplicationName)(::windows_core::Interface::as_raw(self), imagefilename.into_param().abi()).ok()
    }
    pub unsafe fn ServiceName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ServiceName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServiceName<P0>(&self, servicename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetServiceName)(::windows_core::Interface::as_raw(self), servicename.into_param().abi()).ok()
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Protocol)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtocol(&self, protocol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProtocol)(::windows_core::Interface::as_raw(self), protocol).ok()
    }
    pub unsafe fn LocalPorts(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LocalPorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalPorts<P0>(&self, portnumbers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetLocalPorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn RemotePorts(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RemotePorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemotePorts<P0>(&self, portnumbers: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemotePorts)(::windows_core::Interface::as_raw(self), portnumbers.into_param().abi()).ok()
    }
    pub unsafe fn LocalAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LocalAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalAddresses<P0>(&self, localaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetLocalAddresses)(::windows_core::Interface::as_raw(self), localaddrs.into_param().abi()).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RemoteAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    pub unsafe fn IcmpTypesAndCodes(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIcmpTypesAndCodes<P0>(&self, icmptypesandcodes: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetIcmpTypesAndCodes)(::windows_core::Interface::as_raw(self), icmptypesandcodes.into_param().abi()).ok()
    }
    pub unsafe fn Direction(&self) -> ::windows_core::Result<NET_FW_RULE_DIRECTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Direction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDirection)(::windows_core::Interface::as_raw(self), dir).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Interfaces(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Interfaces)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetInterfaces(&self, interfaces: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetInterfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(interfaces)).ok()
    }
    pub unsafe fn InterfaceTypes(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InterfaceTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInterfaceTypes<P0>(&self, interfacetypes: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetInterfaceTypes)(::windows_core::Interface::as_raw(self), interfacetypes.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    pub unsafe fn Grouping(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Grouping)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGrouping<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetGrouping)(::windows_core::Interface::as_raw(self), context.into_param().abi()).ok()
    }
    pub unsafe fn Profiles(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Profiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProfiles(&self, profiletypesbitmask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProfiles)(::windows_core::Interface::as_raw(self), profiletypesbitmask).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EdgeTraversal(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EdgeTraversal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEdgeTraversal<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetEdgeTraversal)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    pub unsafe fn Action(&self) -> ::windows_core::Result<NET_FW_ACTION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Action)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAction(&self, action: NET_FW_ACTION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAction)(::windows_core::Interface::as_raw(self), action).ok()
    }
    pub unsafe fn EdgeTraversalOptions(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EdgeTraversalOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEdgeTraversalOptions(&self, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEdgeTraversalOptions)(::windows_core::Interface::as_raw(self), loptions).ok()
    }
    pub unsafe fn LocalAppPackageId(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalAppPackageId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalAppPackageId<P0>(&self, wszpackageid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalAppPackageId)(::windows_core::Interface::as_raw(self), wszpackageid.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserOwner(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalUserOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalUserOwner<P0>(&self, wszuserowner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalUserOwner)(::windows_core::Interface::as_raw(self), wszuserowner.into_param().abi()).ok()
    }
    pub unsafe fn LocalUserAuthorizedList(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalUserAuthorizedList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocalUserAuthorizedList<P0>(&self, wszuserauthlist: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLocalUserAuthorizedList)(::windows_core::Interface::as_raw(self), wszuserauthlist.into_param().abi()).ok()
    }
    pub unsafe fn RemoteUserAuthorizedList(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteUserAuthorizedList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteUserAuthorizedList<P0>(&self, wszuserauthlist: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteUserAuthorizedList)(::windows_core::Interface::as_raw(self), wszuserauthlist.into_param().abi()).ok()
    }
    pub unsafe fn RemoteMachineAuthorizedList(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteMachineAuthorizedList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteMachineAuthorizedList<P0>(&self, wszuserauthlist: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteMachineAuthorizedList)(::windows_core::Interface::as_raw(self), wszuserauthlist.into_param().abi()).ok()
    }
    pub unsafe fn SecureFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SecureFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSecureFlags(&self, loptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecureFlags)(::windows_core::Interface::as_raw(self), loptions).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwRule3, ::windows_core::IUnknown, super::super::System::Com::IDispatch, INetFwRule, INetFwRule2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwRule3 {
    type Vtable = INetFwRule3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwRule3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb21563ff_d696_4222_ab46_4e89b73ab34a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRule3_Vtbl {
    pub base__: INetFwRule2_Vtbl,
    pub LocalAppPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpackageid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalAppPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpackageid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LocalUserOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserowner: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalUserOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserowner: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LocalUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLocalUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemoteUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemoteUserAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub RemoteMachineAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemoteMachineAuthorizedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuserauthlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SecureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: *mut i32) -> ::windows_core::HRESULT,
    pub SetSecureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwRules(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwRules {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, rule: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<INetFwRule>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), rule.into_param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, name: P0) -> ::windows_core::Result<INetFwRule>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwRules, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwRules {
    type Vtable = INetFwRules_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwRules {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c4c6277_5027_441e_afae_ca1f542da009);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwService(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwService {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<NET_FW_SERVICE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Customized(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Customized)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IpVersion(&self) -> ::windows_core::Result<NET_FW_IP_VERSION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IpVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIpVersion)(::windows_core::Interface::as_raw(self), ipversion).ok()
    }
    pub unsafe fn Scope(&self) -> ::windows_core::Result<NET_FW_SCOPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScope(&self, scope: NET_FW_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScope)(::windows_core::Interface::as_raw(self), scope).ok()
    }
    pub unsafe fn RemoteAddresses(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteAddresses)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddresses<P0>(&self, remoteaddrs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRemoteAddresses)(::windows_core::Interface::as_raw(self), remoteaddrs.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), enabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GloballyOpenPorts(&self) -> ::windows_core::Result<INetFwOpenPorts> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GloballyOpenPorts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwService, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwService {
    type Vtable = INetFwService_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79fd57c8_908e_4a36_9888_d5b3f0a444cf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwService_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Customized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customized: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Customized: usize,
    pub IpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub SetIpVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> ::windows_core::HRESULT,
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scope: NET_FW_SCOPE) -> ::windows_core::HRESULT,
    pub RemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRemoteAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteaddrs: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GloballyOpenPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, openports: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GloballyOpenPorts: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwServiceRestriction(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwServiceRestriction {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestrictService<P0, P1, P2, P3>(&self, servicename: P0, appname: P1, restrictservice: P2, servicesidrestricted: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).RestrictService)(::windows_core::Interface::as_raw(self), servicename.into_param().abi(), appname.into_param().abi(), restrictservice.into_param().abi(), servicesidrestricted.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceRestricted<P0, P1>(&self, servicename: P0, appname: P1) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ServiceRestricted)(::windows_core::Interface::as_raw(self), servicename.into_param().abi(), appname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rules(&self) -> ::windows_core::Result<INetFwRules> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Rules)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwServiceRestriction, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwServiceRestriction {
    type Vtable = INetFwServiceRestriction_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwServiceRestriction {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8267bbe3_f890_491c_b7b6_2db1ef0e5d2b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServiceRestriction_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RestrictService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, appname: ::std::mem::MaybeUninit<::windows_core::BSTR>, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestrictService: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, appname: ::std::mem::MaybeUninit<::windows_core::BSTR>, servicerestricted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceRestricted: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetFwServices(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetFwServices {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> ::windows_core::Result<INetFwService> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), svctype, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetFwServices, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetFwServices {
    type Vtable = INetFwServices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetFwServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79649bb4_903e_421b_94c9_79848e79f6ee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetFwServices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingConfiguration(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingConfiguration {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SharingEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SharingEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SharingConnectionType(&self) -> ::windows_core::Result<SHARINGCONNECTIONTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SharingConnectionType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisableSharing(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableSharing)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableSharing)(::windows_core::Interface::as_raw(self), r#type).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InternetFirewallEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InternetFirewallEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisableInternetFirewall(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisableInternetFirewall)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableInternetFirewall(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableInternetFirewall)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPortMappingCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_EnumPortMappings)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPortMapping<P0, P1>(&self, bstrname: P0, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: P1, etargettype: ICS_TARGETTYPE) -> ::windows_core::Result<INetSharingPortMapping>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddPortMapping)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi(), ucipprotocol, usexternalport, usinternalport, dwoptions, bstrtargetnameoripaddress.into_param().abi(), etargettype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemovePortMapping<P0>(&self, pmapping: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<INetSharingPortMapping>,
    {
        (::windows_core::Interface::vtable(self).RemovePortMapping)(::windows_core::Interface::as_raw(self), pmapping.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingConfiguration, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingConfiguration {
    type Vtable = INetSharingConfiguration_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b6_1cd3_11d1_b1c5_00805fc1270e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingConfiguration_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SharingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SharingEnabled: usize,
    pub SharingConnectionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> ::windows_core::HRESULT,
    pub DisableSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InternetFirewallEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InternetFirewallEnabled: usize,
    pub DisableInternetFirewall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableInternetFirewall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPortMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPortMappings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPortMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPortMapping: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemovePortMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmapping: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemovePortMapping: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingEveryConnectionCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingEveryConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingEveryConnectionCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingEveryConnectionCollection {
    type Vtable = INetSharingEveryConnectionCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingEveryConnectionCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33c4643c_7811_46fa_a89a_768597bd7223);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingEveryConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingManager {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SharingInstalled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SharingInstalled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPublicConnectionCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_EnumPublicConnections)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> ::windows_core::Result<INetSharingPrivateConnectionCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_EnumPrivateConnections)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_INetSharingConfigurationForINetConnection<P0>(&self, pnetconnection: P0) -> ::windows_core::Result<INetSharingConfiguration>
    where
        P0: ::windows_core::IntoParam<INetConnection>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_INetSharingConfigurationForINetConnection)(::windows_core::Interface::as_raw(self), pnetconnection.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumEveryConnection(&self) -> ::windows_core::Result<INetSharingEveryConnectionCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumEveryConnection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_NetConnectionProps<P0>(&self, pnetconnection: P0) -> ::windows_core::Result<INetConnectionProps>
    where
        P0: ::windows_core::IntoParam<INetConnection>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_NetConnectionProps)(::windows_core::Interface::as_raw(self), pnetconnection.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingManager, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingManager {
    type Vtable = INetSharingManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b7_1cd3_11d1_b1c5_00805fc1270e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SharingInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbinstalled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SharingInstalled: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPublicConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPublicConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_EnumPrivateConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_EnumPrivateConnections: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_INetSharingConfigurationForINetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetconnection: *mut ::core::ffi::c_void, ppnetsharingconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_INetSharingConfigurationForINetConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumEveryConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcoll: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumEveryConnection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_NetConnectionProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetconnection: *mut ::core::ffi::c_void, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_NetConnectionProps: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingPortMapping(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMapping {
    pub unsafe fn Disable(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disable)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enable(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows_core::Result<INetSharingPortMappingProps> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Properties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingPortMapping, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingPortMapping {
    type Vtable = INetSharingPortMapping_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingPortMapping {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc08956b1_1cd3_11d1_b1c5_00805fc1270e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnspmp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingPortMappingCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingPortMappingCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingPortMappingCollection {
    type Vtable = INetSharingPortMappingCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingPortMappingCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02e4a2de_da20_4e34_89c8_ac22275a010b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingPortMappingProps(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingProps {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IPProtocol(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IPProtocol)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExternalPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InternalPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Options(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Options)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TargetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TargetIPAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TargetIPAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingPortMappingProps, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingPortMappingProps {
    type Vtable = INetSharingPortMappingProps_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingPortMappingProps {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24b7e9b5_e38f_4685_851b_00892cf5f940);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPortMappingProps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IPProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucipprot: *mut u8) -> ::windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusport: *mut i32) -> ::windows_core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwoptions: *mut i32) -> ::windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TargetIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetipaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingPrivateConnectionCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPrivateConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingPrivateConnectionCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingPrivateConnectionCollection {
    type Vtable = INetSharingPrivateConnectionCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingPrivateConnectionCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38ae69e0_4409_402a_a2cb_e965c727f840);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPrivateConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetSharingPublicConnectionCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPublicConnectionCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(INetSharingPublicConnectionCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for INetSharingPublicConnectionCollection {
    type Vtable = INetSharingPublicConnectionCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for INetSharingPublicConnectionCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d7a6355_f372_4971_a149_bfc927be762a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetSharingPublicConnectionCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStaticPortMapping(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMapping {
    pub unsafe fn ExternalIPAddress(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExternalIPAddress)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExternalPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InternalPort(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InternalPort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Protocol(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InternalClient(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InternalClient)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EditInternalClient<P0>(&self, bstrinternalclient: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EditInternalClient)(::windows_core::Interface::as_raw(self), bstrinternalclient.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, vb: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Enable)(::windows_core::Interface::as_raw(self), vb.into_param().abi()).ok()
    }
    pub unsafe fn EditDescription<P0>(&self, bstrdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).EditDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn EditInternalPort(&self, linternalport: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EditInternalPort)(::windows_core::Interface::as_raw(self), linternalport).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IStaticPortMapping, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IStaticPortMapping {
    type Vtable = IStaticPortMapping_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IStaticPortMapping {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f10711f_729b_41e5_93b8_f21d0f818df1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMapping_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExternalIPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub InternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub InternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EditInternalClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
    pub EditDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub EditInternalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linternalport: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStaticPortMappingCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMappingCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item<P0>(&self, lexternalport: i32, bstrprotocol: P0) -> ::windows_core::Result<IStaticPortMapping>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lexternalport, bstrprotocol.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0>(&self, lexternalport: i32, bstrprotocol: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), lexternalport, bstrprotocol.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<P0, P1, P2, P3>(&self, lexternalport: i32, bstrprotocol: P0, linternalport: i32, bstrinternalclient: P1, benabled: P2, bstrdescription: P3) -> ::windows_core::Result<IStaticPortMapping>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), lexternalport, bstrprotocol.into_param().abi(), linternalport, bstrinternalclient.into_param().abi(), benabled.into_param().abi(), bstrdescription.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IStaticPortMappingCollection, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IStaticPortMappingCollection {
    type Vtable = IStaticPortMappingCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IStaticPortMappingCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd1f3e77_66d6_4664_82c7_36dbb641d0f1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStaticPortMappingCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppspm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexternalport: i32, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternalport: i32, bstrinternalclient: ::std::mem::MaybeUninit<::windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppspm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUPnPNAT(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUPnPNAT {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StaticPortMappingCollection(&self) -> ::windows_core::Result<IStaticPortMappingCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StaticPortMappingCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DynamicPortMappingCollection(&self) -> ::windows_core::Result<IDynamicPortMappingCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DynamicPortMappingCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NATEventManager(&self) -> ::windows_core::Result<INATEventManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NATEventManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IUPnPNAT, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUPnPNAT {
    type Vtable = IUPnPNAT_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IUPnPNAT {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb171c812_cc76_485a_94d8_b6b3a2794e99);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUPnPNAT_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub StaticPortMappingCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppspms: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StaticPortMappingCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DynamicPortMappingCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdpms: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DynamicPortMappingCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NATEventManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NATEventManager: usize,
}
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(3i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(2i32);
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(0i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(1i32);
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(2i32);
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(0i32);
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = SHARINGCONNECTION_ENUM_FLAGS(1i32);
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(1i32);
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = SHARINGCONNECTIONTYPE(0i32);
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = ICS_TARGETTYPE(1i32);
pub const ICSTT_NAME: ICS_TARGETTYPE = ICS_TARGETTYPE(0i32);
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(1i32);
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(2i32);
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(0i32);
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = INET_FIREWALL_AC_CHANGE_TYPE(3i32);
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(4i32);
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(0i32);
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = INET_FIREWALL_AC_CREATION_TYPE(1i32);
pub const NCCF_ALLOW_DUPLICATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2i32);
pub const NCCF_ALLOW_REMOVAL: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4i32);
pub const NCCF_ALLOW_RENAME: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8i32);
pub const NCCF_ALL_USERS: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1i32);
pub const NCCF_BLUETOOTH_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(983040i32);
pub const NCCF_BRANDED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(128i32);
pub const NCCF_BRIDGED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(512i32);
pub const NCCF_DEFAULT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(2048i32);
pub const NCCF_FIREWALLED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(1024i32);
pub const NCCF_HOMENET_CAPABLE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(4096i32);
pub const NCCF_HOSTED_NETWORK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(65536i32);
pub const NCCF_INCOMING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32i32);
pub const NCCF_LAN_MASK: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(15728640i32);
pub const NCCF_NONE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(0i32);
pub const NCCF_OUTGOING_ONLY: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(64i32);
pub const NCCF_QUARANTINED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(16384i32);
pub const NCCF_RESERVED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(32768i32);
pub const NCCF_SHARED: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(256i32);
pub const NCCF_SHARED_PRIVATE: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(8192i32);
pub const NCCF_VIRTUAL_STATION: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(131072i32);
pub const NCCF_WIFI_DIRECT: NETCON_CHARACTERISTIC_FLAGS = NETCON_CHARACTERISTIC_FLAGS(262144i32);
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(0i32);
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = NETCONMGR_ENUM_FLAGS(1i32);
pub const NCM_BRIDGE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(7i32);
pub const NCM_DIRECT: NETCON_MEDIATYPE = NETCON_MEDIATYPE(1i32);
pub const NCM_ISDN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(2i32);
pub const NCM_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(3i32);
pub const NCM_NONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(0i32);
pub const NCM_PHONE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(4i32);
pub const NCM_PPPOE: NETCON_MEDIATYPE = NETCON_MEDIATYPE(6i32);
pub const NCM_SHAREDACCESSHOST_LAN: NETCON_MEDIATYPE = NETCON_MEDIATYPE(8i32);
pub const NCM_SHAREDACCESSHOST_RAS: NETCON_MEDIATYPE = NETCON_MEDIATYPE(9i32);
pub const NCM_TUNNEL: NETCON_MEDIATYPE = NETCON_MEDIATYPE(5i32);
pub const NCS_ACTION_REQUIRED: NETCON_STATUS = NETCON_STATUS(13i32);
pub const NCS_ACTION_REQUIRED_RETRY: NETCON_STATUS = NETCON_STATUS(14i32);
pub const NCS_AUTHENTICATING: NETCON_STATUS = NETCON_STATUS(8i32);
pub const NCS_AUTHENTICATION_FAILED: NETCON_STATUS = NETCON_STATUS(10i32);
pub const NCS_AUTHENTICATION_SUCCEEDED: NETCON_STATUS = NETCON_STATUS(9i32);
pub const NCS_CONNECTED: NETCON_STATUS = NETCON_STATUS(2i32);
pub const NCS_CONNECTING: NETCON_STATUS = NETCON_STATUS(1i32);
pub const NCS_CONNECT_FAILED: NETCON_STATUS = NETCON_STATUS(15i32);
pub const NCS_CREDENTIALS_REQUIRED: NETCON_STATUS = NETCON_STATUS(12i32);
pub const NCS_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(0i32);
pub const NCS_DISCONNECTING: NETCON_STATUS = NETCON_STATUS(3i32);
pub const NCS_HARDWARE_DISABLED: NETCON_STATUS = NETCON_STATUS(5i32);
pub const NCS_HARDWARE_MALFUNCTION: NETCON_STATUS = NETCON_STATUS(6i32);
pub const NCS_HARDWARE_NOT_PRESENT: NETCON_STATUS = NETCON_STATUS(4i32);
pub const NCS_INVALID_ADDRESS: NETCON_STATUS = NETCON_STATUS(11i32);
pub const NCS_MEDIA_DISCONNECTED: NETCON_STATUS = NETCON_STATUS(7i32);
pub const NCT_BRIDGE: NETCON_TYPE = NETCON_TYPE(6i32);
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = NETCON_TYPE(0i32);
pub const NCT_INBOUND: NETCON_TYPE = NETCON_TYPE(1i32);
pub const NCT_INTERNET: NETCON_TYPE = NETCON_TYPE(2i32);
pub const NCT_LAN: NETCON_TYPE = NETCON_TYPE(3i32);
pub const NCT_PHONE: NETCON_TYPE = NETCON_TYPE(4i32);
pub const NCT_TUNNEL: NETCON_TYPE = NETCON_TYPE(5i32);
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(0i32);
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(2i32);
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = NETCONUI_CONNECT_FLAGS(1i32);
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(2i32);
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(3i32);
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(4i32);
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(0i32);
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = NETISO_ERROR_TYPE(1i32);
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = NETISO_FLAG(1i32);
pub const NETISO_FLAG_MAX: NETISO_FLAG = NETISO_FLAG(2i32);
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = NET_FW_ACTION(1i32);
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = NET_FW_ACTION(0i32);
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = NET_FW_ACTION(2i32);
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(4i32);
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(3i32);
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(0i32);
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(1i32);
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = NET_FW_AUTHENTICATE_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(1i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(2i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(3i32);
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = NET_FW_EDGE_TRAVERSAL_TYPE(0i32);
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(256i32);
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(6i32);
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = NET_FW_IP_PROTOCOL(17i32);
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = NET_FW_IP_VERSION(2i32);
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = NET_FW_IP_VERSION(3i32);
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = NET_FW_IP_VERSION(0i32);
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = NET_FW_IP_VERSION(1i32);
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(1i32);
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(2i32);
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = NET_FW_MODIFY_STATE(0i32);
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(2i32);
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(0i32);
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(1i32);
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = NET_FW_POLICY_TYPE(3i32);
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2147483647i32);
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(1i32);
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(2i32);
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = NET_FW_PROFILE_TYPE2(4i32);
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(2i32);
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(0i32);
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(1i32);
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = NET_FW_PROFILE_TYPE(3i32);
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(0i32);
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(3i32);
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(2i32);
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(4i32);
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = NET_FW_RULE_CATEGORY(1i32);
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(1i32);
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(3i32);
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = NET_FW_RULE_DIRECTION(2i32);
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = NET_FW_SCOPE(0i32);
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = NET_FW_SCOPE(2i32);
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = NET_FW_SCOPE(1i32);
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = NET_FW_SCOPE(3i32);
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(0i32);
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(3i32);
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(2i32);
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(4i32);
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = NET_FW_SERVICE_TYPE(1i32);
pub const NetFwAuthorizedApplication: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
pub const NetSharingManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c63c1ad_3956_4ff8_8486_40034758315b);
pub const S_OBJECT_NO_LONGER_VALID: ::windows_core::HRESULT = ::windows_core::HRESULT(2i32);
pub const UPnPNAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae1e00aa_3fd5_403c_8a27_2bbdc30cd0e1);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(pub i32);
impl ::core::marker::Copy for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {}
impl ::core::clone::Clone for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS").field(&self.0).finish()
    }
}
impl FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(pub i32);
impl ::core::marker::Copy for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {}
impl ::core::clone::Clone for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS").field(&self.0).finish()
    }
}
impl FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(pub i32);
impl ::core::marker::Copy for FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {}
impl ::core::clone::Clone for FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FW_DYNAMIC_KEYWORD_ORIGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FW_DYNAMIC_KEYWORD_ORIGIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ICS_TARGETTYPE(pub i32);
impl ::core::marker::Copy for ICS_TARGETTYPE {}
impl ::core::clone::Clone for ICS_TARGETTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICS_TARGETTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ICS_TARGETTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ICS_TARGETTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICS_TARGETTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INET_FIREWALL_AC_CHANGE_TYPE(pub i32);
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE_TYPE {}
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INET_FIREWALL_AC_CHANGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INET_FIREWALL_AC_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INET_FIREWALL_AC_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INET_FIREWALL_AC_CREATION_TYPE(pub i32);
impl ::core::marker::Copy for INET_FIREWALL_AC_CREATION_TYPE {}
impl ::core::clone::Clone for INET_FIREWALL_AC_CREATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INET_FIREWALL_AC_CREATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INET_FIREWALL_AC_CREATION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INET_FIREWALL_AC_CREATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INET_FIREWALL_AC_CREATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETCONMGR_ENUM_FLAGS(pub i32);
impl ::core::marker::Copy for NETCONMGR_ENUM_FLAGS {}
impl ::core::clone::Clone for NETCONMGR_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCONMGR_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETCONMGR_ENUM_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETCONMGR_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCONMGR_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETCONUI_CONNECT_FLAGS(pub i32);
impl ::core::marker::Copy for NETCONUI_CONNECT_FLAGS {}
impl ::core::clone::Clone for NETCONUI_CONNECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCONUI_CONNECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETCONUI_CONNECT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETCONUI_CONNECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCONUI_CONNECT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETCON_CHARACTERISTIC_FLAGS(pub i32);
impl ::core::marker::Copy for NETCON_CHARACTERISTIC_FLAGS {}
impl ::core::clone::Clone for NETCON_CHARACTERISTIC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_CHARACTERISTIC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETCON_CHARACTERISTIC_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETCON_CHARACTERISTIC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_CHARACTERISTIC_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETCON_MEDIATYPE(pub i32);
impl ::core::marker::Copy for NETCON_MEDIATYPE {}
impl ::core::clone::Clone for NETCON_MEDIATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_MEDIATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETCON_MEDIATYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETCON_MEDIATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_MEDIATYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETCON_STATUS(pub i32);
impl ::core::marker::Copy for NETCON_STATUS {}
impl ::core::clone::Clone for NETCON_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETCON_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETCON_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETCON_TYPE(pub i32);
impl ::core::marker::Copy for NETCON_TYPE {}
impl ::core::clone::Clone for NETCON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETCON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETCON_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETCON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETCON_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETISO_ERROR_TYPE(pub i32);
impl ::core::marker::Copy for NETISO_ERROR_TYPE {}
impl ::core::clone::Clone for NETISO_ERROR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETISO_ERROR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETISO_ERROR_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETISO_ERROR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETISO_ERROR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETISO_FLAG(pub i32);
impl ::core::marker::Copy for NETISO_FLAG {}
impl ::core::clone::Clone for NETISO_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETISO_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NETISO_FLAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NETISO_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETISO_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_ACTION(pub i32);
impl ::core::marker::Copy for NET_FW_ACTION {}
impl ::core::clone::Clone for NET_FW_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_AUTHENTICATE_TYPE(pub i32);
impl ::core::marker::Copy for NET_FW_AUTHENTICATE_TYPE {}
impl ::core::clone::Clone for NET_FW_AUTHENTICATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_AUTHENTICATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_AUTHENTICATE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_AUTHENTICATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_AUTHENTICATE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(pub i32);
impl ::core::marker::Copy for NET_FW_EDGE_TRAVERSAL_TYPE {}
impl ::core::clone::Clone for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_EDGE_TRAVERSAL_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_EDGE_TRAVERSAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_EDGE_TRAVERSAL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_IP_PROTOCOL(pub i32);
impl ::core::marker::Copy for NET_FW_IP_PROTOCOL {}
impl ::core::clone::Clone for NET_FW_IP_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_IP_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_IP_PROTOCOL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_IP_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_IP_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_IP_VERSION(pub i32);
impl ::core::marker::Copy for NET_FW_IP_VERSION {}
impl ::core::clone::Clone for NET_FW_IP_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_IP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_IP_VERSION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_IP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_IP_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_MODIFY_STATE(pub i32);
impl ::core::marker::Copy for NET_FW_MODIFY_STATE {}
impl ::core::clone::Clone for NET_FW_MODIFY_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_MODIFY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_MODIFY_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_MODIFY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_MODIFY_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_POLICY_TYPE(pub i32);
impl ::core::marker::Copy for NET_FW_POLICY_TYPE {}
impl ::core::clone::Clone for NET_FW_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_POLICY_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_POLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_PROFILE_TYPE(pub i32);
impl ::core::marker::Copy for NET_FW_PROFILE_TYPE {}
impl ::core::clone::Clone for NET_FW_PROFILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_PROFILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_PROFILE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_PROFILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_PROFILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_PROFILE_TYPE2(pub i32);
impl ::core::marker::Copy for NET_FW_PROFILE_TYPE2 {}
impl ::core::clone::Clone for NET_FW_PROFILE_TYPE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_PROFILE_TYPE2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_PROFILE_TYPE2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_PROFILE_TYPE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_PROFILE_TYPE2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_RULE_CATEGORY(pub i32);
impl ::core::marker::Copy for NET_FW_RULE_CATEGORY {}
impl ::core::clone::Clone for NET_FW_RULE_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_RULE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_RULE_CATEGORY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_RULE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_RULE_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_RULE_DIRECTION(pub i32);
impl ::core::marker::Copy for NET_FW_RULE_DIRECTION {}
impl ::core::clone::Clone for NET_FW_RULE_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_RULE_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_RULE_DIRECTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_RULE_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_RULE_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_SCOPE(pub i32);
impl ::core::marker::Copy for NET_FW_SCOPE {}
impl ::core::clone::Clone for NET_FW_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_SCOPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_FW_SERVICE_TYPE(pub i32);
impl ::core::marker::Copy for NET_FW_SERVICE_TYPE {}
impl ::core::clone::Clone for NET_FW_SERVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_FW_SERVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NET_FW_SERVICE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NET_FW_SERVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_FW_SERVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHARINGCONNECTIONTYPE(pub i32);
impl ::core::marker::Copy for SHARINGCONNECTIONTYPE {}
impl ::core::clone::Clone for SHARINGCONNECTIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARINGCONNECTIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SHARINGCONNECTIONTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SHARINGCONNECTIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARINGCONNECTIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHARINGCONNECTION_ENUM_FLAGS(pub i32);
impl ::core::marker::Copy for SHARINGCONNECTION_ENUM_FLAGS {}
impl ::core::clone::Clone for SHARINGCONNECTION_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHARINGCONNECTION_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SHARINGCONNECTION_ENUM_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SHARINGCONNECTION_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHARINGCONNECTION_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: ::windows_core::GUID,
    pub keyword: ::windows_core::PCWSTR,
    pub flags: u32,
    pub addresses: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for FW_DYNAMIC_KEYWORD_ADDRESS0 {}
impl ::core::clone::Clone for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FW_DYNAMIC_KEYWORD_ADDRESS0").field("id", &self.id).field("keyword", &self.keyword).field("flags", &self.flags).field("addresses", &self.addresses).finish()
    }
}
impl ::windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.keyword == other.keyword && self.flags == other.flags && self.addresses == other.addresses
    }
}
impl ::core::cmp::Eq for FW_DYNAMIC_KEYWORD_ADDRESS0 {}
impl ::core::default::Default for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
impl ::core::marker::Copy for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
impl ::core::clone::Clone for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FW_DYNAMIC_KEYWORD_ADDRESS_DATA0").field("dynamicKeywordAddress", &self.dynamicKeywordAddress).field("next", &self.next).field("schemaVersion", &self.schemaVersion).field("originType", &self.originType).finish()
    }
}
impl ::windows_core::TypeKind for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        self.dynamicKeywordAddress == other.dynamicKeywordAddress && self.next == other.next && self.schemaVersion == other.schemaVersion && self.originType == other.originType
    }
}
impl ::core::cmp::Eq for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
impl ::core::default::Default for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut ::windows_core::PWSTR,
}
impl ::core::marker::Copy for INET_FIREWALL_AC_BINARIES {}
impl ::core::clone::Clone for INET_FIREWALL_AC_BINARIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INET_FIREWALL_AC_BINARIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_AC_BINARIES").field("count", &self.count).field("binaries", &self.binaries).finish()
    }
}
impl ::windows_core::TypeKind for INET_FIREWALL_AC_BINARIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_BINARIES {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.binaries == other.binaries
    }
}
impl ::core::cmp::Eq for INET_FIREWALL_AC_BINARIES {}
impl ::core::default::Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for INET_FIREWALL_AC_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_AC_CAPABILITIES").field("count", &self.count).field("capabilities", &self.capabilities).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::TypeKind for INET_FIREWALL_AC_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_AC_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.capabilities == other.capabilities
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub displayName: ::windows_core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::TypeKind for INET_FIREWALL_AC_CHANGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::TypeKind for INET_FIREWALL_AC_CHANGE_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub appContainerName: ::windows_core::PWSTR,
    pub displayName: ::windows_core::PWSTR,
    pub description: ::windows_core::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: ::windows_core::PWSTR,
    pub packageFullName: ::windows_core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_APP_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for INET_FIREWALL_APP_CONTAINER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INET_FIREWALL_APP_CONTAINER").field("appContainerSid", &self.appContainerSid).field("userSid", &self.userSid).field("appContainerName", &self.appContainerName).field("displayName", &self.displayName).field("description", &self.description).field("capabilities", &self.capabilities).field("binaries", &self.binaries).field("workingDirectory", &self.workingDirectory).field("packageFullName", &self.packageFullName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows_core::TypeKind for INET_FIREWALL_APP_CONTAINER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for INET_FIREWALL_APP_CONTAINER {
    fn eq(&self, other: &Self) -> bool {
        self.appContainerSid == other.appContainerSid && self.userSid == other.userSid && self.appContainerName == other.appContainerName && self.displayName == other.displayName && self.description == other.description && self.capabilities == other.capabilities && self.binaries == other.binaries && self.workingDirectory == other.workingDirectory && self.packageFullName == other.packageFullName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NETCON_PROPERTIES {
    pub guidId: ::windows_core::GUID,
    pub pszwName: ::windows_core::PWSTR,
    pub pszwDeviceName: ::windows_core::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: ::windows_core::GUID,
    pub clsidUiObject: ::windows_core::GUID,
}
impl ::core::marker::Copy for NETCON_PROPERTIES {}
impl ::core::clone::Clone for NETCON_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETCON_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETCON_PROPERTIES").field("guidId", &self.guidId).field("pszwName", &self.pszwName).field("pszwDeviceName", &self.pszwDeviceName).field("Status", &self.Status).field("MediaType", &self.MediaType).field("dwCharacter", &self.dwCharacter).field("clsidThisObject", &self.clsidThisObject).field("clsidUiObject", &self.clsidUiObject).finish()
    }
}
impl ::windows_core::TypeKind for NETCON_PROPERTIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NETCON_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.guidId == other.guidId && self.pszwName == other.pszwName && self.pszwDeviceName == other.pszwDeviceName && self.Status == other.Status && self.MediaType == other.MediaType && self.dwCharacter == other.dwCharacter && self.clsidThisObject == other.clsidThisObject && self.clsidUiObject == other.clsidUiObject
    }
}
impl ::core::cmp::Eq for NETCON_PROPERTIES {}
impl ::core::default::Default for NETCON_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PAC_CHANGES_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddress: *const FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_core::GUID) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_core::GUID, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = ::core::option::Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = ::core::option::Option<unsafe extern "system" fn(dynamickeywordaddressid: ::windows_core::GUID, updatedaddresses: ::windows_core::PCWSTR, append: super::super::Foundation::BOOL) -> u32>;
pub type PNETISO_EDP_ID_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, wszenterpriseid: ::windows_core::PCWSTR, dwerr: u32)>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
