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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:28 +0000

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

pub type SdadcClearIntReg = crate::RegValueT<SdadcClearIntReg_SPEC>;

impl SdadcClearIntReg {
    #[inline(always)]
    pub fn sdadc_clr_int(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SdadcClearIntReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SdadcClearIntReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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

pub type SdadcCtrlReg = crate::RegValueT<SdadcCtrlReg_SPEC>;

impl SdadcCtrlReg {
    #[inline(always)]
    pub fn sdadc_dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_mint(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_int(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SdadcCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,SdadcCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_ldo_ok(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SdadcCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,SdadcCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_vref_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_cont(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_osr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, SdadcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_se(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_inn_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, SdadcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_inp_sel(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, u8, SdadcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,u8,u8,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdadc_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SdadcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SdadcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type SdadcGainCorrReg = crate::RegValueT<SdadcGainCorrReg_SPEC>;

impl SdadcGainCorrReg {
    #[inline(always)]
    pub fn sdadc_gain_corr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        SdadcGainCorrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            SdadcGainCorrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type SdadcOffsCorrReg = crate::RegValueT<SdadcOffsCorrReg_SPEC>;

impl SdadcOffsCorrReg {
    #[inline(always)]
    pub fn sdadc_offs_corr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        SdadcOffsCorrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            SdadcOffsCorrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type SdadcResultReg = crate::RegValueT<SdadcResultReg_SPEC>;

impl SdadcResultReg {
    #[inline(always)]
    pub fn sdadc_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SdadcResultReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SdadcResultReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SdadcResultReg {
    #[inline(always)]
    fn default() -> SdadcResultReg {
        <crate::RegValueT<SdadcResultReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
