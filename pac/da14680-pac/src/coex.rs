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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:57 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"COEX registers"]
unsafe impl ::core::marker::Send for super::Coex {}
unsafe impl ::core::marker::Sync for super::Coex {}
impl super::Coex {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "COEX BLE PTI Control Register"]
    #[inline(always)]
    pub const fn coex_ble_pti_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexBlePtiReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexBlePtiReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "COEX Control Register"]
    #[inline(always)]
    pub const fn coex_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "COEX Diagnostic Monitor Register"]
    #[inline(always)]
    pub const fn coex_diag_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexDiagReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexDiagReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "COEX FTDF PTI Control Register"]
    #[inline(always)]
    pub const fn coex_ftdf_pti_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexFtdfPtiReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexFtdfPtiReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "COEX Interrupt Mask Register"]
    #[inline(always)]
    pub const fn coex_int_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexIntMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexIntMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "COEX Interrupt Status Register"]
    #[inline(always)]
    pub const fn coex_int_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexIntStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexIntStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri16_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri16Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri16Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri17_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri17Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri17Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "COEX Priority Register"]
    #[inline(always)]
    pub const fn coex_pri9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "COEX Status Register"]
    #[inline(always)]
    pub const fn coex_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexBlePtiReg_SPEC;
impl crate::sealed::RegSpec for CoexBlePtiReg_SPEC {
    type DataType = u16;
}

#[doc = "COEX BLE PTI Control Register"]
pub type CoexBlePtiReg = crate::RegValueT<CoexBlePtiReg_SPEC>;

