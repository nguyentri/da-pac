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
#[doc = r"GPADC registers"]
unsafe impl ::core::marker::Send for super::Gpadc {}
unsafe impl ::core::marker::Sync for super::Gpadc {}
impl super::Gpadc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "General Purpose ADC Clear Interrupt Register"]
    #[inline(always)]
    pub const fn gp_adc_clear_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcClearIntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcClearIntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "General Purpose ADC Second Control Register"]
    #[inline(always)]
    pub const fn gp_adc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "General Purpose ADC Third Control Register"]
    #[inline(always)]
    pub const fn gp_adc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "General Purpose ADC Control Register"]
    #[inline(always)]
    pub const fn gp_adc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "General Purpose ADC Negative Offset Register"]
    #[inline(always)]
    pub const fn gp_adc_offn_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcOffnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcOffnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "General Purpose ADC Positive Offset Register"]
    #[inline(always)]
    pub const fn gp_adc_offp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcOffpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcOffpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn gp_adc_param_dif_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcParamDifReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcParamDifReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn gp_adc_param_se_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcParamSeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcParamSeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "General Purpose ADC Result Register"]
    #[inline(always)]
    pub const fn gp_adc_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "General Purpose ADC Trim Register"]
    #[inline(always)]
    pub const fn gp_adc_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcClearIntReg_SPEC;
impl crate::sealed::RegSpec for GpAdcClearIntReg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Clear Interrupt Register"]
pub type GpAdcClearIntReg = crate::RegValueT<GpAdcClearIntReg_SPEC>;

