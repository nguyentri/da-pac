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
#[doc = r"SDADC registers"]
unsafe impl ::core::marker::Send for super::Sdadc {}
unsafe impl ::core::marker::Sync for super::Sdadc {}
impl super::Sdadc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Sigma Delta ADC Clear Interrupt Register"]
    #[inline(always)]
    pub const fn sdadc_clear_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SdadcClearIntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdadcClearIntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Sigma Delta ADC Control Register"]
    #[inline(always)]
    pub const fn sdadc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SdadcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdadcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Sigma Delta ADC Gain Correction Register"]
    #[inline(always)]
    pub const fn sdadc_gain_corr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SdadcGainCorrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdadcGainCorrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Sigma Delta ADC Offset Correction Register"]
    #[inline(always)]
    pub const fn sdadc_offs_corr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SdadcOffsCorrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdadcOffsCorrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Sigma Delta ADC Result Register"]
    #[inline(always)]
    pub const fn sdadc_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SdadcResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SdadcResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdadcClearIntReg_SPEC;
impl crate::sealed::RegSpec for SdadcClearIntReg_SPEC {
    type DataType = u32;
}
#[doc = "Sigma Delta ADC Clear Interrupt Register"]
pub type SdadcClearIntReg = crate::RegValueT<SdadcClearIntReg_SPEC>;

impl SdadcClearIntReg {
    #[doc = "Writing any value to this register will clear the ADC_INT interrupt. Reading returns 0."]
    #[inline(always)]
    pub fn sdadc_clr_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, SdadcClearIntReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, SdadcClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for SdadcClearIntReg {
    #[inline(always)]
    fn default() -> SdadcClearIntReg {
        <crate::RegValueT<SdadcClearIntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdadcCtrlReg_SPEC;
impl crate::sealed::RegSpec for SdadcCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Sigma Delta ADC Control Register"]
pub type SdadcCtrlReg = crate::RegValueT<SdadcCtrlReg_SPEC>;

impl SdadcCtrlReg {
    #[doc = "0: DMA functionality disabled\n1: DMA functionality enabled"]
    #[inline(always)]
    pub fn sdadc_dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable (mask) SDADC_ADC_INT.\n1: Enable SDADC_ADC_INT to ICU."]
    #[inline(always)]
    pub fn sdadc_mint(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: AD conversion ready and has generated an interrupt. Must be cleared by writing any value to SDADC_CLEAR_INT_REG."]
    #[inline(always)]
    pub fn sdadc_int(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SdadcCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,SdadcCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1: Internal LDO is ready for use"]
    #[inline(always)]
    pub fn sdadc_ldo_ok(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SdadcCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,SdadcCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: Internal bandgap reference.\n1: External reference."]
    #[inline(always)]
    pub fn sdadc_vref_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Manual ADC mode, a single result will be generated after setting the SDADC_START bit.\n1: Continuous ADC mode, new ADC results will be constantly stored in SDADC_RESULT_REG. Still SDADC_START has to be set to start the execution. Wait for SDADC_START to become zero after clearing the SDADC_CONT bit to stop the continuous mode."]
    #[inline(always)]
    pub fn sdadc_cont(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oversample Rate\n0: 128x\n1: 256x\n2: 512x\n3: 1024x"]
    #[inline(always)]
    pub fn sdadc_osr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Differential mode\n1: Single ended mode (Input selection negative side is ignored)"]
    #[inline(always)]
    pub fn sdadc_se(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input selection of negative side.\n0: ADC0 / P1\\[09\\]\n1: ADC1 / P0\\[25\\]\n2: ADC2 / P0\\[08\\]\n3: ADC3 / P0\\[09\\]\n4: ADC4 / P1\\[14\\]\n5: ADC5 / P1\\[20\\]\n6: ADC6 / P1\\[21\\]\n7: ADC7 / P1\\[22\\]"]
    #[inline(always)]
    pub fn sdadc_inn_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input selection of positive side.\n0: ADC0 / P1\\[09\\]\n1: ADC1 / P0\\[25\\]\n2: ADC2 / P0\\[08\\]\n3: ADC3 / P0\\[09\\]\n4: ADC4 / P1\\[14\\]\n5: ADC5 / P1\\[20\\]\n6: ADC6 / P1\\[21\\]\n7: ADC7 / P1\\[22\\]\n8: VBAT (via 4x attenuator, INN connected to ground)"]
    #[inline(always)]
    pub fn sdadc_inp_sel(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0xf,1,0,u8, SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: ADC conversion ready.\n1: If a 1 is written, the ADC starts a conversion. After the conversion this bit will be set to 0 and the SDADC_INT bit will be set. It is not allowed to write this bit while it is not (yet) zero."]
    #[inline(always)]
    pub fn sdadc_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: LDO is off and ADC is disabled.\n1: LDO, bias currents and modulator are enabled."]
    #[inline(always)]
    pub fn sdadc_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdadcCtrlReg {
    #[inline(always)]
    fn default() -> SdadcCtrlReg {
        <crate::RegValueT<SdadcCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdadcGainCorrReg_SPEC;
impl crate::sealed::RegSpec for SdadcGainCorrReg_SPEC {
    type DataType = u32;
}
#[doc = "Sigma Delta ADC Gain Correction Register"]
pub type SdadcGainCorrReg = crate::RegValueT<SdadcGainCorrReg_SPEC>;

impl SdadcGainCorrReg {
    #[doc = "Gain adjust"]
    #[inline(always)]
    pub fn sdadc_gain_corr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, SdadcGainCorrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, SdadcGainCorrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdadcGainCorrReg {
    #[inline(always)]
    fn default() -> SdadcGainCorrReg {
        <crate::RegValueT<SdadcGainCorrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdadcOffsCorrReg_SPEC;
impl crate::sealed::RegSpec for SdadcOffsCorrReg_SPEC {
    type DataType = u32;
}
#[doc = "Sigma Delta ADC Offset Correction Register"]
pub type SdadcOffsCorrReg = crate::RegValueT<SdadcOffsCorrReg_SPEC>;

impl SdadcOffsCorrReg {
    #[doc = "Offset adjust"]
    #[inline(always)]
    pub fn sdadc_offs_corr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, SdadcOffsCorrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, SdadcOffsCorrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SdadcOffsCorrReg {
    #[inline(always)]
    fn default() -> SdadcOffsCorrReg {
        <crate::RegValueT<SdadcOffsCorrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdadcResultReg_SPEC;
impl crate::sealed::RegSpec for SdadcResultReg_SPEC {
    type DataType = u32;
}
#[doc = "Sigma Delta ADC Result Register"]
pub type SdadcResultReg = crate::RegValueT<SdadcResultReg_SPEC>;

impl SdadcResultReg {
    #[doc = "Returns up to 16 bits linear value of the last AD conversion. The effective resolution depends on the OSR used."]
    #[inline(always)]
    pub fn sdadc_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, SdadcResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, SdadcResultReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SdadcResultReg {
    #[inline(always)]
    fn default() -> SdadcResultReg {
        <crate::RegValueT<SdadcResultReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
