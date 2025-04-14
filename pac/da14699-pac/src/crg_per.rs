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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:34 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRG_PER registers"]
unsafe impl ::core::marker::Send for super::CrgPer {}
unsafe impl ::core::marker::Sync for super::CrgPer {}
impl super::CrgPer {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

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

    #[inline(always)]
    pub const fn pcm_div_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PcmDivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PcmDivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pcm_fdiv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PcmFdivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PcmFdivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pdm_div_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdmDivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdmDivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_clk_per_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetClkPerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetClkPerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn set_clk_per_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetClkPerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetClkPerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn src_div_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SrcDivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SrcDivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPerReg_SPEC;
impl crate::sealed::RegSpec for ClkPerReg_SPEC {
    type DataType = u32;
}

pub type ClkPerReg = crate::RegValueT<ClkPerReg_SPEC>;

impl ClkPerReg {
    #[inline(always)]
    pub fn mc_trig_div(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, ClkPerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mc_clk_div(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, ClkPerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mc_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpadc_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct PcmDivReg_SPEC;
impl crate::sealed::RegSpec for PcmDivReg_SPEC {
    type DataType = u32;
}

pub type PcmDivReg = crate::RegValueT<PcmDivReg_SPEC>;

impl PcmDivReg {
    #[inline(always)]
    pub fn pcm_src_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PcmDivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PcmDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_pcm_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PcmDivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PcmDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pcm_div(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, PcmDivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,PcmDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PcmDivReg {
    #[inline(always)]
    fn default() -> PcmDivReg {
        <crate::RegValueT<PcmDivReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcmFdivReg_SPEC;
impl crate::sealed::RegSpec for PcmFdivReg_SPEC {
    type DataType = u32;
}

pub type PcmFdivReg = crate::RegValueT<PcmFdivReg_SPEC>;

impl PcmFdivReg {
    #[inline(always)]
    pub fn pcm_fdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, PcmFdivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,PcmFdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PcmFdivReg {
    #[inline(always)]
    fn default() -> PcmFdivReg {
        <crate::RegValueT<PcmFdivReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdmDivReg_SPEC;
impl crate::sealed::RegSpec for PdmDivReg_SPEC {
    type DataType = u32;
}

pub type PdmDivReg = crate::RegValueT<PdmDivReg_SPEC>;

impl PdmDivReg {
    #[inline(always)]
    pub fn pdm_master_mode(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdmDivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdmDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_pdm_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdmDivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdmDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pdm_div(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, PdmDivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,PdmDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdmDivReg {
    #[inline(always)]
    fn default() -> PdmDivReg {
        <crate::RegValueT<PdmDivReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetClkPerReg_SPEC;
impl crate::sealed::RegSpec for ResetClkPerReg_SPEC {
    type DataType = u32;
}

pub type ResetClkPerReg = crate::RegValueT<ResetClkPerReg_SPEC>;

impl ResetClkPerReg {
    #[inline(always)]
    pub fn mc_trig_div(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, ResetClkPerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,ResetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mc_clk_div(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, ResetClkPerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,ResetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mc_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpadc_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetClkPerReg {
    #[inline(always)]
    fn default() -> ResetClkPerReg {
        <crate::RegValueT<ResetClkPerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetClkPerReg_SPEC;
impl crate::sealed::RegSpec for SetClkPerReg_SPEC {
    type DataType = u32;
}

pub type SetClkPerReg = crate::RegValueT<SetClkPerReg_SPEC>;

impl SetClkPerReg {
    #[inline(always)]
    pub fn mc_trig_div(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, SetClkPerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,SetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mc_clk_div(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, SetClkPerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,SetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mc_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpadc_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetClkPerReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetClkPerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetClkPerReg {
    #[inline(always)]
    fn default() -> SetClkPerReg {
        <crate::RegValueT<SetClkPerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrcDivReg_SPEC;
impl crate::sealed::RegSpec for SrcDivReg_SPEC {
    type DataType = u32;
}

pub type SrcDivReg = crate::RegValueT<SrcDivReg_SPEC>;

impl SrcDivReg {
    #[inline(always)]
    pub fn clk_src_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SrcDivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SrcDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn src_div(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SrcDivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,SrcDivReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SrcDivReg {
    #[inline(always)]
    fn default() -> SrcDivReg {
        <crate::RegValueT<SrcDivReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
