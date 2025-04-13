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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:24:23 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ble580_gr01 registers"]
unsafe impl ::core::marker::Send for super::Ble580Gr01 {}
unsafe impl ::core::marker::Sync for super::Ble580Gr01 {}
impl super::Ble580Gr01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Active scan register"]
    #[inline(always)]
    pub const fn ble_actscanstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleActscanstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleActscanstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Advertising Channel Map"]
    #[inline(always)]
    pub const fn ble_advchmap_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAdvchmapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAdvchmapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Advertising Packet Interval"]
    #[inline(always)]
    pub const fn ble_advtim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAdvtimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAdvtimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Start AES register"]
    #[inline(always)]
    pub const fn ble_aescntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAescntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAescntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "AES encryption key"]
    #[inline(always)]
    pub const fn ble_aeskey127_96_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAeskey12796Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAeskey12796Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "AES encryption key"]
    #[inline(always)]
    pub const fn ble_aeskey31_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAeskey310Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAeskey310Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "AES encryption key"]
    #[inline(always)]
    pub const fn ble_aeskey63_32_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAeskey6332Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAeskey6332Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "AES encryption key"]
    #[inline(always)]
    pub const fn ble_aeskey95_64_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAeskey9564Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAeskey9564Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "Pointer to the block to encrypt/decrypt"]
    #[inline(always)]
    pub const fn ble_aesptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleAesptrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleAesptrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "Base Time Counter"]
    #[inline(always)]
    pub const fn ble_basetimecntcorr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleBasetimecntcorrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleBasetimecntcorrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Base time reference counter"]
    #[inline(always)]
    pub const fn ble_basetimecnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleBasetimecntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleBasetimecntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "BLE device address LSB register"]
    #[inline(always)]
    pub const fn ble_bdaddrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleBdaddrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleBdaddrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "BLE device address MSB register"]
    #[inline(always)]
    pub const fn ble_bdaddru_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleBdaddruReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleBdaddruReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "BLE Control Register 2"]
    #[inline(always)]
    pub const fn ble_cntl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleCntl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleCntl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Rx Descriptor Pointer for the Receive Buffer Chained List"]
    #[inline(always)]
    pub const fn ble_currentrxdescptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleCurrentrxdescptrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleCurrentrxdescptrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Upper limit for the memory zone"]
    #[inline(always)]
    pub const fn ble_debugaddmax_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDebugaddmaxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDebugaddmaxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Lower limit for the memory zone"]
    #[inline(always)]
    pub const fn ble_debugaddmin_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDebugaddminReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDebugaddminReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Deep-Sleep control register"]
    #[inline(always)]
    pub const fn ble_deepslcntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDeepslcntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDeepslcntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Duration of the last deep sleep phase register"]
    #[inline(always)]
    pub const fn ble_deepslstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDeepslstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDeepslstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Time (measured in Low Power clock cycles) in Deep Sleep Mode before waking-up the device"]
    #[inline(always)]
    pub const fn ble_deepslwkup_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDeepslwkupReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDeepslwkupReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Diagnostics Register"]
    #[inline(always)]
    pub const fn ble_diagcntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDiagcntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDiagcntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Debug use only"]
    #[inline(always)]
    pub const fn ble_diagstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDiagstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDiagstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Time in low power oscillator cycles register"]
    #[inline(always)]
    pub const fn ble_enbpreset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleEnbpresetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleEnbpresetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Error Type Status registers"]
    #[inline(always)]
    pub const fn ble_errortypestat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleErrortypestatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleErrortypestatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Phase correction value register"]
    #[inline(always)]
    pub const fn ble_finecntcorr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleFinecntcorrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleFinecntcorrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Fine time reference counter"]
    #[inline(always)]
    pub const fn ble_finetimecnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleFinetimecntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleFinetimecntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Fine Timer Target value"]
    #[inline(always)]
    pub const fn ble_finetimtgt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleFinetimtgtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleFinetimtgtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[doc = "Gross Timer Target value"]
    #[inline(always)]
    pub const fn ble_grosstimtgt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleGrosstimtgtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleGrosstimtgtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "Interrupt acknowledge register"]
    #[inline(always)]
    pub const fn ble_intack_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleIntackReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleIntackReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Interrupt controller register"]
    #[inline(always)]
    pub const fn ble_intcntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleIntcntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleIntcntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt raw status register"]
    #[inline(always)]
    pub const fn ble_intrawstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleIntrawstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleIntrawstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt status register"]
    #[inline(always)]
    pub const fn ble_intstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleIntstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleIntstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Radio interface control register"]
    #[inline(always)]
    pub const fn ble_radiocntl0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRadiocntl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRadiocntl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Radio interface control register"]
    #[inline(always)]
    pub const fn ble_radiocntl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRadiocntl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRadiocntl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "RX/TX power up/down phase register"]
    #[inline(always)]
    pub const fn ble_radiopwrupdn_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRadiopwrupdnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRadiopwrupdnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "RF Testing Register"]
    #[inline(always)]
    pub const fn ble_rftestcntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRftestcntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRftestcntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "BLE/RF Diagnostic IRQ Control Register"]
    #[inline(always)]
    pub const fn ble_rf_diagirq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRfDiagirqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRfDiagirqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "BLE Control register"]
    #[inline(always)]
    pub const fn ble_rwbtlecntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRwbtlecntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRwbtlecntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn ble_rwbtleconf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRwbtleconfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRwbtleconfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "AES / CCM plain MIC value"]
    #[inline(always)]
    pub const fn ble_rxmicval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRxmicvalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRxmicvalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[doc = "Samples the Base Time Counter"]
    #[inline(always)]
    pub const fn ble_sampleclk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleSampleclkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleSampleclkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[doc = "Software Profiling register"]
    #[inline(always)]
    pub const fn ble_swprofiling_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleSwprofilingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleSwprofilingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Timing Generator Register"]
    #[inline(always)]
    pub const fn ble_timgencntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleTimgencntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleTimgencntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "AES / CCM plain MIC value"]
    #[inline(always)]
    pub const fn ble_txmicval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleTxmicvalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleTxmicvalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "Version register"]
    #[inline(always)]
    pub const fn ble_version_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleVersionReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleVersionReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Devices in white list"]
    #[inline(always)]
    pub const fn ble_wlnbdev_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleWlnbdevReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleWlnbdevReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "Start address of private devices list"]
    #[inline(always)]
    pub const fn ble_wlprivaddptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleWlprivaddptrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleWlprivaddptrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[doc = "Start address of public devices list"]
    #[inline(always)]
    pub const fn ble_wlpubaddptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleWlpubaddptrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleWlpubaddptrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleActscanstatReg_SPEC;
impl crate::sealed::RegSpec for BleActscanstatReg_SPEC {
    type DataType = u32;
}
#[doc = "Active scan register"]
pub type BleActscanstatReg = crate::RegValueT<BleActscanstatReg_SPEC>;

