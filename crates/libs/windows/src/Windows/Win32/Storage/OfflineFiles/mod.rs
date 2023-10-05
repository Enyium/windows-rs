#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OfflineFilesEnable<P0>(benable: P0, pbrebootrequired: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("cscapi.dll" "system" fn OfflineFilesEnable(benable : super::super::Foundation:: BOOL, pbrebootrequired : *mut super::super::Foundation:: BOOL) -> u32);
    OfflineFilesEnable(benable.into_param().abi(), pbrebootrequired)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OfflineFilesQueryStatus(pbactive: ::core::option::Option<*mut super::super::Foundation::BOOL>, pbenabled: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> u32 {
    ::windows_targets::link!("cscapi.dll" "system" fn OfflineFilesQueryStatus(pbactive : *mut super::super::Foundation:: BOOL, pbenabled : *mut super::super::Foundation:: BOOL) -> u32);
    OfflineFilesQueryStatus(::core::mem::transmute(pbactive.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbenabled.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OfflineFilesQueryStatusEx(pbactive: ::core::option::Option<*mut super::super::Foundation::BOOL>, pbenabled: ::core::option::Option<*mut super::super::Foundation::BOOL>, pbavailable: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> u32 {
    ::windows_targets::link!("cscapi.dll" "system" fn OfflineFilesQueryStatusEx(pbactive : *mut super::super::Foundation:: BOOL, pbenabled : *mut super::super::Foundation:: BOOL, pbavailable : *mut super::super::Foundation:: BOOL) -> u32);
    OfflineFilesQueryStatusEx(::core::mem::transmute(pbactive.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbenabled.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbavailable.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn OfflineFilesStart() -> u32 {
    ::windows_targets::link!("cscapi.dll" "system" fn OfflineFilesStart() -> u32);
    OfflineFilesStart()
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumOfflineFilesItems(::windows_core::IUnknown);
impl IEnumOfflineFilesItems {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IOfflineFilesItem>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumOfflineFilesItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumOfflineFilesItems, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumOfflineFilesItems {
    type Vtable = IEnumOfflineFilesItems_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumOfflineFilesItems {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda70e815_c361_4407_bc0b_0d7046e5f2cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOfflineFilesItems_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnumOfflineFilesSettings(::windows_core::IUnknown);
impl IEnumOfflineFilesSettings {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IOfflineFilesSetting>], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumOfflineFilesSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumOfflineFilesSettings, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnumOfflineFilesSettings {
    type Vtable = IEnumOfflineFilesSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnumOfflineFilesSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x729680c4_1a38_47bc_9e5c_02c51562ac30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOfflineFilesSettings_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesCache(::windows_core::IUnknown);
impl IOfflineFilesCache {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Synchronize<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows_core::PCWSTR], basync: P1, dwsynccontrol: u32, pisyncconflicthandler: P2, piprogress: P3, psyncid: ::core::option::Option<*const ::windows_core::GUID>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IOfflineFilesSyncConflictHandler>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).Synchronize)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, basync.into_param().abi(), dwsynccontrol, pisyncconflicthandler.into_param().abi(), piprogress.into_param().abi(), ::core::mem::transmute(psyncid.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteItems<P0, P1>(&self, rgpszpaths: &[::windows_core::PCWSTR], dwflags: u32, basync: P0, piprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IOfflineFilesSimpleProgress>,
    {
        (::windows_core::Interface::vtable(self).DeleteItems)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, dwflags, basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteItemsForUser<P0, P1, P2>(&self, pszuser: P0, rgpszpaths: &[::windows_core::PCWSTR], dwflags: u32, basync: P1, piprogress: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IOfflineFilesSimpleProgress>,
    {
        (::windows_core::Interface::vtable(self).DeleteItemsForUser)(::windows_core::Interface::as_raw(self), pszuser.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, dwflags, basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Pin<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows_core::PCWSTR], bdeep: P1, basync: P2, dwpincontrolflags: u32, piprogress: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).Pin)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, bdeep.into_param().abi(), basync.into_param().abi(), dwpincontrolflags, piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unpin<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows_core::PCWSTR], bdeep: P1, basync: P2, dwpincontrolflags: u32, piprogress: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).Unpin)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, bdeep.into_param().abi(), basync.into_param().abi(), dwpincontrolflags, piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncryptionStatus(&self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEncryptionStatus)(::windows_core::Interface::as_raw(self), pbencrypted, pbpartial).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Encrypt<P0, P1, P2, P3>(&self, hwndparent: P0, bencrypt: P1, dwencryptioncontrolflags: u32, basync: P2, piprogress: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).Encrypt)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), bencrypt.into_param().abi(), dwencryptioncontrolflags, basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    pub unsafe fn FindItem<P0>(&self, pszpath: P0, dwqueryflags: u32) -> ::windows_core::Result<IOfflineFilesItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindItem)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), dwqueryflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindItemEx<P0, P1, P2, P3, P4>(&self, pszpath: P0, pincludefilefilter: P1, pincludedirfilter: P2, pexcludefilefilter: P3, pexcludedirfilter: P4, dwqueryflags: u32) -> ::windows_core::Result<IOfflineFilesItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P2: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P3: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P4: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindItemEx)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pincludefilefilter.into_param().abi(), pincludedirfilter.into_param().abi(), pexcludefilefilter.into_param().abi(), pexcludedirfilter.into_param().abi(), dwqueryflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameItem<P0, P1, P2>(&self, pszpathoriginal: P0, pszpathnew: P1, breplaceifexists: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).RenameItem)(::windows_core::Interface::as_raw(self), pszpathoriginal.into_param().abi(), pszpathnew.into_param().abi(), breplaceifexists.into_param().abi()).ok()
    }
    pub unsafe fn GetLocation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLocation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDiskSpaceInformation)(::windows_core::Interface::as_raw(self), pcbvolumetotal, pcblimit, pcbused, pcbunpinnedlimit, pcbunpinnedused).ok()
    }
    pub unsafe fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDiskSpaceLimits)(::windows_core::Interface::as_raw(self), cblimit, cbunpinnedlimit).ok()
    }
    pub unsafe fn ProcessAdminPinPolicy<P0, P1>(&self, ppinprogress: P0, punpinprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
        P1: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).ProcessAdminPinPolicy)(::windows_core::Interface::as_raw(self), ppinprogress.into_param().abi(), punpinprogress.into_param().abi()).ok()
    }
    pub unsafe fn GetSettingObject<P0>(&self, pszsettingname: P0) -> ::windows_core::Result<IOfflineFilesSetting>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingObject)(::windows_core::Interface::as_raw(self), pszsettingname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumSettingObjects(&self) -> ::windows_core::Result<IEnumOfflineFilesSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumSettingObjects)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathCacheable<P0>(&self, pszpath: P0, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).IsPathCacheable)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pbcacheable, psharecachingmode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesCache, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesCache {
    type Vtable = IOfflineFilesCache_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesCache {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x855d6203_7914_48b9_8d40_4c56f5acffc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesCache_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Synchronize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: *mut ::core::ffi::c_void, piprogress: *mut ::core::ffi::c_void, psyncid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Synchronize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteItems: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteItemsForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszuser: ::windows_core::PCWSTR, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteItemsForUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Pin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Pin: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Unpin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const ::windows_core::PCWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unpin: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEncryptionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEncryptionStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Encrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Encrypt: usize,
    pub FindItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, dwqueryflags: u32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindItemEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pincludefilefilter: *mut ::core::ffi::c_void, pincludedirfilter: *mut ::core::ffi::c_void, pexcludefilefilter: *mut ::core::ffi::c_void, pexcludedirfilter: *mut ::core::ffi::c_void, dwqueryflags: u32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RenameItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpathoriginal: ::windows_core::PCWSTR, pszpathnew: ::windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenameItem: usize,
    pub GetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetDiskSpaceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows_core::HRESULT,
    pub SetDiskSpaceLimits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> ::windows_core::HRESULT,
    pub ProcessAdminPinPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinprogress: *mut ::core::ffi::c_void, punpinprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSettingObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsettingname: ::windows_core::PCWSTR, ppsetting: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumSettingObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathCacheable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathCacheable: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesCache2(::windows_core::IUnknown);
impl IOfflineFilesCache2 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Synchronize<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows_core::PCWSTR], basync: P1, dwsynccontrol: u32, pisyncconflicthandler: P2, piprogress: P3, psyncid: ::core::option::Option<*const ::windows_core::GUID>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IOfflineFilesSyncConflictHandler>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Synchronize)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, basync.into_param().abi(), dwsynccontrol, pisyncconflicthandler.into_param().abi(), piprogress.into_param().abi(), ::core::mem::transmute(psyncid.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteItems<P0, P1>(&self, rgpszpaths: &[::windows_core::PCWSTR], dwflags: u32, basync: P0, piprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IOfflineFilesSimpleProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.DeleteItems)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, dwflags, basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteItemsForUser<P0, P1, P2>(&self, pszuser: P0, rgpszpaths: &[::windows_core::PCWSTR], dwflags: u32, basync: P1, piprogress: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IOfflineFilesSimpleProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.DeleteItemsForUser)(::windows_core::Interface::as_raw(self), pszuser.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, dwflags, basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Pin<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows_core::PCWSTR], bdeep: P1, basync: P2, dwpincontrolflags: u32, piprogress: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Pin)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, bdeep.into_param().abi(), basync.into_param().abi(), dwpincontrolflags, piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unpin<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows_core::PCWSTR], bdeep: P1, basync: P2, dwpincontrolflags: u32, piprogress: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Unpin)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, bdeep.into_param().abi(), basync.into_param().abi(), dwpincontrolflags, piprogress.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncryptionStatus(&self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetEncryptionStatus)(::windows_core::Interface::as_raw(self), pbencrypted, pbpartial).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Encrypt<P0, P1, P2, P3>(&self, hwndparent: P0, bencrypt: P1, dwencryptioncontrolflags: u32, basync: P2, piprogress: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.Encrypt)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), bencrypt.into_param().abi(), dwencryptioncontrolflags, basync.into_param().abi(), piprogress.into_param().abi()).ok()
    }
    pub unsafe fn FindItem<P0>(&self, pszpath: P0, dwqueryflags: u32) -> ::windows_core::Result<IOfflineFilesItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FindItem)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), dwqueryflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindItemEx<P0, P1, P2, P3, P4>(&self, pszpath: P0, pincludefilefilter: P1, pincludedirfilter: P2, pexcludefilefilter: P3, pexcludedirfilter: P4, dwqueryflags: u32) -> ::windows_core::Result<IOfflineFilesItem>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P2: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P3: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P4: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FindItemEx)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pincludefilefilter.into_param().abi(), pincludedirfilter.into_param().abi(), pexcludefilefilter.into_param().abi(), pexcludedirfilter.into_param().abi(), dwqueryflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameItem<P0, P1, P2>(&self, pszpathoriginal: P0, pszpathnew: P1, breplaceifexists: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.RenameItem)(::windows_core::Interface::as_raw(self), pszpathoriginal.into_param().abi(), pszpathnew.into_param().abi(), breplaceifexists.into_param().abi()).ok()
    }
    pub unsafe fn GetLocation(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLocation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDiskSpaceInformation)(::windows_core::Interface::as_raw(self), pcbvolumetotal, pcblimit, pcbused, pcbunpinnedlimit, pcbunpinnedused).ok()
    }
    pub unsafe fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDiskSpaceLimits)(::windows_core::Interface::as_raw(self), cblimit, cbunpinnedlimit).ok()
    }
    pub unsafe fn ProcessAdminPinPolicy<P0, P1>(&self, ppinprogress: P0, punpinprogress: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
        P1: ::windows_core::IntoParam<IOfflineFilesSyncProgress>,
    {
        (::windows_core::Interface::vtable(self).base__.ProcessAdminPinPolicy)(::windows_core::Interface::as_raw(self), ppinprogress.into_param().abi(), punpinprogress.into_param().abi()).ok()
    }
    pub unsafe fn GetSettingObject<P0>(&self, pszsettingname: P0) -> ::windows_core::Result<IOfflineFilesSetting>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSettingObject)(::windows_core::Interface::as_raw(self), pszsettingname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumSettingObjects(&self) -> ::windows_core::Result<IEnumOfflineFilesSettings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EnumSettingObjects)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathCacheable<P0>(&self, pszpath: P0, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.IsPathCacheable)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), pbcacheable, psharecachingmode).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameItemEx<P0, P1, P2>(&self, pszpathoriginal: P0, pszpathnew: P1, breplaceifexists: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).RenameItemEx)(::windows_core::Interface::as_raw(self), pszpathoriginal.into_param().abi(), pszpathnew.into_param().abi(), breplaceifexists.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesCache2, ::windows_core::IUnknown, IOfflineFilesCache);
