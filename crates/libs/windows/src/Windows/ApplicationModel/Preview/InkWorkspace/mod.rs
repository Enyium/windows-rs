#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInkWorkspaceHostedAppManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInkWorkspaceHostedAppManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe0a7990_5e59_4bb7_8a63_7d218cd96300);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub SetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    SetThumbnailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInkWorkspaceHostedAppManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInkWorkspaceHostedAppManagerStatics {
    type Vtable = IInkWorkspaceHostedAppManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInkWorkspaceHostedAppManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbfd8cc5_a162_4bc4_84ee_e8716d5233c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InkWorkspaceHostedAppManager(::windows_core::IUnknown);
impl InkWorkspaceHostedAppManager {
    #[doc = "Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn SetThumbnailAsync<P0>(&self, bitmap: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetThumbnailAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetForCurrentApp() -> ::windows_core::Result<InkWorkspaceHostedAppManager> {
        Self::IInkWorkspaceHostedAppManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentApp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkWorkspaceHostedAppManagerStatics<R, F: FnOnce(&IInkWorkspaceHostedAppManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InkWorkspaceHostedAppManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InkWorkspaceHostedAppManager {
    const IID: ::windows_core::GUID = <IInkWorkspaceHostedAppManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager";
}
::windows_core::imp::interface_hierarchy!(InkWorkspaceHostedAppManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InkWorkspaceHostedAppManager {}
unsafe impl ::core::marker::Sync for InkWorkspaceHostedAppManager {}
