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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:15 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CMAC_TIMER_SLP registers"]
unsafe impl ::core::marker::Send for super::CmacTimerSlp {}
unsafe impl ::core::marker::Sync for super::CmacTimerSlp {}
impl super::CmacTimerSlp {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn cm_slp_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmSlpCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmSlpCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cm_slp_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmSlpCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmSlpCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cm_slp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmSlpTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmSlpTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmSlpCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for CmSlpCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type CmSlpCtrl2Reg = crate::RegValueT<CmSlpCtrl2Reg_SPEC>;

impl CmSlpCtrl2Reg {
    #[inline(always)]
    pub fn cmac_wakeup_on_swd_state(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmSlpCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_active(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmSlpCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_clk_state(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CmSlpCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_cnt_sign(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CmSlpCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_irq_set(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmSlpCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmSlpCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_irq_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CmSlpCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CmSlpCtrl2Reg {
    #[inline(always)]
    fn default() -> CmSlpCtrl2Reg {
        <crate::RegValueT<CmSlpCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmSlpCtrlReg_SPEC;
impl crate::sealed::RegSpec for CmSlpCtrlReg_SPEC {
    type DataType = u32;
}

pub type CmSlpCtrlReg = crate::RegValueT<CmSlpCtrlReg_SPEC>;

impl CmSlpCtrlReg {
    #[inline(always)]
    pub fn cmac_wakeup_on_swd_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, CmSlpCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,CmSlpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tclk_from_lpclk(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmSlpCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CmSlpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tclk_from_pclk(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmSlpCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmSlpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_active(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmSlpCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmSlpCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_clk_state(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmSlpCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CmSlpCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_cnt_sign(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmSlpCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CmSlpCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slp_timer_sw(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmSlpCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CmSlpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmSlpCtrlReg {
    #[inline(always)]
    fn default() -> CmSlpCtrlReg {
        <crate::RegValueT<CmSlpCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmSlpTimerReg_SPEC;
impl crate::sealed::RegSpec for CmSlpTimerReg_SPEC {
    type DataType = u32;
}

pub type CmSlpTimerReg = crate::RegValueT<CmSlpTimerReg_SPEC>;

impl CmSlpTimerReg {
    #[inline(always)]
    pub fn cm_slp_timer_value(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        CmSlpTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            CmSlpTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CmSlpTimerReg {
    #[inline(always)]
    fn default() -> CmSlpTimerReg {
        <crate::RegValueT<CmSlpTimerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
