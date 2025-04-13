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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:08 +0000

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
    #[doc = "Capture Timer value for event on GPIO1"]
    #[inline(always)]
    pub const fn captim_capture_gpio1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimCaptureGpio1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimCaptureGpio1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Capture Timer value for event on GPIO2"]
    #[inline(always)]
    pub const fn captim_capture_gpio2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimCaptureGpio2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimCaptureGpio2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Capture Timer control register"]
    #[inline(always)]
    pub const fn captim_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Capture Timer gpio1 selection"]
    #[inline(always)]
    pub const fn captim_gpio1_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimGpio1ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimGpio1ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Capture Timer gpio2 selection"]
    #[inline(always)]
    pub const fn captim_gpio2_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimGpio2ConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimGpio2ConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Capture Timer prescaler value"]
    #[inline(always)]
    pub const fn captim_prescaler_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPrescalerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPrescalerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Capture Timer interrupt status register"]
    #[inline(always)]
    pub const fn captim_prescaler_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPrescalerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPrescalerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Capture Timer pwm dc register"]
    #[inline(always)]
    pub const fn captim_pwm_dc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPwmDcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPwmDcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Capture Timer pwm frequency register. PWM5 period is defined by the reference clock frequency multiplied by this value."]
    #[inline(always)]
    pub const fn captim_pwm_freq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimPwmFreqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimPwmFreqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Capture Timer reload value and Delay in shot mode"]
    #[inline(always)]
    pub const fn captim_reload_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimReloadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimReloadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Capture Timer Shot duration in shot mode"]
    #[inline(always)]
    pub const fn captim_shotwidth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimShotwidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimShotwidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Capture Timer status register"]
    #[inline(always)]
    pub const fn captim_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Capture Timer counter value"]
    #[inline(always)]
    pub const fn captim_timer_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CaptimTimerValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CaptimTimerValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimCaptureGpio1Reg_SPEC;
impl crate::sealed::RegSpec for CaptimCaptureGpio1Reg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer value for event on GPIO1"]
pub type CaptimCaptureGpio1Reg = crate::RegValueT<CaptimCaptureGpio1Reg_SPEC>;

