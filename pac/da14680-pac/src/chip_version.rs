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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:57 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CHIP_VERSION registers"]
unsafe impl ::core::marker::Send for super::ChipVersion {}
unsafe impl ::core::marker::Sync for super::ChipVersion {}
impl super::ChipVersion {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Chip identification register 1."]
    #[inline(always)]
    pub const fn chip_id1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChipId1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChipId1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Chip identification register 2."]
    #[inline(always)]
    pub const fn chip_id2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChipId2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChipId2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "Chip identification register 3."]
    #[inline(always)]
    pub const fn chip_id3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChipId3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChipId3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Chip revision register."]
    #[inline(always)]
    pub const fn chip_revision_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChipRevisionReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChipRevisionReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Software compatibility register."]
    #[inline(always)]
    pub const fn chip_swc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChipSwcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChipSwcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipId1Reg_SPEC;
impl crate::sealed::RegSpec for ChipId1Reg_SPEC {
    type DataType = u8;
}

#[doc = "Chip identification register 1."]
pub type ChipId1Reg = crate::RegValueT<ChipId1Reg_SPEC>;

impl ChipId1Reg {
    #[doc = "First character of device type \"680\" in ASCII."]
    #[inline(always)]
    pub fn chip_id1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ChipId1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ChipId1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChipId1Reg {
    #[inline(always)]
    fn default() -> ChipId1Reg {
        <crate::RegValueT<ChipId1Reg_SPEC> as RegisterValue<_>>::new(54)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipId2Reg_SPEC;
impl crate::sealed::RegSpec for ChipId2Reg_SPEC {
    type DataType = u8;
}

#[doc = "Chip identification register 2."]
pub type ChipId2Reg = crate::RegValueT<ChipId2Reg_SPEC>;

impl ChipId2Reg {
    #[doc = "Second character of device type \"680\" in ASCII."]
    #[inline(always)]
    pub fn chip_id2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ChipId2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ChipId2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChipId2Reg {
    #[inline(always)]
    fn default() -> ChipId2Reg {
        <crate::RegValueT<ChipId2Reg_SPEC> as RegisterValue<_>>::new(56)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipId3Reg_SPEC;
impl crate::sealed::RegSpec for ChipId3Reg_SPEC {
    type DataType = u8;
}

#[doc = "Chip identification register 3."]
pub type ChipId3Reg = crate::RegValueT<ChipId3Reg_SPEC>;

impl ChipId3Reg {
    #[doc = "Third character of device type \"680\" in ASCII."]
    #[inline(always)]
    pub fn chip_id3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ChipId3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ChipId3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChipId3Reg {
    #[inline(always)]
    fn default() -> ChipId3Reg {
        <crate::RegValueT<ChipId3Reg_SPEC> as RegisterValue<_>>::new(48)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipRevisionReg_SPEC;
impl crate::sealed::RegSpec for ChipRevisionReg_SPEC {
    type DataType = u8;
}

#[doc = "Chip revision register."]
pub type ChipRevisionReg = crate::RegValueT<ChipRevisionReg_SPEC>;

impl ChipRevisionReg {
    #[doc = "Chip version, corresponds with type number in ASCII.\n0x41 = \'A\', 0x42 = \'B\'"]
    #[inline(always)]
    pub fn revision_id(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ChipRevisionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ChipRevisionReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChipRevisionReg {
    #[inline(always)]
    fn default() -> ChipRevisionReg {
        <crate::RegValueT<ChipRevisionReg_SPEC> as RegisterValue<_>>::new(66)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipSwcReg_SPEC;
impl crate::sealed::RegSpec for ChipSwcReg_SPEC {
    type DataType = u8;
}

#[doc = "Software compatibility register."]
pub type ChipSwcReg = crate::RegValueT<ChipSwcReg_SPEC>;

impl ChipSwcReg {
    #[doc = "SoftWare Compatibility code.\nInteger (default = 0) which is incremented if a silicon change has impact on the CPU Firmware.\nCan be used by software developers to write silicon revision dependent code."]
    #[inline(always)]
    pub fn chip_swc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, ChipSwcReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,ChipSwcReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChipSwcReg {
    #[inline(always)]
    fn default() -> ChipSwcReg {
        <crate::RegValueT<ChipSwcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
