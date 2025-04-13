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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:53:57 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PDC registers"]
unsafe impl ::core::marker::Send for super::Pdc {}
unsafe impl ::core::marker::Sync for super::Pdc {}
impl super::Pdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Clear a pending PDC bit"]
    #[inline(always)]
    pub const fn pdc_acknowledge_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcAcknowledgeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcAcknowledgeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "PDC control register"]
    #[inline(always)]
    pub const fn pdc_ctrl9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcCtrl9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcCtrl9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Shows any pending IRQ to CM33"]
    #[inline(always)]
    pub const fn pdc_pending_cm33_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcPendingCm33Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcPendingCm33Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Shows any pending IRQ to CM33"]
    #[inline(always)]
    pub const fn pdc_pending_cmac_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcPendingCmacReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcPendingCmacReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Shows any pending wakup event"]
    #[inline(always)]
    pub const fn pdc_pending_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcPendingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcPendingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Shows any pending IRQ to SNC"]
    #[inline(always)]
    pub const fn pdc_pending_snc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcPendingSncReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcPendingSncReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Set a pending PDC bit"]
    #[inline(always)]
    pub const fn pdc_set_pending_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PdcSetPendingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PdcSetPendingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcAcknowledgeReg_SPEC;
impl crate::sealed::RegSpec for PdcAcknowledgeReg_SPEC {
    type DataType = u32;
}
#[doc = "Clear a pending PDC bit"]
pub type PdcAcknowledgeReg = crate::RegValueT<PdcAcknowledgeReg_SPEC>;

