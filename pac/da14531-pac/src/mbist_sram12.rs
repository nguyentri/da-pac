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
#[doc = r"MBIST_SRAM12 registers"]
unsafe impl ::core::marker::Send for super::MbistSram12 {}
unsafe impl ::core::marker::Sync for super::MbistSram12 {}
impl super::MbistSram12 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn mbist_sram12_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram12AddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram12AddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mbist_sram12_rd_lsb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram12RdLsbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram12RdLsbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mbist_sram12_rd_msb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram12RdMsbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram12RdMsbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mbist_sram12_state_reg(
        &self,
    ) -> &'static crate::common::Reg<self::MbistSram12StateReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MbistSram12StateReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram12AddrReg_SPEC;
impl crate::sealed::RegSpec for MbistSram12AddrReg_SPEC {
    type DataType = u16;
}

pub type MbistSram12AddrReg = crate::RegValueT<MbistSram12AddrReg_SPEC>;

impl MbistSram12AddrReg {
    #[doc = "Returns the current address register in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_addr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram12AddrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram12AddrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram12AddrReg {
    #[inline(always)]
    fn default() -> MbistSram12AddrReg {
        <crate::RegValueT<MbistSram12AddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram12RdLsbReg_SPEC;
impl crate::sealed::RegSpec for MbistSram12RdLsbReg_SPEC {
    type DataType = u16;
}

pub type MbistSram12RdLsbReg = crate::RegValueT<MbistSram12RdLsbReg_SPEC>;

impl MbistSram12RdLsbReg {
    #[doc = "Returns the actual LSB read data in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_lsb_data(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram12RdLsbReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram12RdLsbReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram12RdLsbReg {
    #[inline(always)]
    fn default() -> MbistSram12RdLsbReg {
        <crate::RegValueT<MbistSram12RdLsbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram12RdMsbReg_SPEC;
impl crate::sealed::RegSpec for MbistSram12RdMsbReg_SPEC {
    type DataType = u16;
}

pub type MbistSram12RdMsbReg = crate::RegValueT<MbistSram12RdMsbReg_SPEC>;

impl MbistSram12RdMsbReg {
    #[doc = "Returns the actual MSB read data in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_msb_data(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram12RdMsbReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram12RdMsbReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram12RdMsbReg {
    #[inline(always)]
    fn default() -> MbistSram12RdMsbReg {
        <crate::RegValueT<MbistSram12RdMsbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbistSram12StateReg_SPEC;
impl crate::sealed::RegSpec for MbistSram12StateReg_SPEC {
    type DataType = u16;
}

pub type MbistSram12StateReg = crate::RegValueT<MbistSram12StateReg_SPEC>;

impl MbistSram12StateReg {
    #[doc = "Returns the current state in case of a mismatch."]
    #[inline(always)]
    pub fn mbist_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbistSram12StateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,MbistSram12StateReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MbistSram12StateReg {
    #[inline(always)]
    fn default() -> MbistSram12StateReg {
        <crate::RegValueT<MbistSram12StateReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
