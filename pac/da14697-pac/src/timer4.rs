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
#[doc = r"TIMER4 registers"]
unsafe impl ::core::marker::Send for super::Timer4 {}
unsafe impl ::core::marker::Sync for super::Timer4 {}
impl super::Timer4 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Timer value for event on GPIO1"]
    #[inline(always)]
    pub const fn timer4_capture_gpio1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4CaptureGpio1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4CaptureGpio1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Timer value for event on GPIO2"]
    #[inline(always)]
    pub const fn timer4_capture_gpio2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4CaptureGpio2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4CaptureGpio2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Timer clear interrupt"]
    #[inline(always)]
    pub const fn timer4_clear_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4ClearIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4ClearIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Timer control register"]
    #[inline(always)]
    pub const fn timer4_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Timer gpio1 selection"]
    #[inline(always)]
    pub const fn timer4_gpio1_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4Gpio1ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4Gpio1ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Timer gpio2 selection"]
    #[inline(always)]
    pub const fn timer4_gpio2_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4Gpio2ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4Gpio2ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Timer prescaler value"]
    #[inline(always)]
    pub const fn timer4_prescaler_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4PrescalerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4PrescalerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Timer prescaler counter valuew"]
    #[inline(always)]
    pub const fn timer4_prescaler_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4PrescalerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4PrescalerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Timer pwm dc register"]
    #[inline(always)]
    pub const fn timer4_pwm_dc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4PwmDcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4PwmDcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Timer pwm frequency register"]
    #[inline(always)]
    pub const fn timer4_pwm_freq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4PwmFreqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4PwmFreqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Timer reload value and Delay in shot mode"]
    #[inline(always)]
    pub const fn timer4_reload_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4ReloadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4ReloadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Timer status register"]
    #[inline(always)]
    pub const fn timer4_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Timer counter value"]
    #[inline(always)]
    pub const fn timer4_timer_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Timer4TimerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Timer4TimerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4CaptureGpio1Reg_SPEC;
impl crate::sealed::RegSpec for Timer4CaptureGpio1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Timer value for event on GPIO1"]
pub type Timer4CaptureGpio1Reg = crate::RegValueT<Timer4CaptureGpio1Reg_SPEC>;

