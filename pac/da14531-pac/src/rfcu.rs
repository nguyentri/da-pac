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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:04 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"RFCU registers"]
unsafe impl ::core::marker::Send for super::Rfcu {}
unsafe impl ::core::marker::Sync for super::Rfcu {}
impl super::Rfcu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = ""]
    #[inline(always)]
    pub const fn rf_adci_dc_offset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdciDcOffsetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdciDcOffsetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_adcq_dc_offset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcqDcOffsetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcqDcOffsetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_adc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_adc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_adc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdcCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdcCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_adplldig_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdplldigCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdplldigCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_adplldig_rfmon_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAdplldigRfmonCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAdplldigRfmonCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_agc_ext_lut_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAgcExtLutReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAgcExtLutReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_attr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfAttrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfAttrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_calstate_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCalstateReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCalstateReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_cal_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfCalCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfCalCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_diagirq_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDiagirqCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_diagirq_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfDiagirqStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfDiagirqStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_iff_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfIffCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfIffCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_io_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfIoCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfIoCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_irq_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfIrqCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfIrqCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_ldo_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLdoCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLdoCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_ldo_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLdoStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLdoStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_ldo_vref_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLdoVrefSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLdoVrefSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_lna_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLnaCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLnaCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_lna_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLnaCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLnaCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_lna_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfLnaCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfLnaCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_mixer_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfMixerCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfMixerCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_mixer_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfMixerCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfMixerCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_overrule_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfOverruleReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfOverruleReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_pa_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfPaCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfPaCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_radio_init_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfRadioInitReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfRadioInitReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_rfcu_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfRfcuCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfRfcuCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_scan_feedback_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfScanFeedbackReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfScanFeedbackReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn rf_spare_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfSpareReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfSpareReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdciDcOffsetReg_SPEC;
impl crate::sealed::RegSpec for RfAdciDcOffsetReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfAdciDcOffsetReg = crate::RegValueT<RfAdciDcOffsetReg_SPEC>;

