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
#[doc = r"GPREG registers"]
unsafe impl ::core::marker::Send for super::Gpreg {}
unsafe impl ::core::marker::Sync for super::Gpreg {}
impl super::Gpreg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Various debug information register."]
    #[inline(always)]
    pub const fn debug_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DebugReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DebugReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "General purpose system control register."]
    #[inline(always)]
    pub const fn gp_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "General purpose system status register."]
    #[inline(always)]
    pub const fn gp_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Controls unfreezing of various timers/counters (incl. DMA and USB)."]
    #[inline(always)]
    pub const fn reset_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Controls freezing of various timers/counters (incl. DMA and USB)."]
    #[inline(always)]
    pub const fn set_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "USB pads control register"]
    #[inline(always)]
    pub const fn usbpad_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UsbpadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UsbpadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugReg_SPEC;
impl crate::sealed::RegSpec for DebugReg_SPEC {
    type DataType = u32;
}
#[doc = "Various debug information register."]
pub type DebugReg = crate::RegValueT<DebugReg_SPEC>;

impl DebugReg {
    #[doc = "Select the cross CPU halt sensitivity.\n0: Level triggered,\n1: Pulse triggered.\nNote: This bit is retained."]
    #[inline(always)]
    pub fn cross_cpu_halt_sensitivity(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "1: Stall the processor core out of reset (only after a wake-up from JTAG). Debugger access continue when the core is stalled. When set to \'0\' again the core resumes instruction execution.\nThis feature is independent of the PDC (Power Domain Controller) settings. If this bit is set and there is SW/JTAG activity during deep sleep, the SYS CPU is stalled after the wake-up.\nNote: This bit is retained."]
    #[inline(always)]
    pub fn sys_cpuwait_on_jtag(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "1: Stall the processor core out of reset (always after a wake-up). Debugger access continue when the core is stalled. When set to \'0\' again the core resumes instruction execution.\nNote: This bit is retained."]
    #[inline(always)]
    pub fn sys_cpuwait(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "1: CMAC CPU is halted."]
    #[inline(always)]
    pub fn cmac_cpu_is_halted(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DebugReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, DebugReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "1: SYS CPU (ARM CM33) is halted."]
    #[inline(always)]
    pub fn sys_cpu_is_halted(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DebugReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, DebugReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "1: Enable CMAC CPU halting to the SYS CPU (ARM CM33).\nNote 1: This bit is retained.\nNote 2: Set this bit to \'0\' before going into deep sleep to prevent unpredictable halting behavior after waking up."]
    #[inline(always)]
    pub fn halt_cmac_sys_cpu_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "1: Enable SYS CPU (ARM CM33) halting to the CMAC CPU.\nNote 1: This bit is retained.\nNote 2: Set this bit to \'0\' before going into deep sleep to prevent unpredictable halting behavior after waking up."]
    #[inline(always)]
    pub fn halt_sys_cmac_cpu_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "1: Enable Freezing on-chip peripherals (see Note 2) by the CMAC CPU.\nNote 1: This bit is retained.\nNote 2: See \\[RE\\]SET_FREEZE_REG for the specific on-chip peripherals."]
    #[inline(always)]
    pub fn cmac_cpu_freeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "1: Enable Freezing on-chip peripherals (see Note 2) by the SYS CPU (ARM CM33).\nDefault \'1\', freezing of the on-chip peripherals is enabled when the Cortex-M33 is halted in DEBUG State.\nIf \'0\', freezing of the on-chip peripherals is only depending on \\[RE\\]SET_FREEZE_REG except the system watchdog timer. The system watchdog timer is always frozen when the Cortex-M33 is halted in DEBUG State.\nNote 1: This bit is retained.\nNote 2: See \\[RE\\]SET_FREEZE_REG for the specific on-chip peripherals."]
    #[inline(always)]
    pub fn sys_cpu_freeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for DebugReg {
    #[inline(always)]
    fn default() -> DebugReg {
        <crate::RegValueT<DebugReg_SPEC> as RegisterValue<_>>::new(257)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpControlReg_SPEC;
impl crate::sealed::RegSpec for GpControlReg_SPEC {
    type DataType = u32;
}
#[doc = "General purpose system control register."]
pub type GpControlReg = crate::RegValueT<GpControlReg_SPEC>;

impl GpControlReg {
    #[doc = "If \'1\', the AHB-to-AHB bridge is bypassed, reducing the wait cycles needed to access the CMAC Register File, only when the system clock source is the XTAL and both hclk and cmac_hclk are running at 32 MHz, i.e. at the XTAL clock rate."]
    #[inline(always)]
    pub fn cmac_h2h_bridge_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpControlReg {
    #[inline(always)]
    fn default() -> GpControlReg {
        <crate::RegValueT<GpControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpStatusReg_SPEC;
impl crate::sealed::RegSpec for GpStatusReg_SPEC {
    type DataType = u32;
}
#[doc = "General purpose system status register."]
pub type GpStatusReg = crate::RegValueT<GpStatusReg_SPEC>;

impl GpStatusReg {
    #[doc = "If \'1\', it designates that the chip is in Calibration Phase i.e. the OTP has been initially programmed but no Calibration has occured."]
    #[inline(always)]
    pub fn cal_phase(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpStatusReg {
    #[inline(always)]
    fn default() -> GpStatusReg {
        <crate::RegValueT<GpStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetFreezeReg_SPEC;
impl crate::sealed::RegSpec for ResetFreezeReg_SPEC {
    type DataType = u32;
}
#[doc = "Controls unfreezing of various timers/counters (incl. DMA and USB)."]
pub type ResetFreezeReg = crate::RegValueT<ResetFreezeReg_SPEC>;

impl ResetFreezeReg {
    #[doc = "If \'1\', the CMAC SW Watchdog Timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_cmac_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer4 continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim4(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer3 continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim3(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the PWM LED continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_pwmled(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer2 continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim2(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the DMA continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the USB continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_usb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SYS SW Watchdog Timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_sys_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn frz_reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the Wake Up Timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetFreezeReg {
    #[inline(always)]
    fn default() -> ResetFreezeReg {
        <crate::RegValueT<ResetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetFreezeReg_SPEC;
impl crate::sealed::RegSpec for SetFreezeReg_SPEC {
    type DataType = u32;
}
#[doc = "Controls freezing of various timers/counters (incl. DMA and USB)."]
pub type SetFreezeReg = crate::RegValueT<SetFreezeReg_SPEC>;

impl SetFreezeReg {
    #[doc = "If \'1\', the CMAC SW Watchdog Timer is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_cmac_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer4 is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim4(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer3 is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim3(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the PWM LED is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_pwmled(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer2 is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim2(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the DMA is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the USB is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_usb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SYS SW Watchdog Timer is frozen, \'0\' is discarded. WATCHDOG_CTRL_REG\\[NMI_RST\\] must be \'0\' to allow the freeze function."]
    #[inline(always)]
    pub fn frz_sys_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn frz_reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the SW Timer is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If \'1\', the Wake Up Timer is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetFreezeReg {
    #[inline(always)]
    fn default() -> SetFreezeReg {
        <crate::RegValueT<SetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbpadReg_SPEC;
impl crate::sealed::RegSpec for UsbpadReg_SPEC {
    type DataType = u32;
}
#[doc = "USB pads control register"]
pub type UsbpadReg = crate::RegValueT<UsbpadReg_SPEC>;

impl UsbpadReg {
    #[doc = "0: Pull up resistor SW2 is controlled by the USB controller. It is off when the USB is not enabled.\n1: Force the pull up resistor on USBP to be 2.3Kohm"]
    #[inline(always)]
    pub fn usbphy_force_sw2_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UsbpadReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UsbpadReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Pull up resistor SW1 is controlled by the USB controller. It is off when the USB is not enabled.\n1: Force the pull up resistor on USBP to be switched off."]
    #[inline(always)]
    pub fn usbphy_force_sw1_off(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UsbpadReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UsbpadReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: The power for the USB PHY and USB pads is switched on when the USB is enabled.\n1: The power for the USB PHY and USB pads is forced on."]
    #[inline(always)]
    pub fn usbpad_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UsbpadReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UsbpadReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UsbpadReg {
    #[inline(always)]
    fn default() -> UsbpadReg {
        <crate::RegValueT<UsbpadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
