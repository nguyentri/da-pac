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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:53:38 +0000

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
    #[doc = "Analog Signals Status Register"]
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

    #[doc = "bandgap trimming"]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = "Clock control register"]
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

    #[doc = "Radio PLL control register"]
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

    #[doc = "32 kHz RC oscillator register"]
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

    #[doc = "RCX-oscillator control register"]
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

    #[doc = "Switches clock from RC32M to XTAL32M"]
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

    #[doc = "Clock control for the timers"]
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

    #[doc = "Immediate rail resetting. There is no LDO/DCDC gating"]
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

    #[doc = "LDO control register"]
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

    #[doc = "Control the state retention of the GPIO ports"]
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

    #[doc = "Control the state retention of the GPIO ports"]
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

    #[doc = "Control the state retention of the GPIO ports"]
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

    #[doc = "Control the state retention of the GPIO ports"]
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

    #[doc = "Control the state retention of the GPIO ports"]
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

    #[doc = "Control the state retention of the GPIO ports"]
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

    #[doc = "Power Management Unit control register"]
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

    #[doc = "Configures the sleep/wakeup strategy"]
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

    #[doc = "LDO trimming register"]
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

    #[doc = "Selects a GPIO pin for POR generation"]
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

    #[doc = "Time for POR to happen"]
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

    #[doc = "Controls the POR on VBAT"]
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

    #[doc = "Control power state of System RAMS"]
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

    #[doc = "Reset status register"]
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

    #[doc = "Controls secure booting"]
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

    #[doc = "System status register"]
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

    #[doc = "Clear pending IRQ register"]
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
#[doc = "Analog Signals Status Register"]
pub type AnaStatusReg = crate::RegValueT<AnaStatusReg_SPEC>;

