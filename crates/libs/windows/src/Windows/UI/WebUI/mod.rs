#[cfg(feature = "UI_WebUI_Core")]
#[doc = "Required features: `\"UI_WebUI_Core\"`"]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivatedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivatedDeferral {
    type Vtable = IActivatedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivatedDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3bd1978_a431_49d8_a76a_395a4e03dcf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivatedEventArgsDeferral(::windows_core::IUnknown);
impl IActivatedEventArgsDeferral {
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IActivatedEventArgsDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IActivatedEventArgsDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IActivatedEventArgsDeferral {
    type Vtable = IActivatedEventArgsDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivatedEventArgsDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca6d5f74_63c2_44a6_b97b_d9a03c20bc9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ActivatedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivatedOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivatedOperation {
    type Vtable = IActivatedOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivatedOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6a0b4bc_c6ca_42fd_9818_71904e45fed7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHtmlPrintDocumentSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHtmlPrintDocumentSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcea6469a_0e05_467a_abc9_36ec1d4cdcb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlPrintDocumentSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintContent) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintContent) -> ::windows_core::HRESULT,
    pub LeftMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetLeftMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub TopMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetTopMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub RightMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRightMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub BottomMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetBottomMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub EnableHeaderFooter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableHeaderFooter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShrinkToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShrinkToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PercentScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPercentScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalepercent: f32) -> ::windows_core::HRESULT,
    pub PageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrySetPageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpagerange: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INewWebUIViewCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INewWebUIViewCreatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8e1b216_be2b_4c9e_85e7_083143ec4be7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INewWebUIViewCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WebUIView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub ActivatedEventArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    ActivatedEventArgs: usize,
    pub HasPendingNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIActivationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics {
    type Vtable = IWebUIActivationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIActivationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x351b86bd_43b3_482b_85db_35d87b517ad9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub Suspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    Suspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub Resuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resuming: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResuming: usize,
    #[cfg(feature = "Foundation")]
    pub Navigated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Navigated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIActivationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics2 {
    type Vtable = IWebUIActivationStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIActivationStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8e88696_4d78_4aa4_8f06_2a9eadc6c40a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    LeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLeavingBackground: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    EnteredBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnteredBackground: usize,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIActivationStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics3 {
    type Vtable = IWebUIActivationStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIActivationStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91abb686_1af5_4445_b49f_9459f40fc8de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launcharguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, launcharguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIActivationStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics4 {
    type Vtable = IWebUIActivationStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIActivationStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e391429_183f_478d_8a25_67f80d03935b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub NewWebUIViewCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewWebUIViewCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNewWebUIViewCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNewWebUIViewCreated: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIBackgroundTaskInstance(::windows_core::IUnknown);
impl IWebUIBackgroundTaskInstance {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSucceeded)(::windows_core::Interface::as_raw(this), succeeded).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IWebUIBackgroundTaskInstance, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IWebUIBackgroundTaskInstance {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWebUIBackgroundTaskInstance {
    type Vtable = IWebUIBackgroundTaskInstance_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIBackgroundTaskInstance {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23f12c25_e2f7_4741_bc9c_394595de24dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstance_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, succeeded: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIBackgroundTaskInstanceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIBackgroundTaskInstanceStatics {
    type Vtable = IWebUIBackgroundTaskInstanceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIBackgroundTaskInstanceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c7a5291_19ae_4ca3_b94b_fe4ec744a740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstanceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUINavigatedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUINavigatedDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd804204d_831f_46e2_b432_3afce211f962);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUINavigatedEventArgs(::windows_core::IUnknown);
impl IWebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows_core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebUINavigatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IWebUINavigatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUINavigatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa75841b8_2499_4030_a69d_15d2d9cfe524);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NavigatedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUINavigatedOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUINavigatedOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a965f08_8182_4a89_ab67_8492e8750d4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIView {
    type Vtable = IWebUIView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIView {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6783f64f_52da_4fd7_be69_8ef6284b423c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    pub IgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebUIViewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIViewStatics {
    type Vtable = IWebUIViewStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebUIViewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb591e668_8e59_44f9_8803_1b24c9149d30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIViewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithUriAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ActivatedDeferral(::windows_core::IUnknown);
impl ActivatedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for ActivatedDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ActivatedDeferral {
    type Vtable = IActivatedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ActivatedDeferral {
    const IID: ::windows_core::GUID = <IActivatedDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ActivatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedDeferral";
}
::windows_core::imp::interface_hierarchy!(ActivatedDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ActivatedOperation(::windows_core::IUnknown);
impl ActivatedOperation {
    pub fn GetDeferral(&self) -> ::windows_core::Result<ActivatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ActivatedOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ActivatedOperation {
    type Vtable = IActivatedOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ActivatedOperation {
    const IID: ::windows_core::GUID = <IActivatedOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ActivatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedOperation";
}
::windows_core::imp::interface_hierarchy!(ActivatedOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Background\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Background"))]
    pub fn TaskInstance(&self) -> ::windows_core::Result<super::super::ApplicationModel::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskInstance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for BackgroundActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for BackgroundActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.BackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(BackgroundActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct EnteredBackgroundEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventArgs {
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for EnteredBackgroundEventArgs {
    type Vtable = super::super::ApplicationModel::IEnteredBackgroundEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for EnteredBackgroundEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::IEnteredBackgroundEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.EnteredBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
::windows_core::imp::interface_hierarchy!(EnteredBackgroundEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::IEnteredBackgroundEventArgs> for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Send for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Sync for EnteredBackgroundEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HtmlPrintDocumentSource(::windows_core::IUnknown);
impl HtmlPrintDocumentSource {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows_core::Result<PrintContent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContent(&self, value: PrintContent) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LeftMargin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLeftMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLeftMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TopMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TopMargin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTopMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTopMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RightMargin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRightMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRightMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BottomMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BottomMargin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBottomMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBottomMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableHeaderFooter(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnableHeaderFooter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnableHeaderFooter(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableHeaderFooter)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShrinkToFit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShrinkToFit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShrinkToFit(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShrinkToFit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PercentScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PercentScale)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPercentScale(&self, scalepercent: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPercentScale)(::windows_core::Interface::as_raw(this), scalepercent).ok() }
    }
    pub fn PageRange(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageRange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetPageRange(&self, strpagerange: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetPageRange)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(strpagerange), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for HtmlPrintDocumentSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HtmlPrintDocumentSource {
    const IID: ::windows_core::GUID = <IHtmlPrintDocumentSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HtmlPrintDocumentSource {
    const NAME: &'static str = "Windows.UI.WebUI.HtmlPrintDocumentSource";
}
::windows_core::imp::interface_hierarchy!(HtmlPrintDocumentSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for HtmlPrintDocumentSource {}
#[cfg(feature = "Graphics_Printing")]
impl ::windows_core::CanTryInto<super::super::Graphics::Printing::IPrintDocumentSource> for HtmlPrintDocumentSource {}
unsafe impl ::core::marker::Send for HtmlPrintDocumentSource {}
unsafe impl ::core::marker::Sync for HtmlPrintDocumentSource {}
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LeavingBackgroundEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventArgs {
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for LeavingBackgroundEventArgs {
    type Vtable = super::super::ApplicationModel::ILeavingBackgroundEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for LeavingBackgroundEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::ILeavingBackgroundEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.LeavingBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
::windows_core::imp::interface_hierarchy!(LeavingBackgroundEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::ILeavingBackgroundEventArgs> for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Send for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Sync for LeavingBackgroundEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NewWebUIViewCreatedEventArgs(::windows_core::IUnknown);
impl NewWebUIViewCreatedEventArgs {
    pub fn WebUIView(&self) -> ::windows_core::Result<WebUIView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebUIView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActivatedEventArgs(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedEventArgs)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasPendingNavigate(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasPendingNavigate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NewWebUIViewCreatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NewWebUIViewCreatedEventArgs {
    const IID: ::windows_core::GUID = <INewWebUIViewCreatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NewWebUIViewCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.NewWebUIViewCreatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(NewWebUIViewCreatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SuspendingDeferral(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingDeferral {
    #[doc = "Required features: `\"ApplicationModel\"`"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for SuspendingDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingDeferral {
    type Vtable = super::super::ApplicationModel::ISuspendingDeferral_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for SuspendingDeferral {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::ISuspendingDeferral as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingDeferral";
}
#[cfg(feature = "ApplicationModel")]
::windows_core::imp::interface_hierarchy!(SuspendingDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::ISuspendingDeferral> for SuspendingDeferral {}
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SuspendingEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventArgs {
    #[doc = "Required features: `\"ApplicationModel\"`"]
    #[cfg(feature = "ApplicationModel")]
    pub fn SuspendingOperation(&self) -> ::windows_core::Result<super::super::ApplicationModel::SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuspendingOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingEventArgs {
    type Vtable = super::super::ApplicationModel::ISuspendingEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for SuspendingEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::ISuspendingEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingEventArgs";
}
#[cfg(feature = "ApplicationModel")]
::windows_core::imp::interface_hierarchy!(SuspendingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::ISuspendingEventArgs> for SuspendingEventArgs {}
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SuspendingOperation(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingOperation {
    #[doc = "Required features: `\"ApplicationModel\"`"]
    #[cfg(feature = "ApplicationModel")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::ApplicationModel::SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for SuspendingOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingOperation {
    type Vtable = super::super::ApplicationModel::ISuspendingOperation_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for SuspendingOperation {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::ISuspendingOperation as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingOperation";
}
#[cfg(feature = "ApplicationModel")]
::windows_core::imp::interface_hierarchy!(SuspendingOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::ISuspendingOperation> for SuspendingOperation {}
pub struct WebUIApplication;
impl WebUIApplication {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<ActivatedEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Activated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveActivated)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn Suspending<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<SuspendingEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Suspending)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuspending(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSuspending)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Resuming<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<ResumingEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Resuming)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResuming(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveResuming)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Navigated<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<NavigatedEventHandler>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Navigated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigated(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveNavigated)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn LeavingBackground<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<LeavingBackgroundEventHandler>,
    {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LeavingBackground)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLeavingBackground(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveLeavingBackground)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"ApplicationModel\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn EnteredBackground<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<EnteredBackgroundEventHandler>,
    {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnteredBackground)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnteredBackground(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveEnteredBackground)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).EnablePrelaunch)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestRestartAsync(launcharguments: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>> {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestRestartAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(launcharguments), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System"))]
    pub fn RequestRestartForUserAsync<P0>(user: P0, launcharguments: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestRestartForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(launcharguments), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NewWebUIViewCreated<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs>>,
    {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewWebUIViewCreated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWebUIViewCreated(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveNewWebUIViewCreated)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundActivatedEventHandler>,
    {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundActivated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveBackgroundActivated)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics<R, F: FnOnce(&IWebUIActivationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics2<R, F: FnOnce(&IWebUIActivationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics3<R, F: FnOnce(&IWebUIActivationStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics4<R, F: FnOnce(&IWebUIActivationStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebUIApplication, IWebUIActivationStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for WebUIApplication {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIApplication";
}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIAppointmentsProviderAddAppointmentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn AddAppointmentOperation(&self) -> ::windows_core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderAddAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn RemoveAppointmentOperation(&self) -> ::windows_core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows_core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoamingId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn TimeToShow(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeToShow)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
pub struct WebUIBackgroundTaskInstance;
impl WebUIBackgroundTaskInstance {
    pub fn Current() -> ::windows_core::Result<IWebUIBackgroundTaskInstance> {
        Self::IWebUIBackgroundTaskInstanceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUIBackgroundTaskInstanceStatics<R, F: FnOnce(&IWebUIBackgroundTaskInstanceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebUIBackgroundTaskInstance, IWebUIBackgroundTaskInstanceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for WebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstance";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIBackgroundTaskInstanceRuntimeClass(::windows_core::IUnknown);
impl WebUIBackgroundTaskInstanceRuntimeClass {
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Task(&self) -> ::windows_core::Result<super::super::ApplicationModel::Background::BackgroundTaskRegistration> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Progress(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SetProgress(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TriggerDetails(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation"))]
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::Background::BackgroundTaskCanceledEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), cancelhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation"))]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SuspendedCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuspendedCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::ApplicationModel::Background::BackgroundTaskDeferral> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSucceeded)(::windows_core::Interface::as_raw(this), succeeded).ok() }
    }
}
impl ::windows_core::RuntimeType for WebUIBackgroundTaskInstanceRuntimeClass {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WebUIBackgroundTaskInstanceRuntimeClass {
    type Vtable = IWebUIBackgroundTaskInstance_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebUIBackgroundTaskInstanceRuntimeClass {
    const IID: ::windows_core::GUID = <IWebUIBackgroundTaskInstance as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebUIBackgroundTaskInstanceRuntimeClass {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass";
}
::windows_core::imp::interface_hierarchy!(WebUIBackgroundTaskInstanceRuntimeClass, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Background")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Background::IBackgroundTaskInstance> for WebUIBackgroundTaskInstanceRuntimeClass {}
impl ::windows_core::CanTryInto<IWebUIBackgroundTaskInstance> for WebUIBackgroundTaskInstanceRuntimeClass {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIBarcodeScannerPreviewActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIBarcodeScannerPreviewActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ConnectionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIBarcodeScannerPreviewActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs> for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUICachedFileUpdaterActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICachedFileUpdaterActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Provider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Provider"))]
    pub fn CachedFileUpdaterUI(&self) -> ::windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CachedFileUpdaterUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUICachedFileUpdaterActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUICachedFileUpdaterActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUICachedFileUpdaterActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUICachedFileUpdaterActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUICachedFileUpdaterActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUICachedFileUpdaterActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs> for WebUICachedFileUpdaterActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUICameraSettingsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICameraSettingsActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceExtension(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceExtension)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUICameraSettingsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUICameraSettingsActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUICameraSettingsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUICameraSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUICameraSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs> for WebUICameraSettingsActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUICommandLineActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICommandLineActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Operation(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUICommandLineActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUICommandLineActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICommandLineActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUICommandLineActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs> for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUICommandLineActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIContactCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactCallActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIContactCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIContactCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIContactCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIContactCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIContactCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactActivatedEventArgs> for WebUIContactCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs> for WebUIContactCallActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIContactMapActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMapActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Address(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIContactMapActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactMapActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIContactMapActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIContactMapActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIContactMapActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIContactMapActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactActivatedEventArgs> for WebUIContactMapActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs> for WebUIContactMapActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIContactMessageActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMessageActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactMessageActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIContactMessageActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIContactMessageActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIContactMessageActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIContactMessageActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactActivatedEventArgs> for WebUIContactMessageActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs> for WebUIContactMessageActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIContactPanelActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPanelActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn ContactPanel(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactPanel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactPanelActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIContactPanelActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIContactPanelActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs> for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIContactPanelActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIContactPickerActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPickerActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts_Provider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts_Provider"))]
    pub fn ContactPickerUI(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactPickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactPickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIContactPickerActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIContactPickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIContactPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIContactPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs> for WebUIContactPickerActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIContactPostActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPostActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIContactPostActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactPostActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIContactPostActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIContactPostActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIContactPostActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIContactPostActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactActivatedEventArgs> for WebUIContactPostActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs> for WebUIContactPostActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIContactVideoCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactVideoCallActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactVideoCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIContactVideoCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIContactVideoCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIContactVideoCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIContactVideoCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactActivatedEventArgs> for WebUIContactVideoCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs> for WebUIContactVideoCallActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIDeviceActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDeviceActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn DeviceInformationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIDeviceActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIDeviceActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIDeviceActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDeviceActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIDeviceActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIDeviceActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIDeviceActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIDeviceActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUIDeviceActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs> for WebUIDeviceActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIDevicePairingActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDevicePairingActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Enumeration\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Enumeration"))]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIDevicePairingActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIDevicePairingActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIDevicePairingActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIDevicePairingActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIDevicePairingActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIDevicePairingActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs> for WebUIDevicePairingActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIDialReceiverActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDialReceiverActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIDialReceiverActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIDialReceiverActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIDialReceiverActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIDialReceiverActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIDialReceiverActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIFileActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Search"))]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIFileActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIFileActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIFileActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IFileActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIFileActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIFileActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIFileActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIFileActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIFileActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUIFileActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileActivatedEventArgs> for WebUIFileActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles> for WebUIFileActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIFileOpenPickerActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileOpenPickerActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Pickers_Provider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Pickers_Provider"))]
    pub fn FileOpenPickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileOpenPickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIFileOpenPickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIFileOpenPickerActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIFileOpenPickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIFileOpenPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIFileOpenPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIFileOpenPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs> for WebUIFileOpenPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2> for WebUIFileOpenPickerActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIFileOpenPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFileOpenPickerContinuationEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeType for WebUIFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::Interface for WebUIFileOpenPickerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs_Vtbl;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::ComInterface for WebUIFileOpenPickerContinuationEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for WebUIFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
::windows_core::imp::interface_hierarchy!(WebUIFileOpenPickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIFileOpenPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIFileOpenPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIFileOpenPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> for WebUIFileOpenPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs> for WebUIFileOpenPickerContinuationEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIFileSavePickerActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileSavePickerActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Pickers_Provider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Pickers_Provider"))]
    pub fn FileSavePickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileSavePickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIFileSavePickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIFileSavePickerActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIFileSavePickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIFileSavePickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIFileSavePickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIFileSavePickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs> for WebUIFileSavePickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2> for WebUIFileSavePickerActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIFileSavePickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFileSavePickerContinuationEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeType for WebUIFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::Interface for WebUIFileSavePickerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs_Vtbl;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::ComInterface for WebUIFileSavePickerContinuationEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for WebUIFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
::windows_core::imp::interface_hierarchy!(WebUIFileSavePickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIFileSavePickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIFileSavePickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIFileSavePickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> for WebUIFileSavePickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs> for WebUIFileSavePickerContinuationEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIFolderPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFolderPickerContinuationEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeType for WebUIFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::Interface for WebUIFolderPickerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs_Vtbl;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::ComInterface for WebUIFolderPickerContinuationEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for WebUIFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
::windows_core::imp::interface_hierarchy!(WebUIFolderPickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIFolderPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIFolderPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIFolderPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> for WebUIFolderPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs> for WebUIFolderPickerContinuationEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUILaunchActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILaunchActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileActivatedInfo(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::TileActivatedInfo> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileActivatedInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PrelaunchActivated(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrelaunchActivated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUILaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILaunchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUILaunchActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUILaunchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs> for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2> for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs> for WebUILaunchActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUILockScreenActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Info(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Info)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILockScreenActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUILockScreenActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUILockScreenActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUILockScreenActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUILockScreenActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUILockScreenActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUILockScreenActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs> for WebUILockScreenActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUILockScreenCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenCallActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Calls\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Calls"))]
    pub fn CallUI(&self) -> ::windows_core::Result<super::super::ApplicationModel::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILockScreenCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUILockScreenCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUILockScreenCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUILockScreenCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUILockScreenComponentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenComponentActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUILockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILockScreenComponentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUILockScreenComponentActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUILockScreenComponentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUILockScreenComponentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUILockScreenComponentActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUINavigatedDeferral(::windows_core::IUnknown);
impl WebUINavigatedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for WebUINavigatedDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebUINavigatedDeferral {
    const IID: ::windows_core::GUID = <IWebUINavigatedDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebUINavigatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedDeferral";
}
::windows_core::imp::interface_hierarchy!(WebUINavigatedDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUINavigatedEventArgs(::windows_core::IUnknown);
impl WebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows_core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WebUINavigatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebUINavigatedEventArgs {
    const IID: ::windows_core::GUID = <IWebUINavigatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WebUINavigatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebUINavigatedEventArgs> for WebUINavigatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUINavigatedOperation(::windows_core::IUnknown);
impl WebUINavigatedOperation {
    pub fn GetDeferral(&self) -> ::windows_core::Result<WebUINavigatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WebUINavigatedOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebUINavigatedOperation {
    const IID: ::windows_core::GUID = <IWebUINavigatedOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebUINavigatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedOperation";
}
::windows_core::imp::interface_hierarchy!(WebUINavigatedOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIPhoneCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPhoneCallActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPhoneCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIPhoneCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIPhoneCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs> for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIPhoneCallActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIPrint3DWorkflowActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrint3DWorkflowActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Printers_Extensions\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Printers_Extensions"))]
    pub fn Workflow(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Workflow)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPrint3DWorkflowActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIPrint3DWorkflowActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIPrint3DWorkflowActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIPrint3DWorkflowActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIPrint3DWorkflowActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs> for WebUIPrint3DWorkflowActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIPrintTaskSettingsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintTaskSettingsActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Printers_Extensions\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Printers_Extensions"))]
    pub fn Configuration(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPrintTaskSettingsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIPrintTaskSettingsActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIPrintTaskSettingsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIPrintTaskSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIPrintTaskSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs> for WebUIPrintTaskSettingsActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIPrintWorkflowForegroundTaskActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIPrintWorkflowForegroundTaskActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIProtocolActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIProtocolActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIProtocolActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIProtocolActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIProtocolActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIProtocolActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIProtocolActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIProtocolActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUIProtocolActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs> for WebUIProtocolActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for WebUIProtocolActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIProtocolForResultsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolForResultsActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn ProtocolForResultsOperation(&self) -> ::windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolForResultsOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIProtocolForResultsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIProtocolForResultsActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIProtocolForResultsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIRestrictedLaunchActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIRestrictedLaunchActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SharedContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SharedContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIRestrictedLaunchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIRestrictedLaunchActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIRestrictedLaunchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIRestrictedLaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIRestrictedLaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIRestrictedLaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs> for WebUIRestrictedLaunchActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUISearchActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUISearchActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Search\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Search"))]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<super::super::ApplicationModel::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUISearchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUISearchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ISearchActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUISearchActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::ISearchActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUISearchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUISearchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUISearchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUISearchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> for WebUISearchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ISearchActivatedEventArgs> for WebUISearchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails> for WebUISearchActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIShareTargetActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIShareTargetActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_DataTransfer_ShareTarget\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    pub fn ShareOperation(&self) -> ::windows_core::Result<super::super::ApplicationModel::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShareOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIShareTargetActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIShareTargetActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIShareTargetActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIShareTargetActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIShareTargetActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIShareTargetActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs> for WebUIShareTargetActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIStartupTaskActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIStartupTaskActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIStartupTaskActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIStartupTaskActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIStartupTaskActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs> for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIStartupTaskActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIToastNotificationActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIToastNotificationActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn UserInput(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIToastNotificationActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIToastNotificationActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIToastNotificationActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIToastNotificationActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIToastNotificationActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIToastNotificationActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs> for WebUIToastNotificationActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIUserDataAccountProviderActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIUserDataAccountProviderActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_UserDataAccounts_Provider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_UserDataAccounts_Provider"))]
    pub fn Operation(&self) -> ::windows_core::Result<super::super::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIUserDataAccountProviderActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIUserDataAccountProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIUserDataAccountProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIUserDataAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIUserDataAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs> for WebUIUserDataAccountProviderActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIView(::windows_core::IUnknown);
impl WebUIView {
    pub fn ApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WebUIView, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Activated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActivated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IgnoreApplicationContentUriRulesNavigationRestrictions(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IgnoreApplicationContentUriRulesNavigationRestrictions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIgnoreApplicationContentUriRulesNavigationRestrictions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<WebUIView>> {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithUriAsync<P0>(uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithUriAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn Source(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn SetSource<P0>(&self, source: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn DocumentTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn CanGoBack(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanGoBack)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn CanGoForward(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanGoForward)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn SetDefaultBackgroundColor(&self, value: super::Color) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows_core::Result<super::Color> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBackgroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn ContainsFullScreenElement(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElement)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn Settings(&self) -> ::windows_core::Result<super::super::Web::UI::WebViewControlSettings> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Settings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_UI"))]
    pub fn DeferredPermissionRequests(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Web::UI::WebViewControlDeferredPermissionRequest>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeferredPermissionRequests)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn GoForward(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GoForward)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn GoBack(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GoBack)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn Refresh(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Refresh)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn Navigate<P0>(&self, source: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Navigate)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn NavigateToString(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).NavigateToString)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(text)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigateToLocalStreamUri<P0, P1>(&self, source: P0, streamresolver: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::TryIntoParam<super::super::Web::IUriToStreamResolver>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).NavigateToLocalStreamUri)(::windows_core::Interface::as_raw(this), source.into_param().abi(), streamresolver.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Web_Http\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Web_Http", feature = "Web_UI"))]
    pub fn NavigateWithHttpRequestMessage<P0>(&self, requestmessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Web::Http::HttpRequestMessage>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).NavigateWithHttpRequestMessage)(::windows_core::Interface::as_raw(this), requestmessage.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_UI"))]
    pub fn InvokeScriptAsync<P0>(&self, scriptname: &::windows_core::HSTRING, arguments: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InvokeScriptAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(scriptname), arguments.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_UI"))]
    pub fn CapturePreviewToStreamAsync<P0>(&self, stream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapturePreviewToStreamAsync)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "Web_UI"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn BuildLocalStreamUri(&self, contentidentifier: &::windows_core::HSTRING, relativepath: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BuildLocalStreamUri)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contentidentifier), ::core::mem::transmute_copy(relativepath), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<super::super::Web::UI::WebViewControlDeferredPermissionRequest>) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GetDeferredPermissionRequestById)(::windows_core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigationStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNavigationStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ContentLoading<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlContentLoadingEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentLoading)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveContentLoading(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentLoading)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn DOMContentLoaded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DOMContentLoaded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveDOMContentLoaded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNavigationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameNavigationStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameNavigationStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameNavigationStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameContentLoading<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlContentLoadingEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameContentLoading)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameContentLoading(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameContentLoading)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameDOMContentLoaded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameDOMContentLoaded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameDOMContentLoaded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameNavigationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameNavigationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameNavigationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ScriptNotify<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlScriptNotifyEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScriptNotify)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveScriptNotify(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScriptNotify)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn LongRunningScriptDetected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlLongRunningScriptDetectedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LongRunningScriptDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveLongRunningScriptDetected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLongRunningScriptDetected)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnsafeContentWarningDisplaying<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnsafeContentWarningDisplaying)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnviewableContentIdentified<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlUnviewableContentIdentifiedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnviewableContentIdentified)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnviewableContentIdentified(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnviewableContentIdentified)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn PermissionRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlPermissionRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PermissionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemovePermissionRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePermissionRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnsupportedUriSchemeIdentified<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NewWindowRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNewWindowRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewWindowRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNewWindowRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNewWindowRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ContainsFullScreenElementChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElementChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveContainsFullScreenElementChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn WebResourceRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlWebResourceRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebResourceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Web_UI\"`"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveWebResourceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWebResourceRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Web_UI\"`"]
    #[cfg(feature = "Web_UI")]
    pub fn AddInitializeScript(&self, script: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Web::UI::IWebViewControl2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddInitializeScript)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(script)).ok() }
    }
    #[doc(hidden)]
    pub fn IWebUIViewStatics<R, F: FnOnce(&IWebUIViewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebUIView, IWebUIViewStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WebUIView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WebUIView {
    type Vtable = IWebUIView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebUIView {
    const IID: ::windows_core::GUID = <IWebUIView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebUIView {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIView";
}
::windows_core::imp::interface_hierarchy!(WebUIView, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Web_UI")]
impl ::windows_core::CanTryInto<super::super::Web::UI::IWebViewControl> for WebUIView {}
#[cfg(feature = "Web_UI")]
impl ::windows_core::CanTryInto<super::super::Web::UI::IWebViewControl2> for WebUIView {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIVoiceCommandActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIVoiceCommandActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Media_SpeechRecognition\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Media_SpeechRecognition"))]
    pub fn Result(&self) -> ::windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIVoiceCommandActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIVoiceCommandActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIVoiceCommandActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIVoiceCommandActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIVoiceCommandActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIVoiceCommandActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs> for WebUIVoiceCommandActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`"]
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIWalletActionActivatedEventArgs(::windows_core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIWalletActionActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Wallet\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Wallet", feature = "deprecated"))]
    pub fn ActionKind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
    pub fn ActionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeType for WebUIWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::Interface for WebUIWalletActionActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs_Vtbl;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::ComInterface for WebUIWalletActionActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for WebUIWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
::windows_core::imp::interface_hierarchy!(WebUIWalletActionActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIWalletActionActivatedEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIWalletActionActivatedEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs> for WebUIWalletActionActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIWebAccountProviderActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAccountProviderActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"System\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Security_Authentication_Web_Provider\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Security_Authentication_Web_Provider"))]
    pub fn Operation(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIWebAccountProviderActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIWebAccountProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIWebAccountProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIWebAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIWebAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for WebUIWebAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs> for WebUIWebAccountProviderActivatedEventArgs {}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebUIWebAuthenticationBrokerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAuthenticationBrokerContinuationEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Security_Authentication_Web\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Security_Authentication_Web"))]
    pub fn WebAuthenticationResult(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAuthenticationResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const IID: ::windows_core::GUID = <super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
::windows_core::imp::interface_hierarchy!(WebUIWebAuthenticationBrokerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for WebUIWebAuthenticationBrokerContinuationEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<IActivatedEventArgsDeferral> for WebUIWebAuthenticationBrokerContinuationEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> for WebUIWebAuthenticationBrokerContinuationEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs> for WebUIWebAuthenticationBrokerContinuationEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintContent(pub i32);
impl PrintContent {
    pub const AllPages: Self = Self(0i32);
    pub const CurrentPage: Self = Self(1i32);
    pub const CustomPageRange: Self = Self(2i32);
    pub const CurrentSelection: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintContent {}
impl ::core::clone::Clone for PrintContent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintContent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintContent {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintContent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.PrintContent;i4)");
}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ActivatedEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl ActivatedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ActivatedEventHandlerBox::<F> { vtable: &ActivatedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<P0, P1>(&self, sender: P0, eventargs: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<super::super::ApplicationModel::Activation::IActivatedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), eventargs.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct ActivatedEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ActivatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ActivatedEventHandlerBox<F> {
    const VTABLE: ActivatedEventHandler_Vtbl = ActivatedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ActivatedEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&eventargs)).into()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for ActivatedEventHandler {
    type Vtable = ActivatedEventHandler_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for ActivatedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50f1e730_c5d1_4b6b_9adb_8a11756be29c);
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for ActivatedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct ActivatedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Invoke: usize,
}
#[doc = "Required features: `\"ApplicationModel_Activation\"`"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundActivatedEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundActivatedEventHandlerBox::<F> { vtable: &BackgroundActivatedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<P0, P1>(&self, sender: P0, eventargs: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), eventargs.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct BackgroundActivatedEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundActivatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> BackgroundActivatedEventHandlerBox<F> {
    const VTABLE: BackgroundActivatedEventHandler_Vtbl = BackgroundActivatedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <BackgroundActivatedEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&eventargs)).into()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for BackgroundActivatedEventHandler {
    type Vtable = BackgroundActivatedEventHandler_Vtbl;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::ComInterface for BackgroundActivatedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedb19fbb_0761_47cc_9a77_24d7072965ca);
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeType for BackgroundActivatedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundActivatedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Invoke: usize,
}
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct EnteredBackgroundEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = EnteredBackgroundEventHandlerBox::<F> { vtable: &EnteredBackgroundEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "Required features: `\"ApplicationModel\"`"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<super::super::ApplicationModel::IEnteredBackgroundEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct EnteredBackgroundEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const EnteredBackgroundEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> EnteredBackgroundEventHandlerBox<F> {
    const VTABLE: EnteredBackgroundEventHandler_Vtbl = EnteredBackgroundEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <EnteredBackgroundEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for EnteredBackgroundEventHandler {
    type Vtable = EnteredBackgroundEventHandler_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for EnteredBackgroundEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b09a173_b68e_4def_88c1_8de84e5aab2f);
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for EnteredBackgroundEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct EnteredBackgroundEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LeavingBackgroundEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = LeavingBackgroundEventHandlerBox::<F> { vtable: &LeavingBackgroundEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "Required features: `\"ApplicationModel\"`"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<super::super::ApplicationModel::ILeavingBackgroundEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct LeavingBackgroundEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const LeavingBackgroundEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> LeavingBackgroundEventHandlerBox<F> {
    const VTABLE: LeavingBackgroundEventHandler_Vtbl = LeavingBackgroundEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <LeavingBackgroundEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for LeavingBackgroundEventHandler {
    type Vtable = LeavingBackgroundEventHandler_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for LeavingBackgroundEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00b4ccd9_7a9c_4b6b_9ac4_13474f268bc4);
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for LeavingBackgroundEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct LeavingBackgroundEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigatedEventHandler(pub ::windows_core::IUnknown);
impl NavigatedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&IWebUINavigatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = NavigatedEventHandlerBox::<F> { vtable: &NavigatedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<IWebUINavigatedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.try_into_param()?.abi()).ok() }
    }
}
#[repr(C)]
struct NavigatedEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&IWebUINavigatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const NavigatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&IWebUINavigatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> NavigatedEventHandlerBox<F> {
    const VTABLE: NavigatedEventHandler_Vtbl = NavigatedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <NavigatedEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
unsafe impl ::windows_core::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigatedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7af46fe6_40ca_4e49_a7d6_dbdb330cd1a3);
}
impl ::windows_core::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResumingEventHandler(pub ::windows_core::IUnknown);
impl ResumingEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ResumingEventHandlerBox::<F> { vtable: &ResumingEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ResumingEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ResumingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ResumingEventHandlerBox<F> {
    const VTABLE: ResumingEventHandler_Vtbl = ResumingEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ResumingEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender)).into()
    }
}
unsafe impl ::windows_core::Interface for ResumingEventHandler {
    type Vtable = ResumingEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResumingEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26599ba9_a22d_4806_a728_acadc1d075fa);
}
impl ::windows_core::RuntimeType for ResumingEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ResumingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"ApplicationModel\"`"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SuspendingEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::ISuspendingEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SuspendingEventHandlerBox::<F> { vtable: &SuspendingEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "Required features: `\"ApplicationModel\"`"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<super::super::ApplicationModel::ISuspendingEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct SuspendingEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::ISuspendingEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SuspendingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>, ::core::option::Option<&super::super::ApplicationModel::ISuspendingEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> SuspendingEventHandlerBox<F> {
    const VTABLE: SuspendingEventHandler_Vtbl = SuspendingEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <SuspendingEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&e)).into()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingEventHandler {
    type Vtable = SuspendingEventHandler_Vtbl;
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::ComInterface for SuspendingEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x509c429c_78e2_4883_abc8_8960dcde1b5c);
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeType for SuspendingEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct SuspendingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
