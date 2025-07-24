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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:17 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ANAMISC registers"]
unsafe impl ::core::marker::Send for super::Anamisc {}
unsafe impl ::core::marker::Sync for super::Anamisc {}
impl super::Anamisc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Charger control register 1"]
    #[inline(always)]
    pub const fn charger_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Charger control register 2"]
    #[inline(always)]
    pub const fn charger_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Charger status and trimming register"]
    #[inline(always)]
    pub const fn charger_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Count value for oscillator calibration"]
    #[inline(always)]
    pub const fn clk_ref_cnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefCntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefCntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[doc = "Select clock for oscillator calibration"]
    #[inline(always)]
    pub const fn clk_ref_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "DIVN reference cycles, upper 16 bits"]
    #[inline(always)]
    pub const fn clk_ref_val_h_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefValHReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefValHReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[doc = "DIVN reference cycles, lower 16 bits"]
    #[inline(always)]
    pub const fn clk_ref_val_l_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefValLReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefValLReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Fuel Gauge manually add extra charge to SOC_CHARGE_CNTRx_REG"]
    #[inline(always)]
    pub const fn soc_add2ch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocAdd2ChReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocAdd2ChReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[doc = "Fuel Gauge Average charge counter"]
    #[inline(always)]
    pub const fn soc_charge_avg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeAvgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeAvgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Fuel Gauge Charge counter bits 15-0"]
    #[inline(always)]
    pub const fn soc_charge_cntr1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeCntr1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeCntr1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Fuel Gauge Charge counter bits 31-16"]
    #[inline(always)]
    pub const fn soc_charge_cntr2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeCntr2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeCntr2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[doc = "Fuel Gauge Charge counter bits 39-32"]
    #[inline(always)]
    pub const fn soc_charge_cntr3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeCntr3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeCntr3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Fuel Gauge Control register 1"]
    #[inline(always)]
    pub const fn soc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Fuel Gauge Control register 2"]
    #[inline(always)]
    pub const fn soc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "Fuel Gauge Control register 3"]
    #[inline(always)]
    pub const fn soc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Fuel Gauge input test register"]
    #[inline(always)]
    pub const fn soc_ext_in_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocExtInReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocExtInReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Fuel Gauge output test register"]
    #[inline(always)]
    pub const fn soc_ext_out_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocExtOutReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocExtOutReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[doc = "Fuel Gauge Status register"]
    #[inline(always)]
    pub const fn soc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for ChargerCtrl1Reg_SPEC {
    type DataType = u16;
}

#[doc = "Charger control register 1"]
pub type ChargerCtrl1Reg = crate::RegValueT<ChargerCtrl1Reg_SPEC>;

