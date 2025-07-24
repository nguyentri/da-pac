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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:31 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DW registers"]
unsafe impl ::core::marker::Send for super::Dw {}
unsafe impl ::core::marker::Sync for super::Dw {}
impl super::Dw {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "USB Master clock tokens (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_cclm1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaCclm1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaCclm1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "GenDMA Master clock tokens (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_cclm2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaCclm2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaCclm2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "CRYPTO Master clock tokens (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_cclm3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaCclm3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaCclm3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "CRYPTO Master clock tokens (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_cclm4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaCclm4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaCclm4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Default master ID number (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_dflt_master_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaDfltMasterReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaDfltMasterReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "AHB-DMA layer priority level for RFTP (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_pl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaPl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaPl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "AHB-DMA layer priority level for LCD (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_pl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaPl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaPl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "AHB-DMA layer Priority level for GEN-DMA (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_pl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaPl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaPl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "AHB-DMA layer Priority level for CRYPTO-DMA (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_pl4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaPl4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaPl4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Master clock refresh period (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_tcl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaTclReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaTclReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Version ID (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_version_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaVersionReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaVersionReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Weighted-Token Arbitration Scheme Enable (AHB DMA layer only)"]
    #[inline(always)]
    pub const fn ahb_dma_wten_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AhbDmaWtenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AhbDmaWtenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaCclm1Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaCclm1Reg_SPEC {
    type DataType = u32;
}

#[doc = "USB Master clock tokens (AHB DMA layer only)"]
pub type AhbDmaCclm1Reg = crate::RegValueT<AhbDmaCclm1Reg_SPEC>;

impl AhbDmaCclm1Reg {
    #[doc = "Number of tokens (counted in AHB clock cycles) that a master can use on the bus before it has to arbitrate on a bus master with low priority and having tokens. Masters with tokens remaining have priority over masters that have used all of their tokens. User should configure all the token values ensuring that the sum does not exceeds the total allocated number of tokens. If a value of zero is configured, then the bus is deemed to have infinite tokens and will always operate in the upper-tier of arbitration."]
    #[inline(always)]
    pub fn ahb_dma_cclm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        AhbDmaCclm1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AhbDmaCclm1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AhbDmaCclm1Reg {
    #[inline(always)]
    fn default() -> AhbDmaCclm1Reg {
        <crate::RegValueT<AhbDmaCclm1Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaCclm2Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaCclm2Reg_SPEC {
    type DataType = u32;
}

#[doc = "GenDMA Master clock tokens (AHB DMA layer only)"]
pub type AhbDmaCclm2Reg = crate::RegValueT<AhbDmaCclm2Reg_SPEC>;

impl AhbDmaCclm2Reg {
    #[doc = "Refer to AHB_DMA_CCLM1_REG"]
    #[inline(always)]
    pub fn ahb_dma_cclm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        AhbDmaCclm2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AhbDmaCclm2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AhbDmaCclm2Reg {
    #[inline(always)]
    fn default() -> AhbDmaCclm2Reg {
        <crate::RegValueT<AhbDmaCclm2Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaCclm3Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaCclm3Reg_SPEC {
    type DataType = u32;
}

#[doc = "CRYPTO Master clock tokens (AHB DMA layer only)"]
pub type AhbDmaCclm3Reg = crate::RegValueT<AhbDmaCclm3Reg_SPEC>;

impl AhbDmaCclm3Reg {
    #[doc = "AHB_DMA_CCLM1_REG"]
    #[inline(always)]
    pub fn ahb_dma_cclm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        AhbDmaCclm3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AhbDmaCclm3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AhbDmaCclm3Reg {
    #[inline(always)]
    fn default() -> AhbDmaCclm3Reg {
        <crate::RegValueT<AhbDmaCclm3Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaCclm4Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaCclm4Reg_SPEC {
    type DataType = u32;
}

#[doc = "CRYPTO Master clock tokens (AHB DMA layer only)"]
pub type AhbDmaCclm4Reg = crate::RegValueT<AhbDmaCclm4Reg_SPEC>;

impl AhbDmaCclm4Reg {
    #[doc = "AHB_DMA_CCLM1_REG"]
    #[inline(always)]
    pub fn ahb_dma_cclm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        AhbDmaCclm4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AhbDmaCclm4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AhbDmaCclm4Reg {
    #[inline(always)]
    fn default() -> AhbDmaCclm4Reg {
        <crate::RegValueT<AhbDmaCclm4Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaDfltMasterReg_SPEC;
impl crate::sealed::RegSpec for AhbDmaDfltMasterReg_SPEC {
    type DataType = u32;
}

#[doc = "Default master ID number (AHB DMA layer only)"]
pub type AhbDmaDfltMasterReg = crate::RegValueT<AhbDmaDfltMasterReg_SPEC>;

impl AhbDmaDfltMasterReg {
    #[doc = "Default master ID number register. The default master is the master that is granted by the bus when no master has requested ownership.\n0: Dummy master\n1: RFPT\n2: LCD\n3: GEN-DMA\n3: CRYPTO-DMA"]
    #[inline(always)]
    pub fn ahb_dma_dflt_master(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        AhbDmaDfltMasterReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            AhbDmaDfltMasterReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AhbDmaDfltMasterReg {
    #[inline(always)]
    fn default() -> AhbDmaDfltMasterReg {
        <crate::RegValueT<AhbDmaDfltMasterReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaPl1Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaPl1Reg_SPEC {
    type DataType = u32;
}

#[doc = "AHB-DMA layer priority level for RFTP (AHB DMA layer only)"]
pub type AhbDmaPl1Reg = crate::RegValueT<AhbDmaPl1Reg_SPEC>;

impl AhbDmaPl1Reg {
    #[doc = "Arbitration priority for master RFPT. 0: lowest, 15: highest."]
    #[inline(always)]
    pub fn ahb_dma_pl1(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AhbDmaPl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AhbDmaPl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AhbDmaPl1Reg {
    #[inline(always)]
    fn default() -> AhbDmaPl1Reg {
        <crate::RegValueT<AhbDmaPl1Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaPl2Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaPl2Reg_SPEC {
    type DataType = u32;
}

#[doc = "AHB-DMA layer priority level for LCD (AHB DMA layer only)"]
pub type AhbDmaPl2Reg = crate::RegValueT<AhbDmaPl2Reg_SPEC>;

impl AhbDmaPl2Reg {
    #[doc = "Arbitration priority for master LCD. 0: lowest, 15: highest."]
    #[inline(always)]
    pub fn ahb_dma_pl2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AhbDmaPl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AhbDmaPl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AhbDmaPl2Reg {
    #[inline(always)]
    fn default() -> AhbDmaPl2Reg {
        <crate::RegValueT<AhbDmaPl2Reg_SPEC> as RegisterValue<_>>::new(14)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaPl3Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaPl3Reg_SPEC {
    type DataType = u32;
}

#[doc = "AHB-DMA layer Priority level for GEN-DMA (AHB DMA layer only)"]
pub type AhbDmaPl3Reg = crate::RegValueT<AhbDmaPl3Reg_SPEC>;

impl AhbDmaPl3Reg {
    #[doc = "Arbitration priority for master GEN-DMA. 0: lowest, 15: highest."]
    #[inline(always)]
    pub fn ahb_dma_pl3(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AhbDmaPl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AhbDmaPl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AhbDmaPl3Reg {
    #[inline(always)]
    fn default() -> AhbDmaPl3Reg {
        <crate::RegValueT<AhbDmaPl3Reg_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaPl4Reg_SPEC;
impl crate::sealed::RegSpec for AhbDmaPl4Reg_SPEC {
    type DataType = u32;
}

#[doc = "AHB-DMA layer Priority level for CRYPTO-DMA (AHB DMA layer only)"]
pub type AhbDmaPl4Reg = crate::RegValueT<AhbDmaPl4Reg_SPEC>;

impl AhbDmaPl4Reg {
    #[doc = "Arbitration priority for master CRYPTO-DMA. 0: lowest, 15: highest."]
    #[inline(always)]
    pub fn ahb_dma_pl4(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AhbDmaPl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AhbDmaPl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AhbDmaPl4Reg {
    #[inline(always)]
    fn default() -> AhbDmaPl4Reg {
        <crate::RegValueT<AhbDmaPl4Reg_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaTclReg_SPEC;
impl crate::sealed::RegSpec for AhbDmaTclReg_SPEC {
    type DataType = u32;
}

#[doc = "Master clock refresh period (AHB DMA layer only)"]
pub type AhbDmaTclReg = crate::RegValueT<AhbDmaTclReg_SPEC>;

impl AhbDmaTclReg {
    #[doc = "Master clock refresh period, counting clock cycles. An arbitration period is defined over this number of tokens. When a new arbitration period starts, the master counters are reloaded. Recommended value is the sum of the AHB_DMA_CCLMx_REG valuesplus 2 tokens for each master, i.e. plus 6."]
    #[inline(always)]
    pub fn ahb_dma_tcl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, AhbDmaTclReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AhbDmaTclReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AhbDmaTclReg {
    #[inline(always)]
    fn default() -> AhbDmaTclReg {
        <crate::RegValueT<AhbDmaTclReg_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaVersionReg_SPEC;
impl crate::sealed::RegSpec for AhbDmaVersionReg_SPEC {
    type DataType = u32;
}

#[doc = "Version ID (AHB DMA layer only)"]
pub type AhbDmaVersionReg = crate::RegValueT<AhbDmaVersionReg_SPEC>;

impl AhbDmaVersionReg {
    #[inline(always)]
    pub fn ahb_dma_version(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        AhbDmaVersionReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            AhbDmaVersionReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AhbDmaVersionReg {
    #[inline(always)]
    fn default() -> AhbDmaVersionReg {
        <crate::RegValueT<AhbDmaVersionReg_SPEC> as RegisterValue<_>>::new(842085162)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbDmaWtenReg_SPEC;
impl crate::sealed::RegSpec for AhbDmaWtenReg_SPEC {
    type DataType = u32;
}

#[doc = "Weighted-Token Arbitration Scheme Enable (AHB DMA layer only)"]
pub type AhbDmaWtenReg = crate::RegValueT<AhbDmaWtenReg_SPEC>;

impl AhbDmaWtenReg {
    #[doc = "Weighted-token arbitration scheme enable."]
    #[inline(always)]
    pub fn ahb_dma_wten(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AhbDmaWtenReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,AhbDmaWtenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AhbDmaWtenReg {
    #[inline(always)]
    fn default() -> AhbDmaWtenReg {
        <crate::RegValueT<AhbDmaWtenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
