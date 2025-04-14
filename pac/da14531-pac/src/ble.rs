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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:19 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"BLE registers"]
unsafe impl ::core::marker::Send for super::Ble {}
unsafe impl ::core::marker::Sync for super::Ble {}
impl super::Ble {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

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

    #[inline(always)]
    pub const fn ble_blemprio0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleBlemprio0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleBlemprio0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ble_blemprio1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleBlemprio1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleBlemprio1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn ble_coexifcntl0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleCoexifcntl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleCoexifcntl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ble_coexifcntl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleCoexifcntl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleCoexifcntl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn ble_diagcntl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDiagcntl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDiagcntl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(524usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ble_diagcntl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleDiagcntl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleDiagcntl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn ble_em_base_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleEmBaseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleEmBaseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn ble_radiocntl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRadiocntl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRadiocntl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ble_radiocntl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRadiocntl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRadiocntl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn ble_rftestrxstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRftestrxstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRftestrxstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ble_rftesttxstat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRftesttxstatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRftesttxstatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ble_rwblecntl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRwblecntlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRwblecntlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ble_rwbleconf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleRwbleconfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleRwbleconfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

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

pub type BleActscanstatReg = crate::RegValueT<BleActscanstatReg_SPEC>;

impl BleActscanstatReg {
    #[inline(always)]
    pub fn backoff(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BleActscanstatReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BleActscanstatReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn upperlimit(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BleActscanstatReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BleActscanstatReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleActscanstatReg {
    #[inline(always)]
    fn default() -> BleActscanstatReg {
        <crate::RegValueT<BleActscanstatReg_SPEC> as RegisterValue<_>>::new(65537)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleAdvchmapReg_SPEC;
impl crate::sealed::RegSpec for BleAdvchmapReg_SPEC {
    type DataType = u32;
}

pub type BleAdvchmapReg = crate::RegValueT<BleAdvchmapReg_SPEC>;

impl BleAdvchmapReg {
    #[inline(always)]
    pub fn advchmap(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, BleAdvchmapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,BleAdvchmapReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type BleAdvtimReg = crate::RegValueT<BleAdvtimReg_SPEC>;

impl BleAdvtimReg {
    #[inline(always)]
    pub fn advint(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, BleAdvtimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            BleAdvtimReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type BleAescntlReg = crate::RegValueT<BleAescntlReg_SPEC>;

impl BleAescntlReg {
    #[inline(always)]
    pub fn aes_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleAescntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BleAescntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aes_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleAescntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,BleAescntlReg_SPEC,crate::common::W>::from_register(self,0)
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

pub type BleAeskey12796Reg = crate::RegValueT<BleAeskey12796Reg_SPEC>;

impl BleAeskey12796Reg {
    #[inline(always)]
    pub fn aeskey127_96(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type BleAeskey310Reg = crate::RegValueT<BleAeskey310Reg_SPEC>;

impl BleAeskey310Reg {
    #[inline(always)]
    pub fn aeskey31_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type BleAeskey6332Reg = crate::RegValueT<BleAeskey6332Reg_SPEC>;

impl BleAeskey6332Reg {
    #[inline(always)]
    pub fn aeskey63_32(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type BleAeskey9564Reg = crate::RegValueT<BleAeskey9564Reg_SPEC>;

impl BleAeskey9564Reg {
    #[inline(always)]
    pub fn aeskey95_64(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type BleAesptrReg = crate::RegValueT<BleAesptrReg_SPEC>;

impl BleAesptrReg {
    #[inline(always)]
    pub fn aesptr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, BleAesptrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            BleAesptrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type BleBasetimecntcorrReg = crate::RegValueT<BleBasetimecntcorrReg_SPEC>;

impl BleBasetimecntcorrReg {
    #[inline(always)]
    pub fn basetimecntcorr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffffff,
        1,
        0,
        u32,
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

pub type BleBasetimecntReg = crate::RegValueT<BleBasetimecntReg_SPEC>;

impl BleBasetimecntReg {
    #[inline(always)]
    pub fn basetimecnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffffff,
        1,
        0,
        u32,
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

pub type BleBdaddrlReg = crate::RegValueT<BleBdaddrlReg_SPEC>;

impl BleBdaddrlReg {
    #[inline(always)]
    pub fn bdaddrl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        BleBdaddrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
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

pub type BleBdaddruReg = crate::RegValueT<BleBdaddruReg_SPEC>;

impl BleBdaddruReg {
    #[inline(always)]
    pub fn priv_npub(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BleBdaddruReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,BleBdaddruReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bdaddru(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        BleBdaddruReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            BleBdaddruReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
pub struct BleBlemprio0Reg_SPEC;
impl crate::sealed::RegSpec for BleBlemprio0Reg_SPEC {
    type DataType = u32;
}

pub type BleBlemprio0Reg = crate::RegValueT<BleBlemprio0Reg_SPEC>;

impl BleBlemprio0Reg {
    #[inline(always)]
    pub fn blem7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blem6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blem5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blem4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blem3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blem2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blem1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn blem0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, BleBlemprio0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,BleBlemprio0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleBlemprio0Reg {
    #[inline(always)]
    fn default() -> BleBlemprio0Reg {
        <crate::RegValueT<BleBlemprio0Reg_SPEC> as RegisterValue<_>>::new(881438191)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleBlemprio1Reg_SPEC;
impl crate::sealed::RegSpec for BleBlemprio1Reg_SPEC {
    type DataType = u32;
}

pub type BleBlemprio1Reg = crate::RegValueT<BleBlemprio1Reg_SPEC>;

impl BleBlemprio1Reg {
    #[inline(always)]
    pub fn blemdefault(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, BleBlemprio1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,BleBlemprio1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleBlemprio1Reg {
    #[inline(always)]
    fn default() -> BleBlemprio1Reg {
        <crate::RegValueT<BleBlemprio1Reg_SPEC> as RegisterValue<_>>::new(805306368)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleCntl2Reg_SPEC;
impl crate::sealed::RegSpec for BleCntl2Reg_SPEC {
    type DataType = u32;
}

pub type BleCntl2Reg = crate::RegValueT<BleCntl2Reg_SPEC>;

impl BleCntl2Reg {
    #[inline(always)]
    pub fn ble_phy_err_msk_n(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_arp_err_msk_n(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_arp_phy_err_stat(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_rssi_sel(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeuplpstat(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_rpl_spi(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bb_only(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_pti_source_sel(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_clk_sel(
        self,
    ) -> crate::common::RegisterField<9, 0x3f, 1, 0, u8, u8, BleCntl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x3f,1,0,u8,u8,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn radio_pwrdn_allow(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mon_lp_clk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_clk_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleCntl2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleCntl2Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_diag_ovr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn emaccerrmsk(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleCntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BleCntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn emaccerrack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleCntl2Reg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,BleCntl2Reg_SPEC,crate::common::W>::from_register(self,0)
    }

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
pub struct BleCoexifcntl0Reg_SPEC;
impl crate::sealed::RegSpec for BleCoexifcntl0Reg_SPEC {
    type DataType = u32;
}

pub type BleCoexifcntl0Reg = crate::RegValueT<BleCoexifcntl0Reg_SPEC>;

impl BleCoexifcntl0Reg {
    #[inline(always)]
    pub fn wlcrxpriomode(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        u8,
        u8,
        BleCoexifcntl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wlctxpriomode(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        u8,
        u8,
        BleCoexifcntl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wlantxmsk(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, BleCoexifcntl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wlanrxmsk(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, BleCoexifcntl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn syncgen_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleCoexifcntl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BleCoexifcntl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleCoexifcntl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BleCoexifcntl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleCoexifcntl0Reg {
    #[inline(always)]
    fn default() -> BleCoexifcntl0Reg {
        <crate::RegValueT<BleCoexifcntl0Reg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleCoexifcntl1Reg_SPEC;
impl crate::sealed::RegSpec for BleCoexifcntl1Reg_SPEC {
    type DataType = u32;
}

pub type BleCoexifcntl1Reg = crate::RegValueT<BleCoexifcntl1Reg_SPEC>;

impl BleCoexifcntl1Reg {
    #[inline(always)]
    pub fn wlcprxthr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1f,
        1,
        0,
        u8,
        u8,
        BleCoexifcntl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1f,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wlcptxthr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        u8,
        u8,
        BleCoexifcntl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wlcpduration(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7f,
        1,
        0,
        u8,
        u8,
        BleCoexifcntl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7f,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wlcpdelay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        BleCoexifcntl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            BleCoexifcntl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleCoexifcntl1Reg {
    #[inline(always)]
    fn default() -> BleCoexifcntl1Reg {
        <crate::RegValueT<BleCoexifcntl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleCurrentrxdescptrReg_SPEC;
impl crate::sealed::RegSpec for BleCurrentrxdescptrReg_SPEC {
    type DataType = u32;
}

pub type BleCurrentrxdescptrReg = crate::RegValueT<BleCurrentrxdescptrReg_SPEC>;

impl BleCurrentrxdescptrReg {
    #[inline(always)]
    pub fn etptr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        BleCurrentrxdescptrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            BleCurrentrxdescptrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn currentrxdescptr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        BleCurrentrxdescptrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
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

pub type BleDebugaddmaxReg = crate::RegValueT<BleDebugaddmaxReg_SPEC>;

impl BleDebugaddmaxReg {
    #[inline(always)]
    pub fn reg_addmax(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        BleDebugaddmaxReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            BleDebugaddmaxReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn em_addmax(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        BleDebugaddmaxReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
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

pub type BleDebugaddminReg = crate::RegValueT<BleDebugaddminReg_SPEC>;

impl BleDebugaddminReg {
    #[inline(always)]
    pub fn reg_addmin(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        BleDebugaddminReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            BleDebugaddminReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn em_addmin(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        BleDebugaddminReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
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

pub type BleDeepslcntlReg = crate::RegValueT<BleDeepslcntlReg_SPEC>;

impl BleDeepslcntlReg {
    #[inline(always)]
    pub fn extwkupdsb(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleDeepslcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deep_sleep_stat(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleDeepslcntlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,BleDeepslcntlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soft_wakeup_req(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleDeepslcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deep_sleep_corr_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleDeepslcntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,BleDeepslcntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deep_sleep_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleDeepslcntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,BleDeepslcntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deep_sleep_irq_en(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, BleDeepslcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,BleDeepslcntlReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type BleDeepslstatReg = crate::RegValueT<BleDeepslstatReg_SPEC>;

impl BleDeepslstatReg {
    #[inline(always)]
    pub fn deepsldur(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type BleDeepslwkupReg = crate::RegValueT<BleDeepslwkupReg_SPEC>;

impl BleDeepslwkupReg {
    #[inline(always)]
    pub fn deepsltime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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
pub struct BleDiagcntl2Reg_SPEC;
impl crate::sealed::RegSpec for BleDiagcntl2Reg_SPEC {
    type DataType = u32;
}

pub type BleDiagcntl2Reg = crate::RegValueT<BleDiagcntl2Reg_SPEC>;

impl BleDiagcntl2Reg {
    #[inline(always)]
    pub fn diag7_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleDiagcntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleDiagcntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag7(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, BleDiagcntl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x3f,
            1,
            0,
            u8,
            u8,
            BleDiagcntl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn diag6_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, BleDiagcntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,BleDiagcntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag6(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, BleDiagcntl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            u8,
            u8,
            BleDiagcntl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn diag5_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleDiagcntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleDiagcntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag5(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, BleDiagcntl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,BleDiagcntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag4_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleDiagcntl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BleDiagcntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag4(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, BleDiagcntl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,BleDiagcntl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleDiagcntl2Reg {
    #[inline(always)]
    fn default() -> BleDiagcntl2Reg {
        <crate::RegValueT<BleDiagcntl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDiagcntl3Reg_SPEC;
impl crate::sealed::RegSpec for BleDiagcntl3Reg_SPEC {
    type DataType = u32;
}

pub type BleDiagcntl3Reg = crate::RegValueT<BleDiagcntl3Reg_SPEC>;

impl BleDiagcntl3Reg {
    #[inline(always)]
    pub fn diag7_inv(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag7_bit(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag6_inv(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag6_bit(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag5_inv(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag5_bit(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag4_inv(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag4_bit(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag3_inv(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag3_bit(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag2_inv(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag2_bit(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_inv(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_bit(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag0_inv(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleDiagcntl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag0_bit(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, BleDiagcntl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,BleDiagcntl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleDiagcntl3Reg {
    #[inline(always)]
    fn default() -> BleDiagcntl3Reg {
        <crate::RegValueT<BleDiagcntl3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleDiagcntlReg_SPEC;
impl crate::sealed::RegSpec for BleDiagcntlReg_SPEC {
    type DataType = u32;
}

pub type BleDiagcntlReg = crate::RegValueT<BleDiagcntlReg_SPEC>;

impl BleDiagcntlReg {
    #[inline(always)]
    pub fn diag3_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag3(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag2_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag2(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag0_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleDiagcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag0(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, BleDiagcntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,BleDiagcntlReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type BleDiagstatReg = crate::RegValueT<BleDiagstatReg_SPEC>;

impl BleDiagstatReg {
    #[inline(always)]
    pub fn diag3stat(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag2stat(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag1stat(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diag0stat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BleDiagstatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BleDiagstatReg_SPEC,crate::common::R>::from_register(self,0)
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
pub struct BleEmBaseReg_SPEC;
impl crate::sealed::RegSpec for BleEmBaseReg_SPEC {
    type DataType = u32;
}

pub type BleEmBaseReg = crate::RegValueT<BleEmBaseReg_SPEC>;

impl BleEmBaseReg {
    #[inline(always)]
    pub fn ble_em_base_16_10(
        self,
    ) -> crate::common::RegisterField<10, 0x7f, 1, 0, u8, u8, BleEmBaseReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x7f,1,0,u8,u8,BleEmBaseReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleEmBaseReg {
    #[inline(always)]
    fn default() -> BleEmBaseReg {
        <crate::RegValueT<BleEmBaseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleEnbpresetReg_SPEC;
impl crate::sealed::RegSpec for BleEnbpresetReg_SPEC {
    type DataType = u32;
}

pub type BleEnbpresetReg = crate::RegValueT<BleEnbpresetReg_SPEC>;

impl BleEnbpresetReg {
    #[inline(always)]
    pub fn twext(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7ff,
        1,
        0,
        u16,
        u16,
        BleEnbpresetReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7ff,
            1,
            0,
            u16,
            u16,
            BleEnbpresetReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn twirq_set(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x7ff,
        1,
        0,
        u16,
        u16,
        BleEnbpresetReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x7ff,
            1,
            0,
            u16,
            u16,
            BleEnbpresetReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn twirq_reset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        BleEnbpresetReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            BleEnbpresetReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type BleErrortypestatReg = crate::RegValueT<BleErrortypestatReg_SPEC>;

impl BleErrortypestatReg {
    #[inline(always)]
    pub fn concevtirq_error(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<17,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxdata_ptr_error(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<16,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txdata_ptr_error(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxdesc_empty_error(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<14,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txdesc_empty_error(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn csformat_error(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn llchmap_error(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adv_underrun(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BleErrortypestatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ifs_underrun(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn whitelist_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn evt_cntl_apfm_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn evt_schdl_apfm_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn evt_schdl_entry_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn evt_schdl_emacc_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn radio_emacc_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pktcntl_emacc_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxcrypt_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleErrortypestatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleErrortypestatReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type BleFinecntcorrReg = crate::RegValueT<BleFinecntcorrReg_SPEC>;

impl BleFinecntcorrReg {
    #[inline(always)]
    pub fn finecntcorr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        BleFinecntcorrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            BleFinecntcorrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type BleFinetimecntReg = crate::RegValueT<BleFinetimecntReg_SPEC>;

impl BleFinetimecntReg {
    #[inline(always)]
    pub fn finecnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        BleFinetimecntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            BleFinetimecntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type BleFinetimtgtReg = crate::RegValueT<BleFinetimtgtReg_SPEC>;

impl BleFinetimtgtReg {
    #[inline(always)]
    pub fn finetarget(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffffff,
        1,
        0,
        u32,
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

pub type BleGrosstimtgtReg = crate::RegValueT<BleGrosstimtgtReg_SPEC>;

impl BleGrosstimtgtReg {
    #[inline(always)]
    pub fn grosstarget(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        BleGrosstimtgtReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
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

pub type BleIntackReg = crate::RegValueT<BleIntackReg_SPEC>;

impl BleIntackReg {
    #[inline(always)]
    pub fn swintack(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventapfaintack(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn finetgtimintack(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn grosstgtimintack(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn errorintack(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cryptintack(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventintack(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slpintack(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxintack(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntackReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntackReg_SPEC,crate::common::W>::from_register(self,0)
    }

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

pub type BleIntcntlReg = crate::RegValueT<BleIntcntlReg_SPEC>;

impl BleIntcntlReg {
    #[inline(always)]
    pub fn cscntdevmsk(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventapfaintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn finetgtimintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn grosstgtimintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn errorintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cryptintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slpintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxintmsk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        <crate::RegValueT<BleIntcntlReg_SPEC> as RegisterValue<_>>::new(33055)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleIntrawstatReg_SPEC;
impl crate::sealed::RegSpec for BleIntrawstatReg_SPEC {
    type DataType = u32;
}

pub type BleIntrawstatReg = crate::RegValueT<BleIntrawstatReg_SPEC>;

impl BleIntrawstatReg {
    #[inline(always)]
    pub fn swintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventapfaintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn finetgtimintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn grosstgtimintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn errorintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cryptintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slpintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxintrawstat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntrawstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntrawstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type BleIntstatReg = crate::RegValueT<BleIntstatReg_SPEC>;

impl BleIntstatReg {
    #[inline(always)]
    pub fn swintstat(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventapfaintstat(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn finetgtimintstat(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn grosstgtimintstat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn errorintstat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cryptintstat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eventintstat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slpintstat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxintstat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleIntstatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleIntstatReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type BleRadiocntl0Reg = crate::RegValueT<BleRadiocntl0Reg_SPEC>;

impl NoBitfieldReg<BleRadiocntl0Reg_SPEC> for BleRadiocntl0Reg {}
impl ::core::default::Default for BleRadiocntl0Reg {
    #[inline(always)]
    fn default() -> BleRadiocntl0Reg {
        <crate::RegValueT<BleRadiocntl0Reg_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRadiocntl1Reg_SPEC;
impl crate::sealed::RegSpec for BleRadiocntl1Reg_SPEC {
    type DataType = u32;
}

pub type BleRadiocntl1Reg = crate::RegValueT<BleRadiocntl1Reg_SPEC>;

impl BleRadiocntl1Reg {
    #[inline(always)]
    pub fn xrfsel(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        u8,
        u8,
        BleRadiocntl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            u8,
            u8,
            BleRadiocntl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
pub struct BleRadiocntl2Reg_SPEC;
impl crate::sealed::RegSpec for BleRadiocntl2Reg_SPEC {
    type DataType = u32;
}

pub type BleRadiocntl2Reg = crate::RegValueT<BleRadiocntl2Reg_SPEC>;

impl NoBitfieldReg<BleRadiocntl2Reg_SPEC> for BleRadiocntl2Reg {}
impl ::core::default::Default for BleRadiocntl2Reg {
    #[inline(always)]
    fn default() -> BleRadiocntl2Reg {
        <crate::RegValueT<BleRadiocntl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRadiocntl3Reg_SPEC;
impl crate::sealed::RegSpec for BleRadiocntl3Reg_SPEC {
    type DataType = u32;
}

pub type BleRadiocntl3Reg = crate::RegValueT<BleRadiocntl3Reg_SPEC>;

impl NoBitfieldReg<BleRadiocntl3Reg_SPEC> for BleRadiocntl3Reg {}
impl ::core::default::Default for BleRadiocntl3Reg {
    #[inline(always)]
    fn default() -> BleRadiocntl3Reg {
        <crate::RegValueT<BleRadiocntl3Reg_SPEC> as RegisterValue<_>>::new(64)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRadiopwrupdnReg_SPEC;
impl crate::sealed::RegSpec for BleRadiopwrupdnReg_SPEC {
    type DataType = u32;
}

pub type BleRadiopwrupdnReg = crate::RegValueT<BleRadiopwrupdnReg_SPEC>;

impl BleRadiopwrupdnReg {
    #[inline(always)]
    pub fn rtrip_delay(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7f,
        1,
        0,
        u8,
        u8,
        BleRadiopwrupdnReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7f,
            1,
            0,
            u8,
            u8,
            BleRadiopwrupdnReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxpwrup(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        BleRadiopwrupdnReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            BleRadiopwrupdnReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txpwrdn(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        BleRadiopwrupdnReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            BleRadiopwrupdnReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txpwrup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        BleRadiopwrupdnReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            BleRadiopwrupdnReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type BleRftestcntlReg = crate::RegValueT<BleRftestcntlReg_SPEC>;

impl BleRftestcntlReg {
    #[inline(always)]
    pub fn infiniterx(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxpktcnten(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn infinitetx(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txlengthsrc(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prbstype(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txpldsrc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txpktcnten(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BleRftestcntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,BleRftestcntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txlength(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BleRftestcntlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BleRftestcntlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
pub struct BleRftestrxstatReg_SPEC;
impl crate::sealed::RegSpec for BleRftestrxstatReg_SPEC {
    type DataType = u32;
}

pub type BleRftestrxstatReg = crate::RegValueT<BleRftestrxstatReg_SPEC>;

impl BleRftestrxstatReg {
    #[inline(always)]
    pub fn rxpktcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        BleRftestrxstatReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            BleRftestrxstatReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleRftestrxstatReg {
    #[inline(always)]
    fn default() -> BleRftestrxstatReg {
        <crate::RegValueT<BleRftestrxstatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRftesttxstatReg_SPEC;
impl crate::sealed::RegSpec for BleRftesttxstatReg_SPEC {
    type DataType = u32;
}

pub type BleRftesttxstatReg = crate::RegValueT<BleRftesttxstatReg_SPEC>;

impl BleRftesttxstatReg {
    #[inline(always)]
    pub fn txpktcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        BleRftesttxstatReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            BleRftesttxstatReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleRftesttxstatReg {
    #[inline(always)]
    fn default() -> BleRftesttxstatReg {
        <crate::RegValueT<BleRftesttxstatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRwblecntlReg_SPEC;
impl crate::sealed::RegSpec for BleRwblecntlReg_SPEC {
    type DataType = u32;
}

pub type BleRwblecntlReg = crate::RegValueT<BleRwblecntlReg_SPEC>;

impl BleRwblecntlReg {
    #[inline(always)]
    pub fn master_soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleRwblecntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31,1,0,BleRwblecntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn master_tgsoft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, BleRwblecntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,BleRwblecntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reg_soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swint_req(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, BleRwblecntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,BleRwblecntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rftest_abort(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, BleRwblecntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26,1,0,BleRwblecntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn advert_abort(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, BleRwblecntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,BleRwblecntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scan_abort(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, BleRwblecntlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24,1,0,BleRwblecntlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn md_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sn_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn nesn_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crypt_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn whit_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hop_remap_dsb(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn advertfilt_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rwble_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BleRwblecntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxwinszdef(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, BleRwblecntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn syncerr(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, BleRwblecntlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,BleRwblecntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRwblecntlReg {
    #[inline(always)]
    fn default() -> BleRwblecntlReg {
        <crate::RegValueT<BleRwblecntlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRwbleconfReg_SPEC;
impl crate::sealed::RegSpec for BleRwbleconfReg_SPEC {
    type DataType = u32;
}

pub type BleRwbleconfReg = crate::RegValueT<BleRwbleconfReg_SPEC>;

impl BleRwbleconfReg {
    #[inline(always)]
    pub fn add_width(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, BleRwbleconfReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfif(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, BleRwbleconfReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_sel(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, BleRwbleconfReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn decipher(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BleRwbleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dmmode(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BleRwbleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn intmode(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BleRwbleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BleRwbleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usedbg(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BleRwbleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usecrypt(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BleRwbleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buswidth(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleRwbleconfReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BleRwbleconfReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleRwbleconfReg {
    #[inline(always)]
    fn default() -> BleRwbleconfReg {
        <crate::RegValueT<BleRwbleconfReg_SPEC> as RegisterValue<_>>::new(251789343)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleRxmicvalReg_SPEC;
impl crate::sealed::RegSpec for BleRxmicvalReg_SPEC {
    type DataType = u32;
}

pub type BleRxmicvalReg = crate::RegValueT<BleRxmicvalReg_SPEC>;

impl BleRxmicvalReg {
    #[inline(always)]
    pub fn rxmicval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        BleRxmicvalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
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

pub type BleSampleclkReg = crate::RegValueT<BleSampleclkReg_SPEC>;

impl BleSampleclkReg {
    #[inline(always)]
    pub fn samp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BleSampleclkReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,BleSampleclkReg_SPEC,crate::common::W>::from_register(self,0)
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

pub type BleSwprofilingReg = crate::RegValueT<BleSwprofilingReg_SPEC>;

impl BleSwprofilingReg {
    #[inline(always)]
    pub fn swprofval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type BleTimgencntlReg = crate::RegValueT<BleTimgencntlReg_SPEC>;

impl BleTimgencntlReg {
    #[inline(always)]
    pub fn apfm_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BleTimgencntlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,BleTimgencntlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prefetchabort_time(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3ff,
        1,
        0,
        u16,
        u16,
        BleTimgencntlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3ff,
            1,
            0,
            u16,
            u16,
            BleTimgencntlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn prefetch_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        u16,
        u16,
        BleTimgencntlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            u16,
            u16,
            BleTimgencntlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleTimgencntlReg {
    #[inline(always)]
    fn default() -> BleTimgencntlReg {
        <crate::RegValueT<BleTimgencntlReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleTxmicvalReg_SPEC;
impl crate::sealed::RegSpec for BleTxmicvalReg_SPEC {
    type DataType = u32;
}

pub type BleTxmicvalReg = crate::RegValueT<BleTxmicvalReg_SPEC>;

impl BleTxmicvalReg {
    #[inline(always)]
    pub fn txmicval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        BleTxmicvalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
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

pub type BleVersionReg = crate::RegValueT<BleVersionReg_SPEC>;

impl BleVersionReg {
    #[inline(always)]
    pub fn typ(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, BleVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, BleVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn upg(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BleVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn build(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BleVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BleVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BleVersionReg {
    #[inline(always)]
    fn default() -> BleVersionReg {
        <crate::RegValueT<BleVersionReg_SPEC> as RegisterValue<_>>::new(117506048)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleWlnbdevReg_SPEC;
impl crate::sealed::RegSpec for BleWlnbdevReg_SPEC {
    type DataType = u32;
}

pub type BleWlnbdevReg = crate::RegValueT<BleWlnbdevReg_SPEC>;

impl BleWlnbdevReg {
    #[inline(always)]
    pub fn nbprivdev(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, BleWlnbdevReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,BleWlnbdevReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn nbpubdev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, BleWlnbdevReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,BleWlnbdevReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type BleWlprivaddptrReg = crate::RegValueT<BleWlprivaddptrReg_SPEC>;

impl BleWlprivaddptrReg {
    #[inline(always)]
    pub fn wlprivaddptr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
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

pub type BleWlpubaddptrReg = crate::RegValueT<BleWlpubaddptrReg_SPEC>;

impl BleWlpubaddptrReg {
    #[inline(always)]
    pub fn wlpubaddptr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        BleWlpubaddptrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
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
