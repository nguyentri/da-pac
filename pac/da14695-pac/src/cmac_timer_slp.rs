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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:53:57 +0000

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
    #[doc = "CMAC Sleep Control 2 (no RMW)"]
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

    #[doc = "CMAC Sleep Control 1 (allowed to RMW)"]
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

    #[doc = "CMAC Sleep Timer"]
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
#[doc = "CMAC Sleep Control 2 (no RMW)"]
pub type CmSlpCtrl2Reg = crate::RegValueT<CmSlpCtrl2Reg_SPEC>;

impl CmSlpCtrl2Reg {
    #[doc = "Provides the current state of CMAC_WAKEUP_ON_SWD.\nWriting \'1\' will clear this bit.\nWriting \'0\' has no effect.\nWhen CM_SLP_CTRL_REG->CMAC_WAKEUP_ON_SWD_EN=1 and the Radio Power Domain is down and SYS_CTRL_REG->CMAC_DEBUGGER_ENABLE=1 then any negative edge on SWDCLK pin will set the CMAC_WAKEUP_ON_SWD.\nThe CMAC_WAKEUP_ON_SWD logically OR-ed with SLP_TIMER_IRQ is connected to PDC and thus it is able to wake up the CMAC, allowing the connection of CMAC Cortex with the debugger.\nNote: If the pins are not used as CMAC SWD, then keep SYS_CTRL_REG->CMAC_DEBUGGER_ENABLE=0 to avoid false wakeup triggers."]
    #[inline(always)]
    pub fn cmac_wakeup_on_swd_state(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmSlpCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to CM_SLP_CTRL_REG->SLP_TIMER_ACTIVE"]
    #[inline(always)]
    pub fn slp_timer_active(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmSlpCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to CM_SLP_CTRL_REG->LP_CLK_STATE."]
    #[inline(always)]
    pub fn lp_clk_state(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CmSlpCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer sign"]
    #[inline(always)]
    pub fn slp_timer_cnt_sign(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CmSlpCtrl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Writing \'1\' will cause the IRQ to be set. This field remains to \'1\' until the IRQ is set.\nWriting \'0\' has no effect.\nSystem CPU SW may use this field to force CMAC to wakeup through SLP_TIMER.\nNote that typically SW wakes up CMAC through the SYS2CMAC_IRQ (via PDC)."]
    #[inline(always)]
    pub fn slp_timer_irq_set(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmSlpCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing \'1\' will cause the IRQ to be cleared. This field remains to \'1\' until the IRQ is cleared.\nWriting \'0\' has no effect.\nNote that clearing the IRQ is not possible as long as the Timer is \"1\", since the Expire event has higher priority."]
    #[inline(always)]
    pub fn slp_timer_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmSlpCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmSlpCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Provides the current state of the CMAC Sleep Timer IRQ."]
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
#[doc = "CMAC Sleep Control 1 (allowed to RMW)"]
pub type CmSlpCtrlReg = crate::RegValueT<CmSlpCtrlReg_SPEC>;

impl CmSlpCtrlReg {
    #[doc = "If \'1\' then enable the generation of CMAC_WAKEUP_ON_SWD."]
    #[inline(always)]
    pub fn cmac_wakeup_on_swd_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, CmSlpCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,CmSlpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "T Clock"]
    #[inline(always)]
    pub fn tclk_from_lpclk(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmSlpCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CmSlpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "T Clock"]
    #[inline(always)]
    pub fn tclk_from_pclk(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmSlpCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmSlpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to SLP_TIMER_SW"]
    #[inline(always)]
    pub fn slp_timer_active(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmSlpCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmSlpCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LP_CLK state"]
    #[inline(always)]
    pub fn lp_clk_state(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmSlpCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CmSlpCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to CM_SLP_CTRL2_REG->SLP_TIMER_CNT_SIGN."]
    #[inline(always)]
    pub fn slp_timer_cnt_sign(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmSlpCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CmSlpCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer SW"]
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
#[doc = "CMAC Sleep Timer"]
pub type CmSlpTimerReg = crate::RegValueT<CmSlpTimerReg_SPEC>;

impl CmSlpTimerReg {
    #[doc = "Timer value"]
    #[inline(always)]
    pub fn cm_slp_timer_value(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, CmSlpTimerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
