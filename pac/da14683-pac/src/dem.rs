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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DEM registers"]
unsafe impl ::core::marker::Send for super::Dem {}
unsafe impl ::core::marker::Sync for super::Dem {}
impl super::Dem {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn rf_ftdf_sigdet_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfFtdfSigdetCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfFtdfSigdetCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfFtdfSigdetCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfFtdfSigdetCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfFtdfSigdetCtrlReg = crate::RegValueT<RfFtdfSigdetCtrlReg_SPEC>;

impl RfFtdfSigdetCtrlReg {
    #[inline(always)]
    pub fn sigdet_delay(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x3,
        1,
        0,
        u8,
        u8,
        RfFtdfSigdetCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x3,
            1,
            0,
            u8,
            u8,
            RfFtdfSigdetCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sigdet_sfactor1(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x3,
        1,
        0,
        u8,
        u8,
        RfFtdfSigdetCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x3,
            1,
            0,
            u8,
            u8,
            RfFtdfSigdetCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sigdet_sfactor2(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x3,
        1,
        0,
        u8,
        u8,
        RfFtdfSigdetCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x3,
            1,
            0,
            u8,
            u8,
            RfFtdfSigdetCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sigdet_diff(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfFtdfSigdetCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,RfFtdfSigdetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sigdet_threshold(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3f,
        1,
        0,
        u8,
        u8,
        RfFtdfSigdetCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3f,
            1,
            0,
            u8,
            u8,
            RfFtdfSigdetCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sigdet_mode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        RfFtdfSigdetCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            RfFtdfSigdetCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfFtdfSigdetCtrlReg {
    #[inline(always)]
    fn default() -> RfFtdfSigdetCtrlReg {
        <crate::RegValueT<RfFtdfSigdetCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
