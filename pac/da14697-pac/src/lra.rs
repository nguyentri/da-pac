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
#[doc = r"LRA registers"]
unsafe impl ::core::marker::Send for super::Lra {}
unsafe impl ::core::marker::Sync for super::Lra {}
impl super::Lra {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

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

pub type LraAdcCtrl1Reg = crate::RegValueT<LraAdcCtrl1Reg_SPEC>;

impl LraAdcCtrl1Reg {
    #[inline(always)]
    pub fn lra_adc_busy(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,LraAdcCtrl1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_adc_offset(
        self,
    ) -> crate::common::RegisterField<9, 0xff, 1, 0, u8, u8, LraAdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xff,1,0,u8,u8,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_adc_test_param(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_adc_test_in_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_adc_freq(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, LraAdcCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_adc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lra_adc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraAdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LraAdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type LraAdcResultReg = crate::RegValueT<LraAdcResultReg_SPEC>;

impl LraAdcResultReg {
    #[inline(always)]
    pub fn man_flt_in(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraAdcResultReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraAdcResultReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gp_adc_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraAdcResultReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraAdcResultReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraBrdHsReg = crate::RegValueT<LraBrdHsReg_SPEC>;

impl LraBrdHsReg {
    #[inline(always)]
    pub fn trim_gain(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, u8, LraBrdHsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0xf,1,0,u8,u8,LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hsgnd_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, LraBrdHsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scp_hs_trim(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, LraBrdHsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scp_hs_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraBrdHsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn erc_hs_trim(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, LraBrdHsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,LraBrdHsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type LraBrdLsReg = crate::RegValueT<LraBrdLsReg_SPEC>;

impl LraBrdLsReg {
    #[inline(always)]
    pub fn scp_ls_trim_n(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, LraBrdLsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scp_ls_trim_p(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, LraBrdLsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scp_ls_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraBrdLsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn erc_ls_trim(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, LraBrdLsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,LraBrdLsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type LraBrdStatReg = crate::RegValueT<LraBrdStatReg_SPEC>;

impl LraBrdStatReg {
    #[inline(always)]
    pub fn scp_hs_out(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scp_ls_comp_out_n(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scp_ls_comp_out_p(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sc_event_ls(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sc_event_hs(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn loop_stat(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lsn_on(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lsp_on(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hsn_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hsp_on(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lsn_stat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lsp_stat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hsn_stat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraBrdStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,LraBrdStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type LraCtrl1Reg = crate::RegValueT<LraCtrl1Reg_SPEC>;

impl LraCtrl1Reg {
    #[inline(always)]
    pub fn smp_idx(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, LraCtrl1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,LraCtrl1Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_scp_event_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_ctrl_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_idx(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, LraCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_div(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, LraCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smp_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, LraCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pulldown_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn loop_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hbridge_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LraCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type LraCtrl2Reg = crate::RegValueT<LraCtrl2Reg_SPEC>;

impl LraCtrl2Reg {
    #[inline(always)]
    pub fn half_period(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, LraCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn auto_mode(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smp_mode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn polarity(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn flt_in_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LraCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pwm_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, LraCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,LraCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type LraDftReg = crate::RegValueT<LraDftReg_SPEC>;

impl LraDftReg {
    #[inline(always)]
    pub fn spare(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, LraDftReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swm_sel(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swm_man(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pwm_sel(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pwm_man(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer_trim(
        self,
    ) -> crate::common::RegisterField<23, 0x3, 1, 0, u8, u8, LraDftReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x3,1,0,u8,u8,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer_scale_trim(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, u8, LraDftReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x3,1,0,u8,u8,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dft_sel(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dft_force_hspn(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dft_en_timer(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, LraDftReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dft_stall(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, LraDftReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dft_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraDftReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,LraDftReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type LraFltCoef1Reg = crate::RegValueT<LraFltCoef1Reg_SPEC>;

impl LraFltCoef1Reg {
    #[inline(always)]
    pub fn flt_coef_01(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltCoef1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltCoef1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn flt_coef_00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltCoef1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltCoef1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type LraFltCoef2Reg = crate::RegValueT<LraFltCoef2Reg_SPEC>;

impl LraFltCoef2Reg {
    #[inline(always)]
    pub fn flt_coef_10(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltCoef2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltCoef2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn flt_coef_02(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltCoef2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltCoef2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type LraFltCoef3Reg = crate::RegValueT<LraFltCoef3Reg_SPEC>;

impl LraFltCoef3Reg {
    #[inline(always)]
    pub fn flt_coef_12(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltCoef3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltCoef3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn flt_coef_11(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltCoef3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltCoef3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type LraFltSmp1Reg = crate::RegValueT<LraFltSmp1Reg_SPEC>;

impl LraFltSmp1Reg {
    #[inline(always)]
    pub fn lra_smp_2(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraFltSmp2Reg = crate::RegValueT<LraFltSmp2Reg_SPEC>;

impl LraFltSmp2Reg {
    #[inline(always)]
    pub fn lra_smp_4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_3(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraFltSmp3Reg = crate::RegValueT<LraFltSmp3Reg_SPEC>;

impl LraFltSmp3Reg {
    #[inline(always)]
    pub fn lra_smp_6(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_5(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraFltSmp4Reg = crate::RegValueT<LraFltSmp4Reg_SPEC>;

impl LraFltSmp4Reg {
    #[inline(always)]
    pub fn lra_smp_8(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_7(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp4Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraFltSmp5Reg = crate::RegValueT<LraFltSmp5Reg_SPEC>;

impl LraFltSmp5Reg {
    #[inline(always)]
    pub fn lra_smp_10(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp5Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp5Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_9(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp5Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp5Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraFltSmp6Reg = crate::RegValueT<LraFltSmp6Reg_SPEC>;

impl LraFltSmp6Reg {
    #[inline(always)]
    pub fn lra_smp_12(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp6Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp6Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_11(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp6Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp6Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraFltSmp7Reg = crate::RegValueT<LraFltSmp7Reg_SPEC>;

impl LraFltSmp7Reg {
    #[inline(always)]
    pub fn lra_smp_14(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp7Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp7Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_13(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp7Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp7Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraFltSmp8Reg = crate::RegValueT<LraFltSmp8Reg_SPEC>;

impl LraFltSmp8Reg {
    #[inline(always)]
    pub fn lra_smp_16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LraFltSmp8Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp8Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lra_smp_15(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LraFltSmp8Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LraFltSmp8Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type LraLdoReg = crate::RegValueT<LraLdoReg_SPEC>;

impl LraLdoReg {
    #[inline(always)]
    pub fn ldo_ok(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LraLdoReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,LraLdoReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_tst(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LraLdoReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LraLdoReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
