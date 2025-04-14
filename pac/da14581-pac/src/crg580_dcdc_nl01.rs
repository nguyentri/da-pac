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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"crg580_dcdc_nl01 registers"]
unsafe impl ::core::marker::Send for super::Crg580DcdcNl01 {}
unsafe impl ::core::marker::Sync for super::Crg580DcdcNl01 {}
impl super::Crg580DcdcNl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dcdc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type DcdcCtrl2Reg = crate::RegValueT<DcdcCtrl2Reg_SPEC>;

impl DcdcCtrl2Reg {
    #[inline(always)]
    pub fn dcdc_volt_lev(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vbat3v_lev(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_ton(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_cur_lim(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_auto_cal(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl2Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl2Reg {
        <crate::RegValueT<DcdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(35872)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl3Reg_SPEC {
    type DataType = u16;
}

pub type DcdcCtrl3Reg = crate::RegValueT<DcdcCtrl3Reg_SPEC>;

impl DcdcCtrl3Reg {
    #[inline(always)]
    pub fn dcdc_timeout(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, DcdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,DcdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_idle_clk(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, DcdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,DcdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buck_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl3Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl3Reg {
        <crate::RegValueT<DcdcCtrl3Reg_SPEC> as RegisterValue<_>>::new(21)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrlReg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrlReg_SPEC {
    type DataType = u16;
}

pub type DcdcCtrlReg = crate::RegValueT<DcdcCtrlReg_SPEC>;

impl DcdcCtrlReg {
    #[inline(always)]
    pub fn dcdc_tune(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_drive_osw(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_drive_psw(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_drive_nsw(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_mode(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vbat1v_lev(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrlReg {
    #[inline(always)]
    fn default() -> DcdcCtrlReg {
        <crate::RegValueT<DcdcCtrlReg_SPEC> as RegisterValue<_>>::new(12)
    }
}