unsafe impl ::windows_core::Interface for IOfflineFilesCache2 {
    type Vtable = IOfflineFilesCache2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesCache2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c075039_1551_4ed9_8781_56705c04d3c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesCache2_Vtbl {
    pub base__: IOfflineFilesCache_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RenameItemEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpathoriginal: ::windows_core::PCWSTR, pszpathnew: ::windows_core::PCWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RenameItemEx: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesChangeInfo(::windows_core::IUnknown);
impl IOfflineFilesChangeInfo {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDirty(&self, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).IsDirty)(::windows_core::Interface::as_raw(self), pbdirty)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDeletedOffline(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsDeletedOffline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCreatedOffline(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsCreatedOffline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLocallyModifiedData(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsLocallyModifiedData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLocallyModifiedAttributes(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsLocallyModifiedAttributes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLocallyModifiedTime(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsLocallyModifiedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesChangeInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesChangeInfo {
    type Vtable = IOfflineFilesChangeInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesChangeInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa96e6fa4_e0d1_4c29_960b_ee508fe68c72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesChangeInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDirty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDeletedOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDeletedOffline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCreatedOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCreatedOffline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocallyModifiedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocallyModifiedData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocallyModifiedAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocallyModifiedAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocallyModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocallyModifiedTime: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesConnectionInfo(::windows_core::IUnknown);
impl IOfflineFilesConnectionInfo {
    pub unsafe fn GetConnectState(&self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConnectState)(::windows_core::Interface::as_raw(self), pconnectstate, pofflinereason).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectState<P0>(&self, hwndparent: P0, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).SetConnectState)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), dwflags, connectstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransitionOnline<P0>(&self, hwndparent: P0, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).TransitionOnline)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransitionOffline<P0, P1>(&self, hwndparent: P0, dwflags: u32, bforceopenfilesclosed: P1) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TransitionOffline)(::windows_core::Interface::as_raw(self), hwndparent.into_param().abi(), dwflags, bforceopenfilesclosed.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesConnectionInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesConnectionInfo {
    type Vtable = IOfflineFilesConnectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesConnectionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefb23a09_a867_4be8_83a6_86969a7d0856);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesConnectionInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetConnectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransitionOnline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransitionOnline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransitionOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransitionOffline: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesDirectoryItem(::windows_core::IUnknown);
impl IOfflineFilesDirectoryItem {
    pub unsafe fn GetItemType(&self) -> ::windows_core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetItemType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows_core::Result<IOfflineFilesItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetParentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsMarkedForDeletion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesDirectoryItem, ::windows_core::IUnknown, IOfflineFilesItem);
unsafe impl ::windows_core::Interface for IOfflineFilesDirectoryItem {
    type Vtable = IOfflineFilesDirectoryItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesDirectoryItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2273597a_a08c_4a00_a37a_c1ae4e9a1cfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesDirectoryItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesDirtyInfo(::windows_core::IUnknown);
impl IOfflineFilesDirtyInfo {
    pub unsafe fn LocalDirtyByteCount(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LocalDirtyByteCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoteDirtyByteCount(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RemoteDirtyByteCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesDirtyInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesDirtyInfo {
    type Vtable = IOfflineFilesDirtyInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesDirtyInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f50ce33_bac9_4eaa_a11d_da0e527d047d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesDirtyInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LocalDirtyByteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows_core::HRESULT,
    pub RemoteDirtyByteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesErrorInfo(::windows_core::IUnknown);
impl IOfflineFilesErrorInfo {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawData(&self) -> ::windows_core::Result<*mut super::super::System::Com::BYTE_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRawData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesErrorInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesErrorInfo {
    type Vtable = IOfflineFilesErrorInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesErrorInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7112fa5f_7571_435a_8eb7_195c7c1429bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesErrorInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRawData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRawData: usize,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesEvents(::windows_core::IUnknown);
impl IOfflineFilesEvents {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CacheMoved)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    pub unsafe fn CacheIsFull(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CacheIsFull)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CacheIsCorrupted)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EncryptionChanged<P0, P1, P2, P3>(&self, bwasencrypted: P0, bwaspartial: P1, bisencrypted: P2, bispartial: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).EncryptionChanged)(::windows_core::Interface::as_raw(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SyncBegin)(::windows_core::Interface::as_raw(self), rsyncid).ok()
    }
    pub unsafe fn SyncFileResult<P0>(&self, rsyncid: *const ::windows_core::GUID, pszfile: P0, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SyncFileResult)(::windows_core::Interface::as_raw(self), rsyncid, pszfile.into_param().abi(), hrresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SyncConflictRecAdded)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SyncConflictRecUpdated)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SyncConflictRecRemoved)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SyncEnd)(::windows_core::Interface::as_raw(self), rsyncid, hrresult).ok()
    }
    pub unsafe fn NetTransportArrived(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NetTransportArrived)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NoNetTransports(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NoNetTransports)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemDisconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemReconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemNotAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemNotPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ItemModified<P0, P1, P2>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).ItemModified)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype, bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemAddedToCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemDeletedFromCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ItemRenamed)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn DataLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DataLost)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Ping(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Ping)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesEvents {
    type Vtable = IOfflineFilesEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe25585c1_0caa_4eb1_873b_1cae5b77c314);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CacheMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszoldpath: ::windows_core::PCWSTR, psznewpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub CacheIsFull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CacheIsCorrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EncryptionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EncryptionChanged: usize,
    pub SyncBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SyncFileResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows_core::GUID, pszfile: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConflictRecAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszconflictpath: ::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConflictRecAdded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConflictRecUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszconflictpath: ::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConflictRecUpdated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConflictRecRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszconflictpath: ::windows_core::PCWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConflictRecRemoved: usize,
    pub SyncEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub NetTransportArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NoNetTransports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ItemDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub ItemReconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub ItemAvailableOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub ItemNotAvailableOffline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub ItemPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub ItemNotPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ItemModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ItemModified: usize,
    pub ItemAddedToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub ItemDeletedFromCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub ItemRenamed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszoldpath: ::windows_core::PCWSTR, psznewpath: ::windows_core::PCWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub DataLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Ping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesEvents2(::windows_core::IUnknown);
impl IOfflineFilesEvents2 {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.CacheMoved)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    pub unsafe fn CacheIsFull(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CacheIsFull)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CacheIsCorrupted)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Enabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EncryptionChanged<P0, P1, P2, P3>(&self, bwasencrypted: P0, bwaspartial: P1, bisencrypted: P2, bispartial: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.EncryptionChanged)(::windows_core::Interface::as_raw(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SyncBegin)(::windows_core::Interface::as_raw(self), rsyncid).ok()
    }
    pub unsafe fn SyncFileResult<P0>(&self, rsyncid: *const ::windows_core::GUID, pszfile: P0, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SyncFileResult)(::windows_core::Interface::as_raw(self), rsyncid, pszfile.into_param().abi(), hrresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SyncConflictRecAdded)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SyncConflictRecUpdated)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SyncConflictRecRemoved)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SyncEnd)(::windows_core::Interface::as_raw(self), rsyncid, hrresult).ok()
    }
    pub unsafe fn NetTransportArrived(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.NetTransportArrived)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NoNetTransports(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.NoNetTransports)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemDisconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemReconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemNotAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemNotPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ItemModified<P0, P1, P2>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemModified)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype, bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemAddedToCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemDeletedFromCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.ItemRenamed)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn DataLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DataLost)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Ping(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Ping)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ItemReconnectBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ItemReconnectEnd)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CacheEvictBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictEnd(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CacheEvictEnd)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackgroundSyncBegin)(::windows_core::Interface::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackgroundSyncEnd)(::windows_core::Interface::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PolicyChangeDetected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreferenceChangeDetected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SettingsChangesApplied)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesEvents2, ::windows_core::IUnknown, IOfflineFilesEvents);