impl BleActscanstatReg {
    #[doc = "Active scan mode back-off counter initialization value."]
    #[inline(always)]
    pub fn backoff(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, BleActscanstatReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1ff,
            1,
            0,
            u16,
            BleActscanstatReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Active scan mode upper limit counter value."]
    #[inline(always)]
    pub fn upperlimit(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, BleActscanstatReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, BleActscanstatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleActscanstatReg {
    #[inline(always)]
    fn default() -> BleActscanstatReg {
        <crate::RegValueT<BleActscanstatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAdvchmapReg_SPEC;
impl crate::sealed::RegSpec for BleAdvchmapReg_SPEC {
    type DataType = u32;
}
#[doc = "Advertising Channel Map"]
pub type BleAdvchmapReg = crate::RegValueT<BleAdvchmapReg_SPEC>;

impl BleAdvchmapReg {
    #[doc = "Advertising Channel Map, defined as per the advertising connection settings. Contains advertising channels index 37 to 39. If ADVCHMAP\\[i\\] equals:\n0: Do not use data channel i+37.\n1: Use data channel i+37."]
    #[inline(always)]
    pub fn advchmap(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, BleAdvchmapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8, BleAdvchmapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleAdvchmapReg {
    #[inline(always)]
    fn default() -> BleAdvchmapReg {
        <crate::RegValueT<BleAdvchmapReg_SPEC> as RegisterValue<_>>::new(7)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAdvtimReg_SPEC;
impl crate::sealed::RegSpec for BleAdvtimReg_SPEC {
    type DataType = u32;
}
#[doc = "Advertising Packet Interval"]
pub type BleAdvtimReg = crate::RegValueT<BleAdvtimReg_SPEC>;

impl BleAdvtimReg {
    #[doc = "Advertising Packet Interval defines the time interval in between two ADV_xxx packet sent.\nValue is in usec.\nValue to program depends of the used Advertising Packet type and the device filtering policy. Please refer to Table 3-10 for details about ADVINT programming range."]
    #[inline(always)]
    pub fn advint(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, BleAdvtimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, BleAdvtimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleAdvtimReg {
    #[inline(always)]
    fn default() -> BleAdvtimReg {
        <crate::RegValueT<BleAdvtimReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAescntlReg_SPEC;
impl crate::sealed::RegSpec for BleAescntlReg_SPEC {
    type DataType = u32;
}
#[doc = "Start AES register"]
pub type BleAescntlReg = crate::RegValueT<BleAescntlReg_SPEC>;

impl BleAescntlReg {
    #[doc = "Writing a 1 starts AES-128 ciphering process.\nThis bit is reset once the process is finished (i.e BLE_CRYPT_IRQ interrupt occurs, even masked)"]
    #[inline(always)]
    pub fn aes_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleAescntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BleAescntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleAescntlReg {
    #[inline(always)]
    fn default() -> BleAescntlReg {
        <crate::RegValueT<BleAescntlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAeskey12796Reg_SPEC;
impl crate::sealed::RegSpec for BleAeskey12796Reg_SPEC {
    type DataType = u32;
}
#[doc = "AES encryption key"]
pub type BleAeskey12796Reg = crate::RegValueT<BleAeskey12796Reg_SPEC>;

impl BleAeskey12796Reg {
    #[doc = "AES encryption 128-bit key. Bit 127 down to 96"]
    #[inline(always)]
    pub fn aeskey127_96(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        BleAeskey12796Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleAeskey12796Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleAeskey12796Reg {
    #[inline(always)]
    fn default() -> BleAeskey12796Reg {
        <crate::RegValueT<BleAeskey12796Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAeskey310Reg_SPEC;
impl crate::sealed::RegSpec for BleAeskey310Reg_SPEC {
    type DataType = u32;
}
#[doc = "AES encryption key"]
pub type BleAeskey310Reg = crate::RegValueT<BleAeskey310Reg_SPEC>;

impl BleAeskey310Reg {
    #[doc = "AES encryption 128-bit key. Bit 31 down to 0"]
    #[inline(always)]
    pub fn aeskey31_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        BleAeskey310Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleAeskey310Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleAeskey310Reg {
    #[inline(always)]
    fn default() -> BleAeskey310Reg {
        <crate::RegValueT<BleAeskey310Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAeskey6332Reg_SPEC;
impl crate::sealed::RegSpec for BleAeskey6332Reg_SPEC {
    type DataType = u32;
}
#[doc = "AES encryption key"]
pub type BleAeskey6332Reg = crate::RegValueT<BleAeskey6332Reg_SPEC>;

impl BleAeskey6332Reg {
    #[doc = "AES encryption 128-bit key. Bit 63 down to 32"]
    #[inline(always)]
    pub fn aeskey63_32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        BleAeskey6332Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleAeskey6332Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleAeskey6332Reg {
    #[inline(always)]
    fn default() -> BleAeskey6332Reg {
        <crate::RegValueT<BleAeskey6332Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAeskey9564Reg_SPEC;
impl crate::sealed::RegSpec for BleAeskey9564Reg_SPEC {
    type DataType = u32;
}
#[doc = "AES encryption key"]
pub type BleAeskey9564Reg = crate::RegValueT<BleAeskey9564Reg_SPEC>;

impl BleAeskey9564Reg {
    #[doc = "AES encryption 128-bit key. Bit 95 down to 64"]
    #[inline(always)]
    pub fn aeskey95_64(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        BleAeskey9564Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleAeskey9564Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleAeskey9564Reg {
    #[inline(always)]
    fn default() -> BleAeskey9564Reg {
        <crate::RegValueT<BleAeskey9564Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAesptrReg_SPEC;
impl crate::sealed::RegSpec for BleAesptrReg_SPEC {
    type DataType = u32;
}
#[doc = "Pointer to the block to encrypt/decrypt"]
pub type BleAesptrReg = crate::RegValueT<BleAesptrReg_SPEC>;

impl BleAesptrReg {
    #[doc = "Pointer to the memory zone where the block to encrypt/decrypt is stored."]
    #[inline(always)]
    pub fn aesptr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BleAesptrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, BleAesptrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleAesptrReg {
    #[inline(always)]
    fn default() -> BleAesptrReg {
        <crate::RegValueT<BleAesptrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleBasetimecntcorrReg_SPEC;
impl crate::sealed::RegSpec for BleBasetimecntcorrReg_SPEC {
    type DataType = u32;
}
#[doc = "Base Time Counter"]
pub type BleBasetimecntcorrReg = crate::RegValueT<BleBasetimecntcorrReg_SPEC>;

impl BleBasetimecntcorrReg {
    #[doc = "Base Time Counter correction value."]
    #[inline(always)]
    pub fn basetimecntcorr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffffff,
        1,
        0,
        u32,
        BleBasetimecntcorrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ffffff,
            1,
            0,
            u32,
            BleBasetimecntcorrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleBasetimecntcorrReg {
    #[inline(always)]
    fn default() -> BleBasetimecntcorrReg {
        <crate::RegValueT<BleBasetimecntcorrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleBasetimecntReg_SPEC;
impl crate::sealed::RegSpec for BleBasetimecntReg_SPEC {
    type DataType = u32;
}
#[doc = "Base time reference counter"]
pub type BleBasetimecntReg = crate::RegValueT<BleBasetimecntReg_SPEC>;

impl BleBasetimecntReg {
    #[doc = "Value of the 625us base time reference counter. Updated each time BLE_SAMPLECLK_REG\\[SAMP\\] is written. Used by the SW in order to synchronize with the HW."]
    #[inline(always)]
    pub fn basetimecnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffffff,
        1,
        0,
        u32,
        BleBasetimecntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7ffffff,
            1,
            0,
            u32,
            BleBasetimecntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleBasetimecntReg {
    #[inline(always)]
    fn default() -> BleBasetimecntReg {
        <crate::RegValueT<BleBasetimecntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleBdaddrlReg_SPEC;
impl crate::sealed::RegSpec for BleBdaddrlReg_SPEC {
    type DataType = u32;
}
#[doc = "BLE device address LSB register"]
pub type BleBdaddrlReg = crate::RegValueT<BleBdaddrlReg_SPEC>;

impl BleBdaddrlReg {
    #[doc = "Bluetooth Low Energy Device Address. LSB part."]
    #[inline(always)]
    pub fn bdaddrl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, BleBdaddrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleBdaddrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleBdaddrlReg {
    #[inline(always)]
    fn default() -> BleBdaddrlReg {
        <crate::RegValueT<BleBdaddrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleBdaddruReg_SPEC;
impl crate::sealed::RegSpec for BleBdaddruReg_SPEC {
    type DataType = u32;
}
#[doc = "BLE device address MSB register"]
pub type BleBdaddruReg = crate::RegValueT<BleBdaddruReg_SPEC>;

impl BleBdaddruReg {
    #[doc = "Bluetooth Low Energy Device Address privacy indicator\n0: Public Bluetooth Device Address\n1: Private Bluetooth Device Address"]
    #[inline(always)]
    pub fn priv_npub(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BleBdaddruReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,BleBdaddruReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bluetooth Low Energy Device Address. MSB part."]
    #[inline(always)]
    pub fn bdaddru(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BleBdaddruReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, BleBdaddruReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleBdaddruReg {
    #[inline(always)]
    fn default() -> BleBdaddruReg {
        <crate::RegValueT<BleBdaddruReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleCntl2Reg_SPEC;
impl crate::sealed::RegSpec for BleCntl2Reg_SPEC {
    type DataType = u32;
}
#[doc = "BLE Control Register 2"]
pub type BleCntl2Reg = crate::RegValueT<BleCntl2Reg_SPEC>;

impl BleCntl2Reg {
    #[doc = "0: Select Peak-hold RSSI value (default).\n1: Select current Average RSSI value."]
    #[inline(always)]
    pub fn ble_rssi_sel(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The status of the BLE_WAKEUP_LP_IRQ. The Interrupt Service Routine of BLE_WAKEUP_LP_IRQ should return only when the WAKEUPLPSTAT is cleared.\nNote that BLE_WAKEUP_LP_IRQ is automatically acknowledged after the power up of the Radio Subsystem, plus one Low Power Clock period."]
    #[inline(always)]
    pub fn wakeuplpstat(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Keep to 0."]
    #[inline(always)]
    pub fn sw_rpl_spi(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Keep to 0."]
    #[inline(always)]
    pub fn bb_only(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Keep to 0."]
    #[inline(always)]
    pub fn radio_only(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BLE Clock Select.\nSpecifies the BLE master clock absolute frequency in MHz.\nTypical values are 16 and 8.\nValue depends on the selected XTAL frequency and the value of CLK_RADIO_REG\\[BLE_DIV\\] bitfield. For example, if XTAL oscillates at 16MHz and CLK_RADIO_REG\\[BLE_DIV\\] = 1 (divide by 2), then BLE master clock frequency is 8MHz and BLE_CLK_SEL should be set to value 8.\nThe selected BLE master clock frequency (affected by BLE_DIV and BLE_CLK_SEL) must be modified and set only during the initialization time, i.e. before setting BLE_RWBTLECNTL_REG\\[RWBLE_EN\\] to 1.\nRefer also to BLE_RWBTLECONF_REG\\[CLK_SEL\\]."]
    #[inline(always)]
    pub fn ble_clk_sel(
        self,
    ) -> crate::common::RegisterField<9, 0x3f, 1, 0, u8, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x3f,1,0,u8, BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This active high signal indicates when it is allowed for the BLE core (embedded in the Radio sub-System power domain) to be powered down.\nAfter the assertion of the BLE_DEEPSLCNTL_REG\\[DEEP_SLEEP_ON\\] a hardware sequence based on the Low Power clock will cause the assertion of RADIO_PWRDN_ALLOW. The RADIO_PWRDN_ALLOW will be cleared to \'0\' when the BLE core exits from the sleep state, i.e. when the BLE_SLP_IRQ will be asserted."]
    #[inline(always)]
    pub fn radio_pwrdn_allow(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The SW can only write a \'0\' to this bit.\nWhenever a positive edge of the low power clock used by the BLE Timers is detected, then the HW will automatically set this bit to \'1\'. This functionality will not work if BLE Timer is in reset state (refer to CLK_RADIO_REG\\[BLE_LP_RESET\\]).\nThis bit can be used for SW synchronization, to debug the low power clock, etc."]
    #[inline(always)]
    pub fn mon_lp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: BLE uses low power clock\n1: BLE uses master clock"]
    #[inline(always)]
    pub fn ble_clk_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "BLE/RADIO Diagnostic Port Reverse order.\nWhen this bit is \'1\', the mapping of the diagnostic bus DIAGPORT\\[7:0\\] (controlled by DIAGPORT_SEL) to GPIOs (controlled by Pxy_MODE_REG\\[PID\\]) is reversed. The mapping is:\nIf \'0\' then DIAGPORT\\[7\\] is mapped to P0\\[7\\], etc.\nDIAGPORT\\[4\\] is mapped to P0\\[4\\],\nDIAGPORT\\[3\\] is mapped to P0\\[3\\] and P1\\[3\\], etc. and\nDIAGPORT\\[0\\] is mapped to P0\\[0\\] and P1\\[0\\].\n\nIf \'1\' then DIAGPORT\\[7\\] is mapped to P0\\[0\\] and P1\\[0\\], etc.\nDIAGPORT\\[4\\] is mapped to P0\\[3\\] and P1\\[3\\],\nDIAGPORT\\[3\\] is mapped to P0\\[4\\], etc. and\nDIAGPORT\\[0\\] is mapped to P0\\[7\\]."]
    #[inline(always)]
    pub fn diagport_reverse(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BLE/RADIO Diagnostic Port Selection.\nControls the multiplexing of the internal diagnostic signals towards the 8-bit diagnostic bus DIAGPORT\\[7:0\\]. The DIAGPORT\\[7:0\\] bit order may or may not be reversed by using the DIAGPORT_REVERSE bitfield and then it will be directed to the GPIOs P0\\[7:0\\] and P1\\[3:0\\]. (Note that the P1\\[3:0\\] diagnostic signals are the same with P0\\[3:0\\] signals.)\nThe DIAGPORT\\[7:0\\] value, depending on the DIAGPORT_SEL value, is:\n00: {BLE_DIAG2\\[7:5\\], BLE_DIAG1\\[4:3\\], BLE_DIAG0\\[2:0\\]}\n01: {BLE_DIAG2\\[7:5\\], BLE_DIAG1\\[4:3\\], BLE_DIAG0\\[2\\]\n, wakeup_lp_irq, deep_sleep_stat_32k}\n10: RADIO_DIAG0\\[7:0\\]\n11: RADIO_DIAG1\\[7:0\\]"]
    #[inline(always)]
    pub fn diagport_sel(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Exchange Memory Access Error Mask:\nWhen cleared to \'0\' the EM_ACC_ERR will not cause an BLE_ERROR_IRQ interrupt.\nWhen set to \'1\' an BLE_ERROR_IRQ will be generated as long as EM_ACC_ERR is \'1\'."]
    #[inline(always)]
    pub fn emaccerrmsk(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Exchange Memory Access Error Acknowledge.\nWhen the SW writes a \'1\' to this bit then the EMACCERRSTAT bit will be cleared.\nWhen the SW writes \'0\' it will have no affect.\nThe read value is always \'0\'."]
    #[inline(always)]
    pub fn emaccerrack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleCntl2Reg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,BleCntl2Reg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Exchange Memory Access Error Status:\nThe bit is read-only and can be cleared only by writing a \'1\' at EMACCERRACK bitfield.\nThis bit will be set to \'1\' by the hardware when the controller will access an EM page that is not mapped according to the EM_MAPPING value.\nWhen this bit is \'1\' then the BLE_ERROR_IRQ will be asserted as long as EMACCERRMSK is \'1\'."]
    #[inline(always)]
    pub fn emaccerrstat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleCntl2Reg {
    #[inline(always)]
    fn default() -> BleCntl2Reg {
        <crate::RegValueT<BleCntl2Reg_SPEC> as RegisterValue<_>>::new(4)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleCurrentrxdescptrReg_SPEC;
impl crate::sealed::RegSpec for BleCurrentrxdescptrReg_SPEC {
    type DataType = u32;
}
#[doc = "Rx Descriptor Pointer for the Receive Buffer Chained List"]
pub type BleCurrentrxdescptrReg = crate::RegValueT<BleCurrentrxdescptrReg_SPEC>;

impl BleCurrentrxdescptrReg {
    #[doc = "Rx Descriptor Pointer that determines the starting point of the Receive Buffer Chained List."]
    #[inline(always)]
    pub fn currentrxdescptr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        BleCurrentrxdescptrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            BleCurrentrxdescptrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleCurrentrxdescptrReg {
    #[inline(always)]
    fn default() -> BleCurrentrxdescptrReg {
        <crate::RegValueT<BleCurrentrxdescptrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDebugaddmaxReg_SPEC;
impl crate::sealed::RegSpec for BleDebugaddmaxReg_SPEC {
    type DataType = u32;
}
#[doc = "Upper limit for the memory zone"]
pub type BleDebugaddmaxReg = crate::RegValueT<BleDebugaddmaxReg_SPEC>;

impl BleDebugaddmaxReg {
    #[doc = "Upper limit for the memory zone."]
    #[inline(always)]
    pub fn addmax(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BleDebugaddmaxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            BleDebugaddmaxReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleDebugaddmaxReg {
    #[inline(always)]
    fn default() -> BleDebugaddmaxReg {
        <crate::RegValueT<BleDebugaddmaxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDebugaddminReg_SPEC;
impl crate::sealed::RegSpec for BleDebugaddminReg_SPEC {
    type DataType = u32;
}
#[doc = "Lower limit for the memory zone"]
pub type BleDebugaddminReg = crate::RegValueT<BleDebugaddminReg_SPEC>;

impl BleDebugaddminReg {
    #[doc = "Lower limit for the memory zone."]
    #[inline(always)]
    pub fn addmin(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BleDebugaddminReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            BleDebugaddminReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleDebugaddminReg {
    #[inline(always)]
    fn default() -> BleDebugaddminReg {
        <crate::RegValueT<BleDebugaddminReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDeepslcntlReg_SPEC;
impl crate::sealed::RegSpec for BleDeepslcntlReg_SPEC {
    type DataType = u32;
}
#[doc = "Deep-Sleep control register"]
pub type BleDeepslcntlReg = crate::RegValueT<BleDeepslcntlReg_SPEC>;

impl BleDeepslcntlReg {
    #[doc = "External Wake-Up disable\n0: BLE Core can be woken by external wake-up\n1: BLE Core cannot be woken up by external wake-up"]
    #[inline(always)]
    pub fn extwkupdsb(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleDeepslcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicator of current Deep Sleep clock mux status:\n0: BLE Core is not yet in Deep Sleep Mode\n1: BLE Core is in Deep Sleep Mode (only Low Power Clock is running)"]
    #[inline(always)]
    pub fn deep_sleep_stat(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleDeepslcntlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,BleDeepslcntlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Wake Up Request from BLE Software. Applies when system is in Deep Sleep Mode. It wakes up the BLE Core when written with a 1. Always read as 0. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn soft_wakeup_req(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleDeepslcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "625us base time reference integer and fractional part correction. Applies when system has been woken-up from Deep Sleep Mode. It enables Fine Counter and Base Time counter when written with a 1. Always read as 0. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn deep_sleep_corr_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleDeepslcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: BLE Core in normal active mode\n1: Request BLE Core to switch in deep sleep mode.\nThis bit is reset on DEEP_SLEEP_STAT falling edge."]
    #[inline(always)]
    pub fn deep_sleep_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleDeepslcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Always set to \'3\' when DEEP_SLEEP_ON is set to \'1\'.\nIt controls the generation of BLE_WAKEUP_LP_IRQ."]
    #[inline(always)]
    pub fn deep_sleep_irq_en(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, BleDeepslcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleDeepslcntlReg {
    #[inline(always)]
    fn default() -> BleDeepslcntlReg {
        <crate::RegValueT<BleDeepslcntlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDeepslstatReg_SPEC;
impl crate::sealed::RegSpec for BleDeepslstatReg_SPEC {
    type DataType = u32;
}
#[doc = "Duration of the last deep sleep phase register"]
pub type BleDeepslstatReg = crate::RegValueT<BleDeepslstatReg_SPEC>;

impl BleDeepslstatReg {
    #[doc = "Actual duration of the last deep sleep phase measured in Low Power Clock cycles. DEEPSLDUR is set to zero at the beginning of the deep sleep phase, and is incremented at each Low Power Clock cycle until the end of the deep sleep phase."]
    #[inline(always)]
    pub fn deepsldur(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        BleDeepslstatReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleDeepslstatReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleDeepslstatReg {
    #[inline(always)]
    fn default() -> BleDeepslstatReg {
        <crate::RegValueT<BleDeepslstatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDeepslwkupReg_SPEC;
impl crate::sealed::RegSpec for BleDeepslwkupReg_SPEC {
    type DataType = u32;
}
#[doc = "Time (measured in Low Power clock cycles) in Deep Sleep Mode before waking-up the device"]
pub type BleDeepslwkupReg = crate::RegValueT<BleDeepslwkupReg_SPEC>;

impl BleDeepslwkupReg {
    #[doc = "Determines the time in Low Power Clock cycles to spend in Deep Sleep Mode before waking-up the device. This ensures a maximum of 37 hours and 16mn sleep mode capabilities at 32kHz. This ensures a maximum of 36 hours and 16mn sleep mode capabilities at 32.768kHz. If DEEPSLTIME is set to zero, the Deep Sleep Time duration is considered as infinite, and only wake up requests can restore active behaviour\nBLE Software must ensure DEEPSLTIME value to be greater than 2 in order to cope with control resynchronization requirements"]
    #[inline(always)]
    pub fn deepsltime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        BleDeepslwkupReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleDeepslwkupReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleDeepslwkupReg {
    #[inline(always)]
    fn default() -> BleDeepslwkupReg {
        <crate::RegValueT<BleDeepslwkupReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDiagcntlReg_SPEC;
impl crate::sealed::RegSpec for BleDiagcntlReg_SPEC {
    type DataType = u32;
}
#[doc = "Diagnostics Register"]
pub type BleDiagcntlReg = crate::RegValueT<BleDiagcntlReg_SPEC>;

impl BleDiagcntlReg {
    #[doc = "0: Disable diagnostic port 3 output. All outputs are set to 0.\n1: Enable diagnostic port 3 output."]
    #[inline(always)]
    pub fn diag3_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Only relevant when DIAG3_EN = 1.\nSelection of the outputs that must be driven to the diagnostic port 3."]
    #[inline(always)]
    pub fn diag3(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3f,1,0,u8, BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable diagnostic port 2 output. All outputs are set to 0.\n1: Enable diagnostic port 2 output."]
    #[inline(always)]
    pub fn diag2_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Only relevant when DIAG2_EN = 1.\nSelection of the outputs that must be driven to the diagnostic port 2."]
    #[inline(always)]
    pub fn diag2(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8, BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable diagnostic port 1 output. All outputs are set to 0.\n1: Enable diagnostic port 1 output."]
    #[inline(always)]
    pub fn diag1_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Only relevant when DIAG1_EN = 1.\nSelection of the outputs that must be driven to the diagnostic port 1."]
    #[inline(always)]
    pub fn diag1(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8, BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable diagnostic port 0 output. All outputs are set to 0.\n1: Enable diagnostic port 0 output."]
    #[inline(always)]
    pub fn diag0_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Only relevant when DIAG0_EN = 1.\nSelection of the outputs that must be driven to the diagnostic port 0."]
    #[inline(always)]
    pub fn diag0(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleDiagcntlReg {
    #[inline(always)]
    fn default() -> BleDiagcntlReg {
        <crate::RegValueT<BleDiagcntlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDiagstatReg_SPEC;
impl crate::sealed::RegSpec for BleDiagstatReg_SPEC {
    type DataType = u32;
}
#[doc = "Debug use only"]
pub type BleDiagstatReg = crate::RegValueT<BleDiagstatReg_SPEC>;

impl BleDiagstatReg {
    #[doc = "Directly connected to ble_dbg3\\[7:0\\] output. Debug use only."]
    #[inline(always)]
    pub fn diag3stat(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8, BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Directly connected to ble_dbg2\\[7:0\\] output. Debug use only."]
    #[inline(always)]
    pub fn diag2stat(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Directly connected to ble_dbg1\\[7:0\\] output. Debug use only."]
    #[inline(always)]
    pub fn diag1stat(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Directly connected to ble_dbg0\\[7:0\\] output. Debug use only."]
    #[inline(always)]
    pub fn diag0stat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleDiagstatReg {
    #[inline(always)]
    fn default() -> BleDiagstatReg {
        <crate::RegValueT<BleDiagstatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleEnbpresetReg_SPEC;
impl crate::sealed::RegSpec for BleEnbpresetReg_SPEC {
    type DataType = u32;
}
#[doc = "Time in low power oscillator cycles register"]
pub type BleEnbpresetReg = crate::RegValueT<BleEnbpresetReg_SPEC>;

impl BleEnbpresetReg {
    #[doc = "Minimum and recommended value is \'TWIRQ_RESET + 1\'.\nIn the case of wake-up due to an external wake-up request, TWEXT specifies the time delay in low power oscillator cycles to deassert BLE_WAKEUP_LP_IRQ.\nRefer also to GP_CONTROL_REG\\[BLE_WAKEUP_REQ\\].\nRange is \\[0...64 ms\\] for 32kHz; \\[0...62.5 ms\\] for 32.768kHz"]
    #[inline(always)]
    pub fn twext(
        self,
    ) -> crate::common::RegisterField<21, 0x7ff, 1, 0, u16, BleEnbpresetReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x7ff,1,0,u16, BleEnbpresetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum value is \'TWIRQ_RESET + 1\'.\nTime in low power oscillator cycles to set BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration.\nRefer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\].\nRange is \\[0...64 ms\\] for 32kHz; \\[0...62.5 ms\\] for 32.768kHz"]
    #[inline(always)]
    pub fn twirq_set(
        self,
    ) -> crate::common::RegisterField<10, 0x7ff, 1, 0, u16, BleEnbpresetReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x7ff,1,0,u16, BleEnbpresetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Recommended value is 1.\nTime in low power oscillator cycles to reset BLE_WAKEUP_LP_IRQ before the BLE sleep timer expiration.\nRefer also to BLE_DEEPSLWKUP_REG\\[DEEPSLTIME\\].\nRange is \\[0...32 ms\\] for 32kHz; \\[0...31.25 ms\\] for 32.768kHz."]
    #[inline(always)]
    pub fn twirq_reset(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, BleEnbpresetReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, BleEnbpresetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleEnbpresetReg {
    #[inline(always)]
    fn default() -> BleEnbpresetReg {
        <crate::RegValueT<BleEnbpresetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleErrortypestatReg_SPEC;
impl crate::sealed::RegSpec for BleErrortypestatReg_SPEC {
    type DataType = u32;
}
#[doc = "Error Type Status registers"]
pub type BleErrortypestatReg = crate::RegValueT<BleErrortypestatReg_SPEC>;

impl BleErrortypestatReg {
    #[doc = "Indicates whether CS-FORMAT has been programmed with an invalid value: this is a major software programming failure.\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn csformat_error(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates whether CS-TXPTR is null, this is a major software programming failure.\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn cstxptr_error(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Radio Controller Exchange Memory access error, happens when Exchange Memory access are not served in time and data are corrupted.\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn radio_emacc_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Link Layer Channel Map error, happens when actual number of CS-LLCHMAP bit set to one is different from CS-NBCHGOOD at the beginning of Frequency Hopping process\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn llchmap_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Inter Frame Space Under run, occurs if IFS time is not enough to update and read Control Structure/Descriptors, and/or White List parsing is not finished and/or Decryption time is too long to be finished on time\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn ifs_underrun(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Real Time Decryption Error, happens when decryption is not finished before IFS time\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn rxcrypt_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "White List Timeout Error, occurs if White List parsing is not finished on time\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn whitelist_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Anticipated Pre-Fetch Mechanism error, happens when 3 consecutive Exchange Table entry have been programmed,\n0: no error\n1: Error occured"]
    #[inline(always)]
    pub fn apfm_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Tx Descriptor Error, happens when fetched Tx Descriptor has TXDONE bit not set\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn txdesc_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Packet Controller Exchange Memory access error, happens when Exchange Memory access are not served in time and Tx/Rx data are corrupted\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn pktcntl_emacc_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Real Time Encryption Error, happens when encryption is not finished before Tx Payload has to be sent\n0: No error\n1: Error occurred"]
    #[inline(always)]
    pub fn txcrypt_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleErrortypestatReg {
    #[inline(always)]
    fn default() -> BleErrortypestatReg {
        <crate::RegValueT<BleErrortypestatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleFinecntcorrReg_SPEC;
impl crate::sealed::RegSpec for BleFinecntcorrReg_SPEC {
    type DataType = u32;
}
#[doc = "Phase correction value register"]
pub type BleFinecntcorrReg = crate::RegValueT<BleFinecntcorrReg_SPEC>;

impl BleFinecntcorrReg {
    #[doc = "Phase correction value for the 625usec reference counter (i.e Fine Counter) in us."]
    #[inline(always)]
    pub fn finecntcorr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, BleFinecntcorrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, BleFinecntcorrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleFinecntcorrReg {
    #[inline(always)]
    fn default() -> BleFinecntcorrReg {
        <crate::RegValueT<BleFinecntcorrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleFinetimecntReg_SPEC;
impl crate::sealed::RegSpec for BleFinetimecntReg_SPEC {
    type DataType = u32;
}
#[doc = "Fine time reference counter"]
pub type BleFinetimecntReg = crate::RegValueT<BleFinetimecntReg_SPEC>;

impl BleFinetimecntReg {
    #[doc = "Value of the current usec fine time reference counter. Updated each usec. Used by the SW in order to synchronize with the HW, and obtain a more precise sleep duration"]
    #[inline(always)]
    pub fn finecnt(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, BleFinetimecntReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, BleFinetimecntReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleFinetimecntReg {
    #[inline(always)]
    fn default() -> BleFinetimecntReg {
        <crate::RegValueT<BleFinetimecntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleFinetimtgtReg_SPEC;
impl crate::sealed::RegSpec for BleFinetimtgtReg_SPEC {
    type DataType = u32;
}
#[doc = "Fine Timer Target value"]
pub type BleFinetimtgtReg = crate::RegValueT<BleFinetimtgtReg_SPEC>;

impl BleFinetimtgtReg {
    #[doc = "Fine Timer Target value on which a BLE_FINETGTIM_IRQ must be generated. This timer has a precision of 625us: interrupt is generated only when FINETARGET = BASETIMECNT"]
    #[inline(always)]
    pub fn finetarget(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffffff,
        1,
        0,
        u32,
        BleFinetimtgtReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ffffff,
            1,
            0,
            u32,
            BleFinetimtgtReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleFinetimtgtReg {
    #[inline(always)]
    fn default() -> BleFinetimtgtReg {
        <crate::RegValueT<BleFinetimtgtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleGrosstimtgtReg_SPEC;
impl crate::sealed::RegSpec for BleGrosstimtgtReg_SPEC {
    type DataType = u32;
}
#[doc = "Gross Timer Target value"]
pub type BleGrosstimtgtReg = crate::RegValueT<BleGrosstimtgtReg_SPEC>;

impl BleGrosstimtgtReg {
    #[doc = "Gross Timer Target value on which a BLE_GROSSTGTIM_IRQ must be generated. This timer has a precision of 10ms: interrupt is generated only when GROSSTARGET\\[15:0\\] = BASETIMECNT\\[19:4\\] and BASETIMECNT\\[3:0\\] = 0."]
    #[inline(always)]
    pub fn grosstarget(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BleGrosstimtgtReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            BleGrosstimtgtReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleGrosstimtgtReg {
    #[inline(always)]
    fn default() -> BleGrosstimtgtReg {
        <crate::RegValueT<BleGrosstimtgtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleIntackReg_SPEC;
impl crate::sealed::RegSpec for BleIntackReg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt acknowledge register"]
pub type BleIntackReg = crate::RegValueT<BleIntackReg_SPEC>;

impl BleIntackReg {
    #[doc = "Radio Controller interrupt acknowledgement bit\nSoftware writing 1 acknowledges the Error interrupt. This bit resets RADIOCNTLINTSTAT and RADIOCNTLINTRAWSTAT flags."]
    #[inline(always)]
    pub fn radiocntlintack(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Fine Target Timer interrupt acknowledgement bit\nSoftware writing 1 acknowledges the Error interrupt. This bit resets FINETGTIMINTSTAT and FINETGTIMINTRAWSTAT flags."]
    #[inline(always)]
    pub fn finetgtimintack(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Gross Target Timer interrupt acknowledgement bit\nSoftware writing 1 acknowledges the Error interrupt. This bit resets GROSSTGTIMINTSTAT and GROSSTGTIMINTRAWSTAT flags."]
    #[inline(always)]
    pub fn grosstgtimintack(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Error interrupt acknowledgement bit\nSoftware writing 1 acknowledges the Error interrupt. This bit resets ERRORINTSTAT and ERRORINTRAWSTAT flags."]
    #[inline(always)]
    pub fn errorintack(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Encryption/Decryption interrupt acknowledgement bit Software writing 1 acknowledges the Encryption / Decryption interrupt. This bit resets CRYPTINTSTAT and CRYPTINTRAWSTAT flags."]
    #[inline(always)]
    pub fn cryptintack(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "End of Event interrupt acknowledgment bit\nSoftware writing 1 acknowledges the End of Advertising / Scanning / Connection interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags."]
    #[inline(always)]
    pub fn eventintack(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "End of Deep Sleep interrupt acknowledgment bit\nSoftware writing 1 acknowledges the End of Sleep Mode interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags."]
    #[inline(always)]
    pub fn slpintack(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Packet Reception interrupt acknowledgment bit\nSoftware writing 1 acknowledges the Rx interrupt. This bit resets RXINTSTAT and RXINTRAWSTAT flags."]
    #[inline(always)]
    pub fn rxintack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "625us base time reference interrupt acknowledgment bit\nSoftware writing 1 acknowledges the CLKN interrupt. This bit resets CLKINTSTAT and CLKINTRAWSTAT flags."]
    #[inline(always)]
    pub fn cscntintack(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for BleIntackReg {
    #[inline(always)]
    fn default() -> BleIntackReg {
        <crate::RegValueT<BleIntackReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleIntcntlReg_SPEC;
impl crate::sealed::RegSpec for BleIntcntlReg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt controller register"]
pub type BleIntcntlReg = crate::RegValueT<BleIntcntlReg_SPEC>;

impl BleIntcntlReg {
    #[doc = "Selection of the CS counter that generates an interrupt. For example, if INTCNTL\\[3\\] is set, an interrupt is sent each time CS counter equals 3."]
    #[inline(always)]
    pub fn intcscntl(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, BleIntcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CSCNT interrupt mask during event. This bit allows to enable CSCNT interrupt generation during events (i.e advertising, scanning, initiating, and connection)\n0: CSCNT Interrupt not generated during events.\n1: CSCNT Interrupt generated during events."]
    #[inline(always)]
    pub fn cscntdevmsk(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Radio Controller interrupt mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn radiocntlintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fine Target Timer Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn finetgtimintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Gross Target Timer Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn grosstgtimintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Interrupt Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn errorintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Encryption / Decryption Interrupt Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn cryptintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "End of event Interrupt Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn eventintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sleep Mode Interrupt Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn slpintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rx Interrupt Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn rxintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "625usec Base Time Interrupt Mask\n0: Interrupt not generated\n1: Interrupt generated"]
    #[inline(always)]
    pub fn cscntintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleIntcntlReg {
    #[inline(always)]
    fn default() -> BleIntcntlReg {
        <crate::RegValueT<BleIntcntlReg_SPEC> as RegisterValue<_>>::new(16744479)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleIntrawstatReg_SPEC;
impl crate::sealed::RegSpec for BleIntrawstatReg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt raw status register"]
pub type BleIntrawstatReg = crate::RegValueT<BleIntrawstatReg_SPEC>;

impl BleIntrawstatReg {
    #[doc = "Radio Controller interrupt raw status\n0: No Gross Target Timer interrupt.\n1: A Gross Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn radiocntlintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Fine Target Timer Error interrupt raw status\n0: No Fine Target Timer interrupt.\n1: A Fine Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn finetgtimintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Gross Target Timer interrupt raw status\n0: No Gross Target Timer interrupt.\n1: A Gross Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn grosstgtimintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Error interrupt raw status\n0: No Error interrupt.\n1: An Error interrupt is pending."]
    #[inline(always)]
    pub fn errorintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Encryption/Decryption interrupt raw status\n0: No Encryption / Decryption interrupt.\n1: An Encryption / Decryption interrupt is pending."]
    #[inline(always)]
    pub fn cryptintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "End of Event interrupt raw status\n0: No End of Advertising / Scanning / Connection interrupt.\n1: An End of Advertising / Scanning / Connection interrupt is pending."]
    #[inline(always)]
    pub fn eventintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sleep interrupt raw status\n0: No End of Sleep Mode interrupt.\n1: An End of Sleep Mode interrupt is pending."]
    #[inline(always)]
    pub fn slpintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Packet Reception interrupt raw status\n0: No Rx interrupt.\n1: An Rx interrupt is pending."]
    #[inline(always)]
    pub fn rxintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "625us base time reference interrupt raw status\n0: No 625us Base Time interrupt.\n1: A 625us Base Time interrupt is pending."]
    #[inline(always)]
    pub fn cscntintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleIntrawstatReg {
    #[inline(always)]
    fn default() -> BleIntrawstatReg {
        <crate::RegValueT<BleIntrawstatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleIntstatReg_SPEC;
impl crate::sealed::RegSpec for BleIntstatReg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt status register"]
pub type BleIntstatReg = crate::RegValueT<BleIntstatReg_SPEC>;

impl BleIntstatReg {
    #[doc = "Radio Controller interrupt status\n0: No Gross Target Timer interrupt.\n1: A Gross Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn radiocntlintstat(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked Fine Target Timer Error interrupt status\n0: No Fine Target Timer interrupt.\n1: A Fine Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn finetgtimintstat(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked Gross Target Timer interrupt status\n0: No Gross Target Timer interrupt.\n1: A Gross Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn grosstgtimintstat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked Error interrupt status\n0: No Error interrupt.\n1: An Error interrupt is pending."]
    #[inline(always)]
    pub fn errorintstat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked Encryption/Decryption interrupt status\n0: No Encryption / Decryption interrupt.\n1: An Encryption / Decryption interrupt is pending."]
    #[inline(always)]
    pub fn cryptintstat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked End of Event interrupt status\n0: No End of Advertising / Scanning / Connection interrupt.\n1: An End of Advertising / Scanning / Connection interrupt is pending."]
    #[inline(always)]
    pub fn eventintstat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked Sleep interrupt status\n0: No End of Sleep Mode interrupt.\n1: An End of Sleep Mode interrupt is pending."]
    #[inline(always)]
    pub fn slpintstat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked Packet Reception interrupt status\n0: No Rx interrupt.\n1: An Rx interrupt is pending."]
    #[inline(always)]
    pub fn rxintstat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Masked 625us base time reference interrupt status\n0: No 625us Base Time interrupt.\n1: A 625us Base Time interrupt is pending."]
    #[inline(always)]
    pub fn cscntintstat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleIntstatReg {
    #[inline(always)]
    fn default() -> BleIntstatReg {
        <crate::RegValueT<BleIntstatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRadiocntl0Reg_SPEC;
impl crate::sealed::RegSpec for BleRadiocntl0Reg_SPEC {
    type DataType = u32;
}
#[doc = "Radio interface control register"]
pub type BleRadiocntl0Reg = crate::RegValueT<BleRadiocntl0Reg_SPEC>;

impl BleRadiocntl0Reg {
    #[doc = "Enable the use of delayed DC compensated data path in Radio Correlator block\n1: Enable\n0: Disable\nMust be set to \'0\'."]
    #[inline(always)]
    pub fn dpcorr_en(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, BleRadiocntl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,BleRadiocntl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRadiocntl0Reg {
    #[inline(always)]
    fn default() -> BleRadiocntl0Reg {
        <crate::RegValueT<BleRadiocntl0Reg_SPEC> as RegisterValue<_>>::new(8450)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRadiocntl1Reg_SPEC;
impl crate::sealed::RegSpec for BleRadiocntl1Reg_SPEC {
    type DataType = u32;
}
#[doc = "Radio interface control register"]
pub type BleRadiocntl1Reg = crate::RegValueT<BleRadiocntl1Reg_SPEC>;

impl BleRadiocntl1Reg {
    #[doc = "Extended radio selection field, Must be set to \'00011\'."]
    #[inline(always)]
    pub fn xrfsel(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, BleRadiocntl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8, BleRadiocntl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRadiocntl1Reg {
    #[inline(always)]
    fn default() -> BleRadiocntl1Reg {
        <crate::RegValueT<BleRadiocntl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRadiopwrupdnReg_SPEC;
impl crate::sealed::RegSpec for BleRadiopwrupdnReg_SPEC {
    type DataType = u32;
}
#[doc = "RX/TX power up/down phase register"]
pub type BleRadiopwrupdnReg = crate::RegValueT<BleRadiopwrupdnReg_SPEC>;

impl BleRadiopwrupdnReg {
    #[doc = "Defines round trip delay value. This value correspond to the addition of data latency in Tx and data latency in Rx. Value is in us."]
    #[inline(always)]
    pub fn rtrip_delay(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, BleRadiopwrupdnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7f,1,0,u8, BleRadiopwrupdnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This register holds the length in us of the Rx power up phase for the current radio device. Default value is 210 us (reset value). Operating range depends of the selected radio."]
    #[inline(always)]
    pub fn rxpwrup(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, BleRadiopwrupdnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, BleRadiopwrupdnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This register extends the length in us of the Tx power down phase for the current radio device. Default value is 3us (reset value). Operating range depends of the selected radio."]
    #[inline(always)]
    pub fn txpwrdn(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, BleRadiopwrupdnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, BleRadiopwrupdnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This register holds the length in us of the Tx power up phase for the current radio device. Default value is 210 us (reset value). Operating range depends of the selected radio."]
    #[inline(always)]
    pub fn txpwrup(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, BleRadiopwrupdnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, BleRadiopwrupdnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRadiopwrupdnReg {
    #[inline(always)]
    fn default() -> BleRadiopwrupdnReg {
        <crate::RegValueT<BleRadiopwrupdnReg_SPEC> as RegisterValue<_>>::new(13763538)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRftestcntlReg_SPEC;
impl crate::sealed::RegSpec for BleRftestcntlReg_SPEC {
    type DataType = u32;
}
#[doc = "RF Testing Register"]
pub type BleRftestcntlReg = crate::RegValueT<BleRftestcntlReg_SPEC>;

impl BleRftestcntlReg {
    #[doc = "Applicable for all frame format\n0: Normal mode of operation\n1: Infinite Rx window"]
    #[inline(always)]
    pub fn infiniterx(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Applicable for all frame format\n0: Normal mode of operation.\n1: Infinite Tx packet / Normal start of a packet but endless payload\nIn case of infinite Tx payload, and when PRBS source is not selected, then RFTESTCNTL-TXLENGTH field provides the length of the pattern to repeat in the payload."]
    #[inline(always)]
    pub fn infinitetx(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Applicable only in Tx/Rx RF Test mode\n0: Normal mode of operation: TXDESC-TXADVLEN controls the Tx packet payload size\n1: Uses RFTESTCNTL-TXLENGTH packet length (can support up to 512 bytes transmit)"]
    #[inline(always)]
    pub fn txlengthsrc(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Applicable only in Tx/Rx RF Test mode\n0: Tx Packet Payload are PRBS9 type\n1: Tx Packet Payload are PRBS15 type\nPRBS9 is defined as p(x)=1+x5+x9. The LFSR used for the PRBS9 generator must be initialized with 0x1FF value.\nPRBS15 is defined as p(x)=1+x+x2+x12+x13+x14. The LFSR used for the PRBS15 generator must be initialized with 0x7FFF value."]
    #[inline(always)]
    pub fn prbstype(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Applicable only in Tx/Rx RF Test mode\n0: Tx Packet Payload source is the Control Structure\n1: Tx Packet Payload are PRBS generator"]
    #[inline(always)]
    pub fn txpldsrc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Applicable only in Tx/Rx RF Test mode\nTx packet length in number of byte"]
    #[inline(always)]
    pub fn txlength(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, BleRftestcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRftestcntlReg {
    #[inline(always)]
    fn default() -> BleRftestcntlReg {
        <crate::RegValueT<BleRftestcntlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRfDiagirqReg_SPEC;
impl crate::sealed::RegSpec for BleRfDiagirqReg_SPEC {
    type DataType = u32;
}
#[doc = "BLE/RF Diagnostic IRQ Control Register"]
pub type BleRfDiagirqReg = crate::RegValueT<BleRfDiagirqReg_SPEC>;

impl BleRfDiagirqReg {
    #[doc = "Same as DIAGIRQ_STAT_0."]
    #[inline(always)]
    pub fn diagirq_stat_3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleRfDiagirqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,BleRfDiagirqReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_EDGE_0."]
    #[inline(always)]
    pub fn diagirq_edge_3(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_BSEL_0."]
    #[inline(always)]
    pub fn diagirq_bsel_3(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x7,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_WSEL_0."]
    #[inline(always)]
    pub fn diagirq_wsel_3(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x3,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_MASK_0."]
    #[inline(always)]
    pub fn diagirq_mask_3(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_STAT_0."]
    #[inline(always)]
    pub fn diagirq_stat_2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, BleRfDiagirqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,BleRfDiagirqReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_EDGE_0."]
    #[inline(always)]
    pub fn diagirq_edge_2(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_BSEL_0."]
    #[inline(always)]
    pub fn diagirq_bsel_2(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x7,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_WSEL_0."]
    #[inline(always)]
    pub fn diagirq_wsel_2(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_MASK_0."]
    #[inline(always)]
    pub fn diagirq_mask_2(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_STAT_0."]
    #[inline(always)]
    pub fn diagirq_stat_1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleRfDiagirqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,BleRfDiagirqReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_EDGE_0."]
    #[inline(always)]
    pub fn diagirq_edge_1(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_BSEL_0."]
    #[inline(always)]
    pub fn diagirq_bsel_1(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_WSEL_0."]
    #[inline(always)]
    pub fn diagirq_wsel_1(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x3,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Same as DIAGIRQ_MASK_0."]
    #[inline(always)]
    pub fn diagirq_mask_1(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Diagnostic IRQ Status 0\nThis bit is read only. It is automatically cleared to \'0\' on each read of the BLE_RF_DIAGIRQ_REG register. It is automatically asserted to \'1\' on each detection of the selected edge, of the selected bit, of the selected word."]
    #[inline(always)]
    pub fn diagirq_stat_0(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleRfDiagirqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleRfDiagirqReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnostic IRQ Edge 0\nSelects the edge of the selected bit (refer to DIAGIRQ_BSEL_0) that will trigger the assertion of DIAGIRQ_STAT_0.\nIf \'0\' then the positive edge is selected, when \'1\' the negative edge is selected."]
    #[inline(always)]
    pub fn diagirq_edge_0(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Diagnostic IRQ Bit Select 0\nSelects the bit of the 8-bit bus (as selected by the DIAGIRQ_WSEL_0) that will be used for the IRQ generation."]
    #[inline(always)]
    pub fn diagirq_bsel_0(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Diagnostic IRQ Word Select 0\nSelects the 8-bit diagnostic bus that will be used for the IRQ generation.\n00: Selects the BLE_DIAG0\n01: Selects the BLE_DIAG1\n10: Selects the RADIO_DIAG0\n11: Selects the RADIO_DIAG1"]
    #[inline(always)]
    pub fn diagirq_wsel_0(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, BleRfDiagirqReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8, BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Diagnostic IRQ Mask 0\nWhen set to \'1\' a BLE_RF_DIAG_IRQ will be generated on each rise of the DIAGIRQ_STAT_0 bit.\nWhen cleared to \'0\' no IRQ will be generated."]
    #[inline(always)]
    pub fn diagirq_mask_0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleRfDiagirqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BleRfDiagirqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRfDiagirqReg {
    #[inline(always)]
    fn default() -> BleRfDiagirqReg {
        <crate::RegValueT<BleRfDiagirqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRwbtlecntlReg_SPEC;
impl crate::sealed::RegSpec for BleRwbtlecntlReg_SPEC {
    type DataType = u32;
}
#[doc = "BLE Control register"]
pub type BleRwbtlecntlReg = crate::RegValueT<BleRwbtlecntlReg_SPEC>;

impl BleRwbtlecntlReg {
    #[doc = "Reset the complete system except registers and timing generator, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn master_soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reset the timing generator, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn master_tgsoft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reset the complete register block, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn reg_soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Abort the current RF Testing defined as per CS-FORMAT when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0.\nNote that when RFTEST_ABORT is requested.\n1) In case of infinite Tx, the Packet Controller FSM stops at the end of the current byte in process, and processes accordingly the packet CRC.\n2) In case of Infinite Rx, the Packet Controller FSM either stops as the end of the current Packet reception (if Access address has been detected), or simply stop the processing switching off the RF."]
    #[inline(always)]
    pub fn rftest_abort(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Abort the current Advertising event when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn advert_abort(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Abort the current scan window when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn scan_abort(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation of MD bits management\n1: Allow a single Tx/Rx exchange whatever the MD bits are"]
    #[inline(always)]
    pub fn md_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation of Sequence number\n1: Sequence Number Management disabled:\nvalue forced by SW from Tx Descriptor\nvalue ignored in Rx, meaning that no SN error reported."]
    #[inline(always)]
    pub fn sn_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation of Sequence number\n1: Sequence Number Management disabled:\nvalue forced by SW from Tx Descriptor\nvalue ignored in Rx, meaning that no SN error reported."]
    #[inline(always)]
    pub fn nesn_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation. Encryption / Decryption enabled.\n1: Encryption / Decryption disabled.\nNote that if CS-CRYPT_EN is set, then MIC is generated, and only data encryption is disabled, meaning data sent are plain data."]
    #[inline(always)]
    pub fn crypt_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation. Whitening enabled.\n1: Whitening disabled."]
    #[inline(always)]
    pub fn whit_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation. CRC removed from data stream.\n1: CRC stripping disabled on Rx packets, CRC replaced by 0x000 in Tx"]
    #[inline(always)]
    pub fn crc_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Normal operation. Frequency Hopping Remapping algorithm enabled.\n1: Frequency Hopping Remapping algorithm disabled"]
    #[inline(always)]
    pub fn hop_remap_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Applicable only if device is in Initiator mode\n0: Window Offset field in CONNECT_REQ comes from Tx Data Buffer\n1: Window Offset field in CONNECT_REQ comes from Event Controller processing and is replaced in real time by Packet Controller"]
    #[inline(always)]
    pub fn txwinoffsel(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Selects Rx Descriptor Pointer value from Control Structure\n1: Selects Rx Descriptor Pointer value from CURRENTRXDESCPTR register"]
    #[inline(always)]
    pub fn rxdescptrsel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Advertising Channels Error Filtering Enable control\n0: BLE Core reports all errors to BLE Software\n1: BLE Core reports only correctly received packet, without error to BLE Software"]
    #[inline(always)]
    pub fn adverrfilt_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: WLAN synchronization pulse generation disabled\n1: WLAN synchronization pulse generation enabled"]
    #[inline(always)]
    pub fn wlsync_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: Disable BLE Core Exchange Table pre-fetch mechanism.\n1: Enable BLE Core Exchange table pre-fetch mechanism."]
    #[inline(always)]
    pub fn rwble_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleRwbtlecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Default Rx Window size in us. Used when device\na) is master connected\nb) performs its second receipt.\n0 is not a valid value. Recommended value is 10."]
    #[inline(always)]
    pub fn rxwinszdef(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, BleRwbtlecntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8, BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates the maximum number of errors allowed to recognize the synchronization word."]
    #[inline(always)]
    pub fn syncerr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, BleRwbtlecntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8, BleRwbtlecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRwbtlecntlReg {
    #[inline(always)]
    fn default() -> BleRwbtlecntlReg {
        <crate::RegValueT<BleRwbtlecntlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRwbtleconfReg_SPEC;
impl crate::sealed::RegSpec for BleRwbtleconfReg_SPEC {
    type DataType = u32;
}
#[doc = "Configuration register"]
pub type BleRwbtleconfReg = crate::RegValueT<BleRwbtleconfReg_SPEC>;

impl BleRwbtleconfReg {
    #[doc = "Value of the BLE_ADDRESS_WIDTH parameter converted into binary."]
    #[inline(always)]
    pub fn add_width(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, BleRwbtleconfReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x3f,1,0,u8, BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Supported radio interfaces.\n0001000: on-chip radio\nothers: reserved"]
    #[inline(always)]
    pub fn rfif(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, BleRwbtleconfReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8, BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Operating Frequency (in MHz).\nThis field is a copy of the BLE_CNTL2_REG\\[BLE_CLK_SEL\\] value."]
    #[inline(always)]
    pub fn clk_sel(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, BleRwbtleconfReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8, BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: BLE Core is used as a standalone BLE device\n1: BLE Core is used in a Dual Mode device"]
    #[inline(always)]
    pub fn dmmode(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleRwbtleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: Interrupts are edge level generated, i.e pulse.\n1: Interrupts are trigger level generated, i.e stays active at 1 till acknowledgement"]
    #[inline(always)]
    pub fn intmode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleRwbtleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: WLAN Coexistence mechanism not present\n1: WLAN Coexistence mechanism present"]
    #[inline(always)]
    pub fn wlan(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleRwbtleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: Diagnostic port not instantiated\n1: Diagnostic port instantiated"]
    #[inline(always)]
    pub fn usedbg(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleRwbtleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0: Encryption block not present\n1: Encryption block present"]
    #[inline(always)]
    pub fn usecrypt(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleRwbtleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Processor bus width:\n0: 16 bits\n1: 32 bits"]
    #[inline(always)]
    pub fn buswidth(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleRwbtleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BleRwbtleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRwbtleconfReg {
    #[inline(always)]
    fn default() -> BleRwbtleconfReg {
        <crate::RegValueT<BleRwbtleconfReg_SPEC> as RegisterValue<_>>::new(235405335)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRxmicvalReg_SPEC;
impl crate::sealed::RegSpec for BleRxmicvalReg_SPEC {
    type DataType = u32;
}
#[doc = "AES / CCM plain MIC value"]
pub type BleRxmicvalReg = crate::RegValueT<BleRxmicvalReg_SPEC>;

impl BleRxmicvalReg {
    #[doc = "AES / CCM plain MIC value. Valid on BLE_CRYPT_IRQ interrupt (even masked)"]
    #[inline(always)]
    pub fn rxmicval(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, BleRxmicvalReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleRxmicvalReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleRxmicvalReg {
    #[inline(always)]
    fn default() -> BleRxmicvalReg {
        <crate::RegValueT<BleRxmicvalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleSampleclkReg_SPEC;
impl crate::sealed::RegSpec for BleSampleclkReg_SPEC {
    type DataType = u32;
}
#[doc = "Samples the Base Time Counter"]
pub type BleSampleclkReg = crate::RegValueT<BleSampleclkReg_SPEC>;

impl BleSampleclkReg {
    #[doc = "Writing a 1 samples the Base Time Counter value in BASETIMECNT register. Resets at 0 when action is performed."]
    #[inline(always)]
    pub fn samp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleSampleclkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BleSampleclkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleSampleclkReg {
    #[inline(always)]
    fn default() -> BleSampleclkReg {
        <crate::RegValueT<BleSampleclkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleSwprofilingReg_SPEC;
impl crate::sealed::RegSpec for BleSwprofilingReg_SPEC {
    type DataType = u32;
}
#[doc = "Software Profiling register"]
pub type BleSwprofilingReg = crate::RegValueT<BleSwprofilingReg_SPEC>;

impl BleSwprofilingReg {
    #[doc = "Software Profiling register: used by BLE Software for profiling purpose: this value is copied on Diagnostic port"]
    #[inline(always)]
    pub fn swprofval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        BleSwprofilingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleSwprofilingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleSwprofilingReg {
    #[inline(always)]
    fn default() -> BleSwprofilingReg {
        <crate::RegValueT<BleSwprofilingReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleTimgencntlReg_SPEC;
impl crate::sealed::RegSpec for BleTimgencntlReg_SPEC {
    type DataType = u32;
}
#[doc = "Timing Generator Register"]
pub type BleTimgencntlReg = crate::RegValueT<BleTimgencntlReg_SPEC>;

impl BleTimgencntlReg {
    #[doc = "Defines Exchange Table pre-fetch instant in us"]
    #[inline(always)]
    pub fn preftech_time(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, BleTimgencntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, BleTimgencntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleTimgencntlReg {
    #[inline(always)]
    fn default() -> BleTimgencntlReg {
        <crate::RegValueT<BleTimgencntlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleTxmicvalReg_SPEC;
impl crate::sealed::RegSpec for BleTxmicvalReg_SPEC {
    type DataType = u32;
}
#[doc = "AES / CCM plain MIC value"]
pub type BleTxmicvalReg = crate::RegValueT<BleTxmicvalReg_SPEC>;

impl BleTxmicvalReg {
    #[doc = "AES / CCM plain MIC value. Valid on BLE_CRYPT_IRQ interrupt (even masked)"]
    #[inline(always)]
    pub fn txmicval(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, BleTxmicvalReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            BleTxmicvalReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleTxmicvalReg {
    #[inline(always)]
    fn default() -> BleTxmicvalReg {
        <crate::RegValueT<BleTxmicvalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleVersionReg_SPEC;
impl crate::sealed::RegSpec for BleVersionReg_SPEC {
    type DataType = u32;
}
#[doc = "Version register"]
pub type BleVersionReg = crate::RegValueT<BleVersionReg_SPEC>;

impl BleVersionReg {
    #[doc = "BLE Core Type - 0x6 means BT4.0 (i.e correspond LL version assigned number)"]
    #[inline(always)]
    pub fn typ(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, BleVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8, BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "BLE Core version - Major release number.(Correspond to FS v1.11)"]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, BleVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "BLE Core upgrade - Upgrade number. (Correspond to FS v1.11)"]
    #[inline(always)]
    pub fn upg(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, BleVersionReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "BLE Core Build - Build number"]
    #[inline(always)]
    pub fn build(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, BleVersionReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleVersionReg {
    #[inline(always)]
    fn default() -> BleVersionReg {
        <crate::RegValueT<BleVersionReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleWlnbdevReg_SPEC;
impl crate::sealed::RegSpec for BleWlnbdevReg_SPEC {
    type DataType = u32;
}
#[doc = "Devices in white list"]
pub type BleWlnbdevReg = crate::RegValueT<BleWlnbdevReg_SPEC>;

impl BleWlnbdevReg {
    #[doc = "Number of private devices in the white list."]
    #[inline(always)]
    pub fn nbprivdev(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, BleWlnbdevReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, BleWlnbdevReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of public devices in the white list."]
    #[inline(always)]
    pub fn nbpubdev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, BleWlnbdevReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, BleWlnbdevReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleWlnbdevReg {
    #[inline(always)]
    fn default() -> BleWlnbdevReg {
        <crate::RegValueT<BleWlnbdevReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleWlprivaddptrReg_SPEC;
impl crate::sealed::RegSpec for BleWlprivaddptrReg_SPEC {
    type DataType = u32;
}
#[doc = "Start address of private devices list"]
pub type BleWlprivaddptrReg = crate::RegValueT<BleWlprivaddptrReg_SPEC>;

impl BleWlprivaddptrReg {
    #[doc = "Start address pointer of the private devices white list."]
    #[inline(always)]
    pub fn wlprivaddptr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        BleWlprivaddptrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            BleWlprivaddptrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleWlprivaddptrReg {
    #[inline(always)]
    fn default() -> BleWlprivaddptrReg {
        <crate::RegValueT<BleWlprivaddptrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleWlpubaddptrReg_SPEC;
impl crate::sealed::RegSpec for BleWlpubaddptrReg_SPEC {
    type DataType = u32;
}
#[doc = "Start address of public devices list"]
pub type BleWlpubaddptrReg = crate::RegValueT<BleWlpubaddptrReg_SPEC>;

impl BleWlpubaddptrReg {
    #[doc = "Start address pointer of the public devices white list."]
    #[inline(always)]
    pub fn wlpubaddptr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BleWlpubaddptrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            BleWlpubaddptrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleWlpubaddptrReg {
    #[inline(always)]
    fn default() -> BleWlpubaddptrReg {
        <crate::RegValueT<BleWlpubaddptrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
