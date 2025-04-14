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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:40 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ambacore580_patch_gr00 registers"]
unsafe impl ::core::marker::Send for super::Ambacore580PatchGr00 {}
unsafe impl ::core::marker::Sync for super::Ambacore580PatchGr00 {}
impl super::Ambacore580PatchGr00 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn patch_addr0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_valid_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchValidReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchValidReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_valid_reset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchValidResetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchValidResetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_valid_set_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchValidSetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchValidSetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr0Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr0Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr0Reg = crate::RegValueT<PatchAddr0Reg_SPEC>;

impl PatchAddr0Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr0Reg {
    #[inline(always)]
    fn default() -> PatchAddr0Reg {
        <crate::RegValueT<PatchAddr0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr1Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr1Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr1Reg = crate::RegValueT<PatchAddr1Reg_SPEC>;

impl PatchAddr1Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr1Reg {
    #[inline(always)]
    fn default() -> PatchAddr1Reg {
        <crate::RegValueT<PatchAddr1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr2Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr2Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr2Reg = crate::RegValueT<PatchAddr2Reg_SPEC>;

impl PatchAddr2Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr2Reg {
    #[inline(always)]
    fn default() -> PatchAddr2Reg {
        <crate::RegValueT<PatchAddr2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr3Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr3Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr3Reg = crate::RegValueT<PatchAddr3Reg_SPEC>;

impl PatchAddr3Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr3Reg {
    #[inline(always)]
    fn default() -> PatchAddr3Reg {
        <crate::RegValueT<PatchAddr3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr4Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr4Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr4Reg = crate::RegValueT<PatchAddr4Reg_SPEC>;

impl PatchAddr4Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr4Reg {
    #[inline(always)]
    fn default() -> PatchAddr4Reg {
        <crate::RegValueT<PatchAddr4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr5Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr5Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr5Reg = crate::RegValueT<PatchAddr5Reg_SPEC>;

impl PatchAddr5Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr5Reg {
    #[inline(always)]
    fn default() -> PatchAddr5Reg {
        <crate::RegValueT<PatchAddr5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr6Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr6Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr6Reg = crate::RegValueT<PatchAddr6Reg_SPEC>;

impl PatchAddr6Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr6Reg {
    #[inline(always)]
    fn default() -> PatchAddr6Reg {
        <crate::RegValueT<PatchAddr6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr7Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr7Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr7Reg = crate::RegValueT<PatchAddr7Reg_SPEC>;

impl PatchAddr7Reg {
    #[inline(always)]
    pub fn patch_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchAddr7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchAddr7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr7Reg {
    #[inline(always)]
    fn default() -> PatchAddr7Reg {
        <crate::RegValueT<PatchAddr7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData0Reg_SPEC;
impl crate::sealed::RegSpec for PatchData0Reg_SPEC {
    type DataType = u32;
}

pub type PatchData0Reg = crate::RegValueT<PatchData0Reg_SPEC>;

impl PatchData0Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData0Reg {
    #[inline(always)]
    fn default() -> PatchData0Reg {
        <crate::RegValueT<PatchData0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData1Reg_SPEC;
impl crate::sealed::RegSpec for PatchData1Reg_SPEC {
    type DataType = u32;
}

pub type PatchData1Reg = crate::RegValueT<PatchData1Reg_SPEC>;

impl PatchData1Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData1Reg {
    #[inline(always)]
    fn default() -> PatchData1Reg {
        <crate::RegValueT<PatchData1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData2Reg_SPEC;
impl crate::sealed::RegSpec for PatchData2Reg_SPEC {
    type DataType = u32;
}

pub type PatchData2Reg = crate::RegValueT<PatchData2Reg_SPEC>;

impl PatchData2Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData2Reg {
    #[inline(always)]
    fn default() -> PatchData2Reg {
        <crate::RegValueT<PatchData2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData3Reg_SPEC;
impl crate::sealed::RegSpec for PatchData3Reg_SPEC {
    type DataType = u32;
}

pub type PatchData3Reg = crate::RegValueT<PatchData3Reg_SPEC>;

impl PatchData3Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData3Reg {
    #[inline(always)]
    fn default() -> PatchData3Reg {
        <crate::RegValueT<PatchData3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData4Reg_SPEC;
impl crate::sealed::RegSpec for PatchData4Reg_SPEC {
    type DataType = u32;
}

pub type PatchData4Reg = crate::RegValueT<PatchData4Reg_SPEC>;

impl PatchData4Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData4Reg {
    #[inline(always)]
    fn default() -> PatchData4Reg {
        <crate::RegValueT<PatchData4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData5Reg_SPEC;
impl crate::sealed::RegSpec for PatchData5Reg_SPEC {
    type DataType = u32;
}

pub type PatchData5Reg = crate::RegValueT<PatchData5Reg_SPEC>;

impl PatchData5Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData5Reg {
    #[inline(always)]
    fn default() -> PatchData5Reg {
        <crate::RegValueT<PatchData5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData6Reg_SPEC;
impl crate::sealed::RegSpec for PatchData6Reg_SPEC {
    type DataType = u32;
}

pub type PatchData6Reg = crate::RegValueT<PatchData6Reg_SPEC>;

impl PatchData6Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData6Reg {
    #[inline(always)]
    fn default() -> PatchData6Reg {
        <crate::RegValueT<PatchData6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData7Reg_SPEC;
impl crate::sealed::RegSpec for PatchData7Reg_SPEC {
    type DataType = u32;
}

pub type PatchData7Reg = crate::RegValueT<PatchData7Reg_SPEC>;

impl PatchData7Reg {
    #[inline(always)]
    pub fn patch_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        PatchData7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData7Reg {
    #[inline(always)]
    fn default() -> PatchData7Reg {
        <crate::RegValueT<PatchData7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchValidReg_SPEC;
impl crate::sealed::RegSpec for PatchValidReg_SPEC {
    type DataType = u32;
}

pub type PatchValidReg = crate::RegValueT<PatchValidReg_SPEC>;

impl PatchValidReg {
    #[inline(always)]
    pub fn patch_valid(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, PatchValidReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,PatchValidReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PatchValidReg {
    #[inline(always)]
    fn default() -> PatchValidReg {
        <crate::RegValueT<PatchValidReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchValidResetReg_SPEC;
impl crate::sealed::RegSpec for PatchValidResetReg_SPEC {
    type DataType = u32;
}

pub type PatchValidResetReg = crate::RegValueT<PatchValidResetReg_SPEC>;

impl PatchValidResetReg {
    #[inline(always)]
    pub fn patch_valid_reset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        PatchValidResetReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            PatchValidResetReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchValidResetReg {
    #[inline(always)]
    fn default() -> PatchValidResetReg {
        <crate::RegValueT<PatchValidResetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchValidSetReg_SPEC;
impl crate::sealed::RegSpec for PatchValidSetReg_SPEC {
    type DataType = u32;
}

pub type PatchValidSetReg = crate::RegValueT<PatchValidSetReg_SPEC>;

impl PatchValidSetReg {
    #[inline(always)]
    pub fn patch_valid_set(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, PatchValidSetReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            PatchValidSetReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchValidSetReg {
    #[inline(always)]
    fn default() -> PatchValidSetReg {
        <crate::RegValueT<PatchValidSetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
