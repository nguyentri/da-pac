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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:54:07 +0000

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
    #[doc = "Xtal frequency trimming register."]
    #[inline(always)]
    pub const fn clk_freq_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkFreqTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkFreqTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "System PLL status register."]
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

    #[doc = "Control trimming of the XTAL32M"]
    #[inline(always)]
    pub const fn trim_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TrimCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TrimCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
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
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFreqTrimReg_SPEC;
impl crate::sealed::RegSpec for ClkFreqTrimReg_SPEC {
    type DataType = u32;
}
#[doc = "Xtal frequency trimming register."]
pub type ClkFreqTrimReg = crate::RegValueT<ClkFreqTrimReg_SPEC>;

impl ClkFreqTrimReg {
    #[doc = "Xtal frequency trimming register - START phase of startup\n0x2BF = lowest frequency (high load capacitance)\n0x000 = highest frequency (low load capacitance)\nCload = 5.0p + 6.09p * XTAL32M_TRIM/0x2BF- this includes the PCB parasitic capacitances of the reference desing"]
    #[inline(always)]
    pub fn xtal32m_start(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3ff,1,0,u16, ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Xtal frequency trimming register - RAMP phase of startup.\n0x2BF = lowest frequency (high load capacitance)\n0x000 = highest frequency (low load capacitance)\nCload = 5.0p + 6.09p * XTAL32M_TRIM/0x2BF- this includes the PCB parasitic capacitances of the reference desing"]
    #[inline(always)]
    pub fn xtal32m_ramp(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16, ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Xtal frequency trimming register.\n0x2BF = lowest frequency (high load capacitance)\n0x000 = highest frequency (low load capacitance)\nCload = 5.0p + 6.09p * XTAL32M_TRIM/0x2BF- this includes the PCB parasitic capacitances of the reference desing"]
    #[inline(always)]
    pub fn xtal32m_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkFreqTrimReg {
    #[inline(always)]
    fn default() -> ClkFreqTrimReg {
        <crate::RegValueT<ClkFreqTrimReg_SPEC> as RegisterValue<_>>::new(737869168)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysStatusReg_SPEC;
impl crate::sealed::RegSpec for PllSysStatusReg_SPEC {
    type DataType = u32;
}
#[doc = "System PLL status register."]
pub type PllSysStatusReg = crate::RegValueT<PllSysStatusReg_SPEC>;

impl PllSysStatusReg {
    #[doc = "1: Indicates that LDO PLL is in regulation."]
    #[inline(always)]
    pub fn ldo_pll_ok(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that calibration has finished."]
    #[inline(always)]
    pub fn pll_calibration_end(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Calibrated VCO current."]
    #[inline(always)]
    pub fn pll_best_min_cur(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, PllSysStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3f,1,0,u8, PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: PLL locked"]
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
pub struct TrimCtrlReg_SPEC;
impl crate::sealed::RegSpec for TrimCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Control trimming of the XTAL32M"]
pub type TrimCtrlReg = crate::RegValueT<TrimCtrlReg_SPEC>;

impl TrimCtrlReg {
    #[doc = "Designates that the XTAL can be safely used as the CPU clock. When XTAL_CLK_CNT reases this value, the signal XTAL_SETTLE_READY will be set"]
    #[inline(always)]
    pub fn xtal_settle_n(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, TrimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select which source controls the XTAL trimming\n0b00: xtal counter. Starts CLK_FREQ_TRIM_REG\\[XTAL32M_START\\] after COUNT_N * 32 xtal pulses trim is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\].\n0b01: xtal OK filter. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\], when xtal is ramping is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\].\n0b10: statically forced off. Only uses CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\].\n0b11: xtal OK filter, 2 stage. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\] switches to CLK_FREQ_TRIM_REG\\[XTAL32M_RAMP\\] after timeout (sw1=\'1\', XTAL32M_CTRL0_REG\\[XTAL32M_SW_DELAY\\]), and switches to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\] when sw2=\'1\'."]
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
        <crate::RegValueT<TrimCtrlReg_SPEC> as RegisterValue<_>>::new(1314)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XtalrdyCtrlReg_SPEC;
impl crate::sealed::RegSpec for XtalrdyCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Control register for XTALRDY IRQ"]
pub type XtalrdyCtrlReg = crate::RegValueT<XtalrdyCtrlReg_SPEC>;

impl XtalrdyCtrlReg {
    #[doc = "XTALRDY IRQ timer clock selection:\n0: 32KHz\n1: 256kHz"]
    #[inline(always)]
    pub fn xtalrdy_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, XtalrdyCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,XtalrdyCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of 32kHz or 256kHz cycles between the crystal is enabled, and the XTALRDY_IRQ is fired. Frequency set by XTALRDY_CLK_SEL.\n0x00: no interrupt"]
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
        <crate::RegValueT<XtalrdyCtrlReg_SPEC> as RegisterValue<_>>::new(256)
    }
}