impl CoexBlePtiReg {
    #[doc = "This value specifies the PTI value that characterizes the next BLE transaction that will be initiated on the following \"ble_active\" positive edge. The value should remain constant during the high period of the \"ble_active\" signal."]
    #[inline(always)]
    pub fn coex_ble_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexBlePtiReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexBlePtiReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexBlePtiReg {
    #[inline(always)]
    fn default() -> CoexBlePtiReg {
        <crate::RegValueT<CoexBlePtiReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexCtrlReg_SPEC;
impl crate::sealed::RegSpec for CoexCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Control Register"]
pub type CoexCtrlReg = crate::RegValueT<CoexCtrlReg_SPEC>;

impl CoexCtrlReg {
    #[doc = "If set to \"1\" then all BLE requests are ignored by masking the internal \"ble_active\" signal. Refer also to IGNORE_BLE_STAT."]
    #[inline(always)]
    pub fn ignore_ble(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set to \"1\" then all FTDF requests are ignored by masking the internal \"ftdf_active\" signal. Refer also to IGNORE_FTDF_STAT."]
    #[inline(always)]
    pub fn ignore_ftdf(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set to \"1\" then all EXT requests are ignored by masking the internal \"ext_act\" signal (\"ext_act\" is the logical OR of \"ext_act0\" and \"ext_act1\"). Refer also to IGNORE_EXT_STAT."]
    #[inline(always)]
    pub fn ignore_ext(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select the logic driving the BLE core input \"ble.radio_busy\":\n0: (decision==BLE) AND rfcu.radio_busy.\n1: Hold to \"0\".\n2: (decision==FTDF) OR (decision==EXT) OR rfcu.radio_busy.\n3: (decision==FTDF) OR (decision==EXT).\nSelection \"0\" is the default, while selection \"2\" is the recommended value if the BLE SW supports it."]
    #[inline(always)]
    pub fn sel_ble_radio_busy(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, CoexCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set to \"1\" then the COEX block will drive the WLAN_TX and WLAN_RX inputs of the BLE core. Otherwise both BLE inputs will be forced to \"0\"."]
    #[inline(always)]
    pub fn sel_ble_wlan_tx_rx(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "It controls the source of the FTDF PTI value that the COEX Arbiter will use.\nIf \"0\" then use the COEX_FTDF_PTI_REG.\nIf \"1\" then use the PTI value provided by the FTDF core."]
    #[inline(always)]
    pub fn sel_ftdf_pti(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set to \"1\" and the COEX decision is different than \"FTDF\", then the CCA_STAT signal going to FTDF (generated from the radio) will be forced to \"1\"; otherwise the FTDF.CCA_STAT will be driven with the signal generated from the radio.\nRecommended value for SEL_FTDF_CCA is \"1\"."]
    #[inline(always)]
    pub fn sel_ftdf_cca(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The COEX block can provide internal diagnostic signals by overwriting the BLE diagnostic bus, which is forwarded to GPIO multiplexing. There is no need to program the BLE registers, but only this field and the GPIO PID fields. The encoding of this bitfield is:\n0: Don\'t overwrite any BLE diagnostic signal.\n1: Overwrite the BLE Diagnostic bits 2 down to 0:\nP2\\[2\\]: \"closing\" sub-state\nP2\\[1:0\\]:\"decision\" state\n2: Overwrite the BLE Diagnostic bits 5 down to 3:\nP1\\[2\\]: \"closing\" sub-state\nP1\\[1:0\\]:\"decision\" state\n3: Reserved.."]
    #[inline(always)]
    pub fn sel_coex_diag(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CoexCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Controls the behavior of the SMART_ACT (and SMART_PRI as a consequence).\nIf SMART_ACT_IMPL=\"0\" then if any BLE or FTDF MAC request is active then SMART_ACT will be asserted. SMART_ACT will actually be the logical OR of \"ble_active\" and \"ftdf_active\" internal signals. SMART_ACT will be asserted regardless the decision of the Arbiter to allow or disallow the access to the on-chip radio from the active MAC(s).\nif SMART_ACT_IMPL=\"1\" then if the Arbiter\'s decision is to allow EXTernal MAC, then keep SMART_ACT to \"0\", otherwise follow the implementation of SMART_ACT_IMPL=\"0\"."]
    #[inline(always)]
    pub fn smart_act_impl(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set to \"1\" then the current BLE transaction will complete normally and after that no further decision will be taken by the arbiter. The SW must keep this bit to \"1\" as long as it performs write operations on the COEX_PRI*_REG registers. As soon as the update on the priority registers will be completed, the SW should clear this bit.\nNote: This bit is updated with the COEX_CLK, so depending on the relationship between the PCLK and COEX_CLK periods a write operation to this bit may be effective in more than one PCLK clock cycles, e.g. when the COEX_CLK rate is slower than the PCLK."]
    #[inline(always)]
    pub fn prging_arbiter(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexCtrlReg {
    #[inline(always)]
    fn default() -> CoexCtrlReg {
        <crate::RegValueT<CoexCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexDiagReg_SPEC;
impl crate::sealed::RegSpec for CoexDiagReg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Diagnostic Monitor Register"]
pub type CoexDiagReg = crate::RegValueT<CoexDiagReg_SPEC>;

impl CoexDiagReg {
    #[doc = "provides the current value of the diagnostic bus forwarded to the GPIO multiplexing (named PPA). Refer to the Pxy_MODE_REG\\[PID\\] value BLE_DIAG."]
    #[inline(always)]
    pub fn coex_diag_mon(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, CoexDiagReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,CoexDiagReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexDiagReg {
    #[inline(always)]
    fn default() -> CoexDiagReg {
        <crate::RegValueT<CoexDiagReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexFtdfPtiReg_SPEC;
impl crate::sealed::RegSpec for CoexFtdfPtiReg_SPEC {
    type DataType = u16;
}

#[doc = "COEX FTDF PTI Control Register"]
pub type CoexFtdfPtiReg = crate::RegValueT<CoexFtdfPtiReg_SPEC>;

impl CoexFtdfPtiReg {
    #[doc = "This value specifies the PTI value that characterizes the next FTDF transaction that will be initiated on the following \"ftdf_active\" positive edge. The value should remain constant during the high period of the \"ftdf_active\" signal. Refer also to bitfield COEX_CTRL_REG.SEL_FTDF_PTI."]
    #[inline(always)]
    pub fn coex_ftdf_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexFtdfPtiReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexFtdfPtiReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexFtdfPtiReg {
    #[inline(always)]
    fn default() -> CoexFtdfPtiReg {
        <crate::RegValueT<CoexFtdfPtiReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexIntMaskReg_SPEC;
impl crate::sealed::RegSpec for CoexIntMaskReg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Interrupt Mask Register"]
pub type CoexIntMaskReg = crate::RegValueT<CoexIntMaskReg_SPEC>;

impl CoexIntMaskReg {
    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_DECISION_SW\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_decision_sw(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_START_MID\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_start_mid(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_CLOSING_BRK\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_closing_brk(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_RADIO_BUSY_F\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_radio_busy_f(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_RADIO_BUSY_R\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_radio_busy_r(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_BLE_ACTIVE_F\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_ble_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_BLE_ACTIVE_R\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_ble_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_FTDF_ACTIVE_F\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_FTDF_ACTIVE_R\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_EXT_ACT_F\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_ext_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_EXT_ACT_R\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_ext_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_SMART_PRI_F\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_smart_pri_f(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_SMART_PRI_R\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_smart_pri_r(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_SMART_ACT_F\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_smart_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \"1\" then a \"1\" on COEX_INT_STAT_REG\\[COEX_IRQ_ON_SMART_ACT_R\\] will cause COEX_IRQ_STAT to be set also to \"1\"."]
    #[inline(always)]
    pub fn coex_irq_on_smart_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If set to \"1\" then sent an COEX_IRQ event to CPU as long as COEX_INT_STAT_REG\\[COEX_IRQ_STAT\\] is \"1\".\nIf cleared then don\'t sent any IRQ event to CPU."]
    #[inline(always)]
    pub fn coex_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexIntMaskReg {
    #[inline(always)]
    fn default() -> CoexIntMaskReg {
        <crate::RegValueT<CoexIntMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexIntStatReg_SPEC;
impl crate::sealed::RegSpec for CoexIntStatReg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Interrupt Status Register"]
pub type CoexIntStatReg = crate::RegValueT<CoexIntStatReg_SPEC>;

impl CoexIntStatReg {
    #[doc = "IRQ event when the decision switches to a new MAC. It ignores the intermediate transitions to DECISION==NONE.\nNote that after a Radio domain reset, the first transition of the decision to any MAC will also trigger this event."]
    #[inline(always)]
    pub fn coex_irq_on_decision_sw(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event when the decision switches to a MAC, while the TX_EN or RX_EN of this MAC are high. This event signals a potential break of a transmission or reception."]
    #[inline(always)]
    pub fn coex_irq_on_start_mid(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ if while entering into \"closing\" sub-state, the TX_EN or RX_EN are active. This event signals a potential break of a transmission or reception."]
    #[inline(always)]
    pub fn coex_irq_on_closing_brk(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on falling edge of RADIO_BUSY."]
    #[inline(always)]
    pub fn coex_irq_on_radio_busy_f(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on rising edge of RADIO_BUSY."]
    #[inline(always)]
    pub fn coex_irq_on_radio_busy_r(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on falling edge of BLE_ACTIVE internal signal."]
    #[inline(always)]
    pub fn coex_irq_on_ble_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on rising edge of BLE_ACTIVE internal signal."]
    #[inline(always)]
    pub fn coex_irq_on_ble_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on falling edge of FTDF_ACTIVE internal signal."]
    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on rising edge of FTDF_ACTIVE internal signal."]
    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "RQ event on falling edge of EXT_ACT."]
    #[inline(always)]
    pub fn coex_irq_on_ext_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on rising edge of EXT_ACT."]
    #[inline(always)]
    pub fn coex_irq_on_ext_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on falling edge of SMART_PRI."]
    #[inline(always)]
    pub fn coex_irq_on_smart_pri_f(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on rising edge of SMART_PRI."]
    #[inline(always)]
    pub fn coex_irq_on_smart_pri_r(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on falling edge of SMART_ACT."]
    #[inline(always)]
    pub fn coex_irq_on_smart_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "IRQ event on rising edge of SMART_ACT."]
    #[inline(always)]
    pub fn coex_irq_on_smart_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "For each COEX_IRQ_ON_* bitfield of COEX_INT_STAT_REG the corresponding mask is applied and afterwards all the intermediate results are combined with a logical OR in order to produce the COEX_IRQ_STAT bitfield. If furthermore the COEX_INT_MASK_REG\\[COEX_IRQ_MASK\\] is set to \"1\", then a COEX_IRQ signal will be forwarded to the CPU.\n\nNote: Each COEX_IRQ_ON_* bitfield of COEX_INT_STAT_REG will be set to \"1\" on the detection of the corresponding event and will be cleared to \"0\" on the read of COEX_INT_STAT_REG. The automated clear may delay a couple of PCLK cycles, depending on the relationship between PCLK and COEX_CLK."]
    #[inline(always)]
    pub fn coex_irq_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexIntStatReg {
    #[inline(always)]
    fn default() -> CoexIntStatReg {
        <crate::RegValueT<CoexIntStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri10Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri10Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri10Reg = crate::RegValueT<CoexPri10Reg_SPEC>;

impl CoexPri10Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri10Reg {
    #[inline(always)]
    fn default() -> CoexPri10Reg {
        <crate::RegValueT<CoexPri10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri11Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri11Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri11Reg = crate::RegValueT<CoexPri11Reg_SPEC>;

impl CoexPri11Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri11Reg {
    #[inline(always)]
    fn default() -> CoexPri11Reg {
        <crate::RegValueT<CoexPri11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri12Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri12Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri12Reg = crate::RegValueT<CoexPri12Reg_SPEC>;

impl CoexPri12Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri12Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri12Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri12Reg {
    #[inline(always)]
    fn default() -> CoexPri12Reg {
        <crate::RegValueT<CoexPri12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri13Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri13Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri13Reg = crate::RegValueT<CoexPri13Reg_SPEC>;

impl CoexPri13Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri13Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri13Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri13Reg {
    #[inline(always)]
    fn default() -> CoexPri13Reg {
        <crate::RegValueT<CoexPri13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri14Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri14Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri14Reg = crate::RegValueT<CoexPri14Reg_SPEC>;

impl CoexPri14Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri14Reg {
    #[inline(always)]
    fn default() -> CoexPri14Reg {
        <crate::RegValueT<CoexPri14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri15Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri15Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri15Reg = crate::RegValueT<CoexPri15Reg_SPEC>;

impl CoexPri15Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri15Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri15Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri15Reg {
    #[inline(always)]
    fn default() -> CoexPri15Reg {
        <crate::RegValueT<CoexPri15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri16Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri16Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri16Reg = crate::RegValueT<CoexPri16Reg_SPEC>;

impl CoexPri16Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri16Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri16Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri16Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri16Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri16Reg {
    #[inline(always)]
    fn default() -> CoexPri16Reg {
        <crate::RegValueT<CoexPri16Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri17Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri17Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri17Reg = crate::RegValueT<CoexPri17Reg_SPEC>;

impl CoexPri17Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri17Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri17Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri17Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri17Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri17Reg {
    #[inline(always)]
    fn default() -> CoexPri17Reg {
        <crate::RegValueT<CoexPri17Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri1Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri1Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri1Reg = crate::RegValueT<CoexPri1Reg_SPEC>;

impl CoexPri1Reg {
    #[doc = "Specifies the MAC that has been assigned with the specific priority level. The MAC encoding follows the COEX_DECISION bitfield encoding."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The priority level specified by this register will be applied to the packets coming from the MAC specified by the COEX_PRI_MAC bitfield and characterized with the PTI value specified by the COEX_PRI_PTI bitfield."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri1Reg {
    #[inline(always)]
    fn default() -> CoexPri1Reg {
        <crate::RegValueT<CoexPri1Reg_SPEC> as RegisterValue<_>>::new(24)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri2Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri2Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri2Reg = crate::RegValueT<CoexPri2Reg_SPEC>;

impl CoexPri2Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri2Reg {
    #[inline(always)]
    fn default() -> CoexPri2Reg {
        <crate::RegValueT<CoexPri2Reg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri3Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri3Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri3Reg = crate::RegValueT<CoexPri3Reg_SPEC>;

impl CoexPri3Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri3Reg {
    #[inline(always)]
    fn default() -> CoexPri3Reg {
        <crate::RegValueT<CoexPri3Reg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri4Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri4Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri4Reg = crate::RegValueT<CoexPri4Reg_SPEC>;

impl CoexPri4Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri4Reg {
    #[inline(always)]
    fn default() -> CoexPri4Reg {
        <crate::RegValueT<CoexPri4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri5Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri5Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri5Reg = crate::RegValueT<CoexPri5Reg_SPEC>;

impl CoexPri5Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri5Reg {
    #[inline(always)]
    fn default() -> CoexPri5Reg {
        <crate::RegValueT<CoexPri5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri6Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri6Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri6Reg = crate::RegValueT<CoexPri6Reg_SPEC>;

impl CoexPri6Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri6Reg {
    #[inline(always)]
    fn default() -> CoexPri6Reg {
        <crate::RegValueT<CoexPri6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri7Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri7Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri7Reg = crate::RegValueT<CoexPri7Reg_SPEC>;

impl CoexPri7Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri7Reg {
    #[inline(always)]
    fn default() -> CoexPri7Reg {
        <crate::RegValueT<CoexPri7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri8Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri8Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri8Reg = crate::RegValueT<CoexPri8Reg_SPEC>;

impl CoexPri8Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri8Reg {
    #[inline(always)]
    fn default() -> CoexPri8Reg {
        <crate::RegValueT<CoexPri8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri9Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri9Reg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Priority Register"]
pub type CoexPri9Reg = crate::RegValueT<CoexPri9Reg_SPEC>;

impl CoexPri9Reg {
    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Refer to COEX_PRI1_REG."]
    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri9Reg {
    #[inline(always)]
    fn default() -> CoexPri9Reg {
        <crate::RegValueT<CoexPri9Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexStatReg_SPEC;
impl crate::sealed::RegSpec for CoexStatReg_SPEC {
    type DataType = u16;
}

#[doc = "COEX Status Register"]
pub type CoexStatReg = crate::RegValueT<CoexStatReg_SPEC>;

impl CoexStatReg {
    #[doc = "This signal is constantly \"1\" on FTDF-only chips.\nIf set to \"1\" then all BLE requests are ignored by masking immediately the request signal from the BLE.\nIn more detail, the internal signal \"ble_active\" is the logical AND of this bitfield and the \"ble.event_in_process\"."]
    #[inline(always)]
    pub fn ignore_ble_stat(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This signal is constantly \"1\" on BLE-only chips.\nIf set to \"1\" then all FTDF requests are ignored by masking immediately the request signal from the FTDF.\nIn more detail, the internal signal \"ftdf_active\" is the logical AND of this bitfield and the \"ftdf.phy_en\"."]
    #[inline(always)]
    pub fn ignore_ftdf_stat(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "If set to \"1\" then all EXT requests are ignored by masking immediately the request signal from the external MAC.\nIn more detail, the internal signal \"ext_active\" is the logical AND of this bitfield and the \"ext_act\"."]
    #[inline(always)]
    pub fn ignore_ext_stat(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Current state of RADIO_BUSY signal generated from RFCU, which is the logical OR among all Radio DCFs.\nNote that the arbiter will process this value with one COEX clock cycle delay."]
    #[inline(always)]
    pub fn coex_radio_busy(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Current state of the pin."]
    #[inline(always)]
    pub fn ext_act1(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Current state of the pin."]
    #[inline(always)]
    pub fn ext_act0(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Current state of the pin."]
    #[inline(always)]
    pub fn smart_pri(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Current state of the pin."]
    #[inline(always)]
    pub fn smart_act(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Provides the value of the \"CLOSING\" substate."]
    #[inline(always)]
    pub fn coex_decision_closing(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Decision values:\n0: Decision is NONE.\n1: Decision is BLE.\n2: Decision is FTDF.\n3: Decision is EXT.\nNote: If \"0\" (i.e. decision is NONE) then no MAC will have access to the on-chip radio. As a consequence, the SMART_PRI signal will stay low, since no on-chip (SMART) MAC will have priority.\nNote: The decision NONE will be held as long as there is no \"*_active\" internal signal from BLE, FTDF or EXT. Also, if in programming state and the last transaction has been finished, then the decision will be held also to NONE.\nNote: While in programming mode, the COEX_PRIx_REGs are considered as invalid, which means that no new decision can be taken."]
    #[inline(always)]
    pub fn coex_decision(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CoexStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Provides the number \"x\" of the COEX_PRIx_REG that win the last arbitration cycle. If \"0\" then it is a null pointer, pointing to no COEX_PRIx_REG."]
    #[inline(always)]
    pub fn coex_decision_ptr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, CoexStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexStatReg {
    #[inline(always)]
    fn default() -> CoexStatReg {
        <crate::RegValueT<CoexStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
