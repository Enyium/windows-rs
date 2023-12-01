#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConditionForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConditionForceEffect {
    type Vtable = IConditionForceEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConditionForceEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32d1ea68_3695_4e69_85c0_cd1944189140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConditionForceEffectKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConditionForceEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConditionForceEffectFactory {
    type Vtable = IConditionForceEffectFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConditionForceEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91a99264_1810_4eb6_a773_bfd3b8cddbab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: ConditionForceEffectKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConstantForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConstantForceEffect {
    type Vtable = IConstantForceEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConstantForceEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bfa0140_f3c7_415c_b068_0f068734bce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstantForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IForceFeedbackEffect(::windows_core::IUnknown);
impl IForceFeedbackEffect {
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gain(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetGain(value)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.State(&mut result__)).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Start()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Stop()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IForceFeedbackEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IForceFeedbackEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a17fba0c-2ae4-48c2-8063-eabd0777cb89}");
}
unsafe impl ::windows_core::Interface for IForceFeedbackEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IForceFeedbackEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectState) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IForceFeedbackMotor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IForceFeedbackMotor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackMotor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AreEffectsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SupportedAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectAxes) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadEffectAsync: usize,
    pub PauseAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnloadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnloadEffectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPeriodicForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPeriodicForceEffect {
    type Vtable = IPeriodicForceEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPeriodicForceEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c5138d7_fc75_4d52_9a0a_efe4cab5fe64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeriodicForceEffectKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPeriodicForceEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPeriodicForceEffectFactory {
    type Vtable = IPeriodicForceEffectFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPeriodicForceEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f62eb1a_9851_477b_b318_35ecaa15070f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: PeriodicForceEffectKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRampForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRampForceEffect {
    type Vtable = IRampForceEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRampForceEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1f81259_1ca6_4080_b56d_b43f3354d052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRampForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConditionForceEffect(::windows_core::IUnknown);
impl ConditionForceEffect {
    pub fn Kind(&self) -> ::windows_core::Result<ConditionForceEffectKind> {
        let this = &::windows_core::ComInterface::cast::<IConditionForceEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Kind(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IConditionForceEffect>(self)?;
        unsafe { ::windows_core::vcall!(this.SetParameters(direction, positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias)).ok() }
    }
    pub fn CreateInstance(effectkind: ConditionForceEffectKind) -> ::windows_core::Result<ConditionForceEffect> {
        Self::IConditionForceEffectFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateInstance(effectkind, &mut result__)).from_abi(result__)
        })
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gain(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetGain(value)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.State(&mut result__)).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Start()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Stop()).ok() }
    }
    #[doc(hidden)]
    pub fn IConditionForceEffectFactory<R, F: FnOnce(&IConditionForceEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ConditionForceEffect, IConditionForceEffectFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ConditionForceEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConditionForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows_core::Interface for ConditionForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConditionForceEffect {
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
}
::windows_core::imp::interface_hierarchy!(ConditionForceEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IForceFeedbackEffect> for ConditionForceEffect {}
unsafe impl ::core::marker::Send for ConditionForceEffect {}
unsafe impl ::core::marker::Sync for ConditionForceEffect {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConstantForceEffect(::windows_core::IUnknown);
impl ConstantForceEffect {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ConstantForceEffect, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IConstantForceEffect>(self)?;
        unsafe { ::windows_core::vcall!(this.SetParameters(vector, duration)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IConstantForceEffect>(self)?;
        unsafe { ::windows_core::vcall!(this.SetParametersWithEnvelope(vector, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount)).ok() }
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gain(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetGain(value)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.State(&mut result__)).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Start()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Stop()).ok() }
    }
}
impl ::windows_core::RuntimeType for ConstantForceEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConstantForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows_core::Interface for ConstantForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConstantForceEffect {
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
}
::windows_core::imp::interface_hierarchy!(ConstantForceEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IForceFeedbackEffect> for ConstantForceEffect {}
unsafe impl ::core::marker::Send for ConstantForceEffect {}
unsafe impl ::core::marker::Sync for ConstantForceEffect {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ForceFeedbackMotor(::windows_core::IUnknown);
impl ForceFeedbackMotor {
    pub fn AreEffectsPaused(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AreEffectsPaused(&mut result__)).from_abi(result__)
        }
    }
    pub fn MasterGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MasterGain(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetMasterGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetMasterGain(value)).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsEnabled(&mut result__)).from_abi(result__)
        }
    }
    pub fn SupportedAxes(&self) -> ::windows_core::Result<ForceFeedbackEffectAxes> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SupportedAxes(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LoadEffectAsync<P0>(&self, effect: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>
    where
        P0: ::windows_core::TryIntoParam<IForceFeedbackEffect>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LoadEffectAsync(effect.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    pub fn PauseAllEffects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.PauseAllEffects()).ok() }
    }
    pub fn ResumeAllEffects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.ResumeAllEffects()).ok() }
    }
    pub fn StopAllEffects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.StopAllEffects()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisableAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryDisableAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryEnableAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryEnableAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryResetAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryResetAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryUnloadEffectAsync<P0>(&self, effect: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<IForceFeedbackEffect>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryUnloadEffectAsync(effect.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ForceFeedbackMotor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor;{8d3d417c-a5ea-4516-8026-2b00f74ef6e5})");
}
unsafe impl ::windows_core::Interface for ForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ForceFeedbackMotor {
    const IID: ::windows_core::GUID = <IForceFeedbackMotor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
}
::windows_core::imp::interface_hierarchy!(ForceFeedbackMotor, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ForceFeedbackMotor {}
unsafe impl ::core::marker::Sync for ForceFeedbackMotor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PeriodicForceEffect(::windows_core::IUnknown);
impl PeriodicForceEffect {
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gain(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetGain(value)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.State(&mut result__)).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Start()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Stop()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<PeriodicForceEffectKind> {
        let this = &::windows_core::ComInterface::cast::<IPeriodicForceEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Kind(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { ::windows_core::vcall!(this.SetParameters(vector, frequency, phase, bias, duration)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { ::windows_core::vcall!(this.SetParametersWithEnvelope(vector, frequency, phase, bias, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount)).ok() }
    }
    pub fn CreateInstance(effectkind: PeriodicForceEffectKind) -> ::windows_core::Result<PeriodicForceEffect> {
        Self::IPeriodicForceEffectFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateInstance(effectkind, &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPeriodicForceEffectFactory<R, F: FnOnce(&IPeriodicForceEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PeriodicForceEffect, IPeriodicForceEffectFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PeriodicForceEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows_core::Interface for PeriodicForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PeriodicForceEffect {
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
}
::windows_core::imp::interface_hierarchy!(PeriodicForceEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IForceFeedbackEffect> for PeriodicForceEffect {}
unsafe impl ::core::marker::Send for PeriodicForceEffect {}
unsafe impl ::core::marker::Sync for PeriodicForceEffect {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RampForceEffect(::windows_core::IUnknown);
impl RampForceEffect {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RampForceEffect, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gain(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetGain(value)).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.State(&mut result__)).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Start()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Stop()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IRampForceEffect>(self)?;
        unsafe { ::windows_core::vcall!(this.SetParameters(startvector, endvector, duration)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IRampForceEffect>(self)?;
        unsafe { ::windows_core::vcall!(this.SetParametersWithEnvelope(startvector, endvector, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount)).ok() }
    }
}
impl ::windows_core::RuntimeType for RampForceEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.RampForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
}
unsafe impl ::windows_core::Interface for RampForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RampForceEffect {
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.RampForceEffect";
}
::windows_core::imp::interface_hierarchy!(RampForceEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IForceFeedbackEffect> for RampForceEffect {}
unsafe impl ::core::marker::Send for RampForceEffect {}
unsafe impl ::core::marker::Sync for RampForceEffect {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: Self = Self(0i32);
    pub const Damper: Self = Self(1i32);
    pub const Inertia: Self = Self(2i32);
    pub const Friction: Self = Self(3i32);
}
impl ::core::marker::Copy for ConditionForceEffectKind {}
impl ::core::clone::Clone for ConditionForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConditionForceEffectKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ConditionForceEffectKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ConditionForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConditionForceEffectKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ConditionForceEffectKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: Self = Self(0u32);
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for ForceFeedbackEffectAxes {}
impl ::core::clone::Clone for ForceFeedbackEffectAxes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackEffectAxes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ForceFeedbackEffectAxes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ForceFeedbackEffectAxes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectAxes").field(&self.0).finish()
    }
}
impl ForceFeedbackEffectAxes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ForceFeedbackEffectAxes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ForceFeedbackEffectAxes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ForceFeedbackEffectAxes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for ForceFeedbackEffectAxes {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Faulted: Self = Self(3i32);
}
impl ::core::marker::Copy for ForceFeedbackEffectState {}
impl ::core::clone::Clone for ForceFeedbackEffectState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackEffectState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ForceFeedbackEffectState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ForceFeedbackEffectState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ForceFeedbackEffectState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: Self = Self(0i32);
    pub const EffectStorageFull: Self = Self(1i32);
    pub const EffectNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for ForceFeedbackLoadEffectResult {}
impl ::core::clone::Clone for ForceFeedbackLoadEffectResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackLoadEffectResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ForceFeedbackLoadEffectResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ForceFeedbackLoadEffectResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackLoadEffectResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ForceFeedbackLoadEffectResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: Self = Self(0i32);
    pub const SineWave: Self = Self(1i32);
    pub const TriangleWave: Self = Self(2i32);
    pub const SawtoothWaveUp: Self = Self(3i32);
    pub const SawtoothWaveDown: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicForceEffectKind {}
impl ::core::clone::Clone for PeriodicForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeriodicForceEffectKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PeriodicForceEffectKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PeriodicForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicForceEffectKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PeriodicForceEffectKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
