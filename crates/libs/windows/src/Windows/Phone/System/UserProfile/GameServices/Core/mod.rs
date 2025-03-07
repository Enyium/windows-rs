#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGameService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameService {
    type Vtable = IGameService_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGameService {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e2d5098_48a9_4efc_afd6_8e6da09003fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ServiceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceUri: usize,
    #[cfg(feature = "Foundation")]
    pub GetGamerProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGamerProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetInstalledGameItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInstalledGameItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPartnerTokenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPartnerTokenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPrivilegesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPrivilegesAsync: usize,
    pub GrantAchievement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, achievementid: u32) -> ::windows_core::HRESULT,
    pub GrantAvatarAward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, avatarawardid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PostResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PostResult: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGameService2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameService2 {
    type Vtable = IGameService2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGameService2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2364ef6_ea17_4be5_8d8a_c860885e051f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameService2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub NotifyPartnerTokenExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audienceuri: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyPartnerTokenExpired: usize,
    pub GetAuthenticationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGameServicePropertyCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGameServicePropertyCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameServicePropertyCollection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetPropertyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPropertyAsync: usize,
}
pub struct GameService;
impl GameService {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceUri() -> ::windows_core::Result<super::super::super::super::super::Foundation::Uri> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetGamerProfileAsync() -> ::windows_core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGamerProfileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetInstalledGameItemsAsync() -> ::windows_core::Result<super::super::super::super::super::Foundation::IAsyncOperation<GameServicePropertyCollection>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInstalledGameItemsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetPartnerTokenAsync<P0>(audienceuri: P0) -> ::windows_core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::super::Foundation::Uri>,
    {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPartnerTokenAsync)(::windows_core::Interface::as_raw(this), audienceuri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetPrivilegesAsync() -> ::windows_core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>> {
        Self::IGameService(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPrivilegesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GrantAchievement(achievementid: u32) -> ::windows_core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows_core::Interface::vtable(this).GrantAchievement)(::windows_core::Interface::as_raw(this), achievementid).ok() })
    }
    pub fn GrantAvatarAward(avatarawardid: u32) -> ::windows_core::Result<()> {
        Self::IGameService(|this| unsafe { (::windows_core::Interface::vtable(this).GrantAvatarAward)(::windows_core::Interface::as_raw(this), avatarawardid).ok() })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PostResult<P0>(gamevariant: u32, scorekind: GameServiceScoreKind, scorevalue: i64, gameoutcome: GameServiceGameOutcome, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::super::super::Storage::Streams::IBuffer>,
    {
        Self::IGameService(|this| unsafe { (::windows_core::Interface::vtable(this).PostResult)(::windows_core::Interface::as_raw(this), gamevariant, scorekind, scorevalue, gameoutcome, buffer.try_into_param()?.abi()).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyPartnerTokenExpired<P0>(audienceuri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::super::Foundation::Uri>,
    {
        Self::IGameService2(|this| unsafe { (::windows_core::Interface::vtable(this).NotifyPartnerTokenExpired)(::windows_core::Interface::as_raw(this), audienceuri.into_param().abi()).ok() })
    }
    pub fn GetAuthenticationStatus() -> ::windows_core::Result<u32> {
        Self::IGameService2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAuthenticationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameService<R, F: FnOnce(&IGameService) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GameService, IGameService> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGameService2<R, F: FnOnce(&IGameService2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GameService, IGameService2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for GameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameService";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GameServicePropertyCollection(::windows_core::IUnknown);
impl GameServicePropertyCollection {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetPropertyAsync(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::super::Foundation::IAsyncOperation<::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertyAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GameServicePropertyCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GameServicePropertyCollection {
    type Vtable = IGameServicePropertyCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GameServicePropertyCollection {
    const IID: ::windows_core::GUID = <IGameServicePropertyCollection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection";
}
::windows_core::imp::interface_hierarchy!(GameServicePropertyCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GameServicePropertyCollection {}
unsafe impl ::core::marker::Sync for GameServicePropertyCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: Self = Self(0i32);
    pub const Win: Self = Self(1i32);
    pub const Loss: Self = Self(2i32);
    pub const Tie: Self = Self(3i32);
}
impl ::core::marker::Copy for GameServiceGameOutcome {}
impl ::core::clone::Clone for GameServiceGameOutcome {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameServiceGameOutcome {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GameServiceGameOutcome {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GameServiceGameOutcome {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceGameOutcome").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameServiceGameOutcome {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceGameOutcome;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: Self = Self(0i32);
    pub const Time: Self = Self(1i32);
}
impl ::core::marker::Copy for GameServiceScoreKind {}
impl ::core::clone::Clone for GameServiceScoreKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GameServiceScoreKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GameServiceScoreKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GameServiceScoreKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameServiceScoreKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GameServiceScoreKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceScoreKind;i4)");
}
