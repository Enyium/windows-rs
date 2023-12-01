#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHolographicCameraInterop(::windows_core::IUnknown);
impl IHolographicCameraInterop {
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12BackBufferResource<P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDirect3D12BackBufferResource(pdevice.into_param().abi(), ptexture2ddesc, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<P0, P1>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P1) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Device>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDirect3D12HardwareProtectedBackBufferResource(pdevice.into_param().abi(), ptexture2ddesc, pprotectedresourcesession.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        ::windows_core::vcall!(self.AcquireDirect3D12BufferResource(presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        ::windows_core::vcall!(self.AcquireDirect3D12BufferResourceWithTimeout(presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), duration)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<P0>(&self, presourcetounacquire: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        ::windows_core::vcall!(self.UnacquireDirect3D12BufferResource(presourcetounacquire.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IHolographicCameraInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IHolographicCameraInterop {
    type Vtable = IHolographicCameraInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHolographicCameraInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cc1f9c5_6d02_41fa_9500_e1809eb48eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12BackBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12BackBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedBackBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedBackBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHolographicCameraRenderingParametersInterop(::windows_core::IUnknown);
impl IHolographicCameraRenderingParametersInterop {
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        ::windows_core::vcall!(self.CommitDirect3D12Resource(pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), colorresourcefencesignalvalue)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<P0, P1, P2, P3>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: P2, pdepthresourcefence: P3, depthresourcefencesignalvalue: u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
        P2: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P3: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        ::windows_core::vcall!(self.CommitDirect3D12ResourceWithDepthData(pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), colorresourcefencesignalvalue, pdepthresourcetocommit.into_param().abi(), pdepthresourcefence.into_param().abi(), depthresourcefencesignalvalue)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IHolographicCameraRenderingParametersInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IHolographicCameraRenderingParametersInterop {
    type Vtable = IHolographicCameraRenderingParametersInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHolographicCameraRenderingParametersInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf75b68d6_d1fd_4707_aafd_fa6f4c0e3bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParametersInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12ResourceWithDepthData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: *mut ::core::ffi::c_void, pdepthresourcefence: *mut ::core::ffi::c_void, depthresourcefencesignalvalue: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12ResourceWithDepthData: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHolographicQuadLayerInterop(::windows_core::IUnknown);
impl IHolographicQuadLayerInterop {
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12ContentBufferResource<P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDirect3D12ContentBufferResource(pdevice.into_param().abi(), ptexture2ddesc, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<P0, P1>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P1) -> ::windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Device>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDirect3D12HardwareProtectedContentBufferResource(pdevice.into_param().abi(), ptexture2ddesc, pprotectedresourcesession.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        ::windows_core::vcall!(self.AcquireDirect3D12BufferResource(presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        ::windows_core::vcall!(self.AcquireDirect3D12BufferResourceWithTimeout(presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), duration)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<P0>(&self, presourcetounacquire: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        ::windows_core::vcall!(self.UnacquireDirect3D12BufferResource(presourcetounacquire.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IHolographicQuadLayerInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IHolographicQuadLayerInterop {
    type Vtable = IHolographicQuadLayerInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHolographicQuadLayerInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfa688f0_639e_4a47_83d7_6b7f5ebf7fed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12ContentBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12ContentBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedContentBufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut ::core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedContentBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetoacquire: *mut ::core::ffi::c_void, pcommandqueue: *mut ::core::ffi::c_void, duration: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presourcetounacquire: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHolographicQuadLayerUpdateParametersInterop(::windows_core::IUnknown);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[doc = "Required features: `\"Win32_Graphics_Direct3D12\"`"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        ::windows_core::vcall!(self.CommitDirect3D12Resource(pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), colorresourcefencesignalvalue)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IHolographicQuadLayerUpdateParametersInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IHolographicQuadLayerUpdateParametersInterop {
    type Vtable = IHolographicQuadLayerUpdateParametersInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHolographicQuadLayerUpdateParametersInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5f549cd_c909_444f_8809_7cc18a9c8920);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorresourcetocommit: *mut ::core::ffi::c_void, pcolorresourcefence: *mut ::core::ffi::c_void, colorresourcefencesignalvalue: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
