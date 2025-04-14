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
#[doc = r"CRG_XTAL registers"]
unsafe impl ::core::marker::Send for super::CrgXtal {}
unsafe impl ::core::marker::Sync for super::CrgXtal {}
impl super::CrgXtal {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn pll_sys_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_usb_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllUsbCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_usb_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllUsbCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllUsbCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_usb_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllUsbCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllUsbCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_usb_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllUsbStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllUsbStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_sys_irq_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetSysIrqCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetSysIrqCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn set_sys_irq_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetSysIrqCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetSysIrqCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sys_irq_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SysIrqCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SysIrqCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_cap_meas_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MCapMeasReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MCapMeasReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_fsm_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MFsmReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MFsmReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_irq_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MIrqCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MIrqCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_irq_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MIrqStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MIrqStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_settle_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MSettleReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MSettleReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_stat0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MStat0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MStat0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type PllSysCtrl1Reg = crate::RegValueT<PllSysCtrl1Reg_SPEC>;

impl PllSysCtrl1Reg {
    #[inline(always)]
    pub fn pll_out_div(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_sel_min_cur_int(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_pre_div(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_n_div(
        self,
    ) -> crate::common::RegisterField<4, 0x7f, 1, 0, u8, u8, PllSysCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7f,1,0,u8,u8,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_pll_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_pll_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_rst_n(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl1Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl1Reg {
        <crate::RegValueT<PllSysCtrl1Reg_SPEC> as RegisterValue<_>>::new(59552)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type PllSysCtrl2Reg = crate::RegValueT<PllSysCtrl2Reg_SPEC>;

impl PllSysCtrl2Reg {
    #[inline(always)]
    pub fn pll_recalib(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllSysCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PllSysCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl2Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl2Reg {
        <crate::RegValueT<PllSysCtrl2Reg_SPEC> as RegisterValue<_>>::new(3200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl3Reg_SPEC {
    type DataType = u32;
}

pub type PllSysCtrl3Reg = crate::RegValueT<PllSysCtrl3Reg_SPEC>;

impl PllSysCtrl3Reg {
    #[inline(always)]
    pub fn pll_min_current(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, PllSysCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,PllSysCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl3Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl3Reg {
        <crate::RegValueT<PllSysCtrl3Reg_SPEC> as RegisterValue<_>>::new(39024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysStatusReg_SPEC;
impl crate::sealed::RegSpec for PllSysStatusReg_SPEC {
    type DataType = u32;
}

pub type PllSysStatusReg = crate::RegValueT<PllSysStatusReg_SPEC>;

impl PllSysStatusReg {
    #[inline(always)]
    pub fn ldo_pll_ok(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_calibration_end(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_best_min_cur(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, u8, PllSysStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_lock_fine(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysStatusReg {
    #[inline(always)]
    fn default() -> PllSysStatusReg {
        <crate::RegValueT<PllSysStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsbCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for PllUsbCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type PllUsbCtrl1Reg = crate::RegValueT<PllUsbCtrl1Reg_SPEC>;

impl PllUsbCtrl1Reg {
    #[inline(always)]
    pub fn pll_out_div(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_sel_min_cur_int(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_pre_div(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_n_div(
        self,
    ) -> crate::common::RegisterField<4, 0x7f, 1, 0, u8, u8, PllUsbCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7f,1,0,u8,u8,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_pll_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_pll_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_rst_n(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllUsbCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PllUsbCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllUsbCtrl1Reg {
    #[inline(always)]
    fn default() -> PllUsbCtrl1Reg {
        <crate::RegValueT<PllUsbCtrl1Reg_SPEC> as RegisterValue<_>>::new(26768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsbCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for PllUsbCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type PllUsbCtrl2Reg = crate::RegValueT<PllUsbCtrl2Reg_SPEC>;

impl PllUsbCtrl2Reg {
    #[inline(always)]
    pub fn pll_recalib(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllUsbCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PllUsbCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllUsbCtrl2Reg {
    #[inline(always)]
    fn default() -> PllUsbCtrl2Reg {
        <crate::RegValueT<PllUsbCtrl2Reg_SPEC> as RegisterValue<_>>::new(3200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsbCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for PllUsbCtrl3Reg_SPEC {
    type DataType = u32;
}

pub type PllUsbCtrl3Reg = crate::RegValueT<PllUsbCtrl3Reg_SPEC>;

impl PllUsbCtrl3Reg {
    #[inline(always)]
    pub fn pll_min_current(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, PllUsbCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,PllUsbCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllUsbCtrl3Reg {
    #[inline(always)]
    fn default() -> PllUsbCtrl3Reg {
        <crate::RegValueT<PllUsbCtrl3Reg_SPEC> as RegisterValue<_>>::new(39024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsbStatusReg_SPEC;
impl crate::sealed::RegSpec for PllUsbStatusReg_SPEC {
    type DataType = u32;
}

pub type PllUsbStatusReg = crate::RegValueT<PllUsbStatusReg_SPEC>;

impl PllUsbStatusReg {
    #[inline(always)]
    pub fn ldo_pll_ok(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllUsbStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,PllUsbStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_calibration_end(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllUsbStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,PllUsbStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_best_min_cur(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, u8, PllUsbStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,PllUsbStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_lock_fine(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllUsbStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,PllUsbStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PllUsbStatusReg {
    #[inline(always)]
    fn default() -> PllUsbStatusReg {
        <crate::RegValueT<PllUsbStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetSysIrqCtrlReg_SPEC;
impl crate::sealed::RegSpec for ResetSysIrqCtrlReg_SPEC {
    type DataType = u32;
}

pub type ResetSysIrqCtrlReg = crate::RegValueT<ResetSysIrqCtrlReg_SPEC>;

impl ResetSysIrqCtrlReg {
    #[inline(always)]
    pub fn cmac2snc_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac2sys_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc2sys_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc2cmac_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys2snc_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys2cmac_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetSysIrqCtrlReg {
    #[inline(always)]
    fn default() -> ResetSysIrqCtrlReg {
        <crate::RegValueT<ResetSysIrqCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetSysIrqCtrlReg_SPEC;
impl crate::sealed::RegSpec for SetSysIrqCtrlReg_SPEC {
    type DataType = u32;
}

pub type SetSysIrqCtrlReg = crate::RegValueT<SetSysIrqCtrlReg_SPEC>;

impl SetSysIrqCtrlReg {
    #[inline(always)]
    pub fn cmac2snc_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac2sys_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc2sys_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc2cmac_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys2snc_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys2cmac_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetSysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetSysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetSysIrqCtrlReg {
    #[inline(always)]
    fn default() -> SetSysIrqCtrlReg {
        <crate::RegValueT<SetSysIrqCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysIrqCtrlReg_SPEC;
impl crate::sealed::RegSpec for SysIrqCtrlReg_SPEC {
    type DataType = u32;
}

pub type SysIrqCtrlReg = crate::RegValueT<SysIrqCtrlReg_SPEC>;

impl SysIrqCtrlReg {
    #[inline(always)]
    pub fn cmac2snc_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac2sys_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc2sys_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc2cmac_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys2snc_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys2cmac_irq_bit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SysIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SysIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SysIrqCtrlReg {
    #[inline(always)]
    fn default() -> SysIrqCtrlReg {
        <crate::RegValueT<SysIrqCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MCapMeasReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MCapMeasReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MCapMeasReg = crate::RegValueT<Xtal32MCapMeasReg_SPEC>;

impl Xtal32MCapMeasReg {
    #[inline(always)]
    pub fn xtal32m_meas_time(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, Xtal32MCapMeasReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            u8,
            u8,
            Xtal32MCapMeasReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_meas_start(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Xtal32MCapMeasReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Xtal32MCapMeasReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_meas_cur(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, Xtal32MCapMeasReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x3,
            1,
            0,
            u8,
            u8,
            Xtal32MCapMeasReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_cap_select(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Xtal32MCapMeasReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            Xtal32MCapMeasReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xtal32MCapMeasReg {
    #[inline(always)]
    fn default() -> Xtal32MCapMeasReg {
        <crate::RegValueT<Xtal32MCapMeasReg_SPEC> as RegisterValue<_>>::new(144)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MCtrlReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MCtrlReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MCtrlReg = crate::RegValueT<Xtal32MCtrlReg_SPEC>;

impl Xtal32MCtrlReg {
    #[inline(always)]
    pub fn xtal32m_drive_cycles(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, Xtal32MCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,Xtal32MCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Xtal32MCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Xtal32MCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_biasprot(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Xtal32MCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,Xtal32MCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_ldo_sah(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Xtal32MCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Xtal32MCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_ampreg_sah(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Xtal32MCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Xtal32MCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_bias_sah(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Xtal32MCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Xtal32MCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MCtrlReg {
    #[inline(always)]
    fn default() -> Xtal32MCtrlReg {
        <crate::RegValueT<Xtal32MCtrlReg_SPEC> as RegisterValue<_>>::new(1045)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MFsmReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MFsmReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MFsmReg = crate::RegValueT<Xtal32MFsmReg_SPEC>;

impl Xtal32MFsmReg {
    #[inline(always)]
    pub fn xtal32m_boost_mode(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Xtal32MFsmReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Xtal32MFsmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_fsm_apply_config(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Xtal32MFsmReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Xtal32MFsmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_fsm_force_idle(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Xtal32MFsmReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Xtal32MFsmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_mode(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Xtal32MFsmReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Xtal32MFsmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_trim_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Xtal32MFsmReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Xtal32MFsmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cur_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Xtal32MFsmReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Xtal32MFsmReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MFsmReg {
    #[inline(always)]
    fn default() -> Xtal32MFsmReg {
        <crate::RegValueT<Xtal32MFsmReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MIrqCtrlReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MIrqCtrlReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MIrqCtrlReg = crate::RegValueT<Xtal32MIrqCtrlReg_SPEC>;

impl Xtal32MIrqCtrlReg {
    #[inline(always)]
    pub fn xtal32m_irq_cap_ctrl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        u8,
        u8,
        Xtal32MIrqCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            u8,
            u8,
            Xtal32MIrqCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_irq_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Xtal32MIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Xtal32MIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_irq_clk(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Xtal32MIrqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Xtal32MIrqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_irq_cnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Xtal32MIrqCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Xtal32MIrqCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xtal32MIrqCtrlReg {
    #[inline(always)]
    fn default() -> Xtal32MIrqCtrlReg {
        <crate::RegValueT<Xtal32MIrqCtrlReg_SPEC> as RegisterValue<_>>::new(2303)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MIrqStatReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MIrqStatReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MIrqStatReg = crate::RegValueT<Xtal32MIrqStatReg_SPEC>;

impl Xtal32MIrqStatReg {
    #[inline(always)]
    pub fn xtal32m_irq_count_cap(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Xtal32MIrqStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            Xtal32MIrqStatReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_irq_count_stat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xtal32MIrqStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Xtal32MIrqStatReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xtal32MIrqStatReg {
    #[inline(always)]
    fn default() -> Xtal32MIrqStatReg {
        <crate::RegValueT<Xtal32MIrqStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MSettleReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MSettleReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MSettleReg = crate::RegValueT<Xtal32MSettleReg_SPEC>;

impl Xtal32MSettleReg {
    #[inline(always)]
    pub fn xtal32m_timeout(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x7f,
        1,
        0,
        u8,
        u8,
        Xtal32MSettleReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x7f,
            1,
            0,
            u8,
            u8,
            Xtal32MSettleReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_blank(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, u8, Xtal32MSettleReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            u8,
            u8,
            Xtal32MSettleReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_lvl(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, Xtal32MSettleReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            17,
            0x3,
            1,
            0,
            u8,
            u8,
            Xtal32MSettleReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_ampl_set(
        self,
    ) -> crate::common::RegisterField<14, 0x7, 1, 0, u8, u8, Xtal32MSettleReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            14,
            0x7,
            1,
            0,
            u8,
            u8,
            Xtal32MSettleReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_cur_set(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, Xtal32MSettleReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0xf,
            1,
            0,
            u8,
            u8,
            Xtal32MSettleReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_trim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        Xtal32MSettleReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            Xtal32MSettleReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xtal32MSettleReg {
    #[inline(always)]
    fn default() -> Xtal32MSettleReg {
        <crate::RegValueT<Xtal32MSettleReg_SPEC> as RegisterValue<_>>::new(1716524)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MStartReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MStartReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MStartReg = crate::RegValueT<Xtal32MStartReg_SPEC>;

impl Xtal32MStartReg {
    #[inline(always)]
    pub fn xtal32m_timeout(
        self,
    ) -> crate::common::RegisterField<22, 0x7f, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            22,
            0x7f,
            1,
            0,
            u8,
            u8,
            Xtal32MStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_blank(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x7,1,0,u8,u8,Xtal32MStartReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_lvl(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,Xtal32MStartReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_ampl_set(
        self,
    ) -> crate::common::RegisterField<14, 0x7, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x7,1,0,u8,u8,Xtal32MStartReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cur_set(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,Xtal32MStartReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_trim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        Xtal32MStartReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            Xtal32MStartReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xtal32MStartReg {
    #[inline(always)]
    fn default() -> Xtal32MStartReg {
        <crate::RegValueT<Xtal32MStartReg_SPEC> as RegisterValue<_>>::new(1847596)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MStat0Reg_SPEC;
impl crate::sealed::RegSpec for Xtal32MStat0Reg_SPEC {
    type DataType = u32;
}

pub type Xtal32MStat0Reg = crate::RegValueT<Xtal32MStat0Reg_SPEC>;

impl Xtal32MStat0Reg {
    #[inline(always)]
    pub fn xtal32m_overload(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Xtal32MStat0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_lvl_stat(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, u8, Xtal32MStat0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x3,1,0,u8,u8,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_ampl_trim(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, Xtal32MStat0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_trim_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3ff,
        1,
        0,
        u16,
        u16,
        Xtal32MStat0Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3ff,
            1,
            0,
            u16,
            u16,
            Xtal32MStat0Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xtal32m_cur_set_stat(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, Xtal32MStat0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_ldo_ok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Xtal32MStat0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_out(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, Xtal32MStat0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_state(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Xtal32MStat0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_out_hold(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Xtal32MStat0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_ready(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Xtal32MStat0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Xtal32MStat0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MStat0Reg {
    #[inline(always)]
    fn default() -> Xtal32MStat0Reg {
        <crate::RegValueT<Xtal32MStat0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MTrimReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MTrimReg_SPEC {
    type DataType = u32;
}

pub type Xtal32MTrimReg = crate::RegValueT<Xtal32MTrimReg_SPEC>;

impl Xtal32MTrimReg {
    #[inline(always)]
    pub fn xtal32m_boost_trim(
        self,
    ) -> crate::common::RegisterField<19, 0x3f, 1, 0, u8, u8, Xtal32MTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x3f,1,0,u8,u8,Xtal32MTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cmp_lvl(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, Xtal32MTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,Xtal32MTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_ampl_set(
        self,
    ) -> crate::common::RegisterField<14, 0x7, 1, 0, u8, u8, Xtal32MTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x7,1,0,u8,u8,Xtal32MTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cur_set(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, Xtal32MTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,Xtal32MTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_trim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        Xtal32MTrimReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            Xtal32MTrimReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xtal32MTrimReg {
    #[inline(always)]
    fn default() -> Xtal32MTrimReg {
        <crate::RegValueT<Xtal32MTrimReg_SPEC> as RegisterValue<_>>::new(138540)
    }
}
