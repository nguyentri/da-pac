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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:49 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"r_rfcu580_nl01 registers"]
unsafe impl ::core::marker::Send for super::RRfcu580Nl01 {}
unsafe impl ::core::marker::Sync for super::RRfcu580Nl01 {}
impl super::RRfcu580Nl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn bias_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BiasCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BiasCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1536usize),
            )
        }
    }

    #[doc = "Must be Retained"]
    #[inline(always)]
    pub const fn rf_adci_dc_offset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdciDcOffsetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdciDcOffsetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }

    #[doc = "Must be Retained"]
    #[inline(always)]
    pub const fn rf_adcq_dc_offset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcqDcOffsetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcqDcOffsetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(786usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_adc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2096usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_adc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2098usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_adc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_afc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAfcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAfcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2146usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_lut_01_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcLut01Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcLut01Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_lut_23_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcLut23Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcLut23Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2130usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_lut_45_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcLut45Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcLut45Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_lut_67_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcLut67Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcLut67Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2134usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_lut_89_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcLut89Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcLut89Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_agc_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2304usize),
            )
        }
    }

    #[doc = "Changed functionality of bits \\[7:6\\]"]
    #[inline(always)]
    pub const fn rf_bmcw_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfBmcwReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfBmcwReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_calcap1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCalcap1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCalcap1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_calcap2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCalcap2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCalcap2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_calstate_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCalstateReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCalstateReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cal_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCalCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCalCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1298usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1302usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1304usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1306usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1282usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1286usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1290usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1294usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1296usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cp_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCpCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCpCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_dc_offset_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDcOffsetCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDcOffsetCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2150usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_dc_offset_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDcOffsetCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDcOffsetCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_dc_offset_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDcOffsetCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDcOffsetCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2154usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_dc_offset_ctrl4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDcOffsetCtrl4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDcOffsetCtrl4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2156usize),
            )
        }
    }

    #[doc = "Must be Retained"]
    #[inline(always)]
    pub const fn rf_dc_offset_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDcOffsetResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDcOffsetResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(788usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_dem_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDemCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDemCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1042usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1044usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1046usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1050usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1052usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config16_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig16Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig16Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1054usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config17_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig17Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig17Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1056usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config18_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig18Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig18Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1058usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config19_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig19Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig19Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1060usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config20_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig20Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig20Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1062usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config21_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig21Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig21Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1064usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config22_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig22Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig22Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1066usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config23_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig23Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig23Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1068usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1026usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1030usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1032usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1034usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1036usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1038usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_iff_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfIffCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfIffCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2080usize),
            )
        }
    }

    #[doc = "Must be Retained"]
    #[inline(always)]
    pub const fn rf_iff_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfIffResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfIffResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_irq_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfIrqCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfIrqCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_lf_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLfCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLfCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3168usize),
            )
        }
    }

    #[doc = "LF resistor setting"]
    #[inline(always)]
    pub const fn rf_lf_res_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLfResCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLfResCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3154usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_mgain_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfMgainCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfMgainCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3082usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_mgain_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfMgainCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfMgainCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3080usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_mgc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfMgcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfMgcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3088usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_mixer_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfMixerCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfMixerCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2064usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_mixer_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfMixerCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfMixerCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2066usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_overrule_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfOverruleReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfOverruleReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Removed obsolete values of bits 10:7, pa_pw back to 4"]
    #[inline(always)]
    pub const fn rf_pa_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfPaCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfPaCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2560usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_pfd_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfPfdCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfPfdCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_radig_spare_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfRadigSpareReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfRadigSpareReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_ref_osc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfRefOscReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfRefOscReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(514usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_rssi_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfRssiResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfRssiResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2306usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_scan_feedback_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfScanFeedbackReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfScanFeedbackReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_spare1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSpare1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSpare1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1538usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_synth_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSynthCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSynthCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_synth_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSynthCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3074usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_synth_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSynthCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSynthCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3076usize),
            )
        }
    }

    #[doc = "Must be Retained"]
    #[inline(always)]
    pub const fn rf_synth_result2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSynthResult2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSynthResult2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(792usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_synth_result3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSynthResult3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSynthResult3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(794usize),
            )
        }
    }

    #[doc = "Must be Retained"]
    #[inline(always)]
    pub const fn rf_synth_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSynthResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSynthResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(790usize),
            )
        }
    }

    #[doc = "TDC settings"]
    #[inline(always)]
    pub const fn rf_tdc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfTdcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfTdcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3184usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_vcocal_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfVcocalCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfVcocalCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3078usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_vcovar_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfVcovarCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfVcovarCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3104usize),
            )
        }
    }

    #[doc = "LUT entry for bit 14 of the VCO calibration capacitance"]
    #[inline(always)]
    pub const fn rf_vco_calcap_bit14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfVcoCalcapBit14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfVcoCalcapBit14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3106usize),
            )
        }
    }

    #[doc = "LUT entry for bit 15 of the VCO calibration capacitance"]
    #[inline(always)]
    pub const fn rf_vco_calcap_bit15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfVcoCalcapBit15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfVcoCalcapBit15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3108usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BiasCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for BiasCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type BiasCtrl1Reg = crate::RegValueT<BiasCtrl1Reg_SPEC>;

