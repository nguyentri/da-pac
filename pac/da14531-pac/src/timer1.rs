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
#[doc = r"TIMER1 registers"]
unsafe impl ::core::marker::Send for super::Timer1 {}
unsafe impl ::core::marker::Sync for super::Timer1 {}
impl super::Timer1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Timer1 value for event on GPIO1"]
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

    #[doc = "Timer1 value for event on GPIO2"]
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

    #[doc = "Timer1 Capture control register"]
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

    #[doc = "Clear event register"]
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

    #[doc = "Timer1 control register"]
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

    #[doc = "Timer1 counter value"]
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

#[doc = "Timer1 value for event on GPIO1"]
pub type Timer1Capcnt1ValueReg = crate::RegValueT<Timer1Capcnt1ValueReg_SPEC>;

impl Timer1Capcnt1ValueReg {
    #[doc = "In Counter mode : Not used\nIn Capture mode: Gives the RTC time stamp (high part) when an IN1 event was occurred"]
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

    #[doc = "In Counter mode : Gives the number of timer clock cycles minus 1 which was measured during TIMER1_IN1_PERIOD_MAX periods of IN1\nIn Capture mode (TIMER1_IN1_STAMP_TYPE=0) : Gives the Counter value when an IN1 event was occurred\nIn Capture mode (TIMER1_IN1_STAMP_TYPE=1) : Gives the RTC time stamp (low part) when an IN1 event was occurred"]
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

#[doc = "Timer1 value for event on GPIO2"]
pub type Timer1Capcnt2ValueReg = crate::RegValueT<Timer1Capcnt2ValueReg_SPEC>;

impl Timer1Capcnt2ValueReg {
    #[doc = "In Counter mode : Not used\nIn Capture mode: Gives the RTC time stamp (high part) when an IN2 event was occurred"]
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

    #[doc = "In Counter mode : Gives the number of timer clock cycles minus 1 which was measured during TIMER1_IN2_PERIOD_MAX periods of IN2\nIn Capture mode (TIMER1_IN2_STAMP_TYPE=0) : Gives the Counter value when an IN2 event was occurred\nIn Capture mode (TIMER1_IN2_STAMP_TYPE=1) : Gives the RTC time stamp (low part) when an IN2 event was occurred"]
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

#[doc = "Timer1 Capture control register"]
pub type Timer1CaptureReg = crate::RegValueT<Timer1CaptureReg_SPEC>;

impl Timer1CaptureReg {
    #[doc = "0 = On each event store the counter value\n1 = On each event store the RTC time stamp"]
    #[inline(always)]
    pub fn timer1_in2_stamp_type(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gives the number of periods +1 of IN2, in which module counts"]
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

    #[doc = "1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN2_PERIOD_MAX\n0 = Interrupt is masked"]
    #[inline(always)]
    pub fn timer1_in2_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Capture mode\n1 = Count mode"]
    #[inline(always)]
    pub fn timer1_in2_count_en(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Rising edge event\n1 = Falling edge event\nit should be written when TIMER1_GPIO2_CONF=0 to prevent false events"]
    #[inline(always)]
    pub fn timer1_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0,13,14,15 = IN2 is not used\n1..12 = Defines the P0 pin (0..11) module will use as IN2"]
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

    #[doc = "0 = On each event store the counter value\n1 = On each event store the RTC time stamp"]
    #[inline(always)]
    pub fn timer1_in1_stamp_type(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Gives the number of periods +1 of IN1, in which module counts"]
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

    #[doc = "1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN1_PERIOD_MAX\n0 = Interrupt is masked"]
    #[inline(always)]
    pub fn timer1_in1_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Capture mode\n1 = Count mode"]
    #[inline(always)]
    pub fn timer1_in1_count_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Rising edge event\n1 = Falling edge event\nit should be written when TIMER1_GPIO1_CONF=0 to prevent false events"]
    #[inline(always)]
    pub fn timer1_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Timer1CaptureReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Timer1CaptureReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0,13,14,15 = IN1 is not used\n1..12 = Defines the P0 pin (0..11) module will use as IN1"]
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

#[doc = "Clear event register"]
pub type Timer1ClrEventReg = crate::RegValueT<Timer1ClrEventReg_SPEC>;

impl Timer1ClrEventReg {
    #[doc = "Write 1 to clear the TIMER1_IN2_EVENT and TIMER1_IN2_OVRFLW"]
    #[inline(always)]
    pub fn timer1_clr_in2_event(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Timer1ClrEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Timer1ClrEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write 1 to clear the TIMER1_IN1_EVENT and TIMER1_IN1_OVRFLW"]
    #[inline(always)]
    pub fn timer1_clr_in1_event(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Timer1ClrEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Timer1ClrEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write 1 to clear the TIMER1_TIMER_EVENT"]
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

#[doc = "Timer1 control register"]
pub type Timer1CtrlReg = crate::RegValueT<Timer1CtrlReg_SPEC>;

impl Timer1CtrlReg {
    #[doc = "0 = timer1 clock is disabled\n1 = timer1 clock is enabled"]
    #[inline(always)]
    pub fn timer1_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Timer1 use the clock LP clock\n1 = Timer1 use the system clock"]
    #[inline(always)]
    pub fn timer1_use_sys_clk(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Applicable when timer counts up\n1 = timer1 goes to zero when it reaches the max value.\n0 = timer1 goes to zero when it reaches the reload value."]
    #[inline(always)]
    pub fn timer1_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = timer1 IRQ masked\n1 = timer1 IRQ unmasked"]
    #[inline(always)]
    pub fn timer1_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = timer1 counts up\n1 = timer1 counts down"]
    #[inline(always)]
    pub fn timer1_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Timer1 disabled\n1 = Timer1 enabled"]
    #[inline(always)]
    pub fn timer1_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Timer1CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Timer1CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reload or max value in timer mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
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

#[doc = "Timer1 counter value"]
pub type Timer1StatusReg = crate::RegValueT<Timer1StatusReg_SPEC>;

impl Timer1StatusReg {
    #[doc = "1 = New IN2 event occurred while Interrupt was pending.\nTIMER1_CAPCNT2_VALUE_REG gives the time stamp of the first event."]
    #[inline(always)]
    pub fn timer1_in2_ovrflw(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = New IN1 event occurred while Interrupt was pending.\nTIMER1_CAPCNT1_VALUE_REG gives the time stamp of the first event."]
    #[inline(always)]
    pub fn timer1_in1_ovrflw(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = Pending Capture 2 interrupt. It has be clear writing 1 to TIMER1_CLR_IN2_EVENT"]
    #[inline(always)]
    pub fn timer1_in2_event(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = Pending Capture 1 interrupt. It has be clear writing 1 to TIMER1_CLR_IN1_EVENT"]
    #[inline(always)]
    pub fn timer1_in1_event(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1 = Pending Timer interrupt. it has be clear writing 1\' to TIMER1_CLR_TIMER_EVENT"]
    #[inline(always)]
    pub fn timer1_timer_event(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Timer1StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,Timer1StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gives the current timer value"]
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
