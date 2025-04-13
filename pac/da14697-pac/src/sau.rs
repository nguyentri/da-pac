/*
DISCLAIMER
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products.
No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all
applicable laws, including copyright laws.
THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING THIS SOFTWARE, WHETHER EXPRESS, IMPLIED
OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NON-INFRINGEMENT.  ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED.TO THE MAXIMUM EXTENT PERMITTED NOT PROHIBITED BY
LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES SHALL BE LIABLE FOR ANY DIRECT,
INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS SOFTWARE, EVEN IF RENESAS OR
ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability
of this software. By using this software, you agree to the additional terms and conditions found by accessing the
following link:
http://www.renesas.com/disclaimer

*/
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:54:07 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Security Attribution Unit"]
unsafe impl ::core::marker::Send for super::Sau {}
unsafe impl ::core::marker::Sync for super::Sau {}
impl super::Sau {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &'static crate::common::Reg<self::Ctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Type Register"]
    #[inline(always)]
    pub const fn r#type(&self) -> &'static crate::common::Reg<self::Type_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Type_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Region Number Register"]
    #[inline(always)]
    pub const fn rnr(&self) -> &'static crate::common::Reg<self::Rnr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rnr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Region Base Address Register"]
    #[inline(always)]
    pub const fn rbar(&self) -> &'static crate::common::Reg<self::Rbar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rbar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Region Limit Address Register"]
    #[inline(always)]
    pub const fn rlar(&self) -> &'static crate::common::Reg<self::Rlar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rlar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl_SPEC;
impl crate::sealed::RegSpec for Ctrl_SPEC {
    type DataType = u32;
}
#[doc = "Control Register"]
pub type Ctrl = crate::RegValueT<Ctrl_SPEC>;

impl NoBitfieldReg<Ctrl_SPEC> for Ctrl {}
impl ::core::default::Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        <crate::RegValueT<Ctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type_SPEC;
impl crate::sealed::RegSpec for Type_SPEC {
    type DataType = u32;
}
#[doc = "Type Register"]
pub type Type = crate::RegValueT<Type_SPEC>;

impl NoBitfieldReg<Type_SPEC> for Type {}
impl ::core::default::Default for Type {
    #[inline(always)]
    fn default() -> Type {
        <crate::RegValueT<Type_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rnr_SPEC;
impl crate::sealed::RegSpec for Rnr_SPEC {
    type DataType = u32;
}
#[doc = "Region Number Register"]
pub type Rnr = crate::RegValueT<Rnr_SPEC>;

impl NoBitfieldReg<Rnr_SPEC> for Rnr {}
impl ::core::default::Default for Rnr {
    #[inline(always)]
    fn default() -> Rnr {
        <crate::RegValueT<Rnr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbar_SPEC;
impl crate::sealed::RegSpec for Rbar_SPEC {
    type DataType = u32;
}
#[doc = "Region Base Address Register"]
pub type Rbar = crate::RegValueT<Rbar_SPEC>;

impl NoBitfieldReg<Rbar_SPEC> for Rbar {}
impl ::core::default::Default for Rbar {
    #[inline(always)]
    fn default() -> Rbar {
        <crate::RegValueT<Rbar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlar_SPEC;
impl crate::sealed::RegSpec for Rlar_SPEC {
    type DataType = u32;
}
#[doc = "Region Limit Address Register"]
pub type Rlar = crate::RegValueT<Rlar_SPEC>;

impl NoBitfieldReg<Rlar_SPEC> for Rlar {}
impl ::core::default::Default for Rlar {
    #[inline(always)]
    fn default() -> Rlar {
        <crate::RegValueT<Rlar_SPEC> as RegisterValue<_>>::new(0)
    }
}
