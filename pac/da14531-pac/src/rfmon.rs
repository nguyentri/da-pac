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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:04 +0000

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
    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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
    type DataType = u16;
}
#[doc = ""]
pub type RfmonAddrReg = crate::RegValueT<RfmonAddrReg_SPEC>;

impl RfmonAddrReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rfmon_addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, RfmonAddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, RfmonAddrReg_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u16;
}
#[doc = ""]
pub type RfmonCrvAddrReg = crate::RegValueT<RfmonCrvAddrReg_SPEC>;

impl RfmonCrvAddrReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rfmon_crv_addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, RfmonCrvAddrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, RfmonCrvAddrReg_SPEC,crate::common::R>::from_register(self,0)
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
    type DataType = u16;
}
#[doc = ""]
pub type RfmonCrvLenReg = crate::RegValueT<RfmonCrvLenReg_SPEC>;

impl RfmonCrvLenReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rfmon_crv_len(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, RfmonCrvLenReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, RfmonCrvLenReg_SPEC,crate::common::R>::from_register(self,0)
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
    type DataType = u16;
}
#[doc = ""]
pub type RfmonCtrlReg = crate::RegValueT<RfmonCtrlReg_SPEC>;

impl RfmonCtrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rfmon_circ_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfmonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfmonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
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
    type DataType = u16;
}
#[doc = ""]
pub type RfmonLenReg = crate::RegValueT<RfmonLenReg_SPEC>;

impl RfmonLenReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rfmon_len(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, RfmonLenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, RfmonLenReg_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u16;
}
#[doc = ""]
pub type RfmonStatReg = crate::RegValueT<RfmonStatReg_SPEC>;

impl RfmonStatReg {
    #[doc = ""]
    #[inline(always)]
    pub fn rfmon_oflow_stk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RfmonStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RfmonStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
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
