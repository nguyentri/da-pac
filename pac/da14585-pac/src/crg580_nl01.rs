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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:26:13 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"crg580_nl01 registers"]
unsafe impl ::core::marker::Send for super::Crg580Nl01 {}
unsafe impl ::core::marker::Sync for super::Crg580Nl01 {}
impl super::Crg580Nl01 {
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

    #[doc = "16 MHz RC-oscillator register"]
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

    #[doc = "(in CRG)"]
    #[inline(always)]
    pub const fn rf_io_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfIoCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfIoCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "(in CRG)"]
    #[inline(always)]
    pub const fn rf_lna_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLnaCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLnaCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "(in CRG)"]
    #[inline(always)]
    pub const fn rf_lna_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLnaCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLnaCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "(in CRG)"]
    #[inline(always)]
    pub const fn rf_lna_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLnaCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLnaCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "(in CRG)"]
    #[inline(always)]
    pub const fn rf_rssi_comp_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfRssiCompCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfRssiCompCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_vco_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfVcoCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfVcoCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[doc = "(in CRG)"]
    #[inline(always)]
    pub const fn spotp_test_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpotpTestReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpotpTestReg_SPEC, crate::common::RW>::from_ptr(
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
    #[doc = "Indicates that DCDC is in boost mode"]
    #[inline(always)]
    pub fn boost_selected(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that BANDGAP is OK"]
    #[inline(always)]
    pub fn bandgap_ok(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that VBAT is above threshold while in BOOST converter mode."]
    #[inline(always)]
    pub fn boost_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that LDO_ANA is in regulation. This LDO is used for the general-purpose ADC only"]
    #[inline(always)]
    pub fn ldo_ana_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that LDO_VDD is in regulation"]
    #[inline(always)]
    pub fn ldo_vdd_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that LDO_OTP is in regulation"]
    #[inline(always)]
    pub fn ldo_otp_ok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that VDCDC is above threshold."]
    #[inline(always)]
    pub fn vdcdc_ok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that VBAT1V is above threshold."]
    #[inline(always)]
    pub fn vbat1v_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that VBAT1V is available."]
    #[inline(always)]
    pub fn vbat1v_available(
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
#[doc = "Bandgap trimming"]
pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[doc = "Test-mode, do not use.\nIt disables the bandgap core (voltages will continue for some time, but will slowely drift away)"]
    #[inline(always)]
    pub fn bgr_lowpower(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_ret_trim(
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
pub struct Clk16MReg_SPEC;
impl crate::sealed::RegSpec for Clk16MReg_SPEC {
    type DataType = u16;
}
#[doc = "16 MHz RC-oscillator register"]
pub type Clk16MReg = crate::RegValueT<Clk16MReg_SPEC>;

impl Clk16MReg {
    #[doc = "Enables noise flter in 16 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal16_noise_filt_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables Ibias sample/hold function in 16 MHz crystal oscillator. This bit should be set when the system wake up and reset before entering deep or extended sleep mode."]
    #[inline(always)]
    pub fn xtal16_bias_sh_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bias current for the 16 MHz XTAL oscillator.\n0x0: minimum\n0x7: maximum"]
    #[inline(always)]
    pub fn xtal16_cur_set(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the frequency of the RC16M oscillator.\n0x0: lowest frequency\n0xF: highest frequency"]
    #[inline(always)]
    pub fn rc16m_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0xf,1,0,u8, Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the 16 MHz RC oscillator"]
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
        <crate::RegValueT<Clk16MReg_SPEC> as RegisterValue<_>>::new(160)
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
    #[doc = "Setting this bit disables the amplitude regulation of the XTAL32kHz oscillator.\nSet this bit to \'1\' for an external clock applied at XTAL32Kp.\nKeep this bit \'0\' with a crystal between XTAL32Kp and XTAL32Km."]
    #[inline(always)]
    pub fn xtal32k_disable_ampreg(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the frequency of the RC32K oscillator.\n0x0: lowest frequency\n0x7: default\n0xF: highest frequency"]
    #[inline(always)]
    pub fn rc32k_trim(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the 32 kHz RC oscillator"]
    #[inline(always)]
    pub fn rc32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bias current for the 32kHz XTAL oscillator.\n0x0: minimum\n0x3: default\n0xF: maximum\nFor each application there is an optimal setting for which the startup behaviour is optimal."]
    #[inline(always)]
    pub fn xtal32k_cur(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8, Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting for the bias resistor of the 32 kHz XTAL oscillator.\n0x0: maximum\n0x3: minimum\nPrefered setting will be provided by Dialog."]
    #[inline(always)]
    pub fn xtal32k_rbias(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the 32 kHz XTAL oscillator"]
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
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "AHB interface and microprocessor clock (HCLK). HCLK is source clock divided by:\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8"]
    #[inline(always)]
    pub fn hclk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Indicates that the XTAL16M clock is used as clock, and may not be switched off"]
    #[inline(always)]
    pub fn running_at_xtal16m(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that the RC16M clock is used as clock"]
    #[inline(always)]
    pub fn running_at_rc16m(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that either the RC32k or XTAL32k is being used as clock"]
    #[inline(always)]
    pub fn running_at_32k(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Disable spikefilter in digital clock"]
    #[inline(always)]
    pub fn xtal16m_spike_flt_disable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Setting this bit instantaneously disables the 16 MHz crystal oscillator. Also, after sleep/wakeup cycle, the oscillator will not be enabled. This bit may not be set to \'1\'when \'RUNNING_AT_XTAL16M is \'1\' to prevent deadlock. After resetting this bit, wait for XTAL16_SETTLED or XTAL16_TRIM_READY to become \'1\' before switching to XTAL16 clock source."]
    #[inline(always)]
    pub fn xtal16m_disable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the clock source.\n0x0: XTAL16M (check the XTAL16_SETTLED and XTAL16_TRIM_READY bits!!)\n0x1: RC16M\n0x2/0x3: either RC32k or XTAL32k is used"]
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
        <crate::RegValueT<ClkCtrlReg_SPEC> as RegisterValue<_>>::new(128)
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
    #[doc = "Xtal frequency course trimming register.\n0x0: lowest frequency\n0x7: highest frequencyIncrement or decrement the binary value with 1. Wait approximately 200 us to allow the adjustment to settle."]
    #[inline(always)]
    pub fn coarse_adj(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8, ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Xtal frequency fine trimming register.\n0x00: lowest frequency\n0xFF: highest frequency"]
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
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable SPI clock"]
    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Division factor for SPI\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8"]
    #[inline(always)]
    pub fn spi_div(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Division factor for TIMER0 and TIMER2\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8"]
    #[inline(always)]
    pub fn tmr_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkPerReg {
    #[inline(always)]
    fn default() -> ClkPerReg {
        <crate::RegValueT<ClkPerReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Division factor for RF Control Unit\n0x0: divide by 1\n0x1: divide by 2\n0x2: divide by 4\n0x3: divide by 8\nThe programmed frequency must be exactly 8 MHz."]
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
    #[doc = "Selects RCX oscillator.\n0 : RC32K oscillator\n1: RCX oscillator"]
    #[inline(always)]
    pub fn rcx20k_select(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
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
    #[doc = "Controls the frequency of the RCX oscillator.\n0x0: lowest frequency\n0x7: default\n0xF: highest frequency"]
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
        <crate::RegValueT<ClkRcx20KReg_SPEC> as RegisterValue<_>>::new(376)
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
    #[doc = "Select the retainability of the 4 retention RAM macros.\n\'1\' is retainable, \'0\' is power gated.\n(3) is RETRAM4\n(2) is RETRAM3\n(1) is RETRAM2\n(0) is RETRAM1"]
    #[inline(always)]
    pub fn retention_mode(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Force the DCDC into boost mode at next wakeup.\nSetting this bit reduces the deepsleep current.\nFORCE_BOOST has highest priority.\nWhen either FORCE_BOOST or FORCE_BUCK have been written, these bits cannot be changed."]
    #[inline(always)]
    pub fn force_boost(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Force the DCDC into buck mode at next wakeup.\nSetting this bit reduces the deepsleep current.\nFORCE_BOOST has highest priority.\nWhen either FORCE_BOOST or FORCE_BUCK have been written, these bits cannot be changed."]
    #[inline(always)]
    pub fn force_buck(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the HCLK division during OTP mirroring"]
    #[inline(always)]
    pub fn otp_copy_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put the digital part of the radio in powerdown"]
    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Put all peripherals (I2C, UART, SPI, ADC) in powerdown"]
    #[inline(always)]
    pub fn periph_sleep(
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
pub struct RfIoCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfIoCtrl1Reg_SPEC {
    type DataType = u16;
}
#[doc = "(in CRG)"]
pub type RfIoCtrl1Reg = crate::RegValueT<RfIoCtrl1Reg_SPEC>;

impl RfIoCtrl1Reg {
    #[doc = "Trim the RFIO input capacitance\n00: Minimum capacitance\n10: Nominal capacitance\n1F: Maximal capacitance"]
    #[inline(always)]
    pub fn rfio_trim1_cap(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, RfIoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, RfIoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfIoCtrl1Reg {
    #[inline(always)]
    fn default() -> RfIoCtrl1Reg {
        <crate::RegValueT<RfIoCtrl1Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLnaCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfLnaCtrl1Reg_SPEC {
    type DataType = u16;
}
#[doc = "(in CRG)"]
pub type RfLnaCtrl1Reg = crate::RegValueT<RfLnaCtrl1Reg_SPEC>;

impl RfLnaCtrl1Reg {
    #[doc = "Trim the LNA output capacitance for CN > 19\n00: Minimum capacitance\n10: Nominal capacitance\n1F: Maximal capacitance"]
    #[inline(always)]
    pub fn lna_trim_cd_hf(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, RfLnaCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8, RfLnaCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trim the LNA output capacitance for CN 00: Minimum capacitance\n10: Nominal capacitance\n1F: Maximal capacitance"]
    #[inline(always)]
    pub fn lna_trim_cd_lf(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, RfLnaCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, RfLnaCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLnaCtrl1Reg {
    #[inline(always)]
    fn default() -> RfLnaCtrl1Reg {
        <crate::RegValueT<RfLnaCtrl1Reg_SPEC> as RegisterValue<_>>::new(590)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLnaCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfLnaCtrl2Reg_SPEC {
    type DataType = u16;
}
#[doc = "(in CRG)"]
pub type RfLnaCtrl2Reg = crate::RegValueT<RfLnaCtrl2Reg_SPEC>;

impl RfLnaCtrl2Reg {
    #[doc = "Trim the LNA bias resistor for optimum transcunductance (gain) in AGC settings 2 and 3\n00: Minimum transconductance\n10: Nominal transconductance\n1F: Maximal transconductance"]
    #[inline(always)]
    pub fn lna_trim_gm_lo(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, RfLnaCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8, RfLnaCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trim the LNA bias resistor for optimum transcunductance (gain) in AGC settings 0 and 1\n00: Minimum transconductance\n10: Nominal transconductance\n1F: Maximal transconductance"]
    #[inline(always)]
    pub fn lna_trim_gm_hi(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, RfLnaCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, RfLnaCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLnaCtrl2Reg {
    #[inline(always)]
    fn default() -> RfLnaCtrl2Reg {
        <crate::RegValueT<RfLnaCtrl2Reg_SPEC> as RegisterValue<_>>::new(550)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLnaCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for RfLnaCtrl3Reg_SPEC {
    type DataType = u16;
}
#[doc = "(in CRG)"]
pub type RfLnaCtrl3Reg = crate::RegValueT<RfLnaCtrl3Reg_SPEC>;

impl RfLnaCtrl3Reg {
    #[doc = "Trim the LNA gate-source capacitance\n00: Minimum capacitance\n10: Nominal capacitance\n1F: Maximal capacitance"]
    #[inline(always)]
    pub fn lna_trim_cgs(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLnaCtrl3Reg {
    #[inline(always)]
    fn default() -> RfLnaCtrl3Reg {
        <crate::RegValueT<RfLnaCtrl3Reg_SPEC> as RegisterValue<_>>::new(7)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfRssiCompCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfRssiCompCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "(in CRG)"]
pub type RfRssiCompCtrlReg = crate::RegValueT<RfRssiCompCtrlReg_SPEC>;

impl RfRssiCompCtrlReg {
    #[doc = "RSSI compensation value for LNA gain setting 00\n\'0x0\': -8\n\'0x1\': -7\n\'0x2\': -6\n\'0x3\': -5\n\'0x4\': -4\n\'0x5\': -3\n\'0x6\': -2\n\'0x7\': -1\n\'0x8\': 0 (reset)\n\'0x9\': 1\n\'0xA\': 2\n\'0xB\': 3\n\'0xC\': 4\n\'0xD\': 5\n\'0xE\': 6\n\'0xF\': 7"]
    #[inline(always)]
    pub fn rssi_comp00(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, RfRssiCompCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8, RfRssiCompCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RSSI compensation value for LNA gain setting 11 relative to 00\nCoding identical to RSSI_COMP01."]
    #[inline(always)]
    pub fn rssi_comp11(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, RfRssiCompCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, RfRssiCompCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RSSI compensation value for LNA gain setting 10 relative to 00\nCoding identical to RSSI_COMP01."]
    #[inline(always)]
    pub fn rssi_comp10(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, RfRssiCompCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8, RfRssiCompCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RSSI compensation value for LNA gain setting 01 relative to 00\n\'0x0\': -4\n\'0x1\': -3\n\'0x2\': -2\n\'0x3\': -1\n\'0x4\': 0\n\'0x5\': 1\n\'0x6\': 2\n\'0x7\': 3 (reset)\n\'0x8\': 4\n\'0x9\': 5\n\'0xA\': 6\n\'0xB\': 7\n\'0xC\': 8\n\'0xD\': 9\n\'0xE\': 10\n\'0xF\': 11"]
    #[inline(always)]
    pub fn rssi_comp01(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RfRssiCompCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, RfRssiCompCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfRssiCompCtrlReg {
    #[inline(always)]
    fn default() -> RfRssiCompCtrlReg {
        <crate::RegValueT<RfRssiCompCtrlReg_SPEC> as RegisterValue<_>>::new(34679)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfVcoCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfVcoCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = ""]
pub type RfVcoCtrlReg = crate::RegValueT<RfVcoCtrlReg_SPEC>;

impl RfVcoCtrlReg {
    #[doc = "Set the desired amplitude of the VCO\'0\': minimum amplitude\n\'4\': default amplitude\n\'F\': maximum amplitude"]
    #[inline(always)]
    pub fn vco_ampl_set(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RfVcoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, RfVcoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfVcoCtrlReg {
    #[inline(always)]
    fn default() -> RfVcoCtrlReg {
        <crate::RegValueT<RfVcoCtrlReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpotpTestReg_SPEC;
impl crate::sealed::RegSpec for SpotpTestReg_SPEC {
    type DataType = u16;
}
#[doc = "(in CRG)"]
pub type SpotpTestReg = crate::RegValueT<SpotpTestReg_SPEC>;

impl SpotpTestReg {
    #[doc = "Bypass LDO and put VBAT directly on OTP_VDDIO"]
    #[inline(always)]
    pub fn ldo_otp_write(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpotpTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SpotpTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the SPOTP testmode. Reset by write or PowerOnReset"]
    #[inline(always)]
    pub fn spotp_active(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpotpTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SpotpTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpotpTestReg {
    #[inline(always)]
    fn default() -> SpotpTestReg {
        <crate::RegValueT<SpotpTestReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Reset request for the OTP controller."]
    #[inline(always)]
    pub fn otpc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latches the control signals of the pads for state retention in powerdown mode.\n0: Control signals are retained\n1: Latch is transparant, pad can be recontrolled"]
    #[inline(always)]
    pub fn pad_latch_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables OTP to SysRAM copy action after waking up PD_SYS"]
    #[inline(always)]
    pub fn otp_copy(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the clock source of the 32 kHz clock\n0 = RC-oscillator\n1 = 32 kHz crystal oscillator"]
    #[inline(always)]
    pub fn clk32_source(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the development phase mode.\nIf this bit is set, the SysRAM cell will not power gated during sleep (extended sleep).\nNo copy action to SysRAM is done when the system wakes up.\nFor emulating startup time, the OTP_COPY bit still needs to be set."]
    #[inline(always)]
    pub fn ret_sysram(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls which memory is located at address 0x0000 for execution.\n0x0: ROM\n0x1: OTP\n0x2: SysRAM\n0x3: RetRAM"]
    #[inline(always)]
    pub fn remap_adr0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Indicates that XTAL16 has had > 2 ms of settle time"]
    #[inline(always)]
    pub fn xtal16_settled(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that XTAL trimming mechanism is ready, i.e. the trimming equals CLK_FREQ_TRIM_REG."]
    #[inline(always)]
    pub fn xtal16_trim_ready(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_DBG is functional"]
    #[inline(always)]
    pub fn dbg_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that PD_DBG is in power down"]
    #[inline(always)]
    pub fn dbg_is_down(
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
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(85)
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
    #[doc = "Defines the delay between XTAL16M enable and applying the CLK_FREQ_TRIM_REG in steps of 250 us.\n0x0: apply directly\n0x1: wait between 0 and 250 us\n0x2: wait between 250 us and 500 us\netc."]
    #[inline(always)]
    pub fn trim_time(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, TrimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the delay between applying CLK_FREQ_TRIM_REG and XTAL16_SETTLED in steps of 250 us.\n0x0: XTAL16_SETTLED is set direcly\n0x1: wait between 0 and 250 us\n0x2: wait between 250 us and 500 us\netc."]
    #[inline(always)]
    pub fn settle_time(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, TrimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, TrimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TrimCtrlReg {
    #[inline(always)]
    fn default() -> TrimCtrlReg {
        <crate::RegValueT<TrimCtrlReg_SPEC> as RegisterValue<_>>::new(162)
    }
}