impl BiasCtrl1Reg {
    #[doc = "Tuning of the IF filter bias current"]
    #[inline(always)]
    pub fn iff_bias_set(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, BiasCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,BiasCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Tuning of the VCO bias current"]
    #[inline(always)]
    pub fn vco_bias_set(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, BiasCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,BiasCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Tuning of the charge pump bias current"]
    #[inline(always)]
    pub fn cp_bias_set(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, BiasCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,BiasCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Tuning of the mixer bias current"]
    #[inline(always)]
    pub fn mix_bias_set(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, BiasCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,BiasCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BiasCtrl1Reg {
    #[inline(always)]
    fn default() -> BiasCtrl1Reg {
        <crate::RegValueT<BiasCtrl1Reg_SPEC> as RegisterValue<_>>::new(34952)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdciDcOffsetReg_SPEC;
impl crate::sealed::RegSpec for RfAdciDcOffsetReg_SPEC {
    type DataType = u16;
}

#[doc = "Must be Retained"]
pub type RfAdciDcOffsetReg = crate::RegValueT<RfAdciDcOffsetReg_SPEC>;

impl RfAdciDcOffsetReg {
    #[doc = "DC offset compensation in the I path (inverting input) in sign-magnitude notarion (i.e. -31 : 1 : 31 mV)"]
    #[inline(always)]
    pub fn adc_offn_i_rd(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfAdciDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfAdciDcOffsetReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DC offset compensation in the I path (non-inverting input) in sign-magnitude notarion (i.e. -31 : 1 : 31 mV)"]
    #[inline(always)]
    pub fn adc_offp_i_rd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfAdciDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfAdciDcOffsetReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfAdciDcOffsetReg {
    #[inline(always)]
    fn default() -> RfAdciDcOffsetReg {
        <crate::RegValueT<RfAdciDcOffsetReg_SPEC> as RegisterValue<_>>::new(32896)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdcqDcOffsetReg_SPEC;
impl crate::sealed::RegSpec for RfAdcqDcOffsetReg_SPEC {
    type DataType = u16;
}

#[doc = "Must be Retained"]
pub type RfAdcqDcOffsetReg = crate::RegValueT<RfAdcqDcOffsetReg_SPEC>;

impl RfAdcqDcOffsetReg {
    #[doc = "DC offset compensation in the Q path (inverting input) in sign-magnitude notarion (i.e. -31 : 1 : 31 mV)"]
    #[inline(always)]
    pub fn adc_offn_q_rd(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfAdcqDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfAdcqDcOffsetReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DC offset compensation in the Q path (non-inverting input) in sign-magnitude notarion (i.e. -31 : 1 : 31 mV)"]
    #[inline(always)]
    pub fn adc_offp_q_rd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfAdcqDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfAdcqDcOffsetReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfAdcqDcOffsetReg {
    #[inline(always)]
    fn default() -> RfAdcqDcOffsetReg {
        <crate::RegValueT<RfAdcqDcOffsetReg_SPEC> as RegisterValue<_>>::new(32896)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdcCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfAdcCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type RfAdcCtrl1Reg = crate::RegValueT<RfAdcCtrl1Reg_SPEC>;

impl RfAdcCtrl1Reg {
    #[doc = "Change polarity of ADC input."]
    #[inline(always)]
    pub fn adc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal operation\n1: Short the inputs of the ADC (used for DC offset cal)"]
    #[inline(always)]
    pub fn adc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,RfAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal operation (i.e. Use automatically calibrated value)\n1: Use ADC_OFFx_y_WR to set the DC offset compensation values in the ADC (x = N or P, y = I or Q"]
    #[inline(always)]
    pub fn adc_dc_offset_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAdcCtrl1Reg {
    #[inline(always)]
    fn default() -> RfAdcCtrl1Reg {
        <crate::RegValueT<RfAdcCtrl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdcCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfAdcCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type RfAdcCtrl2Reg = crate::RegValueT<RfAdcCtrl2Reg_SPEC>;

impl RfAdcCtrl2Reg {
    #[doc = "External value for the DC offset compensation in the I path negative side. With common mode input voltage at Vpwrp/2, this value is 512-ADC_OFFP_Q_WR."]
    #[inline(always)]
    pub fn adc_offn_i_wr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfAdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,RfAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "External value for the DC offset compensation in the I path positive side."]
    #[inline(always)]
    pub fn adc_offp_i_wr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfAdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,RfAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAdcCtrl2Reg {
    #[inline(always)]
    fn default() -> RfAdcCtrl2Reg {
        <crate::RegValueT<RfAdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdcCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for RfAdcCtrl3Reg_SPEC {
    type DataType = u16;
}

pub type RfAdcCtrl3Reg = crate::RegValueT<RfAdcCtrl3Reg_SPEC>;

impl RfAdcCtrl3Reg {
    #[doc = "External value for the DC offset compensation in the Q path negative side. With common mode input voltage at Vpwrp/2, this value is 512-ADC_OFFP_Q_WR."]
    #[inline(always)]
    pub fn adc_offn_q_wr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfAdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,RfAdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "External value for the DC offset compensation in the Q path positive side."]
    #[inline(always)]
    pub fn adc_offp_q_wr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfAdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,RfAdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAdcCtrl3Reg {
    #[inline(always)]
    fn default() -> RfAdcCtrl3Reg {
        <crate::RegValueT<RfAdcCtrl3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAfcCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfAfcCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfAfcCtrlReg = crate::RegValueT<RfAfcCtrlReg_SPEC>;

impl RfAfcCtrlReg {
    #[doc = "Choose the method to use for AFC tracking during the slot\nDescription TBD"]
    #[inline(always)]
    pub fn pole2(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAfcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAfcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Choose the method to use for AFC tracking during the slot\nDescription TBD"]
    #[inline(always)]
    pub fn pole1(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RfAfcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RfAfcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Choose the method to use for AFC tracking during the slot\nDescription TBD"]
    #[inline(always)]
    pub fn afc_mode(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfAfcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfAfcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAfcCtrlReg {
    #[inline(always)]
    fn default() -> RfAfcCtrlReg {
        <crate::RegValueT<RfAfcCtrlReg_SPEC> as RegisterValue<_>>::new(245)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfAgcCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type RfAgcCtrl1Reg = crate::RegValueT<RfAgcCtrl1Reg_SPEC>;

impl RfAgcCtrl1Reg {
    #[doc = "Choose the method to use for AGC evaluation\nDescription TBD"]
    #[inline(always)]
    pub fn agc_mode(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AGC hysteresis high threshold (switch up one AGC_SETTING_R step when exceeding this level)"]
    #[inline(always)]
    pub fn agc_th_high(
        self,
    ) -> crate::common::RegisterField<7, 0x7f, 1, 0, u8, u8, RfAgcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7f,1,0,u8,u8,RfAgcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AGC hysteresis low threshold (switch down one AGC_SETTING_R step when dropping below this level)"]
    #[inline(always)]
    pub fn agc_th_low(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, RfAgcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,RfAgcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcCtrl1Reg {
    #[inline(always)]
    fn default() -> RfAgcCtrl1Reg {
        <crate::RegValueT<RfAgcCtrl1Reg_SPEC> as RegisterValue<_>>::new(38923)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfAgcCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type RfAgcCtrl2Reg = crate::RegValueT<RfAgcCtrl2Reg_SPEC>;

impl RfAgcCtrl2Reg {
    #[doc = "Enable the slow AGC mode (no consecutive AGC setting switches)"]
    #[inline(always)]
    pub fn slow_agc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfAgcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Fixed AGC setting to be used to configure LNA, VGA1 and VGA2 when AGCSETTING_SEL = 1\n0: Highest gain as configured in RF_AGC_LUT_01_REG\n1: Lower gain as configured in RF_AGC_LUT_01_REG\n2: Still lower gain as configured in RF_AGC_LUT_23_REG\n...\n9-F: Lowest gain as configured in RF_AGC_LUT_89_REG"]
    #[inline(always)]
    pub fn agcsetting_wr(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfAgcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LNA, VGA1 and VGA2 gains\n\'0\': controlled by AGC\'1\': provided manually through AGCSETTING_WR"]
    #[inline(always)]
    pub fn agcsetting_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfAgcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': AGC always active\'1\': Freeze gain after Access Address detection"]
    #[inline(always)]
    pub fn en_frz_gain(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfAgcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RSSI threshold for the packet detection"]
    #[inline(always)]
    pub fn rssi_th(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, RfAgcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcCtrl2Reg {
    #[inline(always)]
    fn default() -> RfAgcCtrl2Reg {
        <crate::RegValueT<RfAgcCtrl2Reg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcLut01Reg_SPEC;
impl crate::sealed::RegSpec for RfAgcLut01Reg_SPEC {
    type DataType = u16;
}

pub type RfAgcLut01Reg = crate::RegValueT<RfAgcLut01Reg_SPEC>;

impl RfAgcLut01Reg {
    #[doc = "LNA gain setting while in AGC setting 0"]
    #[inline(always)]
    pub fn lna_gain1(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 1"]
    #[inline(always)]
    pub fn vga1_gain1(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 1"]
    #[inline(always)]
    pub fn vga2_gain1(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LNA gain setting while in AGC setting 0"]
    #[inline(always)]
    pub fn lna_gain0(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 0"]
    #[inline(always)]
    pub fn vga1_gain0(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 0"]
    #[inline(always)]
    pub fn vga2_gain0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcLut01Reg {
    #[inline(always)]
    fn default() -> RfAgcLut01Reg {
        <crate::RegValueT<RfAgcLut01Reg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcLut23Reg_SPEC;
impl crate::sealed::RegSpec for RfAgcLut23Reg_SPEC {
    type DataType = u16;
}

pub type RfAgcLut23Reg = crate::RegValueT<RfAgcLut23Reg_SPEC>;

impl RfAgcLut23Reg {
    #[doc = "VGA1 gain setting while in AGC setting 3"]
    #[inline(always)]
    pub fn vga1_gain3(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut23Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut23Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 3"]
    #[inline(always)]
    pub fn vga2_gain3(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut23Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut23Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 2"]
    #[inline(always)]
    pub fn vga1_gain2(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut23Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut23Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 2"]
    #[inline(always)]
    pub fn vga2_gain2(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RfAgcLut23Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RfAgcLut23Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcLut23Reg {
    #[inline(always)]
    fn default() -> RfAgcLut23Reg {
        <crate::RegValueT<RfAgcLut23Reg_SPEC> as RegisterValue<_>>::new(2313)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcLut45Reg_SPEC;
impl crate::sealed::RegSpec for RfAgcLut45Reg_SPEC {
    type DataType = u16;
}

pub type RfAgcLut45Reg = crate::RegValueT<RfAgcLut45Reg_SPEC>;

impl RfAgcLut45Reg {
    #[doc = "LNA gain setting while in AGC setting 5"]
    #[inline(always)]
    pub fn lna_gain5(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 5"]
    #[inline(always)]
    pub fn vga1_gain5(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 5"]
    #[inline(always)]
    pub fn vga2_gain5(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LNA gain setting while in AGC setting 4"]
    #[inline(always)]
    pub fn lna_gain4(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 4"]
    #[inline(always)]
    pub fn vga1_gain4(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 4"]
    #[inline(always)]
    pub fn vga2_gain4(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcLut45Reg {
    #[inline(always)]
    fn default() -> RfAgcLut45Reg {
        <crate::RegValueT<RfAgcLut45Reg_SPEC> as RegisterValue<_>>::new(19274)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcLut67Reg_SPEC;
impl crate::sealed::RegSpec for RfAgcLut67Reg_SPEC {
    type DataType = u16;
}

pub type RfAgcLut67Reg = crate::RegValueT<RfAgcLut67Reg_SPEC>;

impl RfAgcLut67Reg {
    #[doc = "LNA gain setting while in AGC setting 7"]
    #[inline(always)]
    pub fn lna_gain7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 7"]
    #[inline(always)]
    pub fn vga1_gain7(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 7"]
    #[inline(always)]
    pub fn vga2_gain7(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LNA gain setting while in AGC setting 6"]
    #[inline(always)]
    pub fn lna_gain6(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 6"]
    #[inline(always)]
    pub fn vga1_gain6(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 6"]
    #[inline(always)]
    pub fn vga2_gain6(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcLut67Reg {
    #[inline(always)]
    fn default() -> RfAgcLut67Reg {
        <crate::RegValueT<RfAgcLut67Reg_SPEC> as RegisterValue<_>>::new(23379)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcLut89Reg_SPEC;
impl crate::sealed::RegSpec for RfAgcLut89Reg_SPEC {
    type DataType = u16;
}

pub type RfAgcLut89Reg = crate::RegValueT<RfAgcLut89Reg_SPEC>;

impl RfAgcLut89Reg {
    #[doc = "LNA gain setting while in AGC setting 9"]
    #[inline(always)]
    pub fn lna_gain9(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 9"]
    #[inline(always)]
    pub fn vga1_gain9(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 9"]
    #[inline(always)]
    pub fn vga2_gain9(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LNA gain setting while in AGC setting 8"]
    #[inline(always)]
    pub fn lna_gain8(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA1 gain setting while in AGC setting 8"]
    #[inline(always)]
    pub fn vga1_gain8(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VGA2 gain setting while in AGC setting 8"]
    #[inline(always)]
    pub fn vga2_gain8(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcLut89Reg {
    #[inline(always)]
    fn default() -> RfAgcLut89Reg {
        <crate::RegValueT<RfAgcLut89Reg_SPEC> as RegisterValue<_>>::new(27491)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcResultReg_SPEC;
impl crate::sealed::RegSpec for RfAgcResultReg_SPEC {
    type DataType = u16;
}

pub type RfAgcResultReg = crate::RegValueT<RfAgcResultReg_SPEC>;

impl RfAgcResultReg {
    #[doc = "AGC setting as automatically selected in receive mode to configure LNA, VGA1 and VGA2\n0: Highest gain as configured in RF_AGC_LUT_01_REG\n1: Lower gain as configured in RF_AGC_LUT_01_REG\n2: Still lower gain as configured in RF_AGC_LUT_23_REG\n...\n9-F: Lowest gain as configured in RF_AGC_LUT_89_REG"]
    #[inline(always)]
    pub fn agcsetting_rd(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfAgcResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfAgcResultReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Frequency offset estimation (in 2s complement) with a resolution of approximately 5 kHz."]
    #[inline(always)]
    pub fn afc_rd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfAgcResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,RfAgcResultReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcResultReg {
    #[inline(always)]
    fn default() -> RfAgcResultReg {
        <crate::RegValueT<RfAgcResultReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfBmcwReg_SPEC;
impl crate::sealed::RegSpec for RfBmcwReg_SPEC {
    type DataType = u16;
}

#[doc = "Changed functionality of bits \\[7:6\\]"]
pub type RfBmcwReg = crate::RegValueT<RfBmcwReg_SPEC>;

impl RfBmcwReg {
    #[doc = "Select between:\n1 = use CN_WR as channel number\n0 = use BLE Frequency word (normal function)."]
    #[inline(always)]
    pub fn cn_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfBmcwReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfBmcwReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\\[7:6\\] = offset to RFCAL_CAP_WR, coarse calibraton LUT\n\\[5:0\\] = channel number"]
    #[inline(always)]
    pub fn cn_wr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfBmcwReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,RfBmcwReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfBmcwReg {
    #[inline(always)]
    fn default() -> RfBmcwReg {
        <crate::RegValueT<RfBmcwReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCalcap1Reg_SPEC;
impl crate::sealed::RegSpec for RfCalcap1Reg_SPEC {
    type DataType = u16;
}

pub type RfCalcap1Reg = crate::RegValueT<RfCalcap1Reg_SPEC>;

impl RfCalcap1Reg {
    #[doc = "Lowest 16 bits of vco_calcap"]
    #[inline(always)]
    pub fn vco_calcap_low(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, RfCalcap1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,RfCalcap1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfCalcap1Reg {
    #[inline(always)]
    fn default() -> RfCalcap1Reg {
        <crate::RegValueT<RfCalcap1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCalcap2Reg_SPEC;
impl crate::sealed::RegSpec for RfCalcap2Reg_SPEC {
    type DataType = u16;
}

pub type RfCalcap2Reg = crate::RegValueT<RfCalcap2Reg_SPEC>;

impl RfCalcap2Reg {
    #[doc = "Highest 2 bits of vco_calcap."]
    #[inline(always)]
    pub fn vco_calcap_high(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, RfCalcap2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,RfCalcap2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfCalcap2Reg {
    #[inline(always)]
    fn default() -> RfCalcap2Reg {
        <crate::RegValueT<RfCalcap2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCalstateReg_SPEC;
impl crate::sealed::RegSpec for RfCalstateReg_SPEC {
    type DataType = u16;
}

pub type RfCalstateReg = crate::RegValueT<RfCalstateReg_SPEC>;

impl RfCalstateReg {
    #[doc = "Value of the calstate state machine"]
    #[inline(always)]
    pub fn calstate(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfCalstateReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfCalstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfCalstateReg {
    #[inline(always)]
    fn default() -> RfCalstateReg {
        <crate::RegValueT<RfCalstateReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCalCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfCalCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfCalCtrlReg = crate::RegValueT<RfCalCtrlReg_SPEC>;

impl RfCalCtrlReg {
    #[doc = "Do not calibrate the VCO during Cal cycle"]
    #[inline(always)]
    pub fn vco_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Do not calibrate the VGA2 Offset during Cal cycle"]
    #[inline(always)]
    pub fn dc_offset_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Do not calibrate the IFF center frequency during Cal cycle"]
    #[inline(always)]
    pub fn iff_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Do not calibrate the VCO and Modulation Gain during Cal cycle"]
    #[inline(always)]
    pub fn mgain_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "End of calibration trigger.\nReading returns the eo_cal status signal which can be cleared by writing to RF_IRQ_CTRL_REG.EO_CAL_CLEAR"]
    #[inline(always)]
    pub fn eo_cal(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfCalCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,RfCalCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Start of calibration trigger.Writing a 1 starts calibration.1Reading returns the calibration status (1 = busy calibrating)."]
    #[inline(always)]
    pub fn so_cal(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfCalCtrlReg {
    #[inline(always)]
    fn default() -> RfCalCtrlReg {
        <crate::RegValueT<RfCalCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer10Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer10Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer10Reg = crate::RegValueT<RfCntrlTimer10Reg_SPEC>;

impl RfCntrlTimer10Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer10Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer10Reg {
        <crate::RegValueT<RfCntrlTimer10Reg_SPEC> as RegisterValue<_>>::new(554)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer11Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer11Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer11Reg = crate::RegValueT<RfCntrlTimer11Reg_SPEC>;

impl RfCntrlTimer11Reg {
    #[doc = "Offset w.r.t. end switch instant eo_tx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_tx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer11Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer11Reg {
        <crate::RegValueT<RfCntrlTimer11Reg_SPEC> as RegisterValue<_>>::new(564)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer12Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer12Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer12Reg = crate::RegValueT<RfCntrlTimer12Reg_SPEC>;

impl RfCntrlTimer12Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer12Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer12Reg {
        <crate::RegValueT<RfCntrlTimer12Reg_SPEC> as RegisterValue<_>>::new(572)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer13Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer13Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer13Reg = crate::RegValueT<RfCntrlTimer13Reg_SPEC>;

impl RfCntrlTimer13Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer13Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer13Reg {
        <crate::RegValueT<RfCntrlTimer13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer14Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer14Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer14Reg = crate::RegValueT<RfCntrlTimer14Reg_SPEC>;

impl RfCntrlTimer14Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfCntrlTimer14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer14Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer14Reg {
        <crate::RegValueT<RfCntrlTimer14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer1Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer1Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer1Reg = crate::RegValueT<RfCntrlTimer1Reg_SPEC>;

impl RfCntrlTimer1Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx"]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/so_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer1Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer1Reg {
        <crate::RegValueT<RfCntrlTimer1Reg_SPEC> as RegisterValue<_>>::new(2048)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer2Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer2Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer2Reg = crate::RegValueT<RfCntrlTimer2Reg_SPEC>;

impl RfCntrlTimer2Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/so_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer2Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer2Reg {
        <crate::RegValueT<RfCntrlTimer2Reg_SPEC> as RegisterValue<_>>::new(1288)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer3Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer3Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer3Reg = crate::RegValueT<RfCntrlTimer3Reg_SPEC>;

impl RfCntrlTimer3Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/so_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer3Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer3Reg {
        <crate::RegValueT<RfCntrlTimer3Reg_SPEC> as RegisterValue<_>>::new(528)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer4Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer4Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer4Reg = crate::RegValueT<RfCntrlTimer4Reg_SPEC>;

impl RfCntrlTimer4Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer4Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer4Reg {
        <crate::RegValueT<RfCntrlTimer4Reg_SPEC> as RegisterValue<_>>::new(554)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer5Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer5Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer5Reg = crate::RegValueT<RfCntrlTimer5Reg_SPEC>;

impl RfCntrlTimer5Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer5Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer5Reg {
        <crate::RegValueT<RfCntrlTimer5Reg_SPEC> as RegisterValue<_>>::new(536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer6Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer6Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer6Reg = crate::RegValueT<RfCntrlTimer6Reg_SPEC>;

impl RfCntrlTimer6Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer6Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer6Reg {
        <crate::RegValueT<RfCntrlTimer6Reg_SPEC> as RegisterValue<_>>::new(562)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer7Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer7Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer7Reg = crate::RegValueT<RfCntrlTimer7Reg_SPEC>;

impl RfCntrlTimer7Reg {
    #[doc = "Offset w.r.t. end switch instant eo_tx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_tx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer7Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer7Reg {
        <crate::RegValueT<RfCntrlTimer7Reg_SPEC> as RegisterValue<_>>::new(536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer8Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer8Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer8Reg = crate::RegValueT<RfCntrlTimer8Reg_SPEC>;

impl RfCntrlTimer8Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer8Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer8Reg {
        <crate::RegValueT<RfCntrlTimer8Reg_SPEC> as RegisterValue<_>>::new(564)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer9Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer9Reg_SPEC {
    type DataType = u16;
}

pub type RfCntrlTimer9Reg = crate::RegValueT<RfCntrlTimer9Reg_SPEC>;

impl RfCntrlTimer9Reg {
    #[doc = "Offset w.r.t. end switch instant eo_rx/eo_tx."]
    #[inline(always)]
    pub fn reset_offset(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfCntrlTimer9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Offset w.r.t. start switch instant so_rx/eo_tx."]
    #[inline(always)]
    pub fn set_offset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfCntrlTimer9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer9Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer9Reg {
        <crate::RegValueT<RfCntrlTimer9Reg_SPEC> as RegisterValue<_>>::new(569)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCpCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfCpCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfCpCtrlReg = crate::RegValueT<RfCpCtrlReg_SPEC>;

impl RfCpCtrlReg {
    #[doc = "CP current setting during PLL - Lock in TX mode\n1111: 45 µA (fastest, setting 0)\n0111: 15 µA (setting 1)\n0011: 7.5 µA (setting 2)\n0001: 3.75 µA (slowest, setting 3)\nIntermediate values are possible (but not recommended). Calculate the effective value with:\nbit 0: 3.75 µA\nbit 1: 3.75 µA\nbit 2: 7.5 µA\nbit 3: 30 µA"]
    #[inline(always)]
    pub fn cp_cur_tx(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, RfCpCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,RfCpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CP current setting during PLL - lock in RX mode\nsame coding as for CP_CUR_TX"]
    #[inline(always)]
    pub fn cp_cur_rx(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfCpCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfCpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Chargepump current setting during PLL settling in TX mode\nsame coding as for CP_CUR_TX"]
    #[inline(always)]
    pub fn cp_cur_set_tx(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfCpCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfCpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Chargepump current setting during PLL settling in RX mode.\nsame coding as for CP_CUR_TX"]
    #[inline(always)]
    pub fn cp_cur_set_rx(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfCpCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfCpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfCpCtrlReg {
    #[inline(always)]
    fn default() -> RfCpCtrlReg {
        <crate::RegValueT<RfCpCtrlReg_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfDcOffsetCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfDcOffsetCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type RfDcOffsetCtrl1Reg = crate::RegValueT<RfDcOffsetCtrl1Reg_SPEC>;

impl RfDcOffsetCtrl1Reg {
    #[doc = "DC offset compensation value in Q channel valid when DCOFFSET_SEL = 1"]
    #[inline(always)]
    pub fn dcoffset_q_wr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DC offset compensation value in I channel valid when DCOFFSET_SEL = 1"]
    #[inline(always)]
    pub fn dcoffset_i_wr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfDcOffsetCtrl1Reg {
    #[inline(always)]
    fn default() -> RfDcOffsetCtrl1Reg {
        <crate::RegValueT<RfDcOffsetCtrl1Reg_SPEC> as RegisterValue<_>>::new(32896)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfDcOffsetCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfDcOffsetCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type RfDcOffsetCtrl2Reg = crate::RegValueT<RfDcOffsetCtrl2Reg_SPEC>;

impl RfDcOffsetCtrl2Reg {
    #[doc = "Number of gain settings for the full DC offset calibration"]
    #[inline(always)]
    pub fn dcngain(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x3,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x3,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Number of the steps per.gain setting for the full or partial DC offset calibrations"]
    #[inline(always)]
    pub fn dcnstep(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Selects the pole of the digital high pass fitlers\nEncoding: TBD"]
    #[inline(always)]
    pub fn dcpole(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable flag for the partial DC offset calibration (executed when the demodulator is enabled)."]
    #[inline(always)]
    pub fn dcparcal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfDcOffsetCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfDcOffsetCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': Normal operation\n\'1\': Use the manual DC offset compensation values from RF_DC_OFFSET_CTRL1_REG"]
    #[inline(always)]
    pub fn dcoffset_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfDcOffsetCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfDcOffsetCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfDcOffsetCtrl2Reg {
    #[inline(always)]
    fn default() -> RfDcOffsetCtrl2Reg {
        <crate::RegValueT<RfDcOffsetCtrl2Reg_SPEC> as RegisterValue<_>>::new(304)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfDcOffsetCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for RfDcOffsetCtrl3Reg_SPEC {
    type DataType = u16;
}

pub type RfDcOffsetCtrl3Reg = crate::RegValueT<RfDcOffsetCtrl3Reg_SPEC>;

impl RfDcOffsetCtrl3Reg {
    #[doc = "Quadrature feedback gain for the DC offset calibration"]
    #[inline(always)]
    pub fn dcbeta_q(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Inphase feedback gain for the DC offset calibration"]
    #[inline(always)]
    pub fn dcbeta_i(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfDcOffsetCtrl3Reg {
    #[inline(always)]
    fn default() -> RfDcOffsetCtrl3Reg {
        <crate::RegValueT<RfDcOffsetCtrl3Reg_SPEC> as RegisterValue<_>>::new(46097)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfDcOffsetCtrl4Reg_SPEC;
impl crate::sealed::RegSpec for RfDcOffsetCtrl4Reg_SPEC {
    type DataType = u16;
}

pub type RfDcOffsetCtrl4Reg = crate::RegValueT<RfDcOffsetCtrl4Reg_SPEC>;

impl RfDcOffsetCtrl4Reg {
    #[doc = "AGC setting for forth last the gain step for the full DC offset calibration"]
    #[inline(always)]
    pub fn dcagcsetting_full3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGC setting for third last the gain step for the full DC offset calibration"]
    #[inline(always)]
    pub fn dcagcsetting_full2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGC setting for second last the gain step for the full DC offset calibration"]
    #[inline(always)]
    pub fn dcagcsetting_full1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "AGC setting for last the gain step for the full DC offset calibration"]
    #[inline(always)]
    pub fn dcagcsetting_full0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        RfDcOffsetCtrl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            RfDcOffsetCtrl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfDcOffsetCtrl4Reg {
    #[inline(always)]
    fn default() -> RfDcOffsetCtrl4Reg {
        <crate::RegValueT<RfDcOffsetCtrl4Reg_SPEC> as RegisterValue<_>>::new(39200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfDcOffsetResultReg_SPEC;
impl crate::sealed::RegSpec for RfDcOffsetResultReg_SPEC {
    type DataType = u16;
}

#[doc = "Must be Retained"]
pub type RfDcOffsetResultReg = crate::RegValueT<RfDcOffsetResultReg_SPEC>;

impl RfDcOffsetResultReg {
    #[doc = "DC offset compensation value in Q channel valid when DCOFFSET_SEL = 0"]
    #[inline(always)]
    pub fn dcoffset_q_rd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfDcOffsetResultReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfDcOffsetResultReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DC offset compensation value in I channel valid when DCOFFSET_SEL = 0."]
    #[inline(always)]
    pub fn dcoffset_i_rd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfDcOffsetResultReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfDcOffsetResultReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfDcOffsetResultReg {
    #[inline(always)]
    fn default() -> RfDcOffsetResultReg {
        <crate::RegValueT<RfDcOffsetResultReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfDemCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfDemCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfDemCtrlReg = crate::RegValueT<RfDemCtrlReg_SPEC>;

impl RfDemCtrlReg {
    #[doc = "Enable the equalizer in the demodulator"]
    #[inline(always)]
    pub fn equal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfDemCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfDemCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Threshold for the 0101 pattern matching"]
    #[inline(always)]
    pub fn match0101_th(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, u8, RfDemCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,u8,u8,RfDemCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Invert \'frequency\' polarity of the demodulator"]
    #[inline(always)]
    pub fn dem_hsi_pol(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfDemCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfDemCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': Normal operation\n\'1\': Invert the polarity of the received bits"]
    #[inline(always)]
    pub fn rxdata_inv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfDemCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfDemCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfDemCtrlReg {
    #[inline(always)]
    fn default() -> RfDemCtrlReg {
        <crate::RegValueT<RfDemCtrlReg_SPEC> as RegisterValue<_>>::new(88)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig10Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig10Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig10Reg = crate::RegValueT<RfEnableConfig10Reg_SPEC>;

impl RfEnableConfig10Reg {
    #[doc = "Timing configuration for enable of the ADC"]
    #[inline(always)]
    pub fn adc_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the VCO"]
    #[inline(always)]
    pub fn vco_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig10Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig10Reg {
        <crate::RegValueT<RfEnableConfig10Reg_SPEC> as RegisterValue<_>>::new(1075)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig11Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig11Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig11Reg = crate::RegValueT<RfEnableConfig11Reg_SPEC>;

impl RfEnableConfig11Reg {
    #[doc = "Timing configuration for enable of main divider of the LO buffer"]
    #[inline(always)]
    pub fn md_lobuf_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of CP"]
    #[inline(always)]
    pub fn cp_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig11Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig11Reg {
        <crate::RegValueT<RfEnableConfig11Reg_SPEC> as RegisterValue<_>>::new(13107)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig12Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig12Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig12Reg = crate::RegValueT<RfEnableConfig12Reg_SPEC>;

impl RfEnableConfig12Reg {
    #[doc = "Timing configuration for the phase frequency detector"]
    #[inline(always)]
    pub fn pfd_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for the gauss module"]
    #[inline(always)]
    pub fn gauss_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig12Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig12Reg {
        <crate::RegValueT<RfEnableConfig12Reg_SPEC> as RegisterValue<_>>::new(13104)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig13Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig13Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig13Reg = crate::RegValueT<RfEnableConfig13Reg_SPEC>;

impl RfEnableConfig13Reg {
    #[doc = "Timing configuration for the rfio"]
    #[inline(always)]
    pub fn rfio_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for the PA lobuffer"]
    #[inline(always)]
    pub fn lobuf_pa_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig13Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig13Reg {
        <crate::RegValueT<RfEnableConfig13Reg_SPEC> as RegisterValue<_>>::new(36912)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig14Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig14Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig14Reg = crate::RegValueT<RfEnableConfig14Reg_SPEC>;

impl RfEnableConfig14Reg {
    #[doc = "Timing configuration for the rxi lobuffer"]
    #[inline(always)]
    pub fn lobuf_rxiq_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for the 2 divider"]
    #[inline(always)]
    pub fn div2_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig14Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig14Reg {
        <crate::RegValueT<RfEnableConfig14Reg_SPEC> as RegisterValue<_>>::new(819)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig15Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig15Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig15Reg = crate::RegValueT<RfEnableConfig15Reg_SPEC>;

impl RfEnableConfig15Reg {
    #[doc = "Timing configuration for the CP bias S/H switch"]
    #[inline(always)]
    pub fn cp_bias_sh_open(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for the VCO bias S/H switch"]
    #[inline(always)]
    pub fn vco_bias_sh_open_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig15Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig15Reg {
        <crate::RegValueT<RfEnableConfig15Reg_SPEC> as RegisterValue<_>>::new(21845)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig16Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig16Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig16Reg = crate::RegValueT<RfEnableConfig16Reg_SPEC>;

impl RfEnableConfig16Reg {
    #[doc = "Timing configuration for iffmix bias S/H switch"]
    #[inline(always)]
    pub fn iff_bias_sh_open_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig16Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig16Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for gauss bias S/H switch"]
    #[inline(always)]
    pub fn gauss_bias_sh_open_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig16Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig16Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig16Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig16Reg {
        <crate::RegValueT<RfEnableConfig16Reg_SPEC> as RegisterValue<_>>::new(1616)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig17Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig17Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig17Reg = crate::RegValueT<RfEnableConfig17Reg_SPEC>;

impl RfEnableConfig17Reg {
    #[doc = "Timing configuration for pa bias S/H switch"]
    #[inline(always)]
    pub fn mix_bias_sh_open_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig17Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig17Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for the plldig"]
    #[inline(always)]
    pub fn plldig_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig17Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig17Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig17Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig17Reg {
        <crate::RegValueT<RfEnableConfig17Reg_SPEC> as RegisterValue<_>>::new(1587)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig18Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig18Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig18Reg = crate::RegValueT<RfEnableConfig18Reg_SPEC>;

impl RfEnableConfig18Reg {
    #[doc = "Timing configuration for pllclosed"]
    #[inline(always)]
    pub fn pllclosed_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig18Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig18Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for demodulator"]
    #[inline(always)]
    pub fn dem_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig18Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig18Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig18Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig18Reg {
        <crate::RegValueT<RfEnableConfig18Reg_SPEC> as RegisterValue<_>>::new(30472)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig19Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig19Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig19Reg = crate::RegValueT<RfEnableConfig19Reg_SPEC>;

impl RfEnableConfig19Reg {
    #[doc = "Timing configuration for radio LDO auto zero enable"]
    #[inline(always)]
    pub fn ldo_zero_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig19Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig19Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for calibration slot"]
    #[inline(always)]
    pub fn cal_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig19Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig19Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig19Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig19Reg {
        <crate::RegValueT<RfEnableConfig19Reg_SPEC> as RegisterValue<_>>::new(4352)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig1Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig1Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig1Reg = crate::RegValueT<RfEnableConfig1Reg_SPEC>;

impl RfEnableConfig1Reg {
    #[doc = "Timing configuration for enable of the lna ldo"]
    #[inline(always)]
    pub fn lna_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the lna core"]
    #[inline(always)]
    pub fn lna_core_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig1Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig1Reg {
        <crate::RegValueT<RfEnableConfig1Reg_SPEC> as RegisterValue<_>>::new(521)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig20Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig20Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig20Reg = crate::RegValueT<RfEnableConfig20Reg_SPEC>;

impl RfEnableConfig20Reg {
    #[doc = "Timing configuration for time to digital converter"]
    #[inline(always)]
    pub fn tdc_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for RFIO LDO"]
    #[inline(always)]
    pub fn ldo_rfio_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig20Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig20Reg {
        <crate::RegValueT<RfEnableConfig20Reg_SPEC> as RegisterValue<_>>::new(34)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig21Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig21Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig21Reg = crate::RegValueT<RfEnableConfig21Reg_SPEC>;

impl RfEnableConfig21Reg {
    #[doc = "Timing configuration for bias block for RFIO and RFPA"]
    #[inline(always)]
    pub fn rfio_bias_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig21Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for S/H switch of bias block for RFIO/RFPA"]
    #[inline(always)]
    pub fn rfio_bias_sh_open(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig21Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig21Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig21Reg {
        <crate::RegValueT<RfEnableConfig21Reg_SPEC> as RegisterValue<_>>::new(41136)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig22Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig22Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig22Reg = crate::RegValueT<RfEnableConfig22Reg_SPEC>;

impl RfEnableConfig22Reg {
    #[doc = "Timing configuration for LDO for the radio IO buffer"]
    #[inline(always)]
    pub fn ldo_radio_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig22Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig22Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for the enable of the ADC clock"]
    #[inline(always)]
    pub fn adc_clk_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig22Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig22Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig22Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig22Reg {
        <crate::RegValueT<RfEnableConfig22Reg_SPEC> as RegisterValue<_>>::new(8712)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig23Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig23Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig23Reg = crate::RegValueT<RfEnableConfig23Reg_SPEC>;

impl RfEnableConfig23Reg {
    #[doc = "Timing configuration for tr_pwm_off_en"]
    #[inline(always)]
    pub fn tr_pwm_off_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig23Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig23Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for spare_en_3"]
    #[inline(always)]
    pub fn spare_en_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig23Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig23Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig23Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig23Reg {
        <crate::RegValueT<RfEnableConfig23Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig2Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig2Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig2Reg = crate::RegValueT<RfEnableConfig2Reg_SPEC>;

impl RfEnableConfig2Reg {
    #[doc = "Timing configuration for enable of the lna cgm"]
    #[inline(always)]
    pub fn lna_cgm_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the mix ldo"]
    #[inline(always)]
    pub fn mix_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig2Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig2Reg {
        <crate::RegValueT<RfEnableConfig2Reg_SPEC> as RegisterValue<_>>::new(290)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig3Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig3Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig3Reg = crate::RegValueT<RfEnableConfig3Reg_SPEC>;

impl RfEnableConfig3Reg {
    #[doc = "Timing configuration for enable of the iff ldo"]
    #[inline(always)]
    pub fn iff_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the ifadc ldo"]
    #[inline(always)]
    pub fn ifadc_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig3Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig3Reg {
        <crate::RegValueT<RfEnableConfig3Reg_SPEC> as RegisterValue<_>>::new(8708)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig4Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig4Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig4Reg = crate::RegValueT<RfEnableConfig4Reg_SPEC>;

impl RfEnableConfig4Reg {
    #[doc = "Timing configuration for enable of the vco ldo"]
    #[inline(always)]
    pub fn vco_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the md ldo"]
    #[inline(always)]
    pub fn md_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig4Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig4Reg {
        <crate::RegValueT<RfEnableConfig4Reg_SPEC> as RegisterValue<_>>::new(8738)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig5Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig5Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig5Reg = crate::RegValueT<RfEnableConfig5Reg_SPEC>;

impl RfEnableConfig5Reg {
    #[doc = "Timing configuration for enable of the pfd ldo"]
    #[inline(always)]
    pub fn pfd_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the pa ldo"]
    #[inline(always)]
    pub fn pa_ldo_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig5Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig5Reg {
        <crate::RegValueT<RfEnableConfig5Reg_SPEC> as RegisterValue<_>>::new(8738)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig6Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig6Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig6Reg = crate::RegValueT<RfEnableConfig6Reg_SPEC>;

impl RfEnableConfig6Reg {
    #[doc = "Timing configuration for the dynamic CP current switching"]
    #[inline(always)]
    pub fn cp_switch_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for the VCO bias"]
    #[inline(always)]
    pub fn vco_bias_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig6Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig6Reg {
        <crate::RegValueT<RfEnableConfig6Reg_SPEC> as RegisterValue<_>>::new(1075)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig7Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig7Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig7Reg = crate::RegValueT<RfEnableConfig7Reg_SPEC>;

impl RfEnableConfig7Reg {
    #[doc = "Timing configuration for enable of the CP bias"]
    #[inline(always)]
    pub fn cp_bias_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "autozero control signal of the lna ldo"]
    #[inline(always)]
    pub fn lna_ldo_zero(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig7Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig7Reg {
        <crate::RegValueT<RfEnableConfig7Reg_SPEC> as RegisterValue<_>>::new(13056)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig8Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig8Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig8Reg = crate::RegValueT<RfEnableConfig8Reg_SPEC>;

impl RfEnableConfig8Reg {
    #[doc = "Timing configuration for enable of the PA ramp"]
    #[inline(always)]
    pub fn pa_ramp_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the PA"]
    #[inline(always)]
    pub fn pa_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig8Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig8Reg {
        <crate::RegValueT<RfEnableConfig8Reg_SPEC> as RegisterValue<_>>::new(49312)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig9Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig9Reg_SPEC {
    type DataType = u16;
}

pub type RfEnableConfig9Reg = crate::RegValueT<RfEnableConfig9Reg_SPEC>;

impl RfEnableConfig9Reg {
    #[doc = "Timing configuration for enable of the mixer"]
    #[inline(always)]
    pub fn mix_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Timing configuration for enable of the iff"]
    #[inline(always)]
    pub fn iff_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        RfEnableConfig9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfEnableConfig9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig9Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig9Reg {
        <crate::RegValueT<RfEnableConfig9Reg_SPEC> as RegisterValue<_>>::new(1028)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfIffCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfIffCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type RfIffCtrl1Reg = crate::RegValueT<RfIffCtrl1Reg_SPEC>;

impl RfIffCtrl1Reg {
    #[doc = "Disable the DC offset current DAC"]
    #[inline(always)]
    pub fn iff_dcoc_dac_dis(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': normal operation;\n\'1\': Enable reference oscillator."]
    #[inline(always)]
    pub fn ro_to_pins(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': normal operation;\n\'1\': Mute IFF by short circuit of VGA1 input.\nNote: set TGATE_MIXER_IF to \'0\' for isolation from the IRM"]
    #[inline(always)]
    pub fn if_mute(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': use value as determined by IF calibration for IF filter;\n\'1\': use the value written to IF_CAL_CAP for IF filter."]
    #[inline(always)]
    pub fn if_cal_cap_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "External value for IF calibration capacitance"]
    #[inline(always)]
    pub fn if_cal_cap_wr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, RfIffCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfIffCtrl1Reg {
    #[inline(always)]
    fn default() -> RfIffCtrl1Reg {
        <crate::RegValueT<RfIffCtrl1Reg_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfIffResultReg_SPEC;
impl crate::sealed::RegSpec for RfIffResultReg_SPEC {
    type DataType = u16;
}

#[doc = "Must be Retained"]
pub type RfIffResultReg = crate::RegValueT<RfIffResultReg_SPEC>;

impl RfIffResultReg {
    #[doc = "IF calibration result capacitance."]
    #[inline(always)]
    pub fn if_cal_cap_rd(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, RfIffResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,RfIffResultReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfIffResultReg {
    #[inline(always)]
    fn default() -> RfIffResultReg {
        <crate::RegValueT<RfIffResultReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfIrqCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfIrqCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfIrqCtrlReg = crate::RegValueT<RfIrqCtrlReg_SPEC>;

impl RfIrqCtrlReg {
    #[doc = "Writing any value to this bit clears eo_cal interrupt."]
    #[inline(always)]
    pub fn eo_cal_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfIrqCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RfIrqCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfIrqCtrlReg {
    #[inline(always)]
    fn default() -> RfIrqCtrlReg {
        <crate::RegValueT<RfIrqCtrlReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLfCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfLfCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfLfCtrlReg = crate::RegValueT<RfLfCtrlReg_SPEC>;

impl RfLfCtrlReg {
    #[doc = "\'0\': R4 in place, \'1\': R4 shorted, C2 and C4 in parallel."]
    #[inline(always)]
    pub fn lf_short_r4(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfLfCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfLfCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': Normal operation: use IF_CAL_CAP_RD (as determined by IF calibration) for the loop filter capacitance;\n\'1\': use the value written to LF_CAL_CAP_WR"]
    #[inline(always)]
    pub fn lf_cal_cap_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfLfCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfLfCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "External value for loop filter calibration capacitance"]
    #[inline(always)]
    pub fn lf_cal_cap_wr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, RfLfCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,RfLfCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLfCtrlReg {
    #[inline(always)]
    fn default() -> RfLfCtrlReg {
        <crate::RegValueT<RfLfCtrlReg_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLfResCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfLfResCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "LF resistor setting"]
pub type RfLfResCtrlReg = crate::RegValueT<RfLfResCtrlReg_SPEC>;

impl RfLfResCtrlReg {
    #[doc = "Loopfilter resistor setting during PLL - Lock in TX mode\n1xxx: 72 k (fastest, setting 0)\n01xx: 120 k (setting 1)\n001x: 168 k (setting2)\n000x: 240 k (slowest, setting 3)"]
    #[inline(always)]
    pub fn lf_res_tx(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, RfLfResCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,RfLfResCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Loopfilter resistor setting during PLL - Lock in RX mode\nsame coding as for LF_RES_TX"]
    #[inline(always)]
    pub fn lf_res_rx(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfLfResCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfLfResCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Loopfilter resistor setting during PLL settling in TX mode\n\nsame coding as for LF_RES_TX"]
    #[inline(always)]
    pub fn lf_res_set_tx(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfLfResCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfLfResCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Loopfilter resistor setting during PLL settling in RX mode\nsame coding as for LF_RES_TX"]
    #[inline(always)]
    pub fn lf_res_set_rx(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfLfResCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfLfResCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLfResCtrlReg {
    #[inline(always)]
    fn default() -> RfLfResCtrlReg {
        <crate::RegValueT<RfLfResCtrlReg_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfMgainCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfMgainCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type RfMgainCtrl2Reg = crate::RegValueT<RfMgainCtrl2Reg_SPEC>;

impl RfMgainCtrl2Reg {
    #[doc = "Number of symbols for transmit0 and transmit1 length during mgain calibration."]
    #[inline(always)]
    pub fn mgain_transmit_length(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, RfMgainCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,RfMgainCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfMgainCtrl2Reg {
    #[inline(always)]
    fn default() -> RfMgainCtrl2Reg {
        <crate::RegValueT<RfMgainCtrl2Reg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfMgainCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfMgainCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfMgainCtrlReg = crate::RegValueT<RfMgainCtrlReg_SPEC>;

impl RfMgainCtrlReg {
    #[doc = "Kmod channel dependent trimming constant.\n0: No trimming is activated\n>0: The modulation gain in the direct path is modified with a factor: 1-SGNx(KMOD_ALPHA+2)x(CN-CN_CAL_RD)/2048"]
    #[inline(always)]
    pub fn kmod_alpha(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, RfMgainCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Average over a number of comparator output values\n0: 1 value\n1: 3 values\n2: 5 values\n3: 7 values"]
    #[inline(always)]
    pub fn mgain_aver(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, RfMgainCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Invert the output of the modulation gain comparator before usage."]
    #[inline(always)]
    pub fn mgain_cmp_inv(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RfMgainCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Length of a modulation gain calibration step\n0: 4 symbol periods\n1: 8 symbol periods"]
    #[inline(always)]
    pub fn mgain_dbl_transmit(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfMgainCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal operation\n1: Use GAUSS_GAIN_WR for the modulation gain"]
    #[inline(always)]
    pub fn gauss_gain_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfMgainCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfMgainCtrlReg {
    #[inline(always)]
    fn default() -> RfMgainCtrlReg {
        <crate::RegValueT<RfMgainCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfMgcCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfMgcCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfMgcCtrlReg = crate::RegValueT<RfMgcCtrlReg_SPEC>;

impl RfMgcCtrlReg {
    #[doc = "Reserved bits for Gauss DAC settings"]
    #[inline(always)]
    pub fn gauss_dac_ctrl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RfMgcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RfMgcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Switch in an aditional pole on the mgc amplifer to have extra filtering of the loopfilter voltage.\n\'0\': no pole (bandwidth limited by the opamp)\n\'1\': switch in pole to reduce amplifier bandwidth"]
    #[inline(always)]
    pub fn mgc_pole_sw(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfMgcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfMgcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set the desired gain for the mgc calibration amplifier.\n\'0\': gain= 15x\n\'1\': gain= 10x"]
    #[inline(always)]
    pub fn mgc_gain_set(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfMgcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfMgcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfMgcCtrlReg {
    #[inline(always)]
    fn default() -> RfMgcCtrlReg {
        <crate::RegValueT<RfMgcCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfMixerCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfMixerCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type RfMixerCtrl1Reg = crate::RegValueT<RfMixerCtrl1Reg_SPEC>;

impl RfMixerCtrl1Reg {
    #[doc = "Spare registers for mixer control"]
    #[inline(always)]
    pub fn mix_spare(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim the common mode voltage at the input of the TIA\n0: Minimum voltage\n8: Nominal voltage\nF: Maximal voltage"]
    #[inline(always)]
    pub fn mix_trim_vcm(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim the bias current of the TIA\n0: Minimum bias current\n8: Nominal bias current\nF: Maximal bias current"]
    #[inline(always)]
    pub fn mix_trim_ibias(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim the Mixer bias resistor for optimum transcunductance\n0: Minimum transconductance\n8: Nominal transconductance\nF: Maximal transconductance"]
    #[inline(always)]
    pub fn mix_trim_gmbias(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfMixerCtrl1Reg {
    #[inline(always)]
    fn default() -> RfMixerCtrl1Reg {
        <crate::RegValueT<RfMixerCtrl1Reg_SPEC> as RegisterValue<_>>::new(53)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfMixerCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfMixerCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type RfMixerCtrl2Reg = crate::RegValueT<RfMixerCtrl2Reg_SPEC>;

impl RfMixerCtrl2Reg {
    #[doc = "\'0\': Normal operation: use IF_CAL_CAP_RD (as determined by IF calibration) for the loop filter capacitance;\n\'1\': use the value written to MIX_CAL_CAP_WR"]
    #[inline(always)]
    pub fn mix_cal_cap_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfMixerCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfMixerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "External value for calibration of mixer pole capacitance"]
    #[inline(always)]
    pub fn mix_cal_cap_wr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, RfMixerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,RfMixerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfMixerCtrl2Reg {
    #[inline(always)]
    fn default() -> RfMixerCtrl2Reg {
        <crate::RegValueT<RfMixerCtrl2Reg_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfOverruleReg_SPEC;
impl crate::sealed::RegSpec for RfOverruleReg_SPEC {
    type DataType = u16;
}

pub type RfOverruleReg = crate::RegValueT<RfOverruleReg_SPEC>;

impl RfOverruleReg {
    #[doc = "Enable rx_en"]
    #[inline(always)]
    pub fn rx_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable rx_en"]
    #[inline(always)]
    pub fn rx_dis_wr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable tx_en"]
    #[inline(always)]
    pub fn tx_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable tx_en"]
    #[inline(always)]
    pub fn tx_dis_wr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfOverruleReg {
    #[inline(always)]
    fn default() -> RfOverruleReg {
        <crate::RegValueT<RfOverruleReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfPaCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfPaCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Removed obsolete values of bits 10:7, pa_pw back to 4"]
pub type RfPaCtrlReg = crate::RegValueT<RfPaCtrlReg_SPEC>;

impl RfPaCtrlReg {
    #[doc = "Control for PA supply voltage (output power)"]
    #[inline(always)]
    pub fn level_ldo_rfpa(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, u8, RfPaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0xf,1,0,u8,u8,RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ramping speed setting of the driver stage:\n\'0x0\': slowest (1.25 uA)\n\'0x1\': 2x faster (2.5 uA)\n\'0x2\': default ramping speed\n\'0x3\': fastest"]
    #[inline(always)]
    pub fn pa_rampspeed(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, RfPaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pulse width setting to control HD2\n\'0\': not active\n\'1\': 48.8 percent duty cycle\n\'2\': 49.4 percent duty cycle\n\'3\': 49.7 percent duty cycle\n\'4\': 50   percent duty cycle (default)\n\'5\': 50.3 percent duty cycle\n\'6\': 50.6 percent duty cycle\n\'7\': 51.2 percent duty cycle"]
    #[inline(always)]
    pub fn pa_pw(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, RfPaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets gain/DC Current setting of the differential to single ended converter\n\'0\': smallest current setting (60 uA)\n\'1\': current setting 1 (80 uA)\n\'2\': default (100 uA)\n\'3\': largest current setting (120 uA)"]
    #[inline(always)]
    pub fn pa_gain(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, RfPaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfPaCtrlReg {
    #[inline(always)]
    fn default() -> RfPaCtrlReg {
        <crate::RegValueT<RfPaCtrlReg_SPEC> as RegisterValue<_>>::new(31959)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfPfdCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfPfdCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfPfdCtrlReg = crate::RegValueT<RfPfdCtrlReg_SPEC>;

impl RfPfdCtrlReg {
    #[doc = "\'0\': Normal operation (UP: implies RCLK leads NCLK)\n\'1\': Inverted operation (UP: implies NCLK leads RCLK)"]
    #[inline(always)]
    pub fn pfd_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfPfdCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfPfdCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable manual override of PFD output\n\'0\': Normal operation\n\'1\': PFD ouput given by FIXED_CUR_SET"]
    #[inline(always)]
    pub fn fixed_cur_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfPfdCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfPfdCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set the PFD output in a fixed position such that the CP output current is constant\'0\': UP = 0, DN = 0\n\'1\': UP = 0, DN = 1\n\'2\': UP = 1, DN = 0\n\'3\': UP = 1, DN = 1"]
    #[inline(always)]
    pub fn fixed_cur_set(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, RfPfdCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,RfPfdCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfPfdCtrlReg {
    #[inline(always)]
    fn default() -> RfPfdCtrlReg {
        <crate::RegValueT<RfPfdCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfRadigSpareReg_SPEC;
impl crate::sealed::RegSpec for RfRadigSpareReg_SPEC {
    type DataType = u16;
}

pub type RfRadigSpareReg = crate::RegValueT<RfRadigSpareReg_SPEC>;

impl RfRadigSpareReg {
    #[doc = "Spare bits to be defined later"]
    #[inline(always)]
    pub fn radig_spare(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1fff,
        1,
        0,
        u16,
        u16,
        RfRadigSpareReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1fff,
            1,
            0,
            u16,
            u16,
            RfRadigSpareReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfRadigSpareReg {
    #[inline(always)]
    fn default() -> RfRadigSpareReg {
        <crate::RegValueT<RfRadigSpareReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfRefOscReg_SPEC;
impl crate::sealed::RegSpec for RfRefOscReg_SPEC {
    type DataType = u16;
}

pub type RfRefOscReg = crate::RegValueT<RfRefOscReg_SPEC>;

impl RfRefOscReg {
    #[doc = "number of clock pulses corresponding to the value of CNT_RO"]
    #[inline(always)]
    pub fn cnt_clk(
        self,
    ) -> crate::common::RegisterField<6, 0x1ff, 1, 0, u16, u16, RfRefOscReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1ff,1,0,u16,u16,RfRefOscReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "number of reference oscillator periods that need to be counted"]
    #[inline(always)]
    pub fn cnt_ro(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, RfRefOscReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,RfRefOscReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfRefOscReg {
    #[inline(always)]
    fn default() -> RfRefOscReg {
        <crate::RegValueT<RfRefOscReg_SPEC> as RegisterValue<_>>::new(8790)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfRssiResultReg_SPEC;
impl crate::sealed::RegSpec for RfRssiResultReg_SPEC {
    type DataType = u16;
}

pub type RfRssiResultReg = crate::RegValueT<RfRssiResultReg_SPEC>;

impl RfRssiResultReg {
    #[doc = "RSSI value measured in averaging mode in continuous RX mode (used for LNA selectivity calibration)"]
    #[inline(always)]
    pub fn rssi_avg_rd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3ff,
        1,
        0,
        u16,
        u16,
        RfRssiResultReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x3ff,
            1,
            0,
            u16,
            u16,
            RfRssiResultReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "RSSI value measured in peak-hold mode during the preamble and Access Addres detection"]
    #[inline(always)]
    pub fn rssi_ph_rd(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, RfRssiResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,RfRssiResultReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfRssiResultReg {
    #[inline(always)]
    fn default() -> RfRssiResultReg {
        <crate::RegValueT<RfRssiResultReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfScanFeedbackReg_SPEC;
impl crate::sealed::RegSpec for RfScanFeedbackReg_SPEC {
    type DataType = u16;
}

pub type RfScanFeedbackReg = crate::RegValueT<RfScanFeedbackReg_SPEC>;

impl RfScanFeedbackReg {
    #[doc = "Cp_cur value during scan."]
    #[inline(always)]
    pub fn cp_cur(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfScanFeedbackReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfScanFeedbackReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Lf_res value during scan."]
    #[inline(always)]
    pub fn lf_res(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfScanFeedbackReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfScanFeedbackReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfScanFeedbackReg {
    #[inline(always)]
    fn default() -> RfScanFeedbackReg {
        <crate::RegValueT<RfScanFeedbackReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSpare1Reg_SPEC;
impl crate::sealed::RegSpec for RfSpare1Reg_SPEC {
    type DataType = u16;
}

pub type RfSpare1Reg = crate::RegValueT<RfSpare1Reg_SPEC>;

impl RfSpare1Reg {
    #[doc = "Spare bits for radio"]
    #[inline(always)]
    pub fn rf_spare1(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, RfSpare1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,RfSpare1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Choose the transfer function mode of the IFF\n0: Normal operation (complex)\n1: Test mode (real, approx. 16 dB less gain at 1 MHz than in complex mode);"]
    #[inline(always)]
    pub fn iff_real_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfSpare1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfSpare1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfSpare1Reg {
    #[inline(always)]
    fn default() -> RfSpare1Reg {
        <crate::RegValueT<RfSpare1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSynthCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfSynthCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type RfSynthCtrl1Reg = crate::RegValueT<RfSynthCtrl1Reg_SPEC>;

impl RfSynthCtrl1Reg {
    #[doc = "High Side Injection polarity\n0: LO frequency is lower than the wanted RF frequency\n1: LO frequency is higher than the wanted RF frequency"]
    #[inline(always)]
    pub fn pll_hsi_pol(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfSynthCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfSynthCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel Spacing\n0: 1MHz\n1: 2MHz"]
    #[inline(always)]
    pub fn cs(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RfSynthCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,RfSynthCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sign bit for the channel step\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn sgn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfSynthCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfSynthCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 0 frequency in MHz"]
    #[inline(always)]
    pub fn channel_zero(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        RfSynthCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            RfSynthCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfSynthCtrl1Reg {
    #[inline(always)]
    fn default() -> RfSynthCtrl1Reg {
        <crate::RegValueT<RfSynthCtrl1Reg_SPEC> as RegisterValue<_>>::new(10594)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSynthCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfSynthCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type RfSynthCtrl2Reg = crate::RegValueT<RfSynthCtrl2Reg_SPEC>;

impl RfSynthCtrl2Reg {
    #[doc = "0: BT = 0.5; 1: BT = 0.6"]
    #[inline(always)]
    pub fn bt_sel(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable the end of packet detection\n\n0: End of packet detection enabled\n1: End of pakcet detection disabled"]
    #[inline(always)]
    pub fn eo_packet_dis(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select polarity of the modulation prior to the pulse shaping\n0: Normal operation\n1: Invert the modulation signal"]
    #[inline(always)]
    pub fn txdata_inv(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select the output resolution in the analog signal path\n0: 8 bit resolution for the shaping signal\n1: 6 bit resolution for the shaping signal"]
    #[inline(always)]
    pub fn gauss_86(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select polarity of the analog modulation path\n0: Normal operation\n1: Invert the signal in the analog signal path"]
    #[inline(always)]
    pub fn gauss_inv(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Additional delay in analog signal path in RCLK cycles"]
    #[inline(always)]
    pub fn delay(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfSynthCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Modulation Index selection\n0:h = 1/2 (Î\u{94}f = 250 kHz)\n1: h = 1/4 (Î\u{94}f = 125 kHz)\n2: h = 17/32 (Î\u{94}f = 266 kHz)\n3: h = 35/64 (Î\u{94}f = 273 kHz)"]
    #[inline(always)]
    pub fn modindex(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RfSynthCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Order of the sigma-delta modulator in TX mode"]
    #[inline(always)]
    pub fn sd_order_tx(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RfSynthCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Order of the sigma-delta modulator in RX mode"]
    #[inline(always)]
    pub fn sd_order_rx(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, RfSynthCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfSynthCtrl2Reg {
    #[inline(always)]
    fn default() -> RfSynthCtrl2Reg {
        <crate::RegValueT<RfSynthCtrl2Reg_SPEC> as RegisterValue<_>>::new(1035)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSynthCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for RfSynthCtrl3Reg_SPEC {
    type DataType = u16;
}

pub type RfSynthCtrl3Reg = crate::RegValueT<RfSynthCtrl3Reg_SPEC>;

impl RfSynthCtrl3Reg {
    #[doc = "0: Normal operation\n1: Use the externally provided value for the modulation"]
    #[inline(always)]
    pub fn modval_sel(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfSynthCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfSynthCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Externally provided modulation value in 2s complement\nÎ\u{94}f = 16 MHz x MODVAL_WR/16348"]
    #[inline(always)]
    pub fn modval_wr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        RfSynthCtrl3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            RfSynthCtrl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfSynthCtrl3Reg {
    #[inline(always)]
    fn default() -> RfSynthCtrl3Reg {
        <crate::RegValueT<RfSynthCtrl3Reg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSynthResult2Reg_SPEC;
impl crate::sealed::RegSpec for RfSynthResult2Reg_SPEC {
    type DataType = u16;
}

#[doc = "Must be Retained"]
pub type RfSynthResult2Reg = crate::RegValueT<RfSynthResult2Reg_SPEC>;

impl RfSynthResult2Reg {
    #[doc = "Result of the modulation gain calibration (Retained)"]
    #[inline(always)]
    pub fn cn_cal_rd(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, RfSynthResult2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            8,
            0x7f,
            1,
            0,
            u8,
            u8,
            RfSynthResult2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Modulation gain after trimming (Not Retained)"]
    #[inline(always)]
    pub fn gauss_gain_rd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfSynthResult2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfSynthResult2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfSynthResult2Reg {
    #[inline(always)]
    fn default() -> RfSynthResult2Reg {
        <crate::RegValueT<RfSynthResult2Reg_SPEC> as RegisterValue<_>>::new(5248)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSynthResult3Reg_SPEC;
impl crate::sealed::RegSpec for RfSynthResult3Reg_SPEC {
    type DataType = u16;
}

pub type RfSynthResult3Reg = crate::RegValueT<RfSynthResult3Reg_SPEC>;

impl RfSynthResult3Reg {
    #[doc = "Content of the calibration counter"]
    #[inline(always)]
    pub fn mdstate_rd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        RfSynthResult3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            RfSynthResult3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfSynthResult3Reg {
    #[inline(always)]
    fn default() -> RfSynthResult3Reg {
        <crate::RegValueT<RfSynthResult3Reg_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSynthResultReg_SPEC;
impl crate::sealed::RegSpec for RfSynthResultReg_SPEC {
    type DataType = u16;
}

#[doc = "Must be Retained"]
pub type RfSynthResultReg = crate::RegValueT<RfSynthResultReg_SPEC>;

impl RfSynthResultReg {
    #[doc = "Result of the VCO calibration (Not Retained)"]
    #[inline(always)]
    pub fn vco_freqtrim_rd(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfSynthResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfSynthResultReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Result of the modulation gain calibration (Retained)"]
    #[inline(always)]
    pub fn gauss_gain_cal_rd(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, RfSynthResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,RfSynthResultReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfSynthResultReg {
    #[inline(always)]
    fn default() -> RfSynthResultReg {
        <crate::RegValueT<RfSynthResultReg_SPEC> as RegisterValue<_>>::new(1152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfTdcCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfTdcCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "TDC settings"]
pub type RfTdcCtrlReg = crate::RegValueT<RfTdcCtrlReg_SPEC>;

impl RfTdcCtrlReg {
    #[doc = "\'0\': Normal Operation (no measurement possible)\n\'1\': Connect the PFD inputs also to the TDC inputs"]
    #[inline(always)]
    pub fn tdc_connect(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfTdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select how calibration is performed.\nPhase 1:\n\'0\': Count during 1 RCLK period (expect 60-70 as result)\n\'1\': Count during 2 RCLK period (expect 120-140 as result)\n\'2\': Count during 3 RCLK period (expect 180 -210 as result)\n\'3\': not allowed\nPhase 2, base the resolution measurement on:\n\'0\': 1 overlap of fast and slow oscillators (NF=NS+1)\n\'1\': 2 overlaps of fast and slow oscillators (NF=NS+2)\n\'2\': 1 overlap of fast and slow oscillators (NF=NS+1)\n\'3\': Not allowed"]
    #[inline(always)]
    pub fn ref_ctrl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, RfTdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select calibration option 2\n\'0\': normal operation (pfd measurement mode or cal mode 1)\n\'1\': measure the slow - fast oscillator period (calibration phase 2)\n------- Note CAL_PH_1 must be 0 in this setting"]
    #[inline(always)]
    pub fn cal_ph_2(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfTdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select calibration option 1\n\'0\': normal operation (pfd measurement or cal mode 2)\n\'1\': measure the fast oscillator period (calibration phase 1)\n------- Note: CAL_PH_2 must be 0 in this setting"]
    #[inline(always)]
    pub fn cal_ph_1(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfTdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim the slow oscillator\n\'0\': Minimum frequency (default)\n\'F\': Maximum frequency"]
    #[inline(always)]
    pub fn ctrl_slow(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfTdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Trim the fast oscillator\n\'0\': mimimum frequency\n\'F\': maximum frequency"]
    #[inline(always)]
    pub fn ctrl_fast(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfTdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfTdcCtrlReg {
    #[inline(always)]
    fn default() -> RfTdcCtrlReg {
        <crate::RegValueT<RfTdcCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfVcocalCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfVcocalCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfVcocalCtrlReg = crate::RegValueT<RfVcocalCtrlReg_SPEC>;

impl RfVcocalCtrlReg {
    #[doc = "Length of a VCO calibration step\n0: 1 us\n1: 2 us\n2: 3 us\n3: 4 us"]
    #[inline(always)]
    pub fn vcocal_period(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, RfVcocalCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,RfVcocalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Normal operation\n1: Use VCO_FREQTRIM_WR for the VCO calibration"]
    #[inline(always)]
    pub fn vco_freqtrim_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfVcocalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfVcocalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Externally provided VCO calibration value"]
    #[inline(always)]
    pub fn vco_freqtrim_wr(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RfVcocalCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RfVcocalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfVcocalCtrlReg {
    #[inline(always)]
    fn default() -> RfVcocalCtrlReg {
        <crate::RegValueT<RfVcocalCtrlReg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfVcovarCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfVcovarCtrlReg_SPEC {
    type DataType = u16;
}

pub type RfVcovarCtrlReg = crate::RegValueT<RfVcovarCtrlReg_SPEC>;

impl RfVcovarCtrlReg {
    #[doc = "Bias voltage of the VCO Modulation varactor (high Vmod)\n0: low\n1: mid\n2: nominal\n3: high"]
    #[inline(always)]
    pub fn mod_var_v1(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias voltage of the VCO Modulation varactor (low Vmod)\n0: low\n1: mid\n2: nominal\n3: high"]
    #[inline(always)]
    pub fn mod_var_v0(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias voltage of the VCO Tuning varactor (high Vtune)\n001: low\n010: nominal\n100: high\nothers: not allowed"]
    #[inline(always)]
    pub fn tune_var_v3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias voltage of the VCO Tuning varactor (high-mid Vtune)\nCoding identical to TUNE_VAR_V3"]
    #[inline(always)]
    pub fn tune_var_v2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias voltage of the VCO Tuning varactor (low-mid Vtune)\nCoding identical to TUNE_VAR_V3"]
    #[inline(always)]
    pub fn tune_var_v1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Bias voltage of the VCO Tuning varactor (low Vtune)\nCoding identical to TUNE_VAR_V3"]
    #[inline(always)]
    pub fn tune_var_v0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfVcovarCtrlReg {
    #[inline(always)]
    fn default() -> RfVcovarCtrlReg {
        <crate::RegValueT<RfVcovarCtrlReg_SPEC> as RegisterValue<_>>::new(42130)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfVcoCalcapBit14Reg_SPEC;
impl crate::sealed::RegSpec for RfVcoCalcapBit14Reg_SPEC {
    type DataType = u16;
}

#[doc = "LUT entry for bit 14 of the VCO calibration capacitance"]
pub type RfVcoCalcapBit14Reg = crate::RegValueT<RfVcoCalcapBit14Reg_SPEC>;

impl RfVcoCalcapBit14Reg {
    #[doc = "LUT entry for bit 14 of the VCO calibration capacitance"]
    #[inline(always)]
    pub fn vco_calcap_bit14(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        RfVcoCalcapBit14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            RfVcoCalcapBit14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfVcoCalcapBit14Reg {
    #[inline(always)]
    fn default() -> RfVcoCalcapBit14Reg {
        <crate::RegValueT<RfVcoCalcapBit14Reg_SPEC> as RegisterValue<_>>::new(21917)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfVcoCalcapBit15Reg_SPEC;
impl crate::sealed::RegSpec for RfVcoCalcapBit15Reg_SPEC {
    type DataType = u16;
}

#[doc = "LUT entry for bit 15 of the VCO calibration capacitance"]
pub type RfVcoCalcapBit15Reg = crate::RegValueT<RfVcoCalcapBit15Reg_SPEC>;

impl RfVcoCalcapBit15Reg {
    #[doc = "LUT entry for bit 15 of the VCO calibration capacitance"]
    #[inline(always)]
    pub fn vco_calcap_bit15(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        RfVcoCalcapBit15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            RfVcoCalcapBit15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfVcoCalcapBit15Reg {
    #[inline(always)]
    fn default() -> RfVcoCalcapBit15Reg {
        <crate::RegValueT<RfVcoCalcapBit15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
