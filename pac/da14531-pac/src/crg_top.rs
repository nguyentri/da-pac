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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:19 +0000

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
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

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
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

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

pub type AnaStatusReg = crate::RegValueT<AnaStatusReg_SPEC>;

impl AnaStatusReg {
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

    #[inline(always)]
    pub fn ldo_gpadc_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_xtal_ok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_selected(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_high(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vbat_low(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bandgap_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbat_high_nok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbat_high_ok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_ok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_low_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkAmbaReg_SPEC;
impl crate::sealed::RegSpec for ClkAmbaReg_SPEC {
    type DataType = u16;
}

pub type ClkAmbaReg = crate::RegValueT<ClkAmbaReg_SPEC>;

impl ClkAmbaReg {
    #[inline(always)]
    pub fn otp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type ClkCtrlReg = crate::RegValueT<ClkCtrlReg_SPEC>;

impl ClkCtrlReg {
    #[inline(always)]
    pub fn running_at_xtal32m(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_rc32m(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_lp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_clk_sel(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_disable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
        <crate::RegValueT<ClkCtrlReg_SPEC> as RegisterValue<_>>::new(65)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFreqTrimReg_SPEC;
impl crate::sealed::RegSpec for ClkFreqTrimReg_SPEC {
    type DataType = u16;
}

pub type ClkFreqTrimReg = crate::RegValueT<ClkFreqTrimReg_SPEC>;

impl ClkFreqTrimReg {
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

pub type ClkPerReg = crate::RegValueT<ClkPerReg_SPEC>;

impl ClkPerReg {
    #[inline(always)]
    pub fn quad_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart1_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeupct_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type ClkRadioReg = crate::RegValueT<ClkRadioReg_SPEC>;

impl ClkRadioReg {
    #[inline(always)]
    pub fn ble_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_lp_reset(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type ClkRc32MReg = crate::RegValueT<ClkRc32MReg_SPEC>;

impl ClkRc32MReg {
    #[inline(always)]
    pub fn rc32m_cosc(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, u8, ClkRc32MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xf,1,0,u8,u8,ClkRc32MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32m_range(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, ClkRc32MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,ClkRc32MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32m_bias(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRc32MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRc32MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
pub struct ClkXtal32KReg_SPEC;
impl crate::sealed::RegSpec for ClkXtal32KReg_SPEC {
    type DataType = u16;
}

pub type ClkXtal32KReg = crate::RegValueT<ClkXtal32KReg_SPEC>;

impl ClkXtal32KReg {
    #[inline(always)]
    pub fn xtal32k_xtal1_bias_disable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_disable_ampreg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_cur(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, ClkXtal32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_rbias(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, ClkXtal32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[inline(always)]
    pub fn map_bandgap_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otp_copy_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, PmuCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type PmuSleepReg = crate::RegValueT<PmuSleepReg_SPEC>;

impl PmuSleepReg {
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
pub struct PowerCtrlReg_SPEC;
impl crate::sealed::RegSpec for PowerCtrlReg_SPEC {
    type DataType = u16;
}

pub type PowerCtrlReg = crate::RegValueT<PowerCtrlReg_SPEC>;

impl PowerCtrlReg {
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

    #[inline(always)]
    pub fn por_vbat_low_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cp_disable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vref_hold_force(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_low_ctrl_reg(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, PowerCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_disable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_ret_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbat_hl_connect(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmp_vbat_high_ok_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type PowerLevelReg = crate::RegValueT<PowerLevelReg_SPEC>;

impl PowerLevelReg {
    #[inline(always)]
    pub fn dcdc_trim(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, PowerLevelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

    #[inline(always)]
    pub fn ldo_xtal_trim(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, PowerLevelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,PowerLevelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RamPwrCtrlReg = crate::RegValueT<RamPwrCtrlReg_SPEC>;

impl RamPwrCtrlReg {
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
pub struct SysCtrlReg_SPEC;
impl crate::sealed::RegSpec for SysCtrlReg_SPEC {
    type DataType = u16;
}

pub type SysCtrlReg = crate::RegValueT<SysCtrlReg_SPEC>;

impl SysCtrlReg {
    #[inline(always)]
    pub fn sw_reset(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SysCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,SysCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timeout_disable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn debugger_enable(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otp_copy(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_phase(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type SysStatReg = crate::RegValueT<SysStatReg_SPEC>;

impl SysStatReg {
    #[inline(always)]
    pub fn xtal32m_settled(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_trim_ready(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbg_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_is_down(
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
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(69)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimCtrlReg_SPEC;
impl crate::sealed::RegSpec for TrimCtrlReg_SPEC {
    type DataType = u16;
}

pub type TrimCtrlReg = crate::RegValueT<TrimCtrlReg_SPEC>;

impl TrimCtrlReg {
    #[inline(always)]
    pub fn xtal_settle_n(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal_trim_select(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, TrimCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Xtal32MCtrl0Reg = crate::RegValueT<Xtal32MCtrl0Reg_SPEC>;

impl Xtal32MCtrl0Reg {
    #[inline(always)]
    pub fn xtal32m_spare(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, Xtal32MCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn core_ampl_trim(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Xtal32MCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn core_cur_set(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, Xtal32MCtrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn core_ampl_reg_nullbias(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Xtal32MCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Xtal32MCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Xtal32MStartReg = crate::RegValueT<Xtal32MStartReg_SPEC>;

impl Xtal32MStartReg {
    #[inline(always)]
    pub fn xtal32m_ramp(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Xtal32MStartReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Xtal32MStartReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Xtal32MTrstatReg = crate::RegValueT<Xtal32MTrstatReg_SPEC>;

impl Xtal32MTrstatReg {
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

pub type XtalrdyCtrlReg = crate::RegValueT<XtalrdyCtrlReg_SPEC>;

impl XtalrdyCtrlReg {
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
