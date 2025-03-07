#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICurrentTimeChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICurrentTimeChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99711324_edc7_4bf5_91f6_3c8627db59e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentTimeChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMuteChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMuteChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4b4f5f6_af1f_4f1e_b437_7da32400e1d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToConnection(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToConnection {
    type Vtable = IPlayToConnection_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x112fbfc8_f235_4fde_8d41_9bf27c9e9a40);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Transferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Transferred: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveTransferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveTransferred: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Error: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveError: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToConnectionErrorEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToConnectionErrorEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf5eada6_88e6_445f_9d40_d9b9f8939896);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionErrorEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionError) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Code: usize,
    #[cfg(feature = "deprecated")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Message: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToConnectionStateChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToConnectionStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68c4b50f_0c20_4980_8602_58c62238d423);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub PreviousState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousState: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentState: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToConnectionTransferredEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToConnectionTransferredEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfae3193a_0683_47d9_8df0_18cbb48984d8);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionTransferredEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub PreviousSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PreviousSource: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToManager(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToManager {
    type Vtable = IPlayToManager_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf56a206e_1b77_42ef_8f0d_b949f8d9b260);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SourceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SourceSelected: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSourceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSourceSelected: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultSourceSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultSourceSelection: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultSourceSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultSourceSelection: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToManagerStatics {
    type Vtable = IPlayToManagerStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64e6a887_3982_4f3b_ba20_6155e435325b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
    #[cfg(feature = "deprecated")]
    pub ShowPlayToUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowPlayToUI: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToReceiver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayToReceiver {
    type Vtable = IPlayToReceiver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayToReceiver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac15cf47_a162_4aa6_af1b_3aa35f3b9069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToReceiver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PlayRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlayRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlayRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PauseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePauseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePauseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SourceChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentTimeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentTimeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentTimeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentTimeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub MuteChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MuteChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMuteChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMuteChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub VolumeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VolumeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVolumeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVolumeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub TimeUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimeUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimeUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub StopRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopRequested: usize,
    pub NotifyVolumeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: f64, mute: bool) -> ::windows_core::HRESULT,
    pub NotifyRateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rate: f64) -> ::windows_core::HRESULT,
    pub NotifyLoadedMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotifyTimeUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currenttime: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyTimeUpdate: usize,
    #[cfg(feature = "Foundation")]
    pub NotifyDurationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyDurationChange: usize,
    pub NotifySeeking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifySeeked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSupportsImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SupportsImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToSource(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToSource {
    type Vtable = IPlayToSource_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f138a08_fbb7_4b09_8356_aa5f4e335c31);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Connection: usize,
    #[cfg(feature = "deprecated")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Next: usize,
    #[cfg(feature = "deprecated")]
    pub SetNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetNext: usize,
    #[cfg(feature = "deprecated")]
    pub PlayNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlayNext: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToSourceDeferral(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToSourceDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4100891d_278e_4f29_859b_a9e501053e7d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Complete: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToSourceRequest(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToSourceRequest {
    type Vtable = IPlayToSourceRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToSourceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8584665_64f4_44a0_ac0d_468d2b8fda83);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Deadline: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayErrorString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayErrorString: usize,
    #[cfg(feature = "deprecated")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDeferral: usize,
    #[cfg(feature = "deprecated")]
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSource: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToSourceRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToSourceRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5cdc330_29df_4ec6_9da9_9fbdfcfc1b3e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub SourceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SourceRequest: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToSourceSelectedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToSourceSelectedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c9d8511_5202_4dcb_8c67_abda12bb3c12);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceSelectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    FriendlyName: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    Icon: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsImage: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsAudio: usize,
    #[cfg(feature = "deprecated")]
    pub SupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportsVideo: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayToSourceWithPreferredSourceUri(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IPlayToSourceWithPreferredSourceUri {
    type Vtable = IPlayToSourceWithPreferredSourceUri_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IPlayToSourceWithPreferredSourceUri {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaab253eb_3301_4dc4_afba_b2f2ed9635a0);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceWithPreferredSourceUri_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PreferredSourceUri: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPreferredSourceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaybackRateChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaybackRateChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f5661ae_2c88_4cca_8540_d586095d13a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Rate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISourceChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISourceChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb3f3a96_7aa6_4a8b_86e7_54f6c6d34f64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Album: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Genre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation")]
    pub Rating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rating: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVolumeChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVolumeChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f026d5c_cf75_4c2b_913e_6d7c6c329179);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CurrentTimeChangeRequestedEventArgs(::windows_core::IUnknown);
impl CurrentTimeChangeRequestedEventArgs {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Time)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CurrentTimeChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CurrentTimeChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <ICurrentTimeChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CurrentTimeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CurrentTimeChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MuteChangeRequestedEventArgs(::windows_core::IUnknown);
impl MuteChangeRequestedEventArgs {
    pub fn Mute(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mute)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MuteChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MuteChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <IMuteChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MuteChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.MuteChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MuteChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToConnection(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnection {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn State(&self) -> ::windows_core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Transferred<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transferred)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveTransferred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTransferred)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Error<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveError(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveError)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToConnection {
    type Vtable = IPlayToConnection_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToConnection {
    const IID: ::windows_core::GUID = <IPlayToConnection as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToConnection {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnection";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnection {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnection {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToConnectionErrorEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionErrorEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Code(&self) -> ::windows_core::Result<PlayToConnectionError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Code)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToConnectionErrorEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToConnectionErrorEventArgs {
    const IID: ::windows_core::GUID = <IPlayToConnectionErrorEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToConnectionErrorEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionErrorEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToConnectionErrorEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionErrorEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionErrorEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToConnectionStateChangedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionStateChangedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PreviousState(&self) -> ::windows_core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentState(&self) -> ::windows_core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToConnectionStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IPlayToConnectionStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToConnectionStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionStateChangedEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToConnectionTransferredEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionTransferredEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PreviousSource(&self) -> ::windows_core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentSource(&self) -> ::windows_core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToConnectionTransferredEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToConnectionTransferredEventArgs {
    const IID: ::windows_core::GUID = <IPlayToConnectionTransferredEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToConnectionTransferredEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToConnectionTransferredEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionTransferredEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionTransferredEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToManager(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToManager {
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceSelected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceSelected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceSelected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceSelected)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetDefaultSourceSelection(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultSourceSelection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn DefaultSourceSelection(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultSourceSelection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows_core::Result<PlayToManager> {
        Self::IPlayToManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ShowPlayToUI() -> ::windows_core::Result<()> {
        Self::IPlayToManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowPlayToUI)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPlayToManagerStatics<R, F: FnOnce(&IPlayToManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayToManager, IPlayToManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToManager {
    type Vtable = IPlayToManager_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToManager {
    const IID: ::windows_core::GUID = <IPlayToManager as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToManager {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToManager";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToManager {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToReceiver(::windows_core::IUnknown);
impl PlayToReceiver {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayToReceiver, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PlayRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlayRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlayRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlayRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PauseRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PauseRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePauseRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePauseRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SourceChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRateChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRateChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackRateChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackRateChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentTimeChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentTimeChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentTimeChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentTimeChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MuteChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MuteChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMuteChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMuteChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn VolumeChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VolumeChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVolumeChangeRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVolumeChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimeUpdateRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeUpdateRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimeUpdateRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTimeUpdateRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StopRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NotifyVolumeChange(&self, volume: f64, mute: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyVolumeChange)(::windows_core::Interface::as_raw(this), volume, mute).ok() }
    }
    pub fn NotifyRateChange(&self, rate: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyRateChange)(::windows_core::Interface::as_raw(this), rate).ok() }
    }
    pub fn NotifyLoadedMetadata(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyLoadedMetadata)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyTimeUpdate(&self, currenttime: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyTimeUpdate)(::windows_core::Interface::as_raw(this), currenttime).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyDurationChange(&self, duration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyDurationChange)(::windows_core::Interface::as_raw(this), duration).ok() }
    }
    pub fn NotifySeeking(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifySeeking)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifySeeked(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifySeeked)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyPaused(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyPaused)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyPlaying(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyPlaying)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyEnded(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyEnded)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyError(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyError)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn NotifyStopped(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyStopped)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFriendlyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetSupportsImage(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSupportsImage)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsImage(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsImage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSupportsAudio)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsAudio(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsAudio)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSupportsVideo)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsVideo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsVideo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayToReceiver {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PlayToReceiver {
    type Vtable = IPlayToReceiver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayToReceiver {
    const IID: ::windows_core::GUID = <IPlayToReceiver as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayToReceiver {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToReceiver";
}
::windows_core::imp::interface_hierarchy!(PlayToReceiver, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToSource(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSource {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Connection(&self) -> ::windows_core::Result<PlayToConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Next(&self) -> ::windows_core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Next)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetNext<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PlayToSource>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNext)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PlayNext(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PlayNext)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PreferredSourceUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredSourceUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetPreferredSourceUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredSourceUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToSource {
    type Vtable = IPlayToSource_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToSource {
    const IID: ::windows_core::GUID = <IPlayToSource as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToSource {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSource";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSource {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSource {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToSourceDeferral(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceDeferral {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToSourceDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferral_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToSourceDeferral {
    const IID: ::windows_core::GUID = <IPlayToSourceDeferral as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToSourceDeferral {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceDeferral";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToSourceDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceDeferral {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceDeferral {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToSourceRequest(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceRequest {
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayErrorString(&self, errorstring: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DisplayErrorString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(errorstring)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<PlayToSourceDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PlayToSource>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToSourceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToSourceRequest {
    type Vtable = IPlayToSourceRequest_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToSourceRequest {
    const IID: ::windows_core::GUID = <IPlayToSourceRequest as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToSourceRequest {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequest";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToSourceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceRequest {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceRequest {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToSourceRequestedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceRequestedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SourceRequest(&self) -> ::windows_core::Result<PlayToSourceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToSourceRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToSourceRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPlayToSourceRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToSourceRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceRequestedEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayToSourceSelectedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceSelectedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn Icon(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Icon)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsImage(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsImage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsAudio(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsAudio)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsVideo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsVideo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToSourceSelectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for PlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for PlayToSourceSelectedEventArgs {
    const IID: ::windows_core::GUID = <IPlayToSourceSelectedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for PlayToSourceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(PlayToSourceSelectedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceSelectedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceSelectedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlaybackRateChangeRequestedEventArgs(::windows_core::IUnknown);
impl PlaybackRateChangeRequestedEventArgs {
    pub fn Rate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaybackRateChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPlaybackRateChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PlaybackRateChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SourceChangeRequestedEventArgs(::windows_core::IUnknown);
impl SourceChangeRequestedEventArgs {
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Author(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Author)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Album(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Album)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Genre(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Genre)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Date)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Rating(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rating)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SourceChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SourceChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <ISourceChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SourceChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.SourceChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SourceChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VolumeChangeRequestedEventArgs(::windows_core::IUnknown);
impl VolumeChangeRequestedEventArgs {
    pub fn Volume(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Volume)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for VolumeChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VolumeChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <IVolumeChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VolumeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.VolumeChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VolumeChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayToConnectionError(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionError {
    pub const None: Self = Self(0i32);
    pub const DeviceNotResponding: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionError {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionError {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PlayToConnectionError {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for PlayToConnectionError {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionError").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToConnectionError {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionError;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayToConnectionState(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for PlayToConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for PlayToConnectionState {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for PlayToConnectionState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionState;i4)");
}
