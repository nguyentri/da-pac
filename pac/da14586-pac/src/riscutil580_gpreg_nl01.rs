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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:45 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"riscutil580_gpreg_nl01 registers"]
unsafe impl ::core::marker::Send for super::Riscutil580GpregNl01 {}
unsafe impl ::core::marker::Sync for super::Riscutil580GpregNl01 {}
impl super::Riscutil580GpregNl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn debug_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DebugReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DebugReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn set_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugReg_SPEC;
impl crate::sealed::RegSpec for DebugReg_SPEC {
    type DataType = u16;
}

pub type DebugReg = crate::RegValueT<DebugReg_SPEC>;

impl DebugReg {
    #[inline(always)]
    pub fn debugs_freeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for DebugReg {
    #[inline(always)]
    fn default() -> DebugReg {
        <crate::RegValueT<DebugReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpControlReg_SPEC;
impl crate::sealed::RegSpec for GpControlReg_SPEC {
    type DataType = u16;
}

pub type GpControlReg = crate::RegValueT<GpControlReg_SPEC>;

impl GpControlReg {
    #[inline(always)]
    pub fn em_map(
        self,
    ) -> crate::common::RegisterField<1, 0x1f, 1, 0, u8, u8, GpControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1f,1,0,u8,u8,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_wakeup_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpControlReg {
    #[inline(always)]
    fn default() -> GpControlReg {
        <crate::RegValueT<GpControlReg_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpStatusReg_SPEC;
impl crate::sealed::RegSpec for GpStatusReg_SPEC {
    type DataType = u16;
}

pub type GpStatusReg = crate::RegValueT<GpStatusReg_SPEC>;

impl GpStatusReg {
    #[inline(always)]
    pub fn cal_phase(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpStatusReg {
    #[inline(always)]
    fn default() -> GpStatusReg {
        <crate::RegValueT<GpStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetFreezeReg_SPEC;
impl crate::sealed::RegSpec for ResetFreezeReg_SPEC {
    type DataType = u16;
}

pub type ResetFreezeReg = crate::RegValueT<ResetFreezeReg_SPEC>;

impl ResetFreezeReg {
    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_swtim(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetFreezeReg {
    #[inline(always)]
    fn default() -> ResetFreezeReg {
        <crate::RegValueT<ResetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetFreezeReg_SPEC;
impl crate::sealed::RegSpec for SetFreezeReg_SPEC {
    type DataType = u16;
}

pub type SetFreezeReg = crate::RegValueT<SetFreezeReg_SPEC>;

impl SetFreezeReg {
    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_swtim(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetFreezeReg {
    #[inline(always)]
    fn default() -> SetFreezeReg {
        <crate::RegValueT<SetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
