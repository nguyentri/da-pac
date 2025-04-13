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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"TRNG registers"]
unsafe impl ::core::marker::Send for super::Trng {}
unsafe impl ::core::marker::Sync for super::Trng {}
impl super::Trng {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "TRNG control register"]
    #[inline(always)]
    pub const fn trng_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TrngCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TrngCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "TRNG FIFO level register"]
    #[inline(always)]
    pub const fn trng_fifolvl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TrngFifolvlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TrngFifolvlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "TRNG Version register"]
    #[inline(always)]
    pub const fn trng_ver_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TrngVerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TrngVerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngCtrlReg_SPEC;
impl crate::sealed::RegSpec for TrngCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "TRNG control register"]
pub type TrngCtrlReg = crate::RegValueT<TrngCtrlReg_SPEC>;

impl TrngCtrlReg {
    #[doc = "0: select the TRNG with asynchronous free running oscillators (default)\n1: select the pseudo-random generator with synchronous oscillators (for simulation purpose only)"]
    #[inline(always)]
    pub fn trng_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TrngCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,TrngCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable the TRNG\n1: Enable the TRNG this signal is ignored when the FIFO is full"]
    #[inline(always)]
    pub fn trng_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TrngCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TrngCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TrngCtrlReg {
    #[inline(always)]
    fn default() -> TrngCtrlReg {
        <crate::RegValueT<TrngCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngFifolvlReg_SPEC;
impl crate::sealed::RegSpec for TrngFifolvlReg_SPEC {
    type DataType = u32;
}
#[doc = "TRNG FIFO level register"]
pub type TrngFifolvlReg = crate::RegValueT<TrngFifolvlReg_SPEC>;

impl TrngFifolvlReg {
    #[doc = "1:FIFO full indication. This bit is cleared if the FIFO is read."]
    #[inline(always)]
    pub fn trng_fifofull(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, TrngFifolvlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,TrngFifolvlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Number of 32 bit words of random data in the FIFO (max 31) until the FIFO is full. When it is 0 and TRNG_FIFOFULL is 1, it means the FIFO is full."]
    #[inline(always)]
    pub fn trng_fifolvl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TrngFifolvlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, TrngFifolvlReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for TrngFifolvlReg {
    #[inline(always)]
    fn default() -> TrngFifolvlReg {
        <crate::RegValueT<TrngFifolvlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngVerReg_SPEC;
impl crate::sealed::RegSpec for TrngVerReg_SPEC {
    type DataType = u32;
}
#[doc = "TRNG Version register"]
pub type TrngVerReg = crate::RegValueT<TrngVerReg_SPEC>;

impl TrngVerReg {
    #[doc = "Major version number"]
    #[inline(always)]
    pub fn trng_maj(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, TrngVerReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, TrngVerReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Minor version number"]
    #[inline(always)]
    pub fn trng_min(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, TrngVerReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, TrngVerReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SVN revision number"]
    #[inline(always)]
    pub fn trng_svn(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TrngVerReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, TrngVerReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for TrngVerReg {
    #[inline(always)]
    fn default() -> TrngVerReg {
        <crate::RegValueT<TrngVerReg_SPEC> as RegisterValue<_>>::new(259)
    }
}