unsafe impl ::windows_core::Interface for IOfflineFilesEvents2 {
    type Vtable = IOfflineFilesEvents2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesEvents2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ead8f56_ff76_4faa_a795_6f6ef792498b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents2_Vtbl {
    pub base__: IOfflineFilesEvents_Vtbl,
    pub ItemReconnectBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ItemReconnectEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CacheEvictBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CacheEvictEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BackgroundSyncBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows_core::HRESULT,
    pub BackgroundSyncEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows_core::HRESULT,
    pub PolicyChangeDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PreferenceChangeDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SettingsChangesApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesEvents3(::windows_core::IUnknown);
impl IOfflineFilesEvents3 {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CacheMoved)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    pub unsafe fn CacheIsFull(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CacheIsFull)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CacheIsCorrupted)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Enabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EncryptionChanged<P0, P1, P2, P3>(&self, bwasencrypted: P0, bwaspartial: P1, bisencrypted: P2, bispartial: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.EncryptionChanged)(::windows_core::Interface::as_raw(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SyncBegin)(::windows_core::Interface::as_raw(self), rsyncid).ok()
    }
    pub unsafe fn SyncFileResult<P0>(&self, rsyncid: *const ::windows_core::GUID, pszfile: P0, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SyncFileResult)(::windows_core::Interface::as_raw(self), rsyncid, pszfile.into_param().abi(), hrresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SyncConflictRecAdded)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SyncConflictRecUpdated)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SyncConflictRecRemoved)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SyncEnd)(::windows_core::Interface::as_raw(self), rsyncid, hrresult).ok()
    }
    pub unsafe fn NetTransportArrived(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.NetTransportArrived)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NoNetTransports(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.NoNetTransports)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemDisconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemReconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemNotAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemNotPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ItemModified<P0, P1, P2>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemModified)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype, bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemAddedToCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemDeletedFromCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.ItemRenamed)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn DataLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DataLost)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Ping(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Ping)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ItemReconnectBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ItemReconnectEnd)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CacheEvictBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictEnd(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CacheEvictEnd)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BackgroundSyncBegin)(::windows_core::Interface::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BackgroundSyncEnd)(::windows_core::Interface::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.PolicyChangeDetected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.PreferenceChangeDetected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SettingsChangesApplied)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransparentCacheItemNotify<P0, P1, P2, P3>(&self, pszpath: P0, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2, pzsoldpath: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).TransparentCacheItemNotify)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), eventtype, itemtype, bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi(), pzsoldpath.into_param().abi()).ok()
    }
    pub unsafe fn PrefetchFileBegin<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).PrefetchFileBegin)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn PrefetchFileEnd<P0>(&self, pszpath: P0, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).PrefetchFileEnd)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), hrresult).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesEvents3, ::windows_core::IUnknown, IOfflineFilesEvents, IOfflineFilesEvents2);
