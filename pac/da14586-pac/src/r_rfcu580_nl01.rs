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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:45 +0000

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
    #[inline(always)]
    pub fn iff_bias_set(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, BiasCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,BiasCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vco_bias_set(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, BiasCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,BiasCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cp_bias_set(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, BiasCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,BiasCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RfAdciDcOffsetReg = crate::RegValueT<RfAdciDcOffsetReg_SPEC>;

impl RfAdciDcOffsetReg {
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

pub type RfAdcqDcOffsetReg = crate::RegValueT<RfAdcqDcOffsetReg_SPEC>;

impl RfAdcqDcOffsetReg {
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
    #[inline(always)]
    pub fn adc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,RfAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn adc_offn_i_wr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfAdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,RfAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn adc_offn_q_wr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, RfAdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,RfAdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn pole2(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAfcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAfcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pole1(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RfAfcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RfAfcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn agc_mode(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn agc_th_high(
        self,
    ) -> crate::common::RegisterField<7, 0x7f, 1, 0, u8, u8, RfAgcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x7f,1,0,u8,u8,RfAgcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn slow_agc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfAgcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn agcsetting_wr(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfAgcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn agcsetting_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfAgcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn en_frz_gain(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfAgcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfAgcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn lna_gain1(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain1(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga2_gain1(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lna_gain0(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain0(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn vga1_gain3(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut23Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut23Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga2_gain3(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut23Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut23Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain2(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut23Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut23Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn lna_gain5(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain5(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga2_gain5(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lna_gain4(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain4(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut45Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut45Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn lna_gain7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain7(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga2_gain7(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lna_gain6(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain6(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut67Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut67Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn lna_gain9(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain9(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga2_gain9(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lna_gain8(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vga1_gain8(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfAgcLut89Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfAgcLut89Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn agcsetting_rd(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfAgcResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfAgcResultReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type RfBmcwReg = crate::RegValueT<RfBmcwReg_SPEC>;

impl RfBmcwReg {
    #[inline(always)]
    pub fn cn_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfBmcwReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfBmcwReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn vco_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dc_offset_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iff_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mgain_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eo_cal(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfCalCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,RfCalCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn cp_cur_tx(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, RfCpCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,RfCpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cp_cur_rx(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfCpCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfCpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cp_cur_set_tx(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfCpCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfCpCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

    #[inline(always)]
    pub fn dcparcal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfDcOffsetCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfDcOffsetCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RfDcOffsetResultReg = crate::RegValueT<RfDcOffsetResultReg_SPEC>;

impl RfDcOffsetResultReg {
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
    #[inline(always)]
    pub fn equal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfDemCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfDemCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn match0101_th(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, u8, RfDemCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,u8,u8,RfDemCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dem_hsi_pol(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfDemCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfDemCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn iff_dcoc_dac_dis(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ro_to_pins(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn if_mute(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn if_cal_cap_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfIffCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfIffCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RfIffResultReg = crate::RegValueT<RfIffResultReg_SPEC>;

impl RfIffResultReg {
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
    #[inline(always)]
    pub fn lf_short_r4(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfLfCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfLfCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lf_cal_cap_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfLfCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfLfCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RfLfResCtrlReg = crate::RegValueT<RfLfResCtrlReg_SPEC>;

impl RfLfResCtrlReg {
    #[inline(always)]
    pub fn lf_res_tx(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, RfLfResCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,RfLfResCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lf_res_rx(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfLfResCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfLfResCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lf_res_set_tx(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfLfResCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfLfResCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn kmod_alpha(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, RfMgainCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mgain_aver(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, RfMgainCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mgain_cmp_inv(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RfMgainCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mgain_dbl_transmit(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfMgainCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfMgainCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn gauss_dac_ctrl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RfMgcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RfMgcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mgc_pole_sw(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfMgcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfMgcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn mix_spare(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mix_trim_vcm(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mix_trim_ibias(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn mix_cal_cap_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfMixerCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfMixerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn rx_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_dis_wr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RfPaCtrlReg = crate::RegValueT<RfPaCtrlReg_SPEC>;

impl RfPaCtrlReg {
    #[inline(always)]
    pub fn level_ldo_rfpa(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, u8, RfPaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0xf,1,0,u8,u8,RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pa_rampspeed(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, RfPaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pa_pw(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, RfPaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn pfd_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfPfdCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfPfdCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fixed_cur_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfPfdCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfPfdCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn cnt_clk(
        self,
    ) -> crate::common::RegisterField<6, 0x1ff, 1, 0, u16, u16, RfRefOscReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1ff,1,0,u16,u16,RfRefOscReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn cp_cur(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfScanFeedbackReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfScanFeedbackReg_SPEC,crate::common::R>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn rf_spare1(
        self,
    ) -> crate::common::RegisterField<1, 0x7fff, 1, 0, u16, u16, RfSpare1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fff,1,0,u16,u16,RfSpare1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn pll_hsi_pol(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfSynthCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfSynthCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cs(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RfSynthCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,RfSynthCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sgn(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfSynthCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfSynthCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn bt_sel(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eo_packet_dis(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txdata_inv(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gauss_86(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gauss_inv(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfSynthCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn delay(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RfSynthCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn modindex(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RfSynthCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sd_order_tx(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RfSynthCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RfSynthCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn modval_sel(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfSynthCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfSynthCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RfSynthResult2Reg = crate::RegValueT<RfSynthResult2Reg_SPEC>;

impl RfSynthResult2Reg {
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

pub type RfSynthResultReg = crate::RegValueT<RfSynthResultReg_SPEC>;

impl RfSynthResultReg {
    #[inline(always)]
    pub fn vco_freqtrim_rd(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RfSynthResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RfSynthResultReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type RfTdcCtrlReg = crate::RegValueT<RfTdcCtrlReg_SPEC>;

impl RfTdcCtrlReg {
    #[inline(always)]
    pub fn tdc_connect(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfTdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ref_ctrl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, RfTdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cal_ph_2(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfTdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cal_ph_1(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfTdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctrl_slow(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RfTdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RfTdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn vcocal_period(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, RfVcocalCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,RfVcocalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vco_freqtrim_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfVcocalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfVcocalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
    #[inline(always)]
    pub fn mod_var_v1(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mod_var_v0(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tune_var_v3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tune_var_v2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tune_var_v1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, RfVcovarCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,RfVcovarCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type RfVcoCalcapBit14Reg = crate::RegValueT<RfVcoCalcapBit14Reg_SPEC>;

impl RfVcoCalcapBit14Reg {
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

pub type RfVcoCalcapBit15Reg = crate::RegValueT<RfVcoCalcapBit15Reg_SPEC>;

impl RfVcoCalcapBit15Reg {
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
