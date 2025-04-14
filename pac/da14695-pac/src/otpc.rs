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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:21 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"OTPC registers"]
unsafe impl ::core::marker::Send for super::Otpc {}
unsafe impl ::core::marker::Sync for super::Otpc {}
impl super::Otpc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn otpc_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_paddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_pword_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPwordReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPwordReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_tim1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcTim1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcTim1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_tim2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcTim2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcTim2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcModeReg_SPEC;
impl crate::sealed::RegSpec for OtpcModeReg_SPEC {
    type DataType = u32;
}

pub type OtpcModeReg = crate::RegValueT<OtpcModeReg_SPEC>;

impl OtpcModeReg {
    #[inline(always)]
    pub fn otpc_mode_prg_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, OtpcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_mode_ht_marg_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_mode_use_tst_row(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_mode_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, OtpcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcModeReg {
    #[inline(always)]
    fn default() -> OtpcModeReg {
        <crate::RegValueT<OtpcModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPaddrReg_SPEC;
impl crate::sealed::RegSpec for OtpcPaddrReg_SPEC {
    type DataType = u32;
}

pub type OtpcPaddrReg = crate::RegValueT<OtpcPaddrReg_SPEC>;

impl OtpcPaddrReg {
    #[inline(always)]
    pub fn otpc_paddr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OtpcPaddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OtpcPaddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcPaddrReg {
    #[inline(always)]
    fn default() -> OtpcPaddrReg {
        <crate::RegValueT<OtpcPaddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPwordReg_SPEC;
impl crate::sealed::RegSpec for OtpcPwordReg_SPEC {
    type DataType = u32;
}

pub type OtpcPwordReg = crate::RegValueT<OtpcPwordReg_SPEC>;

impl OtpcPwordReg {
    #[inline(always)]
    pub fn otpc_pword(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcPwordReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcPwordReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcPwordReg {
    #[inline(always)]
    fn default() -> OtpcPwordReg {
        <crate::RegValueT<OtpcPwordReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcStatReg_SPEC;
impl crate::sealed::RegSpec for OtpcStatReg_SPEC {
    type DataType = u32;
}

pub type OtpcStatReg = crate::RegValueT<OtpcStatReg_SPEC>;

impl OtpcStatReg {
    #[inline(always)]
    pub fn otpc_stat_mrdy(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_pbuf_empty(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_prdy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcStatReg {
    #[inline(always)]
    fn default() -> OtpcStatReg {
        <crate::RegValueT<OtpcStatReg_SPEC> as RegisterValue<_>>::new(7)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcTim1Reg_SPEC;
impl crate::sealed::RegSpec for OtpcTim1Reg_SPEC {
    type DataType = u32;
}

pub type OtpcTim1Reg = crate::RegValueT<OtpcTim1Reg_SPEC>;

impl OtpcTim1Reg {
    #[inline(always)]
    pub fn otpc_tim1_us_t_csp(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7f,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim1_us_t_cs(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim1_us_t_pl(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim1_cc_t_rd(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim1_cc_t_20ns(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim1_cc_t_1us(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcTim1Reg {
    #[inline(always)]
    fn default() -> OtpcTim1Reg {
        <crate::RegValueT<OtpcTim1Reg_SPEC> as RegisterValue<_>>::new(161026079)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcTim2Reg_SPEC;
impl crate::sealed::RegSpec for OtpcTim2Reg_SPEC {
    type DataType = u32;
}

pub type OtpcTim2Reg = crate::RegValueT<OtpcTim2Reg_SPEC>;

impl OtpcTim2Reg {
    #[inline(always)]
    pub fn otpc_tim2_us_add_cc_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, OtpcTim2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim2_us_t_sas(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x3,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim2_us_t_pph(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim2_us_t_vds(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x7,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim2_us_t_pps(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim2_us_t_ppr(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim2_us_t_pwi(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_tim2_us_t_pw(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcTim2Reg {
    #[inline(always)]
    fn default() -> OtpcTim2Reg {
        <crate::RegValueT<OtpcTim2Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}