unsafe impl ::windows_core::Interface for IOfflineFilesEvents3 {
    type Vtable = IOfflineFilesEvents3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesEvents3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ba04a45_ee69_42f0_9ab1_7db5c8805808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents3_Vtbl {
    pub base__: IOfflineFilesEvents2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TransparentCacheItemNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransparentCacheItemNotify: usize,
    pub PrefetchFileBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub PrefetchFileEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesEvents4(::windows_core::IUnknown);
impl IOfflineFilesEvents4 {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CacheMoved)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi()).ok()
    }
    pub unsafe fn CacheIsFull(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CacheIsFull)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CacheIsCorrupted)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled<P0>(&self, benabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Enabled)(::windows_core::Interface::as_raw(self), benabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EncryptionChanged<P0, P1, P2, P3>(&self, bwasencrypted: P0, bwaspartial: P1, bisencrypted: P2, bispartial: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.EncryptionChanged)(::windows_core::Interface::as_raw(self), bwasencrypted.into_param().abi(), bwaspartial.into_param().abi(), bisencrypted.into_param().abi(), bispartial.into_param().abi()).ok()
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SyncBegin)(::windows_core::Interface::as_raw(self), rsyncid).ok()
    }
    pub unsafe fn SyncFileResult<P0>(&self, rsyncid: *const ::windows_core::GUID, pszfile: P0, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SyncFileResult)(::windows_core::Interface::as_raw(self), rsyncid, pszfile.into_param().abi(), hrresult).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SyncConflictRecAdded)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SyncConflictRecUpdated)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SyncConflictRecRemoved)(::windows_core::Interface::as_raw(self), pszconflictpath.into_param().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SyncEnd)(::windows_core::Interface::as_raw(self), rsyncid, hrresult).ok()
    }
    pub unsafe fn NetTransportArrived(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.NetTransportArrived)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NoNetTransports(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.NoNetTransports)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemDisconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemReconnected)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemNotAvailableOffline)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemNotPinned)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ItemModified<P0, P1, P2>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemModified)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype, bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi()).ok()
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemAddedToCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemDeletedFromCache)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ItemRenamed)(::windows_core::Interface::as_raw(self), pszoldpath.into_param().abi(), psznewpath.into_param().abi(), itemtype).ok()
    }
    pub unsafe fn DataLost(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.DataLost)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Ping(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Ping)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ItemReconnectBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ItemReconnectEnd)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CacheEvictBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictEnd(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CacheEvictEnd)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.BackgroundSyncBegin)(::windows_core::Interface::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.BackgroundSyncEnd)(::windows_core::Interface::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.PolicyChangeDetected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.PreferenceChangeDetected)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SettingsChangesApplied)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransparentCacheItemNotify<P0, P1, P2, P3>(&self, pszpath: P0, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2, pzsoldpath: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.TransparentCacheItemNotify)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), eventtype, itemtype, bmodifieddata.into_param().abi(), bmodifiedattributes.into_param().abi(), pzsoldpath.into_param().abi()).ok()
    }
    pub unsafe fn PrefetchFileBegin<P0>(&self, pszpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PrefetchFileBegin)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi()).ok()
    }
    pub unsafe fn PrefetchFileEnd<P0>(&self, pszpath: P0, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.PrefetchFileEnd)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), hrresult).ok()
    }
    pub unsafe fn PrefetchCloseHandleBegin(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrefetchCloseHandleBegin)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PrefetchCloseHandleEnd(&self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PrefetchCloseHandleEnd)(::windows_core::Interface::as_raw(self), dwclosedhandlecount, dwopenhandlecount, hrresult).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesEvents4, ::windows_core::IUnknown, IOfflineFilesEvents, IOfflineFilesEvents2, IOfflineFilesEvents3);
