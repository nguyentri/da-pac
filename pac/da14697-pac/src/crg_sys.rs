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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRG_SYS registers"]
unsafe impl ::core::marker::Send for super::CrgSys {}
unsafe impl ::core::marker::Sync for super::CrgSys {}
impl super::CrgSys {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn batcheck_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BatcheckReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BatcheckReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_sys_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkSysReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkSysReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BatcheckReg_SPEC;
impl crate::sealed::RegSpec for BatcheckReg_SPEC {
    type DataType = u32;
}

pub type BatcheckReg = crate::RegValueT<BatcheckReg_SPEC>;

impl BatcheckReg {
    #[inline(always)]
    pub fn batcheck_load_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BatcheckReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BatcheckReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn batcheck_iload(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, BatcheckReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,BatcheckReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn batcheck_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, BatcheckReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,BatcheckReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BatcheckReg {
    #[inline(always)]
    fn default() -> BatcheckReg {
        <crate::RegValueT<BatcheckReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSysReg_SPEC;
impl crate::sealed::RegSpec for ClkSysReg_SPEC {
    type DataType = u32;
}

pub type ClkSysReg = crate::RegValueT<ClkSysReg_SPEC>;

impl ClkSysReg {
    #[inline(always)]
    pub fn clk_chg_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkSysReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkSysReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcd_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkSysReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkSysReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcd_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkSysReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkSysReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcd_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkSysReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkSysReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkSysReg {
    #[inline(always)]
    fn default() -> ClkSysReg {
        <crate::RegValueT<ClkSysReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
