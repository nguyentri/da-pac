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
#[doc = r"PATCH registers"]
unsafe impl ::core::marker::Send for super::Patch {}
unsafe impl ::core::marker::Sync for super::Patch {}
impl super::Patch {
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
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr16_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr16Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr16Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr17_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr17Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr17Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr18_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr18Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr18Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr19_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr19Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr19Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr20_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr20Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr20Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr21_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr21Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr21Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_addr9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchAddr9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchAddr9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data20_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData20Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData20Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[inline(always)]
    pub const fn patch_data21_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PatchData21Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PatchData21Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr0Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr10Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr10Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr10Reg = crate::RegValueT<PatchAddr10Reg_SPEC>;

impl PatchAddr10Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr10Reg {
    #[inline(always)]
    fn default() -> PatchAddr10Reg {
        <crate::RegValueT<PatchAddr10Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr11Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr11Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr11Reg = crate::RegValueT<PatchAddr11Reg_SPEC>;

impl PatchAddr11Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr11Reg {
    #[inline(always)]
    fn default() -> PatchAddr11Reg {
        <crate::RegValueT<PatchAddr11Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr12Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr12Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr12Reg = crate::RegValueT<PatchAddr12Reg_SPEC>;

impl PatchAddr12Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr12Reg {
    #[inline(always)]
    fn default() -> PatchAddr12Reg {
        <crate::RegValueT<PatchAddr12Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr13Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr13Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr13Reg = crate::RegValueT<PatchAddr13Reg_SPEC>;

impl PatchAddr13Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr13Reg {
    #[inline(always)]
    fn default() -> PatchAddr13Reg {
        <crate::RegValueT<PatchAddr13Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr14Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr14Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr14Reg = crate::RegValueT<PatchAddr14Reg_SPEC>;

impl PatchAddr14Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr14Reg {
    #[inline(always)]
    fn default() -> PatchAddr14Reg {
        <crate::RegValueT<PatchAddr14Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr15Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr15Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr15Reg = crate::RegValueT<PatchAddr15Reg_SPEC>;

impl PatchAddr15Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr15Reg {
    #[inline(always)]
    fn default() -> PatchAddr15Reg {
        <crate::RegValueT<PatchAddr15Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr16Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr16Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr16Reg = crate::RegValueT<PatchAddr16Reg_SPEC>;

impl PatchAddr16Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr16Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr16Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr16Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr16Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr16Reg {
    #[inline(always)]
    fn default() -> PatchAddr16Reg {
        <crate::RegValueT<PatchAddr16Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr17Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr17Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr17Reg = crate::RegValueT<PatchAddr17Reg_SPEC>;

impl PatchAddr17Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr17Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr17Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr17Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr17Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr17Reg {
    #[inline(always)]
    fn default() -> PatchAddr17Reg {
        <crate::RegValueT<PatchAddr17Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr18Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr18Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr18Reg = crate::RegValueT<PatchAddr18Reg_SPEC>;

impl PatchAddr18Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr18Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr18Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr18Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr18Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr18Reg {
    #[inline(always)]
    fn default() -> PatchAddr18Reg {
        <crate::RegValueT<PatchAddr18Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr19Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr19Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr19Reg = crate::RegValueT<PatchAddr19Reg_SPEC>;

impl PatchAddr19Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr19Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr19Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr19Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr19Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr19Reg {
    #[inline(always)]
    fn default() -> PatchAddr19Reg {
        <crate::RegValueT<PatchAddr19Reg_SPEC> as RegisterValue<_>>::new(133169152)
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr1Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr20Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr20Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr20Reg = crate::RegValueT<PatchAddr20Reg_SPEC>;

impl PatchAddr20Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr20Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr20Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_d(
        self,
    ) -> crate::common::RegisterField<
        2,
        0xffff,
        1,
        0,
        u16,
        u16,
        PatchAddr20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0xffff,
            1,
            0,
            u16,
            u16,
            PatchAddr20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr20Reg {
    #[inline(always)]
    fn default() -> PatchAddr20Reg {
        <crate::RegValueT<PatchAddr20Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr21Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr21Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr21Reg = crate::RegValueT<PatchAddr21Reg_SPEC>;

impl PatchAddr21Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr21Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr21Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_d(
        self,
    ) -> crate::common::RegisterField<
        2,
        0xffff,
        1,
        0,
        u16,
        u16,
        PatchAddr21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0xffff,
            1,
            0,
            u16,
            u16,
            PatchAddr21Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr21Reg {
    #[inline(always)]
    fn default() -> PatchAddr21Reg {
        <crate::RegValueT<PatchAddr21Reg_SPEC> as RegisterValue<_>>::new(133169152)
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr2Reg_SPEC> as RegisterValue<_>>::new(133169152)
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr3Reg_SPEC> as RegisterValue<_>>::new(133169152)
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr4Reg_SPEC> as RegisterValue<_>>::new(133169152)
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr5Reg_SPEC> as RegisterValue<_>>::new(133169152)
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr6Reg_SPEC> as RegisterValue<_>>::new(133169152)
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
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
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
        <crate::RegValueT<PatchAddr7Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr8Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr8Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr8Reg = crate::RegValueT<PatchAddr8Reg_SPEC>;

impl PatchAddr8Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr8Reg {
    #[inline(always)]
    fn default() -> PatchAddr8Reg {
        <crate::RegValueT<PatchAddr8Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchAddr9Reg_SPEC;
impl crate::sealed::RegSpec for PatchAddr9Reg_SPEC {
    type DataType = u32;
}

pub type PatchAddr9Reg = crate::RegValueT<PatchAddr9Reg_SPEC>;

impl PatchAddr9Reg {
    #[inline(always)]
    pub fn patch_addr_19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PatchAddr9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PatchAddr9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn patch_addr_c(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        PatchAddr9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            PatchAddr9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchAddr9Reg {
    #[inline(always)]
    fn default() -> PatchAddr9Reg {
        <crate::RegValueT<PatchAddr9Reg_SPEC> as RegisterValue<_>>::new(133169152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData20Reg_SPEC;
impl crate::sealed::RegSpec for PatchData20Reg_SPEC {
    type DataType = u32;
}

pub type PatchData20Reg = crate::RegValueT<PatchData20Reg_SPEC>;

impl PatchData20Reg {
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
        PatchData20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData20Reg {
    #[inline(always)]
    fn default() -> PatchData20Reg {
        <crate::RegValueT<PatchData20Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PatchData21Reg_SPEC;
impl crate::sealed::RegSpec for PatchData21Reg_SPEC {
    type DataType = u32;
}

pub type PatchData21Reg = crate::RegValueT<PatchData21Reg_SPEC>;

impl PatchData21Reg {
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
        PatchData21Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            PatchData21Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchData21Reg {
    #[inline(always)]
    fn default() -> PatchData21Reg {
        <crate::RegValueT<PatchData21Reg_SPEC> as RegisterValue<_>>::new(0)
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
    ) -> crate::common::RegisterField<
        0,
        0x3fffff,
        1,
        0,
        u32,
        u32,
        PatchValidReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fffff,
            1,
            0,
            u32,
            u32,
            PatchValidReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PatchValidReg {
    #[inline(always)]
    fn default() -> PatchValidReg {
        <crate::RegValueT<PatchValidReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