unsafe impl ::windows_core::Interface for IOfflineFilesEvents4 {
    type Vtable = IOfflineFilesEvents4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesEvents4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbd69b1e_c7d2_473e_b35f_9d8c24c0c484);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEvents4_Vtbl {
    pub base__: IOfflineFilesEvents3_Vtbl,
    pub PrefetchCloseHandleBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrefetchCloseHandleEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesEventsFilter(::windows_core::IUnknown);
impl IOfflineFilesEventsFilter {
    pub unsafe fn GetPathFilter(&self, ppszfilter: *mut ::windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPathFilter)(::windows_core::Interface::as_raw(self), ppszfilter, pmatch).ok()
    }
    pub unsafe fn GetIncludedEvents(&self, prgevents: &mut [OFFLINEFILES_EVENTS], pcevents: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIncludedEvents)(::windows_core::Interface::as_raw(self), prgevents.len() as _, ::core::mem::transmute(prgevents.as_ptr()), pcevents).ok()
    }
    pub unsafe fn GetExcludedEvents(&self, prgevents: &mut [OFFLINEFILES_EVENTS], pcevents: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetExcludedEvents)(::windows_core::Interface::as_raw(self), prgevents.len() as _, ::core::mem::transmute(prgevents.as_ptr()), pcevents).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesEventsFilter, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesEventsFilter {
    type Vtable = IOfflineFilesEventsFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesEventsFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33fc4e1b_0716_40fa_ba65_6e62a84a846f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesEventsFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPathFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszfilter: *mut ::windows_core::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows_core::HRESULT,
    pub GetIncludedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_core::HRESULT,
    pub GetExcludedEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesFileItem(::windows_core::IUnknown);
impl IOfflineFilesFileItem {
    pub unsafe fn GetItemType(&self) -> ::windows_core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetItemType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows_core::Result<IOfflineFilesItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetParentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsMarkedForDeletion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSparse(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsSparse)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEncrypted(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEncrypted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesFileItem, ::windows_core::IUnknown, IOfflineFilesItem);
unsafe impl ::windows_core::Interface for IOfflineFilesFileItem {
    type Vtable = IOfflineFilesFileItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesFileItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8dfadead_26c2_4eff_8a72_6b50723d9a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesFileItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSparse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSparse: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEncrypted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEncrypted: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesFileSysInfo(::windows_core::IUnknown);
impl IOfflineFilesFileSysInfo {
    pub unsafe fn GetAttributes(&self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAttributes)(::windows_core::Interface::as_raw(self), copy, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTimes)(::windows_core::Interface::as_raw(self), copy, pftcreationtime, pftlastwritetime, pftchangetime, pftlastaccesstime).ok()
    }
    pub unsafe fn GetFileSize(&self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFileSize)(::windows_core::Interface::as_raw(self), copy, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesFileSysInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesFileSysInfo {
    type Vtable = IOfflineFilesFileSysInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesFileSysInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc1a163f_7bfd_4d88_9c66_96ea9a6a3d6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesFileSysInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimes: usize,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesGhostInfo(::windows_core::IUnknown);
impl IOfflineFilesGhostInfo {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGhosted(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsGhosted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesGhostInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesGhostInfo {
    type Vtable = IOfflineFilesGhostInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesGhostInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b09d48c_8ab5_464f_a755_a59d92f99429);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesGhostInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsGhosted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsGhosted: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesItem(::windows_core::IUnknown);
impl IOfflineFilesItem {
    pub unsafe fn GetItemType(&self) -> ::windows_core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItemType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows_core::Result<IOfflineFilesItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetParentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsMarkedForDeletion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesItem, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesItem {
    type Vtable = IOfflineFilesItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a753da6_e044_4f12_a718_5d14d079a906);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItem_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwqueryflags: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMarkedForDeletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMarkedForDeletion: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesItemContainer(::windows_core::IUnknown);
impl IOfflineFilesItemContainer {
    pub unsafe fn EnumItems(&self, dwqueryflags: u32) -> ::windows_core::Result<IEnumOfflineFilesItems> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumItems)(::windows_core::Interface::as_raw(self), dwqueryflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumItemsEx<P0, P1, P2, P3>(&self, pincludefilefilter: P0, pincludedirfilter: P1, pexcludefilefilter: P2, pexcludedirfilter: P3, dwenumflags: u32, dwqueryflags: u32) -> ::windows_core::Result<IEnumOfflineFilesItems>
    where
        P0: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P1: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P2: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
        P3: ::windows_core::IntoParam<IOfflineFilesItemFilter>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnumItemsEx)(::windows_core::Interface::as_raw(self), pincludefilefilter.into_param().abi(), pincludedirfilter.into_param().abi(), pexcludefilefilter.into_param().abi(), pexcludedirfilter.into_param().abi(), dwenumflags, dwqueryflags, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesItemContainer, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesItemContainer {
    type Vtable = IOfflineFilesItemContainer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesItemContainer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3836f049_9413_45dd_bf46_b5aaa82dc310);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItemContainer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub EnumItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwqueryflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnumItemsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pincludefilefilter: *mut ::core::ffi::c_void, pincludedirfilter: *mut ::core::ffi::c_void, pexcludefilefilter: *mut ::core::ffi::c_void, pexcludedirfilter: *mut ::core::ffi::c_void, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesItemFilter(::windows_core::IUnknown);
impl IOfflineFilesItemFilter {
    pub unsafe fn GetFilterFlags(&self, pullflags: *mut u64, pullmask: *mut u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFilterFlags)(::windows_core::Interface::as_raw(self), pullflags, pullmask).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeFilter(&self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTimeFilter)(::windows_core::Interface::as_raw(self), pfttime, pbevaltimeofday, ptimetype, pcompare).ok()
    }
    pub unsafe fn GetPatternFilter(&self, pszpattern: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPatternFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszpattern.as_ptr()), pszpattern.len() as _).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesItemFilter, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesItemFilter {
    type Vtable = IOfflineFilesItemFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesItemFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4b5a26c_dc05_4f20_ada4_551f1077be5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesItemFilter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFilterFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimeFilter: usize,
    pub GetPatternFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpattern: ::windows_core::PWSTR, cchpattern: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesPinInfo(::windows_core::IUnknown);
impl IOfflineFilesPinInfo {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinned(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsPinned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForUser(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPinnedForUser)(::windows_core::Interface::as_raw(self), pbpinnedforuser, pbinherit).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPinnedForUserByPolicy)(::windows_core::Interface::as_raw(self), pbpinnedforuser, pbinherit).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPinnedForComputer)(::windows_core::Interface::as_raw(self), pbpinnedforcomputer, pbinherit).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPinnedForFolderRedirection)(::windows_core::Interface::as_raw(self), pbpinnedforfolderredirection, pbinherit).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesPinInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesPinInfo {
    type Vtable = IOfflineFilesPinInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesPinInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b2b0655_b3fd_497d_adeb_bd156bc8355b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesPinInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinned: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForUserByPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForUserByPolicy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForComputer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinnedForFolderRedirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinnedForFolderRedirection: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesPinInfo2(::windows_core::IUnknown);
impl IOfflineFilesPinInfo2 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinned(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsPinned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForUser(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsPinnedForUser)(::windows_core::Interface::as_raw(self), pbpinnedforuser, pbinherit).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsPinnedForUserByPolicy)(::windows_core::Interface::as_raw(self), pbpinnedforuser, pbinherit).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsPinnedForComputer)(::windows_core::Interface::as_raw(self), pbpinnedforcomputer, pbinherit).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsPinnedForFolderRedirection)(::windows_core::Interface::as_raw(self), pbpinnedforfolderredirection, pbinherit).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPartlyPinned(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsPartlyPinned)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesPinInfo2, ::windows_core::IUnknown, IOfflineFilesPinInfo);
unsafe impl ::windows_core::Interface for IOfflineFilesPinInfo2 {
    type Vtable = IOfflineFilesPinInfo2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesPinInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x623c58a2_42ed_4ad7_b69a_0f1b30a72d0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesPinInfo2_Vtbl {
    pub base__: IOfflineFilesPinInfo_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPartlyPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPartlyPinned: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesProgress(::windows_core::IUnknown);
impl IOfflineFilesProgress {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Begin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryAbort(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryAbort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn End(&self, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).End)(::windows_core::Interface::as_raw(self), hrresult).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesProgress, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesProgress {
    type Vtable = IOfflineFilesProgress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesProgress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfad63237_c55b_4911_9850_bcf96d4c979e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesProgress_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryAbort: usize,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesServerItem(::windows_core::IUnknown);
impl IOfflineFilesServerItem {
    pub unsafe fn GetItemType(&self) -> ::windows_core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetItemType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows_core::Result<IOfflineFilesItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetParentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsMarkedForDeletion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesServerItem, ::windows_core::IUnknown, IOfflineFilesItem);
unsafe impl ::windows_core::Interface for IOfflineFilesServerItem {
    type Vtable = IOfflineFilesServerItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesServerItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b1c9576_a92b_4151_8e9e_7c7b3ec2e016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesServerItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSetting(::windows_core::IUnknown);
impl IOfflineFilesSetting {
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetValueType(&self) -> ::windows_core::Result<OFFLINEFILES_SETTING_VALUE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetValueType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPreference(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPreference)(::windows_core::Interface::as_raw(self), pvarvalue, dwscope).ok()
    }
    pub unsafe fn GetPreferenceScope(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPreferenceScope)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPreference(&self, pvarvalue: *const super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreference)(::windows_core::Interface::as_raw(self), pvarvalue, dwscope).ok()
    }
    pub unsafe fn DeletePreference(&self, dwscope: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePreference)(::windows_core::Interface::as_raw(self), dwscope).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPolicy(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPolicy)(::windows_core::Interface::as_raw(self), pvarvalue, dwscope).ok()
    }
    pub unsafe fn GetPolicyScope(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPolicyScope)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetValue(&self, pvarvalue: *mut super::super::System::Variant::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), pvarvalue, pbsetbypolicy).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSetting, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesSetting {
    type Vtable = IOfflineFilesSetting_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSetting {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd871d3f7_f613_48a1_827e_7a34e560fff6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSetting_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPreference: usize,
    pub GetPreferenceScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPreference: usize,
    pub DeletePreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwscope: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, dwscope: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPolicy: usize,
    pub GetPolicyScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetValue: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesShareInfo(::windows_core::IUnknown);
impl IOfflineFilesShareInfo {
    pub unsafe fn GetShareItem(&self) -> ::windows_core::Result<IOfflineFilesShareItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetShareItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetShareCachingMode(&self) -> ::windows_core::Result<OFFLINEFILES_CACHING_MODE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetShareCachingMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShareDfsJunction(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsShareDfsJunction)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesShareInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesShareInfo {
    type Vtable = IOfflineFilesShareInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesShareInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bcc43e7_31ce_4ca4_8ccd_1cff2dc494da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesShareInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetShareItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppshareitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetShareCachingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsShareDfsJunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsShareDfsJunction: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesShareItem(::windows_core::IUnknown);
impl IOfflineFilesShareItem {
    pub unsafe fn GetItemType(&self) -> ::windows_core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetItemType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows_core::Result<IOfflineFilesItem> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetParentItem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsMarkedForDeletion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesShareItem, ::windows_core::IUnknown, IOfflineFilesItem);
unsafe impl ::windows_core::Interface for IOfflineFilesShareItem {
    type Vtable = IOfflineFilesShareItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesShareItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbab7e48d_4804_41b5_a44d_0f199b06b145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesShareItem_Vtbl {
    pub base__: IOfflineFilesItem_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSimpleProgress(::windows_core::IUnknown);
impl IOfflineFilesSimpleProgress {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Begin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryAbort(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryAbort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn End(&self, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.End)(::windows_core::Interface::as_raw(self), hrresult).ok()
    }
    pub unsafe fn ItemBegin<P0>(&self, pszfile: P0) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ItemBegin)(::windows_core::Interface::as_raw(self), pszfile.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ItemResult<P0>(&self, pszfile: P0, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ItemResult)(::windows_core::Interface::as_raw(self), pszfile.into_param().abi(), hrresult, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSimpleProgress, ::windows_core::IUnknown, IOfflineFilesProgress);
unsafe impl ::windows_core::Interface for IOfflineFilesSimpleProgress {
    type Vtable = IOfflineFilesSimpleProgress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSimpleProgress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc34f7f9b_c43d_4f9d_a776_c0eb6de5d401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSimpleProgress_Vtbl {
    pub base__: IOfflineFilesProgress_Vtbl,
    pub ItemBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT,
    pub ItemResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSuspend(::windows_core::IUnknown);
impl IOfflineFilesSuspend {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuspendRoot<P0>(&self, bsuspend: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SuspendRoot)(::windows_core::Interface::as_raw(self), bsuspend.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSuspend, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesSuspend {
    type Vtable = IOfflineFilesSuspend_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSuspend {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62c4560f_bc0b_48ca_ad9d_34cb528d99a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSuspend_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SuspendRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsuspend: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SuspendRoot: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSuspendInfo(::windows_core::IUnknown);
impl IOfflineFilesSuspendInfo {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSuspended(&self, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsSuspended)(::windows_core::Interface::as_raw(self), pbsuspended, pbsuspendedroot).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSuspendInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesSuspendInfo {
    type Vtable = IOfflineFilesSuspendInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSuspendInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa457c25b_4e9c_4b04_85af_8932ccd97889);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSuspendInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSuspended: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSyncConflictHandler(::windows_core::IUnknown);
impl IOfflineFilesSyncConflictHandler {
    pub unsafe fn ResolveConflict<P0>(&self, pszpath: P0, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ResolveConflict)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), fstateknown, state, fchangedetails, pconflictresolution, ppsznewname).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSyncConflictHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesSyncConflictHandler {
    type Vtable = IOfflineFilesSyncConflictHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSyncConflictHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6dd5092_c65c_46b6_97b8_fadd08e7e1be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncConflictHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ResolveConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSyncErrorInfo(::windows_core::IUnknown);
impl IOfflineFilesSyncErrorInfo {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawData(&self) -> ::windows_core::Result<*mut super::super::System::Com::BYTE_BLOB> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRawData)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSyncOperation(&self) -> ::windows_core::Result<OFFLINEFILES_SYNC_OPERATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSyncOperation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItemChangeFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItemChangeFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InfoEnumerated(&self, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InfoEnumerated)(::windows_core::Interface::as_raw(self), pblocalenumerated, pbremoteenumerated, pboriginalenumerated).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InfoAvailable(&self, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InfoAvailable)(::windows_core::Interface::as_raw(self), pblocalinfo, pbremoteinfo, pboriginalinfo).ok()
    }
    pub unsafe fn GetLocalInfo(&self) -> ::windows_core::Result<IOfflineFilesSyncErrorItemInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRemoteInfo(&self) -> ::windows_core::Result<IOfflineFilesSyncErrorItemInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRemoteInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOriginalInfo(&self) -> ::windows_core::Result<IOfflineFilesSyncErrorItemInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOriginalInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSyncErrorInfo, ::windows_core::IUnknown, IOfflineFilesErrorInfo);
unsafe impl ::windows_core::Interface for IOfflineFilesSyncErrorInfo {
    type Vtable = IOfflineFilesSyncErrorInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSyncErrorInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59f95e46_eb54_49d1_be76_de95458d01b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncErrorInfo_Vtbl {
    pub base__: IOfflineFilesErrorInfo_Vtbl,
    pub GetSyncOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows_core::HRESULT,
    pub GetItemChangeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwitemchangeflags: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InfoEnumerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfoEnumerated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InfoAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InfoAvailable: usize,
    pub GetLocalInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRemoteInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOriginalInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSyncErrorItemInfo(::windows_core::IUnknown);
impl IOfflineFilesSyncErrorItemInfo {
    pub unsafe fn GetFileAttributes(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFileAttributes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileTimes(&self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFileTimes)(::windows_core::Interface::as_raw(self), pftlastwrite, pftchange).ok()
    }
    pub unsafe fn GetFileSize(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFileSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSyncErrorItemInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesSyncErrorItemInfo {
    type Vtable = IOfflineFilesSyncErrorItemInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSyncErrorItemInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecdbaf0d_6a18_4d55_8017_108f7660ba44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncErrorItemInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFileAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileTimes: usize,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psize: *mut i64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesSyncProgress(::windows_core::IUnknown);
impl IOfflineFilesSyncProgress {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Begin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryAbort(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryAbort)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn End(&self, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.End)(::windows_core::Interface::as_raw(self), hrresult).ok()
    }
    pub unsafe fn SyncItemBegin<P0>(&self, pszfile: P0) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SyncItemBegin)(::windows_core::Interface::as_raw(self), pszfile.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SyncItemResult<P0, P1>(&self, pszfile: P0, hrresult: ::windows_core::HRESULT, perrorinfo: P1) -> ::windows_core::Result<OFFLINEFILES_OP_RESPONSE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IOfflineFilesSyncErrorInfo>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SyncItemResult)(::windows_core::Interface::as_raw(self), pszfile.into_param().abi(), hrresult, perrorinfo.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesSyncProgress, ::windows_core::IUnknown, IOfflineFilesProgress);
unsafe impl ::windows_core::Interface for IOfflineFilesSyncProgress {
    type Vtable = IOfflineFilesSyncProgress_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesSyncProgress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6931f49a_6fc7_4c1b_b265_56793fc451b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesSyncProgress_Vtbl {
    pub base__: IOfflineFilesProgress_Vtbl,
    pub SyncItemBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT,
    pub SyncItemResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR, hrresult: ::windows_core::HRESULT, perrorinfo: *mut ::core::ffi::c_void, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOfflineFilesTransparentCacheInfo(::windows_core::IUnknown);
impl IOfflineFilesTransparentCacheInfo {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTransparentlyCached(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsTransparentlyCached)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IOfflineFilesTransparentCacheInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOfflineFilesTransparentCacheInfo {
    type Vtable = IOfflineFilesTransparentCacheInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOfflineFilesTransparentCacheInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcaf4a01_5b68_4b56_a6a1_8d2786ede8e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineFilesTransparentCacheInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTransparentlyCached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTransparentlyCached: usize,
}
pub const OFFLINEFILES_CACHING_MODE_AUTO_DOC: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(3i32);
pub const OFFLINEFILES_CACHING_MODE_AUTO_PROGANDDOC: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(4i32);
pub const OFFLINEFILES_CACHING_MODE_MANUAL: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(2i32);
pub const OFFLINEFILES_CACHING_MODE_NOCACHING: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(1i32);
pub const OFFLINEFILES_CACHING_MODE_NONE: OFFLINEFILES_CACHING_MODE = OFFLINEFILES_CACHING_MODE(0i32);
pub const OFFLINEFILES_CHANGES_LOCAL_ATTRIBUTES: u32 = 2u32;
pub const OFFLINEFILES_CHANGES_LOCAL_SIZE: u32 = 1u32;
pub const OFFLINEFILES_CHANGES_LOCAL_TIME: u32 = 4u32;
pub const OFFLINEFILES_CHANGES_NONE: u32 = 0u32;
pub const OFFLINEFILES_CHANGES_REMOTE_ATTRIBUTES: u32 = 16u32;
pub const OFFLINEFILES_CHANGES_REMOTE_SIZE: u32 = 8u32;
pub const OFFLINEFILES_CHANGES_REMOTE_TIME: u32 = 32u32;
pub const OFFLINEFILES_COMPARE_EQ: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(0i32);
pub const OFFLINEFILES_COMPARE_GT: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(3i32);
pub const OFFLINEFILES_COMPARE_GTE: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(5i32);
pub const OFFLINEFILES_COMPARE_LT: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(2i32);
pub const OFFLINEFILES_COMPARE_LTE: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(4i32);
pub const OFFLINEFILES_COMPARE_NEQ: OFFLINEFILES_COMPARE = OFFLINEFILES_COMPARE(1i32);
pub const OFFLINEFILES_CONNECT_STATE_OFFLINE: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(1i32);
pub const OFFLINEFILES_CONNECT_STATE_ONLINE: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(2i32);
pub const OFFLINEFILES_CONNECT_STATE_PARTLY_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(4i32);
pub const OFFLINEFILES_CONNECT_STATE_TRANSPARENTLY_CACHED: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(3i32);
pub const OFFLINEFILES_CONNECT_STATE_UNKNOWN: OFFLINEFILES_CONNECT_STATE = OFFLINEFILES_CONNECT_STATE(0i32);
pub const OFFLINEFILES_DELETE_FLAG_ADMIN: u32 = 2147483648u32;
pub const OFFLINEFILES_DELETE_FLAG_DELMODIFIED: u32 = 4u32;
pub const OFFLINEFILES_DELETE_FLAG_NOAUTOCACHED: u32 = 1u32;
pub const OFFLINEFILES_DELETE_FLAG_NOPINNED: u32 = 2u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_ENCRYPTION_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_ENUM_FLAT: u32 = 1u32;
pub const OFFLINEFILES_ENUM_FLAT_FILESONLY: u32 = 2u32;
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(11i32);
pub const OFFLINEFILES_EVENT_BACKGROUNDSYNCEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(12i32);
pub const OFFLINEFILES_EVENT_CACHEEVICTBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(29i32);
pub const OFFLINEFILES_EVENT_CACHEEVICTEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(30i32);
pub const OFFLINEFILES_EVENT_CACHEISCORRUPTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(2i32);
pub const OFFLINEFILES_EVENT_CACHEISFULL: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(1i32);
pub const OFFLINEFILES_EVENT_CACHEMOVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(0i32);
pub const OFFLINEFILES_EVENT_DATALOST: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(25i32);
pub const OFFLINEFILES_EVENT_ENABLED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(3i32);
pub const OFFLINEFILES_EVENT_ENCRYPTIONCHANGED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(4i32);
pub const OFFLINEFILES_EVENT_ITEMADDEDTOCACHE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(22i32);
pub const OFFLINEFILES_EVENT_ITEMAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(17i32);
pub const OFFLINEFILES_EVENT_ITEMDELETEDFROMCACHE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(23i32);
pub const OFFLINEFILES_EVENT_ITEMDISCONNECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(15i32);
pub const OFFLINEFILES_EVENT_ITEMMODIFIED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(21i32);
pub const OFFLINEFILES_EVENT_ITEMNOTAVAILABLEOFFLINE: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(18i32);
pub const OFFLINEFILES_EVENT_ITEMNOTPINNED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(20i32);
pub const OFFLINEFILES_EVENT_ITEMPINNED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(19i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(27i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(16i32);
pub const OFFLINEFILES_EVENT_ITEMRECONNECTEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(28i32);
pub const OFFLINEFILES_EVENT_ITEMRENAMED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(24i32);
pub const OFFLINEFILES_EVENT_NETTRANSPORTARRIVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(13i32);
pub const OFFLINEFILES_EVENT_NONETTRANSPORTS: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(14i32);
pub const OFFLINEFILES_EVENT_PING: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(26i32);
pub const OFFLINEFILES_EVENT_POLICYCHANGEDETECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(31i32);
pub const OFFLINEFILES_EVENT_PREFERENCECHANGEDETECTED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(32i32);
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(37i32);
pub const OFFLINEFILES_EVENT_PREFETCHCLOSEHANDLEEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(38i32);
pub const OFFLINEFILES_EVENT_PREFETCHFILEBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(35i32);
pub const OFFLINEFILES_EVENT_PREFETCHFILEEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(36i32);
pub const OFFLINEFILES_EVENT_SETTINGSCHANGESAPPLIED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(33i32);
pub const OFFLINEFILES_EVENT_SYNCBEGIN: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(5i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECADDED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(7i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECREMOVED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(9i32);
pub const OFFLINEFILES_EVENT_SYNCCONFLICTRECUPDATED: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(8i32);
pub const OFFLINEFILES_EVENT_SYNCEND: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(10i32);
pub const OFFLINEFILES_EVENT_SYNCFILERESULT: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(6i32);
pub const OFFLINEFILES_EVENT_TRANSPARENTCACHEITEMNOTIFY: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(34i32);
pub const OFFLINEFILES_ITEM_COPY_LOCAL: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(0i32);
pub const OFFLINEFILES_ITEM_COPY_ORIGINAL: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(2i32);
pub const OFFLINEFILES_ITEM_COPY_REMOTE: OFFLINEFILES_ITEM_COPY = OFFLINEFILES_ITEM_COPY(1i32);
pub const OFFLINEFILES_ITEM_FILTER_FLAG_CREATED: u32 = 8u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DELETED: u32 = 16u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRECTORY: u32 = 256u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_DIRTY: u32 = 32u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_FILE: u32 = 128u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GHOST: u32 = 8192u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_ANYACCESS: u32 = 33554432u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_READ: u32 = 16777216u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_GUEST_WRITE: u32 = 8388608u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED: u32 = 4u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_ATTRIBUTES: u32 = 2u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_MODIFIED_DATA: u32 = 1u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OFFLINE: u32 = 32768u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_ONLINE: u32 = 65536u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_ANYACCESS: u32 = 4194304u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_READ: u32 = 2097152u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_OTHER_WRITE: u32 = 1048576u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED: u32 = 4096u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_COMPUTER: u32 = 2048u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_OTHERS: u32 = 1024u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_PINNED_USER: u32 = 512u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SPARSE: u32 = 64u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_SUSPENDED: u32 = 16384u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_ANYACCESS: u32 = 524288u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_READ: u32 = 262144u32;
pub const OFFLINEFILES_ITEM_FILTER_FLAG_USER_WRITE: u32 = 131072u32;
pub const OFFLINEFILES_ITEM_QUERY_ADMIN: u32 = 2147483648u32;
pub const OFFLINEFILES_ITEM_QUERY_ATTEMPT_TRANSITIONONLINE: u32 = 32u32;
pub const OFFLINEFILES_ITEM_QUERY_CONNECTIONSTATE: u32 = 2u32;
pub const OFFLINEFILES_ITEM_QUERY_INCLUDETRANSPARENTCACHE: u32 = 16u32;
pub const OFFLINEFILES_ITEM_QUERY_LOCALDIRTYBYTECOUNT: u32 = 4u32;
pub const OFFLINEFILES_ITEM_QUERY_REMOTEDIRTYBYTECOUNT: u32 = 8u32;
pub const OFFLINEFILES_ITEM_QUERY_REMOTEINFO: u32 = 1u32;
pub const OFFLINEFILES_ITEM_TIME_CREATION: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(0i32);
pub const OFFLINEFILES_ITEM_TIME_LASTACCESS: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(1i32);
pub const OFFLINEFILES_ITEM_TIME_LASTWRITE: OFFLINEFILES_ITEM_TIME = OFFLINEFILES_ITEM_TIME(2i32);
pub const OFFLINEFILES_ITEM_TYPE_DIRECTORY: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(1i32);
pub const OFFLINEFILES_ITEM_TYPE_FILE: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(0i32);
pub const OFFLINEFILES_ITEM_TYPE_SERVER: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(3i32);
pub const OFFLINEFILES_ITEM_TYPE_SHARE: OFFLINEFILES_ITEM_TYPE = OFFLINEFILES_ITEM_TYPE(2i32);
pub const OFFLINEFILES_NUM_EVENTS: OFFLINEFILES_EVENTS = OFFLINEFILES_EVENTS(39i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_ERROR: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(4i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_FORCED: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(2i32);
pub const OFFLINEFILES_OFFLINE_REASON_CONNECTION_SLOW: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(3i32);
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_SUSPENDED: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(6i32);
pub const OFFLINEFILES_OFFLINE_REASON_ITEM_VERSION_CONFLICT: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(5i32);
pub const OFFLINEFILES_OFFLINE_REASON_NOT_APPLICABLE: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(1i32);
pub const OFFLINEFILES_OFFLINE_REASON_UNKNOWN: OFFLINEFILES_OFFLINE_REASON = OFFLINEFILES_OFFLINE_REASON(0i32);
pub const OFFLINEFILES_OP_ABORT: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(2i32);
pub const OFFLINEFILES_OP_CONTINUE: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(0i32);
pub const OFFLINEFILES_OP_RETRY: OFFLINEFILES_OP_RESPONSE = OFFLINEFILES_OP_RESPONSE(1i32);
pub const OFFLINEFILES_PATHFILTER_CHILD: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(1i32);
pub const OFFLINEFILES_PATHFILTER_DESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(2i32);
pub const OFFLINEFILES_PATHFILTER_SELF: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(0i32);
pub const OFFLINEFILES_PATHFILTER_SELFORCHILD: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(3i32);
pub const OFFLINEFILES_PATHFILTER_SELFORDESCENDENT: OFFLINEFILES_PATHFILTER_MATCH = OFFLINEFILES_PATHFILTER_MATCH(4i32);
pub const OFFLINEFILES_PINLINKTARGETS_ALWAYS: u32 = 2u32;
pub const OFFLINEFILES_PINLINKTARGETS_EXPLICIT: u32 = 1u32;
pub const OFFLINEFILES_PINLINKTARGETS_NEVER: u32 = 0u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FILL: u32 = 1u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORALL: u32 = 128u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORREDIR: u32 = 256u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER: u32 = 32u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_FORUSER_POLICY: u32 = 64u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_PIN_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
pub const OFFLINEFILES_SETTING_PinLinkTargets: ::windows_core::PCWSTR = ::windows_core::w!("LinkTargetCaching");
pub const OFFLINEFILES_SETTING_SCOPE_COMPUTER: u32 = 2u32;
pub const OFFLINEFILES_SETTING_SCOPE_USER: u32 = 1u32;
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(4i32);
pub const OFFLINEFILES_SETTING_VALUE_2DIM_ARRAY_BSTR_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(3i32);
pub const OFFLINEFILES_SETTING_VALUE_BSTR: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(1i32);
pub const OFFLINEFILES_SETTING_VALUE_BSTR_DBLNULTERM: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(2i32);
pub const OFFLINEFILES_SETTING_VALUE_UI4: OFFLINEFILES_SETTING_VALUE_TYPE = OFFLINEFILES_SETTING_VALUE_TYPE(0i32);
pub const OFFLINEFILES_SYNC_CONFLICT_ABORT: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(7i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPALLCHANGES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(3i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLATEST: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(4i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPLOCAL: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(1i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_KEEPREMOTE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(2i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_LOG: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(5i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NONE: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(0i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_NUMCODES: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(8i32);
pub const OFFLINEFILES_SYNC_CONFLICT_RESOLVE_SKIP: OFFLINEFILES_SYNC_CONFLICT_RESOLVE = OFFLINEFILES_SYNC_CONFLICT_RESOLVE(6i32);
pub const OFFLINEFILES_SYNC_CONTROL_CR_DEFAULT: u32 = 0u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLATEST: u32 = 805306368u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPLOCAL: u32 = 268435456u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_KEEPREMOTE: u32 = 536870912u32;
pub const OFFLINEFILES_SYNC_CONTROL_CR_MASK: u32 = 4026531840u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_ASYNCPROGRESS: u32 = 1024u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_BACKGROUND: u32 = 65536u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_CONSOLE: u32 = 4096u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_FILLSPARSE: u32 = 1u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_INTERACTIVE: u32 = 2048u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_LOWPRIORITY: u32 = 512u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_NONEWFILESOUT: u32 = 131072u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORALL: u32 = 128u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORREDIR: u32 = 256u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER: u32 = 32u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINFORUSER_POLICY: u32 = 64u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINLINKTARGETS: u32 = 16u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_PINNEWFILES: u32 = 8u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SKIPSUSPENDEDDIRS: u32 = 8192u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCIN: u32 = 2u32;
pub const OFFLINEFILES_SYNC_CONTROL_FLAG_SYNCOUT: u32 = 4u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_ATTRIBUTES: u32 = 8u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_CHANGETIME: u32 = 1u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_FILESIZE: u32 = 4u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_NONE: u32 = 0u32;
pub const OFFLINEFILES_SYNC_ITEM_CHANGE_WRITETIME: u32 = 2u32;
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_CLIENT: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(1i32);
pub const OFFLINEFILES_SYNC_OPERATION_CREATE_COPY_ON_SERVER: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(0i32);
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_CLIENT_COPY: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(5i32);
pub const OFFLINEFILES_SYNC_OPERATION_DELETE_SERVER_COPY: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(4i32);
pub const OFFLINEFILES_SYNC_OPERATION_PIN: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(6i32);
pub const OFFLINEFILES_SYNC_OPERATION_PREPARE: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(7i32);
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_CLIENT: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(3i32);
pub const OFFLINEFILES_SYNC_OPERATION_SYNC_TO_SERVER: OFFLINEFILES_SYNC_OPERATION = OFFLINEFILES_SYNC_OPERATION(2i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(37i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(35i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(36i32);
pub const OFFLINEFILES_SYNC_STATE_DeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(34i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(42i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(28i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(29i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(27i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(26i32);
pub const OFFLINEFILES_SYNC_STATE_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(47i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(25i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(24i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(21i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(23i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(22i32);
pub const OFFLINEFILES_SYNC_STATE_DirCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(20i32);
pub const OFFLINEFILES_SYNC_STATE_DirDeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(49i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(4i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(3i32);
pub const OFFLINEFILES_SYNC_STATE_DirOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(5i32);
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(43i32);
pub const OFFLINEFILES_SYNC_STATE_DirRenamedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(48i32);
pub const OFFLINEFILES_SYNC_STATE_DirSparseOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(41i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(39i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(12i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(15i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(14i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(13i32);
pub const OFFLINEFILES_SYNC_STATE_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(44i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(11i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(8i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(10i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(7i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(9i32);
pub const OFFLINEFILES_SYNC_STATE_FileCreatedOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(6i32);
pub const OFFLINEFILES_SYNC_STATE_FileDeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(46i32);
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(1i32);
pub const OFFLINEFILES_SYNC_STATE_FileOnClient_NoServerCopy: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(2i32);
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(40i32);
pub const OFFLINEFILES_SYNC_STATE_FileRenamedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(45i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(53i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(52i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(51i32);
pub const OFFLINEFILES_SYNC_STATE_FileReplacedAndDeletedOnClient_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(50i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(38i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_ChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(16i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DeletedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(17i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(19i32);
pub const OFFLINEFILES_SYNC_STATE_FileSparseOnClient_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(18i32);
pub const OFFLINEFILES_SYNC_STATE_LOCAL_KNOWN: u32 = 1u32;
pub const OFFLINEFILES_SYNC_STATE_NUMSTATES: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(54i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(33i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_DirOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(31i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileChangedOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(32i32);
pub const OFFLINEFILES_SYNC_STATE_NoClientCopy_FileOnServer: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(30i32);
pub const OFFLINEFILES_SYNC_STATE_REMOTE_KNOWN: u32 = 2u32;
pub const OFFLINEFILES_SYNC_STATE_Stable: OFFLINEFILES_SYNC_STATE = OFFLINEFILES_SYNC_STATE(0i32);
pub const OFFLINEFILES_TRANSITION_FLAG_CONSOLE: u32 = 2u32;
pub const OFFLINEFILES_TRANSITION_FLAG_INTERACTIVE: u32 = 1u32;
pub const OfflineFilesCache: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48c6be7c_3871_43cc_b46f_1449a1bb2ff3);
pub const OfflineFilesSetting: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd3659e9_a920_4123_ad64_7fc76c7aacdf);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_CACHING_MODE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_CACHING_MODE {}
impl ::core::clone::Clone for OFFLINEFILES_CACHING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_CACHING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_CACHING_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_CACHING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_CACHING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_COMPARE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_COMPARE {}
impl ::core::clone::Clone for OFFLINEFILES_COMPARE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_COMPARE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_COMPARE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_COMPARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_COMPARE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_CONNECT_STATE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_CONNECT_STATE {}
impl ::core::clone::Clone for OFFLINEFILES_CONNECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_CONNECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_CONNECT_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_CONNECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_CONNECT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_EVENTS(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_EVENTS {}
impl ::core::clone::Clone for OFFLINEFILES_EVENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_EVENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_EVENTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_EVENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_EVENTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_ITEM_COPY(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_ITEM_COPY {}
impl ::core::clone::Clone for OFFLINEFILES_ITEM_COPY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_ITEM_COPY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_ITEM_COPY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_ITEM_COPY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_ITEM_COPY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_ITEM_TIME(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_ITEM_TIME {}
impl ::core::clone::Clone for OFFLINEFILES_ITEM_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_ITEM_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_ITEM_TIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_ITEM_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_ITEM_TIME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_ITEM_TYPE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_ITEM_TYPE {}
impl ::core::clone::Clone for OFFLINEFILES_ITEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_ITEM_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_ITEM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_OFFLINE_REASON(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_OFFLINE_REASON {}
impl ::core::clone::Clone for OFFLINEFILES_OFFLINE_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_OFFLINE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_OFFLINE_REASON {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_OFFLINE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_OFFLINE_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_OP_RESPONSE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_OP_RESPONSE {}
impl ::core::clone::Clone for OFFLINEFILES_OP_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_OP_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_OP_RESPONSE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_OP_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_OP_RESPONSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_PATHFILTER_MATCH(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_PATHFILTER_MATCH {}
impl ::core::clone::Clone for OFFLINEFILES_PATHFILTER_MATCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_PATHFILTER_MATCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_PATHFILTER_MATCH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_PATHFILTER_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_PATHFILTER_MATCH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_SETTING_VALUE_TYPE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_SETTING_VALUE_TYPE {}
impl ::core::clone::Clone for OFFLINEFILES_SETTING_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_SETTING_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_SETTING_VALUE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_SETTING_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SETTING_VALUE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_SYNC_CONFLICT_RESOLVE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {}
impl ::core::clone::Clone for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SYNC_CONFLICT_RESOLVE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_SYNC_OPERATION(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_SYNC_OPERATION {}
impl ::core::clone::Clone for OFFLINEFILES_SYNC_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_SYNC_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_SYNC_OPERATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_SYNC_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SYNC_OPERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OFFLINEFILES_SYNC_STATE(pub i32);
impl ::core::marker::Copy for OFFLINEFILES_SYNC_STATE {}
impl ::core::clone::Clone for OFFLINEFILES_SYNC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFLINEFILES_SYNC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OFFLINEFILES_SYNC_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLINEFILES_SYNC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SYNC_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
