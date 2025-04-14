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
#[doc = r"DCDC_BOOST registers"]
unsafe impl ::core::marker::Send for super::DcdcBoost {}
unsafe impl ::core::marker::Sync for super::DcdcBoost {}
impl super::DcdcBoost {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn boost_ctrl_reg0(
        &self,
    ) -> &'static crate::common::Reg<self::BoostCtrlReg0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostCtrlReg0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn boost_ctrl_reg1(
        &self,
    ) -> &'static crate::common::Reg<self::BoostCtrlReg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostCtrlReg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn boost_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BoostStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn boost_test_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BoostTestCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BoostTestCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vled_pwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VledPwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VledPwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vled_pwr_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VledPwrStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VledPwrStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostCtrlReg0_SPEC;
impl crate::sealed::RegSpec for BoostCtrlReg0_SPEC {
    type DataType = u32;
}

pub type BoostCtrlReg0 = crate::RegValueT<BoostCtrlReg0_SPEC>;

impl BoostCtrlReg0 {
    #[inline(always)]
    pub fn boost_timeout_trig_delay(
        self,
    ) -> crate::common::RegisterField<17, 0xf, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0xf,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_psw_timeout(
        self,
    ) -> crate::common::RegisterField<13, 0xf, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0xf,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_nsw_timeout(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xf,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_ok_clr_count(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_idle_clk_div(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_vled_trim(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_vled_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, BoostCtrlReg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,BoostCtrlReg0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BoostCtrlReg0 {
    #[inline(always)]
    fn default() -> BoostCtrlReg0 {
        <crate::RegValueT<BoostCtrlReg0_SPEC> as RegisterValue<_>>::new(593196)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostCtrlReg1_SPEC;
impl crate::sealed::RegSpec for BoostCtrlReg1_SPEC {
    type DataType = u32;
}

pub type BoostCtrlReg1 = crate::RegValueT<BoostCtrlReg1_SPEC>;

impl BoostCtrlReg1 {
    #[inline(always)]
    pub fn boost_cur_lim_sleep_fixed(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BoostCtrlReg1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_cur_lim_sleep(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_cur_lim_step(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_cur_lim_max(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_cur_lim_min(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, BoostCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,BoostCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BoostCtrlReg1 {
    #[inline(always)]
    fn default() -> BoostCtrlReg1 {
        <crate::RegValueT<BoostCtrlReg1_SPEC> as RegisterValue<_>>::new(261092)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostStatusReg_SPEC;
impl crate::sealed::RegSpec for BoostStatusReg_SPEC {
    type DataType = u32;
}

pub type BoostStatusReg = crate::RegValueT<BoostStatusReg_SPEC>;

impl BoostStatusReg {
    #[inline(always)]
    pub fn boost_comp_trim(
        self,
    ) -> crate::common::RegisterField<18, 0x3f, 1, 0, u8, u8, BoostStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x3f,1,0,u8,u8,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_idle(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_cur_lim(
        self,
    ) -> crate::common::RegisterField<12, 0x1f, 1, 0, u8, u8, BoostStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1f,1,0,u8,u8,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_comp_p_dyn_p(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_comp_p_dyn_n(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_comp_p_cont(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_comp_n_cont(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_timeout_psw(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_timeout_nsw(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_vout_nok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_vout_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_sw_state(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, BoostStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_startup_complete(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_vled_ok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BoostStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BoostStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BoostStatusReg {
    #[inline(always)]
    fn default() -> BoostStatusReg {
        <crate::RegValueT<BoostStatusReg_SPEC> as RegisterValue<_>>::new(147456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BoostTestCtrlReg_SPEC;
impl crate::sealed::RegSpec for BoostTestCtrlReg_SPEC {
    type DataType = u32;
}

pub type BoostTestCtrlReg = crate::RegValueT<BoostTestCtrlReg_SPEC>;

impl BoostTestCtrlReg {
    #[inline(always)]
    pub fn boost_iload_sel_test(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, u8, BoostTestCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            27,
            0x3,
            1,
            0,
            u8,
            u8,
            BoostTestCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn boost_lssup_trim(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, BoostTestCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            u8,
            u8,
            BoostTestCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn boost_hsgnd_trim(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, u8, BoostTestCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            u8,
            u8,
            BoostTestCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BoostTestCtrlReg {
    #[inline(always)]
    fn default() -> BoostTestCtrlReg {
        <crate::RegValueT<BoostTestCtrlReg_SPEC> as RegisterValue<_>>::new(15729152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VledPwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for VledPwrCtrlReg_SPEC {
    type DataType = u32;
}

pub type VledPwrCtrlReg = crate::RegValueT<VledPwrCtrlReg_SPEC>;

impl VledPwrCtrlReg {
    #[inline(always)]
    pub fn vled_pwr_force(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, VledPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vled_pwr_manual(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, VledPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vled_pwr_use_vsys_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, VledPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vled_pwr_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, VledPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vsys_ok_debounce(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, VledPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,VledPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VledPwrCtrlReg {
    #[inline(always)]
    fn default() -> VledPwrCtrlReg {
        <crate::RegValueT<VledPwrCtrlReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VledPwrStatusReg_SPEC;
impl crate::sealed::RegSpec for VledPwrStatusReg_SPEC {
    type DataType = u32;
}

pub type VledPwrStatusReg = crate::RegValueT<VledPwrStatusReg_SPEC>;

impl VledPwrStatusReg {
    #[inline(always)]
    pub fn vled_pwr_allow_boost(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vled_pwr_vsys_connected(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vled_pwr_switch_ctrl_state(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, VledPwrStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vsys_ok_debounced(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VledPwrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,VledPwrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for VledPwrStatusReg {
    #[inline(always)]
    fn default() -> VledPwrStatusReg {
        <crate::RegValueT<VledPwrStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
