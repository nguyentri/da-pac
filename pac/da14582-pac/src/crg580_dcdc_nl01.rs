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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:29 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"crg580_dcdc_nl01 registers"]
unsafe impl ::core::marker::Send for super::Crg580DcdcNl01 {}
unsafe impl ::core::marker::Sync for super::Crg580DcdcNl01 {}
impl super::Crg580DcdcNl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DCDC second control register"]
    #[inline(always)]
    pub const fn dcdc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "DCDC thirth control register"]
    #[inline(always)]
    pub const fn dcdc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DCDC control register"]
    #[inline(always)]
    pub const fn dcdc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DcdcCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DcdcCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl2Reg_SPEC {
    type DataType = u16;
}

#[doc = "DCDC second control register"]
pub type DcdcCtrl2Reg = crate::RegValueT<DcdcCtrl2Reg_SPEC>;

impl DcdcCtrl2Reg {
    #[doc = "Nominal output voltage of the DCDC-converter.\nVDCDC = 1.2V + N*25mV"]
    #[inline(always)]
    pub fn dcdc_volt_lev(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Nominal VBAT3V output voltage of the boost converter.\n000 ... 011 = 1.8V + N*25mV\n100 = 2.4V\n101 = 2.5V\n110 = 2.62V\n111 = 2.76V\n(Note: MSB is automatically on if the OTP LDO is enabled.)"]
    #[inline(always)]
    pub fn dcdc_vbat3v_lev(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This defines the minimum on-time of the comparators. For buck-mode use 0x2, for boost-mode use 0x1"]
    #[inline(always)]
    pub fn dcdc_ton(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Current limit in the switches of the DCDC-converter (approximate values):\nN x 10mA"]
    #[inline(always)]
    pub fn dcdc_cur_lim(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control of the automatic calibration of the DCDC-converter. For Buck-mode use 0x1, for Boost-mode use 0x6.\nAutomatic calibration is disabled by setting 0x0"]
    #[inline(always)]
    pub fn dcdc_auto_cal(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, DcdcCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,DcdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl2Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl2Reg {
        <crate::RegValueT<DcdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(35872)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrl3Reg_SPEC {
    type DataType = u16;
}

#[doc = "DCDC thirth control register"]
pub type DcdcCtrl3Reg = crate::RegValueT<DcdcCtrl3Reg_SPEC>;

impl DcdcCtrl3Reg {
    #[doc = "timeout for the idle state to check voltage"]
    #[inline(always)]
    pub fn dcdc_timeout(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, DcdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,DcdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock used as trigger for the idle state to check voltage.\n(Note: when no 16 MHz oscillator is active, the 32 kHz oscillator will be used as trigger independent of the setting below:) \n00 = 16 MHz\n01 = 4 MHz\n10 = 1 MHz\n11 = 250 kHz"]
    #[inline(always)]
    pub fn dcdc_idle_clk(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, DcdcCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,DcdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the buck converter when the device becomes active and VBAT1V is connected to GND."]
    #[inline(always)]
    pub fn buck_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DcdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DcdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrl3Reg {
    #[inline(always)]
    fn default() -> DcdcCtrl3Reg {
        <crate::RegValueT<DcdcCtrl3Reg_SPEC> as RegisterValue<_>>::new(21)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCtrlReg_SPEC;
impl crate::sealed::RegSpec for DcdcCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "DCDC control register"]
pub type DcdcCtrlReg = crate::RegValueT<DcdcCtrlReg_SPEC>;

impl DcdcCtrlReg {
    #[doc = "Tune-bits to compensate for parasitic resistance in the current sense circuit of the DCDC-converter."]
    #[inline(always)]
    pub fn dcdc_tune(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drive level of the switch between SWITCH and VDCDC.\n00 = 100 percent\n01 = 66  percent\n10 = 33  percent\n11 = off"]
    #[inline(always)]
    pub fn dcdc_drive_osw(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drive level of the switch between SWITCH and VBAT3V.\n00 = 100 percent\n01 = 66  percent\n10 = 33  percent\n11 = off"]
    #[inline(always)]
    pub fn dcdc_drive_psw(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drive level of the switch between SWITCH and GROUND.\n00 = 100 percent\n01 = 66  percent\n10 = 33  percent\n11 = off"]
    #[inline(always)]
    pub fn dcdc_drive_nsw(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Testmodes, keep 000."]
    #[inline(always)]
    pub fn dcdc_mode(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If VBAT1V is below this level, the boost converter will be disabled:\n110 = 0.6V\n101 = 0.8V\n011 = 1.0V\n111 = 0V (always OK)"]
    #[inline(always)]
    pub fn dcdc_vbat1v_lev(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, DcdcCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,DcdcCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DcdcCtrlReg {
    #[inline(always)]
    fn default() -> DcdcCtrlReg {
        <crate::RegValueT<DcdcCtrlReg_SPEC> as RegisterValue<_>>::new(12)
    }
}
