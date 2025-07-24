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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:31 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"RTC registers"]
unsafe impl ::core::marker::Send for super::Rtc {}
unsafe impl ::core::marker::Sync for super::Rtc {}
impl super::Rtc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "RTC Alarm Enable Register"]
    #[inline(always)]
    pub const fn rtc_alarm_enable_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcAlarmEnableReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcAlarmEnableReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "RTC Calendar Alram Register"]
    #[inline(always)]
    pub const fn rtc_calendar_alarm_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcCalendarAlarmReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcCalendarAlarmReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "RTC Calendar Register"]
    #[inline(always)]
    pub const fn rtc_calendar_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcCalendarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcCalendarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "RTC Control Register"]
    #[inline(always)]
    pub const fn rtc_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "RTC Event Control Register"]
    #[inline(always)]
    pub const fn rtc_event_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcEventCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcEventCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "RTC Event Flags Register"]
    #[inline(always)]
    pub const fn rtc_event_flags_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcEventFlagsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcEventFlagsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "RTC Hour Mode Register"]
    #[inline(always)]
    pub const fn rtc_hour_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcHourModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcHourModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "RTC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn rtc_interrupt_disable_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcInterruptDisableReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcInterruptDisableReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "RTC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn rtc_interrupt_enable_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcInterruptEnableReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcInterruptEnableReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "RTC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn rtc_interrupt_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcInterruptMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcInterruptMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "RTC Keep RTC Register"]
    #[inline(always)]
    pub const fn rtc_keep_rtc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcKeepRtcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcKeepRtcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "RTC Motor Event Counter Register"]
    #[inline(always)]
    pub const fn rtc_motor_event_cnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcMotorEventCntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcMotorEventCntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "RTC Motor Event Period Register"]
    #[inline(always)]
    pub const fn rtc_motor_event_period_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcMotorEventPeriodReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcMotorEventPeriodReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "RTC PDC Event Clear Register"]
    #[inline(always)]
    pub const fn rtc_pdc_event_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcPdcEventClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcPdcEventClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "RTC PDC Event Counter Register"]
    #[inline(always)]
    pub const fn rtc_pdc_event_cnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcPdcEventCntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcPdcEventCntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "RTC PDC Event Period Register"]
    #[inline(always)]
    pub const fn rtc_pdc_event_period_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcPdcEventPeriodReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcPdcEventPeriodReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "RTC Status Register"]
    #[inline(always)]
    pub const fn rtc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "RTC Time Alarm Register"]
    #[inline(always)]
    pub const fn rtc_time_alarm_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcTimeAlarmReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcTimeAlarmReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "RTC Time Register"]
    #[inline(always)]
    pub const fn rtc_time_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RtcTimeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RtcTimeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcAlarmEnableReg_SPEC;
impl crate::sealed::RegSpec for RtcAlarmEnableReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Alarm Enable Register"]
pub type RtcAlarmEnableReg = crate::RegValueT<RtcAlarmEnableReg_SPEC>;

