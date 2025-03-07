#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintBindingOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintBindingOptionDetails {
    type Vtable = IPrintBindingOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintBindingOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3f4cc98_9564_4f16_a055_a98b9a49e9d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBindingOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintBorderingOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintBorderingOptionDetails {
    type Vtable = IPrintBorderingOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintBorderingOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d73bc8f_fb53_4eb2_985f_1d91de0b7639);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBorderingOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCollationOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCollationOptionDetails {
    type Vtable = IPrintCollationOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCollationOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6abb166_a5a6_40dc_acc3_739f28f1e5d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCollationOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintColorModeOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintColorModeOptionDetails {
    type Vtable = IPrintColorModeOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintColorModeOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdba97704_f1d6_4843_a484_9b447cdcf3b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintColorModeOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCopiesOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCopiesOptionDetails {
    type Vtable = IPrintCopiesOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCopiesOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42053099_4339_4343_898d_2c47b5e0c341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCopiesOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomItemDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCustomItemDetails {
    type Vtable = IPrintCustomItemDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomItemDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5704b637_5c3a_449a_aa36_b3291b1192fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetItemDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ItemDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomItemListOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCustomItemListOptionDetails {
    type Vtable = IPrintCustomItemListOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomItemListOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5fafd88_58f2_4ebd_b90f_51e4f2944c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomItemListOptionDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCustomItemListOptionDetails2 {
    type Vtable = IPrintCustomItemListOptionDetails2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomItemListOptionDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9d6353d_651c_4a39_906e_1091a1801bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, description: ::std::mem::MaybeUninit<::windows_core::HSTRING>, icon: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AddItem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomItemListOptionDetails3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCustomItemListOptionDetails3 {
    type Vtable = IPrintCustomItemListOptionDetails3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomItemListOptionDetails3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fa1b53f_3c34_4868_a407_fc5eab259b21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomOptionDetails(::windows_core::IUnknown);
impl IPrintCustomOptionDetails {
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrintCustomOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintOptionDetails> for IPrintCustomOptionDetails {}
impl ::windows_core::RuntimeType for IPrintCustomOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IPrintCustomOptionDetails {
    type Vtable = IPrintCustomOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe32bde1c_28af_4b90_95da_a3acf320b929);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomTextOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCustomTextOptionDetails {
    type Vtable = IPrintCustomTextOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomTextOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ad171f8_c8bd_4905_9192_0d75136e8b31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetMaxCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomTextOptionDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCustomTextOptionDetails2 {
    type Vtable = IPrintCustomTextOptionDetails2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomTextOptionDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcea70b54_b977_4718_8338_7ed2b0d86fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintCustomToggleOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintCustomToggleOptionDetails {
    type Vtable = IPrintCustomToggleOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintCustomToggleOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9db4d514_e461_4608_8ee9_db6f5ed073c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomToggleOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintDuplexOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintDuplexOptionDetails {
    type Vtable = IPrintDuplexOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintDuplexOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcd94591_d4a4_44fa_b3fe_42e0ba28d5ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDuplexOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintHolePunchOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintHolePunchOptionDetails {
    type Vtable = IPrintHolePunchOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintHolePunchOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6de1f18_482c_4657_9d71_8ddddbea1e1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintHolePunchOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintItemListOptionDetails(::windows_core::IUnknown);
impl IPrintItemListOptionDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrintItemListOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintOptionDetails> for IPrintItemListOptionDetails {}
impl ::windows_core::RuntimeType for IPrintItemListOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IPrintItemListOptionDetails {
    type Vtable = IPrintItemListOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintItemListOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a2257bf_fe61_43d8_a24f_a3f6ab7320e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintItemListOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintMediaSizeOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintMediaSizeOptionDetails {
    type Vtable = IPrintMediaSizeOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintMediaSizeOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c8d5bcf_c0bf_47c8_b84a_628e7d0d1a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintMediaSizeOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintMediaTypeOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintMediaTypeOptionDetails {
    type Vtable = IPrintMediaTypeOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintMediaTypeOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8c7000b_abf3_4abc_8e86_22abc5744a43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintMediaTypeOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintNumberOptionDetails(::windows_core::IUnknown);
impl IPrintNumberOptionDetails {
    pub fn MinValue(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxValue(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrintNumberOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintOptionDetails> for IPrintNumberOptionDetails {}
impl ::windows_core::RuntimeType for IPrintNumberOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IPrintNumberOptionDetails {
    type Vtable = IPrintNumberOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintNumberOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d01bbaf_645c_4de9_965f_6fc6bbc47cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintNumberOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintOptionDetails(::windows_core::IUnknown);
impl IPrintOptionDetails {
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrintOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IPrintOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IPrintOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionType) -> ::windows_core::HRESULT,
    pub SetErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintOptionStates) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionStates) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintOrientationOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintOrientationOptionDetails {
    type Vtable = IPrintOrientationOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintOrientationOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46c38879_66e0_4da0_87b4_d25457824eb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOrientationOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintPageRangeOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintPageRangeOptionDetails {
    type Vtable = IPrintPageRangeOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintPageRangeOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a19e4b7_2be8_4aa7_9ea5_defbe8713b4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintQualityOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintQualityOptionDetails {
    type Vtable = IPrintQualityOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintQualityOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dd06ba1_ce1a_44e6_84f9_3a92ea1e3044);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintQualityOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintStapleOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintStapleOptionDetails {
    type Vtable = IPrintStapleOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintStapleOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd43175bd_9c0b_44e0_84f6_ceebce653800);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintStapleOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintTaskOptionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskOptionChangedEventArgs {
    type Vtable = IPrintTaskOptionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintTaskOptionChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65197d05_a5ee_4307_9407_9acad147679c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintTaskOptionDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskOptionDetails {
    type Vtable = IPrintTaskOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintTaskOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5720af1_a89e_42a6_81af_f8e010b38a68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub CreateItemListOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTextOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OptionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BeginValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginValidation: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBeginValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBeginValidation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintTaskOptionDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskOptionDetails2 {
    type Vtable = IPrintTaskOptionDetails2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintTaskOptionDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53730a09_f968_4692_a177_c074597186db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateToggleOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintTaskOptionDetailsStatic(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintTaskOptionDetailsStatic {
    type Vtable = IPrintTaskOptionDetailsStatic_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintTaskOptionDetailsStatic {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x135da193_0961_4b6e_8766_f13b7fbccd58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetailsStatic_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetFromPrintTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printtaskoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintTextOptionDetails(::windows_core::IUnknown);
impl IPrintTextOptionDetails {
    pub fn MaxCharacters(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxCharacters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrintTextOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintOptionDetails> for IPrintTextOptionDetails {}
impl ::windows_core::RuntimeType for IPrintTextOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IPrintTextOptionDetails {
    type Vtable = IPrintTextOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintTextOptionDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad75e563_5ce4_46bc_9918_ab9fad144c5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTextOptionDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintBindingOptionDetails(::windows_core::IUnknown);
impl PrintBindingOptionDetails {
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintBindingOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintBindingOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintBindingOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintBindingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintBindingOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintBindingOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintBindingOptionDetails {}
unsafe impl ::core::marker::Send for PrintBindingOptionDetails {}
unsafe impl ::core::marker::Sync for PrintBindingOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintBorderingOptionDetails(::windows_core::IUnknown);
impl PrintBorderingOptionDetails {
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintBorderingOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintBorderingOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintBorderingOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintBorderingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintBorderingOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintBorderingOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintBorderingOptionDetails {}
unsafe impl ::core::marker::Send for PrintBorderingOptionDetails {}
unsafe impl ::core::marker::Sync for PrintBorderingOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintCollationOptionDetails(::windows_core::IUnknown);
impl PrintCollationOptionDetails {
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintCollationOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintCollationOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintCollationOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintCollationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintCollationOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintCollationOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintCollationOptionDetails {}
unsafe impl ::core::marker::Send for PrintCollationOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCollationOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintColorModeOptionDetails(::windows_core::IUnknown);
impl PrintColorModeOptionDetails {
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintColorModeOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintColorModeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintColorModeOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintColorModeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintColorModeOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintColorModeOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintColorModeOptionDetails {}
unsafe impl ::core::marker::Send for PrintColorModeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintColorModeOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintCopiesOptionDetails(::windows_core::IUnknown);
impl PrintCopiesOptionDetails {
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinValue(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IPrintNumberOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxValue(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IPrintNumberOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintCopiesOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintCopiesOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintCopiesOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintCopiesOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintCopiesOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintNumberOptionDetails> for PrintCopiesOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintCopiesOptionDetails {}
unsafe impl ::core::marker::Send for PrintCopiesOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCopiesOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintCustomItemDetails(::windows_core::IUnknown);
impl PrintCustomItemDetails {
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetItemDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetItemDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ItemDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintCustomItemDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintCustomItemDetails {
    type Vtable = IPrintCustomItemDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintCustomItemDetails {
    const IID: ::windows_core::GUID = <IPrintCustomItemDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintCustomItemDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails";
}
::windows_core::imp::interface_hierarchy!(PrintCustomItemDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintCustomItemDetails {}
unsafe impl ::core::marker::Sync for PrintCustomItemDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintCustomItemListOptionDetails(::windows_core::IUnknown);
impl PrintCustomItemListOptionDetails {
    pub fn AddItem(&self, itemid: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomItemListOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid), ::core::mem::transmute_copy(displayname)).ok() }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AddItem2<P0>(&self, itemid: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING, description: &::windows_core::HSTRING, icon: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomItemListOptionDetails2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(description), icon.try_into_param()?.abi()).ok() }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintCustomItemListOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintCustomItemListOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintCustomItemListOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintCustomItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintCustomItemListOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintCustomOptionDetails> for PrintCustomItemListOptionDetails {}
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintCustomItemListOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintCustomItemListOptionDetails {}
unsafe impl ::core::marker::Send for PrintCustomItemListOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomItemListOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintCustomTextOptionDetails(::windows_core::IUnknown);
impl PrintCustomTextOptionDetails {
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxCharacters(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomTextOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxCharacters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxCharacters(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomTextOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxCharacters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintCustomTextOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintCustomTextOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintCustomTextOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintCustomTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintCustomTextOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintCustomOptionDetails> for PrintCustomTextOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintCustomTextOptionDetails {}
unsafe impl ::core::marker::Send for PrintCustomTextOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomTextOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintCustomToggleOptionDetails(::windows_core::IUnknown);
impl PrintCustomToggleOptionDetails {
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintCustomToggleOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintCustomToggleOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintCustomToggleOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintCustomToggleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintCustomToggleOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintCustomOptionDetails> for PrintCustomToggleOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintCustomToggleOptionDetails {}
unsafe impl ::core::marker::Send for PrintCustomToggleOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomToggleOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintDuplexOptionDetails(::windows_core::IUnknown);
impl PrintDuplexOptionDetails {
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintDuplexOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintDuplexOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintDuplexOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintDuplexOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintDuplexOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintDuplexOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintDuplexOptionDetails {}
unsafe impl ::core::marker::Send for PrintDuplexOptionDetails {}
unsafe impl ::core::marker::Sync for PrintDuplexOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintHolePunchOptionDetails(::windows_core::IUnknown);
impl PrintHolePunchOptionDetails {
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintHolePunchOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintHolePunchOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintHolePunchOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintHolePunchOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintHolePunchOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintHolePunchOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintHolePunchOptionDetails {}
unsafe impl ::core::marker::Send for PrintHolePunchOptionDetails {}
unsafe impl ::core::marker::Sync for PrintHolePunchOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintMediaSizeOptionDetails(::windows_core::IUnknown);
impl PrintMediaSizeOptionDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintMediaSizeOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintMediaSizeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintMediaSizeOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintMediaSizeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintMediaSizeOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintMediaSizeOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintMediaSizeOptionDetails {}
unsafe impl ::core::marker::Send for PrintMediaSizeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintMediaSizeOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintMediaTypeOptionDetails(::windows_core::IUnknown);
impl PrintMediaTypeOptionDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintMediaTypeOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintMediaTypeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintMediaTypeOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintMediaTypeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintMediaTypeOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintMediaTypeOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintMediaTypeOptionDetails {}
unsafe impl ::core::marker::Send for PrintMediaTypeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintMediaTypeOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintOrientationOptionDetails(::windows_core::IUnknown);
impl PrintOrientationOptionDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintOrientationOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintOrientationOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintOrientationOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintOrientationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintOrientationOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintOrientationOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintOrientationOptionDetails {}
unsafe impl ::core::marker::Send for PrintOrientationOptionDetails {}
unsafe impl ::core::marker::Sync for PrintOrientationOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintPageRangeOptionDetails(::windows_core::IUnknown);
impl PrintPageRangeOptionDetails {
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintPageRangeOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintPageRangeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintPageRangeOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintPageRangeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintPageRangeOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintPageRangeOptionDetails {}
unsafe impl ::core::marker::Send for PrintPageRangeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintPageRangeOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintQualityOptionDetails(::windows_core::IUnknown);
impl PrintQualityOptionDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintQualityOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintQualityOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintQualityOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintQualityOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintQualityOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintQualityOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintQualityOptionDetails {}
unsafe impl ::core::marker::Send for PrintQualityOptionDetails {}
unsafe impl ::core::marker::Sync for PrintQualityOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintStapleOptionDetails(::windows_core::IUnknown);
impl PrintStapleOptionDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows_core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWarningText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WarningText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintStapleOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintStapleOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintStapleOptionDetails {
    const IID: ::windows_core::GUID = <IPrintOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintStapleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintStapleOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPrintItemListOptionDetails> for PrintStapleOptionDetails {}
impl ::windows_core::CanTryInto<IPrintOptionDetails> for PrintStapleOptionDetails {}
unsafe impl ::core::marker::Send for PrintStapleOptionDetails {}
unsafe impl ::core::marker::Sync for PrintStapleOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintTaskOptionChangedEventArgs(::windows_core::IUnknown);
impl PrintTaskOptionChangedEventArgs {
    pub fn OptionId(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintTaskOptionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintTaskOptionChangedEventArgs {
    type Vtable = IPrintTaskOptionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintTaskOptionChangedEventArgs {
    const IID: ::windows_core::GUID = <IPrintTaskOptionChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskOptionChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintTaskOptionChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintTaskOptionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskOptionChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintTaskOptionDetails(::windows_core::IUnknown);
impl PrintTaskOptionDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, IPrintOptionDetails>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Options)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateItemListOption(&self, optionid: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<PrintCustomItemListOptionDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateItemListOption)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(optionid), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateTextOption(&self, optionid: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<PrintCustomTextOptionDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextOption)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(optionid), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn OptionChanged<P0>(&self, eventhandler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionChanged)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionChanged(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOptionChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginValidation<P0>(&self, eventhandler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginValidation)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBeginValidation(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBeginValidation)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CreateToggleOption(&self, optionid: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<PrintCustomToggleOptionDetails> {
        let this = &::windows_core::ComInterface::cast::<IPrintTaskOptionDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateToggleOption)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(optionid), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFromPrintTaskOptions<P0>(printtaskoptions: P0) -> ::windows_core::Result<PrintTaskOptionDetails>
    where
        P0: ::windows_core::IntoParam<super::PrintTaskOptions>,
    {
        Self::IPrintTaskOptionDetailsStatic(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromPrintTaskOptions)(::windows_core::Interface::as_raw(this), printtaskoptions.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows_core::Result<super::PrintPageDescription> {
        let this = &::windows_core::ComInterface::cast::<super::IPrintTaskOptionsCore>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPageDescription)(::windows_core::Interface::as_raw(this), jobpagenumber, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<super::IPrintTaskOptionsCoreUIConfiguration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayedOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IPrintTaskOptionDetailsStatic<R, F: FnOnce(&IPrintTaskOptionDetailsStatic) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PrintTaskOptionDetails, IPrintTaskOptionDetailsStatic> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PrintTaskOptionDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintTaskOptionDetails {
    type Vtable = IPrintTaskOptionDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintTaskOptionDetails {
    const IID: ::windows_core::GUID = <IPrintTaskOptionDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails";
}
::windows_core::imp::interface_hierarchy!(PrintTaskOptionDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IPrintTaskOptionsCore> for PrintTaskOptionDetails {}
impl ::windows_core::CanTryInto<super::IPrintTaskOptionsCoreUIConfiguration> for PrintTaskOptionDetails {}
unsafe impl ::core::marker::Send for PrintTaskOptionDetails {}
unsafe impl ::core::marker::Sync for PrintTaskOptionDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintOptionStates(pub u32);
impl PrintOptionStates {
    pub const None: Self = Self(0u32);
    pub const Enabled: Self = Self(1u32);
    pub const Constrained: Self = Self(2u32);
}
impl ::core::marker::Copy for PrintOptionStates {}
impl ::core::clone::Clone for PrintOptionStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintOptionStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintOptionStates {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintOptionStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintOptionStates").field(&self.0).finish()
    }
}
impl PrintOptionStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PrintOptionStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PrintOptionStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PrintOptionStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PrintOptionStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PrintOptionStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for PrintOptionStates {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.OptionDetails.PrintOptionStates;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintOptionType(pub i32);
impl PrintOptionType {
    pub const Unknown: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Text: Self = Self(2i32);
    pub const ItemList: Self = Self(3i32);
    pub const Toggle: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintOptionType {}
impl ::core::clone::Clone for PrintOptionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintOptionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintOptionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintOptionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintOptionType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintOptionType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.OptionDetails.PrintOptionType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
