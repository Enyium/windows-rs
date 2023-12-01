#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationInterpolator(::windows_core::IUnknown);
impl IUIAnimationInterpolator {
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialValueAndVelocity(initialvalue, initialvelocity)).ok()
    }
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetDuration(duration)).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetDuration(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFinalValue(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFinalValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn InterpolateValue(&self, offset: f64) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.InterpolateValue(offset, &mut result__)).from_abi(result__)
    }
    pub unsafe fn InterpolateVelocity(&self, offset: f64) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.InterpolateVelocity(offset, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetDependencies(initialvaluedependencies, initialvelocitydependencies, durationdependencies)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationInterpolator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationInterpolator {
    type Vtable = IUIAnimationInterpolator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationInterpolator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7815cbba_ddf7_478c_a46c_7b6c738b7978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows_core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64) -> ::windows_core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64) -> ::windows_core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationInterpolator2(::windows_core::IUnknown);
impl IUIAnimationInterpolator2 {
    pub unsafe fn GetDimension(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetDimension(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialValueAndVelocity(initialvalue, initialvelocity, cdimension)).ok()
    }
    pub unsafe fn SetDuration(&self, duration: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetDuration(duration)).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetDuration(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFinalValue(&self, value: &mut [f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFinalValue(::core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap())).ok()
    }
    pub unsafe fn InterpolateValue(&self, offset: f64, value: &mut [f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.InterpolateValue(offset, ::core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap())).ok()
    }
    pub unsafe fn InterpolateVelocity(&self, offset: f64, velocity: &mut [f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.InterpolateVelocity(offset, ::core::mem::transmute(velocity.as_ptr()), velocity.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetPrimitiveInterpolation<P0>(&self, interpolation: P0, cdimension: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPrimitiveInterpolation>,
    {
        ::windows_core::vcall!(self.GetPrimitiveInterpolation(interpolation.into_param().abi(), cdimension)).ok()
    }
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetDependencies(initialvaluedependencies, initialvelocitydependencies, durationdependencies)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationInterpolator2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationInterpolator2 {
    type Vtable = IUIAnimationInterpolator2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationInterpolator2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea76aff8_ea22_4a23_a0ef_a6a966703518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationInterpolator2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows_core::HRESULT,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64) -> ::windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub GetPrimitiveInterpolation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolation: *mut ::core::ffi::c_void, cdimension: u32) -> ::windows_core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationLoopIterationChangeHandler2(::windows_core::IUnknown);
impl IUIAnimationLoopIterationChangeHandler2 {
    pub unsafe fn OnLoopIterationChanged<P0>(&self, storyboard: P0, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard2>,
    {
        ::windows_core::vcall!(self.OnLoopIterationChanged(storyboard.into_param().abi(), id, newiterationcount, olditerationcount)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationLoopIterationChangeHandler2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationLoopIterationChangeHandler2 {
    type Vtable = IUIAnimationLoopIterationChangeHandler2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationLoopIterationChangeHandler2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d3b15a4_4762_47ab_a030_b23221df3ae0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnLoopIterationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, id: usize, newiterationcount: u32, olditerationcount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationManager(::windows_core::IUnknown);
impl IUIAnimationManager {
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows_core::Result<IUIAnimationVariable> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAnimationVariable(initialvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn ScheduleTransition<P0, P1>(&self, variable: P0, transition: P1, timenow: f64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition>,
    {
        ::windows_core::vcall!(self.ScheduleTransition(variable.into_param().abi(), transition.into_param().abi(), timenow)).ok()
    }
    pub unsafe fn CreateStoryboard(&self) -> ::windows_core::Result<IUIAnimationStoryboard> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateStoryboard(&mut result__)).from_abi(result__)
    }
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.FinishAllStoryboards(completiondeadline)).ok()
    }
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AbandonAllStoryboards()).ok()
    }
    pub unsafe fn Update(&self, timenow: f64, updateresult: ::core::option::Option<*mut UI_ANIMATION_UPDATE_RESULT>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Update(timenow, ::core::mem::transmute(updateresult.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetVariableFromTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<IUIAnimationVariable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetVariableFromTag(object.into_param().abi(), id, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetStoryboardFromTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<IUIAnimationStoryboard>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStoryboardFromTag(object.into_param().abi(), id, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStatus(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAnimationMode(mode)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Pause()).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Resume()).ok()
    }
    pub unsafe fn SetManagerEventHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationManagerEventHandler>,
    {
        ::windows_core::vcall!(self.SetManagerEventHandler(handler.into_param().abi())).ok()
    }
    pub unsafe fn SetCancelPriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison>,
    {
        ::windows_core::vcall!(self.SetCancelPriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetTrimPriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison>,
    {
        ::windows_core::vcall!(self.SetTrimPriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetCompressPriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison>,
    {
        ::windows_core::vcall!(self.SetCompressPriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetConcludePriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison>,
    {
        ::windows_core::vcall!(self.SetConcludePriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetDefaultLongestAcceptableDelay(delay)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Shutdown()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationManager {
    type Vtable = IUIAnimationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9169896c_ac8d_4e7d_94e5_67fa4dc2f2e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows_core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationManager2(::windows_core::IUnknown);
impl IUIAnimationManager2 {
    pub unsafe fn CreateAnimationVectorVariable(&self, initialvalue: &[f64]) -> ::windows_core::Result<IUIAnimationVariable2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAnimationVectorVariable(::core::mem::transmute(initialvalue.as_ptr()), initialvalue.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> ::windows_core::Result<IUIAnimationVariable2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAnimationVariable(initialvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn ScheduleTransition<P0, P1>(&self, variable: P0, transition: P1, timenow: f64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable2>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition2>,
    {
        ::windows_core::vcall!(self.ScheduleTransition(variable.into_param().abi(), transition.into_param().abi(), timenow)).ok()
    }
    pub unsafe fn CreateStoryboard(&self) -> ::windows_core::Result<IUIAnimationStoryboard2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateStoryboard(&mut result__)).from_abi(result__)
    }
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.FinishAllStoryboards(completiondeadline)).ok()
    }
    pub unsafe fn AbandonAllStoryboards(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AbandonAllStoryboards()).ok()
    }
    pub unsafe fn Update(&self, timenow: f64, updateresult: ::core::option::Option<*mut UI_ANIMATION_UPDATE_RESULT>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Update(timenow, ::core::mem::transmute(updateresult.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetVariableFromTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<IUIAnimationVariable2>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetVariableFromTag(object.into_param().abi(), id, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetStoryboardFromTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<IUIAnimationStoryboard2>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStoryboardFromTag(object.into_param().abi(), id, &mut result__)).from_abi(result__)
    }
    pub unsafe fn EstimateNextEventTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.EstimateNextEventTime(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStatus(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAnimationMode(mode)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Pause()).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Resume()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManagerEventHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationManagerEventHandler2>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetManagerEventHandler(handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi())).ok()
    }
    pub unsafe fn SetCancelPriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison2>,
    {
        ::windows_core::vcall!(self.SetCancelPriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetTrimPriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison2>,
    {
        ::windows_core::vcall!(self.SetTrimPriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetCompressPriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison2>,
    {
        ::windows_core::vcall!(self.SetCompressPriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetConcludePriorityComparison<P0>(&self, comparison: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationPriorityComparison2>,
    {
        ::windows_core::vcall!(self.SetConcludePriorityComparison(comparison.into_param().abi())).ok()
    }
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetDefaultLongestAcceptableDelay(delay)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Shutdown()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationManager2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationManager2 {
    type Vtable = IUIAnimationManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8b6f7d4_4109_4d3f_acee_879926968cb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManager2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateAnimationVectorVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAnimationVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalue: f64, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, timenow: f64) -> ::windows_core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, variable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EstimateNextEventTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_MODE) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManagerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManagerEventHandler: usize,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, comparison: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationManagerEventHandler(::windows_core::IUnknown);
impl IUIAnimationManagerEventHandler {
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.OnManagerStatusChanged(newstatus, previousstatus)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationManagerEventHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationManagerEventHandler {
    type Vtable = IUIAnimationManagerEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationManagerEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x783321ed_78a3_4366_b574_6af607a64788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationManagerEventHandler2(::windows_core::IUnknown);
impl IUIAnimationManagerEventHandler2 {
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.OnManagerStatusChanged(newstatus, previousstatus)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationManagerEventHandler2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationManagerEventHandler2 {
    type Vtable = IUIAnimationManagerEventHandler2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationManagerEventHandler2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6e022ba_bff3_42ec_9033_e073f33e83c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationManagerEventHandler2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationPrimitiveInterpolation(::windows_core::IUnknown);
impl IUIAnimationPrimitiveInterpolation {
    pub unsafe fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AddCubic(dimension, beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient)).ok()
    }
    pub unsafe fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AddSinusoidal(dimension, beginoffset, bias, amplitude, frequency, phase)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationPrimitiveInterpolation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationPrimitiveInterpolation {
    type Vtable = IUIAnimationPrimitiveInterpolation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationPrimitiveInterpolation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbab20d63_4361_45da_a24f_ab8508846b5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPrimitiveInterpolation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddCubic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationPriorityComparison(::windows_core::IUnknown);
impl IUIAnimationPriorityComparison {
    pub unsafe fn HasPriority<P0, P1>(&self, scheduledstoryboard: P0, newstoryboard: P1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard>,
        P1: ::windows_core::IntoParam<IUIAnimationStoryboard>,
    {
        ::windows_core::vcall!(self.HasPriority(scheduledstoryboard.into_param().abi(), newstoryboard.into_param().abi(), priorityeffect)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationPriorityComparison, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationPriorityComparison {
    type Vtable = IUIAnimationPriorityComparison_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationPriorityComparison {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83fa9b74_5f86_4618_bc6a_a2fac19b3f44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub HasPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationPriorityComparison2(::windows_core::IUnknown);
impl IUIAnimationPriorityComparison2 {
    pub unsafe fn HasPriority<P0, P1>(&self, scheduledstoryboard: P0, newstoryboard: P1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard2>,
        P1: ::windows_core::IntoParam<IUIAnimationStoryboard2>,
    {
        ::windows_core::vcall!(self.HasPriority(scheduledstoryboard.into_param().abi(), newstoryboard.into_param().abi(), priorityeffect)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationPriorityComparison2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationPriorityComparison2 {
    type Vtable = IUIAnimationPriorityComparison2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationPriorityComparison2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b6d7a37_4621_467c_8b05_70131de62ddb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationPriorityComparison2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub HasPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledstoryboard: *mut ::core::ffi::c_void, newstoryboard: *mut ::core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationStoryboard(::windows_core::IUnknown);
impl IUIAnimationStoryboard {
    pub unsafe fn AddTransition<P0, P1>(&self, variable: P0, transition: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition>,
    {
        ::windows_core::vcall!(self.AddTransition(variable.into_param().abi(), transition.into_param().abi())).ok()
    }
    pub unsafe fn AddKeyframeAtOffset<P0>(&self, existingkeyframe: P0, offset: f64) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AddKeyframeAtOffset(existingkeyframe.into_param().abi(), offset, &mut result__)).from_abi(result__)
    }
    pub unsafe fn AddKeyframeAfterTransition<P0>(&self, transition: P0) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::windows_core::IntoParam<IUIAnimationTransition>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AddKeyframeAfterTransition(transition.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn AddTransitionAtKeyframe<P0, P1, P2>(&self, variable: P0, transition: P1, startkeyframe: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition>,
        P2: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
    {
        ::windows_core::vcall!(self.AddTransitionAtKeyframe(variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi())).ok()
    }
    pub unsafe fn AddTransitionBetweenKeyframes<P0, P1, P2, P3>(&self, variable: P0, transition: P1, startkeyframe: P2, endkeyframe: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition>,
        P2: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
        P3: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
    {
        ::windows_core::vcall!(self.AddTransitionBetweenKeyframes(variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi(), endkeyframe.into_param().abi())).ok()
    }
    pub unsafe fn RepeatBetweenKeyframes<P0, P1>(&self, startkeyframe: P0, endkeyframe: P1, repetitioncount: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
        P1: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
    {
        ::windows_core::vcall!(self.RepeatBetweenKeyframes(startkeyframe.into_param().abi(), endkeyframe.into_param().abi(), repetitioncount)).ok()
    }
    pub unsafe fn HoldVariable<P0>(&self, variable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable>,
    {
        ::windows_core::vcall!(self.HoldVariable(variable.into_param().abi())).ok()
    }
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLongestAcceptableDelay(delay)).ok()
    }
    pub unsafe fn Schedule(&self, timenow: f64, schedulingresult: ::core::option::Option<*mut UI_ANIMATION_SCHEDULING_RESULT>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Schedule(timenow, ::core::mem::transmute(schedulingresult.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Conclude(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Conclude()).ok()
    }
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Finish(completiondeadline)).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Abandon()).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.SetTag(object.into_param().abi(), id)).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows_core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTag(::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStatus(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetElapsedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetElapsedTime(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetStoryboardEventHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboardEventHandler>,
    {
        ::windows_core::vcall!(self.SetStoryboardEventHandler(handler.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationStoryboard, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationStoryboard {
    type Vtable = IUIAnimationStoryboard_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationStoryboard {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8ff128f_9bf9_4af1_9e67_e5e410defb84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> ::windows_core::HRESULT,
    pub HoldVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows_core::HRESULT,
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationStoryboard2(::windows_core::IUnknown);
impl IUIAnimationStoryboard2 {
    pub unsafe fn AddTransition<P0, P1>(&self, variable: P0, transition: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable2>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition2>,
    {
        ::windows_core::vcall!(self.AddTransition(variable.into_param().abi(), transition.into_param().abi())).ok()
    }
    pub unsafe fn AddKeyframeAtOffset<P0>(&self, existingkeyframe: P0, offset: f64) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AddKeyframeAtOffset(existingkeyframe.into_param().abi(), offset, &mut result__)).from_abi(result__)
    }
    pub unsafe fn AddKeyframeAfterTransition<P0>(&self, transition: P0) -> ::windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: ::windows_core::IntoParam<IUIAnimationTransition2>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AddKeyframeAfterTransition(transition.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn AddTransitionAtKeyframe<P0, P1, P2>(&self, variable: P0, transition: P1, startkeyframe: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable2>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition2>,
        P2: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
    {
        ::windows_core::vcall!(self.AddTransitionAtKeyframe(variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi())).ok()
    }
    pub unsafe fn AddTransitionBetweenKeyframes<P0, P1, P2, P3>(&self, variable: P0, transition: P1, startkeyframe: P2, endkeyframe: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable2>,
        P1: ::windows_core::IntoParam<IUIAnimationTransition2>,
        P2: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
        P3: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
    {
        ::windows_core::vcall!(self.AddTransitionBetweenKeyframes(variable.into_param().abi(), transition.into_param().abi(), startkeyframe.into_param().abi(), endkeyframe.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RepeatBetweenKeyframes<P0, P1, P2, P3>(&self, startkeyframe: P0, endkeyframe: P1, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: P2, id: usize, fregisterfornextanimationevent: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
        P1: ::windows_core::IntoParam<UI_ANIMATION_KEYFRAME>,
        P2: ::windows_core::IntoParam<IUIAnimationLoopIterationChangeHandler2>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.RepeatBetweenKeyframes(startkeyframe.into_param().abi(), endkeyframe.into_param().abi(), crepetition, repeatmode, piterationchangehandler.into_param().abi(), id, fregisterfornextanimationevent.into_param().abi())).ok()
    }
    pub unsafe fn HoldVariable<P0>(&self, variable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable2>,
    {
        ::windows_core::vcall!(self.HoldVariable(variable.into_param().abi())).ok()
    }
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLongestAcceptableDelay(delay)).ok()
    }
    pub unsafe fn SetSkipDuration(&self, secondsduration: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSkipDuration(secondsduration)).ok()
    }
    pub unsafe fn Schedule(&self, timenow: f64, schedulingresult: ::core::option::Option<*mut UI_ANIMATION_SCHEDULING_RESULT>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Schedule(timenow, ::core::mem::transmute(schedulingresult.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Conclude(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Conclude()).ok()
    }
    pub unsafe fn Finish(&self, completiondeadline: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Finish(completiondeadline)).ok()
    }
    pub unsafe fn Abandon(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Abandon()).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.SetTag(object.into_param().abi(), id)).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows_core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTag(::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStatus(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetElapsedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetElapsedTime(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStoryboardEventHandler<P0, P1, P2>(&self, handler: P0, fregisterstatuschangefornextanimationevent: P1, fregisterupdatefornextanimationevent: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboardEventHandler2>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetStoryboardEventHandler(handler.into_param().abi(), fregisterstatuschangefornextanimationevent.into_param().abi(), fregisterupdatefornextanimationevent.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationStoryboard2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationStoryboard2 {
    type Vtable = IUIAnimationStoryboard2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationStoryboard2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae289cd2_12d4_4945_9419_9e41be034df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboard2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, transition: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: *mut ::core::ffi::c_void, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RepeatBetweenKeyframes: usize,
    pub HoldVariable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64) -> ::windows_core::HRESULT,
    pub SetSkipDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, secondsduration: f64) -> ::windows_core::HRESULT,
    pub Schedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> ::windows_core::HRESULT,
    pub Conclude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Finish: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiondeadline: f64) -> ::windows_core::HRESULT,
    pub Abandon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elapsedtime: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStoryboardEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStoryboardEventHandler: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationStoryboardEventHandler(::windows_core::IUnknown);
impl IUIAnimationStoryboardEventHandler {
    pub unsafe fn OnStoryboardStatusChanged<P0>(&self, storyboard: P0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard>,
    {
        ::windows_core::vcall!(self.OnStoryboardStatusChanged(storyboard.into_param().abi(), newstatus, previousstatus)).ok()
    }
    pub unsafe fn OnStoryboardUpdated<P0>(&self, storyboard: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard>,
    {
        ::windows_core::vcall!(self.OnStoryboardUpdated(storyboard.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationStoryboardEventHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationStoryboardEventHandler {
    type Vtable = IUIAnimationStoryboardEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationStoryboardEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d5c9008_ec7c_4364_9f8a_9af3c58cbae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationStoryboardEventHandler2(::windows_core::IUnknown);
impl IUIAnimationStoryboardEventHandler2 {
    pub unsafe fn OnStoryboardStatusChanged<P0>(&self, storyboard: P0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard2>,
    {
        ::windows_core::vcall!(self.OnStoryboardStatusChanged(storyboard.into_param().abi(), newstatus, previousstatus)).ok()
    }
    pub unsafe fn OnStoryboardUpdated<P0>(&self, storyboard: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard2>,
    {
        ::windows_core::vcall!(self.OnStoryboardUpdated(storyboard.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationStoryboardEventHandler2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationStoryboardEventHandler2 {
    type Vtable = IUIAnimationStoryboardEventHandler2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationStoryboardEventHandler2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbac5f55a_ba7c_414c_b599_fbf850f553c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationStoryboardEventHandler2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> ::windows_core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTimer(::windows_core::IUnknown);
impl IUIAnimationTimer {
    pub unsafe fn SetTimerUpdateHandler<P0>(&self, updatehandler: P0, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationTimerUpdateHandler>,
    {
        ::windows_core::vcall!(self.SetTimerUpdateHandler(updatehandler.into_param().abi(), idlebehavior)).ok()
    }
    pub unsafe fn SetTimerEventHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationTimerEventHandler>,
    {
        ::windows_core::vcall!(self.SetTimerEventHandler(handler.into_param().abi())).ok()
    }
    pub unsafe fn Enable(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Enable()).ok()
    }
    pub unsafe fn Disable(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Disable()).ok()
    }
    pub unsafe fn IsEnabled(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.IsEnabled()).ok()
    }
    pub unsafe fn GetTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetTime(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetFrameRateThreshold(&self, framespersecond: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFrameRateThreshold(framespersecond)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTimer, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTimer {
    type Vtable = IUIAnimationTimer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTimer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b0efad1_a053_41d6_9085_33a689144665);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetTimerUpdateHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatehandler: *mut ::core::ffi::c_void, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> ::windows_core::HRESULT,
    pub SetTimerEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut f64) -> ::windows_core::HRESULT,
    pub SetFrameRateThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTimerClientEventHandler(::windows_core::IUnknown);
impl IUIAnimationTimerClientEventHandler {
    pub unsafe fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.OnTimerClientStatusChanged(newstatus, previousstatus)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTimerClientEventHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTimerClientEventHandler {
    type Vtable = IUIAnimationTimerClientEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTimerClientEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbedb4db6_94fa_4bfb_a47f_ef2d9e408c25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerClientEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnTimerClientStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTimerEventHandler(::windows_core::IUnknown);
impl IUIAnimationTimerEventHandler {
    pub unsafe fn OnPreUpdate(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.OnPreUpdate()).ok()
    }
    pub unsafe fn OnPostUpdate(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.OnPostUpdate()).ok()
    }
    pub unsafe fn OnRenderingTooSlow(&self, framespersecond: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.OnRenderingTooSlow(framespersecond)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTimerEventHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTimerEventHandler {
    type Vtable = IUIAnimationTimerEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTimerEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x274a7dea_d771_4095_abbd_8df7abd23ce3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnPreUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnPostUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnRenderingTooSlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framespersecond: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTimerUpdateHandler(::windows_core::IUnknown);
impl IUIAnimationTimerUpdateHandler {
    pub unsafe fn OnUpdate(&self, timenow: f64) -> ::windows_core::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.OnUpdate(timenow, &mut result__)).from_abi(result__)
    }
    pub unsafe fn SetTimerClientEventHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationTimerClientEventHandler>,
    {
        ::windows_core::vcall!(self.SetTimerClientEventHandler(handler.into_param().abi())).ok()
    }
    pub unsafe fn ClearTimerClientEventHandler(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.ClearTimerClientEventHandler()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTimerUpdateHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTimerUpdateHandler {
    type Vtable = IUIAnimationTimerUpdateHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTimerUpdateHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x195509b7_5d5e_4e3e_b278_ee3759b367ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTimerUpdateHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> ::windows_core::HRESULT,
    pub SetTimerClientEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ClearTimerClientEventHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTransition(::windows_core::IUnknown);
impl IUIAnimationTransition {
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialValue(value)).ok()
    }
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialVelocity(velocity)).ok()
    }
    pub unsafe fn IsDurationKnown(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.IsDurationKnown()).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetDuration(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTransition, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTransition {
    type Vtable = IUIAnimationTransition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTransition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc6ce252_f731_41cf_b610_614b6ca049ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows_core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTransition2(::windows_core::IUnknown);
impl IUIAnimationTransition2 {
    pub unsafe fn GetDimension(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetDimension(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetInitialValue(&self, value: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialValue(value)).ok()
    }
    pub unsafe fn SetInitialVectorValue(&self, value: &[f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialVectorValue(::core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialVelocity(velocity)).ok()
    }
    pub unsafe fn SetInitialVectorVelocity(&self, velocity: &[f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialVectorVelocity(::core::mem::transmute(velocity.as_ptr()), velocity.len().try_into().unwrap())).ok()
    }
    pub unsafe fn IsDurationKnown(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.IsDurationKnown()).ok()
    }
    pub unsafe fn GetDuration(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetDuration(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTransition2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTransition2 {
    type Vtable = IUIAnimationTransition2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTransition2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62ff9123_a85a_4e9b_a218_435a93e268fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransition2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows_core::HRESULT,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SetInitialVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *const f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f64) -> ::windows_core::HRESULT,
    pub SetInitialVectorVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *const f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTransitionFactory(::windows_core::IUnknown);
impl IUIAnimationTransitionFactory {
    pub unsafe fn CreateTransition<P0>(&self, interpolator: P0) -> ::windows_core::Result<IUIAnimationTransition>
    where
        P0: ::windows_core::IntoParam<IUIAnimationInterpolator>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTransition(interpolator.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTransitionFactory, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTransitionFactory {
    type Vtable = IUIAnimationTransitionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTransitionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcd91e03_3e3b_45ad_bbb1_6dfc8153743d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTransitionFactory2(::windows_core::IUnknown);
impl IUIAnimationTransitionFactory2 {
    pub unsafe fn CreateTransition<P0>(&self, interpolator: P0) -> ::windows_core::Result<IUIAnimationTransition2>
    where
        P0: ::windows_core::IntoParam<IUIAnimationInterpolator2>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTransition(interpolator.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTransitionFactory2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTransitionFactory2 {
    type Vtable = IUIAnimationTransitionFactory2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTransitionFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x937d4916_c1a6_42d5_88d8_30344d6efe31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionFactory2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolator: *mut ::core::ffi::c_void, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTransitionLibrary(::windows_core::IUnknown);
impl IUIAnimationTransitionLibrary {
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateInstantaneousTransition(finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateConstantTransition(duration, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDiscreteTransition(delay, finalvalue, hold, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateLinearTransition(duration, finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateLinearTransitionFromSpeed(speed, finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSinusoidalTransitionFromVelocity(duration, period, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSinusoidalTransitionFromRange(duration, minimumvalue, maximumvalue, period, slope, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAccelerateDecelerateTransition(duration, finalvalue, accelerationratio, decelerationratio, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateReversalTransition(duration, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCubicTransition(duration, finalvalue, finalvelocity, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSmoothStopTransition(maximumduration, finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows_core::Result<IUIAnimationTransition> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateParabolicTransitionFromAcceleration(finalvalue, finalvelocity, acceleration, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTransitionLibrary, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTransitionLibrary {
    type Vtable = IUIAnimationTransitionLibrary_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTransitionLibrary {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca5a14b1_d24f_48b8_8fe4_c78169ba954e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationTransitionLibrary2(::windows_core::IUnknown);
impl IUIAnimationTransitionLibrary2 {
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateInstantaneousTransition(finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateInstantaneousVectorTransition(&self, finalvalue: &[f64]) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateInstantaneousVectorTransition(::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateConstantTransition(duration, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDiscreteTransition(delay, finalvalue, hold, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: &[f64], hold: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDiscreteVectorTransition(delay, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), hold, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateLinearTransition(duration, finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: &[f64]) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateLinearVectorTransition(duration, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateLinearTransitionFromSpeed(speed, finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: &[f64]) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateLinearVectorTransitionFromSpeed(speed, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSinusoidalTransitionFromVelocity(duration, period, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSinusoidalTransitionFromRange(duration, minimumvalue, maximumvalue, period, slope, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAccelerateDecelerateTransition(duration, finalvalue, accelerationratio, decelerationratio, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateReversalTransition(duration, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCubicTransition(duration, finalvalue, finalvelocity, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCubicVectorTransition(duration, finalvalue, finalvelocity, cdimension, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSmoothStopTransition(maximumduration, finalvalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateParabolicTransitionFromAcceleration(finalvalue, finalvelocity, acceleration, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCubicBezierLinearTransition(duration, finalvalue, x1, y1, x2, y2, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: &[f64], x1: f64, y1: f64, x2: f64, y2: f64) -> ::windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCubicBezierLinearVectorTransition(duration, ::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), x1, y1, x2, y2, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationTransitionLibrary2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationTransitionLibrary2 {
    type Vtable = IUIAnimationTransitionLibrary2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationTransitionLibrary2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03cfae53_9580_4ee3_b363_2ece51b4af6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationTransitionLibrary2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateInstantaneousVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateDiscreteVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearVectorTransitionFromSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCubicVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCubicBezierLinearTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCubicBezierLinearVectorTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationVariable(::windows_core::IUnknown);
impl IUIAnimationVariable {
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFinalValue(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFinalValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPreviousValue(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPreviousValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetIntegerValue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetIntegerValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFinalIntegerValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPreviousIntegerValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows_core::Result<IUIAnimationStoryboard> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCurrentStoryboard(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLowerBound(bound)).ok()
    }
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetUpperBound(bound)).ok()
    }
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRoundingMode(mode)).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.SetTag(object.into_param().abi(), id)).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows_core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTag(::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn SetVariableChangeHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariableChangeHandler>,
    {
        ::windows_core::vcall!(self.SetVariableChangeHandler(handler.into_param().abi())).ok()
    }
    pub unsafe fn SetVariableIntegerChangeHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariableIntegerChangeHandler>,
    {
        ::windows_core::vcall!(self.SetVariableIntegerChangeHandler(handler.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationVariable, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationVariable {
    type Vtable = IUIAnimationVariable_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationVariable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ceeb155_2849_4ce5_9448_91ff70e1e4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows_core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows_core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows_core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT,
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationVariable2(::windows_core::IUnknown);
impl IUIAnimationVariable2 {
    pub unsafe fn GetDimension(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetDimension(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetVectorValue(&self, value: &mut [f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetVectorValue(::core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_DirectComposition\"`"]
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetCurve<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::DirectComposition::IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.GetCurve(animation.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_DirectComposition\"`"]
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetVectorCurve(&self, animation: &[::core::option::Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetVectorCurve(::core::mem::transmute(animation.as_ptr()), animation.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFinalValue(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFinalValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFinalVectorValue(&self, finalvalue: &mut [f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFinalVectorValue(::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetPreviousValue(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPreviousValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPreviousVectorValue(&self, previousvalue: &mut [f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetPreviousVectorValue(::core::mem::transmute(previousvalue.as_ptr()), previousvalue.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetIntegerValue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetIntegerValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetIntegerVectorValue(&self, value: &mut [i32]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetIntegerVectorValue(::core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFinalIntegerValue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFinalIntegerValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFinalIntegerVectorValue(&self, finalvalue: &mut [i32]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFinalIntegerVectorValue(::core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetPreviousIntegerValue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPreviousIntegerValue(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPreviousIntegerVectorValue(&self, previousvalue: &mut [i32]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetPreviousIntegerVectorValue(::core::mem::transmute(previousvalue.as_ptr()), previousvalue.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetCurrentStoryboard(&self) -> ::windows_core::Result<IUIAnimationStoryboard2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCurrentStoryboard(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetLowerBound(&self, bound: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLowerBound(bound)).ok()
    }
    pub unsafe fn SetLowerBoundVector(&self, bound: &[f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLowerBoundVector(::core::mem::transmute(bound.as_ptr()), bound.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetUpperBound(&self, bound: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetUpperBound(bound)).ok()
    }
    pub unsafe fn SetUpperBoundVector(&self, bound: &[f64]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetUpperBoundVector(::core::mem::transmute(bound.as_ptr()), bound.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRoundingMode(mode)).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.SetTag(object.into_param().abi(), id)).ok()
    }
    pub unsafe fn GetTag(&self, object: ::core::option::Option<*mut ::core::option::Option<::windows_core::IUnknown>>, id: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTag(::core::mem::transmute(object.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(id.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVariableChangeHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariableChangeHandler2>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetVariableChangeHandler(handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVariableIntegerChangeHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariableIntegerChangeHandler2>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetVariableIntegerChangeHandler(handler.into_param().abi(), fregisterfornextanimationevent.into_param().abi())).ok()
    }
    pub unsafe fn SetVariableCurveChangeHandler<P0>(&self, handler: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariableCurveChangeHandler2>,
    {
        ::windows_core::vcall!(self.SetVariableCurveChangeHandler(handler.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationVariable2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationVariable2 {
    type Vtable = IUIAnimationVariable2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationVariable2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4914b304_96ab_44d9_9e77_d5109b7e7466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariable2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dimension: *mut u32) -> ::windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64) -> ::windows_core::HRESULT,
    pub GetVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut f64, cdimension: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetCurve: usize,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetVectorCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *const *mut ::core::ffi::c_void, cdimension: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetVectorCurve: usize,
    pub GetFinalValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64) -> ::windows_core::HRESULT,
    pub GetFinalVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64) -> ::windows_core::HRESULT,
    pub GetPreviousVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT,
    pub GetIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32, cdimension: u32) -> ::windows_core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetFinalIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> ::windows_core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetPreviousIntegerVectorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> ::windows_core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT,
    pub SetLowerBoundVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: f64) -> ::windows_core::HRESULT,
    pub SetUpperBoundVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bound: *const f64, cdimension: u32) -> ::windows_core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableChangeHandler: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVariableIntegerChangeHandler: usize,
    pub SetVariableCurveChangeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationVariableChangeHandler(::windows_core::IUnknown);
impl IUIAnimationVariableChangeHandler {
    pub unsafe fn OnValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: f64, previousvalue: f64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard>,
        P1: ::windows_core::IntoParam<IUIAnimationVariable>,
    {
        ::windows_core::vcall!(self.OnValueChanged(storyboard.into_param().abi(), variable.into_param().abi(), newvalue, previousvalue)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationVariableChangeHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationVariableChangeHandler {
    type Vtable = IUIAnimationVariableChangeHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationVariableChangeHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6358b7ba_87d2_42d5_bf71_82e919dd5862);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: f64, previousvalue: f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationVariableChangeHandler2(::windows_core::IUnknown);
impl IUIAnimationVariableChangeHandler2 {
    pub unsafe fn OnValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard2>,
        P1: ::windows_core::IntoParam<IUIAnimationVariable2>,
    {
        ::windows_core::vcall!(self.OnValueChanged(storyboard.into_param().abi(), variable.into_param().abi(), newvalue, previousvalue, cdimension)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationVariableChangeHandler2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationVariableChangeHandler2 {
    type Vtable = IUIAnimationVariableChangeHandler2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationVariableChangeHandler2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63acc8d2_6eae_4bb0_b879_586dd8cfbe42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableChangeHandler2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationVariableCurveChangeHandler2(::windows_core::IUnknown);
impl IUIAnimationVariableCurveChangeHandler2 {
    pub unsafe fn OnCurveChanged<P0>(&self, variable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationVariable2>,
    {
        ::windows_core::vcall!(self.OnCurveChanged(variable.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationVariableCurveChangeHandler2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationVariableCurveChangeHandler2 {
    type Vtable = IUIAnimationVariableCurveChangeHandler2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationVariableCurveChangeHandler2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72895e91_0145_4c21_9192_5aab40eddf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnCurveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationVariableIntegerChangeHandler(::windows_core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler {
    pub unsafe fn OnIntegerValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: i32, previousvalue: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard>,
        P1: ::windows_core::IntoParam<IUIAnimationVariable>,
    {
        ::windows_core::vcall!(self.OnIntegerValueChanged(storyboard.into_param().abi(), variable.into_param().abi(), newvalue, previousvalue)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationVariableIntegerChangeHandler, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationVariableIntegerChangeHandler {
    type Vtable = IUIAnimationVariableIntegerChangeHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationVariableIntegerChangeHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb3e1550_356e_44b0_99da_85ac6017865e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: i32, previousvalue: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIAnimationVariableIntegerChangeHandler2(::windows_core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler2 {
    pub unsafe fn OnIntegerValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IUIAnimationStoryboard2>,
        P1: ::windows_core::IntoParam<IUIAnimationVariable2>,
    {
        ::windows_core::vcall!(self.OnIntegerValueChanged(storyboard.into_param().abi(), variable.into_param().abi(), newvalue, previousvalue, cdimension)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IUIAnimationVariableIntegerChangeHandler2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIAnimationVariableIntegerChangeHandler2 {
    type Vtable = IUIAnimationVariableIntegerChangeHandler2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIAnimationVariableIntegerChangeHandler2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x829b6cf1_4f3a_4412_ae09_b243eb4c6b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyboard: *mut ::core::ffi::c_void, variable: *mut ::core::ffi::c_void, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> ::windows_core::HRESULT,
}
pub const UIAnimationManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c1fc63a_695c_47e8_a339_1a194be3d0b8);
pub const UIAnimationManager2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd25d8842_8884_4a4a_b321_091314379bdd);
pub const UIAnimationTimer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfcd4a0c_06b6_4384_b768_0daa792c380e);
pub const UIAnimationTransitionFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a9b1cdd_fcd7_419c_8b44_42fd17db1887);
pub const UIAnimationTransitionFactory2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84302f97_7f7b_4040_b190_72ac9d18e420);
pub const UIAnimationTransitionLibrary: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d6322ad_aa85_4ef5_a828_86d71067d145);
pub const UIAnimationTransitionLibrary2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x812f944a_c5c8_4cd9_b0a6_b3da802f228d);
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(8i32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(2i32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(4i32);
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(1i32);
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(0i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(0i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(1i32);
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(1i32);
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(0i32);
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(0i32);
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(2i32);
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = UI_ANIMATION_MODE(1i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(1i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(0i32);
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(1i32);
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(0i32);
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(2i32);
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(1i32);
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(0i32);
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(2i32);
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(4i32);
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(1i32);
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(3i32);
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(0i32);
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(1i32);
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(0i32);
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(0i32);
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(2i32);
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(5i32);
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(7i32);
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(3i32);
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(6i32);
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(1i32);
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(4i32);
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(1i32);
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(0i32);
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(0i32);
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(1i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_DEPENDENCIES(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_DEPENDENCIES {}
impl ::core::clone::Clone for UI_ANIMATION_DEPENDENCIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_DEPENDENCIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_DEPENDENCIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_DEPENDENCIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_DEPENDENCIES").field(&self.0).finish()
    }
}
impl UI_ANIMATION_DEPENDENCIES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_IDLE_BEHAVIOR(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_IDLE_BEHAVIOR {}
impl ::core::clone::Clone for UI_ANIMATION_IDLE_BEHAVIOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_IDLE_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_IDLE_BEHAVIOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_IDLE_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_IDLE_BEHAVIOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_MANAGER_STATUS(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_MANAGER_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_MANAGER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_MANAGER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_MANAGER_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_MANAGER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MANAGER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_MODE(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_PRIORITY_EFFECT(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_PRIORITY_EFFECT {}
impl ::core::clone::Clone for UI_ANIMATION_PRIORITY_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_PRIORITY_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_PRIORITY_EFFECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_PRIORITY_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_PRIORITY_EFFECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_REPEAT_MODE(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_REPEAT_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_REPEAT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_REPEAT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_REPEAT_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_REPEAT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_REPEAT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_ROUNDING_MODE(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_ROUNDING_MODE {}
impl ::core::clone::Clone for UI_ANIMATION_ROUNDING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_ROUNDING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_ROUNDING_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_ROUNDING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_ROUNDING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_SCHEDULING_RESULT(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_SCHEDULING_RESULT {}
impl ::core::clone::Clone for UI_ANIMATION_SCHEDULING_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_SCHEDULING_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_SCHEDULING_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_SCHEDULING_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SCHEDULING_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_SLOPE(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_SLOPE {}
impl ::core::clone::Clone for UI_ANIMATION_SLOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_SLOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_SLOPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_SLOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SLOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_STORYBOARD_STATUS(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_STORYBOARD_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_STORYBOARD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_STORYBOARD_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_STORYBOARD_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_STORYBOARD_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_STORYBOARD_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_TIMER_CLIENT_STATUS(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_TIMER_CLIENT_STATUS {}
impl ::core::clone::Clone for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_TIMER_CLIENT_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_TIMER_CLIENT_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_UPDATE_RESULT(pub i32);
impl ::core::marker::Copy for UI_ANIMATION_UPDATE_RESULT {}
impl ::core::clone::Clone for UI_ANIMATION_UPDATE_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_ANIMATION_UPDATE_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_UPDATE_RESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UI_ANIMATION_UPDATE_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_UPDATE_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_ANIMATION_KEYFRAME(pub isize);
impl ::core::default::Default for UI_ANIMATION_KEYFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for UI_ANIMATION_KEYFRAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for UI_ANIMATION_KEYFRAME {}
impl ::core::fmt::Debug for UI_ANIMATION_KEYFRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_KEYFRAME").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for UI_ANIMATION_KEYFRAME {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
