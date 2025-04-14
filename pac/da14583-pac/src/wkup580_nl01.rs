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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:36 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"wkup580_nl01 registers"]
unsafe impl ::core::marker::Send for super::Wkup580Nl01 {}
unsafe impl ::core::marker::Sync for super::Wkup580Nl01 {}
impl super::Wkup580Nl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn wkup_compare_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupCompareReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupCompareReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_counter_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupCounterReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupCounterReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_reset_cntr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupResetCntrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupResetCntrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_reset_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupResetIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupResetIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_select_p0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectP0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectP0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_select_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_select_p2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectP2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectP2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_select_p3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectP3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectP3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupCompareReg_SPEC;
impl crate::sealed::RegSpec for WkupCompareReg_SPEC {
    type DataType = u16;
}

pub type WkupCompareReg = crate::RegValueT<WkupCompareReg_SPEC>;

impl WkupCompareReg {
    #[inline(always)]
    pub fn compare(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupCompareReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupCompareReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupCompareReg {
    #[inline(always)]
    fn default() -> WkupCompareReg {
        <crate::RegValueT<WkupCompareReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupCounterReg_SPEC;
impl crate::sealed::RegSpec for WkupCounterReg_SPEC {
    type DataType = u16;
}

pub type WkupCounterReg = crate::RegValueT<WkupCounterReg_SPEC>;

impl WkupCounterReg {
    #[inline(always)]
    pub fn event_value(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupCounterReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupCounterReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupCounterReg {
    #[inline(always)]
    fn default() -> WkupCounterReg {
        <crate::RegValueT<WkupCounterReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupCtrlReg_SPEC;
impl crate::sealed::RegSpec for WkupCtrlReg_SPEC {
    type DataType = u16;
}

pub type WkupCtrlReg = crate::RegValueT<WkupCtrlReg_SPEC>;

impl WkupCtrlReg {
    #[inline(always)]
    pub fn wkup_enable_irq(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, WkupCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,WkupCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wkup_sft_keyhit(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, WkupCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,WkupCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wkup_deb_value(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, WkupCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,WkupCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupCtrlReg {
    #[inline(always)]
    fn default() -> WkupCtrlReg {
        <crate::RegValueT<WkupCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupPolP0Reg_SPEC;
impl crate::sealed::RegSpec for WkupPolP0Reg_SPEC {
    type DataType = u16;
}

pub type WkupPolP0Reg = crate::RegValueT<WkupPolP0Reg_SPEC>;

impl WkupPolP0Reg {
    #[inline(always)]
    pub fn wkup_pol_p0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupPolP0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupPolP0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupPolP0Reg {
    #[inline(always)]
    fn default() -> WkupPolP0Reg {
        <crate::RegValueT<WkupPolP0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupPolP1Reg_SPEC;
impl crate::sealed::RegSpec for WkupPolP1Reg_SPEC {
    type DataType = u16;
}

pub type WkupPolP1Reg = crate::RegValueT<WkupPolP1Reg_SPEC>;

impl WkupPolP1Reg {
    #[inline(always)]
    pub fn wkup_pol_p1(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, WkupPolP1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,WkupPolP1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupPolP1Reg {
    #[inline(always)]
    fn default() -> WkupPolP1Reg {
        <crate::RegValueT<WkupPolP1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupPolP2Reg_SPEC;
impl crate::sealed::RegSpec for WkupPolP2Reg_SPEC {
    type DataType = u16;
}

pub type WkupPolP2Reg = crate::RegValueT<WkupPolP2Reg_SPEC>;

impl WkupPolP2Reg {
    #[inline(always)]
    pub fn wkup_pol_p2(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, WkupPolP2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,WkupPolP2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupPolP2Reg {
    #[inline(always)]
    fn default() -> WkupPolP2Reg {
        <crate::RegValueT<WkupPolP2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupPolP3Reg_SPEC;
impl crate::sealed::RegSpec for WkupPolP3Reg_SPEC {
    type DataType = u16;
}

pub type WkupPolP3Reg = crate::RegValueT<WkupPolP3Reg_SPEC>;

impl WkupPolP3Reg {
    #[inline(always)]
    pub fn wkup_pol_p3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupPolP3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupPolP3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupPolP3Reg {
    #[inline(always)]
    fn default() -> WkupPolP3Reg {
        <crate::RegValueT<WkupPolP3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupResetCntrReg_SPEC;
impl crate::sealed::RegSpec for WkupResetCntrReg_SPEC {
    type DataType = u16;
}

pub type WkupResetCntrReg = crate::RegValueT<WkupResetCntrReg_SPEC>;

impl WkupResetCntrReg {
    #[inline(always)]
    pub fn wkup_cntr_rst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        WkupResetCntrReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            WkupResetCntrReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupResetCntrReg {
    #[inline(always)]
    fn default() -> WkupResetCntrReg {
        <crate::RegValueT<WkupResetCntrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupResetIrqReg_SPEC;
impl crate::sealed::RegSpec for WkupResetIrqReg_SPEC {
    type DataType = u16;
}

pub type WkupResetIrqReg = crate::RegValueT<WkupResetIrqReg_SPEC>;

impl WkupResetIrqReg {
    #[inline(always)]
    pub fn wkup_irq_rst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        WkupResetIrqReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            WkupResetIrqReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupResetIrqReg {
    #[inline(always)]
    fn default() -> WkupResetIrqReg {
        <crate::RegValueT<WkupResetIrqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelectP0Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelectP0Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelectP0Reg = crate::RegValueT<WkupSelectP0Reg_SPEC>;

impl WkupSelectP0Reg {
    #[inline(always)]
    pub fn wkup_select_p0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelectP0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupSelectP0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupSelectP0Reg {
    #[inline(always)]
    fn default() -> WkupSelectP0Reg {
        <crate::RegValueT<WkupSelectP0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelectP1Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelectP1Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelectP1Reg = crate::RegValueT<WkupSelectP1Reg_SPEC>;

impl WkupSelectP1Reg {
    #[inline(always)]
    pub fn wkup_select_p1(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, WkupSelectP1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,WkupSelectP1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupSelectP1Reg {
    #[inline(always)]
    fn default() -> WkupSelectP1Reg {
        <crate::RegValueT<WkupSelectP1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelectP2Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelectP2Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelectP2Reg = crate::RegValueT<WkupSelectP2Reg_SPEC>;

impl WkupSelectP2Reg {
    #[inline(always)]
    pub fn wkup_select_p2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        WkupSelectP2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            WkupSelectP2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupSelectP2Reg {
    #[inline(always)]
    fn default() -> WkupSelectP2Reg {
        <crate::RegValueT<WkupSelectP2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelectP3Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelectP3Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelectP3Reg = crate::RegValueT<WkupSelectP3Reg_SPEC>;

impl WkupSelectP3Reg {
    #[inline(always)]
    pub fn wkup_select_p3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelectP3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupSelectP3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupSelectP3Reg {
    #[inline(always)]
    fn default() -> WkupSelectP3Reg {
        <crate::RegValueT<WkupSelectP3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
