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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:15 +0000

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
    pub const fn wkup_clear_p0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupClearP0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupClearP0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_clear_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupClearP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupClearP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
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
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_pol_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_reset_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupResetIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupResetIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_select_p0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectP0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectP0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_select_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_sel_gpio_p0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelGpioP0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelGpioP0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_sel_gpio_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelGpioP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelGpioP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_status_p0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupStatusP0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupStatusP0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wkup_status_p1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupStatusP1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupStatusP1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupClearP0Reg_SPEC;
impl crate::sealed::RegSpec for WkupClearP0Reg_SPEC {
    type DataType = u32;
}

pub type WkupClearP0Reg = crate::RegValueT<WkupClearP0Reg_SPEC>;

impl WkupClearP0Reg {
    #[inline(always)]
    pub fn wkup_clear_p0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        WkupClearP0Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            WkupClearP0Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupClearP0Reg {
    #[inline(always)]
    fn default() -> WkupClearP0Reg {
        <crate::RegValueT<WkupClearP0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupClearP1Reg_SPEC;
impl crate::sealed::RegSpec for WkupClearP1Reg_SPEC {
    type DataType = u32;
}

pub type WkupClearP1Reg = crate::RegValueT<WkupClearP1Reg_SPEC>;

impl WkupClearP1Reg {
    #[inline(always)]
    pub fn wkup_clear_p1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        WkupClearP1Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            WkupClearP1Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupClearP1Reg {
    #[inline(always)]
    fn default() -> WkupClearP1Reg {
        <crate::RegValueT<WkupClearP1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupCtrlReg_SPEC;
impl crate::sealed::RegSpec for WkupCtrlReg_SPEC {
    type DataType = u32;
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
    type DataType = u32;
}

pub type WkupPolP0Reg = crate::RegValueT<WkupPolP0Reg_SPEC>;

impl WkupPolP0Reg {
    #[inline(always)]
    pub fn wkup_pol_p0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        WkupPolP0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            WkupPolP0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    type DataType = u32;
}

pub type WkupPolP1Reg = crate::RegValueT<WkupPolP1Reg_SPEC>;

impl WkupPolP1Reg {
    #[inline(always)]
    pub fn wkup_pol_p1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        WkupPolP1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            WkupPolP1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
pub struct WkupResetIrqReg_SPEC;
impl crate::sealed::RegSpec for WkupResetIrqReg_SPEC {
    type DataType = u32;
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
    type DataType = u32;
}

pub type WkupSelectP0Reg = crate::RegValueT<WkupSelectP0Reg_SPEC>;

impl WkupSelectP0Reg {
    #[inline(always)]
    pub fn wkup_select_p0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        WkupSelectP0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            WkupSelectP0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    type DataType = u32;
}

pub type WkupSelectP1Reg = crate::RegValueT<WkupSelectP1Reg_SPEC>;

impl WkupSelectP1Reg {
    #[inline(always)]
    pub fn wkup_select_p1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        WkupSelectP1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            WkupSelectP1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
pub struct WkupSelGpioP0Reg_SPEC;
impl crate::sealed::RegSpec for WkupSelGpioP0Reg_SPEC {
    type DataType = u32;
}

pub type WkupSelGpioP0Reg = crate::RegValueT<WkupSelGpioP0Reg_SPEC>;

impl WkupSelGpioP0Reg {
    #[inline(always)]
    pub fn wkup_sel_gpio_p0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        WkupSelGpioP0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
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
    type DataType = u32;
}

pub type WkupSelGpioP1Reg = crate::RegValueT<WkupSelGpioP1Reg_SPEC>;

impl WkupSelGpioP1Reg {
    #[inline(always)]
    pub fn wkup_sel_gpio_p1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        WkupSelGpioP1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
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
pub struct WkupStatusP0Reg_SPEC;
impl crate::sealed::RegSpec for WkupStatusP0Reg_SPEC {
    type DataType = u32;
}

pub type WkupStatusP0Reg = crate::RegValueT<WkupStatusP0Reg_SPEC>;

impl WkupStatusP0Reg {
    #[inline(always)]
    pub fn wkup_stat_p0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        WkupStatusP0Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            WkupStatusP0Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupStatusP0Reg {
    #[inline(always)]
    fn default() -> WkupStatusP0Reg {
        <crate::RegValueT<WkupStatusP0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupStatusP1Reg_SPEC;
impl crate::sealed::RegSpec for WkupStatusP1Reg_SPEC {
    type DataType = u32;
}

pub type WkupStatusP1Reg = crate::RegValueT<WkupStatusP1Reg_SPEC>;

impl WkupStatusP1Reg {
    #[inline(always)]
    pub fn wkup_stat_p1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        WkupStatusP1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            WkupStatusP1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupStatusP1Reg {
    #[inline(always)]
    fn default() -> WkupStatusP1Reg {
        <crate::RegValueT<WkupStatusP1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
