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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:19 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"TIMER1 registers"]
unsafe impl ::core::marker::Send for super::Timer1 {}
unsafe impl ::core::marker::Sync for super::Timer1 {}
impl super::Timer1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn timer1_capcnt1_value_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer1Capcnt1ValueReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer1Capcnt1ValueReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer1_capcnt2_value_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer1Capcnt2ValueReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer1Capcnt2ValueReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer1_capture_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer1CaptureReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer1CaptureReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer1_clr_event_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer1ClrEventReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer1ClrEventReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer1_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer1CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer1CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn timer1_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer1StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer1StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1Capcnt1ValueReg_SPEC;
impl crate::sealed::RegSpec for Timer1Capcnt1ValueReg_SPEC {
    type DataType = u32;
}

pub type Timer1Capcnt1ValueReg = crate::RegValueT<Timer1Capcnt1ValueReg_SPEC>;

impl Timer1Capcnt1ValueReg {
    #[inline(always)]
    pub fn timer1_capcnt1_rtc_high(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x7ff,
        1,
        0,
        u16,
        u16,
        Timer1Capcnt1ValueReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x7ff,
            1,
            0,
            u16,
            u16,
            Timer1Capcnt1ValueReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn timer1_capcnt1_value(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        u16,
        u16,
        Timer1Capcnt1ValueReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            u16,
            u16,
            Timer1Capcnt1ValueReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer1Capcnt1ValueReg {
    #[inline(always)]
    fn default() -> Timer1Capcnt1ValueReg {
        <crate::RegValueT<Timer1Capcnt1ValueReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1Capcnt2ValueReg_SPEC;
impl crate::sealed::RegSpec for Timer1Capcnt2ValueReg_SPEC {
    type DataType = u32;
}

pub type Timer1Capcnt2ValueReg = crate::RegValueT<Timer1Capcnt2ValueReg_SPEC>;

impl Timer1Capcnt2ValueReg {
    #[inline(always)]
    pub fn timer1_capcnt2_rtc_high(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x7ff,
        1,
        0,
        u16,
        u16,
        Timer1Capcnt2ValueReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x7ff,
            1,
            0,
            u16,
            u16,
            Timer1Capcnt2ValueReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn timer1_capcnt2_value(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        u16,
        u16,
        Timer1Capcnt2ValueReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            u16,
            u16,
            Timer1Capcnt2ValueReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer1Capcnt2ValueReg {
    #[inline(always)]
    fn default() -> Timer1Capcnt2ValueReg {
        <crate::RegValueT<Timer1Capcnt2ValueReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1CaptureReg_SPEC;
impl crate::sealed::RegSpec for Timer1CaptureReg_SPEC {
    type DataType = u32;
}

pub type Timer1CaptureReg = crate::RegValueT<Timer1CaptureReg_SPEC>;

impl Timer1CaptureReg {
    #[inline(always)]
    pub fn timer1_in2_stamp_type(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in2_period_max(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x3f,
        1,
        0,
        u8,
        u8,
        Timer1CaptureReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer1CaptureReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn timer1_in2_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in2_count_en(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_gpio2_conf(
        self,
    ) -> crate::common::RegisterField<14, 0xf, 1, 0, u8, u8, Timer1CaptureReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            14,
            0xf,
            1,
            0,
            u8,
            u8,
            Timer1CaptureReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn timer1_in1_stamp_type(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in1_period_max(
        self,
    ) -> crate::common::RegisterField<7, 0x3f, 1, 0, u8, u8, Timer1CaptureReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer1CaptureReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn timer1_in1_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in1_count_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_gpio1_conf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Timer1CaptureReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer1CaptureReg {
    #[inline(always)]
    fn default() -> Timer1CaptureReg {
        <crate::RegValueT<Timer1CaptureReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1ClrEventReg_SPEC;
impl crate::sealed::RegSpec for Timer1ClrEventReg_SPEC {
    type DataType = u32;
}

pub type Timer1ClrEventReg = crate::RegValueT<Timer1ClrEventReg_SPEC>;

impl Timer1ClrEventReg {
    #[inline(always)]
    pub fn timer1_clr_in2_event(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Timer1ClrEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Timer1ClrEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_clr_in1_event(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Timer1ClrEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Timer1ClrEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_clr_timer_event(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer1ClrEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Timer1ClrEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer1ClrEventReg {
    #[inline(always)]
    fn default() -> Timer1ClrEventReg {
        <crate::RegValueT<Timer1ClrEventReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1CtrlReg_SPEC;
impl crate::sealed::RegSpec for Timer1CtrlReg_SPEC {
    type DataType = u32;
}

pub type Timer1CtrlReg = crate::RegValueT<Timer1CtrlReg_SPEC>;

impl Timer1CtrlReg {
    #[inline(always)]
    pub fn timer1_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_use_sys_clk(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_reload(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Timer1CtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            u16,
            u16,
            Timer1CtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer1CtrlReg {
    #[inline(always)]
    fn default() -> Timer1CtrlReg {
        <crate::RegValueT<Timer1CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1StatusReg_SPEC;
impl crate::sealed::RegSpec for Timer1StatusReg_SPEC {
    type DataType = u32;
}

pub type Timer1StatusReg = crate::RegValueT<Timer1StatusReg_SPEC>;

impl Timer1StatusReg {
    #[inline(always)]
    pub fn timer1_in2_ovrflw(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in1_ovrflw(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in2_event(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_in1_event(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_timer_event(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timer1_timer_value(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        u16,
        u16,
        Timer1StatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            u16,
            u16,
            Timer1StatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer1StatusReg {
    #[inline(always)]
    fn default() -> Timer1StatusReg {
        <crate::RegValueT<Timer1StatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
