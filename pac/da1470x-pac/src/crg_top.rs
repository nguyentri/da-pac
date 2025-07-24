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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:52 +0000

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

    #[inline(always)]
    pub const fn bias_vref_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BiasVrefSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BiasVrefSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[doc = "Brown Out Detection control register"]
    #[inline(always)]
    pub const fn bod_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
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

    #[doc = "Clock switching register for CMAC clock domain"]
    #[inline(always)]
    pub const fn clk_cmac_switch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkCmacSwitchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkCmacSwitchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
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

    #[doc = "Fast RC control register"]
    #[inline(always)]
    pub const fn clk_rchs_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRchsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRchsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "32/512 kHz RC oscillator register"]
    #[inline(always)]
    pub const fn clk_rclp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRclpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRclpReg_SPEC, crate::common::RW>::from_ptr(
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

    #[doc = "Divisor for RTC 100Hz clock"]
    #[inline(always)]
    pub const fn clk_rtcdiv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRtcdivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRtcdivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_snc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkSncCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkSncCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
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

    #[doc = "32 kHz XTAL oscillator register"]
    #[inline(always)]
    pub const fn clk_xtal32k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkXtal32KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkXtal32KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
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

    #[inline(always)]
    pub const fn lcd_ext_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdExtCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdExtCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
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

    #[doc = "Control the state retention of the GPIO ports"]
    #[inline(always)]
    pub const fn p2_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2PadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Control the state retention of the GPIO ports"]
    #[inline(always)]
    pub const fn p2_reset_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2ResetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2ResetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Control the state retention of the GPIO ports"]
    #[inline(always)]
    pub const fn p2_set_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2SetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2SetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
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

    #[inline(always)]
    pub const fn pmu_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Controls the POR on VBAT"]
    #[inline(always)]
    pub const fn por_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
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

    #[doc = "Power control register"]
    #[inline(always)]
    pub const fn power_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PowerCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PowerCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn power_lvl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PowerLvlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PowerLvlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
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

    #[doc = "Reset control register"]
    #[inline(always)]
    pub const fn rst_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RstCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RstCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
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

    #[doc = "Map signals on GPIOs during sleep"]
    #[inline(always)]
    pub const fn slp_map_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SlpMapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SlpMapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sw_v18f_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SwV18FReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SwV18FReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "System Control register"]
    #[inline(always)]
    pub const fn sys_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SysCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SysCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
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

    #[doc = "IRQ masking"]
    #[inline(always)]
    pub const fn vbus_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wakeup_hibern_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WakeupHibernReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WakeupHibernReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
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
    #[inline(always)]
    pub fn flag_ldo_v30_combined_ok(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Will be the result of XOR operation of the hibernation wakeup pads outputs combined.\nDFT_EN_INPUT_PD_AON_PADS in TEST_CTRL5_REG must be 1."]
    #[inline(always)]
    pub fn xor_dout_wakeup_pads(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "High when VBUS > ( VBAT + 150 mV). Hysteresis is approx. 40 mV"]
    #[inline(always)]
    pub fn vbus_available(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, ldo_adc is active"]
    #[inline(always)]
    pub fn flag_adc_ldo_ok(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "10nA Iref trimming, high when on-chip current is larger than reference current"]
    #[inline(always)]
    pub fn flag_ibias_trim(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "General output of the BOD to indicate that one of the monitored inputs is below the trigger-level."]
    #[inline(always)]
    pub fn bod_vin_nok(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high bandgap is active"]
    #[inline(always)]
    pub fn bg_ok(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, boost dcdc vled is active"]
    #[inline(always)]
    pub fn boost_dcdc_vled_ok(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "If 1 indicates that temperature of LDO_VSYS is above operating conditions"]
    #[inline(always)]
    pub fn ldo_vsys_high_temp(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0x0 : Hibernation mode or VBUS-only ( sysbat switch opened).\n0x1 : Test mode ( sysbat switch closed).\n0x2 : Ideal diode enabled (with VBAT-only or VBUS and VBAT) .\n0x3 : Sleep mode (with VBAT-only, sysbat switch closed)."]
    #[inline(always)]
    pub fn vbat_vsys_state(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, u8, AnaStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x3,1,0,u8,u8,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, the headroom loop is controlling VSYS"]
    #[inline(always)]
    pub fn ldo_vsys_head_lim(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, the current limiter is controlling VSYS"]
    #[inline(always)]
    pub fn ldo_vsys_curr_lim(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, the voltage loop is controlling VSYS"]
    #[inline(always)]
    pub fn ldo_vsys_lim(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, LDO_VSYS is in regulating mode"]
    #[inline(always)]
    pub fn ldo_vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, ldo_mipi is active"]
    #[inline(always)]
    pub fn ldo_mipi_ok(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When high, ldo_v30 is active"]
    #[inline(always)]
    pub fn ldo_v30_ok(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "V18F switch completely closed"]
    #[inline(always)]
    pub fn switch_v18f_ok(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "V18P Rail ok based on DCDC V18P programmed level"]
    #[inline(always)]
    pub fn buck_dcdc_v18p_ok(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "V18 Rail ok based on DCDC V18 programmed level"]
    #[inline(always)]
    pub fn buck_dcdc_v18_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "V14 Rail ok based on DCDC V14 programmed level"]
    #[inline(always)]
    pub fn buck_dcdc_v14_ok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "V12 Rail ok based on DCDC V12 programmed level"]
    #[inline(always)]
    pub fn buck_dcdc_v12_ok(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VBUS is connected (VBUS > 2.5V)"]
    #[inline(always)]
    pub fn comp_vbus_plugin(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "BOOST_VLED_SEL=0x0 : VSYS>4.3\nBOOST_VLED_SEL=0x1 : VSYS>4.55\nBOOST_VLED_SEL=0x2 : VSYS>4.8\nBOOST_VLED_SEL=0x3 : VSYS>4.8"]
    #[inline(always)]
    pub fn comp_vsys_near_vled(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VBUS>VSYS+0.05V"]
    #[inline(always)]
    pub fn comp_vbus_above_vsys(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VSYS> 2.45V"]
    #[inline(always)]
    pub fn comp_vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VBAT> 2.7V"]
    #[inline(always)]
    pub fn comp_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: VBUS> 4.1V\n0: VBUS<3.4V"]
    #[inline(always)]
    pub fn comp_vbus_ok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_ok(
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
    #[doc = "Enable Temperature compensation in the reference current\nfor Charger and LED module.\n\"1\" = enabled ( = default setting)\n\"0\" = Disabled original currents from BGR are used\ninstead of the Temp-compensated currents"]
    #[inline(always)]
    pub fn en_bgr_tccomp(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables a supply clamp inside the bandgap that improves PSRR. Should be enabled by software after cold boot."]
    #[inline(always)]
    pub fn bandgap_enable_clamp(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim register for bandgap"]
    #[inline(always)]
    pub fn bgr_trim(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RAM Transparent Light Sleep (TLS) Core Enable for System RAMs and Cache RAM. Assert low to enable the TLS core feature, which will result in lower leakage current.\nIn case VDD is below 0.81V, it is necessary to hold this pin high to maintain data retention."]
    #[inline(always)]
    pub fn sysram_lpmx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Current trimming for bias"]
    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BandgapReg {
    #[inline(always)]
    fn default() -> BandgapReg {
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(32800)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BiasVrefSelReg_SPEC;
impl crate::sealed::RegSpec for BiasVrefSelReg_SPEC {
    type DataType = u32;
}

pub type BiasVrefSelReg = crate::RegValueT<BiasVrefSelReg_SPEC>;

impl BiasVrefSelReg {
    #[doc = "same coding as BIAS_VREF_RF1_SEL."]
    #[inline(always)]
    pub fn bias_vref_rf2_sel(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, BiasVrefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,BiasVrefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Vref_code | Vref_Voltage (mV)\n0:900\n1:930\n2:960\n3:990\n4:1020\n5:1050\n6:1080\n7:1110\n8:1140\n9:1170\n10:1200\n11:1230\n12:1260\n13:1290\n14:1320\n15:1350"]
    #[inline(always)]
    pub fn bias_vref_rf1_sel(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, BiasVrefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,BiasVrefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BiasVrefSelReg {
    #[inline(always)]
    fn default() -> BiasVrefSelReg {
        <crate::RegValueT<BiasVrefSelReg_SPEC> as RegisterValue<_>>::new(187)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodCtrlReg_SPEC;
impl crate::sealed::RegSpec for BodCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Brown Out Detection control register"]
pub type BodCtrlReg = crate::RegValueT<BodCtrlReg_SPEC>;

impl BodCtrlReg {
    #[doc = "If set, generate power-on reset on channel VBUS"]
    #[inline(always)]
    pub fn bod_vbus_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel VBAT."]
    #[inline(always)]
    pub fn bod_vbat_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel VMIPI"]
    #[inline(always)]
    pub fn bod_mipi_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel VSYS. Bear in mind that there is an additional configurable POR on VSYS rail, check POR_CTRL_REG."]
    #[inline(always)]
    pub fn bod_vsys_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel V18F"]
    #[inline(always)]
    pub fn bod_v18f_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel V18P"]
    #[inline(always)]
    pub fn bod_v18p_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel V18"]
    #[inline(always)]
    pub fn bod_v18_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel V14"]
    #[inline(always)]
    pub fn bod_v14_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set, generate power-on reset on channel V12"]
    #[inline(always)]
    pub fn bod_v12_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel VBUS"]
    #[inline(always)]
    pub fn bod_vbus_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel VBAT."]
    #[inline(always)]
    pub fn bod_vbat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel VMIPI"]
    #[inline(always)]
    pub fn bod_mipi_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel VSYS. Bear in mind that there is an additional configurable POR on VSYS rail, check POR_CTRL_REG."]
    #[inline(always)]
    pub fn bod_vsys_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel V18F"]
    #[inline(always)]
    pub fn bod_v18f_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel V18P"]
    #[inline(always)]
    pub fn bod_v18p_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel V18"]
    #[inline(always)]
    pub fn bod_v18_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel V14"]
    #[inline(always)]
    pub fn bod_v14_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable brown-out detection for channel VDD (V12)"]
    #[inline(always)]
    pub fn bod_v12_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clears BOD_STATUS_REG when this bit is 1 for more than 2us. It must be 0 to register BOD events in BOD_STATUS_REG."]
    #[inline(always)]
    pub fn bod_status_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BodCtrlReg {
    #[inline(always)]
    fn default() -> BodCtrlReg {
        <crate::RegValueT<BodCtrlReg_SPEC> as RegisterValue<_>>::new(523266)
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
    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_vbus(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_vbat(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_vmipi(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_vsys(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_v18f(
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
    pub fn bod_v14(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: below trigger level (BOD event)\n0: above trigger level"]
    #[inline(always)]
    pub fn bod_v12(
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
    #[doc = "Selects pull value when OQSPIF_D* pads are not output.\n0: The pads are pull-down\n1: The pads are pull-up (to V18F)"]
    #[inline(always)]
    pub fn oqspi_pullup_enable(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If this bit is set, the upper 4 pins of the OQSPIF controller can be used as GPIO, P2_07 to P2_04.\nIn this mode, the OQSPIF controller should be restricted to QUAD mode or less.\nNote: the supply remains V18F, so if the supply is off, the pads become floating."]
    #[inline(always)]
    pub fn oqspi_gpio_mode(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock enable for QSPI RAM controller"]
    #[inline(always)]
    pub fn qspic2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock divider setting.\n0b00: divide by 1\n0b01: divide by 2\n0b10: divide by 4\n0b11: divide by 8"]
    #[inline(always)]
    pub fn qspic2_div(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock enable for QSPI FLASH2 controller"]
    #[inline(always)]
    pub fn qspic_enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock divider setting.\n0b00: divide by 1\n0b01: divide by 2\n0b10: divide by 4\n0b11: divide by 8"]
    #[inline(always)]
    pub fn qspic_div(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock enable for Octal SPI controller"]
    #[inline(always)]
    pub fn oqspif_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock divider setting.\n0b00: divide by 1\n0b01: divide by 2\n0b10: divide by 4\n0b11: divide by 8"]
    #[inline(always)]
    pub fn oqspif_div(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock enable for OTP controller"]
    #[inline(always)]
    pub fn otp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock enable for AES crypto block"]
    #[inline(always)]
    pub fn aes_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Slow-APB interface clock, derived from DIVN_CLK:\n0b000: divide divn_clk by 1\n0b001: divide divn_clk by 2\n0b010: divide divn_clk by 4\n0b011: divide divn_clk by 8\n0b1xx: divide divn_clk by 16"]
    #[inline(always)]
    pub fn slow_pclk_div(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Fast-APB interface clock, Cascaded with HCLK:\n0b00: divide hclk by 1\n0b01: divide hclk by 2\n0b10: divide hclk by 4\n0b11: divide hclk by 8"]
    #[inline(always)]
    pub fn pclk_div(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AHB interface and microprocessor clock. Source clock divided by:\n0b000: divide clk by 1\n0b001: divide clk by 2\n0b010: divide clk by 4\n0b011: divide clk by 8\n0b1xx: divide clk by 16"]
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
        <crate::RegValueT<ClkAmbaReg_SPEC> as RegisterValue<_>>::new(82)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCmacSwitchReg_SPEC;
impl crate::sealed::RegSpec for ClkCmacSwitchReg_SPEC {
    type DataType = u32;
}

#[doc = "Clock switching register for CMAC clock domain"]
pub type ClkCmacSwitchReg = crate::RegValueT<ClkCmacSwitchReg_SPEC>;

impl ClkCmacSwitchReg {
    #[doc = "This bit is \'1\' when the CMAC_CLK is enabled, and the switch is set in the XTAL32M position."]
    #[inline(always)]
    pub fn cmac_running_on_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCmacSwitchReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCmacSwitchReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit is \'1\' when the CMAC_CLK is enabled, and the switch is set in the DIVN_CLK position."]
    #[inline(always)]
    pub fn cmac_running_on_divn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkCmacSwitchReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ClkCmacSwitchReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Selects the clock source of the CMAC_CLK.\n0: DIVN_CLK is selected\n1: XTAL32M is selected.\nNote: this bitfield can only be set to \'1\' when the PD_RAD domain is on (RAD_IS_UP), and will be automatically reset to \'0\' when the PD_RAD power domain turns off."]
    #[inline(always)]
    pub fn cmac_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkCmacSwitchReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkCmacSwitchReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkCmacSwitchReg {
    #[inline(always)]
    fn default() -> ClkCmacSwitchReg {
        <crate::RegValueT<ClkCmacSwitchReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Indicates that the PLL clock is used as clock, and may not be switched off"]
    #[inline(always)]
    pub fn running_at_pll(
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

    #[doc = "Indicates that the RCHS clock is used as clock"]
    #[inline(always)]
    pub fn running_at_rchs(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that the RCLP_CLK is being used as clock"]
    #[inline(always)]
    pub fn running_at_rclp(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Selects the clock for the VAD.\n0: Select RCLP clock (normalized for ~32KHz)\n1: Select XTAL32K"]
    #[inline(always)]
    pub fn vad_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the clock source of the LowerPower clock\n0x0: RC32K\n0x1: RCX\n0x2: XTAL32K through the oscillator with an external Crystal.\n0x3: XTAL32K through an external square wave generator (set PID of GPIO to FUNC_GPIO)"]
    #[inline(always)]
    pub fn lp_clk_sel(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source.\n0x0 : XTAL32M\n0x1 : RCHS\n0x2 : The Low Power clock is used\n0x3 : The PLL160Mhz is used\nNote: switching to/from PLL may only be done from/to XTAL32M."]
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
        <crate::RegValueT<ClkCtrlReg_SPEC> as RegisterValue<_>>::new(8193)
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
    #[doc = "Reset request for registers of the radio PHY."]
    #[inline(always)]
    pub fn rad_reg_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn cmac_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct ClkRchsReg_SPEC;
impl crate::sealed::RegSpec for ClkRchsReg_SPEC {
    type DataType = u32;
}

#[doc = "Fast RC control register"]
pub type ClkRchsReg = crate::RegValueT<ClkRchsReg_SPEC>;

impl ClkRchsReg {
    #[doc = "Selects speed of RCHS output\n0b00: 32MHz, by dividing 96 / 3.\n0b01: 96MHz, by dividing 96 /1.\n0b1x: 64MHz.\nNote: switching to/from 64MHz requires the RCHS to settle, which can be > 100us.\nSwitching 32 to/from 96MHz does not require settling."]
    #[inline(always)]
    pub fn rchs_speed(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Course frequency adjustment"]
    #[inline(always)]
    pub fn rchs_init_range(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Fine frequency adjustment"]
    #[inline(always)]
    pub fn rchs_init_del(
        self,
    ) -> crate::common::RegisterField<12, 0xff, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xff,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Fine duty-cycle adjustment.\n0x0: minimum\n0x2: default\n0x4: maximum\n0x5 until 0x7: oscillator does not work"]
    #[inline(always)]
    pub fn rchs_init_dtcf(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Course duty-cycle adjustment.\n0x0: minimum\n0x5: default\n0xA: maximum\n0xB until 0xF: oscillator does not work"]
    #[inline(always)]
    pub fn rchs_init_dtc(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0xf,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias adjustment"]
    #[inline(always)]
    pub fn rchs_bias(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the HighSpeed RC oscillator"]
    #[inline(always)]
    pub fn rchs_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRchsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRchsReg {
    #[inline(always)]
    fn default() -> ClkRchsReg {
        <crate::RegValueT<ClkRchsReg_SPEC> as RegisterValue<_>>::new(1574062)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRclpReg_SPEC;
impl crate::sealed::RegSpec for ClkRclpReg_SPEC {
    type DataType = u32;
}

#[doc = "32/512 kHz RC oscillator register"]
pub type ClkRclpReg = crate::RegValueT<ClkRclpReg_SPEC>;

impl ClkRclpReg {
    #[doc = "Forces RCLP in 32kHz mode. If this bit is 0 then RCLP frequency is 512KHz"]
    #[inline(always)]
    pub fn rclp_low_speed_force(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkRclpReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkRclpReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0000 = lowest frequency\n0111 = default\n1111 = highest frequency"]
    #[inline(always)]
    pub fn rclp_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRclpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRclpReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the 32kHz RC oscillator.\nThis bit is gated to \'0\' automatically when sleep state is entered, and PMU_CTRL_REG.ENABLE_CLKLESS is set to \'1\'.\nDo not disable this bit, as deepsleep state is not correctly entered."]
    #[inline(always)]
    pub fn rclp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRclpReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRclpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRclpReg {
    #[inline(always)]
    fn default() -> ClkRclpReg {
        <crate::RegValueT<ClkRclpReg_SPEC> as RegisterValue<_>>::new(15)
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
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, ClkRcxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Disable the RCX oscillator (watchdog runs at RCLP)\n1: Enable the RCX oscillator (watchdog runs at RCX)"]
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
pub struct ClkRtcdivReg_SPEC;
impl crate::sealed::RegSpec for ClkRtcdivReg_SPEC {
    type DataType = u32;
}

#[doc = "Divisor for RTC 100Hz clock"]
pub type ClkRtcdivReg = crate::RegValueT<ClkRtcdivReg_SPEC>;

impl ClkRtcdivReg {
    #[doc = "Reset request for the RTC module"]
    #[inline(always)]
    pub fn rtc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable for the 100 Hz generation for the RTC block"]
    #[inline(always)]
    pub fn rtc_div_enable(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the denominator for the fractional division:\n0b0: 1000\n0b1: 1024"]
    #[inline(always)]
    pub fn rtc_div_denom(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Integer divisor part for RTC 100Hz generation"]
    #[inline(always)]
    pub fn rtc_div_int(
        self,
    ) -> crate::common::RegisterField<10, 0x1ff, 1, 0, u16, u16, ClkRtcdivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1ff,
            1,
            0,
            u16,
            u16,
            ClkRtcdivReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Fractional divisor part for RTC 100Hz generation.\nif RTC_DIV_DENOM=1, &lt;RTC_DIV_FRAC&gt; out of 1024 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT>\nIf RTC_DIV_DENOM=0, <RTC_DIV_FRAC> out of 1000 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT>"]
    #[inline(always)]
    pub fn rtc_div_frac(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, ClkRtcdivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRtcdivReg {
    #[inline(always)]
    fn default() -> ClkRtcdivReg {
        <crate::RegValueT<ClkRtcdivReg_SPEC> as RegisterValue<_>>::new(335528)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSncCtrlReg_SPEC;
impl crate::sealed::RegSpec for ClkSncCtrlReg_SPEC {
    type DataType = u32;
}

pub type ClkSncCtrlReg = crate::RegValueT<ClkSncCtrlReg_SPEC>;

impl ClkSncCtrlReg {
    #[doc = "A flag which can be used from FW to indicate that the CPU state has been retained and should be restored during the wakeup sequence (at the beginning of Reset Handler).\nThis flag is retained during the power-down periods."]
    #[inline(always)]
    pub fn snc_state_retained(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkSncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkSncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock-enable for the CM0plus in the SNC."]
    #[inline(always)]
    pub fn snc_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkSncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkSncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Force the SNC microprocessor in reset."]
    #[inline(always)]
    pub fn snc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkSncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkSncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkSncCtrlReg {
    #[inline(always)]
    fn default() -> ClkSncCtrlReg {
        <crate::RegValueT<ClkSncCtrlReg_SPEC> as RegisterValue<_>>::new(1)
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
    #[doc = "Maps Timer2_pwm onto P1_17.\nThis state is preserved during deep sleep, to allow PWM output on the pad during deep sleep."]
    #[inline(always)]
    pub fn tmr2_pwm_aon_mode(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps Timer1_pwm onto P1_00\nThis state is preserved during deep sleep, to allow PWM output on the pad during deep sleep."]
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
pub struct ClkXtal32KReg_SPEC;
impl crate::sealed::RegSpec for ClkXtal32KReg_SPEC {
    type DataType = u32;
}

#[doc = "32 kHz XTAL oscillator register"]
pub type ClkXtal32KReg = crate::RegValueT<ClkXtal32KReg_SPEC>;

impl ClkXtal32KReg {
    #[doc = "Disables output buffer, test only"]
    #[inline(always)]
    pub fn xtal32k_disable_output(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Setting this bit disables the amplitude regulation of the XTAL32kHz oscillator.\nSet this bit to \'1\' for an external clock to XTAL32Kp\nKeep this bit \'0\' with a crystal between XTAL32Kp and XTAL32Km"]
    #[inline(always)]
    pub fn xtal32k_disable_ampreg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias current for the 32kHz XTAL oscillator. 0000 is minimum, 1111 is maximum, 0011 is default. For each application there is an optimal setting for which the start-up behavior is optimal"]
    #[inline(always)]
    pub fn xtal32k_cur(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, ClkXtal32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Setting for the bias resistor. 00 is maximum, 11 is minimum. Prefered setting will be provided by Dialog"]
    #[inline(always)]
    pub fn xtal32k_rbias(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, ClkXtal32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the 32kHz XTAL oscillator"]
    #[inline(always)]
    pub fn xtal32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkXtal32KReg {
    #[inline(always)]
    fn default() -> ClkXtal32KReg {
        <crate::RegValueT<ClkXtal32KReg_SPEC> as RegisterValue<_>>::new(46)
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
    #[doc = "1: Enables immediate discharging of the V30 rail. Note that the source is not disabled.\n0: disable immediate discharging of the V30 rail."]
    #[inline(always)]
    pub fn reset_v30(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: Enables immediate discharging of the VPOD rail. Note that the source is not disabled.\n0: disable immediate discharging of the VPOD rail."]
    #[inline(always)]
    pub fn reset_vpod(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: Enables immediate discharging of the V18P rail. Note that the source is not disabled.\n0: disable immediate discharging of the V18P rail."]
    #[inline(always)]
    pub fn reset_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: Enables immediate discharging of the V18 rail. Note that the source is not disabled.\n0: disable immediate discharging of the V18 rail."]
    #[inline(always)]
    pub fn reset_v18(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: Enables immediate discharging of the V14 rail. Note that the source is not disabled.\n0: disable immediate discharging of the V14 rail."]
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
pub struct LcdExtCtrlReg_SPEC;
impl crate::sealed::RegSpec for LcdExtCtrlReg_SPEC {
    type DataType = u32;
}

pub type LcdExtCtrlReg = crate::RegValueT<LcdExtCtrlReg_SPEC>;

impl LcdExtCtrlReg {
    #[inline(always)]
    pub fn lcd_ext_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdExtCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,LcdExtCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reload value for LCD_EXT_CLK generation.\nWhen the counter hits 0x0, it is reloaded with\nLCD_EXT_CNT_RELOAD<<5.\nSo the clock period is F(slp_clk) * 32 * (LCD_EXT_CNT_RELOAD+1)\nValue 0x0 is not allowed.\nThe LCD_EXT_CLK is generated from the SLP_CLK."]
    #[inline(always)]
    pub fn lcd_ext_cnt_reload(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, LcdExtCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            LcdExtCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdExtCtrlReg {
    #[inline(always)]
    fn default() -> LcdExtCtrlReg {
        <crate::RegValueT<LcdExtCtrlReg_SPEC> as RegisterValue<_>>::new(8)
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
    #[doc = "Direct write to the individual pad latching signals.\nLatches the control signals of the pads for state retention in powerdown mode.\n0 = Control signals are retained\n1 = Latch is transparant, pad can be recontrolled"]
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

#[doc = "Control the state retention of the GPIO ports"]
pub type P1PadLatchReg = crate::RegValueT<P1PadLatchReg_SPEC>;

impl P1PadLatchReg {
    #[doc = "Direct write to the individual pad latching signals.\nLatches the control signals of the pads for state retention in powerdown mode.\n0 = Control signals are retained\n1 = Latch is transparant, pad can be recontrolled"]
    #[inline(always)]
    pub fn p1_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P1PadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
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
        <crate::RegValueT<P1PadLatchReg_SPEC> as RegisterValue<_>>::new(2147483647)
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
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P1ResetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
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

#[doc = "Control the state retention of the GPIO ports"]
pub type P1SetPadLatchReg = crate::RegValueT<P1SetPadLatchReg_SPEC>;

impl P1SetPadLatchReg {
    #[doc = "Direct Set of the marked bits. Reading returns 0x0."]
    #[inline(always)]
    pub fn p1_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P1SetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
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
pub struct P2PadLatchReg_SPEC;
impl crate::sealed::RegSpec for P2PadLatchReg_SPEC {
    type DataType = u32;
}

#[doc = "Control the state retention of the GPIO ports"]
pub type P2PadLatchReg = crate::RegValueT<P2PadLatchReg_SPEC>;

impl P2PadLatchReg {
    #[doc = "Direct write to the individual pad latching signals.\nLatches the control signals of the pads for state retention in powerdown mode.\n0 = Control signals are retained\n1 = Latch is transparant, pad can be recontrolled"]
    #[inline(always)]
    pub fn p2_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        P2PadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            P2PadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2PadLatchReg {
    #[inline(always)]
    fn default() -> P2PadLatchReg {
        <crate::RegValueT<P2PadLatchReg_SPEC> as RegisterValue<_>>::new(32767)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2ResetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P2ResetPadLatchReg_SPEC {
    type DataType = u32;
}

#[doc = "Control the state retention of the GPIO ports"]
pub type P2ResetPadLatchReg = crate::RegValueT<P2ResetPadLatchReg_SPEC>;

impl P2ResetPadLatchReg {
    #[doc = "Direct Reset of the marked bits. Reading returns 0x0."]
    #[inline(always)]
    pub fn p2_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        P2ResetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            P2ResetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2ResetPadLatchReg {
    #[inline(always)]
    fn default() -> P2ResetPadLatchReg {
        <crate::RegValueT<P2ResetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2SetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P2SetPadLatchReg_SPEC {
    type DataType = u32;
}

#[doc = "Control the state retention of the GPIO ports"]
pub type P2SetPadLatchReg = crate::RegValueT<P2SetPadLatchReg_SPEC>;

impl P2SetPadLatchReg {
    #[doc = "Direct Set of the marked bits. Reading returns 0x0."]
    #[inline(always)]
    pub fn p2_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        P2SetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            P2SetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2SetPadLatchReg {
    #[inline(always)]
    fn default() -> P2SetPadLatchReg {
        <crate::RegValueT<P2SetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Retain the R-G-B RAMs inside the LCD display controller"]
    #[inline(always)]
    pub fn retain_rgp_ram(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Retain the GPU CLUT memory"]
    #[inline(always)]
    pub fn retain_gpu_clut(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the retainability of the dcache block while PD_CTRL is off.\n\'1\' is retainable, \'0\' is power gated"]
    #[inline(always)]
    pub fn retain_dcache(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Put the GPU power domain (PD_GPU) in powerdown"]
    #[inline(always)]
    pub fn gpu_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Put the Controller power domain (PD_CTRL) in powerdown"]
    #[inline(always)]
    pub fn ctrl_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

    #[doc = "Put the Communications powerdomain (PD_SNC) in powerdown"]
    #[inline(always)]
    pub fn snc_sleep(
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

    #[doc = "Put the audio power domain (PD_AUD) in powerdown"]
    #[inline(always)]
    pub fn aud_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuCtrlReg {
    #[inline(always)]
    fn default() -> PmuCtrlReg {
        <crate::RegValueT<PmuCtrlReg_SPEC> as RegisterValue<_>>::new(1551)
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
    #[doc = "Allows the core to start running on the RC32M while the PMU is still waiting for supplies to settle to the final value. Only use in combination with FAST_WAKEUP and 0.9 V on VDD during sleep."]
    #[inline(always)]
    pub fn ultra_fast_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables early clock switching upon event detection to speed up wakeup time"]
    #[inline(always)]
    pub fn enable_fast_switch(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This setting determines how long the VLED bypass switch is closed when VSYS near VLED flag and VLED VNOK flag are asserted in sleep. Each 1 LSB represents 31.25us. If VLED_BYPASS_REFRESH_TIME(sec) >= BASE_REFRESH_INTERVAL(sec), then VLED bypass switch will remain closed until VLED VNOK flag is asserted, if VSYS near VLED flag was high when going to sleep. If VLED_BYPASS_REFRESH_TIME = 0, then VLED bypass switch will remain opened in sleep independently on the state of VSYS near VLED comparator, and will only be closed if needed during the boost refresh cycles."]
    #[inline(always)]
    pub fn vled_bypass_refresh_time(
        self,
    ) -> crate::common::RegisterField<23, 0x7f, 1, 0, u8, u8, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x7f,1,0,u8,u8,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\"This is the interval at which the BOD comparators/POR will be checked during sleep. Value represents RAILS_REFRESH_INTERVAL times-> BOD and POR are checked every(sec)=BASE_REFRESH_INTERVAL(sec)*RAILS_REFRESH_INTERVAL*BOD_SLEEP_INTERVAL\nIf 0, BOD/POR will not be checked in sleep.\""]
    #[inline(always)]
    pub fn bod_sleep_interval(
        self,
    ) -> crate::common::RegisterField<19, 0xf, 1, 0, u8, u8, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0xf,1,0,u8,u8,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This is the interval at which the power rails reference voltage will be refreshed during sleep. Value represents BASE_REFRESH_INTERVAL times-> Rails refresh every(sec)=BASE_REFRESH_INTERVAL(sec)*RAILS_REFRESH_INTERVAL.\nIf 0, then voltage reference will not be refreshed in sleep, and BOD/POR will not be checked in sleep."]
    #[inline(always)]
    pub fn rails_refresh_interval(
        self,
    ) -> crate::common::RegisterField<5, 0x3fff, 1, 0, u16, u16, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3fff,1,0,u16,u16,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This sets the base time for calculating the intervals at which PMU refreshes during sleep. In addition, if DCDC_VLED_SLEEP_EN=1, this will be the VLED refresh interval. Each 1 LSB represents 0.125ms. Thus, max base time = 3.875ms.\nIf 0, then there will not be any refresh cycle in SLEEP."]
    #[inline(always)]
    pub fn base_refresh_interval(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuSleepReg {
    #[inline(always)]
    fn default() -> PmuSleepReg {
        <crate::RegValueT<PmuSleepReg_SPEC> as RegisterValue<_>>::new(8920584)
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
    pub fn ibias_10n_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_ret_v30_trim(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_v30_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuTrimReg {
    #[inline(always)]
    fn default() -> PmuTrimReg {
        <crate::RegValueT<PmuTrimReg_SPEC> as RegisterValue<_>>::new(1152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorCtrlReg_SPEC;
impl crate::sealed::RegSpec for PorCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Controls the POR on VBAT"]
pub type PorCtrlReg = crate::RegValueT<PorCtrlReg_SPEC>;

impl PorCtrlReg {
    #[doc = "Enables POR_VSYS during BOD check cycles in sleep (If POR_VSYS_DISABLE = 0)"]
    #[inline(always)]
    pub fn por_vsys_sleep_cycle_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\"Selects POR_VSYS threshold level, when hysteresis is disabled (see POR_VSYS_DISABLE)\n0: Vthres = VTH_L (low level)\n1: Vthres = VTH_H (high level)\""]
    #[inline(always)]
    pub fn por_vsys_hyst_sel(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable POR_VSYS hysteresis."]
    #[inline(always)]
    pub fn por_vsys_hyst_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FORCE POR_VSYS to be ON (also in SLEEP)"]
    #[inline(always)]
    pub fn por_vsys_force_on(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Mask POR on VSYS"]
    #[inline(always)]
    pub fn por_vsys_mask(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable POR_VSYS"]
    #[inline(always)]
    pub fn por_vsys_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables POR_V30 during BOD check cycles in sleep (If POR_V30_DISABLE = 0)"]
    #[inline(always)]
    pub fn por_v30_sleep_cycle_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects POR_VDDA_3V0 threshold level, when hysteresis is disabled (see POR_VDDA_3V0_HYST_DISABLE)\n0: Vthres = VTH_L (low level)\n1: Vthres = VTH_H (high level)"]
    #[inline(always)]
    pub fn por_v30_hyst_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable POR_VDDA_3V0 hysteresis; select level with POR_VDDA_3V0_HYST_SEL"]
    #[inline(always)]
    pub fn por_v30_hyst_disable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Force POR_VDDA_3V0 always ON, (also in SLEEP)"]
    #[inline(always)]
    pub fn por_v30_force_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Mask POR_VDDA_3V0"]
    #[inline(always)]
    pub fn por_v30_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable POR_VDDA_3V0"]
    #[inline(always)]
    pub fn por_v30_disable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorCtrlReg {
    #[inline(always)]
    fn default() -> PorCtrlReg {
        <crate::RegValueT<PorCtrlReg_SPEC> as RegisterValue<_>>::new(2080)
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

    #[doc = "0x00: P0_00\n...\n0x1f: P0_31\n0x20: P1_00\n...\n0x3F: P1_31\n0x40: P2_00\n...\n0x4E: P2_14\n0x4F to 0x7E: reserved\n0x7F: POR generation disabled"]
    #[inline(always)]
    pub fn por_pin_select(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, PorPinReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorPinReg {
    #[inline(always)]
    fn default() -> PorPinReg {
        <crate::RegValueT<PorPinReg_SPEC> as RegisterValue<_>>::new(127)
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
pub struct PowerCtrlReg_SPEC;
impl crate::sealed::RegSpec for PowerCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Power control register"]
pub type PowerCtrlReg = crate::RegValueT<PowerCtrlReg_SPEC>;

impl PowerCtrlReg {
    #[doc = "Closes the V18F switch in sleep when DCDC_V18P_SLEEP_EN is 1. See SW_V18F register for more settings."]
    #[inline(always)]
    pub fn sw_v18f_sleep_on(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Closes the V18F switch when DCDC_V18P_EN is 1. See SW_V18F register for more settings."]
    #[inline(always)]
    pub fn sw_v18f_on(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables boost dcdc led rail in sleep mode"]
    #[inline(always)]
    pub fn dcdc_vled_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables boost dcdc led rail in active mode"]
    #[inline(always)]
    pub fn dcdc_vled_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V18p rail in sleep mode"]
    #[inline(always)]
    pub fn dcdc_v18p_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V18p rail in active mode"]
    #[inline(always)]
    pub fn dcdc_v18p_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V18 rail in sleep mode"]
    #[inline(always)]
    pub fn dcdc_v18_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V18 rail in active mode"]
    #[inline(always)]
    pub fn dcdc_v18_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V14 rail in sleep mode"]
    #[inline(always)]
    pub fn dcdc_v14_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V14 rail in active mode"]
    #[inline(always)]
    pub fn dcdc_v14_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V12 rail in sleep mode"]
    #[inline(always)]
    pub fn dcdc_v12_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables buck dcdc V12 rail in active mode"]
    #[inline(always)]
    pub fn dcdc_v12_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disables V12 clamp. During Hibernation, V12 clamp will be enabled regardless of this bit."]
    #[inline(always)]
    pub fn clamp_v12_dis(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables V30 clamp."]
    #[inline(always)]
    pub fn clamp_v30_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables ldo MIPI when DCDC_V18P_EN is 1"]
    #[inline(always)]
    pub fn ldo_mipi_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables ldo V30 ret in sleep mode"]
    #[inline(always)]
    pub fn ldo_ret_v30_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables ldo V30 ret in active mode"]
    #[inline(always)]
    pub fn ldo_ret_v30_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables ldo V30 in sleep mode"]
    #[inline(always)]
    pub fn ldo_v30_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables ldo V30 in active mode"]
    #[inline(always)]
    pub fn ldo_v30_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables ldo start."]
    #[inline(always)]
    pub fn ldo_start_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PowerCtrlReg {
    #[inline(always)]
    fn default() -> PowerCtrlReg {
        <crate::RegValueT<PowerCtrlReg_SPEC> as RegisterValue<_>>::new(836499)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowerLvlReg_SPEC;
impl crate::sealed::RegSpec for PowerLvlReg_SPEC {
    type DataType = u32;
}

pub type PowerLvlReg = crate::RegValueT<PowerLvlReg_SPEC>;

impl PowerLvlReg {
    #[doc = "Level setting for VSYS rail when ldo_vsys is enabled (COMP_VBUS_OK & COMP_VBUS_ABOVE_VSYS): \n0=4.8V\n1=4.6V\n2=4.4V\n3=4.2V"]
    #[inline(always)]
    pub fn vsys_level(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level setting for V18 rail:\n0=1.8V\n1=1.2V"]
    #[inline(always)]
    pub fn v18_level(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, PowerLvlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level setting for V14 rail:\n0=1.2V\n1=1.3V\n2=1.4V\n3=1.5V"]
    #[inline(always)]
    pub fn v14_level(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level setting for V12 rail in sleep:\n0=0.75V\n1=0.9V\n2=1.2V\n3=reserved"]
    #[inline(always)]
    pub fn v12_sleep_level(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level setting for V12 rail:\n0=0.75V\n1=0.9V\n2=1.2V\n3=reserved"]
    #[inline(always)]
    pub fn v12_level(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level setting for V30 in sleep:\n0x0: 3.0V\n0x1: reserved\n0x2: 3.3V\n0x3: 3.3V"]
    #[inline(always)]
    pub fn v30_sleep_level(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level setting for V30:\n0x0: 3.0V\n0x1: reserved\n0x2: 3.3V\n0x3: 3.3V"]
    #[inline(always)]
    pub fn v30_level(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LDO MIPI level: 0.9V+0.05V*LDO_MIPI_LEVEL. Default=1.2V"]
    #[inline(always)]
    pub fn vmipi_level(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level setting for V12 clamp, retained in hibernation (only V12 source in Hibernation). Typical output voltages (not regulated):"]
    #[inline(always)]
    pub fn clamp_v12_level(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PowerLvlReg {
    #[inline(always)]
    fn default() -> PowerLvlReg {
        <crate::RegValueT<PowerLvlReg_SPEC> as RegisterValue<_>>::new(38960)
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
    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram13_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram12_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram11_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram10_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram9_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram8_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram7_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram6_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram5_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram4_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram3_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram2_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM0_PWR_CTRL."]
    #[inline(always)]
    pub fn ram1_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Power state control of the individual RAMs. May only change when the memory isn\'t accessed.\nWhen PD_MEM_IS_UP:\n0x0: Normal operation\n0x1: Normal operation\n0x2: Retained (no access possible)\n0x3: Off (memory content corrupted)\nWhen PD_MEM_IS_DOWN:\n0x0: Retained\n0x1: Off (memory content corrupted)\n0x2: Retained\n0x3: Off (memory content corrupted)"]
    #[inline(always)]
    pub fn ram0_pwr_ctrl(
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

#[doc = "Reset status register"]
pub type ResetStatReg = crate::RegValueT<ResetStatReg_SPEC>;

impl ResetStatReg {
    #[doc = "Indicates that a SNC-Watchdog timeout has happened. Note that it is also set when a POReset has happened."]
    #[inline(always)]
    pub fn snc_wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        <crate::RegValueT<ResetStatReg_SPEC> as RegisterValue<_>>::new(127)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstCtrlReg_SPEC;
impl crate::sealed::RegSpec for RstCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Reset control register"]
pub type RstCtrlReg = crate::RegValueT<RstCtrlReg_SPEC>;

impl RstCtrlReg {
    #[doc = "0: Flush the System Cache memory only at HW reset.\n1: Flush the System Cache memory also at SW reset."]
    #[inline(always)]
    pub fn sys_cache_flush_with_sw_reset(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RstCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RstCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RstCtrlReg {
    #[inline(always)]
    fn default() -> RstCtrlReg {
        <crate::RegValueT<RstCtrlReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "This bit will permanentlly disable any write action to the CS inside the OTP"]
    #[inline(always)]
    pub fn prot_otp_cs_write(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This bit will permanently disable the SNC debugger"]
    #[inline(always)]
    pub fn force_snc_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This bit will permanently disable CPU read capability at OTP offset 0x00000B00 and for the complete segment"]
    #[inline(always)]
    pub fn prot_oqspif_key_read(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This bit will permanently disable ANY write capability at OTP offset 0x00000B00 and for the complete segment"]
    #[inline(always)]
    pub fn prot_oqspif_key_write(
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
pub struct SlpMapReg_SPEC;
impl crate::sealed::RegSpec for SlpMapReg_SPEC {
    type DataType = u32;
}

#[doc = "Map signals on GPIOs during sleep"]
pub type SlpMapReg = crate::RegValueT<SlpMapReg_SPEC>;

impl SlpMapReg {
    #[doc = "Maps inverted LCD_EXT_CLK on P0_10, for LCD XFRP function\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn lcd_inv_ext_clk_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps LCD_EXT_CLK on P0_19, for LCD VCOM/FRP/EXTCOMIN function\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn lcd_ext_clk_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Setting this bit will:\n\n-map bandgap_enable to P0_13\n-map (wokenup OR cmac_slp_timer_expire) to P1_06"]
    #[inline(always)]
    pub fn bandgap_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps RCLP onto P1_23.\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn rclp_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps XTA32k onto P0_31.\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn xtal32k_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps RCX onto P1_22.\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn rcx_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps Timer4_pwm onto P1_31\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn tmr4_pwm_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps Timer3_pwm onto P1_30\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn tmr3_pwm_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maps Timer1_pwm onto P0_30\nThis state is preserved during deep sleep, to allow pin output toggle on the pad during deep sleep."]
    #[inline(always)]
    pub fn tmr_pwm_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SlpMapReg {
    #[inline(always)]
    fn default() -> SlpMapReg {
        <crate::RegValueT<SlpMapReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwV18FReg_SPEC;
impl crate::sealed::RegSpec for SwV18FReg_SPEC {
    type DataType = u32;
}

pub type SwV18FReg = crate::RegValueT<SwV18FReg_SPEC>;

impl SwV18FReg {
    #[doc = "Soft start delay trim"]
    #[inline(always)]
    pub fn delay_trim(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, SwV18FReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,SwV18FReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Skip soft start routine"]
    #[inline(always)]
    pub fn skip_soft_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SwV18FReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SwV18FReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Forces closing sw_V18f, independent of v18p state"]
    #[inline(always)]
    pub fn force_sw_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SwV18FReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SwV18FReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SwV18FReg {
    #[inline(always)]
    fn default() -> SwV18FReg {
        <crate::RegValueT<SwV18FReg_SPEC> as RegisterValue<_>>::new(4)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrlReg_SPEC;
impl crate::sealed::RegSpec for SysCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "System Control register"]
pub type SysCtrlReg = crate::RegValueT<SysCtrlReg_SPEC>;

impl SysCtrlReg {
    #[doc = "Writing a \'1\' to this bit will generate a SW_RESET."]
    #[inline(always)]
    pub fn sw_reset(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SysCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,SysCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Controls accessiblity of Cache RAM:\n0: the cache controller is bypassed, the cacheRAM is visible in the memory space\n1: the cache controller is enabled, the cacheRAM is not visible anymore in the memory space"]
    #[inline(always)]
    pub fn cacheram_mux(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disables timeout in Power statemachine. By default, the statemachine continues if after 2 ms the blocks are not started up. This can be read back from\nANA_STATUS_REG"]
    #[inline(always)]
    pub fn timeout_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable the debugger. This bit is set by the booter according to the OTP header. If not set, the SWDIO and SW_CLK can be used as gpio ports."]
    #[inline(always)]
    pub fn debugger_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable the CMAC debugger. If not set, the SWDIO and SW_CLK can be used as gpio ports."]
    #[inline(always)]
    pub fn snc_debugger_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: normal operation\n1: If ARM is in address range 0 to 0x1FF then the address is remapped to SYS-RAM 0x0080.0000 to 0x0080.01FF. This allows to put the interrupt vector table to be placed in RAM while executing from QSPI."]
    #[inline(always)]
    pub fn remap_intvect(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls which memory is located at address 0x0000 for execution.\n0x0: ROM\n0x1: OTP un-cached\n0x2: OQSPI FLASH cached (see also the CACHE_FLASH_REG.FLASH_REGION.* descriptions)\n\nNote 1: When REMAP_ADR0=0x2, address 0x0 is mapped to FLASH_REGION_BASE + FLASH_REGION_OFFSET<<2. Note 2: When REMAP_ADR0=0x2, the CPU can only access the Flash region \\[FLASH_REGION_BASE + FLASH_REGION_OFFSET<<2, FLASH_REGION_SIZE\\] from the 0x16000000 address range. The complete Flash can be accessed via the 0x36000000 address range but only uncached. 0x3: RAMS un-cached\n0x4: OQSPI FLASH un-cached (for verification only)\n0x5: SYSRAM3 (for SNC-based applications, where SNC uses SYSRAM 1&2)\n0x6: Cache Data RAM un-cached (CACHERAM_MUX=0, for testing purposes only)\n\n\nNote 1: DWord (64 bits) access is not supported by the Cache Data RAM interface in mirrored mode (only 32, 16 and 8 bits). Note 2: DMA access is not supported by the Cache Data RAM interface when REMAP_ADR0=0x6."]
    #[inline(always)]
    pub fn remap_adr0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SysCtrlReg {
    #[inline(always)]
    fn default() -> SysCtrlReg {
        <crate::RegValueT<SysCtrlReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Indicates that PD_GPU is functional"]
    #[inline(always)]
    pub fn gpu_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_GPU is in power down"]
    #[inline(always)]
    pub fn gpu_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_CTRL is functional"]
    #[inline(always)]
    pub fn ctrl_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_CTRL is in power down"]
    #[inline(always)]
    pub fn ctrl_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

    #[doc = "Indicates that PD_SNC is functional"]
    #[inline(always)]
    pub fn snc_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_SNC is in power down"]
    #[inline(always)]
    pub fn snc_is_down(
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

    #[doc = "Indicates that PD_AUD is functional"]
    #[inline(always)]
    pub fn aud_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_AUD is in power down"]
    #[inline(always)]
    pub fn aud_is_down(
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
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(91557)
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqMaskReg_SPEC {
    type DataType = u32;
}

#[doc = "IRQ masking"]
pub type VbusIrqMaskReg = crate::RegValueT<VbusIrqMaskReg_SPEC>;

impl VbusIrqMaskReg {
    #[doc = "Enables waking up from CLKLESS mode when VBUS becomes available.\nFor the system to wakeup, also the VBUS_IRQ_EN_RISE should be set to trigger the PDC."]
    #[inline(always)]
    pub fn vbus_wakeup_clkless(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
pub struct WakeupHibernReg_SPEC;
impl crate::sealed::RegSpec for WakeupHibernReg_SPEC {
    type DataType = u32;
}

pub type WakeupHibernReg = crate::RegValueT<WakeupHibernReg_SPEC>;

impl WakeupHibernReg {
    #[doc = "Enable hibernation mode"]
    #[inline(always)]
    pub fn hibernation_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, WakeupHibernReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,WakeupHibernReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables pulldown for GPIO\\[n\\] during hibernation\nBit 0: P0_20\nBit 1: P0_29\nBit 2: P1_04\nBit 3: P0_28"]
    #[inline(always)]
    pub fn wakeup_pd_en(
        self,
    ) -> crate::common::RegisterField<6, 0xf, 1, 0, u8, u8, WakeupHibernReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0xf,1,0,u8,u8,WakeupHibernReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables GPIO\\[n\\] to wake up from hibernation\nBit 0: P0_20\nBit 1: P0_29\nBit 2: P1_04\nBit 3: P0_28"]
    #[inline(always)]
    pub fn wakeup_en(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, WakeupHibernReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,WakeupHibernReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WakeupHibernReg {
    #[inline(always)]
    fn default() -> WakeupHibernReg {
        <crate::RegValueT<WakeupHibernReg_SPEC> as RegisterValue<_>>::new(4095)
    }
}