impl PdcAcknowledgeReg {
    #[doc = "Writing to this field acknowledges the PDC IRQ request.\nThe data controls which request is acknowledged"]
    #[inline(always)]
    pub fn pdc_acknowledge(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, PdcAcknowledgeReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, PdcAcknowledgeReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcAcknowledgeReg {
    #[inline(always)]
    fn default() -> PdcAcknowledgeReg {
        <crate::RegValueT<PdcAcknowledgeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl0Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl0Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl0Reg = crate::RegValueT<PdcCtrl0Reg_SPEC>;

impl PdcCtrl0Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects which individual bit from the selected bank is used for wakup.\nFor the peripheral banks, selected with TRIG_SELECT = 0x2 or 0x3, only the lower 4 bits are considered."]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects which bank is used as wakeup trigger\nWhen TRIG_SELECT is 0x0, selects GPIO port0 through the WAKEUP block.\nWhen TRIG_SELECT is 0x1, selects GPIO port1 through the WAKEUP block.\nWhen TRIG_SELECT is 0x2 or 0x3, selects the peripheral IRQ.\n\nperipheral IRQ table:\n0x0: Timer\n0x1: Timer2\n0x2: Timer3\n0x3: Timer4\n0x4: RTC Alarm/Rollover\n0x5: RTC Timer\n0x6: CMAC Timer OR wake up from CMAC debugger\n0x7: Motor Controller\n0x8: XTAL32MRDY_IRQ\n0x9: RFDIAG_IRQ\n0xA: CMAC2SYS_IRQ OR VBUS Present IRQ OR JTAG present OR Debounced IO\n0xB: Sensor Node Controller\n0xC to 0xE: reserved\n0xF: Software trigger only"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl0Reg {
    #[inline(always)]
    fn default() -> PdcCtrl0Reg {
        <crate::RegValueT<PdcCtrl0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl10Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl10Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl10Reg = crate::RegValueT<PdcCtrl10Reg_SPEC>;

impl PdcCtrl10Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl10Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl10Reg {
    #[inline(always)]
    fn default() -> PdcCtrl10Reg {
        <crate::RegValueT<PdcCtrl10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl11Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl11Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl11Reg = crate::RegValueT<PdcCtrl11Reg_SPEC>;

impl PdcCtrl11Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl11Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl11Reg {
    #[inline(always)]
    fn default() -> PdcCtrl11Reg {
        <crate::RegValueT<PdcCtrl11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl12Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl12Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl12Reg = crate::RegValueT<PdcCtrl12Reg_SPEC>;

impl PdcCtrl12Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl12Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl12Reg {
    #[inline(always)]
    fn default() -> PdcCtrl12Reg {
        <crate::RegValueT<PdcCtrl12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl13Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl13Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl13Reg = crate::RegValueT<PdcCtrl13Reg_SPEC>;

impl PdcCtrl13Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl13Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl13Reg {
    #[inline(always)]
    fn default() -> PdcCtrl13Reg {
        <crate::RegValueT<PdcCtrl13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl14Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl14Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl14Reg = crate::RegValueT<PdcCtrl14Reg_SPEC>;

impl PdcCtrl14Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl14Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl14Reg {
    #[inline(always)]
    fn default() -> PdcCtrl14Reg {
        <crate::RegValueT<PdcCtrl14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl15Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl15Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl15Reg = crate::RegValueT<PdcCtrl15Reg_SPEC>;

impl PdcCtrl15Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl15Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl15Reg {
    #[inline(always)]
    fn default() -> PdcCtrl15Reg {
        <crate::RegValueT<PdcCtrl15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl1Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl1Reg = crate::RegValueT<PdcCtrl1Reg_SPEC>;

impl PdcCtrl1Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl1Reg {
    #[inline(always)]
    fn default() -> PdcCtrl1Reg {
        <crate::RegValueT<PdcCtrl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl2Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl2Reg = crate::RegValueT<PdcCtrl2Reg_SPEC>;

impl PdcCtrl2Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IIf set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl2Reg {
    #[inline(always)]
    fn default() -> PdcCtrl2Reg {
        <crate::RegValueT<PdcCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl3Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl3Reg = crate::RegValueT<PdcCtrl3Reg_SPEC>;

impl PdcCtrl3Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl3Reg {
    #[inline(always)]
    fn default() -> PdcCtrl3Reg {
        <crate::RegValueT<PdcCtrl3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl4Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl4Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl4Reg = crate::RegValueT<PdcCtrl4Reg_SPEC>;

impl PdcCtrl4Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl4Reg {
    #[inline(always)]
    fn default() -> PdcCtrl4Reg {
        <crate::RegValueT<PdcCtrl4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl5Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl5Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl5Reg = crate::RegValueT<PdcCtrl5Reg_SPEC>;

impl PdcCtrl5Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl5Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl5Reg {
    #[inline(always)]
    fn default() -> PdcCtrl5Reg {
        <crate::RegValueT<PdcCtrl5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl6Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl6Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl6Reg = crate::RegValueT<PdcCtrl6Reg_SPEC>;

impl PdcCtrl6Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl6Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl6Reg {
    #[inline(always)]
    fn default() -> PdcCtrl6Reg {
        <crate::RegValueT<PdcCtrl6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl7Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl7Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl7Reg = crate::RegValueT<PdcCtrl7Reg_SPEC>;

impl PdcCtrl7Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl7Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl7Reg {
    #[inline(always)]
    fn default() -> PdcCtrl7Reg {
        <crate::RegValueT<PdcCtrl7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl8Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl8Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl8Reg = crate::RegValueT<PdcCtrl8Reg_SPEC>;

impl PdcCtrl8Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl8Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl8Reg {
    #[inline(always)]
    fn default() -> PdcCtrl8Reg {
        <crate::RegValueT<PdcCtrl8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcCtrl9Reg_SPEC;
impl crate::sealed::RegSpec for PdcCtrl9Reg_SPEC {
    type DataType = u32;
}
#[doc = "PDC control register"]
pub type PdcCtrl9Reg = crate::RegValueT<PdcCtrl9Reg_SPEC>;

impl PdcCtrl9Reg {
    #[doc = "Chooses which master is triggered when waking up\n0x0: entry is disabled.\n0x1: PD_SYS is woken up and CM33 is triggered\n0x2: PD_RAD is woken up and CMAC is triggered\n0x3: PD_COM is woken up and SNC is triggered"]
    #[inline(always)]
    pub fn pdc_master(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, PdcCtrl9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, PdcCtrl9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_COM for GPIO access. This bit is implied when PDC_MASTER=SNC"]
    #[inline(always)]
    pub fn en_com(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PdcCtrl9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PdcCtrl9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_PER"]
    #[inline(always)]
    pub fn en_per(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PdcCtrl9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PdcCtrl9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, enables PD_TMR"]
    #[inline(always)]
    pub fn en_tmr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PdcCtrl9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PdcCtrl9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the XTAL32M will be started"]
    #[inline(always)]
    pub fn en_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PdcCtrl9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PdcCtrl9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_ID"]
    #[inline(always)]
    pub fn trig_id(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, PdcCtrl9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, PdcCtrl9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For description, see PDC_CTRL0_REG.TRIG_SELECT"]
    #[inline(always)]
    pub fn trig_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, PdcCtrl9Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, PdcCtrl9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcCtrl9Reg {
    #[inline(always)]
    fn default() -> PdcCtrl9Reg {
        <crate::RegValueT<PdcCtrl9Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcPendingCm33Reg_SPEC;
impl crate::sealed::RegSpec for PdcPendingCm33Reg_SPEC {
    type DataType = u32;
}
#[doc = "Shows any pending IRQ to CM33"]
pub type PdcPendingCm33Reg = crate::RegValueT<PdcPendingCm33Reg_SPEC>;

impl PdcPendingCm33Reg {
    #[doc = "Indicates which IRQ ids are pending towards the CM33"]
    #[inline(always)]
    pub fn pdc_pending(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, PdcPendingCm33Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, PdcPendingCm33Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcPendingCm33Reg {
    #[inline(always)]
    fn default() -> PdcPendingCm33Reg {
        <crate::RegValueT<PdcPendingCm33Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcPendingCmacReg_SPEC;
impl crate::sealed::RegSpec for PdcPendingCmacReg_SPEC {
    type DataType = u32;
}
#[doc = "Shows any pending IRQ to CM33"]
pub type PdcPendingCmacReg = crate::RegValueT<PdcPendingCmacReg_SPEC>;

impl PdcPendingCmacReg {
    #[doc = "Indicates which IRQ ids are pending towards the CMAC"]
    #[inline(always)]
    pub fn pdc_pending(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, PdcPendingCmacReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, PdcPendingCmacReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcPendingCmacReg {
    #[inline(always)]
    fn default() -> PdcPendingCmacReg {
        <crate::RegValueT<PdcPendingCmacReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcPendingReg_SPEC;
impl crate::sealed::RegSpec for PdcPendingReg_SPEC {
    type DataType = u32;
}
#[doc = "Shows any pending wakup event"]
pub type PdcPendingReg = crate::RegValueT<PdcPendingReg_SPEC>;

impl PdcPendingReg {
    #[doc = "Indicates which IRQ ids are pending"]
    #[inline(always)]
    pub fn pdc_pending(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, PdcPendingReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, PdcPendingReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcPendingReg {
    #[inline(always)]
    fn default() -> PdcPendingReg {
        <crate::RegValueT<PdcPendingReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcPendingSncReg_SPEC;
impl crate::sealed::RegSpec for PdcPendingSncReg_SPEC {
    type DataType = u32;
}
#[doc = "Shows any pending IRQ to SNC"]
pub type PdcPendingSncReg = crate::RegValueT<PdcPendingSncReg_SPEC>;

impl PdcPendingSncReg {
    #[doc = "Indicates which IRQ ids are pending towards the SensorNodeController"]
    #[inline(always)]
    pub fn pdc_pending(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, PdcPendingSncReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, PdcPendingSncReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcPendingSncReg {
    #[inline(always)]
    fn default() -> PdcPendingSncReg {
        <crate::RegValueT<PdcPendingSncReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdcSetPendingReg_SPEC;
impl crate::sealed::RegSpec for PdcSetPendingReg_SPEC {
    type DataType = u32;
}
#[doc = "Set a pending PDC bit"]
pub type PdcSetPendingReg = crate::RegValueT<PdcSetPendingReg_SPEC>;

impl PdcSetPendingReg {
    #[doc = "Writing to this field sets the PDC wakeup request and IRQ.\nThe data controls which request is acknowledged"]
    #[inline(always)]
    pub fn pdc_set_pending(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, PdcSetPendingReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, PdcSetPendingReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for PdcSetPendingReg {
    #[inline(always)]
    fn default() -> PdcSetPendingReg {
        <crate::RegValueT<PdcSetPendingReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
