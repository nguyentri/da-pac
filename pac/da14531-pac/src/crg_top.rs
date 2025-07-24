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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:12 +0000

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

    #[doc = "Status bit of analog (power management) circuits"]
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

    #[doc = "Bandgap trimming"]
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
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Xtal frequency trimming register"]
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

    #[doc = "Peripheral divider register"]
    #[inline(always)]
    pub const fn clk_per_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkPerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkPerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
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

    #[doc = "32 kHz RC oscillator register"]
    #[inline(always)]
    pub const fn clk_rc32k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRc32KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRc32KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Fast RC control register"]
    #[inline(always)]
    pub const fn clk_rc32m_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRc32MReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRc32MReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
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
                self._svd2pac_as_ptr().add(38usize),
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
                self._svd2pac_as_ptr().add(34usize),
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

    #[doc = "Bandgap refresh interval during sleep"]
    #[inline(always)]
    pub const fn pmu_sleep_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuSleepReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuSleepReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
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
                self._svd2pac_as_ptr().add(64usize),
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
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "Power management control"]
    #[inline(always)]
    pub const fn power_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PowerCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PowerCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[doc = "Power management level and trim settings"]
    #[inline(always)]
    pub const fn power_level_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PowerLevelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PowerLevelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
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
                self._svd2pac_as_ptr().add(24usize),
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
                self._svd2pac_as_ptr().add(18usize),
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
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Control trimming of the XTAL32M"]
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

    #[doc = "Control bits for XTAL32M"]
    #[inline(always)]
    pub const fn xtal32m_ctrl0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MCtrl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MCtrl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Trim values for XTAL32M"]
    #[inline(always)]
    pub const fn xtal32m_start_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MStartReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MStartReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Read back value of current XTAL trimming"]
    #[inline(always)]
    pub const fn xtal32m_trstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MTrstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MTrstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
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
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtalrdy_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::XtalrdyStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::XtalrdyStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
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

#[doc = "Status bit of analog (power management) circuits"]
pub type AnaStatusReg = crate::RegValueT<AnaStatusReg_SPEC>;

