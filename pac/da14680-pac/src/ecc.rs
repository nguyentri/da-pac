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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:50 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ECC registers"]
unsafe impl ::core::marker::Send for super::Ecc {}
unsafe impl ::core::marker::Sync for super::Ecc {}
impl super::Ecc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ecc_command_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccCommandReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccCommandReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecc_config_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccConfigReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccConfigReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecc_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecc_version_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccVersionReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccVersionReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccCommandReg_SPEC;
impl crate::sealed::RegSpec for EccCommandReg_SPEC {
    type DataType = u32;
}

pub type EccCommandReg = crate::RegValueT<EccCommandReg_SPEC>;

impl EccCommandReg {
    #[inline(always)]
    pub fn ecc_calcr2(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_signb(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_signa(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_sizeofoperands(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EccCommandReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_field(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_typeoperation(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, EccCommandReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccCommandReg {
    #[inline(always)]
    fn default() -> EccCommandReg {
        <crate::RegValueT<EccCommandReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccConfigReg_SPEC;
impl crate::sealed::RegSpec for EccConfigReg_SPEC {
    type DataType = u32;
}

pub type EccConfigReg = crate::RegValueT<EccConfigReg_SPEC>;

impl EccConfigReg {
    #[inline(always)]
    pub fn ecc_opptrc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, EccConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,EccConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_opptrb(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, EccConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,EccConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_opptra(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, EccConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,EccConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccConfigReg {
    #[inline(always)]
    fn default() -> EccConfigReg {
        <crate::RegValueT<EccConfigReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccControlReg_SPEC;
impl crate::sealed::RegSpec for EccControlReg_SPEC {
    type DataType = u32;
}

pub type EccControlReg = crate::RegValueT<EccControlReg_SPEC>;

impl EccControlReg {
    #[inline(always)]
    pub fn ecc_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EccControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EccControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccControlReg {
    #[inline(always)]
    fn default() -> EccControlReg {
        <crate::RegValueT<EccControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccStatusReg_SPEC;
impl crate::sealed::RegSpec for EccStatusReg_SPEC {
    type DataType = u32;
}

pub type EccStatusReg = crate::RegValueT<EccStatusReg_SPEC>;

impl EccStatusReg {
    #[inline(always)]
    pub fn ecc_busy(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_primalitytestresult(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_notinvertible(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_param_ab_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_signature_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_param_n_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_couple_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_point_px_atinfinity(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_point_px_notoncurve(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_fail_address(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, EccStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EccStatusReg {
    #[inline(always)]
    fn default() -> EccStatusReg {
        <crate::RegValueT<EccStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccVersionReg_SPEC;
impl crate::sealed::RegSpec for EccVersionReg_SPEC {
    type DataType = u32;
}

pub type EccVersionReg = crate::RegValueT<EccVersionReg_SPEC>;

impl EccVersionReg {
    #[inline(always)]
    pub fn ecc_hvn(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EccVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EccVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_svn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, EccVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,EccVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EccVersionReg {
    #[inline(always)]
    fn default() -> EccVersionReg {
        <crate::RegValueT<EccVersionReg_SPEC> as RegisterValue<_>>::new(1024)
    }
}