impl RfAdciDcOffsetReg {
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offn_i_rd(
        self,
    ) -> crate::common::RegisterField<9, 0x1ff, 1, 0, u16, RfAdciDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1ff,1,0,u16, RfAdciDcOffsetReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offp_i_rd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, RfAdciDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, RfAdciDcOffsetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAdciDcOffsetReg {
    #[inline(always)]
    fn default() -> RfAdciDcOffsetReg {
        <crate::RegValueT<RfAdciDcOffsetReg_SPEC> as RegisterValue<_>>::new(131328)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdcqDcOffsetReg_SPEC;
impl crate::sealed::RegSpec for RfAdcqDcOffsetReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfAdcqDcOffsetReg = crate::RegValueT<RfAdcqDcOffsetReg_SPEC>;

impl RfAdcqDcOffsetReg {
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offn_q_rd(
        self,
    ) -> crate::common::RegisterField<9, 0x1ff, 1, 0, u16, RfAdcqDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1ff,1,0,u16, RfAdcqDcOffsetReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offp_q_rd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, RfAdcqDcOffsetReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, RfAdcqDcOffsetReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAdcqDcOffsetReg {
    #[inline(always)]
    fn default() -> RfAdcqDcOffsetReg {
        <crate::RegValueT<RfAdcqDcOffsetReg_SPEC> as RegisterValue<_>>::new(131328)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdcCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfAdcCtrl1Reg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfAdcCtrl1Reg = crate::RegValueT<RfAdcCtrl1Reg_SPEC>;

impl RfAdcCtrl1Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn adc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RfAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,RfAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
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
    type DataType = u32;
}
#[doc = ""]
pub type RfAdcCtrl2Reg = crate::RegValueT<RfAdcCtrl2Reg_SPEC>;

impl RfAdcCtrl2Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offn_i_wr(
        self,
    ) -> crate::common::RegisterField<9, 0x1ff, 1, 0, u16, RfAdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1ff,1,0,u16, RfAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offp_i_wr(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, RfAdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, RfAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u32;
}
#[doc = ""]
pub type RfAdcCtrl3Reg = crate::RegValueT<RfAdcCtrl3Reg_SPEC>;

impl RfAdcCtrl3Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offn_q_wr(
        self,
    ) -> crate::common::RegisterField<9, 0x1ff, 1, 0, u16, RfAdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1ff,1,0,u16, RfAdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adc_offp_q_wr(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, RfAdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, RfAdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct RfAdplldigCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfAdplldigCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfAdplldigCtrlReg = crate::RegValueT<RfAdplldigCtrlReg_SPEC>;

impl RfAdplldigCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn pwr_sw_tim_ctrl(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, RfAdplldigCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8, RfAdplldigCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn openloop_rdy_wr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfAdplldigCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfAdplldigCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn openloop_rdy_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfAdplldigCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfAdplldigCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAdplldigCtrlReg {
    #[inline(always)]
    fn default() -> RfAdplldigCtrlReg {
        <crate::RegValueT<RfAdplldigCtrlReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAdplldigRfmonCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfAdplldigRfmonCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfAdplldigRfmonCtrlReg = crate::RegValueT<RfAdplldigRfmonCtrlReg_SPEC>;

impl RfAdplldigRfmonCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_rfmon_spare(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        u8,
        RfAdplldigRfmonCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            u8,
            RfAdplldigRfmonCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_rfmon_mux_sel(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        u8,
        RfAdplldigRfmonCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            u8,
            RfAdplldigRfmonCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_sync_clk_inv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfAdplldigRfmonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,RfAdplldigRfmonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAdplldigRfmonCtrlReg {
    #[inline(always)]
    fn default() -> RfAdplldigRfmonCtrlReg {
        <crate::RegValueT<RfAdplldigRfmonCtrlReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAgcExtLutReg_SPEC;
impl crate::sealed::RegSpec for RfAgcExtLutReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfAgcExtLutReg = crate::RegValueT<RfAgcExtLutReg_SPEC>;

impl RfAgcExtLutReg {
    #[doc = ""]
    #[inline(always)]
    pub fn agc_ext_lut(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, RfAgcExtLutReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, RfAgcExtLutReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAgcExtLutReg {
    #[inline(always)]
    fn default() -> RfAgcExtLutReg {
        <crate::RegValueT<RfAgcExtLutReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfAttrReg_SPEC;
impl crate::sealed::RegSpec for RfAttrReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfAttrReg = crate::RegValueT<RfAttrReg_SPEC>;

impl RfAttrReg {
    #[doc = ""]
    #[inline(always)]
    pub fn pa_power_setting(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, RfAttrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, RfAttrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn tia_bias(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfAttrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfAttrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_bias(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, RfAttrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, RfAttrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn iff_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfAttrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfAttrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfAttrReg {
    #[inline(always)]
    fn default() -> RfAttrReg {
        <crate::RegValueT<RfAttrReg_SPEC> as RegisterValue<_>>::new(201328672)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCalstateReg_SPEC;
impl crate::sealed::RegSpec for RfCalstateReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfCalstateReg = crate::RegValueT<RfCalstateReg_SPEC>;

impl RfCalstateReg {
    #[doc = ""]
    #[inline(always)]
    pub fn calstate(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RfCalstateReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, RfCalstateReg_SPEC,crate::common::R>::from_register(self,0)
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
    type DataType = u32;
}
#[doc = ""]
pub type RfCalCtrlReg = crate::RegValueT<RfCalCtrlReg_SPEC>;

impl RfCalCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn dc_offset_cal_dis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_cal_ctrl_spare(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfCalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn eo_cal(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfCalCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,RfCalCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn so_cal(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfCalCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,RfCalCtrlReg_SPEC,crate::common::W>::from_register(self,0)
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
pub struct RfDiagirqCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfDiagirqCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfDiagirqCtrlReg = crate::RegValueT<RfDiagirqCtrlReg_SPEC>;

impl RfDiagirqCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus3_edge_sel(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus3_bit_sel(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x7,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus3_sel(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x3,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus3_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus2_edge_sel(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus2_bit_sel(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x7,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus2_sel(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus2_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus1_edge_sel(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus1_bit_sel(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus1_sel(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x3,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus1_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus0_edge_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus0_bit_sel(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus0_sel(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, RfDiagirqCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8, RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn diag_bus0_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfDiagirqCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfDiagirqCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfDiagirqCtrlReg {
    #[inline(always)]
    fn default() -> RfDiagirqCtrlReg {
        <crate::RegValueT<RfDiagirqCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfDiagirqStatReg_SPEC;
impl crate::sealed::RegSpec for RfDiagirqStatReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfDiagirqStatReg = crate::RegValueT<RfDiagirqStatReg_SPEC>;

impl RfDiagirqStatReg {
    #[doc = ""]
    #[inline(always)]
    pub fn diagirq_stat(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RfDiagirqStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, RfDiagirqStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfDiagirqStatReg {
    #[inline(always)]
    fn default() -> RfDiagirqStatReg {
        <crate::RegValueT<RfDiagirqStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfIffCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfIffCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfIffCtrlReg = crate::RegValueT<RfIffCtrlReg_SPEC>;

impl RfIffCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn iff_dcoc_dac_refcur_ctrl(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, RfIffCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, RfIffCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn iff_complex_dis(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RfIffCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,RfIffCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_iff_ctrl_spare(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, RfIffCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3f,1,0,u8, RfIffCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn iff_dcoc_dac_dis(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfIffCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfIffCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn if_mute(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfIffCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfIffCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn if_cal_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, RfIffCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, RfIffCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfIffCtrlReg {
    #[inline(always)]
    fn default() -> RfIffCtrlReg {
        <crate::RegValueT<RfIffCtrlReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfIoCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfIoCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfIoCtrlReg = crate::RegValueT<RfIoCtrlReg_SPEC>;

impl RfIoCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rfio_tune_cap_trim_tx(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, RfIoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, RfIoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rfio_tune_cap_trim_rx(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RfIoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, RfIoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfIoCtrlReg {
    #[inline(always)]
    fn default() -> RfIoCtrlReg {
        <crate::RegValueT<RfIoCtrlReg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfIrqCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfIrqCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfIrqCtrlReg = crate::RegValueT<RfIrqCtrlReg_SPEC>;

impl RfIrqCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn eo_cal_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfIrqCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,RfIrqCtrlReg_SPEC,crate::common::W>::from_register(self,0)
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
pub struct RfLdoCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfLdoCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfLdoCtrlReg = crate::RegValueT<RfLdoCtrlReg_SPEC>;

impl RfLdoCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dco_hold_ovr_en(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dco_hold_ovr_val(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dtc_hold_ovr_en(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dtc_hold_ovr_val(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_radio_hold_ovr_en(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_radio_hold_ovr_val(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_vref_smpl_time(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, RfLdoCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8, RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dco_cont_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dco_level(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dtc_cont_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dtc_level(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_radio_cont_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_radio_level(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, RfLdoCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, RfLdoCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLdoCtrlReg {
    #[inline(always)]
    fn default() -> RfLdoCtrlReg {
        <crate::RegValueT<RfLdoCtrlReg_SPEC> as RegisterValue<_>>::new(1639219)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLdoStatusReg_SPEC;
impl crate::sealed::RegSpec for RfLdoStatusReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfLdoStatusReg = crate::RegValueT<RfLdoStatusReg_SPEC>;

impl RfLdoStatusReg {
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dtc_vref_hold_rd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dco_vref_hold_rd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_radio_vref_hold_rd(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dtc_en_rd(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn ldo_dco_en_rd(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_ldo_zero_en_rd(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_ldo_en_rd(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn radio_ldo_zero_en_rd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn radio_ldo_en_rd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfLdoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RfLdoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLdoStatusReg {
    #[inline(always)]
    fn default() -> RfLdoStatusReg {
        <crate::RegValueT<RfLdoStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLdoVrefSelReg_SPEC;
impl crate::sealed::RegSpec for RfLdoVrefSelReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfLdoVrefSelReg = crate::RegValueT<RfLdoVrefSelReg_SPEC>;

impl RfLdoVrefSelReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rf_ldo_dco_vref_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfLdoVrefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfLdoVrefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_ldo_dtc_vref_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfLdoVrefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfLdoVrefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_ldo_radio_vref_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfLdoVrefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfLdoVrefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLdoVrefSelReg {
    #[inline(always)]
    fn default() -> RfLdoVrefSelReg {
        <crate::RegValueT<RfLdoVrefSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLnaCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfLnaCtrl1Reg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfLnaCtrl1Reg = crate::RegValueT<RfLnaCtrl1Reg_SPEC>;

impl RfLnaCtrl1Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain4_hp(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, RfLnaCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1f,1,0,u8, RfLnaCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain3_hp(
        self,
    ) -> crate::common::RegisterField<15, 0x1f, 1, 0, u8, RfLnaCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1f,1,0,u8, RfLnaCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain2_hp(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, RfLnaCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8, RfLnaCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain1_hp(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, RfLnaCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8, RfLnaCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain0_hp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, RfLnaCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, RfLnaCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLnaCtrl1Reg {
    #[inline(always)]
    fn default() -> RfLnaCtrl1Reg {
        <crate::RegValueT<RfLnaCtrl1Reg_SPEC> as RegisterValue<_>>::new(17318416)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLnaCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfLnaCtrl2Reg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfLnaCtrl2Reg = crate::RegValueT<RfLnaCtrl2Reg_SPEC>;

impl RfLnaCtrl2Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain4_lp(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, RfLnaCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1f,1,0,u8, RfLnaCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain3_lp(
        self,
    ) -> crate::common::RegisterField<15, 0x1f, 1, 0, u8, RfLnaCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1f,1,0,u8, RfLnaCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain2_lp(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, RfLnaCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8, RfLnaCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain1_lp(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, RfLnaCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8, RfLnaCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_gain0_lp(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, RfLnaCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, RfLnaCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLnaCtrl2Reg {
    #[inline(always)]
    fn default() -> RfLnaCtrl2Reg {
        <crate::RegValueT<RfLnaCtrl2Reg_SPEC> as RegisterValue<_>>::new(14071213)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLnaCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for RfLnaCtrl3Reg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfLnaCtrl3Reg = crate::RegValueT<RfLnaCtrl3Reg_SPEC>;

impl RfLnaCtrl3Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn lna_spare(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_mode_gain4_lp(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_mode_gain3_lp(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_mode_gain2_lp(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_mode_gain1_lp(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_mode_gain0_lp(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn lna_trim_casc(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, RfLnaCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, RfLnaCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfLnaCtrl3Reg {
    #[inline(always)]
    fn default() -> RfLnaCtrl3Reg {
        <crate::RegValueT<RfLnaCtrl3Reg_SPEC> as RegisterValue<_>>::new(1118484)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfMixerCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for RfMixerCtrl1Reg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfMixerCtrl1Reg = crate::RegValueT<RfMixerCtrl1Reg_SPEC>;

impl RfMixerCtrl1Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn mixer_ip2_dac_q_trim(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16, RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn mixer_ip2_dac_i_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, RfMixerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, RfMixerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfMixerCtrl1Reg {
    #[inline(always)]
    fn default() -> RfMixerCtrl1Reg {
        <crate::RegValueT<RfMixerCtrl1Reg_SPEC> as RegisterValue<_>>::new(17760527)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfMixerCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for RfMixerCtrl2Reg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfMixerCtrl2Reg = crate::RegValueT<RfMixerCtrl2Reg_SPEC>;

impl RfMixerCtrl2Reg {
    #[doc = ""]
    #[inline(always)]
    pub fn mix_cal_select(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RfMixerCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,RfMixerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn mix_cal_cap_wr_2m(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, RfMixerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, RfMixerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn mix_cal_cap_wr_1m(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RfMixerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, RfMixerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfMixerCtrl2Reg {
    #[inline(always)]
    fn default() -> RfMixerCtrl2Reg {
        <crate::RegValueT<RfMixerCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfOverruleReg_SPEC;
impl crate::sealed::RegSpec for RfOverruleReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfOverruleReg = crate::RegValueT<RfOverruleReg_SPEC>;

impl RfOverruleReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rx_en_ovr(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn tx_en_ovr(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, RfOverruleReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, RfOverruleReg_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u32;
}
#[doc = ""]
pub type RfPaCtrlReg = crate::RegValueT<RfPaCtrlReg_SPEC>;

impl RfPaCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn pa_ramp_step_speed(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, RfPaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn trim_duty_neg(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, RfPaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn trim_duty_pos(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, RfPaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, RfPaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfPaCtrlReg {
    #[inline(always)]
    fn default() -> RfPaCtrlReg {
        <crate::RegValueT<RfPaCtrlReg_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfRadioInitReg_SPEC;
impl crate::sealed::RegSpec for RfRadioInitReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfRadioInitReg = crate::RegValueT<RfRadioInitReg_SPEC>;

impl RfRadioInitReg {
    #[doc = ""]
    #[inline(always)]
    pub fn radio_init_autoclear(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_hclk_dis(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn radio_regs_rdy(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_hclk_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_hreset_n(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_ldo_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_ldo_en_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn adplldig_pwr_sw1_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn radio_ldo_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn radio_ldo_en_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn radio_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfRadioInitReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfRadioInitReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfRadioInitReg {
    #[inline(always)]
    fn default() -> RfRadioInitReg {
        <crate::RegValueT<RfRadioInitReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfRfcuCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfRfcuCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfRfcuCtrlReg = crate::RegValueT<RfRfcuCtrlReg_SPEC>;

impl RfRfcuCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rf_rfcu_clk_div(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfRfcuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfRfcuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfRfcuCtrlReg {
    #[inline(always)]
    fn default() -> RfRfcuCtrlReg {
        <crate::RegValueT<RfRfcuCtrlReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfScanFeedbackReg_SPEC;
impl crate::sealed::RegSpec for RfScanFeedbackReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfScanFeedbackReg = crate::RegValueT<RfScanFeedbackReg_SPEC>;

impl NoBitfieldReg<RfScanFeedbackReg_SPEC> for RfScanFeedbackReg {}
impl ::core::default::Default for RfScanFeedbackReg {
    #[inline(always)]
    fn default() -> RfScanFeedbackReg {
        <crate::RegValueT<RfScanFeedbackReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfSpareReg_SPEC;
impl crate::sealed::RegSpec for RfSpareReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type RfSpareReg = crate::RegValueT<RfSpareReg_SPEC>;

impl RfSpareReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rf_spare_in_en(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RfSpareReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,RfSpareReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_spare_in(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, RfSpareReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8, RfSpareReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_spare_bits_hv(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, RfSpareReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, RfSpareReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn rf_spare_bits(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, RfSpareReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, RfSpareReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfSpareReg {
    #[inline(always)]
    fn default() -> RfSpareReg {
        <crate::RegValueT<RfSpareReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
