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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CHARGER registers"]
unsafe impl ::core::marker::Send for super::Charger {}
unsafe impl ::core::marker::Sync for super::Charger {}
impl super::Charger {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn charger_cc_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCcChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCcChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_current_param_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCurrentParamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCurrentParamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_current_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCurrentStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCurrentStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_cv_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCvChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCvChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_error_irq_clr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerErrorIrqClrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerErrorIrqClrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_error_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerErrorIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerErrorIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_error_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerErrorIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerErrorIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_jeita_current2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaCurrent2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaCurrent2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_jeita_current_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaCurrentReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaCurrentReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_jeita_v_charge_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVChargeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVChargeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_jeita_v_ovp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVOvpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVOvpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_jeita_v_precharge_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVPrechargeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVPrechargeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_jeita_v_replenish_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVReplenishReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVReplenishReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_lock_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerLockReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerLockReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_pre_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerPreChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerPreChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_pwr_up_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerPwrUpTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerPwrUpTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_state_irq_clr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStateIrqClrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStateIrqClrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_state_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStateIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStateIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_state_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStateIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStateIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_swlock_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerSwlockReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerSwlockReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_tbat_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTbatCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTbatCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_tbat_mon_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTbatMonTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTbatMonTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_tdie_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTdieCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTdieCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_tempset2_param_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTempset2ParamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTempset2ParamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_tempset_param_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTempsetParamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTempsetParamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_thot_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerThotCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerThotCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_total_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTotalChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTotalChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_vbat_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerVbatCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerVbatCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_voltage_param_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerVoltageParamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerVoltageParamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_voltage_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerVoltageStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerVoltageStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_vovp_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerVovpCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerVovpCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCcChargeTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerCcChargeTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerCcChargeTimerReg = crate::RegValueT<ChargerCcChargeTimerReg_SPEC>;

impl ChargerCcChargeTimerReg {
    #[inline(always)]
    pub fn cc_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7fff,
        1,
        0,
        u16,
        u16,
        ChargerCcChargeTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x7fff,
            1,
            0,
            u16,
            u16,
            ChargerCcChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn max_cc_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        ChargerCcChargeTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            ChargerCcChargeTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerCcChargeTimerReg {
    #[inline(always)]
    fn default() -> ChargerCcChargeTimerReg {
        <crate::RegValueT<ChargerCcChargeTimerReg_SPEC> as RegisterValue<_>>::new(7200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCtrlReg_SPEC;
impl crate::sealed::RegSpec for ChargerCtrlReg_SPEC {
    type DataType = u32;
}

pub type ChargerCtrlReg = crate::RegValueT<ChargerCtrlReg_SPEC>;

impl ChargerCtrlReg {
    #[inline(always)]
    pub fn eoc_trigger(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eoc_interval_check_timer(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, u8, ChargerCtrlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x3f,1,0,u8,u8,ChargerCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eoc_interval_check_thres(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, ChargerCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn replenish_mode(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pre_charge_mode(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charge_loop_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn jeita_support_disabled(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_monitor_mode(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, ChargerCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charge_timers_halt_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop_charge_timers(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ntc_low_disable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_prot_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_error_resume(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_prot_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_resume(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charge_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerCtrlReg {
    #[inline(always)]
    fn default() -> ChargerCtrlReg {
        <crate::RegValueT<ChargerCtrlReg_SPEC> as RegisterValue<_>>::new(4156024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCurrentParamReg_SPEC;
impl crate::sealed::RegSpec for ChargerCurrentParamReg_SPEC {
    type DataType = u32;
}

pub type ChargerCurrentParamReg = crate::RegValueT<ChargerCurrentParamReg_SPEC>;

impl ChargerCurrentParamReg {
    #[inline(always)]
    pub fn i_eoc_double_range(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ChargerCurrentParamReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,ChargerCurrentParamReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i_end_of_charge(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        ChargerCurrentParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            ChargerCurrentParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_precharge(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerCurrentParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerCurrentParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_charge(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerCurrentParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerCurrentParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerCurrentParamReg {
    #[inline(always)]
    fn default() -> ChargerCurrentParamReg {
        <crate::RegValueT<ChargerCurrentParamReg_SPEC> as RegisterValue<_>>::new(8390)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCurrentStatusReg_SPEC;
impl crate::sealed::RegSpec for ChargerCurrentStatusReg_SPEC {
    type DataType = u32;
}

pub type ChargerCurrentStatusReg = crate::RegValueT<ChargerCurrentStatusReg_SPEC>;

impl ChargerCurrentStatusReg {
    #[inline(always)]
    pub fn i_precharge_set(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerCurrentStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerCurrentStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_charge_set(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerCurrentStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerCurrentStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerCurrentStatusReg {
    #[inline(always)]
    fn default() -> ChargerCurrentStatusReg {
        <crate::RegValueT<ChargerCurrentStatusReg_SPEC> as RegisterValue<_>>::new(133)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCvChargeTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerCvChargeTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerCvChargeTimerReg = crate::RegValueT<ChargerCvChargeTimerReg_SPEC>;

impl ChargerCvChargeTimerReg {
    #[inline(always)]
    pub fn cv_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7fff,
        1,
        0,
        u16,
        u16,
        ChargerCvChargeTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x7fff,
            1,
            0,
            u16,
            u16,
            ChargerCvChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn max_cv_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        ChargerCvChargeTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            ChargerCvChargeTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerCvChargeTimerReg {
    #[inline(always)]
    fn default() -> ChargerCvChargeTimerReg {
        <crate::RegValueT<ChargerCvChargeTimerReg_SPEC> as RegisterValue<_>>::new(7200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerErrorIrqClrReg_SPEC;
impl crate::sealed::RegSpec for ChargerErrorIrqClrReg_SPEC {
    type DataType = u32;
}

pub type ChargerErrorIrqClrReg = crate::RegValueT<ChargerErrorIrqClrReg_SPEC>;

impl ChargerErrorIrqClrReg {
    #[inline(always)]
    pub fn tbat_error_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_error_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbat_ovp_error_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn total_charge_timeout_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_charge_timeout_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_charge_timeout_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn precharge_timeout_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerErrorIrqClrReg {
    #[inline(always)]
    fn default() -> ChargerErrorIrqClrReg {
        <crate::RegValueT<ChargerErrorIrqClrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerErrorIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for ChargerErrorIrqMaskReg_SPEC {
    type DataType = u32;
}

pub type ChargerErrorIrqMaskReg = crate::RegValueT<ChargerErrorIrqMaskReg_SPEC>;

impl ChargerErrorIrqMaskReg {
    #[inline(always)]
    pub fn tbat_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbat_ovp_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn total_charge_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_charge_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_charge_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn precharge_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerErrorIrqMaskReg {
    #[inline(always)]
    fn default() -> ChargerErrorIrqMaskReg {
        <crate::RegValueT<ChargerErrorIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerErrorIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for ChargerErrorIrqStatusReg_SPEC {
    type DataType = u32;
}

pub type ChargerErrorIrqStatusReg = crate::RegValueT<ChargerErrorIrqStatusReg_SPEC>;

impl ChargerErrorIrqStatusReg {
    #[inline(always)]
    pub fn tbat_error_irq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_error_irq(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbat_ovp_error_irq(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn total_charge_timeout_irq(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_charge_timeout_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_charge_timeout_irq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn precharge_timeout_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerErrorIrqStatusReg {
    #[inline(always)]
    fn default() -> ChargerErrorIrqStatusReg {
        <crate::RegValueT<ChargerErrorIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaCurrent2Reg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaCurrent2Reg_SPEC {
    type DataType = u32;
}

pub type ChargerJeitaCurrent2Reg = crate::RegValueT<ChargerJeitaCurrent2Reg_SPEC>;

impl ChargerJeitaCurrent2Reg {
    #[inline(always)]
    pub fn i_precharge_twarmer(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrent2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrent2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_precharge_tcooler(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrent2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrent2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_charge_twarmer(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrent2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrent2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_charge_tcooler(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrent2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrent2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaCurrent2Reg {
    #[inline(always)]
    fn default() -> ChargerJeitaCurrent2Reg {
        <crate::RegValueT<ChargerJeitaCurrent2Reg_SPEC> as RegisterValue<_>>::new(270597)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaCurrentReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaCurrentReg_SPEC {
    type DataType = u32;
}

pub type ChargerJeitaCurrentReg = crate::RegValueT<ChargerJeitaCurrentReg_SPEC>;

impl ChargerJeitaCurrentReg {
    #[inline(always)]
    pub fn i_precharge_twarm(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrentReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrentReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_precharge_tcool(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrentReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrentReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_charge_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrentReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrentReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i_charge_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaCurrentReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaCurrentReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaCurrentReg {
    #[inline(always)]
    fn default() -> ChargerJeitaCurrentReg {
        <crate::RegValueT<ChargerJeitaCurrentReg_SPEC> as RegisterValue<_>>::new(270597)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaVChargeReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaVChargeReg_SPEC {
    type DataType = u32;
}

pub type ChargerJeitaVChargeReg = crate::RegValueT<ChargerJeitaVChargeReg_SPEC>;

impl ChargerJeitaVChargeReg {
    #[inline(always)]
    pub fn v_charge_twarmer(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVChargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVChargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_charge_tcooler(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVChargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVChargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_charge_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVChargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVChargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_charge_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVChargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVChargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaVChargeReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVChargeReg {
        <crate::RegValueT<ChargerJeitaVChargeReg_SPEC> as RegisterValue<_>>::new(10914408)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaVOvpReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaVOvpReg_SPEC {
    type DataType = u32;
}

pub type ChargerJeitaVOvpReg = crate::RegValueT<ChargerJeitaVOvpReg_SPEC>;

impl ChargerJeitaVOvpReg {
    #[inline(always)]
    pub fn v_ovp_twarmer(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVOvpReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVOvpReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_ovp_tcooler(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVOvpReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVOvpReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_ovp_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVOvpReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVOvpReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_ovp_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVOvpReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVOvpReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaVOvpReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVOvpReg {
        <crate::RegValueT<ChargerJeitaVOvpReg_SPEC> as RegisterValue<_>>::new(14118262)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaVPrechargeReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaVPrechargeReg_SPEC {
    type DataType = u32;
}

pub type ChargerJeitaVPrechargeReg = crate::RegValueT<ChargerJeitaVPrechargeReg_SPEC>;

impl ChargerJeitaVPrechargeReg {
    #[inline(always)]
    pub fn v_precharge_twarmer(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVPrechargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVPrechargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_precharge_tcooler(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVPrechargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVPrechargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_precharge_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVPrechargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVPrechargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_precharge_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVPrechargeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVPrechargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaVPrechargeReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVPrechargeReg {
        <crate::RegValueT<ChargerJeitaVPrechargeReg_SPEC> as RegisterValue<_>>::new(1601927)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaVReplenishReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaVReplenishReg_SPEC {
    type DataType = u32;
}

pub type ChargerJeitaVReplenishReg = crate::RegValueT<ChargerJeitaVReplenishReg_SPEC>;

impl ChargerJeitaVReplenishReg {
    #[inline(always)]
    pub fn v_replenish_twarmer(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVReplenishReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVReplenishReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_replenish_tcooler(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVReplenishReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVReplenishReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_replenish_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVReplenishReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVReplenishReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_replenish_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerJeitaVReplenishReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerJeitaVReplenishReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaVReplenishReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVReplenishReg {
        <crate::RegValueT<ChargerJeitaVReplenishReg_SPEC> as RegisterValue<_>>::new(7993247)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerLockReg_SPEC;
impl crate::sealed::RegSpec for ChargerLockReg_SPEC {
    type DataType = u32;
}

pub type ChargerLockReg = crate::RegValueT<ChargerLockReg_SPEC>;

impl ChargerLockReg {
    #[inline(always)]
    pub fn charger_swlock_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn jeita_i_charge2_lock(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn jeita_i_charge_lock(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn jeita_v_ovp_lock(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn jeita_v_precharge_lock(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn jeita_v_charge_lock(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn total_charge_timeout_lock(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_charge_timeout_lock(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_charge_timeout_lock(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn precharge_timeout_lock(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tempset2_param_lock(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tempset_param_lock(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn current_param_lock(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn voltage_param_lock(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_test_ctrl_lock(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_ctrl_lock(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerLockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChargerLockReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerLockReg {
    #[inline(always)]
    fn default() -> ChargerLockReg {
        <crate::RegValueT<ChargerLockReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerPreChargeTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerPreChargeTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerPreChargeTimerReg = crate::RegValueT<ChargerPreChargeTimerReg_SPEC>;

impl ChargerPreChargeTimerReg {
    #[inline(always)]
    pub fn pre_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7fff,
        1,
        0,
        u16,
        u16,
        ChargerPreChargeTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x7fff,
            1,
            0,
            u16,
            u16,
            ChargerPreChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn max_pre_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        ChargerPreChargeTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            ChargerPreChargeTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerPreChargeTimerReg {
    #[inline(always)]
    fn default() -> ChargerPreChargeTimerReg {
        <crate::RegValueT<ChargerPreChargeTimerReg_SPEC> as RegisterValue<_>>::new(1800)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerPwrUpTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerPwrUpTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerPwrUpTimerReg = crate::RegValueT<ChargerPwrUpTimerReg_SPEC>;

impl ChargerPwrUpTimerReg {
    #[inline(always)]
    pub fn charger_pwr_up_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        u8,
        u8,
        ChargerPwrUpTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            u8,
            u8,
            ChargerPwrUpTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn charger_pwr_up_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        ChargerPwrUpTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            ChargerPwrUpTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerPwrUpTimerReg {
    #[inline(always)]
    fn default() -> ChargerPwrUpTimerReg {
        <crate::RegValueT<ChargerPwrUpTimerReg_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerStateIrqClrReg_SPEC;
impl crate::sealed::RegSpec for ChargerStateIrqClrReg_SPEC {
    type DataType = u32;
}

pub type ChargerStateIrqClrReg = crate::RegValueT<ChargerStateIrqClrReg_SPEC>;

impl ChargerStateIrqClrReg {
    #[inline(always)]
    pub fn cv_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<11,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<10,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_to_cc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<9,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_status_update_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_prot_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_prot_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eoc_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_to_eoc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_eoc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_cv_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn precharge_to_cc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disabled_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerStateIrqClrReg {
    #[inline(always)]
    fn default() -> ChargerStateIrqClrReg {
        <crate::RegValueT<ChargerStateIrqClrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerStateIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for ChargerStateIrqMaskReg_SPEC {
    type DataType = u32;
}

pub type ChargerStateIrqMaskReg = crate::RegValueT<ChargerStateIrqMaskReg_SPEC>;

impl ChargerStateIrqMaskReg {
    #[inline(always)]
    pub fn cv_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_to_cc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_status_update_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_prot_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_prot_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eoc_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_to_eoc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_eoc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_cv_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn precharge_to_cc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disabled_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerStateIrqMaskReg {
    #[inline(always)]
    fn default() -> ChargerStateIrqMaskReg {
        <crate::RegValueT<ChargerStateIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerStateIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for ChargerStateIrqStatusReg_SPEC {
    type DataType = u32;
}

pub type ChargerStateIrqStatusReg = crate::RegValueT<ChargerStateIrqStatusReg_SPEC>;

impl ChargerStateIrqStatusReg {
    #[inline(always)]
    pub fn cv_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_to_cc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<9,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_status_update_irq(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_prot_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_prot_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eoc_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cv_to_eoc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_eoc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cc_to_cv_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn precharge_to_cc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disabled_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerStateIrqStatusReg {
    #[inline(always)]
    fn default() -> ChargerStateIrqStatusReg {
        <crate::RegValueT<ChargerStateIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerStatusReg_SPEC;
impl crate::sealed::RegSpec for ChargerStatusReg_SPEC {
    type DataType = u32;
}

pub type ChargerStatusReg = crate::RegValueT<ChargerStatusReg_SPEC>;

impl ChargerStatusReg {
    #[inline(always)]
    pub fn ovp_events_debounce_cnt(
        self,
    ) -> crate::common::RegisterField<26, 0x7, 1, 0, u8, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x7,1,0,u8,u8,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eoc_events_debounce_cnt(
        self,
    ) -> crate::common::RegisterField<23, 0x7, 1, 0, u8, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<23,0x7,1,0,u8,u8,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_error_debounce_cnt(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_jeita_state(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_state(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_status(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn main_tbat_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tbat_hot_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdie_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbat_ovp_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn main_vbat_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn end_of_charge(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_cv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_cc_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_is_powered_up(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerStatusReg {
    #[inline(always)]
    fn default() -> ChargerStatusReg {
        <crate::RegValueT<ChargerStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerSwlockReg_SPEC;
impl crate::sealed::RegSpec for ChargerSwlockReg_SPEC {
    type DataType = u32;
}

pub type ChargerSwlockReg = crate::RegValueT<ChargerSwlockReg_SPEC>;

impl ChargerSwlockReg {
    #[inline(always)]
    pub fn swlock_fsm_state(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, ChargerSwlockReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,ChargerSwlockReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swlock_status(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerSwlockReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChargerSwlockReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerSwlockReg {
    #[inline(always)]
    fn default() -> ChargerSwlockReg {
        <crate::RegValueT<ChargerSwlockReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerTbatCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerTbatCompTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerTbatCompTimerReg = crate::RegValueT<ChargerTbatCompTimerReg_SPEC>;

impl ChargerTbatCompTimerReg {
    #[inline(always)]
    pub fn tbat_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerTbatCompTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerTbatCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbat_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerTbatCompTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerTbatCompTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerTbatCompTimerReg {
    #[inline(always)]
    fn default() -> ChargerTbatCompTimerReg {
        <crate::RegValueT<ChargerTbatCompTimerReg_SPEC> as RegisterValue<_>>::new(99)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerTbatMonTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerTbatMonTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerTbatMonTimerReg = crate::RegValueT<ChargerTbatMonTimerReg_SPEC>;

impl ChargerTbatMonTimerReg {
    #[inline(always)]
    pub fn tbat_mon_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerTbatMonTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerTbatMonTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbat_mon_interval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerTbatMonTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerTbatMonTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerTbatMonTimerReg {
    #[inline(always)]
    fn default() -> ChargerTbatMonTimerReg {
        <crate::RegValueT<ChargerTbatMonTimerReg_SPEC> as RegisterValue<_>>::new(99)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerTdieCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerTdieCompTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerTdieCompTimerReg = crate::RegValueT<ChargerTdieCompTimerReg_SPEC>;

impl ChargerTdieCompTimerReg {
    #[inline(always)]
    pub fn tdie_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerTdieCompTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerTdieCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdie_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerTdieCompTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerTdieCompTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerTdieCompTimerReg {
    #[inline(always)]
    fn default() -> ChargerTdieCompTimerReg {
        <crate::RegValueT<ChargerTdieCompTimerReg_SPEC> as RegisterValue<_>>::new(99)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerTempset2ParamReg_SPEC;
impl crate::sealed::RegSpec for ChargerTempset2ParamReg_SPEC {
    type DataType = u32;
}

pub type ChargerTempset2ParamReg = crate::RegValueT<ChargerTempset2ParamReg_SPEC>;

impl ChargerTempset2ParamReg {
    #[inline(always)]
    pub fn tbat_warmer(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerTempset2ParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerTempset2ParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbat_warm(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerTempset2ParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerTempset2ParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbat_cool(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerTempset2ParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerTempset2ParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbat_cooler(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerTempset2ParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerTempset2ParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerTempset2ParamReg {
    #[inline(always)]
    fn default() -> ChargerTempset2ParamReg {
        <crate::RegValueT<ChargerTempset2ParamReg_SPEC> as RegisterValue<_>>::new(14542420)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerTempsetParamReg_SPEC;
impl crate::sealed::RegSpec for ChargerTempsetParamReg_SPEC {
    type DataType = u32;
}

pub type ChargerTempsetParamReg = crate::RegValueT<ChargerTempsetParamReg_SPEC>;

impl ChargerTempsetParamReg {
    #[inline(always)]
    pub fn tdie_max(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        ChargerTempsetParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbat_hot(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerTempsetParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tbat_cold(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerTempsetParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerTempsetParamReg {
    #[inline(always)]
    fn default() -> ChargerTempsetParamReg {
        <crate::RegValueT<ChargerTempsetParamReg_SPEC> as RegisterValue<_>>::new(16330)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerThotCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerThotCompTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerThotCompTimerReg = crate::RegValueT<ChargerThotCompTimerReg_SPEC>;

impl ChargerThotCompTimerReg {
    #[inline(always)]
    pub fn thot_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerThotCompTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerThotCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn thot_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerThotCompTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerThotCompTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerThotCompTimerReg {
    #[inline(always)]
    fn default() -> ChargerThotCompTimerReg {
        <crate::RegValueT<ChargerThotCompTimerReg_SPEC> as RegisterValue<_>>::new(99)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerTotalChargeTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerTotalChargeTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerTotalChargeTimerReg = crate::RegValueT<ChargerTotalChargeTimerReg_SPEC>;

impl ChargerTotalChargeTimerReg {
    #[inline(always)]
    pub fn total_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        ChargerTotalChargeTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            ChargerTotalChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn max_total_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        ChargerTotalChargeTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ChargerTotalChargeTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerTotalChargeTimerReg {
    #[inline(always)]
    fn default() -> ChargerTotalChargeTimerReg {
        <crate::RegValueT<ChargerTotalChargeTimerReg_SPEC> as RegisterValue<_>>::new(16200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerVbatCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerVbatCompTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerVbatCompTimerReg = crate::RegValueT<ChargerVbatCompTimerReg_SPEC>;

impl ChargerVbatCompTimerReg {
    #[inline(always)]
    pub fn vbat_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerVbatCompTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerVbatCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vbat_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerVbatCompTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerVbatCompTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerVbatCompTimerReg {
    #[inline(always)]
    fn default() -> ChargerVbatCompTimerReg {
        <crate::RegValueT<ChargerVbatCompTimerReg_SPEC> as RegisterValue<_>>::new(99)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerVoltageParamReg_SPEC;
impl crate::sealed::RegSpec for ChargerVoltageParamReg_SPEC {
    type DataType = u32;
}

pub type ChargerVoltageParamReg = crate::RegValueT<ChargerVoltageParamReg_SPEC>;

impl ChargerVoltageParamReg {
    #[inline(always)]
    pub fn v_ovp(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_replenish(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_precharge(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_charge(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerVoltageParamReg {
    #[inline(always)]
    fn default() -> ChargerVoltageParamReg {
        <crate::RegValueT<ChargerVoltageParamReg_SPEC> as RegisterValue<_>>::new(13242923)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerVoltageStatusReg_SPEC;
impl crate::sealed::RegSpec for ChargerVoltageStatusReg_SPEC {
    type DataType = u32;
}

pub type ChargerVoltageStatusReg = crate::RegValueT<ChargerVoltageStatusReg_SPEC>;

impl ChargerVoltageStatusReg {
    #[inline(always)]
    pub fn v_ovp_set(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_replenish_set(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_precharge_set(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn v_charge_set(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVoltageStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVoltageStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerVoltageStatusReg {
    #[inline(always)]
    fn default() -> ChargerVoltageStatusReg {
        <crate::RegValueT<ChargerVoltageStatusReg_SPEC> as RegisterValue<_>>::new(14283240)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerVovpCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerVovpCompTimerReg_SPEC {
    type DataType = u32;
}

pub type ChargerVovpCompTimerReg = crate::RegValueT<ChargerVovpCompTimerReg_SPEC>;

impl ChargerVovpCompTimerReg {
    #[inline(always)]
    pub fn ovp_interval_check_timer(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVovpCompTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVovpCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vbat_ovp_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerVovpCompTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerVovpCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovp_interval_check_thres(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3f,
        1,
        0,
        u8,
        u8,
        ChargerVovpCompTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3f,
            1,
            0,
            u8,
            u8,
            ChargerVovpCompTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vbat_ovp_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChargerVovpCompTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChargerVovpCompTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerVovpCompTimerReg {
    #[inline(always)]
    fn default() -> ChargerVovpCompTimerReg {
        <crate::RegValueT<ChargerVovpCompTimerReg_SPEC> as RegisterValue<_>>::new(64611)
    }
}
