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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:54:07 +0000

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
    #[doc = "Maximum CC-charge time limit register"]
    #[inline(always)]
    pub const fn charger_cc_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCcChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCcChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Charger main control register"]
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

    #[doc = "Charger current settings register"]
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

    #[doc = "Maximum CV-charge time limit register"]
    #[inline(always)]
    pub const fn charger_cv_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCvChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCvChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Interrupt clear register of Charger Error IRQs"]
    #[inline(always)]
    pub const fn charger_error_irq_clr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerErrorIrqClrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerErrorIrqClrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Mask register of Charger Error IRQs"]
    #[inline(always)]
    pub const fn charger_error_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerErrorIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerErrorIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Status register of Charger Error IRQs"]
    #[inline(always)]
    pub const fn charger_error_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerErrorIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerErrorIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "JEITA-compliant current settings register"]
    #[inline(always)]
    pub const fn charger_jeita_current_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaCurrentReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaCurrentReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "JEITA-compliant Charge voltage settings register"]
    #[inline(always)]
    pub const fn charger_jeita_v_charge_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVChargeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVChargeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "JEITA-compliant OVP settings register"]
    #[inline(always)]
    pub const fn charger_jeita_v_ovp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVOvpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVOvpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "JEITA-compliant Pre-Charge voltage settings register"]
    #[inline(always)]
    pub const fn charger_jeita_v_precharge_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVPrechargeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVPrechargeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "JEITA-compliant Replenish settings register"]
    #[inline(always)]
    pub const fn charger_jeita_v_replenish_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerJeitaVReplenishReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerJeitaVReplenishReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Maximum pre-charge time limit register"]
    #[inline(always)]
    pub const fn charger_pre_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerPreChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerPreChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Charger power-up (settling) timer"]
    #[inline(always)]
    pub const fn charger_pwr_up_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerPwrUpTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerPwrUpTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Interrupt clear register of Charger FSM IRQs"]
    #[inline(always)]
    pub const fn charger_state_irq_clr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStateIrqClrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStateIrqClrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Mask register of Charger FSM IRQs"]
    #[inline(always)]
    pub const fn charger_state_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStateIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStateIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Status register of Charger FSM IRQs"]
    #[inline(always)]
    pub const fn charger_state_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStateIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStateIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Charger main status register"]
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

    #[doc = "Battery temperature (main) comparator timer"]
    #[inline(always)]
    pub const fn charger_tbat_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTbatCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTbatCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Battery temperature monitor interval timer"]
    #[inline(always)]
    pub const fn charger_tbat_mon_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTbatMonTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTbatMonTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Die temperature comparator timer register"]
    #[inline(always)]
    pub const fn charger_tdie_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTdieCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTdieCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Charger battery temperature settings register"]
    #[inline(always)]
    pub const fn charger_tempset_param_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTempsetParamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTempsetParamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Battery temperature comparator timer for \"Hot\" zone"]
    #[inline(always)]
    pub const fn charger_thot_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerThotCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerThotCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Maximum total charge time limit register"]
    #[inline(always)]
    pub const fn charger_total_charge_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerTotalChargeTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerTotalChargeTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Main Vbat comparator timer register"]
    #[inline(always)]
    pub const fn charger_vbat_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerVbatCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerVbatCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Charger voltage settings register"]
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

    #[doc = "Vbat OVP comparator timer register"]
    #[inline(always)]
    pub const fn charger_vovp_comp_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerVovpCompTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerVovpCompTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
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
#[doc = "Maximum CC-charge time limit register"]
pub type ChargerCcChargeTimerReg = crate::RegValueT<ChargerCcChargeTimerReg_SPEC>;

