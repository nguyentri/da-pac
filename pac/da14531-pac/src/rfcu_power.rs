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
#[doc = r"RFCU_POWER registers"]
unsafe impl ::core::marker::Send for super::RfcuPower {}
unsafe impl ::core::marker::Sync for super::RfcuPower {}
impl super::RfcuPower {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn rf_always_en1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAlwaysEn1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_always_en2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAlwaysEn2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_16_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer16Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer16Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(316usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_17_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer17Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer17Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_18_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer18Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer18Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(324usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_19_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer19Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer19Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(328usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_20_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer20Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer20Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(332usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_21_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer21Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer21Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(336usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_22_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer22Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer22Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(340usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_23_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer23Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer23Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(344usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_24_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer24Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer24Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(348usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_25_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer25Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer25Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(352usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_26_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer26Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer26Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(356usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_27_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer27Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer27Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(360usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_28_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer28Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer28Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(364usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_29_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer29Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer29Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(368usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_30_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer30Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer30Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(372usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_31_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer31Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer31Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(376usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_cntrl_timer_9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCntrlTimer9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCntrlTimer9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config16_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig16Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig16Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config17_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig17Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig17Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config18_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig18Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig18Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config19_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig19Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig19Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config20_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig20Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig20Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config21_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig21Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig21Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config22_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig22Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig22Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config23_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig23Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig23Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config24_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig24Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig24Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config25_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig25Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig25Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config26_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig26Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig26Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config27_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig27Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig27Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config28_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig28Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig28Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config29_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig29Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig29Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config30_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig30Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig30Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config31_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig31Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig31Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config32_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig32Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig32Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config33_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig33Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig33Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config34_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig34Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig34Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config35_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig35Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig35Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config36_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig36Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig36Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config37_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig37Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig37Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config38_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig38Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig38Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config39_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig39Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig39Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config40_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig40Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig40Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config41_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig41Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig41Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config42_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig42Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig42Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config43_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig43Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig43Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config44_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig44Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig44Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config45_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig45Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig45Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config46_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig46Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig46Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_enable_config9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfEnableConfig9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfEnableConfig9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_port_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfPortEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfPortEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rf_port_pol_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfPortPolReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfPortPolReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(396usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAlwaysEn1Reg_SPEC;
impl crate::sealed::RegSpec for RfAlwaysEn1Reg_SPEC {
    type DataType = u32;
}

pub type RfAlwaysEn1Reg = crate::RegValueT<RfAlwaysEn1Reg_SPEC>;

impl RfAlwaysEn1Reg {
    #[inline(always)]
    pub fn alw_en_adplldig_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adplldig_rst(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_dco_en(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adc_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_iff_bias_sh_open(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_iff_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_mix_bias_sh_open(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_mix_en(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_lna_cgm_en(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_lna_core_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_pa_en(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_pa_ramp_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_rfio_bias_sh_open(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_rfio_bias_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_rfio_tx_harm_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_rfio_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_rfio_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adplldig_ldo_lp(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adplldig_ldo_activerdy(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_lna_ldo_zero(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_ldo_zero_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_dco_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_dtc_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_tdc_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_iffadc_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_iff_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_mix_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_lna_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_pa_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_rfio_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfAlwaysEn1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfAlwaysEn1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAlwaysEn1Reg {
    #[inline(always)]
    fn default() -> RfAlwaysEn1Reg {
        <crate::RegValueT<RfAlwaysEn1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAlwaysEn2Reg_SPEC;
impl crate::sealed::RegSpec for RfAlwaysEn2Reg_SPEC {
    type DataType = u32;
}

pub type RfAlwaysEn2Reg = crate::RegValueT<RfAlwaysEn2Reg_SPEC>;

impl RfAlwaysEn2Reg {
    #[inline(always)]
    pub fn alw_en_spare5(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_spare4(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_spare3(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_spare2(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_spare1(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_rdy_for_div(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_phy_rdy4bs(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_dem_sigdetect_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_dem_agc_unfreeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_dem_dc_parcal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_dem_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_cal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_lobuf_pa_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adpll_pain_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn alw_en_adplldig_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfAlwaysEn2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfAlwaysEn2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAlwaysEn2Reg {
    #[inline(always)]
    fn default() -> RfAlwaysEn2Reg {
        <crate::RegValueT<RfAlwaysEn2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer10Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer10Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer10Reg_SPEC> as RegisterValue<_>>::new(3094)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer11Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer11Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer11Reg_SPEC> as RegisterValue<_>>::new(2074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer12Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer12Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer12Reg_SPEC> as RegisterValue<_>>::new(3610)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer13Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer13Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer13Reg_SPEC> as RegisterValue<_>>::new(544)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer14Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer14Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer14Reg_SPEC> as RegisterValue<_>>::new(2592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer15Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer15Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer15Reg = crate::RegValueT<RfCntrlTimer15Reg_SPEC>;

impl RfCntrlTimer15Reg {
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
        RfCntrlTimer15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer15Reg_SPEC,
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
        RfCntrlTimer15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer15Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer15Reg {
        <crate::RegValueT<RfCntrlTimer15Reg_SPEC> as RegisterValue<_>>::new(1062)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer16Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer16Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer16Reg = crate::RegValueT<RfCntrlTimer16Reg_SPEC>;

impl RfCntrlTimer16Reg {
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
        RfCntrlTimer16Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer16Reg_SPEC,
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
        RfCntrlTimer16Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer16Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer16Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer16Reg {
        <crate::RegValueT<RfCntrlTimer16Reg_SPEC> as RegisterValue<_>>::new(1072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer17Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer17Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer17Reg = crate::RegValueT<RfCntrlTimer17Reg_SPEC>;

impl RfCntrlTimer17Reg {
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
        RfCntrlTimer17Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer17Reg_SPEC,
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
        RfCntrlTimer17Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer17Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer17Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer17Reg {
        <crate::RegValueT<RfCntrlTimer17Reg_SPEC> as RegisterValue<_>>::new(2103)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer18Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer18Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer18Reg = crate::RegValueT<RfCntrlTimer18Reg_SPEC>;

impl RfCntrlTimer18Reg {
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
        RfCntrlTimer18Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer18Reg_SPEC,
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
        RfCntrlTimer18Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer18Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer18Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer18Reg {
        <crate::RegValueT<RfCntrlTimer18Reg_SPEC> as RegisterValue<_>>::new(2118)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer19Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer19Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer19Reg = crate::RegValueT<RfCntrlTimer19Reg_SPEC>;

impl RfCntrlTimer19Reg {
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
        RfCntrlTimer19Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer19Reg_SPEC,
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
        RfCntrlTimer19Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer19Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer19Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer19Reg {
        <crate::RegValueT<RfCntrlTimer19Reg_SPEC> as RegisterValue<_>>::new(2128)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer1Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer1Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer1Reg_SPEC> as RegisterValue<_>>::new(1024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer20Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer20Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer20Reg = crate::RegValueT<RfCntrlTimer20Reg_SPEC>;

impl RfCntrlTimer20Reg {
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
        RfCntrlTimer20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer20Reg_SPEC,
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
        RfCntrlTimer20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer20Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer20Reg {
        <crate::RegValueT<RfCntrlTimer20Reg_SPEC> as RegisterValue<_>>::new(4698)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer21Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer21Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer21Reg = crate::RegValueT<RfCntrlTimer21Reg_SPEC>;

impl RfCntrlTimer21Reg {
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
        RfCntrlTimer21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer21Reg_SPEC,
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
        RfCntrlTimer21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer21Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer21Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer21Reg {
        <crate::RegValueT<RfCntrlTimer21Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer22Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer22Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer22Reg = crate::RegValueT<RfCntrlTimer22Reg_SPEC>;

impl RfCntrlTimer22Reg {
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
        RfCntrlTimer22Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer22Reg_SPEC,
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
        RfCntrlTimer22Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer22Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer22Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer22Reg {
        <crate::RegValueT<RfCntrlTimer22Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer23Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer23Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer23Reg = crate::RegValueT<RfCntrlTimer23Reg_SPEC>;

impl RfCntrlTimer23Reg {
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
        RfCntrlTimer23Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer23Reg_SPEC,
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
        RfCntrlTimer23Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer23Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer23Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer23Reg {
        <crate::RegValueT<RfCntrlTimer23Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer24Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer24Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer24Reg = crate::RegValueT<RfCntrlTimer24Reg_SPEC>;

impl RfCntrlTimer24Reg {
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
        RfCntrlTimer24Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer24Reg_SPEC,
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
        RfCntrlTimer24Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer24Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer24Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer24Reg {
        <crate::RegValueT<RfCntrlTimer24Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer25Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer25Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer25Reg = crate::RegValueT<RfCntrlTimer25Reg_SPEC>;

impl RfCntrlTimer25Reg {
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
        RfCntrlTimer25Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer25Reg_SPEC,
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
        RfCntrlTimer25Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer25Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer25Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer25Reg {
        <crate::RegValueT<RfCntrlTimer25Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer26Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer26Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer26Reg = crate::RegValueT<RfCntrlTimer26Reg_SPEC>;

impl RfCntrlTimer26Reg {
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
        RfCntrlTimer26Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer26Reg_SPEC,
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
        RfCntrlTimer26Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer26Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer26Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer26Reg {
        <crate::RegValueT<RfCntrlTimer26Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer27Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer27Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer27Reg = crate::RegValueT<RfCntrlTimer27Reg_SPEC>;

impl RfCntrlTimer27Reg {
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
        RfCntrlTimer27Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer27Reg_SPEC,
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
        RfCntrlTimer27Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer27Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer27Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer27Reg {
        <crate::RegValueT<RfCntrlTimer27Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer28Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer28Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer28Reg = crate::RegValueT<RfCntrlTimer28Reg_SPEC>;

impl RfCntrlTimer28Reg {
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
        RfCntrlTimer28Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer28Reg_SPEC,
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
        RfCntrlTimer28Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer28Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer28Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer28Reg {
        <crate::RegValueT<RfCntrlTimer28Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer29Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer29Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer29Reg = crate::RegValueT<RfCntrlTimer29Reg_SPEC>;

impl RfCntrlTimer29Reg {
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
        RfCntrlTimer29Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer29Reg_SPEC,
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
        RfCntrlTimer29Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer29Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer29Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer29Reg {
        <crate::RegValueT<RfCntrlTimer29Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer2Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer2Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer2Reg_SPEC> as RegisterValue<_>>::new(3072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer30Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer30Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer30Reg = crate::RegValueT<RfCntrlTimer30Reg_SPEC>;

impl RfCntrlTimer30Reg {
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
        RfCntrlTimer30Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer30Reg_SPEC,
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
        RfCntrlTimer30Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer30Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer30Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer30Reg {
        <crate::RegValueT<RfCntrlTimer30Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer31Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer31Reg_SPEC {
    type DataType = u32;
}

pub type RfCntrlTimer31Reg = crate::RegValueT<RfCntrlTimer31Reg_SPEC>;

impl RfCntrlTimer31Reg {
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
        RfCntrlTimer31Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer31Reg_SPEC,
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
        RfCntrlTimer31Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            RfCntrlTimer31Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfCntrlTimer31Reg {
    #[inline(always)]
    fn default() -> RfCntrlTimer31Reg {
        <crate::RegValueT<RfCntrlTimer31Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer3Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer3Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer3Reg_SPEC> as RegisterValue<_>>::new(1032)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer4Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer4Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer4Reg_SPEC> as RegisterValue<_>>::new(3080)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer5Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer5Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer5Reg_SPEC> as RegisterValue<_>>::new(530)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer6Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer6Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer6Reg_SPEC> as RegisterValue<_>>::new(2578)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer7Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer7Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer7Reg_SPEC> as RegisterValue<_>>::new(1044)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer8Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer8Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer8Reg_SPEC> as RegisterValue<_>>::new(3092)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCntrlTimer9Reg_SPEC;
impl crate::sealed::RegSpec for RfCntrlTimer9Reg_SPEC {
    type DataType = u32;
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
        <crate::RegValueT<RfCntrlTimer9Reg_SPEC> as RegisterValue<_>>::new(1046)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig0Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig0Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig0Reg = crate::RegValueT<RfEnableConfig0Reg_SPEC>;

impl RfEnableConfig0Reg {
    #[inline(always)]
    pub fn rfio_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfio_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig0Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig0Reg {
        <crate::RegValueT<RfEnableConfig0Reg_SPEC> as RegisterValue<_>>::new(131)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig10Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig10Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig10Reg = crate::RegValueT<RfEnableConfig10Reg_SPEC>;

impl RfEnableConfig10Reg {
    #[inline(always)]
    pub fn lna_ldo_zero_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lna_ldo_zero_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig10Reg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig11Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig11Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig11Reg = crate::RegValueT<RfEnableConfig11Reg_SPEC>;

impl RfEnableConfig11Reg {
    #[inline(always)]
    pub fn adplldig_ldo_activerdy_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adplldig_ldo_activerdy_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig11Reg_SPEC> as RegisterValue<_>>::new(197)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig12Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig12Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig12Reg = crate::RegValueT<RfEnableConfig12Reg_SPEC>;

impl RfEnableConfig12Reg {
    #[inline(always)]
    pub fn adplldig_ldo_lp_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adplldig_ldo_lp_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig12Reg_SPEC> as RegisterValue<_>>::new(395)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig13Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig13Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig13Reg = crate::RegValueT<RfEnableConfig13Reg_SPEC>;

impl RfEnableConfig13Reg {
    #[inline(always)]
    pub fn rfio_rx_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfio_rx_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig13Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig14Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig14Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig14Reg = crate::RegValueT<RfEnableConfig14Reg_SPEC>;

impl RfEnableConfig14Reg {
    #[inline(always)]
    pub fn rfio_tx_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfio_tx_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig14Reg_SPEC> as RegisterValue<_>>::new(448)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig15Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig15Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig15Reg = crate::RegValueT<RfEnableConfig15Reg_SPEC>;

impl RfEnableConfig15Reg {
    #[inline(always)]
    pub fn rfio_tx_harm_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfio_tx_harm_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig15Reg_SPEC> as RegisterValue<_>>::new(448)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig16Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig16Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig16Reg = crate::RegValueT<RfEnableConfig16Reg_SPEC>;

impl RfEnableConfig16Reg {
    #[inline(always)]
    pub fn rfio_bias_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig16Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig16Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfio_bias_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig16Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig16Reg_SPEC> as RegisterValue<_>>::new(143)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig17Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig17Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig17Reg = crate::RegValueT<RfEnableConfig17Reg_SPEC>;

impl RfEnableConfig17Reg {
    #[inline(always)]
    pub fn rfio_bias_sh_open_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig17Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig17Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfio_bias_sh_open_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig17Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig17Reg_SPEC> as RegisterValue<_>>::new(528)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig18Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig18Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig18Reg = crate::RegValueT<RfEnableConfig18Reg_SPEC>;

impl RfEnableConfig18Reg {
    #[inline(always)]
    pub fn pa_ramp_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig18Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig18Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pa_ramp_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig18Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig18Reg_SPEC> as RegisterValue<_>>::new(416)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig19Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig19Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig19Reg = crate::RegValueT<RfEnableConfig19Reg_SPEC>;

impl RfEnableConfig19Reg {
    #[inline(always)]
    pub fn pa_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig19Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig19Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pa_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig19Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig19Reg_SPEC> as RegisterValue<_>>::new(448)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig1Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig1Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig1Reg = crate::RegValueT<RfEnableConfig1Reg_SPEC>;

impl RfEnableConfig1Reg {
    #[inline(always)]
    pub fn pa_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pa_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig1Reg_SPEC> as RegisterValue<_>>::new(131)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig20Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig20Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig20Reg = crate::RegValueT<RfEnableConfig20Reg_SPEC>;

impl RfEnableConfig20Reg {
    #[inline(always)]
    pub fn lna_core_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lna_core_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig20Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig21Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig21Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig21Reg = crate::RegValueT<RfEnableConfig21Reg_SPEC>;

impl RfEnableConfig21Reg {
    #[inline(always)]
    pub fn lna_cgm_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig21Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lna_cgm_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig21Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig22Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig22Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig22Reg = crate::RegValueT<RfEnableConfig22Reg_SPEC>;

impl RfEnableConfig22Reg {
    #[inline(always)]
    pub fn mix_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig22Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig22Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mix_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig22Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig22Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig23Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig23Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig23Reg = crate::RegValueT<RfEnableConfig23Reg_SPEC>;

impl RfEnableConfig23Reg {
    #[inline(always)]
    pub fn mix_bias_sh_open_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig23Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig23Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mix_bias_sh_open_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig23Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig23Reg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig24Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig24Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig24Reg = crate::RegValueT<RfEnableConfig24Reg_SPEC>;

impl RfEnableConfig24Reg {
    #[inline(always)]
    pub fn iff_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig24Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig24Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iff_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig24Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig24Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig24Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig24Reg {
        <crate::RegValueT<RfEnableConfig24Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig25Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig25Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig25Reg = crate::RegValueT<RfEnableConfig25Reg_SPEC>;

impl RfEnableConfig25Reg {
    #[inline(always)]
    pub fn iff_bias_sh_open_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig25Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig25Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iff_bias_sh_open_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig25Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig25Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig25Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig25Reg {
        <crate::RegValueT<RfEnableConfig25Reg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig26Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig26Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig26Reg = crate::RegValueT<RfEnableConfig26Reg_SPEC>;

impl RfEnableConfig26Reg {
    #[inline(always)]
    pub fn adc_clk_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig26Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig26Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adc_clk_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig26Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig26Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig26Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig26Reg {
        <crate::RegValueT<RfEnableConfig26Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig27Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig27Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig27Reg = crate::RegValueT<RfEnableConfig27Reg_SPEC>;

impl RfEnableConfig27Reg {
    #[inline(always)]
    pub fn adc_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig27Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig27Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adc_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig27Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig27Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig27Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig27Reg {
        <crate::RegValueT<RfEnableConfig27Reg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig28Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig28Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig28Reg = crate::RegValueT<RfEnableConfig28Reg_SPEC>;

impl RfEnableConfig28Reg {
    #[inline(always)]
    pub fn adpll_dco_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig28Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig28Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adpll_dco_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig28Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig28Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig28Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig28Reg {
        <crate::RegValueT<RfEnableConfig28Reg_SPEC> as RegisterValue<_>>::new(65)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig29Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig29Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig29Reg = crate::RegValueT<RfEnableConfig29Reg_SPEC>;

impl RfEnableConfig29Reg {
    #[inline(always)]
    pub fn adpll_clk_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig29Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig29Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adpll_clk_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig29Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig29Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig29Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig29Reg {
        <crate::RegValueT<RfEnableConfig29Reg_SPEC> as RegisterValue<_>>::new(65)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig2Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig2Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig2Reg = crate::RegValueT<RfEnableConfig2Reg_SPEC>;

impl RfEnableConfig2Reg {
    #[inline(always)]
    pub fn lna_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lna_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig2Reg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig30Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig30Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig30Reg = crate::RegValueT<RfEnableConfig30Reg_SPEC>;

impl RfEnableConfig30Reg {
    #[inline(always)]
    pub fn adplldig_rst_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig30Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig30Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adplldig_rst_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig30Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig30Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig30Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig30Reg {
        <crate::RegValueT<RfEnableConfig30Reg_SPEC> as RegisterValue<_>>::new(263)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig31Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig31Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig31Reg = crate::RegValueT<RfEnableConfig31Reg_SPEC>;

impl RfEnableConfig31Reg {
    #[inline(always)]
    pub fn adplldig_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig31Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig31Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adplldig_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig31Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig31Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig31Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig31Reg {
        <crate::RegValueT<RfEnableConfig31Reg_SPEC> as RegisterValue<_>>::new(329)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig32Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig32Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig32Reg = crate::RegValueT<RfEnableConfig32Reg_SPEC>;

impl RfEnableConfig32Reg {
    #[inline(always)]
    pub fn adplldig_rx_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig32Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig32Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adplldig_rx_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig32Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig32Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig32Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig32Reg {
        <crate::RegValueT<RfEnableConfig32Reg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig33Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig33Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig33Reg = crate::RegValueT<RfEnableConfig33Reg_SPEC>;

impl RfEnableConfig33Reg {
    #[inline(always)]
    pub fn adplldig_pain_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig33Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig33Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adplldig_pain_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig33Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig33Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig33Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig33Reg {
        <crate::RegValueT<RfEnableConfig33Reg_SPEC> as RegisterValue<_>>::new(480)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig34Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig34Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig34Reg = crate::RegValueT<RfEnableConfig34Reg_SPEC>;

impl RfEnableConfig34Reg {
    #[inline(always)]
    pub fn adpll_lobuf_pa_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig34Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig34Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adpll_lobuf_pa_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig34Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig34Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig34Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig34Reg {
        <crate::RegValueT<RfEnableConfig34Reg_SPEC> as RegisterValue<_>>::new(448)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig35Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig35Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig35Reg = crate::RegValueT<RfEnableConfig35Reg_SPEC>;

impl RfEnableConfig35Reg {
    #[inline(always)]
    pub fn cal_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig35Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig35Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cal_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig35Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig35Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig35Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig35Reg {
        <crate::RegValueT<RfEnableConfig35Reg_SPEC> as RegisterValue<_>>::new(660)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig36Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig36Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig36Reg = crate::RegValueT<RfEnableConfig36Reg_SPEC>;

impl RfEnableConfig36Reg {
    #[inline(always)]
    pub fn dem_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig36Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig36Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dem_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig36Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig36Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig36Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig36Reg {
        <crate::RegValueT<RfEnableConfig36Reg_SPEC> as RegisterValue<_>>::new(17)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig37Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig37Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig37Reg = crate::RegValueT<RfEnableConfig37Reg_SPEC>;

impl RfEnableConfig37Reg {
    #[inline(always)]
    pub fn spare_dem_dc_parcal_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig37Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig37Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dem_dc_parcal_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig37Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig37Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig37Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig37Reg {
        <crate::RegValueT<RfEnableConfig37Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig38Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig38Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig38Reg = crate::RegValueT<RfEnableConfig38Reg_SPEC>;

impl RfEnableConfig38Reg {
    #[inline(always)]
    pub fn spare_dem_agc_unfreeze_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig38Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig38Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dem_agc_unfreeze_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig38Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig38Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig38Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig38Reg {
        <crate::RegValueT<RfEnableConfig38Reg_SPEC> as RegisterValue<_>>::new(17)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig39Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig39Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig39Reg = crate::RegValueT<RfEnableConfig39Reg_SPEC>;

impl RfEnableConfig39Reg {
    #[inline(always)]
    pub fn spare_dem_sigdetect_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig39Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig39Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dem_sigdetect_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig39Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig39Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig39Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig39Reg {
        <crate::RegValueT<RfEnableConfig39Reg_SPEC> as RegisterValue<_>>::new(19)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig3Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig3Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig3Reg = crate::RegValueT<RfEnableConfig3Reg_SPEC>;

impl RfEnableConfig3Reg {
    #[inline(always)]
    pub fn mix_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mix_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig3Reg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig40Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig40Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig40Reg = crate::RegValueT<RfEnableConfig40Reg_SPEC>;

impl RfEnableConfig40Reg {
    #[inline(always)]
    pub fn phy_rdy4bs_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig40Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig40Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phy_rdy4bs_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig40Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig40Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig40Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig40Reg {
        <crate::RegValueT<RfEnableConfig40Reg_SPEC> as RegisterValue<_>>::new(595)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig41Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig41Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig41Reg = crate::RegValueT<RfEnableConfig41Reg_SPEC>;

impl RfEnableConfig41Reg {
    #[inline(always)]
    pub fn adpll_rdy_for_div_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig41Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig41Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adpll_rdy_for_div_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig41Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig41Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig41Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig41Reg {
        <crate::RegValueT<RfEnableConfig41Reg_SPEC> as RegisterValue<_>>::new(197)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig42Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig42Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig42Reg = crate::RegValueT<RfEnableConfig42Reg_SPEC>;

impl RfEnableConfig42Reg {
    #[inline(always)]
    pub fn spare1_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig42Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig42Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spare1_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig42Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig42Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig42Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig42Reg {
        <crate::RegValueT<RfEnableConfig42Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig43Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig43Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig43Reg = crate::RegValueT<RfEnableConfig43Reg_SPEC>;

impl RfEnableConfig43Reg {
    #[inline(always)]
    pub fn spare2_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig43Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig43Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spare2_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig43Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig43Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig43Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig43Reg {
        <crate::RegValueT<RfEnableConfig43Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig44Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig44Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig44Reg = crate::RegValueT<RfEnableConfig44Reg_SPEC>;

impl RfEnableConfig44Reg {
    #[inline(always)]
    pub fn spare3_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig44Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig44Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spare3_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig44Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig44Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig44Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig44Reg {
        <crate::RegValueT<RfEnableConfig44Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig45Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig45Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig45Reg = crate::RegValueT<RfEnableConfig45Reg_SPEC>;

impl RfEnableConfig45Reg {
    #[inline(always)]
    pub fn spare4_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig45Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig45Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spare4_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig45Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig45Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig45Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig45Reg {
        <crate::RegValueT<RfEnableConfig45Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig46Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig46Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig46Reg = crate::RegValueT<RfEnableConfig46Reg_SPEC>;

impl RfEnableConfig46Reg {
    #[inline(always)]
    pub fn spare5_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig46Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig46Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spare5_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig46Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig46Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfEnableConfig46Reg {
    #[inline(always)]
    fn default() -> RfEnableConfig46Reg {
        <crate::RegValueT<RfEnableConfig46Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig4Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig4Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig4Reg = crate::RegValueT<RfEnableConfig4Reg_SPEC>;

impl RfEnableConfig4Reg {
    #[inline(always)]
    pub fn iff_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iff_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig4Reg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig5Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig5Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig5Reg = crate::RegValueT<RfEnableConfig5Reg_SPEC>;

impl RfEnableConfig5Reg {
    #[inline(always)]
    pub fn iffadc_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iffadc_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig5Reg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig6Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig6Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig6Reg = crate::RegValueT<RfEnableConfig6Reg_SPEC>;

impl RfEnableConfig6Reg {
    #[inline(always)]
    pub fn adpll_tdc_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adpll_tdc_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig6Reg_SPEC> as RegisterValue<_>>::new(131)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig7Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig7Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig7Reg = crate::RegValueT<RfEnableConfig7Reg_SPEC>;

impl RfEnableConfig7Reg {
    #[inline(always)]
    pub fn adpll_dtc_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adpll_dtc_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig7Reg_SPEC> as RegisterValue<_>>::new(131)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig8Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig8Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig8Reg = crate::RegValueT<RfEnableConfig8Reg_SPEC>;

impl RfEnableConfig8Reg {
    #[inline(always)]
    pub fn adpll_dco_ldo_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adpll_dco_ldo_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig8Reg_SPEC> as RegisterValue<_>>::new(131)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfEnableConfig9Reg_SPEC;
impl crate::sealed::RegSpec for RfEnableConfig9Reg_SPEC {
    type DataType = u32;
}

pub type RfEnableConfig9Reg = crate::RegValueT<RfEnableConfig9Reg_SPEC>;

impl RfEnableConfig9Reg {
    #[inline(always)]
    pub fn ldo_zero_en_dcf_tx(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1f,
            1,
            0,
            u8,
            u8,
            RfEnableConfig9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ldo_zero_en_dcf_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        RfEnableConfig9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
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
        <crate::RegValueT<RfEnableConfig9Reg_SPEC> as RegisterValue<_>>::new(65)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfPortEnReg_SPEC;
impl crate::sealed::RegSpec for RfPortEnReg_SPEC {
    type DataType = u32;
}

pub type RfPortEnReg = crate::RegValueT<RfPortEnReg_SPEC>;

impl RfPortEnReg {
    #[inline(always)]
    pub fn rf_port4_tx(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port4_rx(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port3_tx(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port3_rx(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port2_tx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port2_rx(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port1_tx(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port1_rx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port0_tx(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port0_rx(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfPortEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfPortEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfPortEnReg {
    #[inline(always)]
    fn default() -> RfPortEnReg {
        <crate::RegValueT<RfPortEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfPortPolReg_SPEC;
impl crate::sealed::RegSpec for RfPortPolReg_SPEC {
    type DataType = u32;
}

pub type RfPortPolReg = crate::RegValueT<RfPortPolReg_SPEC>;

impl RfPortPolReg {
    #[inline(always)]
    pub fn rf_port4_pol(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfPortPolReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfPortPolReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port3_pol(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfPortPolReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfPortPolReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port2_pol(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfPortPolReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfPortPolReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port1_pol(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfPortPolReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfPortPolReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_port0_pol(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfPortPolReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfPortPolReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfPortPolReg {
    #[inline(always)]
    fn default() -> RfPortPolReg {
        <crate::RegValueT<RfPortPolReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
