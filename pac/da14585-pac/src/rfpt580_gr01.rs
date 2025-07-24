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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"rfpt580_gr01 registers"]
unsafe impl ::core::marker::Send for super::Rfpt580Gr01 {}
unsafe impl ::core::marker::Sync for super::Rfpt580Gr01 {}
impl super::Rfpt580Gr01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "AHB master start address"]
    #[inline(always)]
    pub const fn rfpt_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfptAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfptAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Control register"]
    #[inline(always)]
    pub const fn rfpt_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfptCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfptCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Data length register"]
    #[inline(always)]
    pub const fn rfpt_len_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfptLenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfptLenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Status register"]
    #[inline(always)]
    pub const fn rfpt_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RfptStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RfptStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfptAddrReg_SPEC;
impl crate::sealed::RegSpec for RfptAddrReg_SPEC {
    type DataType = u16;
}

#[doc = "AHB master start address"]
pub type RfptAddrReg = crate::RegValueT<RfptAddrReg_SPEC>;

impl RfptAddrReg {
    #[doc = "It is the AHB address used by the AHB-Lite master interface of the controller (the bits \\[15:2\\])."]
    #[inline(always)]
    pub fn rfpt_addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, u16, RfptAddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16,u16,RfptAddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfptAddrReg {
    #[inline(always)]
    fn default() -> RfptAddrReg {
        <crate::RegValueT<RfptAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfptCtrlReg_SPEC;
impl crate::sealed::RegSpec for RfptCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Control register"]
pub type RfptCtrlReg = crate::RegValueT<RfptCtrlReg_SPEC>;

impl RfptCtrlReg {
    #[doc = "Selects the source of data that will be captured.\n0 - Will capture the output of the two ADC.\n1 - Will capture the output of the Phase Detector."]
    #[inline(always)]
    pub fn rfpt_pack_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfptCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfptCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Starts the capturing of the data from the selected source (\nRFPT_PACK_SEL\n).\n0 - There is no capturing of data. The packer of data is idle.\n1 - The controller captures data. The packing of the data in words, is in progress.\nShould be written with 1 in order to start the acquisition of data. After the acquisition of a predefined number of words (\nRFPT_LEN\n), this bit will be auto cleared."]
    #[inline(always)]
    pub fn rfpt_pack_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfptCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RfptCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfptCtrlReg {
    #[inline(always)]
    fn default() -> RfptCtrlReg {
        <crate::RegValueT<RfptCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfptLenReg_SPEC;
impl crate::sealed::RegSpec for RfptLenReg_SPEC {
    type DataType = u16;
}

#[doc = "Data length register"]
pub type RfptLenReg = crate::RegValueT<RfptLenReg_SPEC>;

impl RfptLenReg {
    #[doc = "The number of words (minus one) that should be transfered.\nWhen the selected source of data is the two ADC (\nRFPT_PACK_SEL\n==0), will capture 2*(\nRFPT_LEN\n+ 1) samples from each ADC. Otherwise (\nRFPT_PACK_SEL\n==1), will capture 4*(\nRFPT_LEN\n+ 1) samples from the Phase Detector."]
    #[inline(always)]
    pub fn rfpt_len(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, RfptLenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,RfptLenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RfptLenReg {
    #[inline(always)]
    fn default() -> RfptLenReg {
        <crate::RegValueT<RfptLenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfptStatReg_SPEC;
impl crate::sealed::RegSpec for RfptStatReg_SPEC {
    type DataType = u16;
}

#[doc = "Status register"]
pub type RfptStatReg = crate::RegValueT<RfptStatReg_SPEC>;

impl RfptStatReg {
    #[doc = "Indicates that during the transfer of the data, at least one overflow has detected to the fifo.\n0 - The transfer completed without overflows.\n1 - At least one overflow occured in the fifo.\nWrite 1 to clear this bit."]
    #[inline(always)]
    pub fn rfpt_oflow_stk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfptStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfptStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the state of the controller.\n0 - The controller is idle.\n1 - The controller is active. The capturing process and/or the dma activity is in progress.\nThe controller will activated (RFPT_ACTIVE == 1), when RFPT_PACK_EN will be written with 1. Will return to inactive state, after the end of the capturing process (RFPT_PACK_EN==0) and the transfer of all of the data to the memory."]
    #[inline(always)]
    pub fn rfpt_active(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RfptStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RfptStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RfptStatReg {
    #[inline(always)]
    fn default() -> RfptStatReg {
        <crate::RegValueT<RfptStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
