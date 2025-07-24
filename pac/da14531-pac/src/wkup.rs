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
#[doc = r"WKUP registers"]
unsafe impl ::core::marker::Send for super::Wkup {}
unsafe impl ::core::marker::Sync for super::Wkup {}
impl super::Wkup {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Select the sensitivity polarity for each P1 input"]
    #[inline(always)]
    pub const fn wkup2_pol_gpio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Wkup2PolGpioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wkup2PolGpioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Select which inputs from P1 port can trigger wkup counter"]
    #[inline(always)]
    pub const fn wkup2_select_gpio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Wkup2SelectGpioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wkup2SelectGpioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Number of events before wakeup interrupt"]
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

    #[doc = "Actual number of events of the wakeup counter"]
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

    #[doc = "Control register for the wakeup counter"]
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

    #[doc = "Reset wakeup interrupt"]
    #[inline(always)]
    pub const fn wkup_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Select the sensitivity polarity for each P0 input"]
    #[inline(always)]
    pub const fn wkup_pol_gpio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupPolGpioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupPolGpioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Select which inputs from P0 port can trigger wkup counter"]
    #[inline(always)]
    pub const fn wkup_select_gpio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WkupSelectGpioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WkupSelectGpioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wkup2PolGpioReg_SPEC;
impl crate::sealed::RegSpec for Wkup2PolGpioReg_SPEC {
    type DataType = u16;
}

#[doc = "Select the sensitivity polarity for each P1 input"]
pub type Wkup2PolGpioReg = crate::RegValueT<Wkup2PolGpioReg_SPEC>;

impl Wkup2PolGpioReg {
    #[doc = "0 = the enabled input P0x increments the event2 counter if that input goes high\n1 = the enabled input P0x increments the event2 counter if that input goes low"]
    #[inline(always)]
    pub fn wkup2_pol_gpio(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        Wkup2PolGpioReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            Wkup2PolGpioReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wkup2PolGpioReg {
    #[inline(always)]
    fn default() -> Wkup2PolGpioReg {
        <crate::RegValueT<Wkup2PolGpioReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wkup2SelectGpioReg_SPEC;
impl crate::sealed::RegSpec for Wkup2SelectGpioReg_SPEC {
    type DataType = u16;
}

#[doc = "Select which inputs from P1 port can trigger wkup counter"]
pub type Wkup2SelectGpioReg = crate::RegValueT<Wkup2SelectGpioReg_SPEC>;

impl Wkup2SelectGpioReg {
    #[doc = "0 = input P0x is not enabled for wakeup event counter\n1 = input P0x is enabled for wakeup event counter"]
    #[inline(always)]
    pub fn wkup2_select_gpio(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        Wkup2SelectGpioReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            Wkup2SelectGpioReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wkup2SelectGpioReg {
    #[inline(always)]
    fn default() -> Wkup2SelectGpioReg {
        <crate::RegValueT<Wkup2SelectGpioReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupCompareReg_SPEC;
impl crate::sealed::RegSpec for WkupCompareReg_SPEC {
    type DataType = u16;
}

#[doc = "Number of events before wakeup interrupt"]
pub type WkupCompareReg = crate::RegValueT<WkupCompareReg_SPEC>;

impl WkupCompareReg {
    #[doc = "Defines the number of events -1 that have to be counted before the wakeup interrupt will be given. value 0 means one event."]
    #[inline(always)]
    pub fn wkup_compare(
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

#[doc = "Actual number of events of the wakeup counter"]
pub type WkupCounterReg = crate::RegValueT<WkupCounterReg_SPEC>;

impl WkupCounterReg {
    #[doc = "This value represents the number of events that have been counted so far. It will be reset by resetting the interrupt."]
    #[inline(always)]
    pub fn event2_value(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, WkupCounterReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,WkupCounterReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This value represents the number of events that have been counted so far. It will be reset by resetting the interrupt."]
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

#[doc = "Control register for the wakeup counter"]
pub type WkupCtrlReg = crate::RegValueT<WkupCtrlReg_SPEC>;

impl WkupCtrlReg {
    #[doc = "0 = no interrupt will be generated\n1 = if the event counter2 reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
    #[inline(always)]
    pub fn wkup2_enable_irq(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, WkupCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,WkupCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = no interrupt will be generated\n1 = if the event counter reaches the value set by WKUP_COMPARE_REG an IRQ will be generated"]
    #[inline(always)]
    pub fn wkup_enable_irq(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, WkupCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,WkupCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = no effect\n1 = emulate key hit. The event counter and counter2 will increment by 1 (after debouncing if enabled). First make this bit 0 before any new key hit can be sensed."]
    #[inline(always)]
    pub fn wkup_sft_keyhit(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, WkupCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,WkupCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Keyboard debounce time (N*1 ms with N = 1 to 63).\n0x0: no debouncing\n0x1 to 0x3F: 1 ms to 63 ms debounce time"]
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
pub struct WkupIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for WkupIrqStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "Reset wakeup interrupt"]
pub type WkupIrqStatusReg = crate::RegValueT<WkupIrqStatusReg_SPEC>;

impl WkupIrqStatusReg {
    #[doc = "writing 1 will reset the event2 counter"]
    #[inline(always)]
    pub fn wkup2_cntr_rst(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, WkupIrqStatusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,WkupIrqStatusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "writing 1 will reset the event counter"]
    #[inline(always)]
    pub fn wkup_cntr_rst(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, WkupIrqStatusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,WkupIrqStatusReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Gives 1 when there is a wkup2 pending IRQ. Writing 1 will reset the interrupt."]
    #[inline(always)]
    pub fn wkup2_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, WkupIrqStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,WkupIrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gives 1 when there is a wkup pending IRQ. Writing 1 will reset the interrupt."]
    #[inline(always)]
    pub fn wkup_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, WkupIrqStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,WkupIrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WkupIrqStatusReg {
    #[inline(always)]
    fn default() -> WkupIrqStatusReg {
        <crate::RegValueT<WkupIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupPolGpioReg_SPEC;
impl crate::sealed::RegSpec for WkupPolGpioReg_SPEC {
    type DataType = u16;
}

#[doc = "Select the sensitivity polarity for each P0 input"]
pub type WkupPolGpioReg = crate::RegValueT<WkupPolGpioReg_SPEC>;

impl WkupPolGpioReg {
    #[doc = "0 = the enabled input P0x increments the event counter if that input goes high\n1 = the enabled input P0x increments the event counter if that input goes low"]
    #[inline(always)]
    pub fn wkup_pol_gpio(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        WkupPolGpioReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            WkupPolGpioReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupPolGpioReg {
    #[inline(always)]
    fn default() -> WkupPolGpioReg {
        <crate::RegValueT<WkupPolGpioReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupSelectGpioReg_SPEC;
impl crate::sealed::RegSpec for WkupSelectGpioReg_SPEC {
    type DataType = u16;
}

#[doc = "Select which inputs from P0 port can trigger wkup counter"]
pub type WkupSelectGpioReg = crate::RegValueT<WkupSelectGpioReg_SPEC>;

impl WkupSelectGpioReg {
    #[doc = "0 = input P0x is not enabled for wakeup event counter\n1 = input P0x is enabled for wakeup event counter"]
    #[inline(always)]
    pub fn wkup_select_gpio(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        WkupSelectGpioReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            WkupSelectGpioReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for WkupSelectGpioReg {
    #[inline(always)]
    fn default() -> WkupSelectGpioReg {
        <crate::RegValueT<WkupSelectGpioReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
