#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISysStorageProviderEventReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISysStorageProviderEventReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe132d1b9_7b9d_5820_9728_4262b5289142);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Json: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISysStorageProviderEventReceivedEventArgsFactory {
    type Vtable = ISysStorageProviderEventReceivedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISysStorageProviderEventReceivedEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde1a780e_e975_5f68_bcc6_fb46281c6a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, json: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISysStorageProviderEventSource(::windows_core::IUnknown);
impl ISysStorageProviderEventSource {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EventReceived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<ISysStorageProviderEventSource, SysStorageProviderEventReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EventReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEventReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEventReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ISysStorageProviderEventSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ISysStorageProviderEventSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISysStorageProviderEventSource {
    type Vtable = ISysStorageProviderEventSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISysStorageProviderEventSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f36c476_9546_536a_8381_2f9a2c08cedd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderEventSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub EventReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EventReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEventReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEventReceived: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISysStorageProviderHandlerFactory(::windows_core::IUnknown);
impl ISysStorageProviderHandlerFactory {
    pub fn GetHttpRequestProvider(&self, syncrootid: &::windows_core::HSTRING) -> ::windows_core::Result<ISysStorageProviderHttpRequestProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHttpRequestProvider)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(syncrootid), &mut result__).from_abi(result__)
        }
    }
    pub fn GetEventSource(&self, syncrootid: &::windows_core::HSTRING, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<ISysStorageProviderEventSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEventSource)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(syncrootid), ::core::mem::transmute_copy(eventname), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISysStorageProviderHandlerFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ISysStorageProviderHandlerFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISysStorageProviderHandlerFactory {
    type Vtable = ISysStorageProviderHandlerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISysStorageProviderHandlerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee798431_8213_5e89_a623_14d8c72b8a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHandlerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetHttpRequestProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetEventSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISysStorageProviderHttpRequestProvider(::windows_core::IUnknown);
impl ISysStorageProviderHttpRequestProvider {
    #[doc = "Required features: `\"Foundation\"`, `\"Web_Http\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub fn SendRequestAsync<P0>(&self, request: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Web::Http::HttpResponseMessage>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Web::Http::HttpRequestMessage>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendRequestAsync)(::windows_core::Interface::as_raw(this), request.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISysStorageProviderHttpRequestProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ISysStorageProviderHttpRequestProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISysStorageProviderHttpRequestProvider {
    type Vtable = ISysStorageProviderHttpRequestProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISysStorageProviderHttpRequestProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb6fefb6_e76a_5c25_a33e_3e78a6e0e0ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISysStorageProviderHttpRequestProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub SendRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))]
    SendRequestAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SysStorageProviderEventReceivedEventArgs(::windows_core::IUnknown);
impl SysStorageProviderEventReceivedEventArgs {
    pub fn Json(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Json)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(json: &::windows_core::HSTRING) -> ::windows_core::Result<SysStorageProviderEventReceivedEventArgs> {
        Self::ISysStorageProviderEventReceivedEventArgsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(json), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISysStorageProviderEventReceivedEventArgsFactory<R, F: FnOnce(&ISysStorageProviderEventReceivedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SysStorageProviderEventReceivedEventArgs, ISysStorageProviderEventReceivedEventArgsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SysStorageProviderEventReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SysStorageProviderEventReceivedEventArgs {
    type Vtable = ISysStorageProviderEventReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SysStorageProviderEventReceivedEventArgs {
    const IID: ::windows_core::GUID = <ISysStorageProviderEventReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SysStorageProviderEventReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Implementation.FileExplorer.SysStorageProviderEventReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SysStorageProviderEventReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SysStorageProviderEventReceivedEventArgs {}
unsafe impl ::core::marker::Sync for SysStorageProviderEventReceivedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
