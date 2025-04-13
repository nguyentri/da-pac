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
#[doc = r"CMAC registers"]
unsafe impl ::core::marker::Send for super::Cmac {}
unsafe impl ::core::marker::Sync for super::Cmac {}
impl super::Cmac {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "CMAC and System Control Register"]
    #[inline(always)]
    pub const fn cm_ctrl_sys_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmCtrlSysReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmCtrlSysReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8192usize),
            )
        }
    }

    #[doc = "Diagnostic IRQ on Word1 - Edge Register"]
    #[inline(always)]
    pub const fn cm_diag_irq1_edge_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmDiagIrq1EdgeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8452usize),
            )
        }
    }

    #[doc = "Diagnostic IRQ on Word1 - Mask Register"]
    #[inline(always)]
    pub const fn cm_diag_irq1_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmDiagIrq1MaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8460usize),
            )
        }
    }

    #[doc = "Diagnostic IRQ on Word1 - Status Register"]
    #[inline(always)]
    pub const fn cm_diag_irq1_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmDiagIrq1StatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8456usize),
            )
        }
    }

    #[doc = "Diagnostic IRQ on Word1 - Word1 Register"]
    #[inline(always)]
    pub const fn cm_diag_irq1_word_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmDiagIrq1WordReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmDiagIrq1WordReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8448usize),
            )
        }
    }

    #[doc = "CMAC Watch Dog Control Register"]
    #[inline(always)]
    pub const fn cm_wdog_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CmWdogReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CmWdogReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8196usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmCtrlSysReg_SPEC;
impl crate::sealed::RegSpec for CmCtrlSysReg_SPEC {
    type DataType = u32;
}
#[doc = "CMAC and System Control Register"]
pub type CmCtrlSysReg = crate::RegValueT<CmCtrlSysReg_SPEC>;

