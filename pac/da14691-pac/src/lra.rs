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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:53:38 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"LRA registers"]
unsafe impl ::core::marker::Send for super::Lra {}
unsafe impl ::core::marker::Sync for super::Lra {}
impl super::Lra {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "General Purpose ADC Control Register"]
    #[inline(always)]
    pub const fn lra_adc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraAdcCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "General Purpose ADC Result Register"]
    #[inline(always)]
    pub const fn lra_adc_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraAdcResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraAdcResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "LRA Bridge Register"]
    #[inline(always)]
    pub const fn lra_brd_hs_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraBrdHsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraBrdHsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "LRA Bridge Register"]
    #[inline(always)]
    pub const fn lra_brd_ls_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraBrdLsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraBrdLsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "LRA Bridge Staus Register"]
    #[inline(always)]
    pub const fn lra_brd_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraBrdStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraBrdStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "General Purpose LRA Control Register"]
    #[inline(always)]
    pub const fn lra_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "General Purpose LRA Control Register"]
    #[inline(always)]
    pub const fn lra_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "LRA test Register"]
    #[inline(always)]
    pub const fn lra_dft_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraDftReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraDftReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "LRA Filter Coefficient Register"]
    #[inline(always)]
    pub const fn lra_flt_coef1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltCoef1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltCoef1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "LRA Filter Coefficient Register"]
    #[inline(always)]
    pub const fn lra_flt_coef2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltCoef2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltCoef2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "LRA Filter Coefficient Register"]
    #[inline(always)]
    pub const fn lra_flt_coef3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltCoef3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltCoef3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "LRA Sample Register"]
    #[inline(always)]
    pub const fn lra_flt_smp8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraFltSmp8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraFltSmp8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "LRA LDO Regsiter"]
    #[inline(always)]
    pub const fn lra_ldo_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LraLdoReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LraLdoReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraAdcCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for LraAdcCtrl1Reg_SPEC {
    type DataType = u32;
}
#[doc = "General Purpose ADC Control Register"]
pub type LraAdcCtrl1Reg = crate::RegValueT<LraAdcCtrl1Reg_SPEC>;

