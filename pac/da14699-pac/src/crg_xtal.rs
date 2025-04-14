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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:34 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRG_XTAL registers"]
unsafe impl ::core::marker::Send for super::CrgXtal {}
unsafe impl ::core::marker::Sync for super::CrgXtal {}
impl super::CrgXtal {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn clk_freq_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkFreqTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkFreqTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn trim_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TrimCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TrimCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtalrdy_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::XtalrdyCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::XtalrdyCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFreqTrimReg_SPEC;
impl crate::sealed::RegSpec for ClkFreqTrimReg_SPEC {
    type DataType = u32;
}

pub type ClkFreqTrimReg = crate::RegValueT<ClkFreqTrimReg_SPEC>;

impl ClkFreqTrimReg {
    #[inline(always)]
    pub fn xtal32m_start(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ClkFreqTrimReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ClkFreqTrimReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_ramp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ClkFreqTrimReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ClkFreqTrimReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_trim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ClkFreqTrimReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ClkFreqTrimReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkFreqTrimReg {
    #[inline(always)]
    fn default() -> ClkFreqTrimReg {
        <crate::RegValueT<ClkFreqTrimReg_SPEC> as RegisterValue<_>>::new(737869168)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysStatusReg_SPEC;
impl crate::sealed::RegSpec for PllSysStatusReg_SPEC {
    type DataType = u32;
}

pub type PllSysStatusReg = crate::RegValueT<PllSysStatusReg_SPEC>;

impl PllSysStatusReg {
    #[inline(always)]
    pub fn ldo_pll_ok(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_calibration_end(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_best_min_cur(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, u8, PllSysStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_lock_fine(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysStatusReg {
    #[inline(always)]
    fn default() -> PllSysStatusReg {
        <crate::RegValueT<PllSysStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimCtrlReg_SPEC;
impl crate::sealed::RegSpec for TrimCtrlReg_SPEC {
    type DataType = u32;
}

pub type TrimCtrlReg = crate::RegValueT<TrimCtrlReg_SPEC>;

impl TrimCtrlReg {
    #[inline(always)]
    pub fn xtal_settle_n(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal_trim_select(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal_count_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TrimCtrlReg {
    #[inline(always)]
    fn default() -> TrimCtrlReg {
        <crate::RegValueT<TrimCtrlReg_SPEC> as RegisterValue<_>>::new(1314)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XtalrdyCtrlReg_SPEC;
impl crate::sealed::RegSpec for XtalrdyCtrlReg_SPEC {
    type DataType = u32;
}

pub type XtalrdyCtrlReg = crate::RegValueT<XtalrdyCtrlReg_SPEC>;

impl XtalrdyCtrlReg {
    #[inline(always)]
    pub fn xtalrdy_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, XtalrdyCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,XtalrdyCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtalrdy_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, XtalrdyCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,XtalrdyCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for XtalrdyCtrlReg {
    #[inline(always)]
    fn default() -> XtalrdyCtrlReg {
        <crate::RegValueT<XtalrdyCtrlReg_SPEC> as RegisterValue<_>>::new(256)
    }
}