impl ChargerCtrl1Reg {
    #[doc = "0: Die temperature protection enabled: charger will be disabled when die temp exceeds value set in DIE_TEMP_SET\n1: Die temperature protection disabled: testmode, use only in agreement with Dialog"]
    #[inline(always)]
    pub fn die_temp_disable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Die temperature protection level.\nCharging will be automatically disabled if set level is exceeded and resumed when temperature has dropped few degrees below set level.\n00: 0oC (do not use, for test only)\n01: 50oC (do not use, for test only)\n10: 80oC (default)\n11: 100oC"]
    #[inline(always)]
    pub fn die_temp_set(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, ChargerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Constant Current levels (typical values)\n0000: 5 mA\n0001: 10 mA\n0010: 30 mA\n0011: 45 mA\n0100: 60 mA\n0101: 90 mA\n0110: 120 mA\n0111: 150 mA\n1000: 180 mA\n1001: 210 mA\n1010: 270 mA\n1011: 300 mA\n1100: 350 mA\n1101: 400 mA"]
    #[inline(always)]
    pub fn charge_cur(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, ChargerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal operation: voltage level higher than 7/8 VDD_USB will disable the charger\n1: NTC low temp limit disabled: use if trickle charging below the minimum temperature is required"]
    #[inline(always)]
    pub fn ntc_low_disable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Charger NTC protection enabled\n1: Charger NTC protection disable"]
    #[inline(always)]
    pub fn ntc_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Charger in powerdown\n1: Charger enabled"]
    #[inline(always)]
    pub fn charge_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Constant Voltage Levels\n00000: 3.00V (reset)\n00001: 3.40V (e.g. 2xNiMH)\n00010: 3.50V\n00011: 3.60V (e.g. Li-phosphate)\n00100: 3.74V\n00101: 3.86V\n00110: 4.00V\n00111: 4.05V\n01000: 4.10V\n01001: 4.15V\n01010: 4.20V (e.g. Li-Co, Li-Mn, NMC)\n01011: 4.25V\n01100: 4.30V\n01101: 4.35V\n01110: 4.40V\n01111: 4.50V\n10000: 4.60V\n10001: 4.90V e.g. 3xNiMH\n10010: 5.00V"]
    #[inline(always)]
    pub fn charge_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, ChargerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerCtrl1Reg {
    #[inline(always)]
    fn default() -> ChargerCtrl1Reg {
        <crate::RegValueT<ChargerCtrl1Reg_SPEC> as RegisterValue<_>>::new(8192)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for ChargerCtrl2Reg_SPEC {
    type DataType = u16;
}

#[doc = "Charger control register 2"]
pub type ChargerCtrl2Reg = crate::RegValueT<ChargerCtrl2Reg_SPEC>;

impl ChargerCtrl2Reg {
    #[doc = "Signals are mapped on SPDIF pin.\nAlso set ANA_TEST_REG\\[ANA_TESTBUS_TO_ADCPIN\\] = 1\n\n000: normal mode (no test selected)\n001: Vptat (temperature sensor) \\[1.4V max\\]\n010: Vbat_sense after divider \\[1.2V\\]\n011: Current loop output \\[0 to vsupply\\]\n100: Voltage loop output \\[0 to vsupply\\]\n101: Imeas or Iref/10\n110: Icharge reduced by 26.6\n111: reserved"]
    #[inline(always)]
    pub fn charger_test(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "do not change, for test purpose only"]
    #[inline(always)]
    pub fn current_offset_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Independent adjustment for the charge level. Adjust range is +/- 1.8 percent. The 4 bits adjustment is in two\'s complement."]
    #[inline(always)]
    pub fn charger_vfloat_adj(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "do not change, for test purpose only"]
    #[inline(always)]
    pub fn current_gain_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerCtrl2Reg {
    #[inline(always)]
    fn default() -> ChargerCtrl2Reg {
        <crate::RegValueT<ChargerCtrl2Reg_SPEC> as RegisterValue<_>>::new(3847)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerStatusReg_SPEC;
impl crate::sealed::RegSpec for ChargerStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "Charger status and trimming register"]
pub type ChargerStatusReg = crate::RegValueT<ChargerStatusReg_SPEC>;

impl ChargerStatusReg {
    #[doc = "0: Dietemp below DIE_TEMP_SET level. Normal operation\n1: Dietemp above DIE_TEMP_SET level. Charging is disabled"]
    #[inline(always)]
    pub fn charger_tmode_prot(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: Battery pack temperature \'ok\' or \'too low\' (voltage level on NTC pin above 1/2 VDD_USB)\n1: Battery pack temperature \'too high\' (voltage level on NTC pin below 1/2 VDD_USB)"]
    #[inline(always)]
    pub fn charger_battemp_high(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: Battery pack temperature \'too low\' or \'too high\' (voltage level on NTC pin below 1/2 or above 7/8 VDD_USB)\n1: Battery pack temperature \'ok\' (voltage level on NTC pin between 1/2 and 7/8 VDD_USB)"]
    #[inline(always)]
    pub fn charger_battemp_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: Battery pack temperature \'ok\' or \'too high\' (voltage level on NTC pin below 7/8 VDD_USB)\n1: Battery pack temperature \'too low\' (voltage level on NTC pin above than 7/8 VDD_USB)"]
    #[inline(always)]
    pub fn charger_battemp_low(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: Actual charge current is between 10...100 percent of set CHARGE_CUR (or CHARGE_ON=0)\n1: Actual charge current <10 percent of set CHARGE_CUR"]
    #[inline(always)]
    pub fn end_of_charge(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: voltage loop not in regulation (or charger is off)\n1: constant voltage mode active, voltage loop in regulation."]
    #[inline(always)]
    pub fn charger_cv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: current loop not in regulation (or charger is off)\n1: constant current mode active, current loop in regulation."]
    #[inline(always)]
    pub fn charger_cc_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerStatusReg {
    #[inline(always)]
    fn default() -> ChargerStatusReg {
        <crate::RegValueT<ChargerStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefCntReg_SPEC;
impl crate::sealed::RegSpec for ClkRefCntReg_SPEC {
    type DataType = u16;
}

#[doc = "Count value for oscillator calibration"]
pub type ClkRefCntReg = crate::RegValueT<ClkRefCntReg_SPEC>;

impl ClkRefCntReg {
    #[doc = "Indicates the calibration time, with a decrement counter to 1."]
    #[inline(always)]
    pub fn ref_cnt_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, ClkRefCntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ClkRefCntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkRefCntReg {
    #[inline(always)]
    fn default() -> ClkRefCntReg {
        <crate::RegValueT<ClkRefCntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefSelReg_SPEC;
impl crate::sealed::RegSpec for ClkRefSelReg_SPEC {
    type DataType = u16;
}

#[doc = "Select clock for oscillator calibration"]
pub type ClkRefSelReg = crate::RegValueT<ClkRefSelReg_SPEC>;

impl ClkRefSelReg {
    #[doc = "Writing a \'1\' starts a calibration. This bit is cleared when calibration is finished, and CLK_REF_VAL is ready."]
    #[inline(always)]
    pub fn ref_cal_start(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkRefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select clock input for calibration:\n0x0 : RC32K oscillator\n0x1 : RC16M oscillator\n0x2 : XTAL32K oscillator\n0x3 : RCX oscillator"]
    #[inline(always)]
    pub fn ref_clk_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkRefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRefSelReg {
    #[inline(always)]
    fn default() -> ClkRefSelReg {
        <crate::RegValueT<ClkRefSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefValHReg_SPEC;
impl crate::sealed::RegSpec for ClkRefValHReg_SPEC {
    type DataType = u16;
}

#[doc = "DIVN reference cycles, upper 16 bits"]
pub type ClkRefValHReg = crate::RegValueT<ClkRefValHReg_SPEC>;

impl ClkRefValHReg {
    #[doc = "Returns the upper 16 bits of DIVN clock cycles counted during the calibration time, defined with REF_CNT_VAL"]
    #[inline(always)]
    pub fn xtal_cnt_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, ClkRefValHReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ClkRefValHReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkRefValHReg {
    #[inline(always)]
    fn default() -> ClkRefValHReg {
        <crate::RegValueT<ClkRefValHReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefValLReg_SPEC;
impl crate::sealed::RegSpec for ClkRefValLReg_SPEC {
    type DataType = u16;
}

#[doc = "DIVN reference cycles, lower 16 bits"]
pub type ClkRefValLReg = crate::RegValueT<ClkRefValLReg_SPEC>;

impl ClkRefValLReg {
    #[doc = "Returns the lower 16 bits of DIVN clock cycles counted during the calibration time, defined with REF_CNT_VAL"]
    #[inline(always)]
    pub fn xtal_cnt_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, ClkRefValLReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ClkRefValLReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkRefValLReg {
    #[inline(always)]
    fn default() -> ClkRefValLReg {
        <crate::RegValueT<ClkRefValLReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocAdd2ChReg_SPEC;
impl crate::sealed::RegSpec for SocAdd2ChReg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge manually add extra charge to SOC_CHARGE_CNTRx_REG"]
pub type SocAdd2ChReg = crate::RegValueT<SocAdd2ChReg_SPEC>;

impl SocAdd2ChReg {
    #[doc = "Extra charge to be added to the SOC_CHARGE_CNTRx_REG per sample period (9-bit + sign + 6 fractional bits"]
    #[inline(always)]
    pub fn soc_add2ch(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SocAdd2ChReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocAdd2ChReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocAdd2ChReg {
    #[inline(always)]
    fn default() -> SocAdd2ChReg {
        <crate::RegValueT<SocAdd2ChReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeAvgReg_SPEC;
impl crate::sealed::RegSpec for SocChargeAvgReg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Average charge counter"]
pub type SocChargeAvgReg = crate::RegValueT<SocChargeAvgReg_SPEC>;

impl SocChargeAvgReg {
    #[doc = "Average of \'charge\' current (9-bit + sign and 6 fractional bits"]
    #[inline(always)]
    pub fn charge_avg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SocChargeAvgReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocChargeAvgReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeAvgReg {
    #[inline(always)]
    fn default() -> SocChargeAvgReg {
        <crate::RegValueT<SocChargeAvgReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeCntr1Reg_SPEC;
impl crate::sealed::RegSpec for SocChargeCntr1Reg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Charge counter bits 15-0"]
pub type SocChargeCntr1Reg = crate::RegValueT<SocChargeCntr1Reg_SPEC>;

impl SocChargeCntr1Reg {
    #[doc = "Sum of the charge-values per sampling period; (bits15:0)\nThe absolute full-scale charge value is 6-bits, At full scale charge current it takes 2^26 sampling periods until overflow of the charge_cnt register after a reset_charge event.\nAt fs=144kHz (=default) this will happen after 33 hours\nAt fs=1.152MHz After 10 hours"]
    #[inline(always)]
    pub fn charge_cnt1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SocChargeCntr1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocChargeCntr1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeCntr1Reg {
    #[inline(always)]
    fn default() -> SocChargeCntr1Reg {
        <crate::RegValueT<SocChargeCntr1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeCntr2Reg_SPEC;
impl crate::sealed::RegSpec for SocChargeCntr2Reg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Charge counter bits 31-16"]
pub type SocChargeCntr2Reg = crate::RegValueT<SocChargeCntr2Reg_SPEC>;

impl SocChargeCntr2Reg {
    #[doc = "Sum of the charge-values per sampling period; (bits23:16)"]
    #[inline(always)]
    pub fn charge_cnt2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SocChargeCntr2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocChargeCntr2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeCntr2Reg {
    #[inline(always)]
    fn default() -> SocChargeCntr2Reg {
        <crate::RegValueT<SocChargeCntr2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeCntr3Reg_SPEC;
impl crate::sealed::RegSpec for SocChargeCntr3Reg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Charge counter bits 39-32"]
pub type SocChargeCntr3Reg = crate::RegValueT<SocChargeCntr3Reg_SPEC>;

impl SocChargeCntr3Reg {
    #[doc = "Sum of the charge-values per sampling period; (bits39:24)"]
    #[inline(always)]
    pub fn charge_cnt3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SocChargeCntr3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            SocChargeCntr3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeCntr3Reg {
    #[inline(always)]
    fn default() -> SocChargeCntr3Reg {
        <crate::RegValueT<SocChargeCntr3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for SocCtrl1Reg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Control register 1"]
pub type SocCtrl1Reg = crate::RegValueT<SocCtrl1Reg_SPEC>;

impl SocCtrl1Reg {
    #[doc = "Integrator capacitor scaler\n0: Cint = 1 pF\n1: Cint = 2 pF\n2: Cint = 4 pF\n3: Cint = 8 pF (=default)"]
    #[inline(always)]
    pub fn soc_cint(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Current DAC scaler\n0: Ibias = 2 uA\n1: Ibias = 1 uA (=default)\n2: Ibias = 0.5 uA\n3: Ibias = 0.25 uA"]
    #[inline(always)]
    pub fn soc_bias(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SOC Sample frequency\n0: automatic mode (tbd)\n1: fs = 18 kHz\n2: fs = 36 kHz\n3: fs = 72 kHz\n4: fs = 144 kHz (=default)\n5: fs = 288 kHz\n6: fs = 576 kHz\n7: fs = 1152 kHz"]
    #[inline(always)]
    pub fn soc_clk(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: low-pass filter at integrator inputs disabled\n1: Enables a low-pass filter at the integrator inputs"]
    #[inline(always)]
    pub fn soc_lpf(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Scales the current DAC (Ibias: default=1uA)\n0: Idac=0.25*Ibias\n1: Idac=0.5*Ibias\n2: Idac=Ibias (=default)\n3: Idac=2*Ibias"]
    #[inline(always)]
    pub fn soc_idac(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the sign of the charge converter input and output to perform a chopper function to eliminate offset voltage (see also SOC_CHOP and \'sign\' on output pin)\n0: non-inverted inputs and outputs\n1: inverted inputs and outputs"]
    #[inline(always)]
    pub fn soc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reserved (not yet implemented): switches the SOC-inputs to the GPIO pins"]
    #[inline(always)]
    pub fn soc_gpio(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal operation\n1: Connect the input voltage to 0V"]
    #[inline(always)]
    pub fn soc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: Reset the SOC_CHARGE_AVG_REG to the last value of SOC_CHARGE_CNTRx_REG"]
    #[inline(always)]
    pub fn soc_reset_avg(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: Reset CHARGE_CNTR_REG"]
    #[inline(always)]
    pub fn soc_reset_charge(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: SOC analog circuits off. CHARGE_CNTRx_REG can still be written for a manual update.\nSee SOC_ADD2CH_REG\n1: SOC analog circuits enabled"]
    #[inline(always)]
    pub fn soc_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocCtrl1Reg {
    #[inline(always)]
    fn default() -> SocCtrl1Reg {
        <crate::RegValueT<SocCtrl1Reg_SPEC> as RegisterValue<_>>::new(55424)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for SocCtrl2Reg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Control register 2"]
pub type SocCtrl2Reg = crate::RegValueT<SocCtrl2Reg_SPEC>;

impl SocCtrl2Reg {
    #[doc = "if HIGH then \'weight\' of Moving Average is forced to 1 if the converter detects significant input change\n(if dcharge > 4*delta_c, or high_limit, or low_limit)"]
    #[inline(always)]
    pub fn soc_dynavg(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SocCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Moving Average Weight factor\ncharge_avg(n) =\n(weight*charge_avg(n-1) + charge(n) ) / (weight+1) where:weight = 2^(soc_maw)"]
    #[inline(always)]
    pub fn soc_maw(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SOC_CMIREG enable"]
    #[inline(always)]
    pub fn soc_cmireg_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SocCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Chopping control\n0: \'external\' chopping control with \'soc_sign\'-input\n1: chop each 2^1*scycle fs-periods\n2: chop each 2^2*scycle fs-periods\n..\n7: chop each 2^7*scycle fs-periods."]
    #[inline(always)]
    pub fn soc_chop(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "adds a common-mode current to Idac to increase the common-mode input-level of the integrator.\nThe common-mode input level is equal to (Idac+Icm)*Rvi;\n0: Icm=0;\n1: Icm=1*Ibias (=default);\n2: Icm=2*Ibias;\n3: Icm=4*Ibias"]
    #[inline(always)]
    pub fn soc_icm(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cycle the current divider segments of Idac\n0: no cycling\n1: cycle each scycle fs-periods"]
    #[inline(always)]
    pub fn soc_dcycle(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SocCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Cycle current segments (8 segments) of Idac\n0: no cycling\n1: cycle each fs-period\n2: cycle each 2 fs-periods\n..\n7: cycle each 7 fs-periods"]
    #[inline(always)]
    pub fn soc_scycle(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Voltage-to-current resistor scaler\n0: Rvi = 25 k\n1: Rvi = 50 k\n2: Rvi = 100 k (= default)\n3: Rvi = 200 k"]
    #[inline(always)]
    pub fn soc_rvi(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocCtrl2Reg {
    #[inline(always)]
    fn default() -> SocCtrl2Reg {
        <crate::RegValueT<SocCtrl2Reg_SPEC> as RegisterValue<_>>::new(30570)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for SocCtrl3Reg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Control register 3"]
pub type SocCtrl3Reg = crate::RegValueT<SocCtrl3Reg_SPEC>;

impl SocCtrl3Reg {
    #[doc = "Common Input Voltage target of regulator (see SOC_CMIREG_ENABLE)\n0: 50 mV\n1: 100 mV\n2: 150 mV\n3: 200 mV"]
    #[inline(always)]
    pub fn soc_vcmi(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, SocCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reserved. (To be implemented)\nHysteresis of the comparator which detects if the integrator voltage is rising or falling"]
    #[inline(always)]
    pub fn soc_dynhys(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SocCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reserved. (To be implemented)\n0: Vint_target = 0V\n1: Vint_target tracks the 2 MSB\'s of the charge register)"]
    #[inline(always)]
    pub fn soc_dyntarg(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SocCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trigger level of the high-limit and low-limit comparators.\n0: low_limit = -50mV; high_limit = +50mV\n1: low_limit = -100mV; high_limit = +100mV (=default)\n2: low_limit = -200mV; high_limit = +200mV\n3: low_limit = -400mV; high_limit = +400mV"]
    #[inline(always)]
    pub fn soc_vsat(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SocCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocCtrl3Reg {
    #[inline(always)]
    fn default() -> SocCtrl3Reg {
        <crate::RegValueT<SocCtrl3Reg_SPEC> as RegisterValue<_>>::new(17)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocExtInReg_SPEC;
impl crate::sealed::RegSpec for SocExtInReg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge input test register"]
pub type SocExtInReg = crate::RegValueT<SocExtInReg_SPEC>;

impl SocExtInReg {
    #[doc = "1: Enable \'external\' control of Idac"]
    #[inline(always)]
    pub fn soc_ext_idac_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: Enable \'external\' control of scycle"]
    #[inline(always)]
    pub fn soc_ext_scycle_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Number of the scycle"]
    #[inline(always)]
    pub fn soc_nr_scycle(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, SocExtInReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Disables the resistor divider DAC. The Idac has 6-bits (plus sign)\n1: Enables the resistor divider DAC. The Idac has 9-bits (plus sign)"]
    #[inline(always)]
    pub fn soc_rdac_dis(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: SOC_IDAC_VAL is positive\n1: SOC_IDAC_VAL is negative"]
    #[inline(always)]
    pub fn soc_idac_sign(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls the current for the DAC.\n0: 0/512*SOC_IDAC\nN: N/512*SOC_IDAC"]
    #[inline(always)]
    pub fn soc_idac_val(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, SocExtInReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocExtInReg {
    #[inline(always)]
    fn default() -> SocExtInReg {
        <crate::RegValueT<SocExtInReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocExtOutReg_SPEC;
impl crate::sealed::RegSpec for SocExtOutReg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge output test register"]
pub type SocExtOutReg = crate::RegValueT<SocExtOutReg_SPEC>;

impl SocExtOutReg {
    #[doc = "Controller event"]
    #[inline(always)]
    pub fn soc_ctrl_event(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Controller state"]
    #[inline(always)]
    pub fn soc_state(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, SocExtOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Rising comparator output"]
    #[inline(always)]
    pub fn soc_rising_comp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Positive comparator output"]
    #[inline(always)]
    pub fn soc_pos_comp(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Low_limit comparator output"]
    #[inline(always)]
    pub fn soc_lowlim_comp(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "High_limit comparator output"]
    #[inline(always)]
    pub fn soc_high_lim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SocExtOutReg {
    #[inline(always)]
    fn default() -> SocExtOutReg {
        <crate::RegValueT<SocExtOutReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocStatusReg_SPEC;
impl crate::sealed::RegSpec for SocStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "Fuel Gauge Status register"]
pub type SocStatusReg = crate::RegValueT<SocStatusReg_SPEC>;

impl SocStatusReg {
    #[doc = "0: Normal Operation\n1: Integrator is pushed over high or low limit.\nReturns to \'0\' if the converter runs for more than 2 sequential sampling periods in a \'safe\' region (dcharge < 2*delta_c)"]
    #[inline(always)]
    pub fn soc_int_locked(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SocStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SocStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: Normal Operation\n1: Integrator exceeds high or low limit with full-scale IDAC (charge) for more than 3 sequential sampling periods"]
    #[inline(always)]
    pub fn soc_int_overload(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SocStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SocStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SocStatusReg {
    #[inline(always)]
    fn default() -> SocStatusReg {
        <crate::RegValueT<SocStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
