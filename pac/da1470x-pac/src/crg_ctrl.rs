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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:41 +0000

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

pub type ClkPdctrlReg = crate::RegValueT<ClkPdctrlReg_SPEC>;

impl ClkPdctrlReg {
    #[inline(always)]
    pub fn emmc_inv_tx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn emmc_inv_rx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn emmc_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn emmc_clk_div(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, u8, ClkPdctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xf,1,0,u8,u8,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdio_inv_tx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdio_inv_rx_clk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdio_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkPdctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdio_clk_div(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, ClkPdctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,ClkPdctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkPdctrlReg {
    #[inline(always)]
    fn default() -> ClkPdctrlReg {
        <crate::RegValueT<ClkPdctrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