impl CmCtrlSysReg {
    #[doc = "Always read as \"1\".\nNote: Creating an always non-zero register value, making easier a visual check of register when power domain is off."]
    #[inline(always)]
    pub fn cmac_const_1(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmac_lockup_state(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmac_wdog_expire_state(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmac_sysmemctrl_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmac_cpu_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmac_bs_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmac_fw_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn mcpu_sleeping_state(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The state of the CMAC M0+ reset signal.\nNote that this reset is driven also by CLK_RADIO_REG->CMAC_SYNCH_RESET."]
    #[inline(always)]
    pub fn cmac_rst_mcpu_state(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn cmac_rst_bs_state(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Writing \"0\" will have no effect.\nWriting \"1\" will clear the CMAC2SYS_IRQ, a process that depends on the state of CMAC and the relationship of the PCLK and CMAC clocks.\nReading will return \"1\" as long as the clearing process is pending, otherwise it will return \"0\"."]
    #[inline(always)]
    pub fn cmac2sys_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmCtrlSysReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmCtrlSysReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The current state of the CMAC2SYS_IRQ signal."]
    #[inline(always)]
    pub fn cmac2sys_irq_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CmCtrlSysReg {
    #[inline(always)]
    fn default() -> CmCtrlSysReg {
        <crate::RegValueT<CmCtrlSysReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmDiagIrq1EdgeReg_SPEC;
impl crate::sealed::RegSpec for CmDiagIrq1EdgeReg_SPEC {
    type DataType = u32;
}
#[doc = "Diagnostic IRQ on Word1 - Edge Register"]
pub type CmDiagIrq1EdgeReg = crate::RegValueT<CmDiagIrq1EdgeReg_SPEC>;

impl CmDiagIrq1EdgeReg {
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: The positive edge is selected to set the corresponding bit of CM_DIAG_IRQ1_STAT_REG.\n1: The negative edge is selected."]
    #[inline(always)]
    pub fn diag1_dcf_21(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmDiagIrq1EdgeReg {
    #[inline(always)]
    fn default() -> CmDiagIrq1EdgeReg {
        <crate::RegValueT<CmDiagIrq1EdgeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmDiagIrq1MaskReg_SPEC;
impl crate::sealed::RegSpec for CmDiagIrq1MaskReg_SPEC {
    type DataType = u32;
}
#[doc = "Diagnostic IRQ on Word1 - Mask Register"]
pub type CmDiagIrq1MaskReg = crate::RegValueT<CmDiagIrq1MaskReg_SPEC>;

impl CmDiagIrq1MaskReg {
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_signal_detected(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_match0101(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_sync_found(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: Raise an DIAG_IRQ when the corresponding bit of CM_DIAG_IRQ1_STAT_REG is also \"1\".\n0: Mask the state of the corresponding bit of CM_DIAG_IRQ1_STAT_REG in order to not trigger DIAG_IRQ."]
    #[inline(always)]
    pub fn diag1_dcf_21(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmDiagIrq1MaskReg {
    #[inline(always)]
    fn default() -> CmDiagIrq1MaskReg {
        <crate::RegValueT<CmDiagIrq1MaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmDiagIrq1StatReg_SPEC;
impl crate::sealed::RegSpec for CmDiagIrq1StatReg_SPEC {
    type DataType = u32;
}
#[doc = "Diagnostic IRQ on Word1 - Status Register"]
pub type CmDiagIrq1StatReg = crate::RegValueT<CmDiagIrq1StatReg_SPEC>;

impl CmDiagIrq1StatReg {
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_signal_detected(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_match0101(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_sync_found(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: the corresponding event is pending.\n0: the corresponding event is not pending.\nWriting a \'1\' will clear the corresponding bit.\nWriting a \'0\' into a bit will have no effect.\nUse this register to detect and acknowledge the source that triggers the DIAG_IRQ."]
    #[inline(always)]
    pub fn diag1_dcf_21(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmDiagIrq1StatReg {
    #[inline(always)]
    fn default() -> CmDiagIrq1StatReg {
        <crate::RegValueT<CmDiagIrq1StatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmDiagIrq1WordReg_SPEC;
impl crate::sealed::RegSpec for CmDiagIrq1WordReg_SPEC {
    type DataType = u32;
}
#[doc = "Diagnostic IRQ on Word1 - Word1 Register"]
pub type CmDiagIrq1WordReg = crate::RegValueT<CmDiagIrq1WordReg_SPEC>;

impl CmDiagIrq1WordReg {
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_signal_detected(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_match0101(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_sync_found(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Refer to bit 0."]
    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Same signal as the one in CM_DIAG_WORD1_REG.\nRefer to CM_DIAG_WORD1_REG for signal description."]
    #[inline(always)]
    pub fn diag1_dcf_21(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CmDiagIrq1WordReg {
    #[inline(always)]
    fn default() -> CmDiagIrq1WordReg {
        <crate::RegValueT<CmDiagIrq1WordReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmWdogReg_SPEC;
impl crate::sealed::RegSpec for CmWdogReg_SPEC {
    type DataType = u32;
}
#[doc = "CMAC Watch Dog Control Register"]
pub type CmWdogReg = crate::RegValueT<CmWdogReg_SPEC>;

impl CmWdogReg {
    #[doc = "A read-only copy of SET_FREEZE_REG->FRZ_CMAC_WDOG value."]
    #[inline(always)]
    pub fn sys2cmac_wdog_freeze(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, CmWdogReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,CmWdogReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Setting to \'1\' will mask the SYS2CMAC_WDOG_FREEZE, which is provided by SET_FREEZE_REG->FRZ_CMAC_WDOG.\nSetting to \"1\" can be done only by writing at the same time CM_WDOG_WRITE_VALID with ones.\nThe field can be only set to \'1\', so it can be set during the initilization and it will not change during the reloadings.\nIt can be reseted either via power cycling the Power Domain or via the CLK_RADIO_REG->CMAC_SYNCH_RESET."]
    #[inline(always)]
    pub fn sys2cmac_wdog_freeze_dis(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, CmWdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,CmWdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit automatically is set to \"1\" as soon as CM_WDOG_CNT=0, causing CM_WDOG_CNT to start counting again from the value of \"16\" and also asserting CM_ERROR_REG->CM_WDOG_EXPIRE_ERR.\nIf the SW will write CM_WDOG_EXPIRE = 0 and CM_WDOG_CNT=0, then at the next WDOG clock cycle the CM_WDOG_EXPIRE will automatically be set to to \"1\".\nIf the SW will write CM_WDOG_EXPIRE = 1 and CM_WDOG_CNT=0, then at the next WDOG clock cycle the CM_WDOG_SYS_RST_REQ will automatically be set to \"1\".\nThe CM_WDOG_SYS_RST_REQ will reset the system and will update the RESET_STAT_REG->CMAC_WDOGRESET_STAT.\nRefer also to CM_EXC_STAT_REG->EXC_WDOG_EARLY."]
    #[inline(always)]
    pub fn cm_wdog_expire(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, CmWdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,CmWdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refer to CM_WDOG_EXPIRE."]
    #[inline(always)]
    pub fn cm_wdog_sys_rst_req(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, CmWdogReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,CmWdogReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "In order to allow a write of any of the remaining fields, this value must be also written simultaneously with the value \"3\".\nReading this field will return always \'0\'."]
    #[inline(always)]
    pub fn cm_wdog_write_valid(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, CmWdogReg_SPEC, crate::common::W> {
        crate::common::RegisterField::<17,0x3,1,0,u8, CmWdogReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Provides access to the counter, which counts down every 10.24 msec.\nFW should reload the WDOG counter by writing to CM_WDOG_REG the value (CM_WDOG_CNT | CM_WDOG_WRITE_VALID), i.e. write can be done only by writing at the same time CM_WDOG_WRITE_VALID with ones.\nThe counter will start counting immediately after the power up of the Power Domain."]
    #[inline(always)]
    pub fn cm_wdog_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, CmWdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, CmWdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmWdogReg {
    #[inline(always)]
    fn default() -> CmWdogReg {
        <crate::RegValueT<CmWdogReg_SPEC> as RegisterValue<_>>::new(8191)
    }
}
