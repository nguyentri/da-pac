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
#[doc = r"CRG_TOP registers"]
unsafe impl ::core::marker::Send for super::CrgTop {}
unsafe impl ::core::marker::Sync for super::CrgTop {}
impl super::CrgTop {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ana_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AnaStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AnaStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bandgap_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BandgapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BandgapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_lvl_ctrl0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodLvlCtrl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodLvlCtrl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_lvl_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodLvlCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodLvlCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_lvl_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodLvlCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodLvlCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_amba_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkAmbaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkAmbaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_radio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRadioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRadioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_rc32k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRc32KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRc32KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_rcx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRcxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRcxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_switch2xtal_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkSwitch2XtalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkSwitch2XtalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_tmr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkTmrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkTmrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn discharge_rail_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DischargeRailReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DischargeRailReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ldo_vddd_high_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LdoVdddHighCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0PadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_reset_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0ResetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0ResetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_set_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0SetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0SetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1PadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_reset_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1ResetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1ResetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_set_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1SetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1SetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pmu_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pmu_sleep_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuSleepReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuSleepReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pmu_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn por_pin_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorPinReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorPinReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn por_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn por_vbat_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorVbatCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorVbatCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ram_pwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RamPwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RamPwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[inline(always)]
    pub const fn secure_boot_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SecureBootReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SecureBootReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sys_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SysStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SysStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vbus_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaStatusReg_SPEC;
impl crate::sealed::RegSpec for AnaStatusReg_SPEC {
    type DataType = u32;
}

pub type AnaStatusReg = crate::RegValueT<AnaStatusReg_SPEC>;

impl AnaStatusReg {
    #[inline(always)]
    pub fn comp_vbus_high(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbus_low(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbat_high(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbat_low(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vdd_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbus_available(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bandgap_ok(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_3v0_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_3v0_vbus_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8p_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_radio_ok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_ok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vdd_high_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vin_nok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AnaStatusReg {
    #[inline(always)]
    fn default() -> AnaStatusReg {
        <crate::RegValueT<AnaStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BandgapReg_SPEC;
impl crate::sealed::RegSpec for BandgapReg_SPEC {
    type DataType = u32;
}

pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[inline(always)]
    pub fn bandgap_enable_clamp(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sysram_lpmx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgr_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BandgapReg {
    #[inline(always)]
    fn default() -> BandgapReg {
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodLvlCtrl0Reg_SPEC;
impl crate::sealed::RegSpec for BodLvlCtrl0Reg_SPEC {
    type DataType = u32;
}

pub type BodLvlCtrl0Reg = crate::RegValueT<BodLvlCtrl0Reg_SPEC>;

impl BodLvlCtrl0Reg {
    #[inline(always)]
    pub fn bod_lvl_v18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BodLvlCtrl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BodLvlCtrl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bod_lvl_v30(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BodLvlCtrl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BodLvlCtrl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bod_lvl_vbat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BodLvlCtrl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BodLvlCtrl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BodLvlCtrl0Reg {
    #[inline(always)]
    fn default() -> BodLvlCtrl0Reg {
        <crate::RegValueT<BodLvlCtrl0Reg_SPEC> as RegisterValue<_>>::new(69078703)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodLvlCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for BodLvlCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type BodLvlCtrl1Reg = crate::RegValueT<BodLvlCtrl1Reg_SPEC>;

impl BodLvlCtrl1Reg {
    #[inline(always)]
    pub fn bod_lvl_vdd_ret(
        self,
    ) -> crate::common::RegisterField<17, 0xff, 1, 0, u8, u8, BodLvlCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0xff,1,0,u8,u8,BodLvlCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_lvl_vdd_on(
        self,
    ) -> crate::common::RegisterField<9, 0xff, 1, 0, u8, u8, BodLvlCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xff,1,0,u8,u8,BodLvlCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_lvl_v18p(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BodLvlCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BodLvlCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BodLvlCtrl1Reg {
    #[inline(always)]
    fn default() -> BodLvlCtrl1Reg {
        <crate::RegValueT<BodLvlCtrl1Reg_SPEC> as RegisterValue<_>>::new(14614791)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodLvlCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for BodLvlCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type BodLvlCtrl2Reg = crate::RegValueT<BodLvlCtrl2Reg_SPEC>;

impl BodLvlCtrl2Reg {
    #[inline(always)]
    pub fn bod_lvl_v14(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BodLvlCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BodLvlCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bod_lvl_v18f(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BodLvlCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BodLvlCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BodLvlCtrl2Reg {
    #[inline(always)]
    fn default() -> BodLvlCtrl2Reg {
        <crate::RegValueT<BodLvlCtrl2Reg_SPEC> as RegisterValue<_>>::new(102151)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodStatusReg_SPEC;
impl crate::sealed::RegSpec for BodStatusReg_SPEC {
    type DataType = u32;
}

pub type BodStatusReg = crate::RegValueT<BodStatusReg_SPEC>;

impl BodStatusReg {
    #[inline(always)]
    pub fn bod_v14(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18f(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vdd(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v30(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vbat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BodStatusReg {
    #[inline(always)]
    fn default() -> BodStatusReg {
        <crate::RegValueT<BodStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkAmbaReg_SPEC;
impl crate::sealed::RegSpec for ClkAmbaReg_SPEC {
    type DataType = u32;
}

pub type ClkAmbaReg = crate::RegValueT<ClkAmbaReg_SPEC>;

impl ClkAmbaReg {
    #[inline(always)]
    pub fn qspi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspi2_div(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspi_div(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trng_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aes_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pclk_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hclk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkAmbaReg {
    #[inline(always)]
    fn default() -> ClkAmbaReg {
        <crate::RegValueT<ClkAmbaReg_SPEC> as RegisterValue<_>>::new(34)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCtrlReg_SPEC;
impl crate::sealed::RegSpec for ClkCtrlReg_SPEC {
    type DataType = u32;
}

pub type ClkCtrlReg = crate::RegValueT<ClkCtrlReg_SPEC>;

impl ClkCtrlReg {
    #[inline(always)]
    pub fn running_at_pll96m(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_xtal32m(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_rc32m(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_lp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_clk_src(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_clk_sel(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_clk_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkCtrlReg {
    #[inline(always)]
    fn default() -> ClkCtrlReg {
        <crate::RegValueT<ClkCtrlReg_SPEC> as RegisterValue<_>>::new(8257)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRadioReg_SPEC;
impl crate::sealed::RegSpec for ClkRadioReg_SPEC {
    type DataType = u32;
}

pub type ClkRadioReg = crate::RegValueT<ClkRadioReg_SPEC>;

impl ClkRadioReg {
    #[inline(always)]
    pub fn rfcu_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_synch_reset(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRadioReg {
    #[inline(always)]
    fn default() -> ClkRadioReg {
        <crate::RegValueT<ClkRadioReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRc32KReg_SPEC;
impl crate::sealed::RegSpec for ClkRc32KReg_SPEC {
    type DataType = u32;
}

pub type ClkRc32KReg = crate::RegValueT<ClkRc32KReg_SPEC>;

impl ClkRc32KReg {
    #[inline(always)]
    pub fn rc32k_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRc32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRc32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRc32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRc32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRc32KReg {
    #[inline(always)]
    fn default() -> ClkRc32KReg {
        <crate::RegValueT<ClkRc32KReg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRcxReg_SPEC;
impl crate::sealed::RegSpec for ClkRcxReg_SPEC {
    type DataType = u32;
}

pub type ClkRcxReg = crate::RegValueT<ClkRcxReg_SPEC>;

impl ClkRcxReg {
    #[inline(always)]
    pub fn rcx_bias(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_c0(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_cadjust(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, ClkRcxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_radjust(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRcxReg {
    #[inline(always)]
    fn default() -> ClkRcxReg {
        <crate::RegValueT<ClkRcxReg_SPEC> as RegisterValue<_>>::new(2812)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSwitch2XtalReg_SPEC;
impl crate::sealed::RegSpec for ClkSwitch2XtalReg_SPEC {
    type DataType = u32;
}

pub type ClkSwitch2XtalReg = crate::RegValueT<ClkSwitch2XtalReg_SPEC>;

impl ClkSwitch2XtalReg {
    #[inline(always)]
    pub fn switch2xtal(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkSwitch2XtalReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,ClkSwitch2XtalReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkSwitch2XtalReg {
    #[inline(always)]
    fn default() -> ClkSwitch2XtalReg {
        <crate::RegValueT<ClkSwitch2XtalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTmrReg_SPEC;
impl crate::sealed::RegSpec for ClkTmrReg_SPEC {
    type DataType = u32;
}

pub type ClkTmrReg = crate::RegValueT<ClkTmrReg_SPEC>;

impl ClkTmrReg {
    #[inline(always)]
    pub fn tmr2_pwm_aon_mode(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr_pwm_aon_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeupct_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkTmrReg {
    #[inline(always)]
    fn default() -> ClkTmrReg {
        <crate::RegValueT<ClkTmrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DischargeRailReg_SPEC;
impl crate::sealed::RegSpec for DischargeRailReg_SPEC {
    type DataType = u32;
}

pub type DischargeRailReg = crate::RegValueT<DischargeRailReg_SPEC>;

impl DischargeRailReg {
    #[inline(always)]
    pub fn reset_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_v18(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_v14(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DischargeRailReg {
    #[inline(always)]
    fn default() -> DischargeRailReg {
        <crate::RegValueT<DischargeRailReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LdoVdddHighCtrlReg_SPEC;
impl crate::sealed::RegSpec for LdoVdddHighCtrlReg_SPEC {
    type DataType = u32;
}

pub type LdoVdddHighCtrlReg = crate::RegValueT<LdoVdddHighCtrlReg_SPEC>;

impl LdoVdddHighCtrlReg {
    #[inline(always)]
    pub fn ldo_vddd_high_low_zout_disable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LdoVdddHighCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vddd_high_static_load_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LdoVdddHighCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vddd_high_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LdoVdddHighCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vddd_high_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LdoVdddHighCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LdoVdddHighCtrlReg {
    #[inline(always)]
    fn default() -> LdoVdddHighCtrlReg {
        <crate::RegValueT<LdoVdddHighCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PadLatchReg_SPEC;
impl crate::sealed::RegSpec for P0PadLatchReg_SPEC {
    type DataType = u32;
}

pub type P0PadLatchReg = crate::RegValueT<P0PadLatchReg_SPEC>;

impl P0PadLatchReg {
    #[inline(always)]
    pub fn p0_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0PadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0PadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0PadLatchReg {
    #[inline(always)]
    fn default() -> P0PadLatchReg {
        <crate::RegValueT<P0PadLatchReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0ResetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P0ResetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P0ResetPadLatchReg = crate::RegValueT<P0ResetPadLatchReg_SPEC>;

impl P0ResetPadLatchReg {
    #[inline(always)]
    pub fn p0_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0ResetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0ResetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0ResetPadLatchReg {
    #[inline(always)]
    fn default() -> P0ResetPadLatchReg {
        <crate::RegValueT<P0ResetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0SetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P0SetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P0SetPadLatchReg = crate::RegValueT<P0SetPadLatchReg_SPEC>;

impl P0SetPadLatchReg {
    #[inline(always)]
    pub fn p0_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0SetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0SetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0SetPadLatchReg {
    #[inline(always)]
    fn default() -> P0SetPadLatchReg {
        <crate::RegValueT<P0SetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PadLatchReg_SPEC;
impl crate::sealed::RegSpec for P1PadLatchReg_SPEC {
    type DataType = u32;
}

pub type P1PadLatchReg = crate::RegValueT<P1PadLatchReg_SPEC>;

impl P1PadLatchReg {
    #[inline(always)]
    pub fn p1_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        P1PadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            P1PadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1PadLatchReg {
    #[inline(always)]
    fn default() -> P1PadLatchReg {
        <crate::RegValueT<P1PadLatchReg_SPEC> as RegisterValue<_>>::new(8388607)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1ResetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P1ResetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P1ResetPadLatchReg = crate::RegValueT<P1ResetPadLatchReg_SPEC>;

impl P1ResetPadLatchReg {
    #[inline(always)]
    pub fn p1_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        P1ResetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            P1ResetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1ResetPadLatchReg {
    #[inline(always)]
    fn default() -> P1ResetPadLatchReg {
        <crate::RegValueT<P1ResetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1SetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P1SetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P1SetPadLatchReg = crate::RegValueT<P1SetPadLatchReg_SPEC>;

impl P1SetPadLatchReg {
    #[inline(always)]
    pub fn p1_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        P1SetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            P1SetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1SetPadLatchReg {
    #[inline(always)]
    fn default() -> P1SetPadLatchReg {
        <crate::RegValueT<P1SetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuCtrlReg_SPEC;
impl crate::sealed::RegSpec for PmuCtrlReg_SPEC {
    type DataType = u32;
}

pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[inline(always)]
    pub fn enable_clkless(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn retain_cache(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_on_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn map_bandgap_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn com_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn periph_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuCtrlReg {
    #[inline(always)]
    fn default() -> PmuCtrlReg {
        <crate::RegValueT<PmuCtrlReg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuSleepReg_SPEC;
impl crate::sealed::RegSpec for PmuSleepReg_SPEC {
    type DataType = u32;
}

pub type PmuSleepReg = crate::RegValueT<PmuSleepReg_SPEC>;

impl PmuSleepReg {
    #[inline(always)]
    pub fn clamp_vdd_wkup_max(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ultra_fast_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fast_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_sleep_interval(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bg_refresh_interval(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuSleepReg {
    #[inline(always)]
    fn default() -> PmuSleepReg {
        <crate::RegValueT<PmuSleepReg_SPEC> as RegisterValue<_>>::new(65664)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuTrimReg_SPEC;
impl crate::sealed::RegSpec for PmuTrimReg_SPEC {
    type DataType = u32;
}

pub type PmuTrimReg = crate::RegValueT<PmuTrimReg_SPEC>;

impl PmuTrimReg {
    #[inline(always)]
    pub fn ldo_1v8_trim(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8p_trim(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_supply_vbat_trim(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_supply_vbus_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuTrimReg {
    #[inline(always)]
    fn default() -> PmuTrimReg {
        <crate::RegValueT<PmuTrimReg_SPEC> as RegisterValue<_>>::new(34816)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorPinReg_SPEC;
impl crate::sealed::RegSpec for PorPinReg_SPEC {
    type DataType = u32;
}

pub type PorPinReg = crate::RegValueT<PorPinReg_SPEC>;

impl PorPinReg {
    #[inline(always)]
    pub fn por_pin_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PorPinReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_pin_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, PorPinReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorPinReg {
    #[inline(always)]
    fn default() -> PorPinReg {
        <crate::RegValueT<PorPinReg_SPEC> as RegisterValue<_>>::new(63)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorTimerReg_SPEC;
impl crate::sealed::RegSpec for PorTimerReg_SPEC {
    type DataType = u32;
}

pub type PorTimerReg = crate::RegValueT<PorTimerReg_SPEC>;

impl PorTimerReg {
    #[inline(always)]
    pub fn por_time(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, PorTimerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,PorTimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorTimerReg {
    #[inline(always)]
    fn default() -> PorTimerReg {
        <crate::RegValueT<PorTimerReg_SPEC> as RegisterValue<_>>::new(24)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorVbatCtrlReg_SPEC;
impl crate::sealed::RegSpec for PorVbatCtrlReg_SPEC {
    type DataType = u32;
}

pub type PorVbatCtrlReg = crate::RegValueT<PorVbatCtrlReg_SPEC>;

impl PorVbatCtrlReg {
    #[inline(always)]
    pub fn por_vbat_mask_n(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PorVbatCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PorVbatCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_hyst_low(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_thres_high(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_thres_low(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorVbatCtrlReg {
    #[inline(always)]
    fn default() -> PorVbatCtrlReg {
        <crate::RegValueT<PorVbatCtrlReg_SPEC> as RegisterValue<_>>::new(12911)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamPwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for RamPwrCtrlReg_SPEC {
    type DataType = u32;
}

pub type RamPwrCtrlReg = crate::RegValueT<RamPwrCtrlReg_SPEC>;

impl RamPwrCtrlReg {
    #[inline(always)]
    pub fn ram8_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram7_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram6_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram5_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram4_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram3_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram2_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram1_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RamPwrCtrlReg {
    #[inline(always)]
    fn default() -> RamPwrCtrlReg {
        <crate::RegValueT<RamPwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetStatReg_SPEC;
impl crate::sealed::RegSpec for ResetStatReg_SPEC {
    type DataType = u32;
}

pub type ResetStatReg = crate::RegValueT<ResetStatReg_SPEC>;

impl ResetStatReg {
    #[inline(always)]
    pub fn cmac_wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swd_hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn poreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetStatReg {
    #[inline(always)]
    fn default() -> ResetStatReg {
        <crate::RegValueT<ResetStatReg_SPEC> as RegisterValue<_>>::new(63)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecureBootReg_SPEC;
impl crate::sealed::RegSpec for SecureBootReg_SPEC {
    type DataType = u32;
}

pub type SecureBootReg = crate::RegValueT<SecureBootReg_SPEC>;

impl SecureBootReg {
    #[inline(always)]
    pub fn prot_qspi_key_read(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_qspi_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_aes_key_read(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_aes_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_sig_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_cmac_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn secure_boot(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SecureBootReg {
    #[inline(always)]
    fn default() -> SecureBootReg {
        <crate::RegValueT<SecureBootReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysStatReg_SPEC;
impl crate::sealed::RegSpec for SysStatReg_SPEC {
    type DataType = u32;
}

pub type SysStatReg = crate::RegValueT<SysStatReg_SPEC>;

impl SysStatReg {
    #[inline(always)]
    pub fn power_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbg_is_active(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn com_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn com_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mem_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mem_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn per_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn per_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rad_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rad_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SysStatReg {
    #[inline(always)]
    fn default() -> SysStatReg {
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(9637)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqClearReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqClearReg_SPEC {
    type DataType = u32;
}

pub type VbusIrqClearReg = crate::RegValueT<VbusIrqClearReg_SPEC>;

impl VbusIrqClearReg {
    #[inline(always)]
    pub fn vbus_irq_clear(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        VbusIrqClearReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            VbusIrqClearReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for VbusIrqClearReg {
    #[inline(always)]
    fn default() -> VbusIrqClearReg {
        <crate::RegValueT<VbusIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
