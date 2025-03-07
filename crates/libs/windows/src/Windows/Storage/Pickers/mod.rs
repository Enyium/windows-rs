#[cfg(feature = "Storage_Pickers_Provider")]
#[doc = "Required features: `\"Storage_Pickers_Provider\"`"]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileOpenPicker {
    type Vtable = IFileOpenPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ca8278a_12c5_4c5f_8977_94547793c241);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows_core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows_core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows_core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows_core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PickMultipleFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PickMultipleFilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPicker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileOpenPicker2 {
    type Vtable = IFileOpenPicker2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPicker2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ceb6cd2_b446_46f7_b265_90f8e55ad650);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickSingleFileAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickSingleFileAndContinue: usize,
    #[cfg(feature = "deprecated")]
    pub PickMultipleFilesAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickMultipleFilesAndContinue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPicker3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileOpenPicker3 {
    type Vtable = IFileOpenPicker3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPicker3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9a5c5b3_c5dc_5b98_bd80_a8d0ca0584d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPicker3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPickerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileOpenPickerStatics {
    type Vtable = IFileOpenPickerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPickerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6821573b_2f02_4833_96d4_abbfad72b67b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ResumePickSingleFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ResumePickSingleFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPickerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileOpenPickerStatics2 {
    type Vtable = IFileOpenPickerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPickerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8917415_eddd_5c98_b6f3_366fdfcad392);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPickerWithOperationId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileOpenPickerWithOperationId {
    type Vtable = IFileOpenPickerWithOperationId_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPickerWithOperationId {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f57b569_2522_4ca5_aa73_a15509f1fcbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerWithOperationId_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PickSingleFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pickeroperationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileSavePicker {
    type Vtable = IFileSavePicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileSavePicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3286ffcb_617f_4cc5_af6a_b3fdf29ad145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows_core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows_core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeChoices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeChoices: usize,
    pub DefaultFileExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDefaultFileExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SuggestedSaveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSuggestedSaveFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SuggestedFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSuggestedFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PickSaveFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSaveFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePicker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileSavePicker2 {
    type Vtable = IFileSavePicker2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileSavePicker2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ec313a2_d24b_449a_8197_e89104fd42cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickSaveFileAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickSaveFileAndContinue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePicker3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileSavePicker3 {
    type Vtable = IFileSavePicker3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileSavePicker3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x698aec69_ba3c_4e51_bd90_4abcbbf4cfaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePicker4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileSavePicker4 {
    type Vtable = IFileSavePicker4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileSavePicker4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7d83a5a_ddfa_5de0_8b70_c842c21988ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePicker4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePickerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileSavePickerStatics {
    type Vtable = IFileSavePickerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileSavePickerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28e3cf9e_961c_5e2c_aed7_e64737f4ce37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFolderPicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFolderPicker {
    type Vtable = IFolderPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFolderPicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x084f7799_f3fb_400a_99b1_7b4a772fd60d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerViewMode) -> ::windows_core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerViewMode) -> ::windows_core::HRESULT,
    pub SettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSettingsIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PickerLocationId) -> ::windows_core::HRESULT,
    pub SetSuggestedStartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PickerLocationId) -> ::windows_core::HRESULT,
    pub CommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCommitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFolderPicker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFolderPicker2 {
    type Vtable = IFolderPicker2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFolderPicker2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8eb3ba97_dc85_4616_be94_9660881f2f5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
    #[cfg(feature = "deprecated")]
    pub PickFolderAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PickFolderAndContinue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFolderPicker3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFolderPicker3 {
    type Vtable = IFolderPicker3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFolderPicker3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x673b1e29_d326_53c0_bd24_a25c714cee36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPicker3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFolderPickerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFolderPickerStatics {
    type Vtable = IFolderPickerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFolderPickerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9be34740_7ca1_5942_a3c8_46f2551ecff3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub CreateForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateForUser: usize,
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileExtensionVector(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl FileExtensionVector {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &::windows_core::HSTRING, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::windows_core::HSTRING]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::windows_core::HSTRING]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for FileExtensionVector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for FileExtensionVector {
    type Vtable = super::super::Foundation::Collections::IVector_Vtbl<::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for FileExtensionVector {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVector<::windows_core::HSTRING> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for FileExtensionVector {
    const NAME: &'static str = "Windows.Storage.Pickers.FileExtensionVector";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FileExtensionVector {
    type Item = ::windows_core::HSTRING;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FileExtensionVector {
    type Item = ::windows_core::HSTRING;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(FileExtensionVector, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>> for FileExtensionVector {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> for FileExtensionVector {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for FileExtensionVector {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for FileExtensionVector {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileOpenPicker(::windows_core::IUnknown);
impl FileOpenPicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileOpenPicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ViewMode(&self) -> ::windows_core::Result<PickerViewMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetViewMode(&self, value: PickerViewMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SettingsIdentifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SettingsIdentifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSettingsIdentifier)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> ::windows_core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedStartLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuggestedStartLocation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommitButtonText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommitButtonText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeFilter(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileTypeFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleFileAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleFileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PickMultipleFilesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickMultipleFilesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IFileOpenPicker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PickSingleFileAndContinue(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFileOpenPicker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PickSingleFileAndContinue)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PickMultipleFilesAndContinue(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFileOpenPicker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PickMultipleFilesAndContinue)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IFileOpenPicker3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ResumePickSingleFileAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        Self::IFileOpenPickerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResumePickSingleFileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> ::windows_core::Result<FileOpenPicker>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IFileOpenPickerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleFileAsync2(&self, pickeroperationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::ComInterface::cast::<IFileOpenPickerWithOperationId>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleFileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(pickeroperationid), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IFileOpenPickerStatics<R, F: FnOnce(&IFileOpenPickerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileOpenPicker, IFileOpenPickerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFileOpenPickerStatics2<R, F: FnOnce(&IFileOpenPickerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileOpenPicker, IFileOpenPickerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FileOpenPicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FileOpenPicker {
    type Vtable = IFileOpenPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileOpenPicker {
    const IID: ::windows_core::GUID = <IFileOpenPicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileOpenPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FileOpenPicker";
}
::windows_core::imp::interface_hierarchy!(FileOpenPicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FileOpenPicker {}
unsafe impl ::core::marker::Sync for FileOpenPicker {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FilePickerFileTypesOrderedMap(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl FilePickerFileTypesOrderedMap {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: &::windows_core::HSTRING, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for FilePickerFileTypesOrderedMap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for FilePickerFileTypesOrderedMap {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for FilePickerFileTypesOrderedMap {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IMap<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for FilePickerFileTypesOrderedMap {
    const NAME: &'static str = "Windows.Storage.Pickers.FilePickerFileTypesOrderedMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FilePickerFileTypesOrderedMap {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FilePickerFileTypesOrderedMap {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(FilePickerFileTypesOrderedMap, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>>> for FilePickerFileTypesOrderedMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>> for FilePickerFileTypesOrderedMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for FilePickerFileTypesOrderedMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for FilePickerFileTypesOrderedMap {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FilePickerSelectedFilesArray(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl FilePickerSelectedFilesArray {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::StorageFile>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::StorageFile>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::StorageFile>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for FilePickerSelectedFilesArray {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for FilePickerSelectedFilesArray {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::StorageFile>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for FilePickerSelectedFilesArray {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVectorView<super::StorageFile> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for FilePickerSelectedFilesArray {
    const NAME: &'static str = "Windows.Storage.Pickers.FilePickerSelectedFilesArray";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for FilePickerSelectedFilesArray {
    type Item = super::StorageFile;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &FilePickerSelectedFilesArray {
    type Item = super::StorageFile;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(FilePickerSelectedFilesArray, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<super::StorageFile>> for FilePickerSelectedFilesArray {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IVectorView<super::StorageFile>> for FilePickerSelectedFilesArray {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for FilePickerSelectedFilesArray {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for FilePickerSelectedFilesArray {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileSavePicker(::windows_core::IUnknown);
impl FileSavePicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileSavePicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SettingsIdentifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SettingsIdentifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSettingsIdentifier)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> ::windows_core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedStartLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuggestedStartLocation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommitButtonText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommitButtonText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeChoices(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileTypeChoices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultFileExtension(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultFileExtension)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDefaultFileExtension(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultFileExtension)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedSaveFile(&self) -> ::windows_core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedSaveFile)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSuggestedSaveFile<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::StorageFile>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuggestedSaveFile)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SuggestedFileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedFileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSuggestedFileName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuggestedFileName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PickSaveFileAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSaveFileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IFileSavePicker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PickSaveFileAndContinue(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFileSavePicker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PickSaveFileAndContinue)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IFileSavePicker3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnterpriseId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFileSavePicker3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEnterpriseId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IFileSavePicker4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> ::windows_core::Result<FileSavePicker>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IFileSavePickerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileSavePickerStatics<R, F: FnOnce(&IFileSavePickerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileSavePicker, IFileSavePickerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FileSavePicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FileSavePicker {
    type Vtable = IFileSavePicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileSavePicker {
    const IID: ::windows_core::GUID = <IFileSavePicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileSavePicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FileSavePicker";
}
::windows_core::imp::interface_hierarchy!(FileSavePicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FileSavePicker {}
unsafe impl ::core::marker::Sync for FileSavePicker {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FolderPicker(::windows_core::IUnknown);
impl FolderPicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FolderPicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ViewMode(&self) -> ::windows_core::Result<PickerViewMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetViewMode(&self, value: PickerViewMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SettingsIdentifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SettingsIdentifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSettingsIdentifier(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSettingsIdentifier)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SuggestedStartLocation(&self) -> ::windows_core::Result<PickerLocationId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedStartLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSuggestedStartLocation(&self, value: PickerLocationId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSuggestedStartLocation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CommitButtonText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommitButtonText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCommitButtonText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommitButtonText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeFilter(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileTypeFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleFolderAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleFolderAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IFolderPicker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PickFolderAndContinue(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFolderPicker2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PickFolderAndContinue)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IFolderPicker3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn CreateForUser<P0>(user: P0) -> ::windows_core::Result<FolderPicker>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IFolderPickerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFolderPickerStatics<R, F: FnOnce(&IFolderPickerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FolderPicker, IFolderPickerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FolderPicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FolderPicker {
    type Vtable = IFolderPicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FolderPicker {
    const IID: ::windows_core::GUID = <IFolderPicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FolderPicker {
    const NAME: &'static str = "Windows.Storage.Pickers.FolderPicker";
}
::windows_core::imp::interface_hierarchy!(FolderPicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FolderPicker {}
unsafe impl ::core::marker::Sync for FolderPicker {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PickerLocationId(pub i32);
impl PickerLocationId {
    pub const DocumentsLibrary: Self = Self(0i32);
    pub const ComputerFolder: Self = Self(1i32);
    pub const Desktop: Self = Self(2i32);
    pub const Downloads: Self = Self(3i32);
    pub const HomeGroup: Self = Self(4i32);
    pub const MusicLibrary: Self = Self(5i32);
    pub const PicturesLibrary: Self = Self(6i32);
    pub const VideosLibrary: Self = Self(7i32);
    pub const Objects3D: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
}
impl ::core::marker::Copy for PickerLocationId {}
impl ::core::clone::Clone for PickerLocationId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PickerLocationId {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PickerLocationId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PickerLocationId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerLocationId").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PickerLocationId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.PickerLocationId;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PickerViewMode(pub i32);
impl PickerViewMode {
    pub const List: Self = Self(0i32);
    pub const Thumbnail: Self = Self(1i32);
}
impl ::core::marker::Copy for PickerViewMode {}
impl ::core::clone::Clone for PickerViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PickerViewMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PickerViewMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PickerViewMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerViewMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PickerViewMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Pickers.PickerViewMode;i4)");
}
