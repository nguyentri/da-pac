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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:50 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DCDC registers"]
unsafe impl ::core::marker::Send for super::Dcdc {}
unsafe impl ::core::marker::Sync for super::Dcdc {}
impl super::Dcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dcdc_ctrl_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_ctrl_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_ctrl_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_ret_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcRet0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcRet0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_ret_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcRet1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcRet1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status_4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_test_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTest0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTest0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_test_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTest1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTest1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_trim_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTrim0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTrim0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_trim_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTrim1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTrim1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_trim_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTrim2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTrim2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_trim_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTrim3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTrim3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v14_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV140Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV140Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v14_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV141Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV141Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v18p_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV18P0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV18P0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v18p_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV18P1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV18P1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v18_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV180Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV180Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v18_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV181Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV181Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_vdd_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcVdd0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcVdd0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_vdd_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcVdd1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcVdd1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl0Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl0Reg_SPEC {
    type DataType = u16;
}

pub type DcdcCtrl0Reg = crate::RegValueT<DcdcCtrl0Reg_SPEC>;

impl DcdcCtrl0Reg {
    #[inline(always)]
    pub fn dcdc_fast_startup(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_brownout_lv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, DcdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,DcdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_idle_clk_div(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, DcdcCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,DcdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_priority(
        self,
    ) -> crate::common::RegisterField<3, 0xff, 1, 0, u8, u8, DcdcCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xff,1,0,u8,u8,DcdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_fw_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, DcdcCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,DcdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl0Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl0Reg {
        <crate::RegValueT<DcdcCtrl0Reg_SPEC> as RegisterValue<_>>::new(12068)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type DcdcCtrl1Reg = crate::RegValueT<DcdcCtrl1Reg_SPEC>;

impl DcdcCtrl1Reg {
    #[inline(always)]
    pub fn dcdc_startup_delay(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1f,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_global_max_idle_time(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_timeout(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl1Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl1Reg {
        <crate::RegValueT<DcdcCtrl1Reg_SPEC> as RegisterValue<_>>::new(21520)
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
    pub fn dcdc_timeout_irq_trig(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_timeout_irq_res(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_tune(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_lssup_trim(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_hsgnd_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl2Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl2Reg {
        <crate::RegValueT<DcdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(34861)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcIrqClearReg_SPEC;
impl crate::sealed::RegSpec for DcdcIrqClearReg_SPEC {
    type DataType = u16;
}

pub type DcdcIrqClearReg = crate::RegValueT<DcdcIrqClearReg_SPEC>;

impl DcdcIrqClearReg {
    #[inline(always)]
    pub fn dcdc_brown_out_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_timeout_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_timeout_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_timeout_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_timeout_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcIrqClearReg {
    #[inline(always)]
    fn default() -> DcdcIrqClearReg {
        <crate::RegValueT<DcdcIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for DcdcIrqMaskReg_SPEC {
    type DataType = u16;
}

pub type DcdcIrqMaskReg = crate::RegValueT<DcdcIrqMaskReg_SPEC>;

impl DcdcIrqMaskReg {
    #[inline(always)]
    pub fn dcdc_brown_out_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_timeout_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_timeout_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_timeout_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_timeout_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcIrqMaskReg {
    #[inline(always)]
    fn default() -> DcdcIrqMaskReg {
        <crate::RegValueT<DcdcIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for DcdcIrqStatusReg_SPEC {
    type DataType = u16;
}

pub type DcdcIrqStatusReg = crate::RegValueT<DcdcIrqStatusReg_SPEC>;

impl DcdcIrqStatusReg {
    #[inline(always)]
    pub fn dcdc_brown_out_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_timeout_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_timeout_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_timeout_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_timeout_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcIrqStatusReg {
    #[inline(always)]
    fn default() -> DcdcIrqStatusReg {
        <crate::RegValueT<DcdcIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcRet0Reg_SPEC;
impl crate::sealed::RegSpec for DcdcRet0Reg_SPEC {
    type DataType = u16;
}

pub type DcdcRet0Reg = crate::RegValueT<DcdcRet0Reg_SPEC>;

impl DcdcRet0Reg {
    #[inline(always)]
    pub fn dcdc_v18p_ret_cycles(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, DcdcRet0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,DcdcRet0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_cur_lim_ret(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, DcdcRet0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,DcdcRet0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_ret_cycles(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DcdcRet0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DcdcRet0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim_ret(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcRet0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcRet0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcRet0Reg {
    #[inline(always)]
    fn default() -> DcdcRet0Reg {
        <crate::RegValueT<DcdcRet0Reg_SPEC> as RegisterValue<_>>::new(43686)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcRet1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcRet1Reg_SPEC {
    type DataType = u16;
}

pub type DcdcRet1Reg = crate::RegValueT<DcdcRet1Reg_SPEC>;

impl DcdcRet1Reg {
    #[inline(always)]
    pub fn dcdc_v18_ret_cycles(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, DcdcRet1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,DcdcRet1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim_ret(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, DcdcRet1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,DcdcRet1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_ret_cycles(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DcdcRet1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DcdcRet1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim_ret(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcRet1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcRet1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcRet1Reg {
    #[inline(always)]
    fn default() -> DcdcRet1Reg {
        <crate::RegValueT<DcdcRet1Reg_SPEC> as RegisterValue<_>>::new(43590)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus0Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus0Reg_SPEC {
    type DataType = u16;
}

pub type DcdcStatus0Reg = crate::RegValueT<DcdcStatus0Reg_SPEC>;

impl DcdcStatus0Reg {
    #[inline(always)]
    pub fn dcdc_charge_reg_3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, DcdcStatus0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,DcdcStatus0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_charge_reg_2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, DcdcStatus0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,DcdcStatus0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_charge_reg_1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, DcdcStatus0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,DcdcStatus0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_charge_reg_0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, DcdcStatus0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,DcdcStatus0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus0Reg {
    #[inline(always)]
    fn default() -> DcdcStatus0Reg {
        <crate::RegValueT<DcdcStatus0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus1Reg_SPEC {
    type DataType = u16;
}

pub type DcdcStatus1Reg = crate::RegValueT<DcdcStatus1Reg_SPEC>;

impl DcdcStatus1Reg {
    #[inline(always)]
    pub fn dcdc_v18p_available(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_available(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_available(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_available(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_ok(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_nok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_nok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_nok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_nok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus1Reg {
    #[inline(always)]
    fn default() -> DcdcStatus1Reg {
        <crate::RegValueT<DcdcStatus1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus2Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus2Reg_SPEC {
    type DataType = u16;
}

pub type DcdcStatus2Reg = crate::RegValueT<DcdcStatus2Reg_SPEC>;

impl DcdcStatus2Reg {
    #[inline(always)]
    pub fn dcdc_v18p_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_nsw_state(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_psw_state(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_p_comp_p(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_p_comp_n(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_p(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_n(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_p_comp(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcStatus2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus2Reg {
    #[inline(always)]
    fn default() -> DcdcStatus2Reg {
        <crate::RegValueT<DcdcStatus2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus3Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus3Reg_SPEC {
    type DataType = u16;
}

pub type DcdcStatus3Reg = crate::RegValueT<DcdcStatus3Reg_SPEC>;

impl DcdcStatus3Reg {
    #[inline(always)]
    pub fn dcdc_lv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, DcdcStatus3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,DcdcStatus3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_i_lim_v18p(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcStatus3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcStatus3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_i_lim_vdd(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcStatus3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcStatus3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus3Reg {
    #[inline(always)]
    fn default() -> DcdcStatus3Reg {
        <crate::RegValueT<DcdcStatus3Reg_SPEC> as RegisterValue<_>>::new(132)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus4Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus4Reg_SPEC {
    type DataType = u16;
}

pub type DcdcStatus4Reg = crate::RegValueT<DcdcStatus4Reg_SPEC>;

impl DcdcStatus4Reg {
    #[inline(always)]
    pub fn dcdc_i_lim_v18(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcStatus4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcStatus4Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_i_lim_v14(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcStatus4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcStatus4Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus4Reg {
    #[inline(always)]
    fn default() -> DcdcStatus4Reg {
        <crate::RegValueT<DcdcStatus4Reg_SPEC> as RegisterValue<_>>::new(132)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTest0Reg_SPEC;
impl crate::sealed::RegSpec for DcdcTest0Reg_SPEC {
    type DataType = u16;
}

pub type DcdcTest0Reg = crate::RegValueT<DcdcTest0Reg_SPEC>;

impl DcdcTest0Reg {
    #[inline(always)]
    pub fn dcdc_force_comp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_current(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_output_monitor(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, DcdcTest0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_ana_test(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, DcdcTest0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_idle(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_vdd(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_v18(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_v14(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_fw(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_nsw(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_psw(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcTest0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcTest0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTest0Reg {
    #[inline(always)]
    fn default() -> DcdcTest0Reg {
        <crate::RegValueT<DcdcTest0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTest1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcTest1Reg_SPEC {
    type DataType = u16;
}

pub type DcdcTest1Reg = crate::RegValueT<DcdcTest1Reg_SPEC>;

impl DcdcTest1Reg {
    #[inline(always)]
    pub fn dcdc_comp_clk(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, DcdcTest1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xf,1,0,u8,u8,DcdcTest1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_test_current(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, u8, DcdcTest1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1f,1,0,u8,u8,DcdcTest1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_test_reg(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, DcdcTest1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,DcdcTest1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTest1Reg {
    #[inline(always)]
    fn default() -> DcdcTest1Reg {
        <crate::RegValueT<DcdcTest1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTrim0Reg_SPEC;
impl crate::sealed::RegSpec for DcdcTrim0Reg_SPEC {
    type DataType = u16;
}

pub type DcdcTrim0Reg = crate::RegValueT<DcdcTrim0Reg_SPEC>;

impl DcdcTrim0Reg {
    #[inline(always)]
    pub fn dcdc_v14_trim_p(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, DcdcTrim0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,DcdcTrim0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_trim_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DcdcTrim0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DcdcTrim0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTrim0Reg {
    #[inline(always)]
    fn default() -> DcdcTrim0Reg {
        <crate::RegValueT<DcdcTrim0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTrim1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcTrim1Reg_SPEC {
    type DataType = u16;
}

pub type DcdcTrim1Reg = crate::RegValueT<DcdcTrim1Reg_SPEC>;

impl DcdcTrim1Reg {
    #[inline(always)]
    pub fn dcdc_v18_trim_p(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, DcdcTrim1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,DcdcTrim1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_trim_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DcdcTrim1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DcdcTrim1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTrim1Reg {
    #[inline(always)]
    fn default() -> DcdcTrim1Reg {
        <crate::RegValueT<DcdcTrim1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTrim2Reg_SPEC;
impl crate::sealed::RegSpec for DcdcTrim2Reg_SPEC {
    type DataType = u16;
}

pub type DcdcTrim2Reg = crate::RegValueT<DcdcTrim2Reg_SPEC>;

impl DcdcTrim2Reg {
    #[inline(always)]
    pub fn dcdc_vdd_trim_p(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, DcdcTrim2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,DcdcTrim2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_trim_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DcdcTrim2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DcdcTrim2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTrim2Reg {
    #[inline(always)]
    fn default() -> DcdcTrim2Reg {
        <crate::RegValueT<DcdcTrim2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTrim3Reg_SPEC;
impl crate::sealed::RegSpec for DcdcTrim3Reg_SPEC {
    type DataType = u16;
}

pub type DcdcTrim3Reg = crate::RegValueT<DcdcTrim3Reg_SPEC>;

impl DcdcTrim3Reg {
    #[inline(always)]
    pub fn dcdc_v18p_trim_p(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, DcdcTrim3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,DcdcTrim3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_trim_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DcdcTrim3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DcdcTrim3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTrim3Reg {
    #[inline(always)]
    fn default() -> DcdcTrim3Reg {
        <crate::RegValueT<DcdcTrim3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTrimReg_SPEC;
impl crate::sealed::RegSpec for DcdcTrimReg_SPEC {
    type DataType = u16;
}

pub type DcdcTrimReg = crate::RegValueT<DcdcTrimReg_SPEC>;

impl DcdcTrimReg {
    #[inline(always)]
    pub fn dcdc_p_comp_man_trim(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, DcdcTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,DcdcTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_p_comp_trim(
        self,
    ) -> crate::common::RegisterField<7, 0x3f, 1, 0, u8, u8, DcdcTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3f,1,0,u8,u8,DcdcTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_man_trim(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DcdcTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,DcdcTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DcdcTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DcdcTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTrimReg {
    #[inline(always)]
    fn default() -> DcdcTrimReg {
        <crate::RegValueT<DcdcTrimReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV140Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV140Reg_SPEC {
    type DataType = u16;
}

pub type DcdcV140Reg = crate::RegValueT<DcdcV140Reg_SPEC>;

impl DcdcV140Reg {
    #[inline(always)]
    pub fn dcdc_v14_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcV140Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcV140Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_voltage(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, DcdcV140Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,DcdcV140Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcV140Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcV140Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcV140Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcV140Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV140Reg {
    #[inline(always)]
    fn default() -> DcdcV140Reg {
        <crate::RegValueT<DcdcV140Reg_SPEC> as RegisterValue<_>>::new(41380)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV141Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV141Reg_SPEC {
    type DataType = u16;
}

pub type DcdcV141Reg = crate::RegValueT<DcdcV141Reg_SPEC>;

impl DcdcV141Reg {
    #[inline(always)]
    pub fn dcdc_v14_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcV141Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcV141Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcV141Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcV141Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, DcdcV141Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,DcdcV141Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_idle_hyst(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcV141Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcV141Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_idle_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcV141Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcV141Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV141Reg {
    #[inline(always)]
    fn default() -> DcdcV141Reg {
        <crate::RegValueT<DcdcV141Reg_SPEC> as RegisterValue<_>>::new(55440)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV18P0Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV18P0Reg_SPEC {
    type DataType = u16;
}

pub type DcdcV18P0Reg = crate::RegValueT<DcdcV18P0Reg_SPEC>;

impl DcdcV18P0Reg {
    #[inline(always)]
    pub fn dcdc_v18p_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcV18P0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcV18P0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_voltage(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, DcdcV18P0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,DcdcV18P0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcV18P0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcV18P0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcV18P0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcV18P0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV18P0Reg {
    #[inline(always)]
    fn default() -> DcdcV18P0Reg {
        <crate::RegValueT<DcdcV18P0Reg_SPEC> as RegisterValue<_>>::new(58340)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV18P1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV18P1Reg_SPEC {
    type DataType = u16;
}

pub type DcdcV18P1Reg = crate::RegValueT<DcdcV18P1Reg_SPEC>;

impl DcdcV18P1Reg {
    #[inline(always)]
    pub fn dcdc_v18p_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcV18P1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcV18P1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcV18P1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcV18P1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, DcdcV18P1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,DcdcV18P1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_idle_hyst(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcV18P1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcV18P1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_idle_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcV18P1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcV18P1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV18P1Reg {
    #[inline(always)]
    fn default() -> DcdcV18P1Reg {
        <crate::RegValueT<DcdcV18P1Reg_SPEC> as RegisterValue<_>>::new(48272)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV180Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV180Reg_SPEC {
    type DataType = u16;
}

pub type DcdcV180Reg = crate::RegValueT<DcdcV180Reg_SPEC>;

impl DcdcV180Reg {
    #[inline(always)]
    pub fn dcdc_v18_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcV180Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcV180Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_voltage(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, DcdcV180Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,DcdcV180Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcV180Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcV180Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcV180Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcV180Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV180Reg {
    #[inline(always)]
    fn default() -> DcdcV180Reg {
        <crate::RegValueT<DcdcV180Reg_SPEC> as RegisterValue<_>>::new(58340)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV181Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV181Reg_SPEC {
    type DataType = u16;
}

pub type DcdcV181Reg = crate::RegValueT<DcdcV181Reg_SPEC>;

impl DcdcV181Reg {
    #[inline(always)]
    pub fn dcdc_v18_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcV181Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcV181Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcV181Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcV181Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, DcdcV181Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,DcdcV181Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_idle_hyst(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcV181Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcV181Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_idle_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcV181Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcV181Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV181Reg {
    #[inline(always)]
    fn default() -> DcdcV181Reg {
        <crate::RegValueT<DcdcV181Reg_SPEC> as RegisterValue<_>>::new(48272)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcVdd0Reg_SPEC;
impl crate::sealed::RegSpec for DcdcVdd0Reg_SPEC {
    type DataType = u16;
}

pub type DcdcVdd0Reg = crate::RegValueT<DcdcVdd0Reg_SPEC>;

impl DcdcVdd0Reg {
    #[inline(always)]
    pub fn dcdc_vdd_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcVdd0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcVdd0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_voltage(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, DcdcVdd0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,DcdcVdd0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcVdd0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcVdd0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcVdd0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcVdd0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcVdd0Reg {
    #[inline(always)]
    fn default() -> DcdcVdd0Reg {
        <crate::RegValueT<DcdcVdd0Reg_SPEC> as RegisterValue<_>>::new(49924)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcVdd1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcVdd1Reg_SPEC {
    type DataType = u16;
}

pub type DcdcVdd1Reg = crate::RegValueT<DcdcVdd1Reg_SPEC>;

impl DcdcVdd1Reg {
    #[inline(always)]
    pub fn dcdc_vdd_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcVdd1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcVdd1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcVdd1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcVdd1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, DcdcVdd1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,DcdcVdd1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_idle_hyst(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcVdd1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcVdd1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_idle_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcVdd1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcVdd1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcVdd1Reg {
    #[inline(always)]
    fn default() -> DcdcVdd1Reg {
        <crate::RegValueT<DcdcVdd1Reg_SPEC> as RegisterValue<_>>::new(60560)
    }
}