impl LraAdcCtrl1Reg {
    #[doc = "0:ADC conversion ready.\n1:ADC conversion in progress."]
    #[inline(always)]
    pub fn lra_adc_busy(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,LraAdcCtrl1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADC offset compensation value.\nSigned value with 3 fractional bits.\n-16 (0x80) to +15.875 (0x7F) in intervals of 0.125 (0x01).\nNote: ADC gain error must be compensated in the calculation of VREF."]
    #[inline(always)]
    pub fn lra_adc_offset(
        self,
    ) -> crate::common::RegisterField<9, 0xff, 1, 0, u8, LraAdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xff,1,0,u8, LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select which inputs will be enabled on the ADC.\n0,1 = normal inputs (i.e. both I and Q inputs connected to LRA-current-sense voltage source)\n2 = I channel connected to the analog input testbus on PORTS P14 and P15, Q channel is muted.\n3 =Q channel connected to the analog input testbus on PORTS P14 and P15, I channel is muted.\nNote: The LRA_ADC_CTRL1_REG\\[ADC_MUTE\\] field takes precedence over this test functionality."]
    #[inline(always)]
    pub fn lra_adc_test_param(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select analog testbus on ADC input."]
    #[inline(always)]
    pub fn lra_adc_test_in_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC clock divider"]
    #[inline(always)]
    pub fn lra_adc_freq(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, LraAdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8, LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Change polarity of ADC input"]
    #[inline(always)]
    pub fn lra_adc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation\n1: Short the inputs of the ADC (used for DC offset cal)"]
    #[inline(always)]
    pub fn lra_adc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: ADC conversion ready.\n1: If a 1 is written, the ADC starts a conversion. After the conversion this bit will be set to 0 and the GP_ADC_INT bit will be set. It is not allowed to write this bit while it is not (yet) zero."]
    #[inline(always)]
    pub fn lra_adc_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,LraAdcCtrl1Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for LraAdcCtrl1Reg {
    #[inline(always)]
    fn default() -> LraAdcCtrl1Reg {
        <crate::RegValueT<LraAdcCtrl1Reg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraAdcResultReg_SPEC;
impl crate::sealed::RegSpec for LraAdcResultReg_SPEC {
    type DataType = u32;
}
#[doc = "General Purpose ADC Result Register"]
pub type LraAdcResultReg = crate::RegValueT<LraAdcResultReg_SPEC>;

impl LraAdcResultReg {
    #[doc = "Manual value to replace the ADC output. Select its use by FLT_IN_SEL."]
    #[inline(always)]
    pub fn man_flt_in(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraAdcResultReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraAdcResultReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Returns the 10 up to 16 bits linear value of the last AD conversion as a signed value. The most significant 11 bits are always valid, the lower 5 bits are only valid in case oversampling has been applied. Two samples results in one extra bit and 32 samples results in 5 extra bits.\nIn the context of the LRA constant current or constant duty cycle control systems, the (non-oversampled) value is interpreted as a signed value with 7 integer bits and 3 fractional bits: -128.000 (0x8000) to +127.875 (0x7FE0) in steps of 0.125 (0x0010). \nNote that the measured values in this context are always positive."]
    #[inline(always)]
    pub fn gp_adc_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraAdcResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraAdcResultReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraAdcResultReg {
    #[inline(always)]
    fn default() -> LraAdcResultReg {
        <crate::RegValueT<LraAdcResultReg_SPEC> as RegisterValue<_>>::new(16384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraBrdHsReg_SPEC;
impl crate::sealed::RegSpec for LraBrdHsReg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Bridge Register"]
pub type LraBrdHsReg = crate::RegValueT<LraBrdHsReg_SPEC>;

impl LraBrdHsReg {
    #[doc = "Current-sensing amplifier gain settings:\n0001: x6\n0010: x8\n0100: x10\n1000: x12"]
    #[inline(always)]
    pub fn trim_gain(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, LraBrdHsReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0xf,1,0,u8, LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HS gnd trim, default at 100\n000: 2.2V and 111:3.6V with 0.2V per step"]
    #[inline(always)]
    pub fn hsgnd_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, LraBrdHsReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HS short-circuit protection limit trimming"]
    #[inline(always)]
    pub fn scp_hs_trim(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, LraBrdHsReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HS short-circuit protection enable"]
    #[inline(always)]
    pub fn scp_hs_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraBrdHsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HS edge-rate control trimming. Lowto-High switching slewing:\n00: 25 MV/s\n01: 50 MV/s\n10: 75 MV/s\n11: 100 MV/s"]
    #[inline(always)]
    pub fn erc_hs_trim(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, LraBrdHsReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HS edge-rate control enable"]
    #[inline(always)]
    pub fn erc_hs_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LraBrdHsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraBrdHsReg {
    #[inline(always)]
    fn default() -> LraBrdHsReg {
        <crate::RegValueT<LraBrdHsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraBrdLsReg_SPEC;
impl crate::sealed::RegSpec for LraBrdLsReg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Bridge Register"]
pub type LraBrdLsReg = crate::RegValueT<LraBrdLsReg_SPEC>;

impl LraBrdLsReg {
    #[doc = "LSN short-circuit protection limit trimming"]
    #[inline(always)]
    pub fn scp_ls_trim_n(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, LraBrdLsReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LSP short-circuit protection limit trimming"]
    #[inline(always)]
    pub fn scp_ls_trim_p(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, LraBrdLsReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LS short-circuit protection enable"]
    #[inline(always)]
    pub fn scp_ls_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraBrdLsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LS edge-rate control trimming. High-to-Low switching slewing:\n00: 25 MV/s\n01: 50 MV/s\n10: 75 MV/s\n11: 100 MV/s"]
    #[inline(always)]
    pub fn erc_ls_trim(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, LraBrdLsReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LS edge-rate control enable"]
    #[inline(always)]
    pub fn erc_ls_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LraBrdLsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraBrdLsReg {
    #[inline(always)]
    fn default() -> LraBrdLsReg {
        <crate::RegValueT<LraBrdLsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraBrdStatReg_SPEC;
impl crate::sealed::RegSpec for LraBrdStatReg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Bridge Staus Register"]
pub type LraBrdStatReg = crate::RegValueT<LraBrdStatReg_SPEC>;

impl LraBrdStatReg {
    #[doc = "HS short circuit comparator output"]
    #[inline(always)]
    pub fn scp_hs_out(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LSN short circuit comparator output"]
    #[inline(always)]
    pub fn scp_ls_comp_out_n(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LSP short circuit comparator output"]
    #[inline(always)]
    pub fn scp_ls_comp_out_p(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: LS short-circuit event detected\n0: no LS short-circuit event detected"]
    #[inline(always)]
    pub fn sc_event_ls(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: HS short-circuit event detected\n0: no HS short-circuit event detected"]
    #[inline(always)]
    pub fn sc_event_hs(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: Loop saturation detected\n0: Loop not saturated"]
    #[inline(always)]
    pub fn loop_stat(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LSN control status"]
    #[inline(always)]
    pub fn lsn_on(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LSP control status"]
    #[inline(always)]
    pub fn lsp_on(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSN control status"]
    #[inline(always)]
    pub fn hsn_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSP control status"]
    #[inline(always)]
    pub fn hsp_on(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LSN power FET gate actual status"]
    #[inline(always)]
    pub fn lsn_stat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LSP power FET gate actual status"]
    #[inline(always)]
    pub fn lsp_stat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSN power FET gate actual status"]
    #[inline(always)]
    pub fn hsn_stat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSP power FET gate actual status"]
    #[inline(always)]
    pub fn hsp_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraBrdStatReg {
    #[inline(always)]
    fn default() -> LraBrdStatReg {
        <crate::RegValueT<LraBrdStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for LraCtrl1Reg_SPEC {
    type DataType = u32;
}
#[doc = "General Purpose LRA Control Register"]
pub type LraCtrl1Reg = crate::RegValueT<LraCtrl1Reg_SPEC>;

impl LraCtrl1Reg {
    #[doc = "Current bin index (0-15). Check if equal to IRQ_IDX before and/or after updating HALF_PERIOD with ISR."]
    #[inline(always)]
    pub fn smp_idx(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, LraCtrl1Reg_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8, LraCtrl1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = interrupt scp event disabled\n1 = interupt scp event enabled"]
    #[inline(always)]
    pub fn irq_scp_event_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = interrupt adc disabled\n1 = interupt adc enabled"]
    #[inline(always)]
    pub fn irq_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = interrupt controller disabled\n1 = interupt controller enabled"]
    #[inline(always)]
    pub fn irq_ctrl_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "At which sample index an IRQ will be generated (0-15). When IRQ_IDX < 8, IRQs are generated at both half cycles (IRQ_IDX and IRQ_IDX+8), otherwise only in the second half cycle."]
    #[inline(always)]
    pub fn irq_idx(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider value of the interrupt request. Number of LRA/ERM periods, between successive IRQs. 0,1=every (half) cycle, depending on IRQ_IDX; 2=every second cycle, IRQ at the end of first or both half cycles (based on IRQ_IDX), etc."]
    #[inline(always)]
    pub fn irq_div(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select which samples to store for the resonance control algorithm.\n0=Sense voltage after down-sampling\n1=Error voltage (after subtraction of VREF and down-sampled sense voltgae input)\n2=Duty cycle signal after loop-filter\n3=Duty cycle signal after summation with DREF"]
    #[inline(always)]
    pub fn smp_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LXP and LXN node pull down enbale, when SC_EVENT=0 && LOOP_EN=0"]
    #[inline(always)]
    pub fn pulldown_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0=disable loop\n1=enable loop"]
    #[inline(always)]
    pub fn loop_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0=lra ldo disabled\n1=lra ldo enabled"]
    #[inline(always)]
    pub fn ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0=lra adc disabled\n1=lra adc enabled"]
    #[inline(always)]
    pub fn adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0=hbridge disabled\n1=hbridge enabled"]
    #[inline(always)]
    pub fn hbridge_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0=lra controller disabled\n1=lra controller enabled"]
    #[inline(always)]
    pub fn lra_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraCtrl1Reg {
    #[inline(always)]
    fn default() -> LraCtrl1Reg {
        <crate::RegValueT<LraCtrl1Reg_SPEC> as RegisterValue<_>>::new(29120)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for LraCtrl2Reg_SPEC {
    type DataType = u32;
}
#[doc = "General Purpose LRA Control Register"]
pub type LraCtrl2Reg = crate::RegValueT<LraCtrl2Reg_SPEC>;

impl LraCtrl2Reg {
    #[doc = "Half of the LRA period, in units of 4 ms (= 125 kHz divided by the resonant frequency of the LRA)."]
    #[inline(always)]
    pub fn half_period(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Automatic frequency control (0=disabled;, 1=enabled, not yet implemented)"]
    #[inline(always)]
    pub fn auto_mode(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sampling mode for data aiding automatic resonance control (0=averaging, 1=last sample)"]
    #[inline(always)]
    pub fn smp_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Polarity of the square wave (0=normal; 1=inverted); Use for rapid stop."]
    #[inline(always)]
    pub fn polarity(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = normal operation\n1 = ADC output overruled by register field MAN_FLT_IN"]
    #[inline(always)]
    pub fn flt_in_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PWM pulse placement: 0=middle, 1=left, 2=right, 3=alternate"]
    #[inline(always)]
    pub fn pwm_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraCtrl2Reg {
    #[inline(always)]
    fn default() -> LraCtrl2Reg {
        <crate::RegValueT<LraCtrl2Reg_SPEC> as RegisterValue<_>>::new(40960000)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraDftReg_SPEC;
impl crate::sealed::RegSpec for LraDftReg_SPEC {
    type DataType = u32;
}
#[doc = "LRA test Register"]
pub type LraDftReg = crate::RegValueT<LraDftReg_SPEC>;

impl LraDftReg {
    #[doc = "spare registers bits , currently not used"]
    #[inline(always)]
    pub fn spare(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8, LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0=use SWM from controller\n1=use SWM_MAN"]
    #[inline(always)]
    pub fn swm_sel(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "swm manual"]
    #[inline(always)]
    pub fn swm_man(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0=use PWM from controller\n1=use PWM_MAN"]
    #[inline(always)]
    pub fn pwm_sel(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "pwm manual"]
    #[inline(always)]
    pub fn pwm_man(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "20ns unit delay cell trimming bits"]
    #[inline(always)]
    pub fn timer_trim(
        self,
    ) -> crate::common::RegisterField<23, 0x3, 1, 0, u8, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0x3,1,0,u8, LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selection of delay of MAG and DEMAG signal:\n00: 60ns\n01: 80ns\n10: 100ns\n11: 120ns"]
    #[inline(always)]
    pub fn timer_scale_trim(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn dft_sel(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Force HSP and HSN power FETs on:\n0: not actived\n1: HSP and HSN are forced on"]
    #[inline(always)]
    pub fn dft_force_hspn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable for the timer trimming"]
    #[inline(always)]
    pub fn dft_en_timer(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Force state machine in a certain state:\n00: No test\n01: High-Z\n10: Mag\n11: Demag"]
    #[inline(always)]
    pub fn dft_stall(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selection of test bus connection"]
    #[inline(always)]
    pub fn dft_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraDftReg {
    #[inline(always)]
    fn default() -> LraDftReg {
        <crate::RegValueT<LraDftReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltCoef1Reg_SPEC;
impl crate::sealed::RegSpec for LraFltCoef1Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Filter Coefficient Register"]
pub type LraFltCoef1Reg = crate::RegValueT<LraFltCoef1Reg_SPEC>;

impl LraFltCoef1Reg {
    #[doc = "Loop filter state-space coefficient a12 (1 sign bit, 1 integer bit, 14 fractional bits, range -2.000 .. +1.999)."]
    #[inline(always)]
    pub fn flt_coef_01(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltCoef1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltCoef1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Loop filter state-space coefficient a11 (1 sign bit, 1 integer bit, 14 fractional bits, range -2.000 .. +1.999)."]
    #[inline(always)]
    pub fn flt_coef_00(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltCoef1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltCoef1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltCoef1Reg {
    #[inline(always)]
    fn default() -> LraFltCoef1Reg {
        <crate::RegValueT<LraFltCoef1Reg_SPEC> as RegisterValue<_>>::new(26873446)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltCoef2Reg_SPEC;
impl crate::sealed::RegSpec for LraFltCoef2Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Filter Coefficient Register"]
pub type LraFltCoef2Reg = crate::RegValueT<LraFltCoef2Reg_SPEC>;

impl LraFltCoef2Reg {
    #[doc = "Loop filter state-space coefficient a21 (1 sign bit, 1 integer bit, 14 fractional bits, range -2.000 .. +1.999)."]
    #[inline(always)]
    pub fn flt_coef_10(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltCoef2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltCoef2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Loop filter state-space coefficient b1 (1 sign bit, 1 integer bit, 14 fractional bits, range -2.000 .. +1.999).\nNote: For correct intended loop gain, modify the intended value of b1 to b1/ADC_GAIN, where\nADC_GAIN is the normalized gain of the ADC (i.e. ADC_GAIN = GP_ADC_VALUE´300 mA/\\[ILRA´128\\])."]
    #[inline(always)]
    pub fn flt_coef_02(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltCoef2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltCoef2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltCoef2Reg {
    #[inline(always)]
    fn default() -> LraFltCoef2Reg {
        <crate::RegValueT<LraFltCoef2Reg_SPEC> as RegisterValue<_>>::new(241569598)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltCoef3Reg_SPEC;
impl crate::sealed::RegSpec for LraFltCoef3Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Filter Coefficient Register"]
pub type LraFltCoef3Reg = crate::RegValueT<LraFltCoef3Reg_SPEC>;

impl LraFltCoef3Reg {
    #[doc = "Loop filter state-space coefficient b2 (1 sign bit, 1 integer bit, 14 fractional bits, range -2.000 .. +1.999).\nNote: For correct intended loop gain, modify the intended value of b1 to b1/ADC_GAIN, where\nADC_GAIN is the normalized gain of the ADC (i.e. ADC_GAIN = GP_ADC_VALUE´300 mA/\\[ILRA´128\\])."]
    #[inline(always)]
    pub fn flt_coef_12(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltCoef3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltCoef3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Loop filter state-space coefficient a22 (1 sign bit, 1 integer bit, 14 fractional bits, range -2.000 .. +1.999)."]
    #[inline(always)]
    pub fn flt_coef_11(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltCoef3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltCoef3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltCoef3Reg {
    #[inline(always)]
    fn default() -> LraFltCoef3Reg {
        <crate::RegValueT<LraFltCoef3Reg_SPEC> as RegisterValue<_>>::new(382599578)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp1Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp1Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp1Reg = crate::RegValueT<LraFltSmp1Reg_SPEC>;

impl LraFltSmp1Reg {
    #[doc = "Second sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_2(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "First sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp1Reg {
    #[inline(always)]
    fn default() -> LraFltSmp1Reg {
        <crate::RegValueT<LraFltSmp1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp2Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp2Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp2Reg = crate::RegValueT<LraFltSmp2Reg_SPEC>;

impl LraFltSmp2Reg {
    #[doc = "Fourth sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_4(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Third sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_3(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp2Reg {
    #[inline(always)]
    fn default() -> LraFltSmp2Reg {
        <crate::RegValueT<LraFltSmp2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp3Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp3Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp3Reg = crate::RegValueT<LraFltSmp3Reg_SPEC>;

impl LraFltSmp3Reg {
    #[doc = "Sixth sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_6(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Fifth sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_5(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp3Reg {
    #[inline(always)]
    fn default() -> LraFltSmp3Reg {
        <crate::RegValueT<LraFltSmp3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp4Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp4Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp4Reg = crate::RegValueT<LraFltSmp4Reg_SPEC>;

impl LraFltSmp4Reg {
    #[doc = "Eighth sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_8(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp4Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Seventh sample in first half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_7(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp4Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp4Reg {
    #[inline(always)]
    fn default() -> LraFltSmp4Reg {
        <crate::RegValueT<LraFltSmp4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp5Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp5Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp5Reg = crate::RegValueT<LraFltSmp5Reg_SPEC>;

impl LraFltSmp5Reg {
    #[doc = "Second sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_10(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp5Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp5Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "First sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_9(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp5Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp5Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp5Reg {
    #[inline(always)]
    fn default() -> LraFltSmp5Reg {
        <crate::RegValueT<LraFltSmp5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp6Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp6Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp6Reg = crate::RegValueT<LraFltSmp6Reg_SPEC>;

impl LraFltSmp6Reg {
    #[doc = "Fourth sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_12(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp6Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp6Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Third sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_11(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp6Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp6Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp6Reg {
    #[inline(always)]
    fn default() -> LraFltSmp6Reg {
        <crate::RegValueT<LraFltSmp6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp7Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp7Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp7Reg = crate::RegValueT<LraFltSmp7Reg_SPEC>;

impl LraFltSmp7Reg {
    #[doc = "Sixth sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_14(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp7Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp7Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Fifth sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_13(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp7Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp7Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp7Reg {
    #[inline(always)]
    fn default() -> LraFltSmp7Reg {
        <crate::RegValueT<LraFltSmp7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraFltSmp8Reg_SPEC;
impl crate::sealed::RegSpec for LraFltSmp8Reg_SPEC {
    type DataType = u32;
}
#[doc = "LRA Sample Register"]
pub type LraFltSmp8Reg = crate::RegValueT<LraFltSmp8Reg_SPEC>;

impl LraFltSmp8Reg {
    #[doc = "Eighth sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_16(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LraFltSmp8Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LraFltSmp8Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Seventh sample in second half-cycle used for resonance control algorithm."]
    #[inline(always)]
    pub fn lra_smp_15(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LraFltSmp8Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LraFltSmp8Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LraFltSmp8Reg {
    #[inline(always)]
    fn default() -> LraFltSmp8Reg {
        <crate::RegValueT<LraFltSmp8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LraLdoReg_SPEC;
impl crate::sealed::RegSpec for LraLdoReg_SPEC {
    type DataType = u32;
}
#[doc = "LRA LDO Regsiter"]
pub type LraLdoReg = crate::RegValueT<LraLdoReg_SPEC>;

impl LraLdoReg {
    #[doc = "0: LDO not yet ok\n1: LDO voltage is ready"]
    #[inline(always)]
    pub fn ldo_ok(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LraLdoReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,LraLdoReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "When set to 1, LDO output is connected to the testbus through a test switch"]
    #[inline(always)]
    pub fn ldo_tst(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraLdoReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LraLdoReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Indicates that the reference input is tracked,\n1: Indicates that the reference input is sampled"]
    #[inline(always)]
    pub fn ldo_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LraLdoReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LraLdoReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LraLdoReg {
    #[inline(always)]
    fn default() -> LraLdoReg {
        <crate::RegValueT<LraLdoReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
