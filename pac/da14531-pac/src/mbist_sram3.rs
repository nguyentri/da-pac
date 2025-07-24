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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"MBIST_SRAM3 registers"]
unsafe impl ::core::marker::Send for super::MbistSram3 {}
unsafe impl ::core::marker::Sync for super::MbistSram3 {}
impl super::MbistSram3 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn mbist_sram3_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram3AddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram3AddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mbist_sram3_rd_lsb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram3RdLsbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram3RdLsbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mbist_sram3_rd_msb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram3RdMsbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram3RdMsbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mbist_sram3_state_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram3StateReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram3StateReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram3AddrReg_SPEC;
impl crate::sealed::RegSpec for MbistSram3AddrReg_SPEC {
    type DataType = u16;
}

pub type MbistSram3AddrReg = crate::RegValueT<MbistSram3AddrReg_SPEC>;

impl MbistSram3AddrReg {
    #[doc = "Returns the current address register in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_addr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram3AddrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram3AddrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram3AddrReg {
    #[inline(always)]
    fn default() -> MbistSram3AddrReg {
        <crate::RegValueT<MbistSram3AddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram3RdLsbReg_SPEC;
impl crate::sealed::RegSpec for MbistSram3RdLsbReg_SPEC {
    type DataType = u16;
}

pub type MbistSram3RdLsbReg = crate::RegValueT<MbistSram3RdLsbReg_SPEC>;

impl MbistSram3RdLsbReg {
    #[doc = "Returns the actual LSB read data in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_lsb_data(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram3RdLsbReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram3RdLsbReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram3RdLsbReg {
    #[inline(always)]
    fn default() -> MbistSram3RdLsbReg {
        <crate::RegValueT<MbistSram3RdLsbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram3RdMsbReg_SPEC;
impl crate::sealed::RegSpec for MbistSram3RdMsbReg_SPEC {
    type DataType = u16;
}

pub type MbistSram3RdMsbReg = crate::RegValueT<MbistSram3RdMsbReg_SPEC>;

impl MbistSram3RdMsbReg {
    #[doc = "Returns the actual MSB read data in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_msb_data(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram3RdMsbReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram3RdMsbReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram3RdMsbReg {
    #[inline(always)]
    fn default() -> MbistSram3RdMsbReg {
        <crate::RegValueT<MbistSram3RdMsbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram3StateReg_SPEC;
impl crate::sealed::RegSpec for MbistSram3StateReg_SPEC {
    type DataType = u16;
}

pub type MbistSram3StateReg = crate::RegValueT<MbistSram3StateReg_SPEC>;

impl MbistSram3StateReg {
    #[doc = "Returns the current state in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram3StateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram3StateReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram3StateReg {
    #[inline(always)]
    fn default() -> MbistSram3StateReg {
        <crate::RegValueT<MbistSram3StateReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