impl CaptimCaptureGpio1Reg {
    #[doc = "Gives the Capture time for event on GPIO1"]
    #[inline(always)]
    pub fn captim_capture_gpio1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        CaptimCaptureGpio1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            CaptimCaptureGpio1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimCaptureGpio1Reg {
    #[inline(always)]
    fn default() -> CaptimCaptureGpio1Reg {
        <crate::RegValueT<CaptimCaptureGpio1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimCaptureGpio2Reg_SPEC;
impl crate::sealed::RegSpec for CaptimCaptureGpio2Reg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer value for event on GPIO2"]
pub type CaptimCaptureGpio2Reg = crate::RegValueT<CaptimCaptureGpio2Reg_SPEC>;

impl CaptimCaptureGpio2Reg {
    #[doc = "Gives the Capture time for event on GPIO2"]
    #[inline(always)]
    pub fn captim_capture_gpio2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        CaptimCaptureGpio2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            CaptimCaptureGpio2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimCaptureGpio2Reg {
    #[inline(always)]
    fn default() -> CaptimCaptureGpio2Reg {
        <crate::RegValueT<CaptimCaptureGpio2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimCtrlReg_SPEC;
impl crate::sealed::RegSpec for CaptimCtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer control register"]
pub type CaptimCtrlReg = crate::RegValueT<CaptimCtrlReg_SPEC>;

impl CaptimCtrlReg {
    #[doc = "\'1\' When Capture Timer use the system clock else use the clock 32KHz"]
    #[inline(always)]
    pub fn captim_sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Only when timer counts up, if it is \'1\' timer does not zero when reaches to reload value. it is zero only when it has the max value."]
    #[inline(always)]
    pub fn captim_free_run_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "\'1\' When Capture timer IRQ unmask, \'0\' masked"]
    #[inline(always)]
    pub fn captim_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "\'1\' When Input1 event type is falling edge, \'0\' rising edge"]
    #[inline(always)]
    pub fn captim_in2_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "\'1\' When Input2 event type is falling edge, \'0\' rising edge"]
    #[inline(always)]
    pub fn captim_in1_event_fall_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "\'1\' when timer counts down, \'0\' count up"]
    #[inline(always)]
    pub fn captim_count_down_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "\'1\' Capture Timer in OneShot mode, \'0\' Capture/Timer mode"]
    #[inline(always)]
    pub fn captim_oneshot_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "\'1\' Capture Timer enabled, else disabled"]
    #[inline(always)]
    pub fn captim_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CaptimCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CaptimCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimCtrlReg {
    #[inline(always)]
    fn default() -> CaptimCtrlReg {
        <crate::RegValueT<CaptimCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimGpio1ConfReg_SPEC;
impl crate::sealed::RegSpec for CaptimGpio1ConfReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer gpio1 selection"]
pub type CaptimGpio1ConfReg = crate::RegValueT<CaptimGpio1ConfReg_SPEC>;

impl CaptimGpio1ConfReg {
    #[doc = "Select one of the 37 GPIOs as IN1, Valid value 0-37. 1 for P00 .. 37 for P47. When 0 Disable input"]
    #[inline(always)]
    pub fn captim_gpio1_conf(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, CaptimGpio1ConfReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, CaptimGpio1ConfReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimGpio1ConfReg {
    #[inline(always)]
    fn default() -> CaptimGpio1ConfReg {
        <crate::RegValueT<CaptimGpio1ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimGpio2ConfReg_SPEC;
impl crate::sealed::RegSpec for CaptimGpio2ConfReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer gpio2 selection"]
pub type CaptimGpio2ConfReg = crate::RegValueT<CaptimGpio2ConfReg_SPEC>;

impl CaptimGpio2ConfReg {
    #[doc = "Select one of the 37 GPIOs as IN2, Valid value 0-37. 1 for P00 .. 37 for P47. When 0 Disable input"]
    #[inline(always)]
    pub fn captim_gpio2_conf(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, CaptimGpio2ConfReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, CaptimGpio2ConfReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimGpio2ConfReg {
    #[inline(always)]
    fn default() -> CaptimGpio2ConfReg {
        <crate::RegValueT<CaptimGpio2ConfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPrescalerReg_SPEC;
impl crate::sealed::RegSpec for CaptimPrescalerReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer prescaler value"]
pub type CaptimPrescalerReg = crate::RegValueT<CaptimPrescalerReg_SPEC>;

impl CaptimPrescalerReg {
    #[doc = "Define the timer count frequncy. Freq = Freq_clock / (value+1)"]
    #[inline(always)]
    pub fn captim_prescaler(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        CaptimPrescalerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            CaptimPrescalerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimPrescalerReg {
    #[inline(always)]
    fn default() -> CaptimPrescalerReg {
        <crate::RegValueT<CaptimPrescalerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPrescalerValReg_SPEC;
impl crate::sealed::RegSpec for CaptimPrescalerValReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer interrupt status register"]
pub type CaptimPrescalerValReg = crate::RegValueT<CaptimPrescalerValReg_SPEC>;

impl CaptimPrescalerValReg {
    #[doc = "Gives the current prescaler value"]
    #[inline(always)]
    pub fn captim_prescaler_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        CaptimPrescalerValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            CaptimPrescalerValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimPrescalerValReg {
    #[inline(always)]
    fn default() -> CaptimPrescalerValReg {
        <crate::RegValueT<CaptimPrescalerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPwmDcReg_SPEC;
impl crate::sealed::RegSpec for CaptimPwmDcReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer pwm dc register"]
pub type CaptimPwmDcReg = crate::RegValueT<CaptimPwmDcReg_SPEC>;

impl CaptimPwmDcReg {
    #[doc = "Define the PWM duty cyucle = pwm_dc / ( pwm_freq+1)"]
    #[inline(always)]
    pub fn captim_pwm_dc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, CaptimPwmDcReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, CaptimPwmDcReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimPwmDcReg {
    #[inline(always)]
    fn default() -> CaptimPwmDcReg {
        <crate::RegValueT<CaptimPwmDcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimPwmFreqReg_SPEC;
impl crate::sealed::RegSpec for CaptimPwmFreqReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer pwm frequency register. PWM5 period is defined by the reference clock frequency multiplied by this value."]
pub type CaptimPwmFreqReg = crate::RegValueT<CaptimPwmFreqReg_SPEC>;

impl CaptimPwmFreqReg {
    #[doc = "Define the PWM frequency. = prescaler frequency / (value+1)"]
    #[inline(always)]
    pub fn captim_pwm_freq(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, CaptimPwmFreqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, CaptimPwmFreqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimPwmFreqReg {
    #[inline(always)]
    fn default() -> CaptimPwmFreqReg {
        <crate::RegValueT<CaptimPwmFreqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimReloadReg_SPEC;
impl crate::sealed::RegSpec for CaptimReloadReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer reload value and Delay in shot mode"]
pub type CaptimReloadReg = crate::RegValueT<CaptimReloadReg_SPEC>;

impl CaptimReloadReg {
    #[doc = "Reload or max value in timer mode, Delay phase duration in oneshot mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
    #[inline(always)]
    pub fn captim_reload(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, CaptimReloadReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, CaptimReloadReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimReloadReg {
    #[inline(always)]
    fn default() -> CaptimReloadReg {
        <crate::RegValueT<CaptimReloadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimShotwidthReg_SPEC;
impl crate::sealed::RegSpec for CaptimShotwidthReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer Shot duration in shot mode"]
pub type CaptimShotwidthReg = crate::RegValueT<CaptimShotwidthReg_SPEC>;

impl CaptimShotwidthReg {
    #[doc = "Shot phase duration in oneshot mode"]
    #[inline(always)]
    pub fn captim_shotwidth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        CaptimShotwidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            CaptimShotwidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CaptimShotwidthReg {
    #[inline(always)]
    fn default() -> CaptimShotwidthReg {
        <crate::RegValueT<CaptimShotwidthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimStatusReg_SPEC;
impl crate::sealed::RegSpec for CaptimStatusReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer status register"]
pub type CaptimStatusReg = crate::RegValueT<CaptimStatusReg_SPEC>;

impl CaptimStatusReg {
    #[doc = "0 : Wait for event, 1 : Delay phase, 2 : Start Shot, 3 : Shot phase"]
    #[inline(always)]
    pub fn captim_oneshot_phase(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, CaptimStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8, CaptimStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Gives the logic level of the IN1"]
    #[inline(always)]
    pub fn captim_in2_state(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CaptimStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CaptimStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Gives the logic level of the IN2"]
    #[inline(always)]
    pub fn captim_in1_state(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CaptimStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CaptimStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimStatusReg {
    #[inline(always)]
    fn default() -> CaptimStatusReg {
        <crate::RegValueT<CaptimStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CaptimTimerValReg_SPEC;
impl crate::sealed::RegSpec for CaptimTimerValReg_SPEC {
    type DataType = u16;
}
#[doc = "Capture Timer counter value"]
pub type CaptimTimerValReg = crate::RegValueT<CaptimTimerValReg_SPEC>;

impl CaptimTimerValReg {
    #[doc = "Gives the current timer value"]
    #[inline(always)]
    pub fn captim_timer_value(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, CaptimTimerValReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, CaptimTimerValReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CaptimTimerValReg {
    #[inline(always)]
    fn default() -> CaptimTimerValReg {
        <crate::RegValueT<CaptimTimerValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
