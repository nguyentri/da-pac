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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:52 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CHG_DET registers"]
unsafe impl ::core::marker::Send for super::ChgDet {}
unsafe impl ::core::marker::Sync for super::ChgDet {}
impl super::ChgDet {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Charge detection ADC control register"]
    #[inline(always)]
    pub const fn chg_det_adc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetAdcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Charge detection DCD time-out timer register (used in the FSM)"]
    #[inline(always)]
    pub const fn chg_det_dcd_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetDcdTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetDcdTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Charge detection FSM control register"]
    #[inline(always)]
    pub const fn chg_det_fsm_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetFsmCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetFsmCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Charge detection FSM status register"]
    #[inline(always)]
    pub const fn chg_det_fsm_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetFsmStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetFsmStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Charge detection IRQ clear register"]
    #[inline(always)]
    pub const fn chg_det_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Charge detection IRQ mask register"]
    #[inline(always)]
    pub const fn chg_det_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Charge detection IRQ status register"]
    #[inline(always)]
    pub const fn chg_det_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Charge detection status register holding the comparator outputs"]
    #[inline(always)]
    pub const fn chg_det_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Charge detection manual (SW-based) mode control register"]
    #[inline(always)]
    pub const fn chg_det_sw_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetSwCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Charge detection timer register (used in the FSM)"]
    #[inline(always)]
    pub const fn chg_det_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChgDetTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChgDetTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetAdcCtrlReg_SPEC;
impl crate::sealed::RegSpec for ChgDetAdcCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection ADC control register"]
pub type ChgDetAdcCtrlReg = crate::RegValueT<ChgDetAdcCtrlReg_SPEC>;

impl ChgDetAdcCtrlReg {
    #[doc = "This bit-field determines the mode to be used when the intention is to measure the USB Dp/Dm levels with the GP_ADC. In specific:\n0 = Should be used when the V30 rail is at 3.0V\n1 = Should be used when the V30 rail is at 3.3V"]
    #[inline(always)]
    pub fn adc_v30_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disables the connection of USBm to the GP_ADC\n1 = Enables the connection of USBm to the GP_ADC through a relative test switch"]
    #[inline(always)]
    pub fn usb_dm_to_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disables the connection of USBp to the GP_ADC\n1 = Enables the connection of USBp to the GP_ADC through a relative test switch"]
    #[inline(always)]
    pub fn usb_dp_to_adc_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetAdcCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetAdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetAdcCtrlReg {
    #[inline(always)]
    fn default() -> ChgDetAdcCtrlReg {
        <crate::RegValueT<ChgDetAdcCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetDcdTimerReg_SPEC;
impl crate::sealed::RegSpec for ChgDetDcdTimerReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection DCD time-out timer register (used in the FSM)"]
pub type ChgDetDcdTimerReg = crate::RegValueT<ChgDetDcdTimerReg_SPEC>;

impl ChgDetDcdTimerReg {
    #[doc = "This bit-field returns the current value of the DCD time-out timer, also ticking with the 1KHz clock, same as the shared 8-bit timer.\nThe specific timer counts only in states NODE_ATTACHED, WAIT_FOR_DCD and CHECK_FOR_DCD, since these are the states where Data Contact Detection is evaluated.\nThe timer is automatically reloaded with the programmed threshold value (DCD_TIMEOUT_THRES) as soon as the FSM returns to its starting state (WAIT_FOR_ATTACH, 0x0)."]
    #[inline(always)]
    pub fn dcd_timer(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChgDetDcdTimerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChgDetDcdTimerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "This bit-field determines the value from which the Data Contact Detection (DCD) time-out timer starts down-counting, until expiring to zero. Non-zero values are recommended."]
    #[inline(always)]
    pub fn dcd_timeout_thres(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        ChgDetDcdTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            ChgDetDcdTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ChgDetDcdTimerReg {
    #[inline(always)]
    fn default() -> ChgDetDcdTimerReg {
        <crate::RegValueT<ChgDetDcdTimerReg_SPEC> as RegisterValue<_>>::new(39322200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetFsmCtrlReg_SPEC;
impl crate::sealed::RegSpec for ChgDetFsmCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection FSM control register"]
pub type ChgDetFsmCtrlReg = crate::RegValueT<ChgDetFsmCtrlReg_SPEC>;

impl ChgDetFsmCtrlReg {
    #[doc = "0 = Charge detection FSM is disabled, analog part is controlled through the USB_CHARGER_CTRL_REG register\'s bit-fields.\n1 = Charge detection FSM is enabled and controls the analog part, overruling the content of USB_CHARGER_CTRL_REG."]
    #[inline(always)]
    pub fn chg_det_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetFsmCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetFsmCtrlReg {
    #[inline(always)]
    fn default() -> ChgDetFsmCtrlReg {
        <crate::RegValueT<ChgDetFsmCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetFsmStatusReg_SPEC;
impl crate::sealed::RegSpec for ChgDetFsmStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection FSM status register"]
pub type ChgDetFsmStatusReg = crate::RegValueT<ChgDetFsmStatusReg_SPEC>;

impl ChgDetFsmStatusReg {
    #[doc = "This bit-field returns the current state of the charge detection block\'s FSM. The supported states are:\n  - 0x0 : WAIT_FOR_ATTACH (starting state)\n  - 0x1 : NODE_ATTACHED (USB_DP_VAL first check)\n  - 0x2 : WAIT_FOR_DCD (Data Contact Detection - first state)\n  - 0x3 : CHECK_FOR_DCD (Data Contact Detection - second state)\n  - 0x4 : WAIT_FOR_PCD (Primary Contact Detection - first state)\n  - 0x5 : CHECK_FOR_PCD (Primary Contact Detection - second state)\n  - 0x6 : WAIT_FOR_SCD (Secondary Contact Detection - first state)\n  - 0x7 : CHECK_FOR_SCD (Secondary Contact Detection - second state)\n  - 0x8 : CHG_PORT_DETECTED (terminal state, indicating the port detection\'s completion).\nIt is noted that:\n  - The FSM returns to state WAIT_FOR_ATTACH either when VBUS_AVAILABLE goes to \'0\', or when it is disabled by the SW.\n  - The FSM may return to state WAIT_FOR_ATTACH from any of the other states when VBUS_AVAILABLE goes to \'0\', implying either a USB detach event or a VBUS voltage drop.\n  - The FSM implements a time-out mechanism during the Data Contact Detection phase (NODE_ATTACHED, WAIT_FOR_DCD and CHECK_FOR_DCD states). A dedicated 10-bit timer counts down in any of these states, until USB_DP_VAL is found to be \'0\' (so we have a contact detected) or until it expires.\n  - The time-out timer\'s expiration means that there is no D+/D- pins contact detected within the required time-out period, as defined through CHG_DET_DCD_TIMER_REG and as required by the spec. Even in this case though, the FSM does not abort, but proceeds normally to the next state, which is the first Primary Detection State, to comply with the Battery Charging specification."]
    #[inline(always)]
    pub fn chg_det_state(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, ChgDetFsmStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            9,
            0xf,
            1,
            0,
            u8,
            u8,
            ChgDetFsmStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "0 = Contact is sensed on the Dp/Dm (data) pins by the FSM, during the Data Contact Detection (DCD) phase (USB_DP_VAL = 0)\n1 = No contact is sensed on the Dp/Dm pins during the DCD phase (USB_DP_VAL = 1)\nNote: Even in the case of no contact detected on the Dp/Dm pins, the FSM continues with the port detection procedure, moving to the Primary Contact Detection (PCD) states."]
    #[inline(always)]
    pub fn no_contact_detected(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection either not yet finished or finished without detecting a 2.4 Amp charging port\n1 = Port detection has finished by detecting a 2.4 Amp charging port"]
    #[inline(always)]
    pub fn port_2p4amp_detected(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection either not yet finished or finished without detecting a 2 Amp charging port\n1 = Port detection has finished by detecting a 2 Amp charging port"]
    #[inline(always)]
    pub fn port_2amp_detected(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection either not yet finished or finished without detecting a 1 Amp charging port\n1 = Port detection has finished by detecting a 1 Amp charging port"]
    #[inline(always)]
    pub fn port_1amp_detected(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection either not yet finished or finished without detecting a PS2 or a Proprietary charger port\n1 = Port detection has finished by detecting either a PS2 or a Proprietary charger port"]
    #[inline(always)]
    pub fn ps2_prop_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection either not yet finished or finished without detecting a DCP port (Dedicated Charging Port)\n1 = Port detection has finished by detecting a DCP port"]
    #[inline(always)]
    pub fn dcp_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection either not yet finished or finished without detecting a CDP port (Charging Downstream Port)\n1 = Port detection has finished by detecting a CDP port"]
    #[inline(always)]
    pub fn cdp_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection either not yet finished or finished without detecting an SDP port (Standard Downstream Port)\n1 = Port detection has finished by detecting an SDP port"]
    #[inline(always)]
    pub fn sdp_port_detected(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Port detection not yet finished, FSM still active\n1 = Port detection completed, FSM has reached its terminal state (CHG_PORT_DETECTED). The port type is accessible through the relevant bit-fields of this register."]
    #[inline(always)]
    pub fn detection_completed(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetFsmStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetFsmStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetFsmStatusReg {
    #[inline(always)]
    fn default() -> ChgDetFsmStatusReg {
        <crate::RegValueT<ChgDetFsmStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetIrqClearReg_SPEC;
impl crate::sealed::RegSpec for ChgDetIrqClearReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection IRQ clear register"]
pub type ChgDetIrqClearReg = crate::RegValueT<ChgDetIrqClearReg_SPEC>;

impl ChgDetIrqClearReg {
    #[doc = "Writing any value to this register clears the charge detection IRQ, reading always returns zero."]
    #[inline(always)]
    pub fn chg_det_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetIrqClearReg {
    #[inline(always)]
    fn default() -> ChgDetIrqClearReg {
        <crate::RegValueT<ChgDetIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for ChgDetIrqMaskReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection IRQ mask register"]
pub type ChgDetIrqMaskReg = crate::RegValueT<ChgDetIrqMaskReg_SPEC>;

impl ChgDetIrqMaskReg {
    #[doc = "0 = Charge detection block\'s IRQ generation is disabled, interrupts to Cortex-M33 are masked.\n1 = Charge detection block\'s IRQ generation is enabled."]
    #[inline(always)]
    pub fn chg_det_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetIrqMaskReg {
    #[inline(always)]
    fn default() -> ChgDetIrqMaskReg {
        <crate::RegValueT<ChgDetIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for ChgDetIrqStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection IRQ status register"]
pub type ChgDetIrqStatusReg = crate::RegValueT<ChgDetIrqStatusReg_SPEC>;

impl ChgDetIrqStatusReg {
    #[doc = "0 = No new charge detection IRQ has been generated.\n1 = A new charge detection IRQ is generated and should be cleared by SW, by writing to CHG_DET_IRQ_CLEAR_REG (interrupt is level-sensitive)."]
    #[inline(always)]
    pub fn chg_det_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetIrqStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetIrqStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetIrqStatusReg {
    #[inline(always)]
    fn default() -> ChgDetIrqStatusReg {
        <crate::RegValueT<ChgDetIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetStatusReg_SPEC;
impl crate::sealed::RegSpec for ChgDetStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection status register holding the comparator outputs"]
pub type ChgDetStatusReg = crate::RegValueT<ChgDetStatusReg_SPEC>;

impl ChgDetStatusReg {
    #[doc = "0: USBp < 2.3V\n1: USBp > 2.5V"]
    #[inline(always)]
    pub fn usb_dm_val2(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0: USBp < 2.3V\n1: USBp > 2.5V"]
    #[inline(always)]
    pub fn usb_dp_val2(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = USBm < 0.8V\n1 = USBm > 1.5V (PS2 or Proprietary Charger)"]
    #[inline(always)]
    pub fn usb_dm_val(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = USBp < 0.8V\n1 = USBp > 1.5V (PS2 or Proprietary Charger)"]
    #[inline(always)]
    pub fn usb_dp_val(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Standard Downstream Port (SDP) or no Dp/Dm contact detected (nothing connected)\n1 = Charging Downstream Port (CDP) or Dedicated Charging Port (DCP)"]
    #[inline(always)]
    pub fn usb_chg_det(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = Charging downstream port is detected\n1 = Dedicated charger is detected\nIt is noted that the control bit VDM_SRC_ON must be set to validate this status bit. \nNote: This register shows the actual status."]
    #[inline(always)]
    pub fn usb_dcp_det(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetStatusReg {
    #[inline(always)]
    fn default() -> ChgDetStatusReg {
        <crate::RegValueT<ChgDetStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetSwCtrlReg_SPEC;
impl crate::sealed::RegSpec for ChgDetSwCtrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection manual (SW-based) mode control register"]
pub type ChgDetSwCtrlReg = crate::RegValueT<ChgDetSwCtrlReg_SPEC>;

impl ChgDetSwCtrlReg {
    #[doc = "0 = Disables the Idm_sink to USBm\n1 = Enables the Idm_sink to USBm"]
    #[inline(always)]
    pub fn idm_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disables the Idp_sink to USBp\n1 = Enables the Idp_sink to USBp"]
    #[inline(always)]
    pub fn idp_sink_on(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disables the Vdm_src\n1 = Enables the Vdm to USBm and also the USB_DCP_DET status bit"]
    #[inline(always)]
    pub fn vdm_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disables the Vdp_src\n1 = Enables the Vdp_src and also the USB_CHG_DET status bit"]
    #[inline(always)]
    pub fn vdp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disables the Idp_src and the Rdm_dwn\n1 = Enables the Idp_src and the Rdm_dwn"]
    #[inline(always)]
    pub fn idp_src_on(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Disables the charge detection analog circuit\n1 = Enables the charge detection analog circuit"]
    #[inline(always)]
    pub fn usb_charge_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChgDetSwCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ChgDetSwCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetSwCtrlReg {
    #[inline(always)]
    fn default() -> ChgDetSwCtrlReg {
        <crate::RegValueT<ChgDetSwCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgDetTimerReg_SPEC;
impl crate::sealed::RegSpec for ChgDetTimerReg_SPEC {
    type DataType = u32;
}

#[doc = "Charge detection timer register (used in the FSM)"]
pub type ChgDetTimerReg = crate::RegValueT<ChgDetTimerReg_SPEC>;

impl ChgDetTimerReg {
    #[doc = "This bit-field returns the current value of the charge detection timer, which is used by the FSM and which down-counts with a clock of 1KHz. The specific clock is enabled by setting the CLK_SYS_REG\\[CLK_CHG_EN\\] bit-field to \'1\' and it is the same clock used by the Charger\'s digital block.\nThe specific timer is automatically re-loaded with the programmed threshold value (CHG_DET_TIMER_THRES), upon expiring to zero or when the charge detection FSM moves to the state NODE_ATTACHED (0x1)."]
    #[inline(always)]
    pub fn chg_det_timer(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, ChgDetTimerReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,ChgDetTimerReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This bit-field determines the value from which the charge detection timer starts down-counting, until expiring to zero. Non-zero values are recommended."]
    #[inline(always)]
    pub fn chg_det_timer_thres(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ChgDetTimerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ChgDetTimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChgDetTimerReg {
    #[inline(always)]
    fn default() -> ChgDetTimerReg {
        <crate::RegValueT<ChgDetTimerReg_SPEC> as RegisterValue<_>>::new(3276850)
    }
}
