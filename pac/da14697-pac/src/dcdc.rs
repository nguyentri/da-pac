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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:28 +0000

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
    pub const fn dcdc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
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
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_status4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcStatus4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcStatus4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_test_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcTestReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcTestReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v18p_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV18PReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV18PReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_v18_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcV18Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcV18Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcdc_vdd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcVddReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcVddReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type DcdcCtrl1Reg = crate::RegValueT<DcdcCtrl1Reg_SPEC>;

impl DcdcCtrl1Reg {
    #[inline(always)]
    pub fn dcdc_sh_enable(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, DcdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_startup_delay(
        self,
    ) -> crate::common::RegisterField<26, 0x1f, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1f,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_idle_max_fast_downramp(
        self,
    ) -> crate::common::RegisterField<20, 0x3f, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3f,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_sw_timeout(
        self,
    ) -> crate::common::RegisterField<15, 0x1f, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1f,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_fast_startup(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_man_lv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, DcdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_auto_lv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, DcdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_idle_clk_div(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_priority(
        self,
    ) -> crate::common::RegisterField<2, 0xff, 1, 0, u8, u8, DcdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xff,1,0,u8,u8,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_fw_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl1Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl1Reg {
        <crate::RegValueT<DcdcCtrl1Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type DcdcCtrl2Reg = crate::RegValueT<DcdcCtrl2Reg_SPEC>;

impl DcdcCtrl2Reg {
    #[inline(always)]
    pub fn dcdc_v_nok_cnt_max(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_trim_man(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, DcdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_trim_val(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    pub fn dcdc_slope_control(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vbtstrp_trim(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_lssup_trim(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_hsgnd_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl2Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl2Reg {
        <crate::RegValueT<DcdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(134777055)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcIrqClearReg_SPEC;
impl crate::sealed::RegSpec for DcdcIrqClearReg_SPEC {
    type DataType = u32;
}

pub type DcdcIrqClearReg = crate::RegValueT<DcdcIrqClearReg_SPEC>;

impl DcdcIrqClearReg {
    #[inline(always)]
    pub fn dcdc_low_vbat_irq_clear(
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
    type DataType = u32;
}

pub type DcdcIrqMaskReg = crate::RegValueT<DcdcIrqMaskReg_SPEC>;

impl DcdcIrqMaskReg {
    #[inline(always)]
    pub fn dcdc_low_vbat_irq_mask(
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
        <crate::RegValueT<DcdcIrqMaskReg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for DcdcIrqStatusReg_SPEC {
    type DataType = u32;
}

pub type DcdcIrqStatusReg = crate::RegValueT<DcdcIrqStatusReg_SPEC>;

impl DcdcIrqStatusReg {
    #[inline(always)]
    pub fn dcdc_low_vbat_irq_status(
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
pub struct DcdcStatus1Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus1Reg_SPEC {
    type DataType = u32;
}

pub type DcdcStatus1Reg = crate::RegValueT<DcdcStatus1Reg_SPEC>;

impl DcdcStatus1Reg {
    #[inline(always)]
    pub fn dcdc_v18p_available(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_available(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_available(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_available(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_comp_ok(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_comp_ok(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_comp_ok(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_comp_ok(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_comp_nok(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_comp_nok(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_comp_nok(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_comp_nok(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_p(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp_n(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_p_comp(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_comp(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_lv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_n_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_p_sw_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcStatus1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_startup_complete(
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
    type DataType = u32;
}

pub type DcdcStatus2Reg = crate::RegValueT<DcdcStatus2Reg_SPEC>;

impl DcdcStatus2Reg {
    #[inline(always)]
    pub fn dcdc_v18p_cur_lim(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, DcdcStatus2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, DcdcStatus2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, DcdcStatus2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, DcdcStatus2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,DcdcStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus2Reg {
    #[inline(always)]
    fn default() -> DcdcStatus2Reg {
        <crate::RegValueT<DcdcStatus2Reg_SPEC> as RegisterValue<_>>::new(8650884)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus3Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus3Reg_SPEC {
    type DataType = u32;
}

pub type DcdcStatus3Reg = crate::RegValueT<DcdcStatus3Reg_SPEC>;

impl DcdcStatus3Reg {
    #[inline(always)]
    pub fn dcdc_v18p_n_comp_trim(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, u8, DcdcStatus3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x3f,1,0,u8,u8,DcdcStatus3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_n_comp_trim(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, DcdcStatus3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,DcdcStatus3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_n_comp_trim(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, DcdcStatus3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,DcdcStatus3Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_n_comp_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DcdcStatus3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DcdcStatus3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus3Reg {
    #[inline(always)]
    fn default() -> DcdcStatus3Reg {
        <crate::RegValueT<DcdcStatus3Reg_SPEC> as RegisterValue<_>>::new(34079240)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcStatus4Reg_SPEC;
impl crate::sealed::RegSpec for DcdcStatus4Reg_SPEC {
    type DataType = u32;
}

pub type DcdcStatus4Reg = crate::RegValueT<DcdcStatus4Reg_SPEC>;

impl DcdcStatus4Reg {
    #[inline(always)]
    pub fn dcdc_charge_reg_3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, DcdcStatus4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,DcdcStatus4Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_charge_reg_2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, DcdcStatus4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,DcdcStatus4Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_charge_reg_1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, DcdcStatus4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,DcdcStatus4Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_charge_reg_0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, DcdcStatus4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,DcdcStatus4Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcStatus4Reg {
    #[inline(always)]
    fn default() -> DcdcStatus4Reg {
        <crate::RegValueT<DcdcStatus4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcTestReg_SPEC;
impl crate::sealed::RegSpec for DcdcTestReg_SPEC {
    type DataType = u32;
}

pub type DcdcTestReg = crate::RegValueT<DcdcTestReg_SPEC>;

impl DcdcTestReg {
    #[inline(always)]
    pub fn dcdc_test_out(
        self,
    ) -> crate::common::RegisterField<25, 0xf, 1, 0, u8, u8, DcdcTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0xf,1,0,u8,u8,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_cur_lim_val(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, u8, DcdcTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1f,1,0,u8,u8,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_comp_clk_val(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, DcdcTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_cur_lim(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_comp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_output_mon(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, DcdcTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_ana_test(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, DcdcTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_idle(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_v18p_sw(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_vdd_sw(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_v18_sw(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_v14_sw(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_fw_sw(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_n_sw(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_force_p_sw(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcTestReg {
    #[inline(always)]
    fn default() -> DcdcTestReg {
        <crate::RegValueT<DcdcTestReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV14Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV14Reg_SPEC {
    type DataType = u32;
}

pub type DcdcV14Reg = crate::RegValueT<DcdcV14Reg_SPEC>;

impl DcdcV14Reg {
    #[inline(always)]
    pub fn dcdc_v14_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, DcdcV14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_trim(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, DcdcV14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<22, 0x1f, 1, 0, u8, u8, DcdcV14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1f,1,0,u8,u8,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<17, 0x1f, 1, 0, u8, u8, DcdcV14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1f,1,0,u8,u8,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, DcdcV14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_idle_hyst(
        self,
    ) -> crate::common::RegisterField<7, 0x1f, 1, 0, u8, u8, DcdcV14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1f,1,0,u8,u8,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_idle_min(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, DcdcV14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcV14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcV14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcV14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV14Reg {
    #[inline(always)]
    fn default() -> DcdcV14Reg {
        <crate::RegValueT<DcdcV14Reg_SPEC> as RegisterValue<_>>::new(55329347)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV18PReg_SPEC;
impl crate::sealed::RegSpec for DcdcV18PReg_SPEC {
    type DataType = u32;
}

pub type DcdcV18PReg = crate::RegValueT<DcdcV18PReg_SPEC>;

impl DcdcV18PReg {
    #[inline(always)]
    pub fn dcdc_v18p_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, DcdcV18PReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_trim(
        self,
    ) -> crate::common::RegisterField<27, 0xf, 1, 0, u8, u8, DcdcV18PReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0xf,1,0,u8,u8,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<22, 0x1f, 1, 0, u8, u8, DcdcV18PReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1f,1,0,u8,u8,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<17, 0x1f, 1, 0, u8, u8, DcdcV18PReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1f,1,0,u8,u8,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, DcdcV18PReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_idle_hyst(
        self,
    ) -> crate::common::RegisterField<7, 0x1f, 1, 0, u8, u8, DcdcV18PReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1f,1,0,u8,u8,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_idle_min(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, DcdcV18PReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcV18PReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcV18PReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcV18PReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV18PReg {
    #[inline(always)]
    fn default() -> DcdcV18PReg {
        <crate::RegValueT<DcdcV18PReg_SPEC> as RegisterValue<_>>::new(1207845442)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcV18Reg_SPEC;
impl crate::sealed::RegSpec for DcdcV18Reg_SPEC {
    type DataType = u32;
}

pub type DcdcV18Reg = crate::RegValueT<DcdcV18Reg_SPEC>;

impl DcdcV18Reg {
    #[inline(always)]
    pub fn dcdc_v18_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, DcdcV18Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_trim(
        self,
    ) -> crate::common::RegisterField<27, 0xf, 1, 0, u8, u8, DcdcV18Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0xf,1,0,u8,u8,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<22, 0x1f, 1, 0, u8, u8, DcdcV18Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1f,1,0,u8,u8,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<17, 0x1f, 1, 0, u8, u8, DcdcV18Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1f,1,0,u8,u8,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, DcdcV18Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_idle_hyst(
        self,
    ) -> crate::common::RegisterField<7, 0x1f, 1, 0, u8, u8, DcdcV18Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1f,1,0,u8,u8,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_idle_min(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, DcdcV18Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcV18Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcV18Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcV18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcV18Reg {
    #[inline(always)]
    fn default() -> DcdcV18Reg {
        <crate::RegValueT<DcdcV18Reg_SPEC> as RegisterValue<_>>::new(1207845442)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcVddReg_SPEC;
impl crate::sealed::RegSpec for DcdcVddReg_SPEC {
    type DataType = u32;
}

pub type DcdcVddReg = crate::RegValueT<DcdcVddReg_SPEC>;

impl DcdcVddReg {
    #[inline(always)]
    pub fn dcdc_vdd_fast_ramping(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, DcdcVddReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_trim(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, u8, u8, DcdcVddReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x7,1,0,u8,u8,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim_max_hv(
        self,
    ) -> crate::common::RegisterField<22, 0x1f, 1, 0, u8, u8, DcdcVddReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1f,1,0,u8,u8,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim_max_lv(
        self,
    ) -> crate::common::RegisterField<17, 0x1f, 1, 0, u8, u8, DcdcVddReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1f,1,0,u8,u8,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, DcdcVddReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_idle_hyst(
        self,
    ) -> crate::common::RegisterField<7, 0x1f, 1, 0, u8, u8, DcdcVddReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1f,1,0,u8,u8,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_idle_min(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, DcdcVddReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_enable_hv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DcdcVddReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vdd_enable_lv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcVddReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcVddReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcVddReg {
    #[inline(always)]
    fn default() -> DcdcVddReg {
        <crate::RegValueT<DcdcVddReg_SPEC> as RegisterValue<_>>::new(639255107)
    }
}
