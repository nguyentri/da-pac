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
#[doc = r"CMAC registers"]
unsafe impl ::core::marker::Send for super::Cmac {}
unsafe impl ::core::marker::Sync for super::Cmac {}
impl super::Cmac {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

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

pub type CmCtrlSysReg = crate::RegValueT<CmCtrlSysReg_SPEC>;

impl CmCtrlSysReg {
    #[inline(always)]
    pub fn cmac_const_1(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_lockup_state(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_wdog_expire_state(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_sysmemctrl_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_cpu_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_bs_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_fw_error_state(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mcpu_sleeping_state(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_rst_mcpu_state(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_rst_bs_state(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmCtrlSysReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmCtrlSysReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac2sys_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmCtrlSysReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmCtrlSysReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type CmDiagIrq1EdgeReg = crate::RegValueT<CmDiagIrq1EdgeReg_SPEC>;

impl CmDiagIrq1EdgeReg {
    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1EdgeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1EdgeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type CmDiagIrq1MaskReg = crate::RegValueT<CmDiagIrq1MaskReg_SPEC>;

impl CmDiagIrq1MaskReg {
    #[inline(always)]
    pub fn diag1_signal_detected(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_match0101(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_sync_found(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1MaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1MaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type CmDiagIrq1StatReg = crate::RegValueT<CmDiagIrq1StatReg_SPEC>;

impl CmDiagIrq1StatReg {
    #[inline(always)]
    pub fn diag1_signal_detected(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_match0101(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_sync_found(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1StatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1StatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type CmDiagIrq1WordReg = crate::RegValueT<CmDiagIrq1WordReg_SPEC>;

impl CmDiagIrq1WordReg {
    #[inline(always)]
    pub fn diag1_signal_detected(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_match0101(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_sync_found(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_phy_tx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_phy_rx_en_rfcu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_26(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_25(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_24(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_23(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_dcf_22(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmDiagIrq1WordReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CmDiagIrq1WordReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type CmWdogReg = crate::RegValueT<CmWdogReg_SPEC>;

impl CmWdogReg {
    #[inline(always)]
    pub fn sys2cmac_wdog_freeze(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, CmWdogReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,CmWdogReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys2cmac_wdog_freeze_dis(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, CmWdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,CmWdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cm_wdog_expire(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, CmWdogReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,CmWdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cm_wdog_sys_rst_req(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, CmWdogReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,CmWdogReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cm_wdog_write_valid(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, CmWdogReg_SPEC, crate::common::W> {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,CmWdogReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cm_wdog_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, CmWdogReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,CmWdogReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CmWdogReg {
    #[inline(always)]
    fn default() -> CmWdogReg {
        <crate::RegValueT<CmWdogReg_SPEC> as RegisterValue<_>>::new(8191)
    }
}