impl GpAdcClearIntReg {
    #[doc = "Writing any value to this register will clear the ADC_INT interrupt. Reading returns 0."]
    #[inline(always)]
    pub fn gp_adc_clr_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GpAdcClearIntReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, GpAdcClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcClearIntReg {
    #[inline(always)]
    fn default() -> GpAdcClearIntReg {
        <crate::RegValueT<GpAdcClearIntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for GpAdcCtrl2Reg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Second Control Register"]
pub type GpAdcCtrl2Reg = crate::RegValueT<GpAdcCtrl2Reg_SPEC>;

impl GpAdcCtrl2Reg {
    #[doc = "0: Data is stored after handshake synchronisation\n1: Data is stored 2 ADC_CLK cycles after internal start trigger\n7: Data is stored 8 ADC_CLK cycles after internal start trigger"]
    #[inline(always)]
    pub fn gp_adc_store_del(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, GpAdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8, GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: The sample time (switch is closed) is two ADC_CLK cycles\n1: The sample time is 1*8 ADC_CLK cycles\n2: The sample time is 2*8 ADC_CLK cycles\n15: The sample time is 15*8 ADC_CLK cycles"]
    #[inline(always)]
    pub fn gp_adc_smpl_time(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0xf,1,0,u8, GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: 1 sample is taken or 2 in case ADC_CHOP is active.\n1: 2 samples are taken.\n2: 4 samples are taken.\n7: 128 samples are taken."]
    #[inline(always)]
    pub fn gp_adc_conv_nrs(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Common mode adjust for offset shifter. Input range is CM +/- 450mV.\n0: CM = 1.25V (Input range 0.80 - 1.70)\n1: CM = 1.30V (Input range 0.85 - 1.75) (default)\n2: CM = 1.35V (Input range 0.90 - 1.80)\n3: CM = 1.40V (input range 0.95 - 1.85)"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_cm(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable input shifter\n1: Enable input shifter (900mV - 1800mV shifted to 0mV - 900mV)"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: Adds 20uA constant load current at the ADC LDO to minimize ripple on the reference voltage of the ADC."]
    #[inline(always)]
    pub fn gp_adc_i20u(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: No attenuator (input voltages up to 0.9V allowed)\n1: Enabling 2x attenuator (input voltages up to 1.8V allowed)\n2: Enabling 3x attenuator (input voltages up to 2.7V allowed)\n3: Enabling 4x attenuator (input voltages up to 3.6V allowed)\nEnabling the attenuator requires a longer sampling time."]
    #[inline(always)]
    pub fn gp_adc_attn(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcCtrl2Reg {
    #[inline(always)]
    fn default() -> GpAdcCtrl2Reg {
        <crate::RegValueT<GpAdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(528)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for GpAdcCtrl3Reg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Third Control Register"]
pub type GpAdcCtrl3Reg = crate::RegValueT<GpAdcCtrl3Reg_SPEC>;

impl GpAdcCtrl3Reg {
    #[doc = "Defines the interval between two ADC conversions in case GP_ADC_CONT is set.\n0: No extra delay between two conversions.\n1: 1.024 ms interval between two conversions.\n2: 2.048 ms interval between two conversions.\n255: 261.12 ms interval between two conversions."]
    #[inline(always)]
    pub fn gp_adc_interval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, GpAdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, GpAdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the delay for enabling the ADC after enabling the LDO.\n0: Not allowed\n1: 4x ADC_CLK period.\nn: n*4x ADC_CLK period."]
    #[inline(always)]
    pub fn gp_adc_en_del(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GpAdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, GpAdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcCtrl3Reg {
    #[inline(always)]
    fn default() -> GpAdcCtrl3Reg {
        <crate::RegValueT<GpAdcCtrl3Reg_SPEC> as RegisterValue<_>>::new(64)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcCtrlReg_SPEC;
impl crate::sealed::RegSpec for GpAdcCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Control Register"]
pub type GpAdcCtrlReg = crate::RegValueT<GpAdcCtrlReg_SPEC>;

impl GpAdcCtrlReg {
    #[doc = "Enables the die-temperature sensor. Output can be measured on GPADC input 4."]
    #[inline(always)]
    pub fn die_temp_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn gp_adc_offs_sh_gain_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: GPADC LDO tracking bandgap reference\n1: GPADC LDO hold sampled bandgap reference"]
    #[inline(always)]
    pub fn gp_adc_ldo_hold(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Chopper mode off\n1: Chopper mode enabled. Takes two samples with opposite GP_ADC_SIGN to cancel the internal offset voltage of the ADC; Highly recommended for DC-measurements."]
    #[inline(always)]
    pub fn gp_adc_chop(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Default\n1: Conversion with opposite sign at input and output to cancel out the internal offset of the ADC and low-frequency"]
    #[inline(always)]
    pub fn gp_adc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation\n1: Mute ADC input. Takes sample at mid-scale (to dertermine the internal offset and/or noise of the ADC with regards to VDD_REF which is also sampled by the ADC)."]
    #[inline(always)]
    pub fn gp_adc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Differential mode\n1: Single ended mode"]
    #[inline(always)]
    pub fn gp_adc_se(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable (mask) GP_ADC_INT.\n1: Enable GP_ADC_INT to ICU."]
    #[inline(always)]
    pub fn gp_adc_mint(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1: AD conversion ready and has generated an interrupt. Must be cleared by writing any value to GP_ADC_CLEAR_INT_REG."]
    #[inline(always)]
    pub fn gp_adc_int(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, GpAdcCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,GpAdcCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: DMA functionality disabled\n1: DMA functionality enabled"]
    #[inline(always)]
    pub fn gp_adc_dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Manual ADC mode, a single result will be generated after setting the GP_ADC_START bit.\n1: Continuous ADC mode, new ADC results will be constantly stored in GP_ADC_RESULT_REG. Still GP_ADC_START has to be set to start the execution. The time between conversions is configurable with GP_ADC_INTERVAL."]
    #[inline(always)]
    pub fn gp_adc_cont(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: ADC conversion ready.\n1: If a 1 is written, the ADC starts a conversion. After the conversion this bit will be set to 0 and the GP_ADC_INT bit will be set. It is not allowed to write this bit while it is not (yet) zero."]
    #[inline(always)]
    pub fn gp_adc_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: LDO is off and ADC is disabled..\n1: LDO is turned on and afterwards the ADC is enabled."]
    #[inline(always)]
    pub fn gp_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcCtrlReg {
    #[inline(always)]
    fn default() -> GpAdcCtrlReg {
        <crate::RegValueT<GpAdcCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcOffnReg_SPEC;
impl crate::sealed::RegSpec for GpAdcOffnReg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Negative Offset Register"]
pub type GpAdcOffnReg = crate::RegValueT<GpAdcOffnReg_SPEC>;

impl GpAdcOffnReg {
    #[doc = "Offset adjust of \'negative\' array of ADC-network (effective if \"GP_ADC_SE=0\", or \"GP_ADC_SE=1 AND GP_ADC_SIGN=1 OR GP_ADC_CHOP=1\")"]
    #[inline(always)]
    pub fn gp_adc_offn(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, GpAdcOffnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, GpAdcOffnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcOffnReg {
    #[inline(always)]
    fn default() -> GpAdcOffnReg {
        <crate::RegValueT<GpAdcOffnReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcOffpReg_SPEC;
impl crate::sealed::RegSpec for GpAdcOffpReg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Positive Offset Register"]
pub type GpAdcOffpReg = crate::RegValueT<GpAdcOffpReg_SPEC>;

impl GpAdcOffpReg {
    #[doc = "Offset adjust of \'positive\' array of ADC-network (effective if \"GP_ADC_SE=0\", or \"GP_ADC_SE=1 AND GP_ADC_SIGN=0 OR GP_ADC_CHOP=1\")"]
    #[inline(always)]
    pub fn gp_adc_offp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, GpAdcOffpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, GpAdcOffpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcOffpReg {
    #[inline(always)]
    fn default() -> GpAdcOffpReg {
        <crate::RegValueT<GpAdcOffpReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcParamDifReg_SPEC;
impl crate::sealed::RegSpec for GpAdcParamDifReg_SPEC {
    type DataType = u16;
}
#[doc = ""]
pub type GpAdcParamDifReg = crate::RegValueT<GpAdcParamDifReg_SPEC>;

impl NoBitfieldReg<GpAdcParamDifReg_SPEC> for GpAdcParamDifReg {}
impl ::core::default::Default for GpAdcParamDifReg {
    #[inline(always)]
    fn default() -> GpAdcParamDifReg {
        <crate::RegValueT<GpAdcParamDifReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcParamSeReg_SPEC;
impl crate::sealed::RegSpec for GpAdcParamSeReg_SPEC {
    type DataType = u16;
}
#[doc = ""]
pub type GpAdcParamSeReg = crate::RegValueT<GpAdcParamSeReg_SPEC>;

impl NoBitfieldReg<GpAdcParamSeReg_SPEC> for GpAdcParamSeReg {}
impl ::core::default::Default for GpAdcParamSeReg {
    #[inline(always)]
    fn default() -> GpAdcParamSeReg {
        <crate::RegValueT<GpAdcParamSeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcResultReg_SPEC;
impl crate::sealed::RegSpec for GpAdcResultReg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Result Register"]
pub type GpAdcResultReg = crate::RegValueT<GpAdcResultReg_SPEC>;

impl GpAdcResultReg {
    #[doc = "Returns the 10 up to 16 bits linear value of the last AD conversion. The upper 10 bits are always valid, the lower 6 bits are only valid in case oversampling has been applied. Two samples results in one extra bit and 64 samples results in six extra bits."]
    #[inline(always)]
    pub fn gp_adc_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GpAdcResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, GpAdcResultReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcResultReg {
    #[inline(always)]
    fn default() -> GpAdcResultReg {
        <crate::RegValueT<GpAdcResultReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcTrimReg_SPEC;
impl crate::sealed::RegSpec for GpAdcTrimReg_SPEC {
    type DataType = u16;
}
#[doc = "General Purpose ADC Trim Register"]
pub type GpAdcTrimReg = crate::RegValueT<GpAdcTrimReg_SPEC>;

impl GpAdcTrimReg {
    #[doc = "GPADC LDO level\n0: 825mV\n1: 850mV\n2: 875mV\n3: 900mV (reset)\n4: 925mV (default)\n5: 950mV\n6: 975mV\n7:1000mV"]
    #[inline(always)]
    pub fn gp_adc_ldo_level(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, GpAdcTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, GpAdcTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Offset Shifter common-mode reference fine trimming: 2mV/LSB\nDefault = mid-scale at 1000"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_vref(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, GpAdcTrimReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, GpAdcTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcTrimReg {
    #[inline(always)]
    fn default() -> GpAdcTrimReg {
        <crate::RegValueT<GpAdcTrimReg_SPEC> as RegisterValue<_>>::new(56)
    }
}
