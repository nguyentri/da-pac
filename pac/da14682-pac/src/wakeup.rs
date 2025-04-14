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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"WAKEUP registers"]
unsafe impl ::core::marker::Send for super::Wakeup {}
unsafe impl ::core::marker::Sync for super::Wakeup {}
impl super::Wakeup {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn wkup_clear_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupClear0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupClear0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_clear_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupClear1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupClear1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_clear_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupClear2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupClear2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
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
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
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

    #[inline(always)]
    pub const fn wkup_select_p4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectP4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectP4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_sel_gpio_p0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelGpioP0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelGpioP0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_sel_gpio_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelGpioP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelGpioP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_sel_gpio_p2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelGpioP2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelGpioP2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_sel_gpio_p3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelGpioP3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelGpioP3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_sel_gpio_p4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelGpioP4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelGpioP4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_status_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupStatus0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupStatus0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_status_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupStatus1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupStatus1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_status_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupStatus2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupStatus2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupClear0Reg_SPEC;
impl crate::sealed::RegSpec for WkupClear0Reg_SPEC {
    type DataType = u16;
}

pub type WkupClear0Reg = crate::RegValueT<WkupClear0Reg_SPEC>;

impl WkupClear0Reg {
    #[inline(always)]
    pub fn wkup_clear_p1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, WkupClear0Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,WkupClear0Reg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wkup_clear_p0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupClear0Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupClear0Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupClear0Reg {
    #[inline(always)]
    fn default() -> WkupClear0Reg {
        <crate::RegValueT<WkupClear0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupClear1Reg_SPEC;
impl crate::sealed::RegSpec for WkupClear1Reg_SPEC {
    type DataType = u16;
}

pub type WkupClear1Reg = crate::RegValueT<WkupClear1Reg_SPEC>;

impl WkupClear1Reg {
    #[inline(always)]
    pub fn wkup_clear_p2(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, WkupClear1Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,WkupClear1Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupClear1Reg {
    #[inline(always)]
    fn default() -> WkupClear1Reg {
        <crate::RegValueT<WkupClear1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupClear2Reg_SPEC;
impl crate::sealed::RegSpec for WkupClear2Reg_SPEC {
    type DataType = u16;
}

pub type WkupClear2Reg = crate::RegValueT<WkupClear2Reg_SPEC>;

impl WkupClear2Reg {
    #[inline(always)]
    pub fn wkup_clear_p4(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, WkupClear2Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,WkupClear2Reg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wkup_clear_p3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupClear2Reg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupClear2Reg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupClear2Reg {
    #[inline(always)]
    fn default() -> WkupClear2Reg {
        <crate::RegValueT<WkupClear2Reg_SPEC> as RegisterValue<_>>::new(0)
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
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupPolP1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupPolP1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, WkupPolP2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,WkupPolP2Reg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct WkupPolP4Reg_SPEC;
impl crate::sealed::RegSpec for WkupPolP4Reg_SPEC {
    type DataType = u16;
}

pub type WkupPolP4Reg = crate::RegValueT<WkupPolP4Reg_SPEC>;

impl WkupPolP4Reg {
    #[inline(always)]
    pub fn wkup_pol_p4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupPolP4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupPolP4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupPolP4Reg {
    #[inline(always)]
    fn default() -> WkupPolP4Reg {
        <crate::RegValueT<WkupPolP4Reg_SPEC> as RegisterValue<_>>::new(0)
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
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelectP1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupSelectP1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, WkupSelectP2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,WkupSelectP2Reg_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelectP4Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelectP4Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelectP4Reg = crate::RegValueT<WkupSelectP4Reg_SPEC>;

impl WkupSelectP4Reg {
    #[inline(always)]
    pub fn wkup_select_p4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelectP4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupSelectP4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupSelectP4Reg {
    #[inline(always)]
    fn default() -> WkupSelectP4Reg {
        <crate::RegValueT<WkupSelectP4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelGpioP0Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelGpioP0Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelGpioP0Reg = crate::RegValueT<WkupSelGpioP0Reg_SPEC>;

impl WkupSelGpioP0Reg {
    #[inline(always)]
    pub fn wkup_sel_gpio_p0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelGpioP0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            WkupSelGpioP0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupSelGpioP0Reg {
    #[inline(always)]
    fn default() -> WkupSelGpioP0Reg {
        <crate::RegValueT<WkupSelGpioP0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelGpioP1Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelGpioP1Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelGpioP1Reg = crate::RegValueT<WkupSelGpioP1Reg_SPEC>;

impl WkupSelGpioP1Reg {
    #[inline(always)]
    pub fn wkup_sel_gpio_p1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelGpioP1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            WkupSelGpioP1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupSelGpioP1Reg {
    #[inline(always)]
    fn default() -> WkupSelGpioP1Reg {
        <crate::RegValueT<WkupSelGpioP1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelGpioP2Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelGpioP2Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelGpioP2Reg = crate::RegValueT<WkupSelGpioP2Reg_SPEC>;

impl WkupSelGpioP2Reg {
    #[inline(always)]
    pub fn wkup_sel_gpio_p2(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, WkupSelGpioP2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            WkupSelGpioP2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupSelGpioP2Reg {
    #[inline(always)]
    fn default() -> WkupSelGpioP2Reg {
        <crate::RegValueT<WkupSelGpioP2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelGpioP3Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelGpioP3Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelGpioP3Reg = crate::RegValueT<WkupSelGpioP3Reg_SPEC>;

impl WkupSelGpioP3Reg {
    #[inline(always)]
    pub fn wkup_sel_gpio_p3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelGpioP3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            WkupSelGpioP3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupSelGpioP3Reg {
    #[inline(always)]
    fn default() -> WkupSelGpioP3Reg {
        <crate::RegValueT<WkupSelGpioP3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelGpioP4Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelGpioP4Reg_SPEC {
    type DataType = u16;
}

pub type WkupSelGpioP4Reg = crate::RegValueT<WkupSelGpioP4Reg_SPEC>;

impl WkupSelGpioP4Reg {
    #[inline(always)]
    pub fn wkup_sel_gpio_p4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupSelGpioP4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            WkupSelGpioP4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupSelGpioP4Reg {
    #[inline(always)]
    fn default() -> WkupSelGpioP4Reg {
        <crate::RegValueT<WkupSelGpioP4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupStatus0Reg_SPEC;
impl crate::sealed::RegSpec for WkupStatus0Reg_SPEC {
    type DataType = u16;
}

pub type WkupStatus0Reg = crate::RegValueT<WkupStatus0Reg_SPEC>;

impl WkupStatus0Reg {
    #[inline(always)]
    pub fn wkup_stat_p1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, WkupStatus0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,WkupStatus0Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wkup_stat_p0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupStatus0Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupStatus0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupStatus0Reg {
    #[inline(always)]
    fn default() -> WkupStatus0Reg {
        <crate::RegValueT<WkupStatus0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupStatus1Reg_SPEC;
impl crate::sealed::RegSpec for WkupStatus1Reg_SPEC {
    type DataType = u16;
}

pub type WkupStatus1Reg = crate::RegValueT<WkupStatus1Reg_SPEC>;

impl WkupStatus1Reg {
    #[inline(always)]
    pub fn wkup_stat_p2(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, WkupStatus1Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,WkupStatus1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupStatus1Reg {
    #[inline(always)]
    fn default() -> WkupStatus1Reg {
        <crate::RegValueT<WkupStatus1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupStatus2Reg_SPEC;
impl crate::sealed::RegSpec for WkupStatus2Reg_SPEC {
    type DataType = u16;
}

pub type WkupStatus2Reg = crate::RegValueT<WkupStatus2Reg_SPEC>;

impl WkupStatus2Reg {
    #[inline(always)]
    pub fn wkup_stat_p4(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, WkupStatus2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,WkupStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wkup_stat_p3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, WkupStatus2Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,WkupStatus2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupStatus2Reg {
    #[inline(always)]
    fn default() -> WkupStatus2Reg {
        <crate::RegValueT<WkupStatus2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
