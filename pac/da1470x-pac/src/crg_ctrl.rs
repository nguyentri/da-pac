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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRG_CTRL registers"]
unsafe impl ::core::marker::Send for super::CrgCtrl {}
unsafe impl ::core::marker::Sync for super::CrgCtrl {}
impl super::CrgCtrl {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Clock control settings for PD_CTRL"]
    #[inline(always)]
    pub const fn clk_pdctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkPdctrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkPdctrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPdctrlReg_SPEC;
impl crate::sealed::RegSpec for ClkPdctrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Clock control settings for PD_CTRL"]
pub type ClkPdctrlReg = crate::RegValueT<ClkPdctrlReg_SPEC>;

impl ClkPdctrlReg {
    #[doc = "Inverts the clock in the TX path"]
    #[inline(always)]
    pub fn emmc_inv_tx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Invert the clock in the RX path, cascaded with INV_TX_CLK"]
    #[inline(always)]
    pub fn emmc_inv_rx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the clock."]
    #[inline(always)]
    pub fn emmc_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "clock divider setting\n0x0 : divide by 16\n0x1 : divide by 1\n0x2 : divide by 2\n0x4 : divide by 4\n0x8 : divide by 8"]
    #[inline(always)]
    pub fn emmc_clk_div(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0xf,1,0,u8, ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Inverts the clock in the TX path"]
    #[inline(always)]
    pub fn sdio_inv_tx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Invert the clock in the RX path, cascaded with INV_TX_CLK"]
    #[inline(always)]
    pub fn sdio_inv_rx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the clock."]
    #[inline(always)]
    pub fn sdio_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "clock divider setting\n0x0 : divide by 16\n0x1 : divide by 1\n0x2 : divide by 2\n0x4 : divide by 4\n0x8 : divide by 8"]
    #[inline(always)]
    pub fn sdio_clk_div(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkPdctrlReg {
    #[inline(always)]
    fn default() -> ClkPdctrlReg {
        <crate::RegValueT<ClkPdctrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
