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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:21 +0000

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

pub type TimerCaptureGpio1Reg = crate::RegValueT<TimerCaptureGpio1Reg_SPEC>;

impl TimerCaptureGpio1Reg {
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

pub type TimerCaptureGpio2Reg = crate::RegValueT<TimerCaptureGpio2Reg_SPEC>;

impl TimerCaptureGpio2Reg {
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

pub type TimerCaptureGpio3Reg = crate::RegValueT<TimerCaptureGpio3Reg_SPEC>;

impl TimerCaptureGpio3Reg {
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

pub type TimerCaptureGpio4Reg = crate::RegValueT<TimerCaptureGpio4Reg_SPEC>;

impl TimerCaptureGpio4Reg {
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

pub type TimerClearGpioEventReg = crate::RegValueT<TimerClearGpioEventReg_SPEC>;

impl TimerClearGpioEventReg {
    #[inline(always)]
    pub fn tim_clear_gpio4_event(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, TimerClearGpioEventReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,TimerClearGpioEventReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_clear_gpio3_event(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, TimerClearGpioEventReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,TimerClearGpioEventReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_clear_gpio2_event(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TimerClearGpioEventReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,TimerClearGpioEventReg_SPEC,crate::common::W>::from_register(self,0)
    }

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

pub type TimerClearIrqReg = crate::RegValueT<TimerClearIrqReg_SPEC>;

impl TimerClearIrqReg {
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

pub type TimerCtrlReg = crate::RegValueT<TimerCtrlReg_SPEC>;

impl TimerCtrlReg {
    #[inline(always)]
    pub fn tim_cap_gpio4_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_cap_gpio3_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_cap_gpio2_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_cap_gpio1_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in4_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in3_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_oneshot_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TimerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,TimerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type TimerGpio1ConfReg = crate::RegValueT<TimerGpio1ConfReg_SPEC>;

impl TimerGpio1ConfReg {
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

pub type TimerGpio2ConfReg = crate::RegValueT<TimerGpio2ConfReg_SPEC>;

impl TimerGpio2ConfReg {
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

pub type TimerGpio3ConfReg = crate::RegValueT<TimerGpio3ConfReg_SPEC>;

impl TimerGpio3ConfReg {
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

pub type TimerGpio4ConfReg = crate::RegValueT<TimerGpio4ConfReg_SPEC>;

impl TimerGpio4ConfReg {
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

pub type TimerPrescalerReg = crate::RegValueT<TimerPrescalerReg_SPEC>;

impl TimerPrescalerReg {
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

pub type TimerPrescalerValReg = crate::RegValueT<TimerPrescalerValReg_SPEC>;

impl TimerPrescalerValReg {
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

pub type TimerPwmDcReg = crate::RegValueT<TimerPwmDcReg_SPEC>;

impl TimerPwmDcReg {
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

pub type TimerPwmFreqReg = crate::RegValueT<TimerPwmFreqReg_SPEC>;

impl TimerPwmFreqReg {
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

pub type TimerReloadReg = crate::RegValueT<TimerReloadReg_SPEC>;

impl TimerReloadReg {
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

pub type TimerShotwidthReg = crate::RegValueT<TimerShotwidthReg_SPEC>;

impl TimerShotwidthReg {
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

pub type TimerStatusReg = crate::RegValueT<TimerStatusReg_SPEC>;

impl TimerStatusReg {
    #[inline(always)]
    pub fn tim_gpio4_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_gpio3_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_gpio2_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_gpio1_event_pending(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_oneshot_phase(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, TimerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_in2_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TimerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,TimerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type TimerTimerValReg = crate::RegValueT<TimerTimerValReg_SPEC>;

impl TimerTimerValReg {
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
