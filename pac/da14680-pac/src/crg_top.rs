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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:57 +0000

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

    #[doc = "Brown Out Detection control register"]
    #[inline(always)]
    pub const fn bod_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
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

    #[doc = "16 MHz RC and xtal oscillator register"]
    #[inline(always)]
    pub const fn clk_16m_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Clk16MReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clk16MReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "32 kHz oscillator register"]
    #[inline(always)]
    pub const fn clk_32k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Clk32KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clk32KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
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
    #[doc = "0x1 -> Switch to LDO_SUPPLY_USB on vbus_available & vbus_high & wokenup (SET to 0x1 after boot)\n0x0 -> Switch to LDO_SUPPLY_USB on vbus_available"]
    #[inline(always)]
    pub fn bypass_cold_boot_disable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0x4 --> 1120 mV\n0x5 --> 1089 mV\n0x6 --> 1058 mV\n0x7 --> 1030 mV\n0x0 --> 1037 mV\n0x1 --> 1005 mV\n0x2 --> 978 mV\n0x3 --> 946 mV\n0x8 --> 952 mV\n0x9 --> 918 mV\n0xA --> 889 mV\n0xB --> 861 mV\n0xC --> 862 mV\n0xD --> 828 mV\n0xE --> 798 mV\n0xF --> 770 mV\nThese values are from simulation and vary over corners"]
    #[inline(always)]
    pub fn ldo_sleep_trim(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Current trimming for bias"]
    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim register for bandgap"]
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
pub struct BodCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for BodCtrl2Reg_SPEC {
    type DataType = u16;
}

#[doc = "Brown Out Detection control register"]
pub type BodCtrl2Reg = crate::RegValueT<BodCtrl2Reg_SPEC>;

impl BodCtrl2Reg {
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
        <crate::RegValueT<BodCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodCtrlReg_SPEC;
impl crate::sealed::RegSpec for BodCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Brown Out Detection control register"]
pub type BodCtrlReg = crate::RegValueT<BodCtrlReg_SPEC>;

impl BodCtrlReg {
    #[doc = "VDD BOD Level; 0=700mV; 1=700mV; 3=800mV; 7=1.05V"]
    #[inline(always)]
    pub fn bod_vdd_lvl(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "V33 BOD Trimming bits"]
    #[inline(always)]
    pub fn bod_v33_trim(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1V8 PA BOD Trimming bits"]
    #[inline(always)]
    pub fn bod_1v8_pa_trim(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1V8 Flash BOD Trimming bits"]
    #[inline(always)]
    pub fn bod_1v8_flash_trim(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VDD BOD Trimming bits"]
    #[inline(always)]
    pub fn bod_vdd_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BodCtrlReg {
    #[inline(always)]
    fn default() -> BodCtrlReg {
        <crate::RegValueT<BodCtrlReg_SPEC> as RegisterValue<_>>::new(1877)
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
    #[doc = "Indicates VBAT > VBAT_Trigger"]
    #[inline(always)]
    pub fn bod_vbat_low(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates V33 > V33_Trigger"]
    #[inline(always)]
    pub fn bod_v33_low(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates V18_Flash > V18_Flash_Trigger"]
    #[inline(always)]
    pub fn bod_1v8_flash_low(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates V18_PA > V18_PA_Trigger"]
    #[inline(always)]
    pub fn bod_1v8_pa_low(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates VDD > VDD_Trigger"]
    #[inline(always)]
    pub fn bod_vdd_low(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Clk16MReg_SPEC;
impl crate::sealed::RegSpec for Clk16MReg_SPEC {
    type DataType = u16;
}

#[doc = "16 MHz RC and xtal oscillator register"]
pub type Clk16MReg = crate::RegValueT<Clk16MReg_SPEC>;

impl Clk16MReg {
    #[doc = "Gates the RC16M enable from the startup block.\nThe enable from the clksel and CLK_16M_REG\\[0\\] are not gated by this bit."]
    #[inline(always)]
    pub fn rc16m_startup_disable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "enables high pass filter"]
    #[inline(always)]
    pub fn xtal16_hpass_flt_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "bypasses spikefilter"]
    #[inline(always)]
    pub fn xtal16_spike_flt_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "sets xtal amplitude, 0 is minimum, 101 is maximum"]
    #[inline(always)]
    pub fn xtal16_amp_trim(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, Clk16MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x7,1,0,u8,u8,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Uses the signal on the xtal-p pin as the clock, the xtal-n pin can float."]
    #[inline(always)]
    pub fn xtal16_ext_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Uses the maximum current, for testing purpose only."]
    #[inline(always)]
    pub fn xtal16_max_current(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "start-up current for the 16MHz XTAL oscillator. 000 is minimum, 110 is maximum."]
    #[inline(always)]
    pub fn xtal16_cur_set(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0000 = lowest frequency\n1111 = highest frequency"]
    #[inline(always)]
    pub fn rc16m_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the 16MHz RC oscillator"]
    #[inline(always)]
    pub fn rc16m_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Clk16MReg {
    #[inline(always)]
    fn default() -> Clk16MReg {
        <crate::RegValueT<Clk16MReg_SPEC> as RegisterValue<_>>::new(3072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk32KReg_SPEC;
impl crate::sealed::RegSpec for Clk32KReg_SPEC {
    type DataType = u16;
}

#[doc = "32 kHz oscillator register"]
pub type Clk32KReg = crate::RegValueT<Clk32KReg_SPEC>;

impl Clk32KReg {
    #[doc = "Setting this bit disables the amplitude regulation of the XTAL32kHz oscillator.\nSet this bit to \'1\' for an external clock to XTAL32Kp\nKeep this bit \'0\' with a crystal between XTAL32Kp and XTAL32Km"]
    #[inline(always)]
    pub fn xtal32k_disable_ampreg(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0000 = lowest frequency\n0111 = default\n1111 = highest frequency"]
    #[inline(always)]
    pub fn rc32k_trim(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the 32kHz RC oscillator"]
    #[inline(always)]
    pub fn rc32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias current for the 32kHz XTAL oscillator. 0000 is minimum, 1111 is maximum, 0011 is default. For each application there is an optimal setting for which the start-up behavior is optimal"]
    #[inline(always)]
    pub fn xtal32k_cur(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Setting for the bias resistor. 00 is maximum, 11 is minimum. Prefered setting will be provided by Dialog"]
    #[inline(always)]
    pub fn xtal32k_rbias(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the 32kHz XTAL oscillator"]
    #[inline(always)]
    pub fn xtal32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Clk32KReg {
    #[inline(always)]
    fn default() -> Clk32KReg {
        <crate::RegValueT<Clk32KReg_SPEC> as RegisterValue<_>>::new(1948)
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
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AHB interface and microprocessor clock. Source clock divided by:\n000 = divide hclk by 1\n001 = divide hclk by 2\n010 = divide hclk by 4\n011 = divide hclk by 8\n1xx = divide hclk by 16"]
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
    type DataType = u16;
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

    #[doc = "Indicates that the XTAL16M clock is used as clock, and may not be switched off"]
    #[inline(always)]
    pub fn running_at_xtal16m(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that the RC16M clock is used as clock"]
    #[inline(always)]
    pub fn running_at_rc16m(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that either the RC32k or XTAL32k is being used as clock"]
    #[inline(always)]
    pub fn running_at_32k(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Sets the clock source of the LowerPower clock\n\'00\': 32 Khz RC Oscillator\n\'01\': RCX Oscillator\n\'10\': XTAL32kHz, when using an external crystal i.c.w. the internal oscillator (set P20 and P21 to FUNC_XTAL32)\n\'11\': XTAL32kHz, when an external generator or MCU applies a square wave on P20 (set P20 to FUNC_GPIO)"]
    #[inline(always)]
    pub fn clk32k_source(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Level of the RF divider to sync with in case XTAL32_MODE is set.\nThis is used to align the internal DIVN clock XTAL@32MHz divided by 2 with the radio clock"]
    #[inline(always)]
    pub fn divn_sync_level(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the DIVN divide-by-2, in case of a 32 MHz crystal (See also XTAL32M_MODE), to keep the DIVN clock at 16 MHz."]
    #[inline(always)]
    pub fn divn_xtal32m_mode(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Divides the PLL clock by 2 before being used"]
    #[inline(always)]
    pub fn pll_div2(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the USB source clock\n0 : PLL clock, divided by 2\n1 : HCLK"]
    #[inline(always)]
    pub fn usb_clk_src(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables dividers in the XTAL for both the RF and the BB PLL."]
    #[inline(always)]
    pub fn xtal32m_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Setting this bit instantaneously disables the 16 MHz crystal oscillator. This bit may not be set to \'1\' when \"RUNNING_AT_XTAL16M is \'1\' to prevent deadlock. After resetting this bit, wait for XTAL16_TRIM_READY to become \'1\' before switching to XTAL16 clock source."]
    #[inline(always)]
    pub fn xtal16m_disable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source.\n0x0 : XTAL16M (check the XTAL16_TRIM_READY bit!!)\n0x1 : RC16M\n0x2 : The Low Power clock is used\n0x3 : The PLL96Mhz is used"]
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
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Xtal frequency fine trimming register.0x00 = lowest frequency\n0xFF = highest frequency"]
    #[inline(always)]
    pub fn fine_adj(
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
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
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

    #[doc = "Division factor for RF Control Unit\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8\nThe programmed frequency must be exactly 8MHz."]
    #[inline(always)]
    pub fn rfcu_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkRcx20KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Temperature control"]
    #[inline(always)]
    pub fn rcx20k_ntc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, ClkRcx20KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0000 = lowest frequency\n0111 = default\n1111 = highest frequency"]
    #[inline(always)]
    pub fn rcx20k_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, ClkRcx20KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRcx20KReg {
    #[inline(always)]
    fn default() -> ClkRcx20KReg {
        <crate::RegValueT<ClkRcx20KReg_SPEC> as RegisterValue<_>>::new(376)
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
    #[doc = "Maps Timer1_pwm onto P0_6, when DEBUGGER_EN = \'0\'.\nThis state is preserved during deep sleep, to allow PWM5 output on the pad during deep sleep."]
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
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct LdoCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for LdoCtrl1Reg_SPEC {
    type DataType = u16;
}

#[doc = "LDO control register"]
pub type LdoCtrl1Reg = crate::RegValueT<LdoCtrl1Reg_SPEC>;

impl LdoCtrl1Reg {
    #[doc = "Enables (1) or disables (0) LDO_RADIO (V14)\nFor fast XTAL startup, this bit may be kept to \'1\' during deep sleep. The LDO is switched off automatically when in deep sleep, and enabled when waking up."]
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
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the output voltage of LDO_CORE\n000 = 1.20 V\n001 = 1.15 V\n010 = 1.10 V\n011 = 1.05 V\n1XX = 1.32 V"]
    #[inline(always)]
    pub fn ldo_core_setvdd(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the output voltage of LDO_SUPPLY_USB\n00 = 2.40 V\n01 = 3.30 V\n10 = 3.45 V\n11 = 3.60 V"]
    #[inline(always)]
    pub fn ldo_supply_usb_level(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the output voltage of LDO_SUPPLY_VBAT\n00 = 2.40 V\n01 = 3.30 V\n10 = 3.45 V\n11 = 3.60 V"]
    #[inline(always)]
    pub fn ldo_supply_vbat_level(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the output voltage of LDO_VBAT_RET\n00 = 2.40 V\n01 = 3.30 V\n10 = 3.45 V\n11 = 3.60 V"]
    #[inline(always)]
    pub fn ldo_vbat_ret_level(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the current limit of LDO_CORE\n00 = Current limiter disabled\n01 = 8 mA\n10 = 60 mA\n11 = 80 mA"]
    #[inline(always)]
    pub fn ldo_core_curlim(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct PmuCtrlReg_SPEC;
impl crate::sealed::RegSpec for PmuCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Power Management Unit control register"]
pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[doc = "Selects the retainability of the ECC RAM during deep sleep.\n\'1\' is retainable, \'0\' is power gated"]
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
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, PmuCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the HCLK division during OTP mirroring"]
    #[inline(always)]
    pub fn otp_copy_div(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, PmuCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Perform a Hardware Reset after waking up. Booter will be started."]
    #[inline(always)]
    pub fn reset_on_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SleepTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SleepTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
pub struct SysCtrlReg_SPEC;
impl crate::sealed::RegSpec for SysCtrlReg_SPEC {
    type DataType = u16;
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

    #[doc = "0: normal operation\n1: If ARM is in address range 0 to 0xFF then the address is remapped to SYS-RAM 0x07FC.0000 to 0x07FC.00FF. This allows to put the interrupt vector table to be placed in RAM while executing from QSPI"]
    #[inline(always)]
    pub fn remap_intvect(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables OTP to SysRAM copy action after waking up PD_SYS"]
    #[inline(always)]
    pub fn otp_copy(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables QSPI initialization after wakeup"]
    #[inline(always)]
    pub fn qspi_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the development phase mode, used in combination with OTP_COPY\nNo copy action to SysRAM is done when the system wakes up.\nFor emulating startup time, the OTP_COPY bit still needs to be set."]
    #[inline(always)]
    pub fn dev_phase(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls accessiblity of Cache RAM:\n0: the cache controller is bypassed, the cacheRAM is visible in the memory space next to the DataRAMs\n1: the cache controller is enabled, the cacheRAM is not visible anymore in the memory space"]
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

    #[doc = "Disables the DRA mode, and released the ARM reset"]
    #[inline(always)]
    pub fn dra_off(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable the debugger. This bit is set by the booter according to the OTP header. If not set, the SWDIO and SW_CLK can be used as gpio ports."]
    #[inline(always)]
    pub fn debugger_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reset request for the OTP controller."]
    #[inline(always)]
    pub fn otpc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Latches the control signals of the pads for state retention in powerdown mode.\n0 = Control signals are retained\n1 = Latch is transparant, pad can be recontrolled"]
    #[inline(always)]
    pub fn pad_latch_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the sequence of the 3 first DataRAMs in the memory space. DataRAM4, DataRAM5 and potentially CacheRAM, cannot not be reshuffled.\n0x0: DataRAM1, DataRAM2, DataRAM3\n0x1: DataRAM2, DataRAM1, DataRAM3\n0x2: DataRAM3, DataRAM1, DataRAM2\n0x3: DataRAM3, DataRAM2, DataRAM1"]
    #[inline(always)]
    pub fn remap_rams(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls which memory is located at address 0x0000 for execution.\n0x0: ROM\n0x1: OTP\n0x2: FLASH\n0x3: RAMS (for the exact configuration see REMAP_RAMS)\n0x4: FLASH un-cached (for verification only)\n0x5: OTP un-cached (for verification only)\n0x6: Cache Data RAM (CACHERAM_MUX=0, for testing purposes only)\nNote 1: DWord (64 bits) access is not supported by the Cache Data RAM interface in mirrored mode (only 32, 16 and 8 bits).\nNote 2: DMA access is not supported by the Cache Data RAM interface when REMAP_ADR0=0x6."]
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
    #[doc = "Indicates that PD_DBG is functional"]
    #[inline(always)]
    pub fn ftdf_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_DBG is in power down"]
    #[inline(always)]
    pub fn ftdf_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_DBG is functional"]
    #[inline(always)]
    pub fn ble_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that PD_DBG is in power down"]
    #[inline(always)]
    pub fn ble_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that XTAL trimming mechanism is ready, i.e. the trimming equals CLK_FREQ_TRIM_REG."]
    #[inline(always)]
    pub fn xtal16_trim_ready(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that a debugger is attached."]
    #[inline(always)]
    pub fn dbg_is_active(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
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
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(1349)
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
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        VbusIrqClearReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            VbusIrqClearReg_SPEC,
            crate::common::RW,
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