impl AnaStatusReg {
    #[doc = "COMP_VBUS_HIGH = 1 -> VBUS > 4V"]
    #[inline(always)]
    pub fn comp_vbus_high(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "COMP_VBUS_LOW = 1 -> VBUS > 3.4V"]
    #[inline(always)]
    pub fn comp_vbus_low(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "COMP_VBAT_HIGH =1 -> VBAT > 2.5V"]
    #[inline(always)]
    pub fn comp_vbat_high(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "COMP_VBAT_LOW = 1 -> VBAT > 1.667V"]
    #[inline(always)]
    pub fn comp_vbat_low(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "COMP_VDD_OK = 1 -> VDD > 1.125V"]
    #[inline(always)]
    pub fn comp_vdd_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "High when VBUS > ( VBAT + 150 mV). Hysteresis is approx. 40 mV"]
    #[inline(always)]
    pub fn vbus_available(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high bandgap is active"]
    #[inline(always)]
    pub fn bandgap_ok(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high LDO_VBAT is active"]
    #[inline(always)]
    pub fn ldo_3v0_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high LDO_VBUS is active"]
    #[inline(always)]
    pub fn ldo_3v0_vbus_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high LDO_IO2 is active"]
    #[inline(always)]
    pub fn ldo_1v8p_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high LDO_IO is active"]
    #[inline(always)]
    pub fn ldo_1v8_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high LDO_RADIO is active"]
    #[inline(always)]
    pub fn ldo_radio_ok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high LDO_CORE(LDO1V2) is active"]
    #[inline(always)]
    pub fn ldo_core_ok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When high the ADC LDO is active. This LDO also supplies part of the LRA"]
    #[inline(always)]
    pub fn ldo_vdd_high_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "General output of the BOD to indicate that one of the monitored inputs is below the trigger-level."]
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
#[doc = "bandgap trimming"]
pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[doc = "Enables a supply clamp inside the bandgap that improves PSRR. Should be enabled by software after cold boot."]
    #[inline(always)]
    pub fn bandgap_enable_clamp(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current trimming for bias"]
    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3f,1,0,u8, BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RAM Transparent Light Sleep (TLS) Core Enable for System RAMs. Assert low to enable the TLS core feature, which will result in lower leakage current.\nIn case VDD is below 0.81V, it is necessary to hold this pin high to maintain data retention."]
    #[inline(always)]
    pub fn sysram_lpmx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodLvlCtrl0Reg_SPEC;
impl crate::sealed::RegSpec for BodLvlCtrl0Reg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type BodLvlCtrl0Reg = crate::RegValueT<BodLvlCtrl0Reg_SPEC>;

impl BodLvlCtrl0Reg {
    #[doc = "Brown-out detection level for V18; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.2 * (BOD_LVL+1)/192"]
    #[inline(always)]
    pub fn bod_lvl_v18(
        self,
    ) -> crate::common::RegisterField<18, 0x1ff, 1, 0, u16, BodLvlCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1ff,1,0,u16, BodLvlCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Brown-out detection level for V30; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.2 * (BOD_LVL+1)/192"]
    #[inline(always)]
    pub fn bod_lvl_v30(
        self,
    ) -> crate::common::RegisterField<9, 0x1ff, 1, 0, u16, BodLvlCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1ff,1,0,u16, BodLvlCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Brown-out detection level for VBAT; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.5*(1.2 * (BOD_LVL+1)/192)"]
    #[inline(always)]
    pub fn bod_lvl_vbat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, BodLvlCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, BodLvlCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = ""]
pub type BodLvlCtrl1Reg = crate::RegValueT<BodLvlCtrl1Reg_SPEC>;

impl BodLvlCtrl1Reg {
    #[doc = "Brown-out detection level for VDD in sleep; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.2 * (BOD_LVL+1)/192"]
    #[inline(always)]
    pub fn bod_lvl_vdd_ret(
        self,
    ) -> crate::common::RegisterField<17, 0xff, 1, 0, u8, BodLvlCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0xff,1,0,u8, BodLvlCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Brown-out detection level for VDD in active; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.2 * (BOD_LVL+1)/192"]
    #[inline(always)]
    pub fn bod_lvl_vdd_on(
        self,
    ) -> crate::common::RegisterField<9, 0xff, 1, 0, u8, BodLvlCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xff,1,0,u8, BodLvlCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Brown-out detection level for V18P; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.2 * (BOD_LVL+1)/192"]
    #[inline(always)]
    pub fn bod_lvl_v18p(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, BodLvlCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, BodLvlCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = ""]
pub type BodLvlCtrl2Reg = crate::RegValueT<BodLvlCtrl2Reg_SPEC>;

impl BodLvlCtrl2Reg {
    #[doc = "Brown-out detection level for V14; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.2 * (BOD_LVL+1)/192"]
    #[inline(always)]
    pub fn bod_lvl_v14(
        self,
    ) -> crate::common::RegisterField<9, 0x1ff, 1, 0, u16, BodLvlCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1ff,1,0,u16, BodLvlCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Brown-out detection level for V18F; disable the bod channel before adjusting the level setting.\nVTH_BOD = 1.2 * (BOD_LVL+1)/192"]
    #[inline(always)]
    pub fn bod_lvl_v18f(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, BodLvlCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, BodLvlCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = ""]
pub type BodStatusReg = crate::RegValueT<BodStatusReg_SPEC>;

impl BodStatusReg {
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_v14(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_v18f(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_vdd(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_v18(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_v30(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
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
#[doc = "HCLK, PCLK, divider and clock gates"]
pub type ClkAmbaReg = crate::RegValueT<ClkAmbaReg_SPEC>;

impl ClkAmbaReg {
    #[doc = "Clock enable for QSPI RAM controller"]
    #[inline(always)]
    pub fn qspi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI divider\n00 = divide by 1\n01 = divide by 2\n10 = divide by 4\n11 = divide by 8"]
    #[inline(always)]
    pub fn qspi2_div(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
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
pub struct ClkCtrlReg_SPEC;
impl crate::sealed::RegSpec for ClkCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Clock control register"]
pub type ClkCtrlReg = crate::RegValueT<ClkCtrlReg_SPEC>;

impl ClkCtrlReg {
    #[doc = "Indicates that the PLL96MHz clock is used as clock, and may not be switched off"]
    #[inline(always)]
    pub fn running_at_pll96m(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that the XTAL32M clock is used as clock, and may not be switched off"]
    #[inline(always)]
    pub fn running_at_xtal32m(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that the RC32M clock is used as clock"]
    #[inline(always)]
    pub fn running_at_rc32m(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that either the LP_CLK is being used as clock"]
    #[inline(always)]
    pub fn running_at_lp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Selects the USB source clock\n0 : PLL clock, divided by 2\n1 : HCLK"]
    #[inline(always)]
    pub fn usb_clk_src(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the clock source of the LowerPower clock\n0x0: RC32K\n0x1: RCX\n0x2: XTAL32K through the oscillator with an external Crystal.\n0x3: XTAL32K through an external square wave generator (set PID of GPIO to FUNC_GPIO)"]
    #[inline(always)]
    pub fn lp_clk_sel(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the clock source.\n0x0 : XTAL32M\n0x1 : RC32M\n0x2 : The Low Power clock is used\n0x3 : The PLL96Mhz is used"]
    #[inline(always)]
    pub fn sys_clk_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Radio PLL control register"]
pub type ClkRadioReg = crate::RegValueT<ClkRadioReg_SPEC>;

impl ClkRadioReg {
    #[doc = "Enable the RF control Unit clock"]
    #[inline(always)]
    pub fn rfcu_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Force synchronous reset to CMAC core and Sleep Timer. Its effective only when both Radio and Timer Power Domains are powered and the clocks are enabled.\nCMAC CPU and CMAC registers, including the retained ones, will be reset.\nIt should be kept in reset for enough time to make sure that it will be captured by CMAC, Low Power and APB clocks."]
    #[inline(always)]
    pub fn cmac_synch_reset(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn cmac_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn cmac_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for CMAC\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8"]
    #[inline(always)]
    pub fn cmac_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "32 kHz RC oscillator register"]
pub type ClkRc32KReg = crate::RegValueT<ClkRc32KReg_SPEC>;

impl ClkRc32KReg {
    #[doc = "0000 = lowest frequency\n0111 = default\n1111 = highest frequency"]
    #[inline(always)]
    pub fn rc32k_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, ClkRc32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0xf,1,0,u8, ClkRc32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the 32kHz RC oscillator"]
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
#[doc = "RCX-oscillator control register"]
pub type ClkRcxReg = crate::RegValueT<ClkRcxReg_SPEC>;

impl ClkRcxReg {
    #[doc = "LDO bias current.\n0x0: minimum\n0xF: maximum"]
    #[inline(always)]
    pub fn rcx_bias(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Add unit capacitance to RC-time delay."]
    #[inline(always)]
    pub fn rcx_c0(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Adjust capacitance part of RC-time delay.\n0x00: minimum capacitance\n0x1F: maximum capacitance"]
    #[inline(always)]
    pub fn rcx_cadjust(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Adjust resistance part of RC-time delay. Lower resistance increases power consumption.\n0x0: maximum resistance\n0x1: minimum resistance"]
    #[inline(always)]
    pub fn rcx_radjust(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable the RCX oscillator"]
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
#[doc = "Switches clock from RC32M to XTAL32M"]
pub type ClkSwitch2XtalReg = crate::RegValueT<ClkSwitch2XtalReg_SPEC>;

impl ClkSwitch2XtalReg {
    #[doc = "When writing to this register, the clock switch will happen from RC32M to XTAL32M. If any other clock is selected than RC32M, the selection is discarded."]
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
#[doc = "Clock control for the timers"]
pub type ClkTmrReg = crate::RegValueT<ClkTmrReg_SPEC>;

impl ClkTmrReg {
    #[doc = "Maps Timer2_pwm onto P1_06.\nThis state is preserved during deep sleep, to allow PWM output on the pad during deep sleep."]
    #[inline(always)]
    pub fn tmr2_pwm_aon_mode(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maps Timer1_pwm onto P1_01\nThis state is preserved during deep sleep, to allow PWM output on the pad during deep sleep."]
    #[inline(always)]
    pub fn tmr_pwm_aon_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the clock"]
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
pub struct LdoVdddHighCtrlReg_SPEC;
impl crate::sealed::RegSpec for LdoVdddHighCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "LDO control register"]
pub type LdoVdddHighCtrlReg = crate::RegValueT<LdoVdddHighCtrlReg_SPEC>;

impl LdoVdddHighCtrlReg {
    #[doc = "Disables the low Zout switch. The low Zout switch pulls the output of the LDO to ground. When 0, the output of the LDO is pulled to ground when the LDO is disabled. When 1, the output of the LDO remains floating when the LDO is disabled."]
    #[inline(always)]
    pub fn ldo_vddd_high_low_zout_disable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LdoVdddHighCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables a static load of approx. 10 uA at the output of the LDO VDDD_HIGH."]
    #[inline(always)]
    pub fn ldo_vddd_high_static_load_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LdoVdddHighCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: LDO VDDD_HIGH off,\n1: LDO VDDD_HIGH on."]
    #[inline(always)]
    pub fn ldo_vddd_high_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LdoVdddHighCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LdoVdddHighCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Indicates that the reference input is tracked,\n1: Indicates that the reference input is sampled."]
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
#[doc = "Control the state retention of the GPIO ports"]
pub type P0PadLatchReg = crate::RegValueT<P0PadLatchReg_SPEC>;

impl P0PadLatchReg {
    #[doc = "Direct write to the specific pad_latch_enable signal"]
    #[inline(always)]
    pub fn p0_latch_en(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P0PadLatchReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
#[doc = "Control the state retention of the GPIO ports"]
pub type P0ResetPadLatchReg = crate::RegValueT<P0ResetPadLatchReg_SPEC>;

impl P0ResetPadLatchReg {
    #[doc = "Direct Reset of the marked bits. Reading returns 0x0."]
    #[inline(always)]
    pub fn p0_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Control the state retention of the GPIO ports"]
pub type P0SetPadLatchReg = crate::RegValueT<P0SetPadLatchReg_SPEC>;

impl P0SetPadLatchReg {
    #[doc = "Direct Set of the marked bits. Reading returns 0x0."]
    #[inline(always)]
    pub fn p0_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Control the state retention of the GPIO ports"]
pub type P1PadLatchReg = crate::RegValueT<P1PadLatchReg_SPEC>;

impl P1PadLatchReg {
    #[doc = "Direct write to the specific pad_latch_enable signal"]
    #[inline(always)]
    pub fn p1_latch_en(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffff, 1, 0, u32, P1PadLatchReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fffff,1,0,u32, P1PadLatchReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Control the state retention of the GPIO ports"]
pub type P1ResetPadLatchReg = crate::RegValueT<P1ResetPadLatchReg_SPEC>;

impl P1ResetPadLatchReg {
    #[doc = "Direct Reset of the marked bits. Reading returns 0x0."]
    #[inline(always)]
    pub fn p1_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
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
#[doc = "Control the state retention of the GPIO ports"]
pub type P1SetPadLatchReg = crate::RegValueT<P1SetPadLatchReg_SPEC>;

impl P1SetPadLatchReg {
    #[doc = "Direct Set of the marked bits. Reading returns 0x0."]
    #[inline(always)]
    pub fn p1_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
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
#[doc = "Power Management Unit control register"]
pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[doc = "Selects the clockless sleep mode. Wakeup is done asynchronously.\nWhen set to \'1\', the lp_clk is stopped during deep sleep, until a wakeup event (not debounced) is detected by the WAKUPCT block.\nWhen set to \'0\', the lp_clk continues running, so the MAC counters keep on running.\nThis mode cannot be combined with regulated sleep, so keep SLEEP_TIMER=0 when using ENABLE_CLKLESS."]
    #[inline(always)]
    pub fn enable_clkless(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the retainability of the cache block during deep sleep.\n\'1\' is retainable, \'0\' is power gated"]
    #[inline(always)]
    pub fn retain_cache(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the System powerdomain (PD_SYS) in powerdown.\nIf this bit is \'1\', and there is no pending IRQ in the PDC for the M33, the PD_SYS will be switched off.\nWakeup should be handled by the PDC."]
    #[inline(always)]
    pub fn sys_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Perform a Hardware Reset after waking up. Booter will be started."]
    #[inline(always)]
    pub fn reset_on_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting this bit will:\n\n-map bandgap_enable to P0_25\n-map (wokenup OR cmac_slp_timer_expire) to P0_16"]
    #[inline(always)]
    pub fn map_bandgap_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the Communications powerdomain (PD_COM) in powerdown"]
    #[inline(always)]
    pub fn com_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the Timers Powerdomain (PD_TIM) in powerdown."]
    #[inline(always)]
    pub fn tim_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the digital part of the radio, including CMAC (PD_RAD) in powerdown"]
    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the peripherals power domain (PD_PER) in powerdown"]
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
#[doc = "Configures the sleep/wakeup strategy"]
pub type PmuSleepReg = crate::RegValueT<PmuSleepReg_SPEC>;

impl PmuSleepReg {
    #[doc = "Forces the VDD clamp voltage to its maximum value when waking up from sleep."]
    #[inline(always)]
    pub fn clamp_vdd_wkup_max(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Allows the core to start running on the RC32M while the PMU is still waiting for supplies to settle to the final value. Only use in combination with FAST_WAKEUP and 0.9 V on VDD during sleep."]
    #[inline(always)]
    pub fn ultra_fast_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Speeds up the wakeup process by enabling all LDOs simultaneously instead of in staggered order. Only use if all voltages have been retained during sleep."]
    #[inline(always)]
    pub fn fast_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This is a value defining the interval every which Brown Out Detection is activated to check on the power rails voltage. The value represents BG_REFRESH_INTERVALs"]
    #[inline(always)]
    pub fn bod_sleep_interval(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This is a value defining the interval every which the Bandgap will be activated for refresh. The value represents ticks of lp_clk/64 e.g. 30,5 us * 64 = 1,9 ms."]
    #[inline(always)]
    pub fn bg_refresh_interval(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16, PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "LDO trimming register"]
pub type PmuTrimReg = crate::RegValueT<PmuTrimReg_SPEC>;

impl PmuTrimReg {
    #[doc = "Trim setting for LDO_1V8\nUnsigned binary notation, trim range ±10  percent"]
    #[inline(always)]
    pub fn ldo_1v8_trim(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, PmuTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trim setting for LDO_1V8P\nUnsigned binary notation, trim range ±10  percent"]
    #[inline(always)]
    pub fn ldo_1v8p_trim(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, PmuTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trim setting for LDO_SUPPLY_VBAT\nSign-magnitude notation, trim range ±10  percent"]
    #[inline(always)]
    pub fn ldo_supply_vbat_trim(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, PmuTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trim setting for LDO_SUPPLY_VBUS\nSign-magnitude notation, trim range ±10  percent"]
    #[inline(always)]
    pub fn ldo_supply_vbus_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, PmuTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Selects a GPIO pin for POR generation"]
pub type PorPinReg = crate::RegValueT<PorPinReg_SPEC>;

impl PorPinReg {
    #[doc = "0: Active Low\n1: Active High\nNote: This applies only for the GPIO pin. Reset pad is always active High"]
    #[inline(always)]
    pub fn por_pin_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PorPinReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0x00: P0_00\n...\n0x1f: P0_31\n0x20: P1_00\n...\n0x36: P1_22\n0x37 to 0x3E: reserved\n0x3F: POR generation disabled"]
    #[inline(always)]
    pub fn por_pin_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, PorPinReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Time for POR to happen"]
pub type PorTimerReg = crate::RegValueT<PorTimerReg_SPEC>;

impl PorTimerReg {
    #[doc = "Time for the POReset to happen.\nFormula:\nTime = POR_TIME x 4096 x RC32 clock period\nDefault value: ~3 seconds"]
    #[inline(always)]
    pub fn por_time(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, PorTimerReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, PorTimerReg_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Controls hysteresis of POR. 20mV per step. Must be set to 0x2 when thres_ctrl_low is set to 0xf."]
    #[inline(always)]
    pub fn por_vbat_hyst_low(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "High-side (PTAT) threshold contribution:\nLevel --> Threshold\n0x0 --> 1.25V\n0x1 --> 1.27V\n0x2 --> 1.29V\n0x3 --> 1.31V\n0x4 --> 1.44V\n0x5 --> 1.49V\n0x6 --> 1.53V\n0x7 --> 1.58V\n0x8 --> 1.63V\n0x9 --> 1.68V\n0xA --> 1.73V\n0xB --> 1.78V0xC --> 1.83V0xD --> 1.87V0xE --> 1.92V\n0xF --> 1.97V"]
    #[inline(always)]
    pub fn por_vbat_thres_high(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, PorVbatCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8, PorVbatCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low-side (CTAT) threshold contribution\nLevel --> Threshold\n0xC --> 1.25V\n0xC --> 1.27V\n0xC --> 1.29V\n0xC --> 1.31V\n0x0 --> 1.44V\n0x1 --> 1.49V\n0x2 --> 1.53V\n0x3 --> 1.58V\n0x4 --> 1.63V\n0x5 --> 1.68V\n0x6 --> 1.73V\n0x7 --> 1.78V\n0x8 --> 1.83V\n0x9 --> 1.87V\n0xA --> 1.92V\n0xB --> 1.97V\n0xF --> 1.63V; use only with POR_VBAT_THRES_LOW=0x6 and POR_VBAT_THRES_HYST=0x2"]
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
pub struct RamPwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for RamPwrCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Control power state of System RAMS"]
pub type RamPwrCtrlReg = crate::RegValueT<RamPwrCtrlReg_SPEC>;

impl RamPwrCtrlReg {
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram8_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram7_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram6_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram5_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram4_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram3_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram2_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power state control of the individual RAMs. May only change when the memory isn\'t accessed.\nWhen PD_MEM_IS_UP:\n0x0: Normal operation\n0x1: Normal operation\n0x2: Retained (no access possible)\n0x3: Off (memory content corrupted)\nWhen PD_MEM_IS_DOWN:\n0x0: Retained\n0x1: Off (memory content corrupted)\n0x2: Retained\n0x3: Off (memory content corrupted)"]
    #[inline(always)]
    pub fn ram1_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, RamPwrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Reset status register"]
pub type ResetStatReg = crate::RegValueT<ResetStatReg_SPEC>;

impl ResetStatReg {
    #[doc = "Indicates that a CMAC-Watchdog timeout has happened. Note that it is also set when a POReset has happened."]
    #[inline(always)]
    pub fn cmac_wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a write to SWD_RESET_REG has happened. Note that it is also set when a POReset has happened."]
    #[inline(always)]
    pub fn swd_hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a Watchdog timeout has happened. Note that it is also set when a POReset has happened."]
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
    #[doc = "Indicates that a PowerOn Reset has happened.\nAll bitfields of RESET_STAT_REG should be read (in order to check the source of reset) and then cleared to \'0\', allowing thus the HW to automatically set to \'1\' the proper bitfields during the next reset event."]
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
#[doc = "Controls secure booting"]
pub type SecureBootReg = crate::RegValueT<SecureBootReg_SPEC>;

impl SecureBootReg {
    #[doc = "This bit will permanently disable CPU read capability at OTP offset 0x00000B00 and for the complete segment"]
    #[inline(always)]
    pub fn prot_qspi_key_read(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit will permanently disable ANY write capability at OTP offset 0x00000B00 and for the complete segment"]
    #[inline(always)]
    pub fn prot_qspi_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit will permanently disable CPU read capability at OTP offset 0x00000A00 and for the complete segment"]
    #[inline(always)]
    pub fn prot_aes_key_read(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit will permanently disable ANY write capability at OTP offset 0x00000A00 and for the complete segment"]
    #[inline(always)]
    pub fn prot_aes_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit will permanently disable ANY write capability at OTP offset 0x000008C0 and for the complete segment"]
    #[inline(always)]
    pub fn prot_sig_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit will permanently disable the CMAC debugger"]
    #[inline(always)]
    pub fn force_cmac_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Follows the respective OTP flag value. Its value is updated by the BootROM code.\n1: The system debugger SWD is totally disabled.\n0: The system debugger is enabled with DEBUGGER_ENABLE"]
    #[inline(always)]
    pub fn force_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Follows the respective OTP flag value. Its value is updated by the BootROM code.\n1: system is a secure system supporting secure boot\n0: system is not supporting secure boot"]
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
#[doc = "System status register"]
pub type SysStatReg = crate::RegValueT<SysStatReg_SPEC>;

impl SysStatReg {
    #[doc = "Indicates that the Startup statemachine is finished, and all power regulation is in order.\nIn UltraFastWakeup mode, the SW needs to wait for this signal before starting any heavy traffic."]
    #[inline(always)]
    pub fn power_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that a debugger is attached."]
    #[inline(always)]
    pub fn dbg_is_active(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_COM is functional"]
    #[inline(always)]
    pub fn com_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_COM is in power down"]
    #[inline(always)]
    pub fn com_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_TIM is functional"]
    #[inline(always)]
    pub fn tim_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_TIM is in power down"]
    #[inline(always)]
    pub fn tim_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_MEM is functional"]
    #[inline(always)]
    pub fn mem_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_MEM is in power down"]
    #[inline(always)]
    pub fn mem_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_SYS is functional"]
    #[inline(always)]
    pub fn sys_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_SYS is in power down"]
    #[inline(always)]
    pub fn sys_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_PER is functional"]
    #[inline(always)]
    pub fn per_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_PER is in power down"]
    #[inline(always)]
    pub fn per_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_RAD is functional"]
    #[inline(always)]
    pub fn rad_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_RAD is in power down"]
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
