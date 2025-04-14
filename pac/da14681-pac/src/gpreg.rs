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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"GPREG registers"]
unsafe impl ::core::marker::Send for super::Gpreg {}
unsafe impl ::core::marker::Sync for super::Gpreg {}
impl super::Gpreg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn debug_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DebugReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DebugReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecc_base_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccBaseAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccBaseAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn led_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LedControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LedControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pll_sys_test_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysTestReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysTestReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn set_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugReg_SPEC;
impl crate::sealed::RegSpec for DebugReg_SPEC {
    type DataType = u16;
}

pub type DebugReg = crate::RegValueT<DebugReg_SPEC>;

impl DebugReg {
    #[inline(always)]
    pub fn debugs_freeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for DebugReg {
    #[inline(always)]
    fn default() -> DebugReg {
        <crate::RegValueT<DebugReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccBaseAddrReg_SPEC;
impl crate::sealed::RegSpec for EccBaseAddrReg_SPEC {
    type DataType = u16;
}

pub type EccBaseAddrReg = crate::RegValueT<EccBaseAddrReg_SPEC>;

impl EccBaseAddrReg {
    #[inline(always)]
    pub fn ecc_base_addr(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, EccBaseAddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,EccBaseAddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccBaseAddrReg {
    #[inline(always)]
    fn default() -> EccBaseAddrReg {
        <crate::RegValueT<EccBaseAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpControlReg_SPEC;
impl crate::sealed::RegSpec for GpControlReg_SPEC {
    type DataType = u16;
}

pub type GpControlReg = crate::RegValueT<GpControlReg_SPEC>;

impl GpControlReg {
    #[inline(always)]
    pub fn ble_deepsldur_monitor(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GpControlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GpControlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_wakeup_lp_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, GpControlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,GpControlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_h2h_bridge_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_wakeup_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpControlReg {
    #[inline(always)]
    fn default() -> GpControlReg {
        <crate::RegValueT<GpControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpStatusReg_SPEC;
impl crate::sealed::RegSpec for GpStatusReg_SPEC {
    type DataType = u16;
}

pub type GpStatusReg = crate::RegValueT<GpStatusReg_SPEC>;

impl GpStatusReg {
    #[inline(always)]
    pub fn cal_phase(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpStatusReg {
    #[inline(always)]
    fn default() -> GpStatusReg {
        <crate::RegValueT<GpStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LedControlReg_SPEC;
impl crate::sealed::RegSpec for LedControlReg_SPEC {
    type DataType = u16;
}

pub type LedControlReg = crate::RegValueT<LedControlReg_SPEC>;

impl LedControlReg {
    #[inline(always)]
    pub fn led3_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led2_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led1_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led3_src_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led2_src_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led1_src_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LedControlReg {
    #[inline(always)]
    fn default() -> LedControlReg {
        <crate::RegValueT<LedControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type PllSysCtrl1Reg = crate::RegValueT<PllSysCtrl1Reg_SPEC>;

impl PllSysCtrl1Reg {
    #[inline(always)]
    pub fn pll_r_div(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, PllSysCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_pll_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_pll_enable(
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
        <crate::RegValueT<PllSysCtrl1Reg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type PllSysCtrl2Reg = crate::RegValueT<PllSysCtrl2Reg_SPEC>;

impl PllSysCtrl2Reg {
    #[inline(always)]
    pub fn pll_sel_min_cur_int(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PllSysCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PllSysCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_del_sel(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, PllSysCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,PllSysCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_n_div(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, PllSysCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,PllSysCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl2Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl2Reg {
        <crate::RegValueT<PllSysCtrl2Reg_SPEC> as RegisterValue<_>>::new(8198)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl3Reg_SPEC {
    type DataType = u16;
}

pub type PllSysCtrl3Reg = crate::RegValueT<PllSysCtrl3Reg_SPEC>;

impl PllSysCtrl3Reg {
    #[inline(always)]
    pub fn pll_recalib(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllSysCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PllSysCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_start_del(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, PllSysCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,PllSysCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_icp_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, PllSysCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,PllSysCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl3Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl3Reg {
        <crate::RegValueT<PllSysCtrl3Reg_SPEC> as RegisterValue<_>>::new(15369)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysStatusReg_SPEC;
impl crate::sealed::RegSpec for PllSysStatusReg_SPEC {
    type DataType = u16;
}

pub type PllSysStatusReg = crate::RegValueT<PllSysStatusReg_SPEC>;

impl PllSysStatusReg {
    #[inline(always)]
    pub fn pll_calibr_end(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_pll_best_min_cur(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, u8, PllSysStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_pll_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
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
pub struct PllSysTestReg_SPEC;
impl crate::sealed::RegSpec for PllSysTestReg_SPEC {
    type DataType = u16;
}

pub type PllSysTestReg = crate::RegValueT<PllSysTestReg_SPEC>;

impl PllSysTestReg {
    #[inline(always)]
    pub fn pll_lock_det_res_cnt(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, PllSysTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_sel_r_div_test(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_sel_n_div_test(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_change(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_open_loop(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_test_vctr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_min_current(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, PllSysTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_dis_loopfilt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysTestReg {
    #[inline(always)]
    fn default() -> PllSysTestReg {
        <crate::RegValueT<PllSysTestReg_SPEC> as RegisterValue<_>>::new(112)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetFreezeReg_SPEC;
impl crate::sealed::RegSpec for ResetFreezeReg_SPEC {
    type DataType = u16;
}

pub type ResetFreezeReg = crate::RegValueT<ResetFreezeReg_SPEC>;

impl ResetFreezeReg {
    #[inline(always)]
    pub fn frz_swtim2(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_swtim1(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_usb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_swtim0(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetFreezeReg {
    #[inline(always)]
    fn default() -> ResetFreezeReg {
        <crate::RegValueT<ResetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetFreezeReg_SPEC;
impl crate::sealed::RegSpec for SetFreezeReg_SPEC {
    type DataType = u16;
}

pub type SetFreezeReg = crate::RegValueT<SetFreezeReg_SPEC>;

impl SetFreezeReg {
    #[inline(always)]
    pub fn frz_swtim2(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_swtim1(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_usb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_swtim0(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetFreezeReg {
    #[inline(always)]
    fn default() -> SetFreezeReg {
        <crate::RegValueT<SetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
