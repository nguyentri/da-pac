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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:38 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"TIMER registers"]
unsafe impl ::core::marker::Send for super::Timer {}
unsafe impl ::core::marker::Sync for super::Timer {}
impl super::Timer {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Timer value for event on GPIO1"]
    #[inline(always)]
    pub const fn timer_capture_gpio1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerCaptureGpio1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerCaptureGpio1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Timer value for event on GPIO2"]
    #[inline(always)]
    pub const fn timer_capture_gpio2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerCaptureGpio2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerCaptureGpio2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Timer value for event on GPIO1"]
    #[inline(always)]
    pub const fn timer_capture_gpio3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerCaptureGpio3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerCaptureGpio3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Timer value for event on GPIO1"]
    #[inline(always)]
    pub const fn timer_capture_gpio4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerCaptureGpio4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerCaptureGpio4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Timer clear gpio event register"]
    #[inline(always)]
    pub const fn timer_clear_gpio_event_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerClearGpioEventReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerClearGpioEventReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Timer clear interrupt"]
    #[inline(always)]
    pub const fn timer_clear_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerClearIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerClearIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Timer control register"]
    #[inline(always)]
    pub const fn timer_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Timer gpio1 selection"]
    #[inline(always)]
    pub const fn timer_gpio1_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerGpio1ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerGpio1ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Timer gpio2 selection"]
    #[inline(always)]
    pub const fn timer_gpio2_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerGpio2ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerGpio2ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Timer gpio3 selection"]
    #[inline(always)]
    pub const fn timer_gpio3_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerGpio3ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerGpio3ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Timer gpio4 selection"]
    #[inline(always)]
    pub const fn timer_gpio4_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerGpio4ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerGpio4ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Timer prescaler value"]
    #[inline(always)]
    pub const fn timer_prescaler_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerPrescalerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerPrescalerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Timer prescaler counter valuew"]
    #[inline(always)]
    pub const fn timer_prescaler_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerPrescalerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerPrescalerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Timer pwm dc register"]
    #[inline(always)]
    pub const fn timer_pwm_dc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerPwmDcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerPwmDcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Timer pwm frequency register"]
    #[inline(always)]
    pub const fn timer_pwm_freq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerPwmFreqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerPwmFreqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Timer reload value and Delay in shot mode"]
    #[inline(always)]
    pub const fn timer_reload_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerReloadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerReloadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Timer Shot duration in shot mode"]
    #[inline(always)]
    pub const fn timer_shotwidth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerShotwidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerShotwidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Timer status register"]
    #[inline(always)]
    pub const fn timer_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Timer counter value"]
    #[inline(always)]
    pub const fn timer_timer_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TimerTimerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TimerTimerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerCaptureGpio1Reg_SPEC;
impl crate::sealed::RegSpec for TimerCaptureGpio1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Timer value for event on GPIO1"]
pub type TimerCaptureGpio1Reg = crate::RegValueT<TimerCaptureGpio1Reg_SPEC>;

impl TimerCaptureGpio1Reg {
    #[doc = "Gives the Capture time for event on GPIO1"]
    #[inline(always)]
    pub fn tim_capture_gpio1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        TimerCaptureGpio1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            TimerCaptureGpio1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerCaptureGpio1Reg {
    #[inline(always)]
    fn default() -> TimerCaptureGpio1Reg {
        <crate::RegValueT<TimerCaptureGpio1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerCaptureGpio2Reg_SPEC;
impl crate::sealed::RegSpec for TimerCaptureGpio2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Timer value for event on GPIO2"]
pub type TimerCaptureGpio2Reg = crate::RegValueT<TimerCaptureGpio2Reg_SPEC>;

impl TimerCaptureGpio2Reg {
    #[doc = "Gives the Capture time for event on GPIO2"]
    #[inline(always)]
    pub fn tim_capture_gpio2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        TimerCaptureGpio2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            TimerCaptureGpio2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerCaptureGpio2Reg {
    #[inline(always)]
    fn default() -> TimerCaptureGpio2Reg {
        <crate::RegValueT<TimerCaptureGpio2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerCaptureGpio3Reg_SPEC;
impl crate::sealed::RegSpec for TimerCaptureGpio3Reg_SPEC {
    type DataType = u32;
}

#[doc = "Timer value for event on GPIO1"]
pub type TimerCaptureGpio3Reg = crate::RegValueT<TimerCaptureGpio3Reg_SPEC>;

impl TimerCaptureGpio3Reg {
    #[doc = "Gives the Capture time for event on GPIO3"]
    #[inline(always)]
    pub fn tim_capture_gpio3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        TimerCaptureGpio3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            TimerCaptureGpio3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerCaptureGpio3Reg {
    #[inline(always)]
    fn default() -> TimerCaptureGpio3Reg {
        <crate::RegValueT<TimerCaptureGpio3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerCaptureGpio4Reg_SPEC;
impl crate::sealed::RegSpec for TimerCaptureGpio4Reg_SPEC {
    type DataType = u32;
}

#[doc = "Timer value for event on GPIO1"]
pub type TimerCaptureGpio4Reg = crate::RegValueT<TimerCaptureGpio4Reg_SPEC>;

impl TimerCaptureGpio4Reg {
    #[doc = "Gives the Capture time for event on GPIO4"]
    #[inline(always)]
    pub fn tim_capture_gpio4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        TimerCaptureGpio4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            TimerCaptureGpio4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerCaptureGpio4Reg {
    #[inline(always)]
    fn default() -> TimerCaptureGpio4Reg {
        <crate::RegValueT<TimerCaptureGpio4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerClearGpioEventReg_SPEC;
impl crate::sealed::RegSpec for TimerClearGpioEventReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer clear gpio event register"]
pub type TimerClearGpioEventReg = crate::RegValueT<TimerClearGpioEventReg_SPEC>;

impl TimerClearGpioEventReg {
    #[doc = "1 = Clear GPIO4 event. Return always 0"]
    #[inline(always)]
    pub fn tim_clear_gpio4_event(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, TimerClearGpioEventReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,TimerClearGpioEventReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "1 = Clear GPIO3 event. Return always 0"]
    #[inline(always)]
    pub fn tim_clear_gpio3_event(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, TimerClearGpioEventReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,TimerClearGpioEventReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "1 = Clear GPIO2 event. Return always 0"]
    #[inline(always)]
    pub fn tim_clear_gpio2_event(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TimerClearGpioEventReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,TimerClearGpioEventReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "1 = Clear GPIO1 event. Return always 0"]
    #[inline(always)]
    pub fn tim_clear_gpio1_event(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TimerClearGpioEventReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,TimerClearGpioEventReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for TimerClearGpioEventReg {
    #[inline(always)]
    fn default() -> TimerClearGpioEventReg {
        <crate::RegValueT<TimerClearGpioEventReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerClearIrqReg_SPEC;
impl crate::sealed::RegSpec for TimerClearIrqReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer clear interrupt"]
pub type TimerClearIrqReg = crate::RegValueT<TimerClearIrqReg_SPEC>;

impl TimerClearIrqReg {
    #[doc = "Write any value clear interrupt"]
    #[inline(always)]
    pub fn tim_clear_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TimerClearIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,TimerClearIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for TimerClearIrqReg {
    #[inline(always)]
    fn default() -> TimerClearIrqReg {
        <crate::RegValueT<TimerClearIrqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerCtrlReg_SPEC;
impl crate::sealed::RegSpec for TimerCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer control register"]
pub type TimerCtrlReg = crate::RegValueT<TimerCtrlReg_SPEC>;

impl TimerCtrlReg {
    #[doc = "0 = Event on GPIO4 does not create a CAPTIM interrrupt\n1 = Event on GPIO4 creates a CAPTIM interrrupt"]
    #[inline(always)]
    pub fn tim_cap_gpio4_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Event on GPIO3 does not create a CAPTIM interrrupt\n1 = Event on GPIO3 creates a CAPTIM interrrupt"]
    #[inline(always)]
    pub fn tim_cap_gpio3_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Event on GPIO2 does not create a CAPTIM interrrupt\n1 = Event on GPIO2 creates a CAPTIM interrrupt"]
    #[inline(always)]
    pub fn tim_cap_gpio2_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Event on GPIO1 does not create a CAPTIM interrrupt\n1 = Event on GPIO1 creates a CAPTIM interrrupt"]
    #[inline(always)]
    pub fn tim_cap_gpio1_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Event input 4 edge type\n1 = falling edge\n0 = rising edge"]
    #[inline(always)]
    pub fn tim_in4_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Event input 3 edge type\n1 = falling edge\n0 = rising edge"]
    #[inline(always)]
    pub fn tim_in3_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timer clock enable\n1 = clock enabled\n0 = clock disabled"]
    #[inline(always)]
    pub fn tim_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select clock\n1 = Timer uses the DIVN clock\n0 = Timer uses the lp clock"]
    #[inline(always)]
    pub fn tim_sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Valid when timer counts up, if it is \'1\' timer does not zero when reaches to reload value. it becomes zero only when it reaches the max value."]
    #[inline(always)]
    pub fn tim_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt mask\n1 = timer IRQ is unmasked\n0 = timer IRQ is masked"]
    #[inline(always)]
    pub fn tim_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Event input 2 edge type\n1 = falling edge\n0 = rising edge"]
    #[inline(always)]
    pub fn tim_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Event input 1 edge type\n1 = falling edge\n0 = rising edge"]
    #[inline(always)]
    pub fn tim_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timer count direction\n1 = down\n0 = up"]
    #[inline(always)]
    pub fn tim_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timer mode\n1 = One shot enabled\n0 = Counter enabled"]
    #[inline(always)]
    pub fn tim_oneshot_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timer enable\n1 = On\n0 = Off"]
    #[inline(always)]
    pub fn tim_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TimerCtrlReg {
    #[inline(always)]
    fn default() -> TimerCtrlReg {
        <crate::RegValueT<TimerCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerGpio1ConfReg_SPEC;
impl crate::sealed::RegSpec for TimerGpio1ConfReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer gpio1 selection"]
pub type TimerGpio1ConfReg = crate::RegValueT<TimerGpio1ConfReg_SPEC>;

impl TimerGpio1ConfReg {
    #[doc = "Select one of the 32 GPIOs as IN1, Valid value 0-32. 1 for the first gpio, 32 for the last gpio. 0 Disable input"]
    #[inline(always)]
    pub fn tim_gpio1_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        TimerGpio1ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            TimerGpio1ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerGpio1ConfReg {
    #[inline(always)]
    fn default() -> TimerGpio1ConfReg {
        <crate::RegValueT<TimerGpio1ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerGpio2ConfReg_SPEC;
impl crate::sealed::RegSpec for TimerGpio2ConfReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer gpio2 selection"]
pub type TimerGpio2ConfReg = crate::RegValueT<TimerGpio2ConfReg_SPEC>;

impl TimerGpio2ConfReg {
    #[doc = "Select one of the 32 GPIOs as IN2, Valid value 0-32. 1 for the first gpio, 32 for the last gpio. 0 Disable input"]
    #[inline(always)]
    pub fn tim_gpio2_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        TimerGpio2ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            TimerGpio2ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerGpio2ConfReg {
    #[inline(always)]
    fn default() -> TimerGpio2ConfReg {
        <crate::RegValueT<TimerGpio2ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerGpio3ConfReg_SPEC;
impl crate::sealed::RegSpec for TimerGpio3ConfReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer gpio3 selection"]
pub type TimerGpio3ConfReg = crate::RegValueT<TimerGpio3ConfReg_SPEC>;

impl TimerGpio3ConfReg {
    #[doc = "Select one of the 32 GPIOs as IN3, Valid value 0-32. 1 for the first gpio, 32 for the last gpio. 0 Disable input"]
    #[inline(always)]
    pub fn tim_gpio3_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        TimerGpio3ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            TimerGpio3ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerGpio3ConfReg {
    #[inline(always)]
    fn default() -> TimerGpio3ConfReg {
        <crate::RegValueT<TimerGpio3ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerGpio4ConfReg_SPEC;
impl crate::sealed::RegSpec for TimerGpio4ConfReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer gpio4 selection"]
pub type TimerGpio4ConfReg = crate::RegValueT<TimerGpio4ConfReg_SPEC>;

impl TimerGpio4ConfReg {
    #[doc = "Select one of the 32 GPIOs as IN4, Valid value 0-32. 1 for the first gpio, 32 for the last gpio. 0 Disable input"]
    #[inline(always)]
    pub fn tim_gpio4_conf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        TimerGpio4ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            TimerGpio4ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerGpio4ConfReg {
    #[inline(always)]
    fn default() -> TimerGpio4ConfReg {
        <crate::RegValueT<TimerGpio4ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerPrescalerReg_SPEC;
impl crate::sealed::RegSpec for TimerPrescalerReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer prescaler value"]
pub type TimerPrescalerReg = crate::RegValueT<TimerPrescalerReg_SPEC>;

impl TimerPrescalerReg {
    #[doc = "Defines the timer count frequency. CLOCK frequency / (TIM_PRESCALER+1)"]
    #[inline(always)]
    pub fn tim_prescaler(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        TimerPrescalerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            TimerPrescalerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerPrescalerReg {
    #[inline(always)]
    fn default() -> TimerPrescalerReg {
        <crate::RegValueT<TimerPrescalerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerPrescalerValReg_SPEC;
impl crate::sealed::RegSpec for TimerPrescalerValReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer prescaler counter valuew"]
pub type TimerPrescalerValReg = crate::RegValueT<TimerPrescalerValReg_SPEC>;

impl TimerPrescalerValReg {
    #[doc = "Gives the current prescaler counter value"]
    #[inline(always)]
    pub fn tim_prescaler_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        TimerPrescalerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            TimerPrescalerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerPrescalerValReg {
    #[inline(always)]
    fn default() -> TimerPrescalerValReg {
        <crate::RegValueT<TimerPrescalerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerPwmDcReg_SPEC;
impl crate::sealed::RegSpec for TimerPwmDcReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer pwm dc register"]
pub type TimerPwmDcReg = crate::RegValueT<TimerPwmDcReg_SPEC>;

impl TimerPwmDcReg {
    #[doc = "Defines the PWM duty cycle. TIM_PWM_DC / ( TIM_PWM_FREQ+1)"]
    #[inline(always)]
    pub fn tim_pwm_dc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        TimerPwmDcReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            TimerPwmDcReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerPwmDcReg {
    #[inline(always)]
    fn default() -> TimerPwmDcReg {
        <crate::RegValueT<TimerPwmDcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerPwmFreqReg_SPEC;
impl crate::sealed::RegSpec for TimerPwmFreqReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer pwm frequency register"]
pub type TimerPwmFreqReg = crate::RegValueT<TimerPwmFreqReg_SPEC>;

impl TimerPwmFreqReg {
    #[doc = "Defines the PWM frequency. Timer clock frequency / (TIM_PWM_FREQ+1)\nTimer clock is clock after prescaler"]
    #[inline(always)]
    pub fn tim_pwm_freq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        TimerPwmFreqReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            TimerPwmFreqReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerPwmFreqReg {
    #[inline(always)]
    fn default() -> TimerPwmFreqReg {
        <crate::RegValueT<TimerPwmFreqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerReloadReg_SPEC;
impl crate::sealed::RegSpec for TimerReloadReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer reload value and Delay in shot mode"]
pub type TimerReloadReg = crate::RegValueT<TimerReloadReg_SPEC>;

impl TimerReloadReg {
    #[doc = "Reload or max value in timer mode, Delay phase duration in oneshot mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
    #[inline(always)]
    pub fn tim_reload(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        TimerReloadReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            TimerReloadReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerReloadReg {
    #[inline(always)]
    fn default() -> TimerReloadReg {
        <crate::RegValueT<TimerReloadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerShotwidthReg_SPEC;
impl crate::sealed::RegSpec for TimerShotwidthReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer Shot duration in shot mode"]
pub type TimerShotwidthReg = crate::RegValueT<TimerShotwidthReg_SPEC>;

impl TimerShotwidthReg {
    #[doc = "Shot phase duration in oneshot mode"]
    #[inline(always)]
    pub fn tim_shotwidth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        TimerShotwidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            TimerShotwidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerShotwidthReg {
    #[inline(always)]
    fn default() -> TimerShotwidthReg {
        <crate::RegValueT<TimerShotwidthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerStatusReg_SPEC;
impl crate::sealed::RegSpec for TimerStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer status register"]
pub type TimerStatusReg = crate::RegValueT<TimerStatusReg_SPEC>;

impl TimerStatusReg {
    #[doc = "When 1, GPIO4 event is pending."]
    #[inline(always)]
    pub fn tim_gpio4_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When 1, GPIO3 event is pending."]
    #[inline(always)]
    pub fn tim_gpio3_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When 1, GPIO2 event is pending."]
    #[inline(always)]
    pub fn tim_gpio2_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When 1, GPIO1 event is pending."]
    #[inline(always)]
    pub fn tim_gpio1_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "OneShot phase\n0 = Wait for event\n1 = Delay phase\n2 = Start Shot\n3 = Shot phase"]
    #[inline(always)]
    pub fn tim_oneshot_phase(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, TimerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gives the logic level of the IN1"]
    #[inline(always)]
    pub fn tim_in2_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gives the logic level of the IN2"]
    #[inline(always)]
    pub fn tim_in1_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for TimerStatusReg {
    #[inline(always)]
    fn default() -> TimerStatusReg {
        <crate::RegValueT<TimerStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimerTimerValReg_SPEC;
impl crate::sealed::RegSpec for TimerTimerValReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer counter value"]
pub type TimerTimerValReg = crate::RegValueT<TimerTimerValReg_SPEC>;

impl TimerTimerValReg {
    #[doc = "Gives the current timer value"]
    #[inline(always)]
    pub fn tim_timer_value(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        u32,
        TimerTimerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            TimerTimerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for TimerTimerValReg {
    #[inline(always)]
    fn default() -> TimerTimerValReg {
        <crate::RegValueT<TimerTimerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
