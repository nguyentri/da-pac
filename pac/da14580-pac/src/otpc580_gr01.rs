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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"otpc580_gr01 registers"]
unsafe impl ::core::marker::Send for super::Otpc580Gr01 {}
unsafe impl ::core::marker::Sync for super::Otpc580Gr01 {}
impl super::Otpc580Gr01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn otpc_ahbadr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcAhbadrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcAhbadrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_celadr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcCeladrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcCeladrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_ffprt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcFfprtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcFfprtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_ffrd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcFfrdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcFfrdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
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
    pub const fn otpc_nwords_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcNwordsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcNwordsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_pctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPctrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPctrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn otpc_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcAhbadrReg_SPEC;
impl crate::sealed::RegSpec for OtpcAhbadrReg_SPEC {
    type DataType = u32;
}

pub type OtpcAhbadrReg = crate::RegValueT<OtpcAhbadrReg_SPEC>;

impl OtpcAhbadrReg {
    #[inline(always)]
    pub fn otpc_ahbadr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3fffffff,
        1,
        0,
        u32,
        u32,
        OtpcAhbadrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3fffffff,
            1,
            0,
            u32,
            u32,
            OtpcAhbadrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcAhbadrReg {
    #[inline(always)]
    fn default() -> OtpcAhbadrReg {
        <crate::RegValueT<OtpcAhbadrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcCeladrReg_SPEC;
impl crate::sealed::RegSpec for OtpcCeladrReg_SPEC {
    type DataType = u32;
}

pub type OtpcCeladrReg = crate::RegValueT<OtpcCeladrReg_SPEC>;

impl OtpcCeladrReg {
    #[inline(always)]
    pub fn otpc_celadr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fff,
        1,
        0,
        u16,
        u16,
        OtpcCeladrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            OtpcCeladrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcCeladrReg {
    #[inline(always)]
    fn default() -> OtpcCeladrReg {
        <crate::RegValueT<OtpcCeladrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcFfprtReg_SPEC;
impl crate::sealed::RegSpec for OtpcFfprtReg_SPEC {
    type DataType = u32;
}

pub type OtpcFfprtReg = crate::RegValueT<OtpcFfprtReg_SPEC>;

impl OtpcFfprtReg {
    #[inline(always)]
    pub fn otpc_ffprt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcFfprtReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcFfprtReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcFfprtReg {
    #[inline(always)]
    fn default() -> OtpcFfprtReg {
        <crate::RegValueT<OtpcFfprtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcFfrdReg_SPEC;
impl crate::sealed::RegSpec for OtpcFfrdReg_SPEC {
    type DataType = u32;
}

pub type OtpcFfrdReg = crate::RegValueT<OtpcFfrdReg_SPEC>;

impl OtpcFfrdReg {
    #[inline(always)]
    pub fn otpc_ffrd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcFfrdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcFfrdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcFfrdReg {
    #[inline(always)]
    fn default() -> OtpcFfrdReg {
        <crate::RegValueT<OtpcFfrdReg_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn otpc_mode_prg_port_mux(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, OtpcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn optc_mode_prg_fast(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_mode_prg_port_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_mode_two_cc_acc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_mode_fifo_flush(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_mode_use_dma(
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
pub struct OtpcNwordsReg_SPEC;
impl crate::sealed::RegSpec for OtpcNwordsReg_SPEC {
    type DataType = u32;
}

pub type OtpcNwordsReg = crate::RegValueT<OtpcNwordsReg_SPEC>;

impl OtpcNwordsReg {
    #[inline(always)]
    pub fn otpc_nwords(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fff,
        1,
        0,
        u16,
        u16,
        OtpcNwordsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            OtpcNwordsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcNwordsReg {
    #[inline(always)]
    fn default() -> OtpcNwordsReg {
        <crate::RegValueT<OtpcNwordsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPctrlReg_SPEC;
impl crate::sealed::RegSpec for OtpcPctrlReg_SPEC {
    type DataType = u32;
}

pub type OtpcPctrlReg = crate::RegValueT<OtpcPctrlReg_SPEC>;

impl OtpcPctrlReg {
    #[inline(always)]
    pub fn otpc_pctrl_enu(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_bitu(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_enl(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_bitl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_bselu(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_badru(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, OtpcPctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_bsell(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_badrl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, OtpcPctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_pctrl_waddr(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, OtpcPctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            OtpcPctrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcPctrlReg {
    #[inline(always)]
    fn default() -> OtpcPctrlReg {
        <crate::RegValueT<OtpcPctrlReg_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn otpc_stat_nwords(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, u16, OtpcStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16,u16,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_terr_u(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_terr_l(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_perr_u(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_perr_l(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_fwords(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, OtpcStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_ardy(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_terror(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_trdy(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_stat_perror(
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
        <crate::RegValueT<OtpcStatReg_SPEC> as RegisterValue<_>>::new(21)
    }
}