impl Timer4CaptureGpio1Reg {
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
        Timer4CaptureGpio1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer4CaptureGpio1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4CaptureGpio1Reg {
    #[inline(always)]
    fn default() -> Timer4CaptureGpio1Reg {
        <crate::RegValueT<Timer4CaptureGpio1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4CaptureGpio2Reg_SPEC;
impl crate::sealed::RegSpec for Timer4CaptureGpio2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Timer value for event on GPIO2"]
pub type Timer4CaptureGpio2Reg = crate::RegValueT<Timer4CaptureGpio2Reg_SPEC>;

impl Timer4CaptureGpio2Reg {
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
        Timer4CaptureGpio2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer4CaptureGpio2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4CaptureGpio2Reg {
    #[inline(always)]
    fn default() -> Timer4CaptureGpio2Reg {
        <crate::RegValueT<Timer4CaptureGpio2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4ClearIrqReg_SPEC;
impl crate::sealed::RegSpec for Timer4ClearIrqReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer clear interrupt"]
pub type Timer4ClearIrqReg = crate::RegValueT<Timer4ClearIrqReg_SPEC>;

impl Timer4ClearIrqReg {
    #[doc = "Write any value clear interrupt"]
    #[inline(always)]
    pub fn tim_clear_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer4ClearIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Timer4ClearIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer4ClearIrqReg {
    #[inline(always)]
    fn default() -> Timer4ClearIrqReg {
        <crate::RegValueT<Timer4ClearIrqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4CtrlReg_SPEC;
impl crate::sealed::RegSpec for Timer4CtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer control register"]
pub type Timer4CtrlReg = crate::RegValueT<Timer4CtrlReg_SPEC>;

impl Timer4CtrlReg {
    #[doc = "Timer clock enable\n1 = clock enabled\n0 = clock disabled"]
    #[inline(always)]
    pub fn tim_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select clock\n1 = Timer uses the DIVN clock\n0 = Timer uses the lp clock"]
    #[inline(always)]
    pub fn tim_sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Valid when timer counts up, if it is \'1\' timer does not zero when reaches to reload value. it becomes zero only when it reaches the max value."]
    #[inline(always)]
    pub fn tim_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interrupt mask\n1 = timer IRQ is unmasked\n0 = timer IRQ is masked"]
    #[inline(always)]
    pub fn tim_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Event input 2 edge type\n1 = falling edge\n0 = rising edge"]
    #[inline(always)]
    pub fn tim_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Event input 1 edge type\n1 = falling edge\n0 = rising edge"]
    #[inline(always)]
    pub fn tim_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timer count direction\n1 = down\n0 = up"]
    #[inline(always)]
    pub fn tim_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Timer enable\n1 = On\n0 = Off"]
    #[inline(always)]
    pub fn tim_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer4CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Timer4CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer4CtrlReg {
    #[inline(always)]
    fn default() -> Timer4CtrlReg {
        <crate::RegValueT<Timer4CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4Gpio1ConfReg_SPEC;
impl crate::sealed::RegSpec for Timer4Gpio1ConfReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer gpio1 selection"]
pub type Timer4Gpio1ConfReg = crate::RegValueT<Timer4Gpio1ConfReg_SPEC>;

impl Timer4Gpio1ConfReg {
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
        Timer4Gpio1ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer4Gpio1ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4Gpio1ConfReg {
    #[inline(always)]
    fn default() -> Timer4Gpio1ConfReg {
        <crate::RegValueT<Timer4Gpio1ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4Gpio2ConfReg_SPEC;
impl crate::sealed::RegSpec for Timer4Gpio2ConfReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer gpio2 selection"]
pub type Timer4Gpio2ConfReg = crate::RegValueT<Timer4Gpio2ConfReg_SPEC>;

impl Timer4Gpio2ConfReg {
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
        Timer4Gpio2ConfReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            Timer4Gpio2ConfReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4Gpio2ConfReg {
    #[inline(always)]
    fn default() -> Timer4Gpio2ConfReg {
        <crate::RegValueT<Timer4Gpio2ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4PrescalerReg_SPEC;
impl crate::sealed::RegSpec for Timer4PrescalerReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer prescaler value"]
pub type Timer4PrescalerReg = crate::RegValueT<Timer4PrescalerReg_SPEC>;

impl Timer4PrescalerReg {
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
        Timer4PrescalerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            Timer4PrescalerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4PrescalerReg {
    #[inline(always)]
    fn default() -> Timer4PrescalerReg {
        <crate::RegValueT<Timer4PrescalerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4PrescalerValReg_SPEC;
impl crate::sealed::RegSpec for Timer4PrescalerValReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer prescaler counter valuew"]
pub type Timer4PrescalerValReg = crate::RegValueT<Timer4PrescalerValReg_SPEC>;

impl Timer4PrescalerValReg {
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
        Timer4PrescalerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            Timer4PrescalerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4PrescalerValReg {
    #[inline(always)]
    fn default() -> Timer4PrescalerValReg {
        <crate::RegValueT<Timer4PrescalerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4PwmDcReg_SPEC;
impl crate::sealed::RegSpec for Timer4PwmDcReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer pwm dc register"]
pub type Timer4PwmDcReg = crate::RegValueT<Timer4PwmDcReg_SPEC>;

impl Timer4PwmDcReg {
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
        Timer4PwmDcReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer4PwmDcReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4PwmDcReg {
    #[inline(always)]
    fn default() -> Timer4PwmDcReg {
        <crate::RegValueT<Timer4PwmDcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4PwmFreqReg_SPEC;
impl crate::sealed::RegSpec for Timer4PwmFreqReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer pwm frequency register"]
pub type Timer4PwmFreqReg = crate::RegValueT<Timer4PwmFreqReg_SPEC>;

impl Timer4PwmFreqReg {
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
        Timer4PwmFreqReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Timer4PwmFreqReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4PwmFreqReg {
    #[inline(always)]
    fn default() -> Timer4PwmFreqReg {
        <crate::RegValueT<Timer4PwmFreqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4ReloadReg_SPEC;
impl crate::sealed::RegSpec for Timer4ReloadReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer reload value and Delay in shot mode"]
pub type Timer4ReloadReg = crate::RegValueT<Timer4ReloadReg_SPEC>;

impl Timer4ReloadReg {
    #[doc = "Reload or max value in timer mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
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
        Timer4ReloadReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer4ReloadReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4ReloadReg {
    #[inline(always)]
    fn default() -> Timer4ReloadReg {
        <crate::RegValueT<Timer4ReloadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4StatusReg_SPEC;
impl crate::sealed::RegSpec for Timer4StatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer status register"]
pub type Timer4StatusReg = crate::RegValueT<Timer4StatusReg_SPEC>;

impl Timer4StatusReg {
    #[doc = "OneShot phase\n0 = Wait for event\n1 = Delay phase\n2 = Start Shot\n3 = Shot phase"]
    #[inline(always)]
    pub fn tim_oneshot_phase(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, Timer4StatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,Timer4StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gives the logic level of the IN1"]
    #[inline(always)]
    pub fn tim_in2_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Timer4StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Timer4StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gives the logic level of the IN2"]
    #[inline(always)]
    pub fn tim_in1_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Timer4StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Timer4StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Timer4StatusReg {
    #[inline(always)]
    fn default() -> Timer4StatusReg {
        <crate::RegValueT<Timer4StatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4TimerValReg_SPEC;
impl crate::sealed::RegSpec for Timer4TimerValReg_SPEC {
    type DataType = u32;
}

#[doc = "Timer counter value"]
pub type Timer4TimerValReg = crate::RegValueT<Timer4TimerValReg_SPEC>;

impl Timer4TimerValReg {
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
        Timer4TimerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            u32,
            Timer4TimerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Timer4TimerValReg {
    #[inline(always)]
    fn default() -> Timer4TimerValReg {
        <crate::RegValueT<Timer4TimerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
