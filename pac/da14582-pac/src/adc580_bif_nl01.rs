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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:32 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"adc580_bif_nl01 registers"]
unsafe impl ::core::marker::Send for super::Adc580BifNl01 {}
unsafe impl ::core::marker::Sync for super::Adc580BifNl01 {}
impl super::Adc580BifNl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gp_adc_clear_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcClearIntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcClearIntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn gp_adc_delay2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcDelay2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcDelay2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_adc_delay_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcDelayReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcDelayReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_adc_offn_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcOffnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcOffnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_adc_offp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcOffpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcOffpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gp_adc_result_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpAdcResultReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpAdcResultReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
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

pub type GpAdcClearIntReg = crate::RegValueT<GpAdcClearIntReg_SPEC>;

impl GpAdcClearIntReg {
    #[inline(always)]
    pub fn gp_adc_clr_int(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        GpAdcClearIntReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            GpAdcClearIntReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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

pub type GpAdcCtrl2Reg = crate::RegValueT<GpAdcCtrl2Reg_SPEC>;

impl GpAdcCtrl2Reg {
    #[inline(always)]
    pub fn gp_adc_i20u(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_idyn(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_attn3x(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_delay_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpAdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpAdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcCtrl2Reg {
    #[inline(always)]
    fn default() -> GpAdcCtrl2Reg {
        <crate::RegValueT<GpAdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcCtrlReg_SPEC;
impl crate::sealed::RegSpec for GpAdcCtrlReg_SPEC {
    type DataType = u16;
}

pub type GpAdcCtrlReg = crate::RegValueT<GpAdcCtrlReg_SPEC>;

impl GpAdcCtrlReg {
    #[inline(always)]
    pub fn gp_adc_ldo_zero(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_ldo_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_chop(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_se(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_sel(
        self,
    ) -> crate::common::RegisterField<6, 0xf, 1, 0, u8, u8, GpAdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0xf,1,0,u8,u8,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_mint(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_int(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, GpAdcCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,GpAdcCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gp_adc_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,GpAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
pub struct GpAdcDelay2Reg_SPEC;
impl crate::sealed::RegSpec for GpAdcDelay2Reg_SPEC {
    type DataType = u16;
}

pub type GpAdcDelay2Reg = crate::RegValueT<GpAdcDelay2Reg_SPEC>;

impl GpAdcDelay2Reg {
    #[inline(always)]
    pub fn del_adc_start(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, GpAdcDelay2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,GpAdcDelay2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn del_adc_en(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GpAdcDelay2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GpAdcDelay2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcDelay2Reg {
    #[inline(always)]
    fn default() -> GpAdcDelay2Reg {
        <crate::RegValueT<GpAdcDelay2Reg_SPEC> as RegisterValue<_>>::new(34944)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcDelayReg_SPEC;
impl crate::sealed::RegSpec for GpAdcDelayReg_SPEC {
    type DataType = u16;
}

pub type GpAdcDelayReg = crate::RegValueT<GpAdcDelayReg_SPEC>;

impl GpAdcDelayReg {
    #[inline(always)]
    pub fn del_ldo_en(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GpAdcDelayReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,GpAdcDelayReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpAdcDelayReg {
    #[inline(always)]
    fn default() -> GpAdcDelayReg {
        <crate::RegValueT<GpAdcDelayReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpAdcOffnReg_SPEC;
impl crate::sealed::RegSpec for GpAdcOffnReg_SPEC {
    type DataType = u16;
}

pub type GpAdcOffnReg = crate::RegValueT<GpAdcOffnReg_SPEC>;

impl GpAdcOffnReg {
    #[inline(always)]
    pub fn gp_adc_offn(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GpAdcOffnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GpAdcOffnReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type GpAdcOffpReg = crate::RegValueT<GpAdcOffpReg_SPEC>;

impl GpAdcOffpReg {
    #[inline(always)]
    pub fn gp_adc_offp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GpAdcOffpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GpAdcOffpReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct GpAdcResultReg_SPEC;
impl crate::sealed::RegSpec for GpAdcResultReg_SPEC {
    type DataType = u16;
}

pub type GpAdcResultReg = crate::RegValueT<GpAdcResultReg_SPEC>;

impl GpAdcResultReg {
    #[inline(always)]
    pub fn gp_adc_val(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GpAdcResultReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            GpAdcResultReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GpAdcResultReg {
    #[inline(always)]
    fn default() -> GpAdcResultReg {
        <crate::RegValueT<GpAdcResultReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
