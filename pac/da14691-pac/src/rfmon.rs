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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"RFMON registers"]
unsafe impl ::core::marker::Send for super::Rfmon {}
unsafe impl ::core::marker::Sync for super::Rfmon {}
impl super::Rfmon {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "AHB master start address"]
    #[inline(always)]
    pub const fn rfmon_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfmonAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfmonAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "AHB master current address"]
    #[inline(always)]
    pub const fn rfmon_crv_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfmonCrvAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfmonCrvAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "The remaining data to be transferred"]
    #[inline(always)]
    pub const fn rfmon_crv_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfmonCrvLenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfmonCrvLenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Control register"]
    #[inline(always)]
    pub const fn rfmon_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfmonCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfmonCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Data length register"]
    #[inline(always)]
    pub const fn rfmon_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfmonLenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfmonLenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Status register"]
    #[inline(always)]
    pub const fn rfmon_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfmonStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfmonStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfmonAddrReg_SPEC;
impl crate::sealed::RegSpec for RfmonAddrReg_SPEC {
    type DataType = u32;
}

#[doc = "AHB master start address"]
pub type RfmonAddrReg = crate::RegValueT<RfmonAddrReg_SPEC>;

impl RfmonAddrReg {
    #[doc = "It is the bits \\[31:2\\] of base address that is used by the AHB master interface of the controller. Defines the AHB address where the controller will start storing data at. Bits \\[1:0\\] of the address are always considered to be 0."]
    #[inline(always)]
    pub fn rfmon_addr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3fffffff,
        1,
        0,
        u32,
        u32,
        RfmonAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3fffffff,
            1,
            0,
            u32,
            u32,
            RfmonAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfmonAddrReg {
    #[inline(always)]
    fn default() -> RfmonAddrReg {
        <crate::RegValueT<RfmonAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfmonCrvAddrReg_SPEC;
impl crate::sealed::RegSpec for RfmonCrvAddrReg_SPEC {
    type DataType = u32;
}

#[doc = "AHB master current address"]
pub type RfmonCrvAddrReg = crate::RegValueT<RfmonCrvAddrReg_SPEC>;

impl RfmonCrvAddrReg {
    #[doc = "Bits \\[31:2\\] of AHB address that will be used by the controller in the next memory access. The bits \\[1:0\\] are always 0."]
    #[inline(always)]
    pub fn rfmon_crv_addr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3fffffff,
        1,
        0,
        u32,
        u32,
        RfmonCrvAddrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x3fffffff,
            1,
            0,
            u32,
            u32,
            RfmonCrvAddrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfmonCrvAddrReg {
    #[inline(always)]
    fn default() -> RfmonCrvAddrReg {
        <crate::RegValueT<RfmonCrvAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfmonCrvLenReg_SPEC;
impl crate::sealed::RegSpec for RfmonCrvLenReg_SPEC {
    type DataType = u32;
}

#[doc = "The remaining data to be transferred"]
pub type RfmonCrvLenReg = crate::RegValueT<RfmonCrvLenReg_SPEC>;

impl RfmonCrvLenReg {
    #[doc = "Indicates the number of words (minus 1) that remain to be transfered."]
    #[inline(always)]
    pub fn rfmon_crv_len(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ffff,
        1,
        0,
        u32,
        u32,
        RfmonCrvLenReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            RfmonCrvLenReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfmonCrvLenReg {
    #[inline(always)]
    fn default() -> RfmonCrvLenReg {
        <crate::RegValueT<RfmonCrvLenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfmonCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfmonCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Control register"]
pub type RfmonCtrlReg = crate::RegValueT<RfmonCtrlReg_SPEC>;

impl RfmonCtrlReg {
    #[doc = "Write this bit with 1, when the required throughput for the transferring of the captured data is close to the capacity of the system bus/memory. The controller will be aggressive in the usage of the bus. The availability of the bus will be affected for the remaining masters."]
    #[inline(always)]
    pub fn rfmon_breq_force(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RfmonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RfmonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write with 1 to enable the circular mode. In circular mode the controller continuously writes data in to the memory until being disabled by software. Data are transferred in the circular buffer in the memory, as defined by RFMON_ADDR_REG and RFMON_LEN_REG registers. Disabling of the controller is realized by writing RFMON_PACK_EN with 0."]
    #[inline(always)]
    pub fn rfmon_circ_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfmonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfmonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Starts capturing data from the test bus\n0 : No data captured.\n1 : Data captured.\nShould be written with 1 to start data acquisition.\nWhen the controller is not in circular mode (RFMON_CIRC_EN = 0) and after capturing a predefined number of words (RFMON_LEN), this bit will be auto cleared.\nIn circular mode (RFMON_CIRC_EN = 1) the RFMON_PACK_EN remains 1 to be cleared by software."]
    #[inline(always)]
    pub fn rfmon_pack_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfmonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfmonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfmonCtrlReg {
    #[inline(always)]
    fn default() -> RfmonCtrlReg {
        <crate::RegValueT<RfmonCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfmonLenReg_SPEC;
impl crate::sealed::RegSpec for RfmonLenReg_SPEC {
    type DataType = u32;
}

#[doc = "Data length register"]
pub type RfmonLenReg = crate::RegValueT<RfmonLenReg_SPEC>;

impl RfmonLenReg {
    #[doc = "The number of words (minus one) that should be captured."]
    #[inline(always)]
    pub fn rfmon_len(
        self,
    ) -> crate::common::RegisterField<0, 0x1ffff, 1, 0, u32, u32, RfmonLenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1ffff,
            1,
            0,
            u32,
            u32,
            RfmonLenReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RfmonLenReg {
    #[inline(always)]
    fn default() -> RfmonLenReg {
        <crate::RegValueT<RfmonLenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfmonStatReg_SPEC;
impl crate::sealed::RegSpec for RfmonStatReg_SPEC {
    type DataType = u32;
}

#[doc = "Status register"]
pub type RfmonStatReg = crate::RegValueT<RfmonStatReg_SPEC>;

impl RfmonStatReg {
    #[doc = "Indicates that during transfer of data, at least one overflow has been detected.\n0 : The transfer completed without overflows.\n1 : At least one overflow occured in the fifo.\nWrite 1 to clear this bit."]
    #[inline(always)]
    pub fn rfmon_oflow_stk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfmonStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfmonStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the state of the controller.\n0 : The controller is idle.\n1 : The controller is active. The capturing process and/or the dma activity is in progress.\nThe controller will be activated (RFMON_ACTIVE == 1), when RFMON_PACK_EN will be written with 1. Will return to inactive state, after the end of the capturing process (RFMON_PACK_EN==0) and the completion of the transfer of all data to memory."]
    #[inline(always)]
    pub fn rfmon_active(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfmonStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RfmonStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfmonStatReg {
    #[inline(always)]
    fn default() -> RfmonStatReg {
        <crate::RegValueT<RfmonStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
