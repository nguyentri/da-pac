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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:17 +0000

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
    #[doc = "status bit of analog (power management) circuits"]
    #[inline(always)]
    pub const fn ana_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AnaStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AnaStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "bandgap trimming"]
    #[inline(always)]
    pub const fn bandgap_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BandgapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BandgapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Brown Out Detection control register"]
    #[inline(always)]
    pub const fn bod_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "Brown Out Detection status register"]
    #[inline(always)]
    pub const fn bod_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "HCLK, PCLK, divider and clock gates"]
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

    #[doc = "Xtal frequency trimming register."]
    #[inline(always)]
    pub const fn clk_freq_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkFreqTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkFreqTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Radio PLL control register"]
    #[inline(always)]
    pub const fn clk_radio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRadioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRadioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "RCX-oscillator control register"]
    #[inline(always)]
    pub const fn clk_rcx20k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRcx20KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRcx20KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Clock control for the timers"]
    #[inline(always)]
    pub const fn clk_tmr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkTmrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkTmrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Immediate rail resetting. There is no LDO/DCDC gating"]
    #[inline(always)]
    pub const fn discharge_rail_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DischargeRailReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DischargeRailReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[doc = "LDO control register"]
    #[inline(always)]
    pub const fn ldo_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LdoCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LdoCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[doc = "LDO control register"]
    #[inline(always)]
    pub const fn ldo_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LdoCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LdoCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Retention LDO control register"]
    #[inline(always)]
    pub const fn ldo_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LdoCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LdoCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Power Management Unit control register"]
    #[inline(always)]
    pub const fn pmu_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Controls rail resetting when RST is pulsed"]
    #[inline(always)]
    pub const fn pmu_reset_rail_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuResetRailReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuResetRailReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Controls the POR on VBAT"]
    #[inline(always)]
    pub const fn por_vbat_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorVbatCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorVbatCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "Reset status register"]
    #[inline(always)]
    pub const fn reset_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(94usize),
            )
        }
    }

    #[doc = "Controls secure booting"]
    #[inline(always)]
    pub const fn secure_boot_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SecureBootReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SecureBootReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[doc = "Timer for regulated sleep"]
    #[inline(always)]
    pub const fn sleep_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SleepTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SleepTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Control trimming of the XTAL16M"]
    #[inline(always)]
    pub const fn trim_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TrimCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TrimCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Clear pending IRQ register"]
    #[inline(always)]
    pub const fn vbus_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "IRQ masking"]
    #[inline(always)]
    pub const fn vbus_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Control register for XTALRDY IRQ"]
    #[inline(always)]
    pub const fn xtalrdy_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::XtalrdyCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::XtalrdyCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaStatusReg_SPEC;
impl crate::sealed::RegSpec for AnaStatusReg_SPEC {
    type DataType = u16;
}
#[doc = "status bit of analog (power management) circuits"]
pub type AnaStatusReg = crate::RegValueT<AnaStatusReg_SPEC>;

