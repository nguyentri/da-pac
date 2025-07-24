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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:49 +0000

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

    #[doc = "Patch entry 0: Address field"]
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

    #[doc = "Patch entry 1: Address field"]
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

    #[doc = "Patch entry 2: Address field"]
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

    #[doc = "Patch entry 3: Address field"]
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

    #[doc = "Patch entry 4: Address field"]
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

    #[doc = "Patch entry 5: Address field"]
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

    #[doc = "Patch entry 6: Address field"]
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

    #[doc = "Patch entry 7: Address field"]
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

    #[doc = "Patch entry 0: Data field"]
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

    #[doc = "Patch entry 1: Data field"]
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

    #[doc = "Patch entry 2: Data field"]
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

    #[doc = "Patch entry 3: Data field"]
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

    #[doc = "Patch entry 4: Data field"]
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

    #[doc = "Patch entry 5: Data field"]
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

    #[doc = "Patch entry 6: Data field"]
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

    #[doc = "Patch entry 7: Data field"]
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

    #[doc = "Validity Control Register"]
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

    #[doc = "Validity Reset Control Register"]
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

    #[doc = "Validity Set Control Register"]
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

#[doc = "Patch entry 0: Address field"]
pub type PatchAddr0Reg = crate::RegValueT<PatchAddr0Reg_SPEC>;

impl PatchAddr0Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 1: Address field"]
pub type PatchAddr1Reg = crate::RegValueT<PatchAddr1Reg_SPEC>;

impl PatchAddr1Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 2: Address field"]
pub type PatchAddr2Reg = crate::RegValueT<PatchAddr2Reg_SPEC>;

impl PatchAddr2Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 3: Address field"]
pub type PatchAddr3Reg = crate::RegValueT<PatchAddr3Reg_SPEC>;

impl PatchAddr3Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 4: Address field"]
pub type PatchAddr4Reg = crate::RegValueT<PatchAddr4Reg_SPEC>;

impl PatchAddr4Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 5: Address field"]
pub type PatchAddr5Reg = crate::RegValueT<PatchAddr5Reg_SPEC>;

impl PatchAddr5Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 6: Address field"]
pub type PatchAddr6Reg = crate::RegValueT<PatchAddr6Reg_SPEC>;

impl PatchAddr6Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 7: Address field"]
pub type PatchAddr7Reg = crate::RegValueT<PatchAddr7Reg_SPEC>;

impl PatchAddr7Reg {
    #[doc = "This is the value which will be compared to the address on the AHB. If a match occurs, the data bus willl be filled with the value in the respective PATCH_DATAx_REG. Bits \\[1:0\\] are read-only and always read as \'0\'.\nNever use the base address 0x0 for values in PATCH_ADDRx_REG because HW Patch block is located after the Address Remapping block."]
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

#[doc = "Patch entry 0: Data field"]
pub type PatchData0Reg = crate::RegValueT<PatchData0Reg_SPEC>;

impl PatchData0Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Patch entry 1: Data field"]
pub type PatchData1Reg = crate::RegValueT<PatchData1Reg_SPEC>;

impl PatchData1Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Patch entry 2: Data field"]
pub type PatchData2Reg = crate::RegValueT<PatchData2Reg_SPEC>;

impl PatchData2Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Patch entry 3: Data field"]
pub type PatchData3Reg = crate::RegValueT<PatchData3Reg_SPEC>;

impl PatchData3Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Patch entry 4: Data field"]
pub type PatchData4Reg = crate::RegValueT<PatchData4Reg_SPEC>;

impl PatchData4Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Patch entry 5: Data field"]
pub type PatchData5Reg = crate::RegValueT<PatchData5Reg_SPEC>;

impl PatchData5Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Patch entry 6: Data field"]
pub type PatchData6Reg = crate::RegValueT<PatchData6Reg_SPEC>;

impl PatchData6Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Patch entry 7: Data field"]
pub type PatchData7Reg = crate::RegValueT<PatchData7Reg_SPEC>;

impl PatchData7Reg {
    #[doc = "This is the value which will be injected into the data bus if there is a match on the comparison of the address with the respective PATCH_ADDRx_REG"]
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

#[doc = "Validity Control Register"]
pub type PatchValidReg = crate::RegValueT<PatchValidReg_SPEC>;

impl PatchValidReg {
    #[doc = "Indicates which patch entry is valid. For example, when bit 0 is high it indicates that entry 0 is valid, i.e. the values of PATCH_ADDR0_REG / PATCH_DATA0_REG, are effective."]
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

#[doc = "Validity Reset Control Register"]
pub type PatchValidResetReg = crate::RegValueT<PatchValidResetReg_SPEC>;

impl PatchValidResetReg {
    #[doc = "Writing a bit with 1 will clear the corresponding bit of PATCH_VALID_REG to 0. Writing a bit with zero is ignored. Read always as 0."]
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

#[doc = "Validity Set Control Register"]
pub type PatchValidSetReg = crate::RegValueT<PatchValidSetReg_SPEC>;

impl PatchValidSetReg {
    #[doc = "Writing a bit with 1 will set the corresponding bit of PATCH_VALID_REG to 1. Writing a bit with 0 is ignored.\nRead always as 0."]
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
