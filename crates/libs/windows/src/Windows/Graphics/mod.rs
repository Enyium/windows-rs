#[cfg(feature = "Graphics_Capture")]
#[doc = "Required features: `\"Graphics_Capture\"`"]
pub mod Capture;
#[cfg(feature = "Graphics_DirectX")]
#[doc = "Required features: `\"Graphics_DirectX\"`"]
pub mod DirectX;
#[cfg(feature = "Graphics_Display")]
#[doc = "Required features: `\"Graphics_Display\"`"]
pub mod Display;
#[cfg(feature = "Graphics_Effects")]
#[doc = "Required features: `\"Graphics_Effects\"`"]
pub mod Effects;
#[cfg(feature = "Graphics_Holographic")]
#[doc = "Required features: `\"Graphics_Holographic\"`"]
pub mod Holographic;
#[cfg(feature = "Graphics_Imaging")]
#[doc = "Required features: `\"Graphics_Imaging\"`"]
pub mod Imaging;
#[cfg(feature = "Graphics_Printing")]
#[doc = "Required features: `\"Graphics_Printing\"`"]
pub mod Printing;
#[cfg(feature = "Graphics_Printing3D")]
#[doc = "Required features: `\"Graphics_Printing3D\"`"]
pub mod Printing3D;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeometrySource2D(::windows_core::IUnknown);
impl IGeometrySource2D {}
::windows_core::imp::interface_hierarchy!(IGeometrySource2D, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IGeometrySource2D {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IGeometrySource2D {
    type Vtable = IGeometrySource2D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeometrySource2D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcaff7902_670c_4181_a624_da977203b845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2D_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(C)]
pub struct DisplayAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for DisplayAdapterId {}
impl ::core::clone::Clone for DisplayAdapterId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DisplayAdapterId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayAdapterId").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl ::windows_core::TypeKind for DisplayAdapterId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for DisplayAdapterId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.DisplayAdapterId;u4;i4)");
}
impl ::core::cmp::PartialEq for DisplayAdapterId {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for DisplayAdapterId {}
impl ::core::default::Default for DisplayAdapterId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DisplayId {
    pub Value: u64,
}
impl ::core::marker::Copy for DisplayId {}
impl ::core::clone::Clone for DisplayId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DisplayId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayId").field("Value", &self.Value).finish()
    }
}
impl ::windows_core::TypeKind for DisplayId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for DisplayId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.DisplayId;u8)");
}
impl ::core::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DisplayId {}
impl ::core::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PointInt32 {
    pub X: i32,
    pub Y: i32,
}
impl ::core::marker::Copy for PointInt32 {}
impl ::core::clone::Clone for PointInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PointInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PointInt32").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::windows_core::TypeKind for PointInt32 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for PointInt32 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.PointInt32;i4;i4)");
}
impl ::core::cmp::PartialEq for PointInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for PointInt32 {}
impl ::core::default::Default for PointInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct RectInt32 {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for RectInt32 {}
impl ::core::clone::Clone for RectInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RectInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RectInt32").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::windows_core::TypeKind for RectInt32 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for RectInt32 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.RectInt32;i4;i4;i4;i4)");
}
impl ::core::cmp::PartialEq for RectInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for RectInt32 {}
impl ::core::default::Default for RectInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SizeInt32 {
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for SizeInt32 {}
impl ::core::clone::Clone for SizeInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SizeInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SizeInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::windows_core::TypeKind for SizeInt32 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for SizeInt32 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.SizeInt32;i4;i4)");
}
impl ::core::cmp::PartialEq for SizeInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for SizeInt32 {}
impl ::core::default::Default for SizeInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