impl AnaStatusReg {
    #[doc = "Indicates the output of the Clockless wakeup XOR tree. If this signal is \'0\', the chip will wake up.\nUse the HIBERN_WKUP_POLARITY bit to set the value to \'1\' before going into hibernation mode."]
    #[inline(always)]
    pub fn clkless_wakeup_stat(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_running(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that LDO_GPADC output is OK"]
    #[inline(always)]
    pub fn ldo_gpadc_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that LDO_XTAL output is OK"]
    #[inline(always)]
    pub fn ldo_xtal_ok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: Buck mode detected\n1: Boost mode detected"]
    #[inline(always)]
    pub fn boost_selected(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Output of VBAT_HIGH supply rail voltage monitoring circuit.\n0: Voltage level on VBAT_HIGH is lower than POR VBAT_HIGH threshold VTH_L (rail not ok, will result in reset if not masked)\n1: Voltage level on VBAT_HIGH is higher than POR VBAT_HIGH threshold VTH_H (rail ok, reset released)"]
    #[inline(always)]
    pub fn por_vbat_high(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Output of VBAT_LOW supply rail voltage monitoring circuit.\n0: Voltage level on VBAT_LOW is lower than POR VBAT_LOW threshold VTH_L (rail not ok, will result in reset if not masked)\n1: Voltage level on VBAT_LOW is higher than POR VBAT_LOW threshold VTH_H (rail ok, reset released)"]
    #[inline(always)]
    pub fn por_vbat_low(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that BANDGAP is OK"]
    #[inline(always)]
    pub fn bandgap_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that VBAT_HIGH < VBAT_LOW -50 mV"]
    #[inline(always)]
    pub fn comp_vbat_high_nok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that VBAT_HIGH > VBAT_LOW +50 mV"]
    #[inline(always)]
    pub fn comp_vbat_high_ok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that VBAT_LOW (buck mode) or VBAT_HIGH (boost mode) is OK"]
    #[inline(always)]
    pub fn dcdc_ok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that LDO_LOW output is OK\n(only valid for high current mode)"]
    #[inline(always)]
    pub fn ldo_low_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that LDO_CORE output is OK"]
    #[inline(always)]
    pub fn ldo_core_ok(
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
    type DataType = u16;
}

#[doc = "Bandgap trimming"]
pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[doc = "Trim setting for bandgap bias current\n10000 -> -25%\n....\n11111 -> ~0%\n00000 -> ~0% (typ)\n...\n01111 -> +32%"]
    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim setting for bandgap voltage\n10000 -> -6.4%\n....\n11111 -> ~0%\n00000 -> ~0% (typ)\n...\n01111 -> +5.8%"]
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
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Clock enable for OTP controller"]
    #[inline(always)]
    pub fn otp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "APB interface clock (PCLK). Divider is cascaded with HCLK_DIV. PCLK is HCLK divided by:\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8"]
    #[inline(always)]
    pub fn pclk_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AHB interface and microprocessor clock (HCLK). HCLK is source clock divided by:\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8"]
    #[inline(always)]
    pub fn hclk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkAmbaReg {
    #[inline(always)]
    fn default() -> ClkAmbaReg {
        <crate::RegValueT<ClkAmbaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCtrlReg_SPEC;
impl crate::sealed::RegSpec for ClkCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Clock control register"]
pub type ClkCtrlReg = crate::RegValueT<ClkCtrlReg_SPEC>;

impl ClkCtrlReg {
    #[doc = "Indicates that the XTAL32M clock is used as clock, and may not be switched off"]
    #[inline(always)]
    pub fn running_at_xtal32m(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that the RC32M clock is used as clock"]
    #[inline(always)]
    pub fn running_at_rc32m(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that either the LP_CLK is being used as system clock"]
    #[inline(always)]
    pub fn running_at_lp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Sets the clock source of the LowerPower clock\n0x0: RC32K\n0x1: RCX\n0x2: XTAL32K through the oscillator with an external Crystal.\n0x3: XTAL32K through an external square wave generator (set PID of P0\\[3\\] to FUNC_GPIO)\nChange this setting before using this clock, and while RUNNING_AT_LP_CLK == 0."]
    #[inline(always)]
    pub fn lp_clk_sel(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Setting this bit instantaneously disables the 32 MHz crystal oscillator. Also, after sleep/wakeup cycle, the oscillator will not be enabled. This bit may not be set to \'1\'when \"RUNNING_AT_XTAL32M is \'1\' to prevent deadlock. After resetting this bit, wait for XTAL32M_SETTLED or XTAL32M_TRIM_READY to become \'1\' before switching to XTAL32M clock source."]
    #[inline(always)]
    pub fn xtal32m_disable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source.\n0x0: XTAL32M (check the XTAL32M_SETTLED and XTAL32M_TRIM_READY bits!!)\n0x1: RC32M\n0x2/0x3: LP_CLK"]
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
        <crate::RegValueT<ClkCtrlReg_SPEC> as RegisterValue<_>>::new(65)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFreqTrimReg_SPEC;
impl crate::sealed::RegSpec for ClkFreqTrimReg_SPEC {
    type DataType = u16;
}

#[doc = "Xtal frequency trimming register"]
pub type ClkFreqTrimReg = crate::RegValueT<ClkFreqTrimReg_SPEC>;

impl ClkFreqTrimReg {
    #[doc = "Xtal frequency fine trimming register.\n0x00: highest frequency\n0xFF: lowest frequency"]
    #[inline(always)]
    pub fn xtal32m_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct ClkPerReg_SPEC;
impl crate::sealed::RegSpec for ClkPerReg_SPEC {
    type DataType = u16;
}

#[doc = "Peripheral divider register"]
pub type ClkPerReg = crate::RegValueT<ClkPerReg_SPEC>;

impl ClkPerReg {
    #[doc = "Enable the Quadrature clock"]
    #[inline(always)]
    pub fn quad_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable SPI clock"]
    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable UART1 clock"]
    #[inline(always)]
    pub fn uart1_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable UART2 clock"]
    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable I2C clock"]
    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable Wakeup CaptureTimer clock"]
    #[inline(always)]
    pub fn wakeupct_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable TIMER0 and TIMER2 clock"]
    #[inline(always)]
    pub fn tmr_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Division factor for TIMER0\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8"]
    #[inline(always)]
    pub fn tmr_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkPerReg {
    #[inline(always)]
    fn default() -> ClkPerReg {
        <crate::RegValueT<ClkPerReg_SPEC> as RegisterValue<_>>::new(2048)
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
    #[doc = "Enable the BLE core clocks"]
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

    #[doc = "Division factor for BLE core blocks\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8\nThe programmed frequency should not be lower than 8 MHz and not faster than the programmed CPU clock frequency. Refer also to BLE_CNTL2_REG\\[BLE_CLK_SEL\\]."]
    #[inline(always)]
    pub fn ble_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable the RF control Unit clock"]
    #[inline(always)]
    pub fn rfcu_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct ClkRc32KReg_SPEC;
impl crate::sealed::RegSpec for ClkRc32KReg_SPEC {
    type DataType = u16;
}

#[doc = "32 kHz RC oscillator register"]
pub type ClkRc32KReg = crate::RegValueT<ClkRc32KReg_SPEC>;

impl ClkRc32KReg {
    #[doc = "0000 = lowest frequency\n0111 = default\n1111 = highest frequency"]
    #[inline(always)]
    pub fn rc32k_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRc32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRc32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Instantly disables the 32kHz RC oscillator\nSleep cycles cannot happen with this clock disabled."]
    #[inline(always)]
    pub fn rc32k_disable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRc32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRc32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRc32KReg {
    #[inline(always)]
    fn default() -> ClkRc32KReg {
        <crate::RegValueT<ClkRc32KReg_SPEC> as RegisterValue<_>>::new(14)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRc32MReg_SPEC;
impl crate::sealed::RegSpec for ClkRc32MReg_SPEC {
    type DataType = u16;
}

#[doc = "Fast RC control register"]
pub type ClkRc32MReg = crate::RegValueT<ClkRc32MReg_SPEC>;

impl ClkRc32MReg {
    #[doc = "C-adjust of RC-oscillator\nA higher value of COSC results in a lower frequency"]
    #[inline(always)]
    pub fn rc32m_cosc(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, u8, ClkRc32MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xf,1,0,u8,u8,ClkRc32MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Coarse adjust\nA higher value of RANGE results in a higher frequency, values 2 and 3 are equal"]
    #[inline(always)]
    pub fn rc32m_range(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, ClkRc32MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,ClkRc32MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias adjustment"]
    #[inline(always)]
    pub fn rc32m_bias(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRc32MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRc32MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Instantly disables the 32MHz RC oscillator\nDisabling of the oscillator during sleep happens automatically."]
    #[inline(always)]
    pub fn rc32m_disable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRc32MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRc32MReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRc32MReg {
    #[inline(always)]
    fn default() -> ClkRc32MReg {
        <crate::RegValueT<ClkRc32MReg_SPEC> as RegisterValue<_>>::new(1934)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRcxReg_SPEC;
impl crate::sealed::RegSpec for ClkRcxReg_SPEC {
    type DataType = u16;
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
pub struct ClkXtal32KReg_SPEC;
impl crate::sealed::RegSpec for ClkXtal32KReg_SPEC {
    type DataType = u16;
}

#[doc = "32 kHz XTAL oscillator register"]
pub type ClkXtal32KReg = crate::RegValueT<ClkXtal32KReg_SPEC>;

impl ClkXtal32KReg {
    #[inline(always)]
    pub fn xtal32k_xtal1_bias_disable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Enables the 32kHz XTAL oscillator.\nAlso set GP_DATA_REG\\[P03_P04_FILT_DIS\\] = 1 for lowest current consumption."]
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
pub struct PmuCtrlReg_SPEC;
impl crate::sealed::RegSpec for PmuCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Power Management Unit control register"]
pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[doc = "Enable wakeup diagnostics mapping. When set, these functions are mapped (please set direction to output)\nP0\\[2\\]: BANDGAP_ENABLE\nP0\\[1\\]: Power WOKENUP\nNote: P0\\[2\\] assigned also to SWD_CLK, thus the debugger must be detached before entering into sleep mode with MAP_BANDGAP_EN=1. Refer also to SYS_STAT_REG->DBG_IS_UP."]
    #[inline(always)]
    pub fn map_bandgap_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the HCLK division during OTP mirroring"]
    #[inline(always)]
    pub fn otp_copy_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, PmuCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Put the digital part of the radio in powerdown"]
    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Put PD_TIM in powerdown"]
    #[inline(always)]
    pub fn tim_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Perform a Hardware Reset after waking up. Booter will be started."]
    #[inline(always)]
    pub fn reset_on_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuCtrlReg {
    #[inline(always)]
    fn default() -> PmuCtrlReg {
        <crate::RegValueT<PmuCtrlReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuSleepReg_SPEC;
impl crate::sealed::RegSpec for PmuSleepReg_SPEC {
    type DataType = u16;
}

#[doc = "Bandgap refresh interval during sleep"]
pub type PmuSleepReg = crate::RegValueT<PmuSleepReg_SPEC>;

impl PmuSleepReg {
    #[doc = "Defines the refresh interval of reference voltages (bandgap activation and sampling), in units of 2ms."]
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
        <crate::RegValueT<PmuSleepReg_SPEC> as RegisterValue<_>>::new(128)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorPinReg_SPEC;
impl crate::sealed::RegSpec for PorPinReg_SPEC {
    type DataType = u16;
}

#[doc = "Selects a GPIO pin for POR generation"]
pub type PorPinReg = crate::RegValueT<PorPinReg_SPEC>;

impl PorPinReg {
    #[doc = "0: Active Low\n1: Active High\nNote: This applies only for the GPIO pin. Reset pad has a fixed polarity"]
    #[inline(always)]
    pub fn por_pin_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PorPinReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the GPIO which is used for POR generation.\n0x0: GPIO pin POReset disabled\n0x1: P0_0\n0x2: P0_1\n...\n0xB: P0_10\n0xC: P0_11\n0xD - 0xF: reserved"]
    #[inline(always)]
    pub fn por_pin_select(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, PorPinReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorPinReg {
    #[inline(always)]
    fn default() -> PorPinReg {
        <crate::RegValueT<PorPinReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorTimerReg_SPEC;
impl crate::sealed::RegSpec for PorTimerReg_SPEC {
    type DataType = u16;
}

#[doc = "Time for POR to happen"]
pub type PorTimerReg = crate::RegValueT<PorTimerReg_SPEC>;

impl PorTimerReg {
    #[doc = "Time for the POReset to happen.\nFormula:\nTime = POR_TIME x 4096 x RC32k clock period\nDefault value: ~3 seconds\nWhen set to 0x00, the POR TIMER is disabled."]
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
    type DataType = u16;
}

#[doc = "Power management control"]
pub type PowerCtrlReg = crate::RegValueT<PowerCtrlReg_SPEC>;

impl PowerCtrlReg {
    #[doc = "Sets the control mode fo the switch between VBAT_HIGH and VBAT_LOW\n0: Manual (default)\n1: Automatic (boost mode only)"]
    #[inline(always)]
    pub fn vbat_hl_connect_mode(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_high_hyst_dis(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_high_hyst_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable por_vbat_high circuit"]
    #[inline(always)]
    pub fn por_vbat_high_disable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_low_hyst_dis(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_low_hyst_sel(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable por_vbat_low circuit"]
    #[inline(always)]
    pub fn por_vbat_low_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disables LDO_CORE charge-pump circuit"]
    #[inline(always)]
    pub fn cp_disable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Forces LDO references in HOLD mode"]
    #[inline(always)]
    pub fn ldo_vref_hold_force(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "00: High-current mode in active, LDO_LOW OFF in sleep\n01: LDO_LOW OFF\n10: Low-current mode in active, Low-current mode in sleep\n11: High-current mode in active, Low-current mode in sleep"]
    #[inline(always)]
    pub fn ldo_low_ctrl_reg(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, PowerCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disables LDO_CORE"]
    #[inline(always)]
    pub fn ldo_core_disable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LDO_CORE_RETENTION\n0: Disabled\n1: Enabled"]
    #[inline(always)]
    pub fn ldo_core_ret_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Switch between VBAT_HIGH and VBAT_LOW\n0: Open\n1: Closed"]
    #[inline(always)]
    pub fn vbat_hl_connect(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable cmp_vbat_high_ok"]
    #[inline(always)]
    pub fn cmp_vbat_high_ok_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable cmp_vbat_high_nok"]
    #[inline(always)]
    pub fn cmp_vbat_high_nok_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PowerCtrlReg {
    #[inline(always)]
    fn default() -> PowerCtrlReg {
        <crate::RegValueT<PowerCtrlReg_SPEC> as RegisterValue<_>>::new(16384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowerLevelReg_SPEC;
impl crate::sealed::RegSpec for PowerLevelReg_SPEC {
    type DataType = u16;
}

#[doc = "Power management level and trim settings"]
pub type PowerLevelReg = crate::RegValueT<PowerLevelReg_SPEC>;

impl PowerLevelReg {
    #[doc = "Delta from DCDC_LEVEL nominal value\n000: -75 mV\n001: -50 mV\n010: -25 mV\n011: 0 (default)\n100: +25 mV\n101: +50 mV\n110: +75 mV\n111: +100 mV"]
    #[inline(always)]
    pub fn dcdc_trim(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, PowerLevelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "00: 1.1 V\n01: 1.8 V (default)\n10: 2.5 V\n11: 3.0 V"]
    #[inline(always)]
    pub fn dcdc_level(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, u8, PowerLevelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x3,1,0,u8,u8,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_ret_cur_trim(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, PowerLevelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Delta from 0.9 V nominal value\n000: -75 mV\n001: -50 mV\n010: -25 mV\n011: 0 (default)\n100: +25 mV\n101: +50 mV\n110: +75 mV\n111: +100 mV"]
    #[inline(always)]
    pub fn ldo_xtal_trim(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, PowerLevelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Delta from 1.1 V nominal value\n000: -75 mV\n001: -50 mV\n010: -25 mV\n011: 0 (default)\n100: +25 mV\n101: +50 mV\n110: +75 mV\n111: +100 mV (coldboot)"]
    #[inline(always)]
    pub fn ldo_low_trim(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, PowerLevelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_level(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PowerLevelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PowerLevelReg {
    #[inline(always)]
    fn default() -> PowerLevelReg {
        <crate::RegValueT<PowerLevelReg_SPEC> as RegisterValue<_>>::new(6718)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamPwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for RamPwrCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Control power state of System RAMS"]
pub type RamPwrCtrlReg = crate::RegValueT<RamPwrCtrlReg_SPEC>;

impl RamPwrCtrlReg {
    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram3_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See description of RAM1_PWR_CTRL."]
    #[inline(always)]
    pub fn ram2_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Power state control of the individual RAMs. May only change when the memory isn\'t accessed.\nWhen in Active or Sleep mode:\n0x0: Normal operation\n0x1: Normal operation\n0x2: Retained (no access possible)\n0x3: Off (memory content corrupted)\nWhen in Extended Sleep, Deep Sleep or Hibernation mode\n0x0: Retained\n0x1: Off (memory content corrupted)\n0x2: Retained\n0x3: Off (memory content corrupted)"]
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
pub struct SysCtrlReg_SPEC;
impl crate::sealed::RegSpec for SysCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "System Control register"]
pub type SysCtrlReg = crate::RegValueT<SysCtrlReg_SPEC>;

impl SysCtrlReg {
    #[doc = "Writing a \'1\' to this bit will reset the device, except for:\nSYS_CTRL_REG\nCLK_FREQ_TRIM_REG\n..."]
    #[inline(always)]
    pub fn sw_reset(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SysCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,SysCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Disables timeout in Power statemachine. By default, the statemachine continues if after 2 ms the blocks are not started up. This can be read back from\nANA_STATUS_REG."]
    #[inline(always)]
    pub fn timeout_disable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable the debugger. This bit is set by the booter according to the OTP header. If not set, the SWDIO and SW_CLK can be used as gpio ports.\n0x0: no debugger enabled.\n0x1: SW_CLK = P0\\[2\\], SW_DIO=P0\\[5\\]\n0x2: SW_CLK = P0\\[2\\], SW_DIO=P0\\[1\\]\n0x3: SW_CLK = P0\\[2\\], SW_DIO=P0\\[10\\]"]
    #[inline(always)]
    pub fn debugger_enable(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reset request for the OTP controller."]
    #[inline(always)]
    pub fn otpc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables OTP to SysRAM copy action after waking up PD_SYS"]
    #[inline(always)]
    pub fn otp_copy(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the development phase mode.\n\nIf this bit is set, in combination with the OTP_COPY bit, the OTP DMA will emulate the OTP mirroring to System RAM.\nNo actual writing to RAM is done, but the exact same amount of time is spend as if the mirroring would take place. This is to mimic the behavior as if the System Code is already in OTP, and the mirroring takes place after waking up, but the (development) code still resides in an external source.\nIf this bit is set to \'0\' and OTP_COPY=\'1\', then the OTP DMA will actually do the OTP mirroring at wakeup."]
    #[inline(always)]
    pub fn dev_phase(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls which memory is located at address 0x0000 for execution.\n0x0: ROM\n0x1: OTP\n0x2: RAM (SysRAM1)\n0x3: RAM (SysRAM3, 28 kBytes offset)\nThis bitfield only takes affect after a Software Reset."]
    #[inline(always)]
    pub fn remap_adr0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SysCtrlReg {
    #[inline(always)]
    fn default() -> SysCtrlReg {
        <crate::RegValueT<SysCtrlReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysStatReg_SPEC;
impl crate::sealed::RegSpec for SysStatReg_SPEC {
    type DataType = u16;
}

#[doc = "System status register"]
pub type SysStatReg = crate::RegValueT<SysStatReg_SPEC>;

impl SysStatReg {
    #[doc = "Indicates that XTAL32M has had its settle time, as defined by TRIM_CTRL_REG\\[XTAL_SETTLE_N\\]"]
    #[inline(always)]
    pub fn xtal32m_settled(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that XTAL trimming mechanism is ready, i.e. the trimming equals CLK_FREQ_TRIM_REG."]
    #[inline(always)]
    pub fn xtal32m_trim_ready(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that the SW debugger is attached and in connection with the ARM."]
    #[inline(always)]
    pub fn dbg_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_TIM is functional"]
    #[inline(always)]
    pub fn tim_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_TIM is in power down"]
    #[inline(always)]
    pub fn tim_is_down(
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
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(69)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimCtrlReg_SPEC;
impl crate::sealed::RegSpec for TrimCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Control trimming of the XTAL32M"]
pub type TrimCtrlReg = crate::RegValueT<TrimCtrlReg_SPEC>;

impl TrimCtrlReg {
    #[doc = "Designates that the XTAL can be safely used as the CPU clock. When XTAL_CLK_CNT reases this value, the signal XTAL32M_SETTLED bit in the SYS_STAT_REG will be set. Counts in steps of 64 xtal clock-cycles."]
    #[inline(always)]
    pub fn xtal_settle_n(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select which source controls the XTAL trimming\n0b00: xtal counter. Starts XTAL32M_START_REG\\[XTAL32M_START\\] after COUNT_N * 32 xtal pulses trim is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\].\n0b01: xtal OK filter. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\], when xtal amplitude is ramping is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\].\n0b10: statically forced off. Only uses CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\].\n0b11: xtal OK filter, 2 stage. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\] switches to CLK_FREQ_TRIM_REG\\[XTAL32M_RAMP\\] after timeout (32us), and switches to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\] when xtal amplitude is ramping up."]
    #[inline(always)]
    pub fn xtal_trim_select(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the number of XTAL cycles to be counted, before the xtal trimming is applied, in steps of 64 cycles.\n0x01: 64\n0x02: 128\n0x3f: 4032"]
    #[inline(always)]
    pub fn xtal_count_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Xtal32MCtrl0Reg_SPEC;
impl crate::sealed::RegSpec for Xtal32MCtrl0Reg_SPEC {
    type DataType = u16;
}

#[doc = "Control bits for XTAL32M"]
pub type Xtal32MCtrl0Reg = crate::RegValueT<Xtal32MCtrl0Reg_SPEC>;

impl Xtal32MCtrl0Reg {
    #[inline(always)]
    pub fn xtal32m_spare(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Xtal32MCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Core amplitude trimming"]
    #[inline(always)]
    pub fn core_ampl_trim(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Xtal32MCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Core current trim setting"]
    #[inline(always)]
    pub fn core_cur_set(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, Xtal32MCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Keep bias in ampl detector alive, even when there is a large drive"]
    #[inline(always)]
    pub fn core_ampl_reg_nullbias(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Xtal32MCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable dcblock/high pass filter circuit"]
    #[inline(always)]
    pub fn dcblock_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Xtal32MCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MCtrl0Reg {
    #[inline(always)]
    fn default() -> Xtal32MCtrl0Reg {
        <crate::RegValueT<Xtal32MCtrl0Reg_SPEC> as RegisterValue<_>>::new(21)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MStartReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MStartReg_SPEC {
    type DataType = u16;
}

#[doc = "Trim values for XTAL32M"]
pub type Xtal32MStartReg = crate::RegValueT<Xtal32MStartReg_SPEC>;

impl Xtal32MStartReg {
    #[doc = "Xtal frequency trimming register.\n0x00 : highest frequency\n0xFF :lowest frequency"]
    #[inline(always)]
    pub fn xtal32m_ramp(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Xtal32MStartReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Xtal frequency trimming register.\n0x0 = highest frequency\n0xF = lowest frequency."]
    #[inline(always)]
    pub fn xtal32m_start(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xtal32MStartReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MStartReg {
    #[inline(always)]
    fn default() -> Xtal32MStartReg {
        <crate::RegValueT<Xtal32MStartReg_SPEC> as RegisterValue<_>>::new(170)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MTrstatReg_SPEC;
impl crate::sealed::RegSpec for Xtal32MTrstatReg_SPEC {
    type DataType = u16;
}

#[doc = "Read back value of current XTAL trimming"]
pub type Xtal32MTrstatReg = crate::RegValueT<Xtal32MTrstatReg_SPEC>;

impl Xtal32MTrstatReg {
    #[doc = "Reads value of the current XTAL trimming"]
    #[inline(always)]
    pub fn xtal32m_trstat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xtal32MTrstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xtal32MTrstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MTrstatReg {
    #[inline(always)]
    fn default() -> Xtal32MTrstatReg {
        <crate::RegValueT<Xtal32MTrstatReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Number of 32kHz cycles between the crystal is enabled, and the XTALRDY_IRQ is fired. 0x00: no interrupt"]
    #[inline(always)]
    pub fn xtalrdy_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, XtalrdyCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,XtalrdyCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for XtalrdyCtrlReg {
    #[inline(always)]
    fn default() -> XtalrdyCtrlReg {
        <crate::RegValueT<XtalrdyCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XtalrdyStatReg_SPEC;
impl crate::sealed::RegSpec for XtalrdyStatReg_SPEC {
    type DataType = u16;
}

pub type XtalrdyStatReg = crate::RegValueT<XtalrdyStatReg_SPEC>;

impl XtalrdyStatReg {
    #[inline(always)]
    pub fn xtalrdy_stat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, XtalrdyStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,XtalrdyStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for XtalrdyStatReg {
    #[inline(always)]
    fn default() -> XtalrdyStatReg {
        <crate::RegValueT<XtalrdyStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
