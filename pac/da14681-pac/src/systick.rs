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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Cortex M0 SysTick registers"]
unsafe impl ::core::marker::Send for super::SysTick {}
unsafe impl ::core::marker::Sync for super::SysTick {}
impl super::SysTick {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ctrl(&self) -> &'static crate::common::Reg<self::Ctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn load(&self) -> &'static crate::common::Reg<self::Load_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Load_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn val(&self) -> &'static crate::common::Reg<self::Val_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Val_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn calib(&self) -> &'static crate::common::Reg<self::Calib_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Calib_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
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

pub type Ctrl = crate::RegValueT<Ctrl_SPEC>;

impl Ctrl {
    #[inline(always)]
    pub fn enable(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn tickint(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn clksource(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn countflag(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        <crate::RegValueT<Ctrl_SPEC> as RegisterValue<_>>::new(4)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Load_SPEC;
impl crate::sealed::RegSpec for Load_SPEC {
    type DataType = u32;
}

pub type Load = crate::RegValueT<Load_SPEC>;

impl Load {
    #[inline(always)]
    pub fn reload(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Load_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Load_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Load {
    #[inline(always)]
    fn default() -> Load {
        <crate::RegValueT<Load_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Val_SPEC;
impl crate::sealed::RegSpec for Val_SPEC {
    type DataType = u32;
}

pub type Val = crate::RegValueT<Val_SPEC>;

impl Val {
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Val_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Val_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Val {
    #[inline(always)]
    fn default() -> Val {
        <crate::RegValueT<Val_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib_SPEC;
impl crate::sealed::RegSpec for Calib_SPEC {
    type DataType = u32;
}

pub type Calib = crate::RegValueT<Calib_SPEC>;

impl Calib {
    #[inline(always)]
    pub fn tenms(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, u32, Calib_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32,u32,Calib_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn skew(self) -> crate::common::RegisterFieldBool<30, 1, 0, Calib_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Calib_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn noref(self) -> crate::common::RegisterFieldBool<31, 1, 0, Calib_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Calib_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Calib {
    #[inline(always)]
    fn default() -> Calib {
        <crate::RegValueT<Calib_SPEC> as RegisterValue<_>>::new(9000)
    }
}