impl ChargerCcChargeTimerReg {
    #[doc = "Returns the current value of the CC-Charge timeout counter, running at a 1Hz clock. The range of the specific timer is identical to the one of the Pre-Charge and the CV-Charge timers, so it may count up to 6 hours, ranging from 0 to MAX_CC_CHARGE_TIME. It is reset to 0 when the Charger\'s FSM is either in DISABLED or in END_OF_CHARGE state."]
    #[inline(always)]
    pub fn cc_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7fff,
        1,
        0,
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
            ChargerCcChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the maximum time (measured in ticks of the Charger\'s 1Hz clock) allowed for the CC (Constant Current) charging stage. If this is exceeded, a CC charge time-out error will be captured by the Charger\'s control unit and its FSM will move to the ERROR state. In order to exit this state and re-start charging, the CHARGER_RESUME bit-field of CHARGER_CTRL_REG must be set.\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn max_cc_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
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
#[doc = "Charger main control register"]
pub type ChargerCtrlReg = crate::RegValueT<ChargerCtrlReg_SPEC>;

impl ChargerCtrlReg {
    #[doc = "The specific bit-field determines the current state of the timer used to periodically check the End-of-Charge signal, as soon as the Charger\'s FSM is either in CC_CHARGE or CV_CHARGE state. Thus, as soon as the Charger\'s FSM enters the CC_CHARGE state:\n  - The timer starts increasing when a positive edge detection on End-of-Charge signal occurs.\n  - It keeps increasing until reaching the programmed EOC_INTERVAL_CHECK_THRES value, if and only if there is no detection of a negative edge on End-of-Charge signal. If this happens, the timer resets and starts over with a new End-of-Charge positive edge.\n  - The timer also resets after having reached its programmed threshold or when the Charger\'s FSM next state is END_OF_CHARGE. This happens only after having found End-of-Charge signal asserted for 4 consecutive checks and provided that the specific signal has not de-asserted during the timer\'s interval.\nNote: It must be noted that out of these two states, the specific timer is kept to zero. It is also noted that this timer runs at the 1Mhz clock of the Charger\'s block and its value always ranges from 0 to the EOC_INTERVAL_CHECK_THRES value set in the respective bit-field of CHARGER_CTRL_REG."]
    #[inline(always)]
    pub fn eoc_interval_check_timer(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, ChargerCtrlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x3f,1,0,u8, ChargerCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "This bit-field determines the periodic interval of checking the End-of-Charge signal, when the Charger\'s FSM is either in CC_CHARGE or in CV_CHARGE state. To implement this, a dedicated timer has been used, counting from zero up to the value programmed into this bit-field (see also EOC_INTERVAL_CHECK_TIMER field\'s description).\nAs soon as this timer reaches the programmed value, the End-of-Charge signal is sampled and depending on its status (high or low), another counter, keeping the number of consecutive End-of-Charge events, is increased or not. See also the description of the EOC_DEBOUNCE_CNT bit-field of CHARGER_STATUS_REG, for this counter.\nNote: The specific bit-field should always be programmed to a non-zero value."]
    #[inline(always)]
    pub fn eoc_interval_check_thres(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, ChargerCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8, ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When this bit-field is set and the Charger\'s FSM is in the BYPASSED state (thus, in Bypass mode), the internal multiplexer inside the digital part of the charger selects the Replenish, instead of the Pre-charge setting to be driven to the main Vbat comparator of the Charger\'s analogue circuitry. By this way, SW can read the respective analogue comparator\'s output in CHARGER_STATUS_REG (bit-field MAIN_VBAT_COMP_OUT), after the battery\'s volrtage has reached the End-of-Charge level, and determine if the battery voltage has dropped below the Replenish level, re-starting the battery charging accordingly.\nNote: When the Charger\'s FSM is active and operational, this bit-field is don\'t care and the FSM determines which level (Pre-charge or Replenish) will be selected and driven to the analogue, depending on the current state. It is also noted that the supported Pre-charge and Replenish levels can be viewed in the respective bit-fields defined in CHARGER_VOLTAGE_PARAM_REG register."]
    #[inline(always)]
    pub fn replenish_mode(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables a signal of the same name with the bit-field, driven from the Charger\'s digital part towards the analogue circuitry, in order to determine the current in Pre-Charge mode. If the Charger\'s FSM is active and operational, the specific bit-field is don\'t care. Hence, it is considered only when the Charger\'s FSM has reached the BYPASSED state (thus, in Bypass mode).\nWith the Charger\'s FSM being bypassed, SW should take over control and set the specific bit-field, in order to deliver the Pre-Charge instead of the normal Charge current to the Charger\'s analogue circuitry, during the Pre-Charge phase.\nNote: See also the description of CHARGER_CURRENT_PARAM_REG register for the Pre-Charge and normal Charge current levels supported."]
    #[inline(always)]
    pub fn pre_charge_mode(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field disables charging, provided that the Charger\'s FSM has switched to the BYPASSED state. This is possible only by setting the CHARGER_BYPASS bit-field of this register.\nThus, as soon as the Charger\'s FSM is bypassed, the respective signal driven by the FSM is overruled by this bit-field, making the analogue part of the Charger controllable also in this mode. If the Charger\'s FSM is not bypassed, this bit-field is don\'t care."]
    #[inline(always)]
    pub fn charge_loop_hold(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Charger\'s JEITA FSM monitoring the battery temperature checks also if battery temperature is in the Warm or Cool zones.\nIn that case, it updates accordingly all the Charger\'s voltage levels (Charge, Pre-Charge, Replenish and OVP) programmed in CHARGER_VOLTAGE_PARAM_REG, as well as the charge and pre-charge current settings of CHARGER_CURRENT_PARAM_REG, depending on the temperature zone determined by the analogue circuitry of the Charger (see also the JEITA registers of the Charger\'s register file for the Voltage/Current levels in Warm and Cool temperature zones).\n1 = Charger\'s JEITA FSM monitoring the battery temperature checks only if battery temperature is either in the Hot or Cold zones. In that case, it notifies the main Charger FSM to stop charging automatically, when in Hot zone. The same will happen also for the case of Cold, unless the NTC_LOW_DISABLE bit-field of CHARGER_CTRL_REG is set.\nNote : It is not recommended to have the specific bit-field kept to \'0\' (and thus the JEITA support enabled), if at the same time the bit-field TBAT_PROT_ENABLE of the same register is also \'0\'. Thus, JEITA support should be coupled with the Battery\'s temperature protection."]
    #[inline(always)]
    pub fn jeita_support_disabled(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Battery temperature pack monitoring modes, according to the following encoding:\n00 = Battery temperature state checked and updated once, as soon as the charger is powered-up and settled.\n01 = Battery temperature state checked periodically, depeding on TBAT_MON_TIMER_REG.TBAT_MON_INTERVAL and provided that Charger has been powered-up and charger\'s FSM is enabled.\n10 = Battery temperature state checked periodically depending on TBAT_MON_TIMER_REG.TBAT_MON_INTERVAL, provided that Charger is powered-up and regardless if the Charger\'s FSM is enabled or not. Hence, this mode can be effective regardless of the state of CHARGE_START bit-field of CHARGER_CTRL_REG.\n11 = When selected, it freezes the Battery temperature monitor FSM, as soon as the latter reaches the CHECK_IDLE state (see also CHARGER_STATUS_REG.CHARGER_JEITA_STATE bit-field\'s description for the states of this FSM). In this mode, the monitoring of Battery temperature is possible only by checking the status of TBAT_HOT_COMP_OUT and MAIN_TBAT_COMP_OUT bit-fields of CHARGER_STATUS_REG, thus by letting SW take over monitoring. This setting may be used in conjunction with Bypass mode (by setting CHARGER_BYPASS of CHARGER_CTRL_REG), so that both charging and battery temperature status monitoring are controlled by SW."]
    #[inline(always)]
    pub fn tbat_monitor_mode(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, ChargerCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Charge timeout timers continue running when charging is disabled because of a Die or of a Battery temperature error.\n1 = Charge timeout timers are halted in case of a Die or of a Battery temperature error.\nIn that case, the global charge timer is stopped as soon as the Charger\'s FSM moves to TDIE_PROT or TBAT_PROT state. Also, either the Pre-Charge, the CC_CHARGE or the CV_CHARGE timer is also stopped, depending on the charging state of the FSM when the Die/Battery temperature error has been detected."]
    #[inline(always)]
    pub fn charge_timers_halt_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Charging is disabled when the battery temperature is found to have reached the \"COLD\" region. Therefore, the Charger\'s FSM moves directly to \"TBAT_PROT\" error and generates an IRQ to notify the system accordingly, in case the respective IRQ mask bit of CHARGER_ERROR_IRQ_MASK_REG is set. Also, CHARGER_ERROR_IRQ_STATUS_REG. TBAT_ERROR_IRQ field is updated accordingly.\n1 = Charging is allowed to continue, even when the battery temperature pack reaches the \"COLD\" region. Consequently, the FSM continues charging and no battery temperature error event is generated."]
    #[inline(always)]
    pub fn ntc_low_disable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Battery temperature protection disabled\n1 = Battery temperature protection enabled.\nCharging will be stopped in case Battery temperature reaches \"Hot\" zone. It will also be disabled when reaching \"Cold\" zone, provided that CHARGER_CTRL_REG.NTC_LOW_DISABLE is not set. This is handled by the Charger\'s FSM, which moves directly to the respective error state (TBAT_PROT), also generating an Error IRQ if the respective IRQ mask bit is set (see also CHARGER_ERROR_IRQ_MASK_REG)."]
    #[inline(always)]
    pub fn tbat_prot_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = FSM will not resume from a Die temperature error. Consequently, its state will be staying to \"TDIE_PROT\", for as long as this bit-field is kept low, regardless of the status of the die tempeture comparator. Also, disabling the specific bit-field will reset the Die temperature error debounce counter, when the Charger\'s FSM is in TDIE_PROT state (so when a Die temperature error has been already detected) and the specific counter will remain frozen to 0 until the TDIE_ERROR_RESUME bit-field is set (see also the TDIE_ERROR_DEBOUNCE_CNT bit-field of CHARGER_STATUS_REG).\n1 = FSM will resume from a Die temperature error, as soon as the respective analogue compator confirms that die temperature is again below the maximum allowed level.\nIt is noted that the maximum Die temperature level is programmable via the CHARGER_TEMPSET_PARAM_REG register\'s respective bit-field (T_DIE_MAX)."]
    #[inline(always)]
    pub fn tdie_error_resume(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Die temperature protection is disabled, thus charging will not be disabled by the Charger\'s FSM in case of a Die temperature error.\n1 = Die temperature protection is enabled, thus the Charger\'s FSM will move to \"TDIE_PROT\" state, disabling charging at the same time.\nIt is noted that the Die temperature error event will be logged in the respective status bit of CHARGER_IRQ_ERROR_STATUS_REG and an IRQ will be generated, if and only if the corresponding mask bit of CHARGER_IRQ_MASK_REG is set."]
    #[inline(always)]
    pub fn tdie_prot_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Charger\'s FSM is not enable to resume from a charge timeout error or a Vbat OVP (Over-Voltage Protection) error. Consequently, FSM stays in \"ERROR\" state.\n1 = Charger\'s FSM will resume from a charge timeout or from an OVP error, thus its state will move from \"ERROR\" to \"DISABLED\" state, so that the charge cycle starts-over.\nIt is noted that in the case of a Vbat OVP error, the FSM will leave \"ERROR\" state, as soon as the Vbat comparator for the OVP level shows that Vbat is again OK (so lower than the OVP setting)."]
    #[inline(always)]
    pub fn charger_resume(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Charger\'s FSM is active and running, notifying SW upon switching between its states\n1 = Charger\'s FSM is bypassed, so its state stays to \"BYPASS\", so SW should take over the monitoring of the battery voltage and control of the charger."]
    #[inline(always)]
    pub fn charger_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Charger\'s FSM is disabled, FSM stays at \"DISABLED\" state\n1 = Charger\'s FSM is enabled, so FSM\'s state can move from DISABLED to the actual charge states, starting from \"PRE_CHARGE\"."]
    #[inline(always)]
    pub fn charge_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ChargerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Charger\'s analogue circuitry is powered-down\n1 = Charger\'s analogue circuitry is being powered-up and will be available after a certain settling time (in ms).\nAs soon as this bit-field is set, the Charger\'s FSM waits for this settling time, before proceeding into DISABLED state, where it checks the Vbat level, as well as the Die temperature and the Battery temperature states. This is mandatory, before the actual charging begins, so before the FSM moves to PRE_CHARGE state.\nIt is finally noted that the settling time is configurable via CHARGER_PWR_UP_TIMER_REG, counting with the 1Khz clock.\nNote: The Charger clocks must have been enabled first, by setting the CLK_SYS_REG\\[CLK_CHG_EN\\] bit-field to \'1\', in order to let the FSM proceed."]
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
        <crate::RegValueT<ChargerCtrlReg_SPEC> as RegisterValue<_>>::new(4153976)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCurrentParamReg_SPEC;
impl crate::sealed::RegSpec for ChargerCurrentParamReg_SPEC {
    type DataType = u32;
}
#[doc = "Charger current settings register"]
pub type ChargerCurrentParamReg = crate::RegValueT<ChargerCurrentParamReg_SPEC>;

impl ChargerCurrentParamReg {
    #[doc = "When set, the specific bit-field enables the doubling of the ( percent) range of End-of-Charge current setting. Consequently, the default lower and upper limits of 6 percent of I_CHARGE (value 0x0 of I_END_OF_CHARGE bit-field) and 20 percent (value 0x7 of the same bit-field) are increased to 12 percent and 40 percent respectively, as soon as the I_EOC_DOUBLE_RANGE field is set."]
    #[inline(always)]
    pub fn i_eoc_double_range(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ChargerCurrentParamReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,ChargerCurrentParamReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "End-of-Charge current setting, ranging from 4 percent(\"000\") to 14.5 percent (\"111\") of the charge current set, with a step\nsize of 1.5 percent, as follows (when I_EOC_DOUBLE_RANGE = 0):\n000 : 4 percent\n001 : 5.5 percent\n010 : 7 percent\n011 : 8.5 percent\n100 : 10 percent\n101 : 11.5 percent\n110 : 13 percent\n111 : 14.5 percent\n\nWhen I_EOC_DOUBLE_RANGE = 1, The range is:\n000 : 8 percent\n001 : 11.5 percent\n010 : 15 percent\n011 : 18.5 percent\n100 : 22 percent\n101 : 25.5 percent\n110 : 29 percent\n111 : 32.5 percent"]
    #[inline(always)]
    pub fn i_end_of_charge(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
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
            ChargerCurrentParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the Pre-Charge current, in mA, ranging from 0.5 to 56mA, according to the following encoding:\n0   :  0.5 mA\n1   :  1  mA\n2   :  1.5mA\n3   :  2 mA\n4   :  2.5mA\n5   :  3  mA\n6   :  3.5mA\n7   :  4  mA\n8   :  4.5mA\n9   :  5  mA\n10 :  5.5mA\n11 :  6  mA\n12 :  6.5mA\n13 :  7  mA\n14 :  7.5mA\n15 :  8  mA\n16 :  9  mA\n17 : 10 mA\n18 : 11 mA\n19 : 12 mA\n20 : 13 mA\n21 : 14 mA\n22 : 15 mA\n23 : 16 mA\n24 : 17 mA\n25 : 18 mA\n26 : 19 mA\n27 : 20 mA\n28 : 21 mA\n29 : 22 mA\n30 : 23 mA\n31 : 24 mA\n32 : 26 mA\n33 : 28 mA\n34 : 30 mA\n35 : 32 mA\n36 : 34 mA\n37 : 36 mA\n38 : 38 mA\n39 : 40 mA\n40 : 42 mA\n41 : 44 mA\n42 : 46 mA\n43 : 48 mA\n44 : 50 mA\n45 : 52 mA\n46 : 54 mA\n47 : 56 mA\n48 : 56 mA\n49 : 56 mA\n50 : 56 mA\n51 : 56 mA\n52 : 56 mA\n53 : 56 mA\n54 : 56 mA\n55 : 56 mA\n56 : 56 mA\n57 : 56 mA\n58 : 56 mA\n59 : 56 mA\n60 : 56 mA\n61 : 56 mA\n62 : 56 mA\n63 : 56 mA"]
    #[inline(always)]
    pub fn i_precharge(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
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
            ChargerCurrentParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the charge current range, in mA. The range is from 5mA to 560mA, according to the following encoding:\n0   :   5 mA\n1   :  10 mA\n2   :  15 mA\n3   :  20 mA\n4   :  25 mA\n5   :  30 mA\n6   :  35 mA\n7   :  40 mA\n8   :  45 mA\n9   :  50 mA\n10 :  55 mA\n11 :  60 mA\n12 :  65 mA\n13 :  70 mA\n14 :  75 mA\n15 :  80 mA\n16 :  90 mA\n17 : 100 mA\n18 : 110 mA\n19 : 120 mA\n20 : 130 mA\n21 : 140 mA\n22 : 150 mA\n23 : 160 mA\n24 : 170 mA\n25 : 180 mA\n26 : 190 mA\n27 : 200 mA\n28 : 210 mA\n29 : 220 mA\n30 : 230 mA\n31 : 240 mA\n32 : 260 mA\n33 : 280 mA\n34 : 300 mA\n35 : 320 mA\n36 : 340 mA\n37 : 360 mA\n38 : 380 mA\n39 : 400 mA\n40 : 420 mA\n41 : 440 mA\n42 : 460 mA\n43 : 480 mA\n44 : 500 mA\n45 : 520 mA\n46 : 540 mA\n47 : 560 mA\n48 : 560 mA\n49 : 560 mA\n50 : 560 mA\n51 : 560 mA\n52 : 560 mA\n53 : 560 mA\n54 : 560 mA\n55 : 560 mA\n56 : 560 mA\n57 : 560 mA\n58 : 560 mA\n59 : 560 mA\n60 : 560 mA\n61 : 560 mA\n62 : 560 mA\n63 : 560 mA\nNote: It has to be noted that the specific values correspond to the normal battery temperature zone. However, the specific register field may be updated by the JEITA FSM (which checks the battery temperature either once or periodically), in order to adapt the Charge current to the new battery temperature zone (see also CHARGER_CTRL_REG.TBAT_MONITOR_MODE field as well). This is valid also for the Pre-Charge current field of this register and provided that JEITA support is enabled in CHARGER_CTRL_REG.\nConsequently, in that case the register return the Charge current settings that abide to the JEITA requirements for the battery (either COOL, WARM or NORMAL)."]
    #[inline(always)]
    pub fn i_charge(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
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
pub struct ChargerCvChargeTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerCvChargeTimerReg_SPEC {
    type DataType = u32;
}
#[doc = "Maximum CV-charge time limit register"]
pub type ChargerCvChargeTimerReg = crate::RegValueT<ChargerCvChargeTimerReg_SPEC>;

impl ChargerCvChargeTimerReg {
    #[doc = "Returns the current value of the CV-Charge timeout counter, running at a 1Hz clock. The range of the specific timer is identical to the one of the Pre-Charge and the CC-Charge timers, so it may count up to 6 hours, ranging from 0 to MAX_CV_CHARGE_TIME. It is reset to 0 when the Charger\'s FSM is either in DISABLED or in END_OF_CHARGE state."]
    #[inline(always)]
    pub fn cv_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7fff,
        1,
        0,
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
            ChargerCvChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the maximum time (measured in ticks of the Charger\'s 1Hz clock) allowed for the CV (Constant Voltage) charging stage. If this is exceeded, a CV charge time-out error will be captured by the Charger\'s control unit and its FSM will move to the ERROR state. In order to exit this state and re-start charging, the CHARGER_RESUME bit-field of CHARGER_CTRL_REG must be set.\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn max_cv_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
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
#[doc = "Interrupt clear register of Charger Error IRQs"]
pub type ChargerErrorIrqClrReg = crate::RegValueT<ChargerErrorIrqClrReg_SPEC>;

impl ChargerErrorIrqClrReg {
    #[doc = "Writing a 1 will reset the respective Charger\'s Error IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn tbat_error_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s Error IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn tdie_error_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s Error IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn vbat_ovp_error_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s Error IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn total_charge_timeout_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s Error IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cv_charge_timeout_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s Error IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cc_charge_timeout_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerErrorIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerErrorIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s Error IRQ status bit ; writing a 0 will have no effect"]
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
#[doc = "Mask register of Charger Error IRQs"]
pub type ChargerErrorIrqMaskReg = crate::RegValueT<ChargerErrorIrqMaskReg_SPEC>;

impl ChargerErrorIrqMaskReg {
    #[doc = "When set, it enables the generation of Battery temperature IRQs.The IRQ is generated as soon as the JEITA FSM detects that the battery temperature is either in the \"Hot\" or in the \"Cold\" temperature region, by sampling the respective comparators\' output."]
    #[inline(always)]
    pub fn tbat_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it enables the generation of Die temperature error IRQs. The IRQ is generated as soon as a Die temperature error is captured, so as soon as the Charger\'s FSM moves to the TDIE_PROT state. For this to happen, the Die temperature comparator should indicate that Die temperature has exceeded the limit defined in CHARGER_TEMPSET_PARAM_REG.TDIE_MAX."]
    #[inline(always)]
    pub fn tdie_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it enables the generation of VBAT_OVP IRQs. The IRQ is generated as soon as the dedicated Vbat comparator shows that Vbat has exceeded the OVP level and the Charger\'s FSM has switched to the respective error state (\"ERROR\")."]
    #[inline(always)]
    pub fn vbat_ovp_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it enables the total charge timeout IRQs. The IRQ is generated as soon as the Charger\'s global charge timer expires, reaching 0."]
    #[inline(always)]
    pub fn total_charge_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it enables the CV charge timeout IRQs. The IRQ is generated as soon as the Charger\'s state timer expires, reaching 0 when the FSM is in the CV_CHARGE state."]
    #[inline(always)]
    pub fn cv_charge_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it enables the CC charge timeout IRQs. The IRQ is generated as soon as the Charger\'s state timer, expires, reaching 0."]
    #[inline(always)]
    pub fn cc_charge_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerErrorIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerErrorIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, it enables the Pre-Charge timeout IRQs. The IRQ is generated as soon as the Charger\'s state timer expires, reaching 0."]
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
#[doc = "Status register of Charger Error IRQs"]
pub type ChargerErrorIrqStatusReg = crate::RegValueT<ChargerErrorIrqStatusReg_SPEC>;

impl ChargerErrorIrqStatusReg {
    #[doc = "0 = No Battery temperature error IRQ event is captured, so charging may continue\n1 = A Battery temperature error IRQ event has been captured, declaring that the Charger\'s FSM has moved to the respective error state (TBAT_PROT).\nNote : The status bit is updated automatically when the Battery temperature is detected to be either in the HOT or in the COLD zone, regardless of the state of the respective IRQ mask bit."]
    #[inline(always)]
    pub fn tbat_error_irq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No Die temperature error IRQ events have been captured, so charging may continue\n1 = A Die temperature error IRQ event is captured, declaring that the Charger\'s FSM has switched to the respective error state (TDIE_PROT) and charging will be automatically stopped.\nNote : The status bit is updated automatically when a Die temperature error is detected, thus when the die temperature is found to have exceeded the programmed level, regardless of the stae of the respective IRQ mask bit."]
    #[inline(always)]
    pub fn tdie_error_irq(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Vbat has not exceeded the Over-Voltage Protection (OVP) level, so charging may continue\n1 = Vbat has exceeded the Over-Voltage level, thus an OVP error event has been captured. The Charger\'s FSM switches to the respective error state (ERROR) as soon as the OVP event is captured by the digital part of the Charger and charging will be automatically stopped."]
    #[inline(always)]
    pub fn vbat_ovp_error_irq(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Total charge time counter has not yet reached the maximum charge time (set in CHARGER_TOTAL_CHARGE_TIME_REG)\n1 = Total charge time counter has reached the maximum charge time programmed. The Charger\'s FSM will move to the respective error state (ERROR) and charging will be automatically stopped, as soon as the specific event is captured."]
    #[inline(always)]
    pub fn total_charge_timeout_irq(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = State charge time counter has not yet reached the maximum CV charge time (set in CHARGER_CV_CHARGE_TIME_REG)\n1 = Total charge time counter has reached the maximum CV charge time programmed. The Charger\'s FSM will move to the respective error state (ERROR) and charging will be automatically stopped, as soon as the specific event is captured."]
    #[inline(always)]
    pub fn cv_charge_timeout_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = State charge time counter has not yet reached the maximum CC charge time (set in CHARGER_CC_CHARGE_TIME_REG)\n1 = Total charge time counter has reached the maximum CC charge time programmed. The Charger\'s FSM will move to the respective error state (ERROR) and charging will be automatically stopped, as soon as the specific event is captured."]
    #[inline(always)]
    pub fn cc_charge_timeout_irq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerErrorIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerErrorIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = State charge time counter has not yet reached the maximum Pre-charge time (set in CHARGER_PRECHARGE_TIME_REG)\n1 = Total charge time counter has reached the maximum Pre-charge time programmed. The Charger\'s FSM will move to the respective error state (ERROR) and charging will be automatically stopped, as soon as the specific event is captured."]
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
pub struct ChargerJeitaCurrentReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaCurrentReg_SPEC {
    type DataType = u32;
}
#[doc = "JEITA-compliant current settings register"]
pub type ChargerJeitaCurrentReg = crate::RegValueT<ChargerJeitaCurrentReg_SPEC>;

impl ChargerJeitaCurrentReg {
    #[doc = "Pre-Charge current setting for the Warm battery temperature zone. Regarding the range of values of this bit-field, see also the description of I_PRECHARGE field of CHARGER_CURRENT_PARAM_REG register."]
    #[inline(always)]
    pub fn i_precharge_twarm(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
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
            ChargerJeitaCurrentReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pre-Charge current setting for the Cool battery temperature zone. Regarding the range of values of this bit-field, see also the description of I_PRECHARGE field of CHARGER_CURRENT_PARAM_REG register."]
    #[inline(always)]
    pub fn i_precharge_tcool(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
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
            ChargerJeitaCurrentReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Charge current setting for the Warm battery temperature pack zone. Regarding the range of values of this bit-field, see also the description of I_CHARGE field of CHARGER_CURRENT_PARAM_REG register."]
    #[inline(always)]
    pub fn i_charge_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
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
            ChargerJeitaCurrentReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Charge current setting for the \"COOL\" battery temperature level. Regarding the range of values of this bit-field, see also the description of I_CHARGE field of CHARGER_CURRENT_PARAM_REG register."]
    #[inline(always)]
    pub fn i_charge_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
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
#[doc = "JEITA-compliant Charge voltage settings register"]
pub type ChargerJeitaVChargeReg = crate::RegValueT<ChargerJeitaVChargeReg_SPEC>;

impl ChargerJeitaVChargeReg {
    #[doc = "Charge voltage setting for the Warm battery temperature zone. Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG register."]
    #[inline(always)]
    pub fn v_charge_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
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
            ChargerJeitaVChargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Charge voltage setting for the Cool battery temperature zone. Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG register."]
    #[inline(always)]
    pub fn v_charge_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
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
            ChargerJeitaVChargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaVChargeReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVChargeReg {
        <crate::RegValueT<ChargerJeitaVChargeReg_SPEC> as RegisterValue<_>>::new(2664)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaVOvpReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaVOvpReg_SPEC {
    type DataType = u32;
}
#[doc = "JEITA-compliant OVP settings register"]
pub type ChargerJeitaVOvpReg = crate::RegValueT<ChargerJeitaVOvpReg_SPEC>;

impl ChargerJeitaVOvpReg {
    #[doc = "VBAT Over-voltage Protection (OVP) setting for the Warm battery temperature zone.Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG."]
    #[inline(always)]
    pub fn v_ovp_twarm(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, ChargerJeitaVOvpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8, ChargerJeitaVOvpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBAT Over-voltage Protection (OVP) setting for the Cool battery temperature zone.Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG."]
    #[inline(always)]
    pub fn v_ovp_tcool(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, ChargerJeitaVOvpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, ChargerJeitaVOvpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerJeitaVOvpReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVOvpReg {
        <crate::RegValueT<ChargerJeitaVOvpReg_SPEC> as RegisterValue<_>>::new(3446)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaVPrechargeReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaVPrechargeReg_SPEC {
    type DataType = u32;
}
#[doc = "JEITA-compliant Pre-Charge voltage settings register"]
pub type ChargerJeitaVPrechargeReg = crate::RegValueT<ChargerJeitaVPrechargeReg_SPEC>;

impl ChargerJeitaVPrechargeReg {
    #[doc = "Pre-Charge voltage setting for the Warm battery temperature zone. Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG register."]
    #[inline(always)]
    pub fn v_precharge_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
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
            ChargerJeitaVPrechargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pre-Charge current setting for the Cool battery temperature zone. Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG register."]
    #[inline(always)]
    pub fn v_precharge_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
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
            ChargerJeitaVPrechargeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaVPrechargeReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVPrechargeReg {
        <crate::RegValueT<ChargerJeitaVPrechargeReg_SPEC> as RegisterValue<_>>::new(391)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerJeitaVReplenishReg_SPEC;
impl crate::sealed::RegSpec for ChargerJeitaVReplenishReg_SPEC {
    type DataType = u32;
}
#[doc = "JEITA-compliant Replenish settings register"]
pub type ChargerJeitaVReplenishReg = crate::RegValueT<ChargerJeitaVReplenishReg_SPEC>;

impl ChargerJeitaVReplenishReg {
    #[doc = "Replenish voltage setting for the Warm battery temperature zone. Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG."]
    #[inline(always)]
    pub fn v_replenish_twarm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
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
            ChargerJeitaVReplenishReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Replenish voltage setting for the Cool battery temperature zone. Regarding the range of values of this bit-field, see also the description of V_CHARGE field of CHARGER_VOLTAGE_PARAM_REG."]
    #[inline(always)]
    pub fn v_replenish_tcool(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
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
            ChargerJeitaVReplenishReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerJeitaVReplenishReg {
    #[inline(always)]
    fn default() -> ChargerJeitaVReplenishReg {
        <crate::RegValueT<ChargerJeitaVReplenishReg_SPEC> as RegisterValue<_>>::new(1951)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerPreChargeTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerPreChargeTimerReg_SPEC {
    type DataType = u32;
}
#[doc = "Maximum pre-charge time limit register"]
pub type ChargerPreChargeTimerReg = crate::RegValueT<ChargerPreChargeTimerReg_SPEC>;

impl ChargerPreChargeTimerReg {
    #[doc = "Returns the current value of the Pre-Charge timeout counter, running at a 1Hz clock. The range of the specific timer is identical to the one of the CC-Charge and the CV-Charge timers, so it may count up to 6 hours, ranging from 0 to MAX_PRE_CHARGE_TIME. It is reset to 0 when the Charger\'s FSM is either in DISABLED or in END_OF_CHARGE state."]
    #[inline(always)]
    pub fn pre_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7fff,
        1,
        0,
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
            ChargerPreChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the maximum time (measured in ticks of the Charger\'s 1Hz clock) allowed for the Pre-Charge stage. If this is exceeded, a Pre-Charge time-out error will be captured by the Charger\'s control unit and its FSM will move to the respective state (ERROR). In order to exit this state and re-start charging, the CHARGER_RESUME bit-field of CHARGER_CTRL_REG must be set.\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn max_pre_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
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
#[doc = "Charger power-up (settling) timer"]
pub type ChargerPwrUpTimerReg = crate::RegValueT<ChargerPwrUpTimerReg_SPEC>;

impl ChargerPwrUpTimerReg {
    #[doc = "Returns the current value of the charger\'s power-up timer, running with the 1Khz clock.\nNote: The specific timer is reset to the value programmed to CHARGER_PWR_UP_SETTLING bit-field, when the Charger\'s analogue circuitry has been enabled, after being disabled initially. By setting CHARGER_CTRL_REG\\[CHARGER_ENABLE\\] to \'0\', the analogue part is disabled and in order to be properly enable, SW has to wait for 1ms (one 1Khz clock period) time. The latter is is needed to ensure that the power-up timer\'s control signals in the Charger\'s digital part will be cleared when the analogue part is again enabled, so that a proper new start-up of the Charger\'s FSM is possible."]
    #[inline(always)]
    pub fn charger_pwr_up_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        ChargerPwrUpTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            ChargerPwrUpTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the charger\'s power-up (settling) time, required for the analogue circuitry of the charger. As soon as the charger is powered-on by setting the CHARGER_ENABLE bit-field of CHARGER_CTRL_REG, the charger\'s FSM loads a dedicated timer with this value and waits for this timer to expire, before proceeding to the next states.\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn charger_pwr_up_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        ChargerPwrUpTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            ChargerPwrUpTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerPwrUpTimerReg {
    #[inline(always)]
    fn default() -> ChargerPwrUpTimerReg {
        <crate::RegValueT<ChargerPwrUpTimerReg_SPEC> as RegisterValue<_>>::new(99)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerStateIrqClrReg_SPEC;
impl crate::sealed::RegSpec for ChargerStateIrqClrReg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt clear register of Charger FSM IRQs"]
pub type ChargerStateIrqClrReg = crate::RegValueT<ChargerStateIrqClrReg_SPEC>;

impl ChargerStateIrqClrReg {
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cv_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<11,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cc_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<10,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cv_to_cc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<9,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the Battery temperature status update IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn tbat_status_update_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn tbat_prot_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn tdie_prot_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn eoc_to_precharge_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cv_to_eoc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cc_to_eoc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn cc_to_cv_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
    #[inline(always)]
    pub fn precharge_to_cc_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStateIrqClrReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStateIrqClrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Writing a 1 will reset the respective Charger\'s State IRQ status bit ; writing a 0 will have no effect"]
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
#[doc = "Mask register of Charger FSM IRQs"]
pub type ChargerStateIrqMaskReg = crate::RegValueT<ChargerStateIrqMaskReg_SPEC>;

impl ChargerStateIrqMaskReg {
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from CV_CHARGE to PRE_CHARGE state."]
    #[inline(always)]
    pub fn cv_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from CC_CHARGE to PRE_CHARGE state."]
    #[inline(always)]
    pub fn cc_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from CV_CHARGE to CC_CHARGE state."]
    #[inline(always)]
    pub fn cv_to_cc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the generation of the Charger\'s state IRQ as soon as the battery temperature status is refreched by the Charger\'s Battery temperature monitor (JEITA) FSM. As soon as the specific FSM checks the current battery temperature level, it notifies the main Charger FSM that it has run and that the Battery temperature pack state is checked (and potentially refreshed with a new status)."]
    #[inline(always)]
    pub fn tbat_status_update_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the Charger\'s state IRQ generation as soon as the Charger\'s FSM switches from the Battery temperature protection state (TBAT_PROT) to PRE_CHARGE, resuming charging."]
    #[inline(always)]
    pub fn tbat_prot_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the Charger\'s state IRQ generation as soon as the Charger\'s FSM switches from the Die temperature protection state (TDIE_PROT) to PRE_CHARGE, resuming charging."]
    #[inline(always)]
    pub fn tdie_prot_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the Charger\'s State IRQ generation as soon as the Charger\'s FSM switches from END_OF_CHARGE again to PRE_CHARGE state. This happens when the Vbat voltage level is detected to be below the Replenish level set."]
    #[inline(always)]
    pub fn eoc_to_precharge_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from CV_CHARGE to END_OF_CHARGE state."]
    #[inline(always)]
    pub fn cv_to_eoc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from CC_CHARGE to END_OF_CHARGE state."]
    #[inline(always)]
    pub fn cc_to_eoc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from CC_CHARGE to CV_CHARGE state."]
    #[inline(always)]
    pub fn cc_to_cv_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from PRE_CHARGE to CC_CHARGE state.."]
    #[inline(always)]
    pub fn precharge_to_cc_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStateIrqMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStateIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When set, this bit-field enables the IRQ generation as soon as the Charger\'s FSM switches from DISABLED to PRE_CHARGE state."]
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
#[doc = "Status register of Charger FSM IRQs"]
pub type ChargerStateIrqStatusReg = crate::RegValueT<ChargerStateIrqStatusReg_SPEC>;

impl ChargerStateIrqStatusReg {
    #[doc = "0 = No transition of the Charger\'s FSM from CV_CHARGE to PRE_CHARGE state has been captured\n1 = Charger\'s FSM has switched from CV_CHARGE to PRE_CHARGE state"]
    #[inline(always)]
    pub fn cv_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from CC_CHARGE to PRE_CHARGE state has been captured\n1 = Charger\'s FSM has switched from CC_CHARGE to PRE_CHARGE state"]
    #[inline(always)]
    pub fn cc_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from CV_CHARGE to CC_CHARGE state has been captured\n1 = Charger\'s FSM has switched from CV_CHARGE to CC_CHARGE state"]
    #[inline(always)]
    pub fn cv_to_cc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<9,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No battery temperature status update event has been captured\n1 = Battery temperature pack\'s status has been checked and refreshed by the Charger\'s Battery temperature monitor FSM. Thus, the new status of the battery temperature should be checked by SW."]
    #[inline(always)]
    pub fn tbat_status_update_irq(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from TBAT_PROT to PRE_CHARGE state has been captured\n1 = Charger\'s FSM has switched from TBAT_PROT to PRE_CHARGE state, resuming charging after having recovered from a battery temperature error."]
    #[inline(always)]
    pub fn tbat_prot_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from TDIE_PROT to PRE_CHARGE state has been captured\n1 = Charger\'s FSM has switched from TDIE_PROT to PRE_CHARGE state, resuming charging after having recovered from a Die temperature error."]
    #[inline(always)]
    pub fn tdie_prot_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from END_OF_CHARGE to PRE_CHARGE state has been captured\n1 = Charger\'s FSM has switched from END_OF_CHARGE to PRE_CHARGE state"]
    #[inline(always)]
    pub fn eoc_to_precharge_irq(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from CV_CHARGE to END_OF_CHARGE state has been captured\n1 = Charger\'s FSM has switched from CV_CHARGE to END_OF_CHARGE state"]
    #[inline(always)]
    pub fn cv_to_eoc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from CC_CHARGE to END_OF_CHARGE state has been captured\n1 = Charger\'s FSM has switched from CC_CHARGE to END_OF_CHARGE state"]
    #[inline(always)]
    pub fn cc_to_eoc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from CC_CHARGE to CV_CHARGE state has been captured\n1 = Charger\'s FSM has switched from CC_CHARGE to CV_CHARGE state"]
    #[inline(always)]
    pub fn cc_to_cv_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from PRE_CHARGE to CC_CHARGE state has been captured\n1 = Charger\'s FSM has switched from PRE_CHARGE to CC_CHARGE state"]
    #[inline(always)]
    pub fn precharge_to_cc_irq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStateIrqStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStateIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = No transition of the Charger\'s FSM from DISABLED to PRE_CHARGE state has been captured\n1 = Charger\'s FSM has switched from DISABLED to PRE_CHARGE state"]
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
#[doc = "Charger main status register"]
pub type ChargerStatusReg = crate::RegValueT<ChargerStatusReg_SPEC>;

impl ChargerStatusReg {
    #[doc = "The specific bit-field returns the consecutive number of times Vbat has exceeded the programmed Over-Voltage Protection (OVP) level. It is used to determine when the Charger\'s FSM will exit any of the charging states (PRE/CC/CV_CHARGE) and will switch to the ERROR state due to an OVP error. This will happen as soon as the respective counter of OVP events reaches or exceeds a fixed number (4), similar to the approach adopted in the End-of-Charge and Die Temperature debouncing mechanisms.\nThe specific counter increases only while the Charger\'s FSM is in any of the three charging,states, the Vbat OVP interval check timer has reached the threshold set and when Vbat OVP comparator\'s output is asserted.\nNote 1 : By default, as soon as the counter reaches 4, the FSM will switch to the ERROR state and the counter will reset again. Thus, in that case the specific counter ranges from 0 to 4 and vice-versa. However, if the monitoring of Vbat OVP comparator\'s state is less frequent than 5 (4+1) times the CHARGER_OVP_COMP_TIMER_REG\\[OVP_INTERVAL_CHECK_THRES\\] and Vbat has exceeded the OVP voltage level based on the comparator\'s output signal, then this counter will exceed 4 and may overflow.\nThis will not harm, however, the detection of the OVP event, as it only increases the number of OVP event occurrences by the debounce timer, until the OVP comparator timer\'s settling time has expired. Thus, the Charger FSM will again switch to ERROR when the counter has reached or exceeded 4 (bit \\[2\\] of OVP_EVENTS_DEBOUNCE_CNT is set) and the OVP comparator\'s timer has expired.\nNote 2: See also the OVP_INTERVAL_CHECK_TIMER, OVP_INTERVAL_CHECK_THRES of CHARGER_OVP_COMP_TIMER_REG, for the debouncing mechanism of the Vbat OVP comparator\'s output."]
    #[inline(always)]
    pub fn ovp_events_debounce_cnt(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x7,1,0,u8, ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The specific bit-field returns the number of times the End-of-Charge signal has been consecutively found to be high. It is used to determine when the Charger\'s FSM will switch from CV_CHARGE to END_OF_CHARGE state, implementing a debounce mechanism on End-of-Charge signal, coming from the analogue circuitry of the Charger towards the FSM.\nThe specific counter, running with the Charger\'s 1Mhz clock:\n  - Increases after detecting that the End-of-Charge signal is high when the respective interval for the End-of-Charge check expires. This actually happens after having detected a positive edge on End-of-Charge signal, since only after that is it possible for the interval timer to start ticking.\n  - Resets to zero when End-of-Charge is seen low when the interval timer has expired or when an End-of-Charge negative edge is seen before the timer\'s expiration, starting-over.\n  - Does not count if End-of-Charge signal is seen high and either the CV_MODE signal (also driven by the analogue circutry) or the End-of-Charge signal of the previous clock cycle is seen low.\n  - Is reset when the Charger\'s FSM is not in either the CC_CHARGE or the CV_CHARGE state or after having reached \"100\"(4). This is the threshold after which the End-of-Charge signal is considered stable by the Charger\'s FSM, to switch to the END_OF_CHARGE state. Thus, in practice, the specific counter (and bit-field) ranges between 0 and 4.\nNote: See also the EOC_INTERVAL_CHECK_TIMER/THRES bit-fields of CHARGER_CTRL_REG."]
    #[inline(always)]
    pub fn eoc_events_debounce_cnt(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x7,1,0,u8, ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The specific bit-field returns the consecutive number of times the Die temperature is seen either above (for the case of an error) or below (for the case of recovering from an error) the set Die temperature level.This is performed by a counter, which is increased:\n- Each time the Die temperature comparator shows that Die temperature exceeds the set level, and while charging is active, provided that Die temperature protection is enabled. If, however, the CHARGER_CTRL_REG.TDIE_PROT_ENABLE bit-field is not set, the counter is reset and stays frozen to zero.\n- Each time the Die temperature comparator shows that Die temperature is again below the set level, and while the FSM is in the Die temperature protection error state (TDIE_PROT) and the TDIE_ERROR_RESUME bit-field of CHARGER_CTRL_REG is set. If the specific bit-field is not set, the debounce counter is reset to 0 and it is kept frozen until the FSM is again enabled to resume from Die temperature errors.\nIf the Die temperature comparator of the Charger\'s analogue circuitry shows that temperature has exceeded the programmed level for four consecutive times and charging is active, the Charger\'s FSM considers this as a Die temperature error and moves to the TDIE_PROT state, resetting the timer at the same time and of course halting charging.\nTo recover from this state and resume charging, the FSM needs to see that Die temperature is below the programmed level for four consecutive times, again, provided that the TDIE_ERROR_RESUME bit-field of CHARGER_CTRL_REG is set. As soon as this happens, the error counter is again reset and the Charger\'s FSM resumes, by moving to PRE_CHARGE state. Consequently, the counter\'s value always ranges from 0 to 4.\nNote: When the Charger\'s FSM is in BYPASSED state, then this bit-field is reset and kept frozen to zero. Consequently, the number of times Die temperature has exceeded the pre-programmed threshold should be determined by SW."]
    #[inline(always)]
    pub fn tdie_error_debounce_cnt(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x7,1,0,u8, ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Returns the state of the Charger\'s JEITA FSM. This FSM is used to update the state of the battery temperature pack, depending on the value programmed in CHARGER_CTRL_REG.TBAT_MONITOR_MODE bit-field. The encoding of the states is as follows:\n0x0 = CHECK_IDLE\n0x1 = CHECK_THOT\n0x2 = CHECK_TCOLD\n0x3 = CHECK_TWARM\n0x4 = CHECK_TCOOL\n0x5 = CHECK_TNORMAL\n0x6 = UPDATE_TBAT\nThe FSM initially is in CHECK_IDLE state and starts checking the battery\'s temperature by visiting the states that check for the respective temperature area (Hot, Cold, Warm, Cool, Normal), in this order.\nIf the battery temperature is found to be in one of the aforementioned zones, it directly moves to UPDATE_TBAT state, to update the battery temperature\'s state and notify the main FSM of the Charger about the battery temperature status, before returning to the CHECK_IDLE state. A Charger State IRQ will also be generated upon refreshing the battery temperature status (see also the description of CHARGER_STATE_IRQ_MASK_REG register)."]
    #[inline(always)]
    pub fn charger_jeita_state(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x7,1,0,u8, ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicating the state of the Charger\'s main FSM, based on the following encoding:\n0x0 = POWER_UP (Charger\'s power-up not yet set)\n0x1 = INIT (Charger is being power-up, FSM waiting for the analogue to settle)\n0x2 = DISABLED (Charger powered-up but charging not yet started)\n0x3 = PRE_CHARGE (Pre-Charge state)\n0x4 = CC_CHARGE (Constant Current state)\n0x5 = CV_CHARGE (Constant Voltage state)\n0x6 = END_OF_CHARGE (End-of-Charge state)\n0x7 = TDIE_PROT (Die temperature protection state, visited when Die temperature limit is exceeded)\n0x8 = TBAT_PROT (Battery temperature protection state, visited when Battery temperature is either COLD or HOT)\n0x9 = BYPASSED (Bypassed state, visited only when the FSM is bypassed and SW takes over control)\n0xA = ERROR (Error state, visited when a charge time-out occurs or in the case of Vbat exceeding over-voltage level)"]
    #[inline(always)]
    pub fn charger_state(
        self,
    ) -> crate::common::RegisterField<14, 0xf, 1, 0, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0xf,1,0,u8, ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Battery pack temperature status, according to the following (\"1-Hot\"-like) encoding:\n00001 : Battery temperature in COLD zone (default)\n00010 : Battery temperature in COOL zone\n00100 : Battery temperature in NORMAL zone (above COOL and below WARM zones)\n01000 : Battery temperature in WARM zone\n10000 : Battery temperature in HOT zone\nIt is noted that, according to the JEITA standard (supported if the JEITA_SUPPORT_ENABLED bit-field of CHARGER_CTRL_REG is not set) , if the battery pack temperature is in the \"HOT\" zone, charging will always be stopped. The same will happen also for the case of the COLD zone, unless the \"NTC_LOW_DISABLE\" bit-field of CHARGER_CTRL_REG is set. In that case, charging will continue.\nIt is finally noted that only the aforementioned values are available for this bit-field, since it is 1-Hot encoding based. Not more than 1 bit can be high at the same time, since this would mean that battery temperature is at two different temperature zones concurrently."]
    #[inline(always)]
    pub fn tbat_status(
        self,
    ) -> crate::common::RegisterField<9, 0x1f, 1, 0, u8, ChargerStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1f,1,0,u8, ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Returns the status of the main battery temperature comparator. This comparator by default checks if the battery temperature is in the Cold zone. However, if JEITA support is enabled and battery temperature is found to not be in either the Hot or the Cold zone, the same comparator is used to check for the Warm and Cool zones, as JEITA suggests.\nThe specific bit-field is suggested to be used in bypass mode and when the JEITA support is disabled (so when the battery temperature is checked agains the Hot and the Cold zones). In that case, the comparator checks the battery temperature agains the Cold level and its status can be as follows:\n0 = Battery temperature pack is found to be below the Cold level, so in the non-allowed Cold temperature zone. Thus, charging will be disabled, provided that the NTC_LOW_DISABLE bit-field of CHARGER_CTRL_REG is not set.\n1 = Battery temperature pack is found to be above the non-allowed Cold temperature zone. Thus, charging will be continued, provided that battery temperature will not be in the Hot zone as well.\nWhen the Charger\'s main FSM is active and JEITA is enabled, the Charger\'s digital block takes over and controls the respective comparator\'s output."]
    #[inline(always)]
    pub fn main_tbat_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Returns the status of the battery temperature comparator dedicated to the Hot temperature zone.\n0 = Battery temperature pack is found to be below the Hot zone\n1 = Battery temperature pack is found to be in the non-allowed Hot temperature zone. Thus, charging will be disabled, provided that battery temperature protection is enabled."]
    #[inline(always)]
    pub fn tbat_hot_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Die temperature is found to be below the programmed level, set in CHARGER_TEMPSET_PARAM_REG.TDIE_SET level (normal operation)\n1 = Die temperature is found to be above the set level.\nCharging will be disabled if Die temperature protection is enabled and the Die temperature is found to be above the set level four consecutive times (see also TDIE_ERROR_DEBOUNCE_CNT bit-field). In that case, the Charger\'s FSM will also move the respective error state (TDIE_PROT) and an IRQ may be generated, if the respective mask bit of CHARGER_ERROR_IRQ_MASK_REG is set."]
    #[inline(always)]
    pub fn tdie_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Vbat has not exceeded the Over-Voltage Protection (OVP) voltage limit, according to the respective analogue comparator\'s output.\n1 = Vbat is found to have exceeded the OVP voltage setting, thus charging should be disabled.\nThe OVP voltage settings are defined in CHARGER_VOLTAGE_PARAM_REG.V_OVP (for the Normal battery temperature zone), as well as in CHARGER_JEITA_V_OVP_REG (for Cool and Warm temperature zones, to comply with JEITA)."]
    #[inline(always)]
    pub fn vbat_ovp_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "This bit-field reflects the status of the main Vbat comparator residing in the analogue circuitry of the Charger.\nThis comparator is used to check Vbat against either the Pre-Charge or the Replenish voltage level, depending on what is driven by the Charger\'s digital block.\nThus, when the FSM is active, the comparator gets as reference the Replenish setting as soon as the FSM has reached the END_OF_CHARGE state. Otherwise,the Pre-Charge voltage setting is driven, including the Bypass mode.\nAccording to the above, the encoding is as follows for the case the comparator compares Vbat against the Pre-Charge level:\n0 = Vbat has not exceeded the set Pre-Charge voltage level.\n1 = Vbat has reached or exceeded the set Pre-Charge voltage level.\nFor the case the comparator compares agains the Replenish level (when the FSM has reached the END_OF_CHARGE state, so when the charging has been completed), the encoding is as follows:\n0 = Vbat has dropped below the set Replenish level, so charging will re-start and the FSM will move to the PRE_CHARGE state.\n1 = Vbat is still greater or equal to the set Replenish level, thus charging remains in hold and the FSM in END_OF_CHARGE state."]
    #[inline(always)]
    pub fn main_vbat_comp_out(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Actual charge current is above the current level programmed in I_END_OF_CHARGE field of CHARGER_CURRENT_PARAM_REG (or charger is off)\n1 = Actual charge current is below the current level programmed in I_END_OF_CHARGE bit-field of CHARGER_CURRENT_PARAM_REG."]
    #[inline(always)]
    pub fn end_of_charge(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Charger\'s voltage loop not in regulation (or Charger is off)\n1 = Charger\'s Constant Voltage (CV) mode active, voltage loop in regulation"]
    #[inline(always)]
    pub fn charger_cv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Charger\'s Current loop not in regulation (or Charger is off)\n1 = Charger\'s Constant Current (CC) mode active, current loop in regulation"]
    #[inline(always)]
    pub fn charger_cc_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = Charger is either off or it is being powered-on but the analogue ciruitry is not yet settled. The charger\'s main FSM is either in POWER_UP or INIT states.\n1 = Charger is powered-up, so its analogue ciruitry should now be settled. The Charger\'s FSM has left both power-up states (POWER_UP, INIT), so charging can start."]
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
        <crate::RegValueT<ChargerStatusReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerTbatCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerTbatCompTimerReg_SPEC {
    type DataType = u32;
}
#[doc = "Battery temperature (main) comparator timer"]
pub type ChargerTbatCompTimerReg = crate::RegValueT<ChargerTbatCompTimerReg_SPEC>;

impl ChargerTbatCompTimerReg {
    #[doc = "Returns the main battery temperature comparator\'s timer, used for the latching of the comparator\'s output. The output of the comparator is used by the JEITA FSM, to determine the current battery temperature pack\'s status."]
    #[inline(always)]
    pub fn tbat_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
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
            ChargerTbatCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Settling time (specified in us) for the main battery temperature comparator, checking for the \"COOL\", \"COLD\" and \"WARM\" levels. The charger\'s digital block uses a dedicated timer to sample the specific comparator\'s output. The comparator\'s output is latched as soon as the timer expires, reaching 0. Then, the timer is reloaded with the settling time value and starts-over, down-counting to 0.\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn tbat_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
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
#[doc = "Battery temperature monitor interval timer"]
pub type ChargerTbatMonTimerReg = crate::RegValueT<ChargerTbatMonTimerReg_SPEC>;

impl ChargerTbatMonTimerReg {
    #[doc = "This is the battery temperature monitoring timer, counting with the Charger\'s 1KHz clock. If the battery monitor mode is accordingly set in the TBAT_MONITOR_MODE bit-field of CHARGER_CTRL_REG (so either to 0x1 or 0x2), this timer is initially loaded with the value set in TBAT_MON_INTERVAL bit-field in the subsequent 1khz cycles starts down-counting to 0. As soon as the specific timer expires,the JEITA FSM starts-over again, to refresh the battery temperature status."]
    #[inline(always)]
    pub fn tbat_mon_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
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
            ChargerTbatMonTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Timing interval (in ms) for the Battery temperature monitoring. This interval determines how often the JEITA FSM will be checking and potentially refreshing the Battery temperature status, by selecting accordingly the proper level (Hot, Cold, Warm, Cool or Normal), based on the feedback of the two battery temperature comparators being present in the Charger\'s analogue circuitry (one for the Hot level and one for Cold, Cool and Warm, to support JEITA).\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn tbat_mon_interval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
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
#[doc = "Die temperature comparator timer register"]
pub type ChargerTdieCompTimerReg = crate::RegValueT<ChargerTdieCompTimerReg_SPEC>;

impl ChargerTdieCompTimerReg {
    #[doc = "Returns the current value of the timer used to determine when the Die temperature comparator\'s output must be sampled by the digital. As soon as the timer expires (down-counting to 0, starting from TDIE_COMP_SETTLING) the comparator\'s output is latched by the Charger\'s digital block and used by the main FSM. After expiring, the timer starts-over again, down-counting, to enable the continuous monitoring of Die temperature by the digital.\nNote: When the Charger\'s FSM is in BYPASSED state, this timer is kept to zero and the SW takes over, sampling the status of the TDIE_PROT_COMP_OUT bit-field of CHARGER_STATUS_REG to determine if the Die temperature limit has been exceeded."]
    #[inline(always)]
    pub fn tdie_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
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
            ChargerTdieCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Settling time threshold (in us) for the Die temperature comparator."]
    #[inline(always)]
    pub fn tdie_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
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
pub struct ChargerTempsetParamReg_SPEC;
impl crate::sealed::RegSpec for ChargerTempsetParamReg_SPEC {
    type DataType = u32;
}
#[doc = "Charger battery temperature settings register"]
pub type ChargerTempsetParamReg = crate::RegValueT<ChargerTempsetParamReg_SPEC>;

impl ChargerTempsetParamReg {
    #[doc = "This bit-field determines the maximum Die temperature level limit, ranging from 0C to 130C, according to the following encoding:\n000: 0 C (mainly for test purposes)\n001: 50 C\n010: 80 C\n011: 90 C\n100: 100 C\n101: 110 C\n110: 120 C\n111: 130 C"]
    #[inline(always)]
    pub fn tdie_max(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        u8,
        ChargerTempsetParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            u8,
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the battery temperature above which the charge current is zero, defining the \"Hot\" battery temperature zone. It ranges from minus 10C to 53C. The range is the same with the one defined in detail in TBAT_COLD bit-field."]
    #[inline(always)]
    pub fn tbat_hot(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        ChargerTempsetParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the battery temperature above which the charge current is reduced, defining the \"Warm\" temperature zone. It ranges from minus 10C to 53C. The range is the same with the one defined in detail in TBAT_COLD bit-field."]
    #[inline(always)]
    pub fn tbat_warm(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
        u8,
        ChargerTempsetParamReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the battery temperature below which the charge current is reduced, defining the \"Cool\" temperature zone. It ranges from minus 10C to 53C and the range is the same with the one defined in TBAT_COLD bit-field."]
    #[inline(always)]
    pub fn tbat_cool(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
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
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the battery temperature below which the charge current is zero, defining the \"Cold\" temperature zone. It ranges from minus 10C to 53C, according to the following encoding:\n0   :  -10 C\n1   :   -9 C\n2   :   -8 C\n3   :   -7 C\n4   :   -6 C\n5   :   -5 C\n6   :   -4 C\n7   :   -3 C\n8   :   -2 C\n9   :   -1 C\n10 :    0 C\n11 :    1 C\n12 :    2 C\n13 :    3 C\n14 :    4 C\n15 :    5 C\n16 :    6 C\n17 :    7 C\n18 :    8 C\n19 :    9 C\n20 :   10 C\n21 :   11 C\n22 :   12 C\n23 :   13 C\n24 :   14 C\n25 :   15 C\n26 :   16 C\n27 :   17 C\n28 :   18 C\n29 :   19 C\n30 :   20 C\n31 :   21 C\n32 :   22 C\n33 :   23 C\n34 :   24 C\n35 :   25 C\n36 :   26 C\n37 :   27 C\n38 :   28 C\n39 :   29 C\n40 :   30 C\n41 :   31 C\n42 :   32 C\n43 :   33 C\n44 :   34 C\n45 :   35 C\n46 :   36 C\n47 :   37 C\n48 :   38 C\n49 :   39 C\n50 :   40 C\n51 :   41 C\n52 :   42 C\n53 :   43 C\n54 :   44 C\n55 :   45 C\n56 :   46 C\n57 :   47 C\n58 :   48 C\n59 :   49 C\n60 :   50 C\n61 :   51 C\n62 :   52 C\n63 :   53 C"]
    #[inline(always)]
    pub fn tbat_cold(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
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
            ChargerTempsetParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChargerTempsetParamReg {
    #[inline(always)]
    fn default() -> ChargerTempsetParamReg {
        <crate::RegValueT<ChargerTempsetParamReg_SPEC> as RegisterValue<_>>::new(64935178)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerThotCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerThotCompTimerReg_SPEC {
    type DataType = u32;
}
#[doc = "Battery temperature comparator timer for \"Hot\" zone"]
pub type ChargerThotCompTimerReg = crate::RegValueT<ChargerThotCompTimerReg_SPEC>;

impl ChargerThotCompTimerReg {
    #[doc = "Returns the battery temperature comparator\'s timer dedicated for the \"Hot\" level."]
    #[inline(always)]
    pub fn thot_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
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
            ChargerThotCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Charger\'s battery temperature comparator settling time (specified in us), specifically for the Hot temperature zone. The charger\'s digital block uses a dedicated timer to sample the specific comparator\'s output. The comparator\'s output is latched as soon as the timer expires, reaching 0. Then, the timer is reloaded with the settling time value and starts-over again\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn thot_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
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
#[doc = "Maximum total charge time limit register"]
pub type ChargerTotalChargeTimerReg = crate::RegValueT<ChargerTotalChargeTimerReg_SPEC>;

impl ChargerTotalChargeTimerReg {
    #[doc = "Returns the current value of the overall charge timeout counter, running at a 1Hz clock. This timer has been set to 16 bits, so that it can count up to 10.5 hours, and ranges from 0 to MAX_TOTAL_CHARGE_TIME. It is reset to 0 when the Charger\'s FSM is either in DISABLED or in END_OF_CHARGE state."]
    #[inline(always)]
    pub fn total_charge_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            ChargerTotalChargeTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the maximum overall charging time allowed (measured in ticks of the 1Hz clock). If this is exceeded, a total charge time-out error will be captured by the Charger\'s controller and its FSM will move to the ERROR state. An IRQ will be also generated if the respective IRQ mask bit of CHARGER_ERROR_IRQ_MASK_REG is already set. In order to to exit this state, the \"CHARGER_RESUME\" bit-field of CHARGER_CTRL_REG must be set, to enable the Charger\'s FSM switch from ERROR to DISABLED state and start-over.\nNote: The specific bit-field should be always set to a non-zero value."]
    #[inline(always)]
    pub fn max_total_charge_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Main Vbat comparator timer register"]
pub type ChargerVbatCompTimerReg = crate::RegValueT<ChargerVbatCompTimerReg_SPEC>;

impl ChargerVbatCompTimerReg {
    #[doc = "Returns the current value of the timer used to determine when the output of the Vbat comparator (checking Vbat vs Pre_Charge and Replenish levels) must be sampled by the digital. As soon as the timer expires (down-counting to 0, starting from the value set in VBAT_COMP_SETTLING), the comparator\'s output is latched by the Charger\'s digital block and used by the FSM.\nNote: When the Charger\'s FSM is in BYPASSED state, this timer is kept to zero and the SW takes over. In this mode, the specific comparator checks the level of Vbat against the Pre-Charge level. Hence, SW can periodically sample the status of this comparator by reading the MAIN_VBAT_COMP_OUT bit-field of CHARGER_STATUS_REG, to determine if Vbat has exceeded the Pre-Charge level or not."]
    #[inline(always)]
    pub fn vbat_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
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
            ChargerVbatCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Settling time threshold (in us) for the Vbat comparator checking Vbat vs the programmed Pre-Charge and Replenish levels. The settings (voltage levels) of the comparator are controlled by the digital block of the Charger and they are driven based on the state of the main FSM (PRE_CHARGE, END_OF_CHARGE)."]
    #[inline(always)]
    pub fn vbat_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
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
#[doc = "Charger voltage settings register"]
pub type ChargerVoltageParamReg = crate::RegValueT<ChargerVoltageParamReg_SPEC>;

impl ChargerVoltageParamReg {
    #[doc = "This bit-field determines the VBAT Over-voltage protection limit. This Over-voltage protection level is used by the Charger\'s analogue circuitry and specifically by a dedicated comparator, the output of which is sampled by the digital block of the Charger. As soon as VBAT is detected to have reached or exceeded this level, the Charger\'s FSM moves to ERROR state, interrupting charging. If the respective Error IRQ mask bit is set, an Error IRQ pulse will be also generated.\nRegarding the actual range of supported values for this bit-field, see the the description of V_CHARGE bit-field of this register."]
    #[inline(always)]
    pub fn v_ovp(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
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
            ChargerVoltageParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the absolute value (in V) of the Replenish voltage threshold. As soon as charging has been completed and the Charger\'s FSM has reached the END_OF_CHARGE state, the respective analogue comparator of the Charger compares VBAT with the Replenish level. If VBAT is found to have dropped below this level, charging should start-over again and in that case, the FSM moves again to the PRE_CHARGE state.\nRegarding the supported Replenish voltage levels, see the description of V_CHARGE bit-field."]
    #[inline(always)]
    pub fn v_replenish(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3f,
        1,
        0,
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
            ChargerVoltageParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the voltage level at which the battery is considered as Pre-charged and therefore the Charger\'s FSM should move to the CC_CHARGE state, entering the Constant Current charging phase.\nRegarding the supported Pre-Charge voltage levels, see also the description of V_CHARGE bit-field of this register."]
    #[inline(always)]
    pub fn v_precharge(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3f,
        1,
        0,
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
            ChargerVoltageParamReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the charge voltage levels supported. The supported levels are determined according to the following encoding:\n0   : 2.80V\n1   : 2.85V\n2   : 2.90V\n3   : 2.95V\n4   : 3.00V\n5   : 3.05V\n6   : 3.10V\n7   : 3.15V\n8   : 3.20V\n9   : 3.25V\n10 : 3.30V\n11 : 3.35V\n12 : 3.40V\n13 : 3.45V\n14 : 3.50V\n15 : 3.55V\n16 : 3.60V\n17 : 3.65V\n18 : 3.70V\n19 : 3.75V\n20 : 3.80V\n21 : 3.82V\n22 : 3.84V\n23 : 3.86V\n24 : 3.88V\n25 : 3.90V\n26 : 3.92V\n27 : 3.94V\n28 : 3.96V\n29 : 3.98V\n30 : 4.00V\n31 : 4.02V\n32 : 4.04V\n33 : 4.06V\n34 : 4.08V\n35 : 4.10V\n36 : 4.12V\n37 : 4.14V\n38 : 4.16V\n39 : 4.18V\n40 : 4.20V\n41 : 4.22V\n42 : 4.24V\n43 : 4.26V\n44 : 4.28V\n45 : 4.30V\n46 : 4.32V\n47 : 4.34V\n48 : 4.36V\n49 : 4.38V\n50 : 4.40V\n51 : 4.42V\n52 : 4.44V\n53 : 4.46V\n54 : 4.48V\n55 : 4.50V\n56 : 4.52V\n57 : 4.54V\n58 : 4.56V\n59 : 4.58V\n60 : 4.60V\n61 : 4.70V\n62 : 4.80V\n63 : 4.90V*\nIt has to be noted that the specific values correspond to the normal battery temperature zone. However, the specific register field may be updated by the JEITA FSM (which checks the battery temperature either once or periodically), in order to adapt the charge voltage to the battery temperature zone (see also CHARGER_CTRL_REG.TBAT_MONITOR_MODE field as well). This is valid also for the other three fields of the current register. Consequently, in that case the register returns the Charge voltage settings that abide to the JEITA requirements for the battery (either COOL, WARM or NORMAL).\nNote: Option \"63\" (4.90V) is not supported for V_CHARGE, V_PRECHARGE and V_REPLENISH bit-fields (and respective levels). It should be used only in the V_OVP bit-field, as the (maximum) Over-voltage protection level."]
    #[inline(always)]
    pub fn v_charge(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
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
pub struct ChargerVovpCompTimerReg_SPEC;
impl crate::sealed::RegSpec for ChargerVovpCompTimerReg_SPEC {
    type DataType = u32;
}
#[doc = "Vbat OVP comparator timer register"]
pub type ChargerVovpCompTimerReg = crate::RegValueT<ChargerVovpCompTimerReg_SPEC>;

impl ChargerVovpCompTimerReg {
    #[doc = "The specific bit-field determines the current state of the timer used to periodically check the output of the Over-Voltage Protection comparator\'s output signal, as soon as the Charger\'s FSM reaches any of the charging states (PRE/CC/CV_CHARGE).\nWhen this happens, the timer starts ticking with the 1Mhz clock, ranging from 0 up to the programmed interval threshold (see also OVP_INTERVAL_CHECK_THRES field). As soon as this timer reaches the programmed threshold value, the Vbat OVP comparator\'s output is evaluated, increasing or not the counter keeping the consecutive OVP events. It is noted that out of the charging states, the specific timer is kept frozen to zero, not counting.\nNote : See also the OVP_OCCURRENCES_CNT bit-field of CHARGER_STATUS_REG for the consecutive OVP events counter."]
    #[inline(always)]
    pub fn ovp_interval_check_timer(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3f,
        1,
        0,
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
            ChargerVovpCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Returns the current value of the timer used to determine when the Vbat Over-Voltage protection (OVP) comparator\'s output must be sampled by the digital. As soon as the timer expires (down-counting to 0, starting from VBAT_OVP_COMP_SETTLING), the comparator\'s output is latched by the Charger\'s digital block and used by the main FSM.\nNote: When the Charger\'s FSM is in BYPASSED state, this timer is kept to zero and the SW takes over, sampling the status of the VBAT_OVP_COMP_OUT bit-field of CHARGER_STATUS_REG to determine if the Vbat has exceeded the OVP limit."]
    #[inline(always)]
    pub fn vbat_ovp_comp_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
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
            ChargerVovpCompTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "This bit-field determines the periodic interval of checking the dedicated Vbat OVP comparator\'s output, when the Charger\'s FSM is in any of the charging states (PRE/CC/CV_CHARGE). The implementation is based on a dedicated timer, counting from zero up to the value programmed into this bit-field (see also OVP_INTERVAL_CHECK_TIMER field\'s description) and only when the FSM is in any of the three charging states. Out of these states, the timer is kept frozen to zero.\nAs soon as this timer reaches the programmed threshold, the Vbat OVP comparator\'s output is sampled and depending on its level, (high or low), another counter, keeping the number of consecutive OVP events, is increased or not. The programmed threshold value should always be non-zero.\nNote: See also the OVP_DEBOUNCE_CNT bit-field of CHARGER_STATUS_REG, for the consecutive OVP events counter."]
    #[inline(always)]
    pub fn ovp_interval_check_thres(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3f,
        1,
        0,
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
            ChargerVovpCompTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Settling time threshold (in us) for the Vbat comparator checking Vbat vs the programmed Over-Voltage level."]
    #[inline(always)]
    pub fn vbat_ovp_comp_settling(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
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