impl AnaStatusReg {
    #[doc = "VDD1V8P > 1.7V"]
    #[inline(always)]
    pub fn comp_1v8_pa_high(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDD1V8 > 1.7V"]
    #[inline(always)]
    pub fn comp_1v8_flash_high(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "V33 > 1.7V"]
    #[inline(always)]
    pub fn comp_v33_high(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VBUS > 3.4V"]
    #[inline(always)]
    pub fn comp_vbus_low(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VBUS > 4V"]
    #[inline(always)]
    pub fn comp_vbus_high(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ldo_vdd1v8 = ok"]
    #[inline(always)]
    pub fn ldo_1v8_flash_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ldo_vdd1v8P = ok"]
    #[inline(always)]
    pub fn ldo_1v8_pa_ok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ldo_core = ok"]
    #[inline(always)]
    pub fn ldo_core_ok(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDD > 1.13V"]
    #[inline(always)]
    pub fn comp_vdd_high(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "bandgap = ok"]
    #[inline(always)]
    pub fn bandgap_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ldo_supply_usb = ok"]
    #[inline(always)]
    pub fn ldo_supply_usb_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ldo_supply_vbat =ok"]
    #[inline(always)]
    pub fn ldo_supply_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "new battery has been detected"]
    #[inline(always)]
    pub fn newbat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "vbus is available (vbus > vbat)"]
    #[inline(always)]
    pub fn vbus_available(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "vbat > 1.7V"]
    #[inline(always)]
    pub fn comp_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ldo_radio = ok"]
    #[inline(always)]
    pub fn ldo_radio_ok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AnaStatusReg {
    #[inline(always)]
    fn default() -> AnaStatusReg {
        <crate::RegValueT<AnaStatusReg_SPEC> as RegisterValue<_>>::new(144)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BandgapReg_SPEC;
impl crate::sealed::RegSpec for BandgapReg_SPEC {
    type DataType = u16;
}
#[doc = "bandgap trimming"]
pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[doc = "0x0 -> LDO_SUPPLY_(VBAT/USB) uses V12 voltage/(V12/2Mohm) current as reference\n0x1 -> LDO_SUPPLY_(VBAT/USB) uses bandgap voltage/bandgap current (1uA) as reference -> set 0x1 in (booter-)software\nSwitch to 0x1 at start of user application when maximum BOD functionality is switched on."]
    #[inline(always)]
    pub fn ldo_supply_use_bgref(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0x4 --> 1120 mV\n0x5 --> 1089 mV\n0x6 --> 1058 mV\n0x7 --> 1030 mV\n0x0 --> 1037 mV\n0x1 --> 1005 mV\n0x2 --> 978 mV\n0x3 --> 946 mV\n0x8 --> 952 mV\n0x9 --> 918 mV\n0xA --> 889 mV\n0xB --> 861 mV\n0xC --> 862 mV\n0xD --> 828 mV\n0xE --> 798 mV\n0xF --> 770 mV\nThese values are from simulation and vary over corners"]
    #[inline(always)]
    pub fn ldo_sleep_trim(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0xf,1,0,u8, BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current trimming for bias"]
    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1f,1,0,u8, BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trim register for bandgap"]
    #[inline(always)]
    pub fn bgr_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BandgapReg {
    #[inline(always)]
    fn default() -> BandgapReg {
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for BodCtrl2Reg_SPEC {
    type DataType = u16;
}
#[doc = "Brown Out Detection control register"]
pub type BodCtrl2Reg = crate::RegValueT<BodCtrl2Reg_SPEC>;

impl BodCtrl2Reg {
    #[doc = "V14 BOD Enable"]
    #[inline(always)]
    pub fn bod_v14_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBAT BOD Enable"]
    #[inline(always)]
    pub fn bod_vbat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1V8 Flash BOD Enable"]
    #[inline(always)]
    pub fn bod_1v8_flash_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1V8 PA BOD Enable"]
    #[inline(always)]
    pub fn bod_1v8_pa_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "V33 BOD Enable"]
    #[inline(always)]
    pub fn bod_v33_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD BOD Enable"]
    #[inline(always)]
    pub fn bod_vdd_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Generate a chip reset on BOD event"]
    #[inline(always)]
    pub fn bod_reset_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BodCtrl2Reg {
    #[inline(always)]
    fn default() -> BodCtrl2Reg {
        <crate::RegValueT<BodCtrl2Reg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodStatusReg_SPEC;
impl crate::sealed::RegSpec for BodStatusReg_SPEC {
    type DataType = u16;
}
#[doc = "Brown Out Detection status register"]
pub type BodStatusReg = crate::RegValueT<BodStatusReg_SPEC>;

impl BodStatusReg {
    #[doc = "Indicates V14 > V14_Trigger"]
    #[inline(always)]
    pub fn bod_v14_low(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates VBAT > VBAT_Trigger"]
    #[inline(always)]
    pub fn bod_vbat_low(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates V33 > V33_Trigger"]
    #[inline(always)]
    pub fn bod_v33_low(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates V18_Flash > V18_Flash_Trigger"]
    #[inline(always)]
    pub fn bod_1v8_flash_low(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates V18_PA > V18_PA_Trigger"]
    #[inline(always)]
    pub fn bod_1v8_pa_low(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates VDD > VDD_Trigger"]
    #[inline(always)]
    pub fn bod_vdd_low(
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
    type DataType = u16;
}
#[doc = "HCLK, PCLK, divider and clock gates"]
pub type ClkAmbaReg = crate::RegValueT<ClkAmbaReg_SPEC>;

impl ClkAmbaReg {
    #[doc = "Clock enable for QSPI controller"]
    #[inline(always)]
    pub fn qspi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI divider\n00 = divide by 1\n01 = divide by 2\n10 = divide by 4\n11 = divide by 8"]
    #[inline(always)]
    pub fn qspi_div(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock enable for OTP controller"]
    #[inline(always)]
    pub fn otp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock enable for TRNG block"]
    #[inline(always)]
    pub fn trng_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock enable for ECC block"]
    #[inline(always)]
    pub fn ecc_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock enable for AES crypto block"]
    #[inline(always)]
    pub fn aes_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "APB interface clock, Cascaded with HCLK:\n00 = divide hclk by 1\n01 = divide hclk by 2\n10 = divide hclk by 4\n11 = divide hclk by 8"]
    #[inline(always)]
    pub fn pclk_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "AHB interface and microprocessor clock. Source clock divided by:\n000 = divide hclk by 1\n001 = divide hclk by 2\n010 = divide hclk by 4\n011 = divide hclk by 8\n1xx = divide hclk by 16"]
    #[inline(always)]
    pub fn hclk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct ClkFreqTrimReg_SPEC;
impl crate::sealed::RegSpec for ClkFreqTrimReg_SPEC {
    type DataType = u16;
}
#[doc = "Xtal frequency trimming register."]
pub type ClkFreqTrimReg = crate::RegValueT<ClkFreqTrimReg_SPEC>;

impl ClkFreqTrimReg {
    #[doc = "Xtal frequency course trimming register.\n0x0 = lowest frequency\n0x7 = highest frequencyIncrement or decrement the binary value with 1. Wait approximately 200usec to allow the adjustment to settle."]
    #[inline(always)]
    pub fn coarse_adj(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8, ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Xtal frequency fine trimming register.0x00 = lowest frequency\n0xFF = highest frequency"]
    #[inline(always)]
    pub fn fine_adj(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkFreqTrimReg {
    #[inline(always)]
    fn default() -> ClkFreqTrimReg {
        <crate::RegValueT<ClkFreqTrimReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRadioReg_SPEC;
impl crate::sealed::RegSpec for ClkRadioReg_SPEC {
    type DataType = u16;
}
#[doc = "Radio PLL control register"]
pub type ClkRadioReg = crate::RegValueT<ClkRadioReg_SPEC>;

impl ClkRadioReg {
    #[doc = "Enable the FTDF MAC core clocks"]
    #[inline(always)]
    pub fn ftdf_mac_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for FTCF MAC clock, relative to the DIVN clock\n00 = Divide by 1\n01 = Divide by 2\n10 = Divide by 4\n11 = Divide by 8\nIt should always be set to 00."]
    #[inline(always)]
    pub fn ftdf_mac_div(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable the BLE core clocks.\nWhen the BLE system clock is disabled, either due to the CLK_RADIO_REG\\[BLE_ENABLE\\] or due to the PMU_CTRL_REG\\[BLE_SLEEP\\], then any access to the BLE Register file will issue a hard fault to the CPU."]
    #[inline(always)]
    pub fn ble_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reset for the BLE LP timer"]
    #[inline(always)]
    pub fn ble_lp_reset(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for BLE core blocks, having as reference the DIVN clock:\n00 = Divide by 1\n01 = Divide by 2\n10 = Divide by 4\n11 = Divide by 8\nThe programmed frequency should not be lower than 8MHz, not faster than 16MHz and not faster than the programmed CPU clock frequency. Refer also to BLE_CNTL2_REG\\[BLE_CLK_SEL\\]."]
    #[inline(always)]
    pub fn ble_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable the RF control Unit clock"]
    #[inline(always)]
    pub fn rfcu_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for RF Control Unit\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8\nThe programmed frequency must be exactly 8MHz."]
    #[inline(always)]
    pub fn rfcu_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRadioReg {
    #[inline(always)]
    fn default() -> ClkRadioReg {
        <crate::RegValueT<ClkRadioReg_SPEC> as RegisterValue<_>>::new(64)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRcx20KReg_SPEC;
impl crate::sealed::RegSpec for ClkRcx20KReg_SPEC {
    type DataType = u16;
}
#[doc = "RCX-oscillator control register"]
pub type ClkRcx20KReg = crate::RegValueT<ClkRcx20KReg_SPEC>;

impl ClkRcx20KReg {
    #[doc = "Enable the RCX oscillator"]
    #[inline(always)]
    pub fn rcx20k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extra low frequency"]
    #[inline(always)]
    pub fn rcx20k_lowf(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bias control"]
    #[inline(always)]
    pub fn rcx20k_bias(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Temperature control"]
    #[inline(always)]
    pub fn rcx20k_ntc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0000 = lowest frequency\n0111 = default\n1111 = highest frequency"]
    #[inline(always)]
    pub fn rcx20k_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRcx20KReg {
    #[inline(always)]
    fn default() -> ClkRcx20KReg {
        <crate::RegValueT<ClkRcx20KReg_SPEC> as RegisterValue<_>>::new(1218)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTmrReg_SPEC;
impl crate::sealed::RegSpec for ClkTmrReg_SPEC {
    type DataType = u16;
}
#[doc = "Clock control for the timers"]
pub type ClkTmrReg = crate::RegValueT<ClkTmrReg_SPEC>;

impl ClkTmrReg {
    #[doc = "Maps Timer1_pwm onto P06, when DEBUGGER_EN = \'0\'.\nThis state is preserved during deep sleep, to allow PWM output on the pad during deep sleep."]
    #[inline(always)]
    pub fn p06_tmr1_pwm_mode(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn wakeupct_enable(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn breath_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn tmr2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable timer clock"]
    #[inline(always)]
    pub fn tmr2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for Timer\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8"]
    #[inline(always)]
    pub fn tmr2_div(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn tmr1_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable timer clock"]
    #[inline(always)]
    pub fn tmr1_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for Timer\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8"]
    #[inline(always)]
    pub fn tmr1_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn tmr0_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable timer clock"]
    #[inline(always)]
    pub fn tmr0_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for Timer\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8"]
    #[inline(always)]
    pub fn tmr0_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u16;
}
#[doc = "Immediate rail resetting. There is no LDO/DCDC gating"]
pub type DischargeRailReg = crate::RegValueT<DischargeRailReg_SPEC>;

impl DischargeRailReg {
    #[doc = "1: Enables immediate discharging of the V18P rail. Note that the source is not disabled.\n0: disable immediate discharging of the V18P rail.\nThis bit is ORed with the automatic function controlled by PMU_RESET_RAIL_REG.RESET_V18P"]
    #[inline(always)]
    pub fn reset_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: Enables immediate discharging of the V18 rail. Note that the source is not disabled.\n0: disable immediate discharging of the V18 rail.\nThis bit is ORed with the automatic function controlled by PMU_RESET_RAIL_REG.RESET_V18"]
    #[inline(always)]
    pub fn reset_v18(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: Enables immediate discharging of the V14 rail. Note that the source is not disabled.\n0: disable immediate discharging of the V14 rail.\nThis bit is ORed with the automatic function controlled by PMU_RESET_RAIL_REG.RESET_V14"]
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
pub struct LdoCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for LdoCtrl1Reg_SPEC {
    type DataType = u16;
}
#[doc = "LDO control register"]
pub type LdoCtrl1Reg = crate::RegValueT<LdoCtrl1Reg_SPEC>;

impl LdoCtrl1Reg {
    #[doc = "Enables (1) or disables (0) LDO_RADIO\nFor fast XTAL startup, this bit may be kept to \'1\' during deep sleep. The LDO is switched off automatically when in deep sleep, and enabled when waking up."]
    #[inline(always)]
    pub fn ldo_radio_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the output voltage of LDO_RADIO\n000 = 1.30 V\n001 = 1.35 V\n010 = 1.40 V\n011 = 1.45 V\n1XX = 1.50 V"]
    #[inline(always)]
    pub fn ldo_radio_setvdd(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the output voltage of LDO_CORE\n000 = 1.20 V\n001 = 1.15 V\n010 = 1.10 V\n011 = 1.05 V\n1XX = 1.32 V"]
    #[inline(always)]
    pub fn ldo_core_setvdd(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the output voltage of LDO_SUPPLY_USB\n00 = 2.40 V\n01 = 3.30 V\n10 = 3.45 V\n11 = 3.00 V"]
    #[inline(always)]
    pub fn ldo_supply_usb_level(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the output voltage of LDO_SUPPLY_VBAT\n00 = 2.40 V\n01 = 3.30 V\n10 = 3.45 V\n11 = 3.00 V"]
    #[inline(always)]
    pub fn ldo_supply_vbat_level(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the output voltage of LDO_VBAT_RET\n00 = 2.40 V\n01 = 3.30 V\n10 = 3.45 V\n11 = 3.00 V"]
    #[inline(always)]
    pub fn ldo_vbat_ret_level(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the current limit of LDO_CORE\n00 = Current limiter disabled\n01 = 8 mA\n10 = 60 mA\n11 = 80 mA"]
    #[inline(always)]
    pub fn ldo_core_curlim(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LdoCtrl1Reg {
    #[inline(always)]
    fn default() -> LdoCtrl1Reg {
        <crate::RegValueT<LdoCtrl1Reg_SPEC> as RegisterValue<_>>::new(87)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LdoCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for LdoCtrl2Reg_SPEC {
    type DataType = u16;
}
#[doc = "LDO control register"]
pub type LdoCtrl2Reg = crate::RegValueT<LdoCtrl2Reg_SPEC>;

impl LdoCtrl2Reg {
    #[doc = "Disables (1) or enables (0) LDO_1V8_PA_RET"]
    #[inline(always)]
    pub fn ldo_1v8_pa_ret_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disables (1) or enables (0) LDO_1V8_FLASH_RET"]
    #[inline(always)]
    pub fn ldo_1v8_flash_ret_disable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disables (1) or enables (0) LDO_VBAT_RET"]
    #[inline(always)]
    pub fn ldo_vbat_ret_disable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables (1) or disables (0) LDO_1V8_PA"]
    #[inline(always)]
    pub fn ldo_1v8_pa_on(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables (1) or disables (0) LDO_1V8_FLASH"]
    #[inline(always)]
    pub fn ldo_1v8_flash_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables (1) or disables (0) LDO_SUPPLY_VBAT and LDO_SUPPLY_USB"]
    #[inline(always)]
    pub fn ldo_3v3_on(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables (1) or disables (0) LDO_CORE"]
    #[inline(always)]
    pub fn ldo_1v2_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LdoCtrl2Reg {
    #[inline(always)]
    fn default() -> LdoCtrl2Reg {
        <crate::RegValueT<LdoCtrl2Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LdoCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for LdoCtrl3Reg_SPEC {
    type DataType = u16;
}
#[doc = "Retention LDO control register"]
pub type LdoCtrl3Reg = crate::RegValueT<LdoCtrl3Reg_SPEC>;

impl LdoCtrl3Reg {
    #[doc = "Setting of this register is \"ORed\" with the vref_hold\ncontrol from the CRG StateMachine.\n\"0\" = CRG controls the T&H of Vref.\n\"1\" = T&H is always in \"Hold\""]
    #[inline(always)]
    pub fn ldo_1v8_pa_ret_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LdoCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LdoCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting of this register is \"ORed\" with the ldo_enable\ncontrol from the CRG StateMachine.\n\"0\" = CRG controls the enable of the LDO.\n\"1\" = LDO is always enabled\nTo activate a retention LDO in \"active-mode\", this bit\nmust be \"1\" and the VREF_HOLD bit must be \"0\"."]
    #[inline(always)]
    pub fn ldo_1v8_pa_ret_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LdoCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LdoCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting of this register is \"ORed\" with the vref_hold\ncontrol from the CRG StateMachine.\n\"0\" = CRG controls the T&H of Vref.\n\"1\" = T&H is always in \"Hold\""]
    #[inline(always)]
    pub fn ldo_1v8_flash_ret_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LdoCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LdoCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting of this register is \"ORed\" with the ldo_enable\ncontrol from the CRG StateMachine.\n\"0\" = CRG controls the enable of the LDO.\n\"1\" = LDO is always enabled\nTo activate a retention LDO in \"active-mode\", this bit\nmust be \"1\" and the VREF_HOLD bit must be \"0\"."]
    #[inline(always)]
    pub fn ldo_1v8_flash_ret_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LdoCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LdoCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting of this register is \"ORed\" with the vref_hold\ncontrol from the CRG StateMachine.\n\"0\" = CRG controls the T&H of Vref.\n\"1\" = T&H is always in \"Hold\""]
    #[inline(always)]
    pub fn ldo_vbat_ret_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LdoCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LdoCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting of this register is \"ORed\" with the ldo_enable\ncontrol from the CRG StateMachine.\n\"0\" = CRG controls the enable of the LDO.\n\"1\" = LDO is always enabled\nTo activate a retention LDO in \"active-mode\", this bit\nmust be \"1\" and the VREF_HOLD bit must be \"0\"."]
    #[inline(always)]
    pub fn ldo_vbat_ret_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LdoCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LdoCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LdoCtrl3Reg {
    #[inline(always)]
    fn default() -> LdoCtrl3Reg {
        <crate::RegValueT<LdoCtrl3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuCtrlReg_SPEC;
impl crate::sealed::RegSpec for PmuCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "Power Management Unit control register"]
pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[doc = "Selects the retainability of the ECC u-Code RAM during deep sleep.\n\'1\' is retainable, \'0\' is power gated"]
    #[inline(always)]
    pub fn retain_eccram(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the retainability of the cache block during deep sleep.\n\'1\' is retainable, \'0\' is power gated"]
    #[inline(always)]
    pub fn retain_cache(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the clockless sleep mode. Wakeup is done asynchronously.\nWhen set to \'1\', the lp_clk is stopped during deep sleep, until a wakeup event (not debounced) is detected by the WAKUPCT block.\nWhen set to \'0\', the lp_clk continues running, so the MAC counters keep on running.\nThis mode cannot be combined with regulated sleep, so keep SLEEP_TIMER=0 when using ENABLE_CLKLESS."]
    #[inline(always)]
    pub fn enable_clkless(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select the retainability of the 5 system memory RAM macros during deep sleep.\n\'1\' is retainable, \'0\' is power gated\n(4) is SYSRAM5\n(3) is SYSRAM4\n(2) is SYSRAM3\n(1) is SYSRAM2\n(0) is SYSRAM1"]
    #[inline(always)]
    pub fn retain_ram(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the HCLK division during OTP mirroring"]
    #[inline(always)]
    pub fn otp_copy_div(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Perform a Hardware Reset after waking up. Booter will be started."]
    #[inline(always)]
    pub fn reset_on_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maps the bandgap_enable to P06"]
    #[inline(always)]
    pub fn map_bandgap_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the FTDF in powerdown"]
    #[inline(always)]
    pub fn ftdf_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the BLE in powerdown.\nWhen the BLE system clock is disabled, either due to the CLK_RADIO_REG\\[BLE_ENABLE\\] or due to the PMU_CTRL_REG\\[BLE_SLEEP\\], then any access to the BLE Register file will issue a hard fault to the CPU."]
    #[inline(always)]
    pub fn ble_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the digital part of the radio in powerdown"]
    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put all peripherals (I2C, UART, SPI, ADC) in powerdown"]
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
pub struct PmuResetRailReg_SPEC;
impl crate::sealed::RegSpec for PmuResetRailReg_SPEC {
    type DataType = u16;
}
#[doc = "Controls rail resetting when RST is pulsed"]
pub type PmuResetRailReg = crate::RegValueT<PmuResetRailReg_SPEC>;

impl PmuResetRailReg {
    #[doc = "1: Enables discharging of the V18P rail when HW reset is pressed\n0: this rail will not be discharged when HW reset is pressed"]
    #[inline(always)]
    pub fn reset_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuResetRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuResetRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: Enables discharging of the V18 rail when HW reset is pressed\n0: this rail will not be discharged when HW reset is pressed"]
    #[inline(always)]
    pub fn reset_v18(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuResetRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuResetRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: Enables discharging of the V14 rail when HW reset is pressed\n0: this rail will not be discharged when HW reset is pressed"]
    #[inline(always)]
    pub fn reset_v14(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PmuResetRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PmuResetRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuResetRailReg {
    #[inline(always)]
    fn default() -> PmuResetRailReg {
        <crate::RegValueT<PmuResetRailReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorVbatCtrlReg_SPEC;
impl crate::sealed::RegSpec for PorVbatCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "Controls the POR on VBAT"]
pub type PorVbatCtrlReg = crate::RegValueT<PorVbatCtrlReg_SPEC>;

impl PorVbatCtrlReg {
    #[doc = "Enables propagation of the generated POR"]
    #[inline(always)]
    pub fn por_vbat_mask_n(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PorVbatCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables generation of the POR"]
    #[inline(always)]
    pub fn por_vbat_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PorVbatCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls hysteresis of POR (20mV/LSB) - Set to 0x0 when non-reset settings are used for POR_VBAT_THRES_LOW and POR_VBAT_THRES_HIGH."]
    #[inline(always)]
    pub fn por_vbat_hyst_low(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "High-side (PTAT) threshold contribution - Align setting with POR_VBAT_THRES_LOW. Set POR_VBAT_HYST_LOW to 0x0, when non-reset settings are used.\nThreshold --> Setting\n1.25V --> 0x0\n1.27V --> 0x1\n1.29V --> 0x2\n1.31V --> 0x3\n1.44V --> 0x4\n1.49V --> 0x5\n1.53V --> 0x6 --> RESET\n1.58V --> 0x7\n1.63V --> 0x8\n1.68V --> 0x9\n1.73V --> 0xA\n1.78V --> 0xB\n1.83V --> 0xC\n1.87V --> 0xD\n1.92V --> 0xE\n1.97V --> 0xF"]
    #[inline(always)]
    pub fn por_vbat_thres_high(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8, PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low-side (CTAT) threshold contribution - Align setting with POR_VBAT_THRES_HIGH. Set POR_VBAT_HYST_LOW to 0x0, when non-reset settings are used.\nThreshold --> Setting\n1.25V --> 0xC\n1.27V --> 0xC\n1.29V --> 0xC\n1.31V --> 0xC\n1.44V --> 0x0\n1.49V --> 0x1\n1.53V --> 0x2\n1.58V --> 0x3\n1.63V --> 0x4\n1.68V --> 0x5\n1.73V --> 0x6\n1.78V --> 0x7\n1.83V --> 0x8\n1.87V --> 0x9\n1.92V --> 0xA\n1.97V --> 0xB\n1.63V -- > 0xF; Reset mode - use only with POR_VBAT_THRES_HIGH=0x6 and POR_VBAT_THRES_HYST=0x2"]
    #[inline(always)]
    pub fn por_vbat_thres_low(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct ResetStatReg_SPEC;
impl crate::sealed::RegSpec for ResetStatReg_SPEC {
    type DataType = u16;
}
#[doc = "Reset status register"]
pub type ResetStatReg = crate::RegValueT<ResetStatReg_SPEC>;

impl ResetStatReg {
    #[doc = "Indicates that a write to SWD_RESET_REG has happened. Note thatit is also set when a POReset has happened."]
    #[inline(always)]
    pub fn swd_hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a Watchdog has happened. Note that it is also set when a POReset has happened."]
    #[inline(always)]
    pub fn wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a SW Reset has happened"]
    #[inline(always)]
    pub fn swreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a HW Reset has happened"]
    #[inline(always)]
    pub fn hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a PowerOn Reset has happened"]
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
        <crate::RegValueT<ResetStatReg_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecureBootReg_SPEC;
impl crate::sealed::RegSpec for SecureBootReg_SPEC {
    type DataType = u16;
}
#[doc = "Controls secure booting"]
pub type SecureBootReg = crate::RegValueT<SecureBootReg_SPEC>;

impl SecureBootReg {
    #[doc = "Follows the respective OTP flag value. Is write-one-only and will be reset by POR only! Its value is updated by the BootROM code.\n1: The system debugger SWD is totally disabled.\n0: The system debugger is enabled with DEBUGGER_ENABLE"]
    #[inline(always)]
    pub fn force_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Follows the respective OTP flag value. Is write-one-only and will be reset by POR only! Its value is updated by the BootROM code.\n1: system is a secure system supporting secure boot\n0: system is not supporting secure boot"]
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
pub struct SleepTimerReg_SPEC;
impl crate::sealed::RegSpec for SleepTimerReg_SPEC {
    type DataType = u16;
}
#[doc = "Timer for regulated sleep"]
pub type SleepTimerReg = crate::RegValueT<SleepTimerReg_SPEC>;

impl SleepTimerReg {
    #[doc = "Defines the amount of ticks of the sleep clock between enabling the bandgap for re-charging the retention LDOs. This value depends on the load and should be calibrated on a per application basis.If set to 0, no recharging cycle will happen at all.\nKeep this value to 0 (no recharging) when using the clockless sleep."]
    #[inline(always)]
    pub fn sleep_timer(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, SleepTimerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, SleepTimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SleepTimerReg {
    #[inline(always)]
    fn default() -> SleepTimerReg {
        <crate::RegValueT<SleepTimerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimCtrlReg_SPEC;
impl crate::sealed::RegSpec for TrimCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "Control trimming of the XTAL16M"]
pub type TrimCtrlReg = crate::RegValueT<TrimCtrlReg_SPEC>;

impl TrimCtrlReg {
    #[doc = "Designates that the XTAL16 can be safely used as the CPU clock. When XTAL16_CLK_CNT reases this value, the signal XTAL_SETTLE_READY will be set"]
    #[inline(always)]
    pub fn xtal_settle_n(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, TrimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select which source controls the XTAL trimming\n0b00: xtal counter. Starts XTAL16M_START_REG, after COUNT_N * 32 xtal pulses trim is changed to CLK_FREQ_TRIM_REG.\n0b01: xtal OK filter. Starts with XTAL16M_START_REG, when xtal is ramping is changed to CLK_FREQ_TRIM_REG\n0b10: statically forced off. Only uses CLK_FREQ_TRIM_REG.\n0b11: xtal OK filter, 2 stage. Starts with XTAL16M_START_REG switches to XTAL16M_RAMP_REG when sw1=\'1\', and switches to CLK_FREQ_TRIM_REG when sw2=\'1\'."]
    #[inline(always)]
    pub fn xtal_trim_select(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, TrimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the number of XTAL cycles to be counted, before the xtal trimming is applied, in steps of 32.\n0x01: 32\n0x02: 64\n0x3f:2016"]
    #[inline(always)]
    pub fn xtal_count_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TrimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TrimCtrlReg {
    #[inline(always)]
    fn default() -> TrimCtrlReg {
        <crate::RegValueT<TrimCtrlReg_SPEC> as RegisterValue<_>>::new(16162)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqClearReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqClearReg_SPEC {
    type DataType = u16;
}
#[doc = "Clear pending IRQ register"]
pub type VbusIrqClearReg = crate::RegValueT<VbusIrqClearReg_SPEC>;

impl VbusIrqClearReg {
    #[doc = "Writing any value to this register will reset the VBUS_IRQ line"]
    #[inline(always)]
    pub fn vbus_irq_clear(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, VbusIrqClearReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, VbusIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for VbusIrqClearReg {
    #[inline(always)]
    fn default() -> VbusIrqClearReg {
        <crate::RegValueT<VbusIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqMaskReg_SPEC {
    type DataType = u16;
}
#[doc = "IRQ masking"]
pub type VbusIrqMaskReg = crate::RegValueT<VbusIrqMaskReg_SPEC>;

impl VbusIrqMaskReg {
    #[doc = "Setting this bit to \'1\' enables VBUS_IRQ generation when the VBUS starts to ramp above threshold"]
    #[inline(always)]
    pub fn vbus_irq_en_rise(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting this bit to \'1\' enables VBUS_IRQ generation when the VBUS starts to fall below threshold"]
    #[inline(always)]
    pub fn vbus_irq_en_fall(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VbusIrqMaskReg {
    #[inline(always)]
    fn default() -> VbusIrqMaskReg {
        <crate::RegValueT<VbusIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XtalrdyCtrlReg_SPEC;
impl crate::sealed::RegSpec for XtalrdyCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "Control register for XTALRDY IRQ"]
pub type XtalrdyCtrlReg = crate::RegValueT<XtalrdyCtrlReg_SPEC>;

impl XtalrdyCtrlReg {
    #[doc = "Number of LP cycles between the crystal is enabled, and the XTALRDY_IRQ is fired.\n0x00: no interrupt"]
    #[inline(always)]
    pub fn xtalrdy_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, XtalrdyCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, XtalrdyCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for XtalrdyCtrlReg {
    #[inline(always)]
    fn default() -> XtalrdyCtrlReg {
        <crate::RegValueT<XtalrdyCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