impl RtcAlarmEnableReg {
    #[doc = "Alarm on month enable. Enable to trigger alarm when data specified in Calendar Alarm Register (M_T and M_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_mnth_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RtcAlarmEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,RtcAlarmEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alarm on date enable. Enable to trigger alarm when data specified in Calendar Alarm Register (D_T and D_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_date_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RtcAlarmEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,RtcAlarmEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alarm on hour enable. Enable to trigger alarm when data specified in Time Alarm Register (PM, HR_T and HR_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_hour_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RtcAlarmEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,RtcAlarmEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alarm on minute enable. Enable to trigger alarm when data specified in Time Alarm Register (M_T and M_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_min_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RtcAlarmEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,RtcAlarmEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alarm on second enable. Enable to trigger alarm when data specified in Time Alarm Register (S_T and S_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_sec_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcAlarmEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RtcAlarmEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alarm on hundredths of a second enable. Enable to trigger alarm when data specified in Time Alarm Register (H_T and H_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_hos_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcAlarmEnableReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RtcAlarmEnableReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcAlarmEnableReg {
    #[inline(always)]
    fn default() -> RtcAlarmEnableReg {
        <crate::RegValueT<RtcAlarmEnableReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcCalendarAlarmReg_SPEC;
impl crate::sealed::RegSpec for RtcCalendarAlarmReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Calendar Alram Register"]
pub type RtcCalendarAlarmReg = crate::RegValueT<RtcCalendarAlarmReg_SPEC>;

impl RtcCalendarAlarmReg {
    #[doc = "Date tens. Represented in BCD digit (0-3)."]
    #[inline(always)]
    pub fn rtc_cal_d_t(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        u8,
        u8,
        RtcCalendarAlarmReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            u8,
            u8,
            RtcCalendarAlarmReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Date units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_d_u(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        RtcCalendarAlarmReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            RtcCalendarAlarmReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Month tens. Represented in BCD digit (0-1)."]
    #[inline(always)]
    pub fn rtc_cal_m_t(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RtcCalendarAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,RtcCalendarAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Month units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_m_u(
        self,
    ) -> crate::common::RegisterField<
        3,
        0xf,
        1,
        0,
        u8,
        u8,
        RtcCalendarAlarmReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0xf,
            1,
            0,
            u8,
            u8,
            RtcCalendarAlarmReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RtcCalendarAlarmReg {
    #[inline(always)]
    fn default() -> RtcCalendarAlarmReg {
        <crate::RegValueT<RtcCalendarAlarmReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcCalendarReg_SPEC;
impl crate::sealed::RegSpec for RtcCalendarReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Calendar Register"]
pub type RtcCalendarReg = crate::RegValueT<RtcCalendarReg_SPEC>;

impl RtcCalendarReg {
    #[doc = "The value in this register has altered since last read. Read and clear"]
    #[inline(always)]
    pub fn rtc_cal_ch(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RtcCalendarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Century tens. Represented in BCD digit (1-2)."]
    #[inline(always)]
    pub fn rtc_cal_c_t(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Century units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_c_u(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Year tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_y_t(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Year units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_y_u(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Date tens. Represented in BCD digit (0-3)."]
    #[inline(always)]
    pub fn rtc_cal_d_t(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Date units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_d_u(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Month tens. Represented in BCD digit (0-1)."]
    #[inline(always)]
    pub fn rtc_cal_m_t(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, RtcCalendarReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Month units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_m_u(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Day of the week (arbitrary) units. Represented in BCD digit (0-7)."]
    #[inline(always)]
    pub fn rtc_day(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, RtcCalendarReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,RtcCalendarReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcCalendarReg {
    #[inline(always)]
    fn default() -> RtcCalendarReg {
        <crate::RegValueT<RtcCalendarReg_SPEC> as RegisterValue<_>>::new(536871183)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcControlReg_SPEC;
impl crate::sealed::RegSpec for RtcControlReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Control Register"]
pub type RtcControlReg = crate::RegValueT<RtcControlReg_SPEC>;

impl RtcControlReg {
    #[doc = "When this field is set high the RTC stops incrementing the calendar value."]
    #[inline(always)]
    pub fn rtc_cal_disable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RtcControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When this field is set high the RTC stops incrementing the time value."]
    #[inline(always)]
    pub fn rtc_time_disable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RtcControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcControlReg {
    #[inline(always)]
    fn default() -> RtcControlReg {
        <crate::RegValueT<RtcControlReg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcEventCtrlReg_SPEC;
impl crate::sealed::RegSpec for RtcEventCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Event Control Register"]
pub type RtcEventCtrlReg = crate::RegValueT<RtcEventCtrlReg_SPEC>;

impl RtcEventCtrlReg {
    #[doc = "0 = Event to PDC is disabled. No clear any pending event\n1 = Even to PDC is enabled"]
    #[inline(always)]
    pub fn rtc_pdc_event_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcEventCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,RtcEventCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Event to Motor is disabled\n1 = Event to Motor is enabled"]
    #[inline(always)]
    pub fn rtc_motor_event_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcEventCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RtcEventCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcEventCtrlReg {
    #[inline(always)]
    fn default() -> RtcEventCtrlReg {
        <crate::RegValueT<RtcEventCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcEventFlagsReg_SPEC;
impl crate::sealed::RegSpec for RtcEventFlagsReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Event Flags Register"]
pub type RtcEventFlagsReg = crate::RegValueT<RtcEventFlagsReg_SPEC>;

impl RtcEventFlagsReg {
    #[doc = "Alarm event flag. Indicate that alarm event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_alrm(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RtcEventFlagsReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,RtcEventFlagsReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Month rolls over event flag. Indicate that month rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_mnth(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RtcEventFlagsReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,RtcEventFlagsReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Date rolls over event flag. Indicate that date rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_date(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RtcEventFlagsReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,RtcEventFlagsReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Hour rolls over event flag. Indicate that hour rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_hour(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RtcEventFlagsReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,RtcEventFlagsReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Minute rolls over event flag. Indicate that minute rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_min(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RtcEventFlagsReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,RtcEventFlagsReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Second rolls over event flag. Indicate that second rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_sec(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcEventFlagsReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,RtcEventFlagsReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Hundredths of a second event flag. Indicate that hundredths of a second rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_hos(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcEventFlagsReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RtcEventFlagsReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcEventFlagsReg {
    #[inline(always)]
    fn default() -> RtcEventFlagsReg {
        <crate::RegValueT<RtcEventFlagsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcHourModeReg_SPEC;
impl crate::sealed::RegSpec for RtcHourModeReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Hour Mode Register"]
pub type RtcHourModeReg = crate::RegValueT<RtcHourModeReg_SPEC>;

impl RtcHourModeReg {
    #[doc = "When this field is set high the RTC operates in 12 hour clock mode; otherwise, times are in 24 hour clock format."]
    #[inline(always)]
    pub fn rtc_hms(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcHourModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RtcHourModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcHourModeReg {
    #[inline(always)]
    fn default() -> RtcHourModeReg {
        <crate::RegValueT<RtcHourModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcInterruptDisableReg_SPEC;
impl crate::sealed::RegSpec for RtcInterruptDisableReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Interrupt Disable Register"]
pub type RtcInterruptDisableReg = crate::RegValueT<RtcInterruptDisableReg_SPEC>;

impl RtcInterruptDisableReg {
    #[doc = "Interrupt on alarm disable. Disable to issue the interrupt when alarm event occurred."]
    #[inline(always)]
    pub fn rtc_alrm_int_dis(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RtcInterruptDisableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,RtcInterruptDisableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on month disable. Disable to issue the interrupt when month event occurred."]
    #[inline(always)]
    pub fn rtc_mnth_int_dis(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RtcInterruptDisableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,RtcInterruptDisableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on date disable. Disable to issue the interrupt when date event occurred."]
    #[inline(always)]
    pub fn rtc_date_int_dis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RtcInterruptDisableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,RtcInterruptDisableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "IInterrupt on hour disable. Disable to issue the interrupt when hour event occurred."]
    #[inline(always)]
    pub fn rtc_hour_int_dis(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RtcInterruptDisableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,RtcInterruptDisableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on minute disable. Disable to issue the interrupt when minute event occurred."]
    #[inline(always)]
    pub fn rtc_min_int_dis(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RtcInterruptDisableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,RtcInterruptDisableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on second disable. Disable to issue the interrupt when second event occurred."]
    #[inline(always)]
    pub fn rtc_sec_int_dis(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcInterruptDisableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,RtcInterruptDisableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on hundredths of a second disable. Disable to issue the interrupt when hundredths of a second event occurred."]
    #[inline(always)]
    pub fn rtc_hos_int_dis(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcInterruptDisableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,RtcInterruptDisableReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcInterruptDisableReg {
    #[inline(always)]
    fn default() -> RtcInterruptDisableReg {
        <crate::RegValueT<RtcInterruptDisableReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcInterruptEnableReg_SPEC;
impl crate::sealed::RegSpec for RtcInterruptEnableReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Interrupt Enable Register"]
pub type RtcInterruptEnableReg = crate::RegValueT<RtcInterruptEnableReg_SPEC>;

impl RtcInterruptEnableReg {
    #[doc = "Interrupt on alarm enable. Enable to issue the interrupt when alarm event occurred."]
    #[inline(always)]
    pub fn rtc_alrm_int_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RtcInterruptEnableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,RtcInterruptEnableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on month enable. Enable to issue the interrupt when month event occurred."]
    #[inline(always)]
    pub fn rtc_mnth_int_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RtcInterruptEnableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,RtcInterruptEnableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on date enable. Enable to issue the interrupt when date event occurred."]
    #[inline(always)]
    pub fn rtc_date_int_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RtcInterruptEnableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,RtcInterruptEnableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on hour enable. Enable to issue the interrupt when hour event occurred."]
    #[inline(always)]
    pub fn rtc_hour_int_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RtcInterruptEnableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,RtcInterruptEnableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on minute enable. Enable to issue the interrupt when minute event occurred."]
    #[inline(always)]
    pub fn rtc_min_int_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RtcInterruptEnableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,RtcInterruptEnableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on second enable. Enable to issue the interrupt when second event occurred."]
    #[inline(always)]
    pub fn rtc_sec_int_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcInterruptEnableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,RtcInterruptEnableReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Interrupt on hundredths of a second enable. Enable to issue the interrupt when hundredths of a second event occurred."]
    #[inline(always)]
    pub fn rtc_hos_int_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcInterruptEnableReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,RtcInterruptEnableReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcInterruptEnableReg {
    #[inline(always)]
    fn default() -> RtcInterruptEnableReg {
        <crate::RegValueT<RtcInterruptEnableReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcInterruptMaskReg_SPEC;
impl crate::sealed::RegSpec for RtcInterruptMaskReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Interrupt Mask Register"]
pub type RtcInterruptMaskReg = crate::RegValueT<RtcInterruptMaskReg_SPEC>;

impl RtcInterruptMaskReg {
    #[doc = "Mask alarm interrupt. It can be cleared (set) by setting corresponding bit (ALRM) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_alrm_int_msk(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, RtcInterruptMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,RtcInterruptMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IMask month interrupt. It can be cleared (set) by setting corresponding bit (MNTH) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_mnth_int_msk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, RtcInterruptMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,RtcInterruptMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Mask date interrupt. It can be cleared (set) by setting corresponding bit (DATE) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_date_int_msk(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, RtcInterruptMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,RtcInterruptMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IMask hour interrupt. It can be cleared (set) by setting corresponding bit (HOUR) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_hour_int_msk(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RtcInterruptMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,RtcInterruptMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IMask minute interrupt. It can be cleared (set) by setting corresponding bit (MIN) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_min_int_msk(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RtcInterruptMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,RtcInterruptMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IMask second interrupt. It can be cleared (set) by setting corresponding bit (SEC) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_sec_int_msk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcInterruptMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,RtcInterruptMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Mask hundredths of a second interrupt. It can be cleared (set) by setting corresponding bit (HOS) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_hos_int_msk(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcInterruptMaskReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RtcInterruptMaskReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcInterruptMaskReg {
    #[inline(always)]
    fn default() -> RtcInterruptMaskReg {
        <crate::RegValueT<RtcInterruptMaskReg_SPEC> as RegisterValue<_>>::new(127)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcKeepRtcReg_SPEC;
impl crate::sealed::RegSpec for RtcKeepRtcReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Keep RTC Register"]
pub type RtcKeepRtcReg = crate::RegValueT<RtcKeepRtcReg_SPEC>;

impl RtcKeepRtcReg {
    #[doc = "Keep RTC. When high, the time and calendar registers and any other registers which directly affect or are affected by the time and calendar registers are NOT reset when software reset is applied. When low, the software reset will reset every register except the keep RTC and control registers."]
    #[inline(always)]
    pub fn rtc_keep(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcKeepRtcReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RtcKeepRtcReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcKeepRtcReg {
    #[inline(always)]
    fn default() -> RtcKeepRtcReg {
        <crate::RegValueT<RtcKeepRtcReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcMotorEventCntReg_SPEC;
impl crate::sealed::RegSpec for RtcMotorEventCntReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Motor Event Counter Register"]
pub type RtcMotorEventCntReg = crate::RegValueT<RtcMotorEventCntReg_SPEC>;

impl RtcMotorEventCntReg {
    #[doc = "It gives the current value of the Motor event counter (0 to RTC_MOTOR_EVENT_PERIOD)"]
    #[inline(always)]
    pub fn rtc_motor_event_cnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        RtcMotorEventCntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            RtcMotorEventCntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RtcMotorEventCntReg {
    #[inline(always)]
    fn default() -> RtcMotorEventCntReg {
        <crate::RegValueT<RtcMotorEventCntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcMotorEventPeriodReg_SPEC;
impl crate::sealed::RegSpec for RtcMotorEventPeriodReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Motor Event Period Register"]
pub type RtcMotorEventPeriodReg = crate::RegValueT<RtcMotorEventPeriodReg_SPEC>;

impl RtcMotorEventPeriodReg {
    #[doc = "RTC wil send an event to motor (if RTC_MOTOR_EVENT_EN=1) every (RTC_MOTOR_EVENT_PERIOD+1)*10ms"]
    #[inline(always)]
    pub fn rtc_motor_event_period(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        RtcMotorEventPeriodReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            RtcMotorEventPeriodReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RtcMotorEventPeriodReg {
    #[inline(always)]
    fn default() -> RtcMotorEventPeriodReg {
        <crate::RegValueT<RtcMotorEventPeriodReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcPdcEventClearReg_SPEC;
impl crate::sealed::RegSpec for RtcPdcEventClearReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC PDC Event Clear Register"]
pub type RtcPdcEventClearReg = crate::RegValueT<RtcPdcEventClearReg_SPEC>;

impl RtcPdcEventClearReg {
    #[doc = "On read, PDC event is cleared"]
    #[inline(always)]
    pub fn pdc_event_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcPdcEventClearReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RtcPdcEventClearReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcPdcEventClearReg {
    #[inline(always)]
    fn default() -> RtcPdcEventClearReg {
        <crate::RegValueT<RtcPdcEventClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcPdcEventCntReg_SPEC;
impl crate::sealed::RegSpec for RtcPdcEventCntReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC PDC Event Counter Register"]
pub type RtcPdcEventCntReg = crate::RegValueT<RtcPdcEventCntReg_SPEC>;

impl RtcPdcEventCntReg {
    #[doc = "It gives the current value of the PDC event counter (0 to RTC_PDC_EVENT_PERIOD)"]
    #[inline(always)]
    pub fn rtc_pdc_event_cnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fff,
        1,
        0,
        u16,
        u16,
        RtcPdcEventCntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            RtcPdcEventCntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RtcPdcEventCntReg {
    #[inline(always)]
    fn default() -> RtcPdcEventCntReg {
        <crate::RegValueT<RtcPdcEventCntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcPdcEventPeriodReg_SPEC;
impl crate::sealed::RegSpec for RtcPdcEventPeriodReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC PDC Event Period Register"]
pub type RtcPdcEventPeriodReg = crate::RegValueT<RtcPdcEventPeriodReg_SPEC>;

impl RtcPdcEventPeriodReg {
    #[doc = "RTC wil send an event to PDC (if RTC_PDC_EVENT_EN=1) every (RTC_PDC_EVENT_PERIOD+1)*10ms"]
    #[inline(always)]
    pub fn rtc_pdc_event_period(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fff,
        1,
        0,
        u16,
        u16,
        RtcPdcEventPeriodReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            RtcPdcEventPeriodReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RtcPdcEventPeriodReg {
    #[inline(always)]
    fn default() -> RtcPdcEventPeriodReg {
        <crate::RegValueT<RtcPdcEventPeriodReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcStatusReg_SPEC;
impl crate::sealed::RegSpec for RtcStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Status Register"]
pub type RtcStatusReg = crate::RegValueT<RtcStatusReg_SPEC>;

impl RtcStatusReg {
    #[doc = "Valid Calendar Alarm. If cleared then indicates that invalid entry occurred when writing to Calendar Alarm Register."]
    #[inline(always)]
    pub fn rtc_valid_cal_alm(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, RtcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,RtcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Valid Time Alarm. If cleared then indicates that invalid entry occurred when writing to Time Alarm Register."]
    #[inline(always)]
    pub fn rtc_valid_time_alm(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, RtcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,RtcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Valid Calendar. If cleared then indicates that invalid entry occurred when writing to Calendar Register."]
    #[inline(always)]
    pub fn rtc_valid_cal(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, RtcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,RtcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Valid Time. If cleared then indicates that invalid entry occurred when writing to Time Register."]
    #[inline(always)]
    pub fn rtc_valid_time(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RtcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,RtcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcStatusReg {
    #[inline(always)]
    fn default() -> RtcStatusReg {
        <crate::RegValueT<RtcStatusReg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcTimeAlarmReg_SPEC;
impl crate::sealed::RegSpec for RtcTimeAlarmReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Time Alarm Register"]
pub type RtcTimeAlarmReg = crate::RegValueT<RtcTimeAlarmReg_SPEC>;

impl RtcTimeAlarmReg {
    #[doc = "In 12 hour clock mode, indicates PM when set."]
    #[inline(always)]
    pub fn rtc_time_pm(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RtcTimeAlarmReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hours tens. Represented in BCD digit (0-2)."]
    #[inline(always)]
    pub fn rtc_time_hr_t(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hours units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_hr_u(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Minutes tens. Represented in BCD digit (0-5)."]
    #[inline(always)]
    pub fn rtc_time_m_t(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Minutes units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_m_u(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Seconds tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_t(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Seconds units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_u(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hundredths of a second tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_t(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hundredths of a second units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_u(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RtcTimeAlarmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RtcTimeAlarmReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcTimeAlarmReg {
    #[inline(always)]
    fn default() -> RtcTimeAlarmReg {
        <crate::RegValueT<RtcTimeAlarmReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcTimeReg_SPEC;
impl crate::sealed::RegSpec for RtcTimeReg_SPEC {
    type DataType = u32;
}

#[doc = "RTC Time Register"]
pub type RtcTimeReg = crate::RegValueT<RtcTimeReg_SPEC>;

impl RtcTimeReg {
    #[doc = "The value in this register has altered since last read. Read and clear."]
    #[inline(always)]
    pub fn rtc_time_ch(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RtcTimeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "In 12 hour clock mode, indicates PM when set."]
    #[inline(always)]
    pub fn rtc_time_pm(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RtcTimeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hours tens. Represented in BCD digit (0-2)."]
    #[inline(always)]
    pub fn rtc_time_hr_t(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hours units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_hr_u(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Minutes tens. Represented in BCD digit (0-5)."]
    #[inline(always)]
    pub fn rtc_time_m_t(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Minutes units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_m_u(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Seconds tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_t(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Seconds units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_u(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hundredths of a second tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_t(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hundredths of a second units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_u(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RtcTimeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RtcTimeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RtcTimeReg {
    #[inline(always)]
    fn default() -> RtcTimeReg {
        <crate::RegValueT<RtcTimeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
