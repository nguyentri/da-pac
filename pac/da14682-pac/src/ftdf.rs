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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"FTDF registers"]
unsafe impl ::core::marker::Send for super::Ftdf {}
unsafe impl ::core::marker::Sync for super::Ftdf {}
impl super::Ftdf {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ftdf_buildtime_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfBuildtime0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfBuildtime0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65552usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_buildtime_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfBuildtime1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfBuildtime1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65556usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_buildtime_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfBuildtime2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfBuildtime2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65560usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_buildtime_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfBuildtime3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfBuildtime3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65564usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_debugcontrol_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfDebugcontrolReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfDebugcontrolReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66448usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_eventcurrval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfEventcurrvalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfEventcurrvalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65624usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_ftdf_ce_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfFtdfCeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfFtdfCeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_ftdf_cm_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfFtdfCmReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfFtdfCmReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_glob_control_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfGlobControl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfGlobControl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65568usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_glob_control_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfGlobControl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfGlobControl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65572usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_glob_control_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfGlobControl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfGlobControl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65576usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_glob_control_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfGlobControl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfGlobControl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65580usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmacreset_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacresetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacresetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66400usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65584usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65804usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65644usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65600usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65604usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65608usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65632usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65636usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65640usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65792usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65796usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControl9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControl9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65800usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_delta_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControlDeltaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControlDeltaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65648usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControlMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControlMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65664usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_os_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControlOsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControlOsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65616usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_control_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacControlStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacControlStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65620usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_event_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacEventReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacEventReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65680usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_manual_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacManual1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65696usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_manual_os_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacManualOsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacManualOsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65700usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_manual_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacManualStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacManualStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65704usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_lmac_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLmacMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLmacMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65684usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_long_addr_0_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLongAddr00Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLongAddr00Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(73728usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_long_addr_1_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfLongAddr10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfLongAddr10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(73732usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_macackwaitduration_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMacackwaitdurationReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfMacackwaitdurationReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65592usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_macenhackwaitduration_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMacenhackwaitdurationReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfMacenhackwaitdurationReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(65596usize))
        }
    }

    #[inline(always)]
    pub const fn ftdf_macfcserrorcount_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMacfcserrorcountReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfMacfcserrorcountReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66368usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_macrxaddrfailfrmcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMacrxaddrfailfrmcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfMacrxaddrfailfrmcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66328usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_macrxstdackfrmokcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMacrxstdackfrmokcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfMacrxstdackfrmokcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66324usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_macrxunsupfrmcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMacrxunsupfrmcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfMacrxunsupfrmcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66332usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_mactstxackdelayval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMactstxackdelayvalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfMactstxackdelayvalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65656usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_mactxstdackfrmcnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMactxstdackfrmcntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfMactxstdackfrmcntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66320usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_phy_parameters_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfPhyParameters0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfPhyParameters0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65920usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_phy_parameters_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfPhyParameters1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfPhyParameters1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65924usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_phy_parameters_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfPhyParameters2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfPhyParameters2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65928usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_phy_parameters_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfPhyParameters3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfPhyParameters3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65932usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rel_name_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRelName0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRelName0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65536usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rel_name_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRelName1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRelName1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65540usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rel_name_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRelName2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRelName2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65544usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rel_name_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRelName3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRelName3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65548usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_control_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxControl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxControl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66048usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_event_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxEventReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxEventReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66052usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_0_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo00Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo00Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32768usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_1_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32896usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_2_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo20Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo20Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33024usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_3_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo30Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo30Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_4_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo40Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo40Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_5_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo50Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo50Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33408usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_6_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo60Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo60Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33536usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_fifo_7_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxFifo70Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxFifo70Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33664usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66056usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta00Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta00Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(640usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta01Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta01Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(656usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta02Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta02Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(672usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(688usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta04Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta04Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(704usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta05Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta05Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(720usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta06Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta06Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(736usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_0_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta07Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta07Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(752usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(644usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(660usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(676usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(692usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(708usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(724usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta16Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta16Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(740usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_meta_1_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxMeta17Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxMeta17Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(756usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_status_delta_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxStatusDeltaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxStatusDeltaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66080usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_status_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxStatusMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxStatusMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66084usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_rx_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfRxStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfRxStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66060usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_seckey_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSeckey0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSeckey0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65816usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_seckey_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSeckey1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSeckey1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65820usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_seckey_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSeckey2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSeckey2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65824usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_seckey_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSeckey3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSeckey3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65828usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_secnonce_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecnonce0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecnonce0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65832usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_secnonce_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecnonce1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecnonce1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65836usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_secnonce_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecnonce2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecnonce2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65840usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_secnonce_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecnonce3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecnonce3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65844usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_security_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecurity0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecurity0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65808usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_security_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecurity1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecurity1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65812usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_security_eventmask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecurityEventmaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecurityEventmaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65876usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_security_event_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecurityEventReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecurityEventReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65872usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_security_os_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecurityOsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecurityOsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65848usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_security_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSecurityStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSecurityStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65856usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_size_and_val_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSizeAndVal0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSizeAndVal0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(73736usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_symboltime2thr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSymboltime2ThrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSymboltime2ThrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66436usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_symboltimesnapshotval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSymboltimesnapshotvalReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfSymboltimesnapshotvalReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(66064usize))
        }
    }

    #[inline(always)]
    pub const fn ftdf_symboltimethr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSymboltimethrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSymboltimethrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66432usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_synctimestampphaseval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSynctimestampphasevalReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfSynctimestampphasevalReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(66336usize))
        }
    }

    #[inline(always)]
    pub const fn ftdf_synctimestampthr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSynctimestampthrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSynctimestampthrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66308usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_synctimestampval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSynctimestampvalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfSynctimestampvalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66312usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_timer_control_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTimerControl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTimerControl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66316usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_timestampcurrphaseval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTimestampcurrphasevalReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfTimestampcurrphasevalReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(65652usize))
        }
    }

    #[inline(always)]
    pub const fn ftdf_timestampcurrval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTimestampcurrvalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTimestampcurrvalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65628usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tsch_control_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTschControl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTschControl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65888usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tsch_control_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTschControl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTschControl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65892usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tsch_control_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTschControl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTschControl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65896usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_txbyte_e_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxbyteEReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxbyteEReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66452usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_txbyte_m_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxbyteMReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxbyteMReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66456usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_txpipepropdelay_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxpipepropdelayReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxpipepropdelayReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65588usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_clear_os_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxClearOsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxClearOsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66692usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_control_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxControl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxControl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_fifo_0_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFifo00Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFifo00Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_fifo_1_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFifo10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFifo10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_fifo_2_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFifo20Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFifo20Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_fifo_3_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFifo30Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFifo30Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_e_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearE0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearE0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66564usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_e_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearE1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearE1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66596usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_e_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearE2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearE2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66628usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_e_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearE3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearE3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66660usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_m_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearM0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearM0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66568usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_m_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearM1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearM1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66600usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_m_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearM2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearM2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66632usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_clear_m_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagClearM3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagClearM3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66664usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_s_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagS0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagS0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66560usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_s_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagS1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagS1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66592usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_s_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagS2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagS2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66624usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_flag_s_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxFlagS3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxFlagS3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66656usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_0_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData00Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData00Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_0_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData01Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData01Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_0_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData02Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData02Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(560usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_1_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_1_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_1_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(548usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_meta_data_1_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxMetaData13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxMetaData13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(564usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_priority_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxPriority0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxPriority0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66576usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_priority_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxPriority1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxPriority1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66608usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_priority_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxPriority2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxPriority2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66640usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_priority_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxPriority3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxPriority3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66672usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_0_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus00Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus00Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(576usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_0_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus01Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus01Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(592usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_0_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus02Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus02Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(608usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(624usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_1_0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(580usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_1_1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(596usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_1_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(612usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_return_status_1_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxReturnStatus13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxReturnStatus13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(628usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_tx_set_os_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTxSetOsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfTxSetOsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66688usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_wakeup_control_os_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfWakeupControlOsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfWakeupControlOsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66404usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftdf_wakeup_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfWakeupControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfWakeupControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(69632usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfBuildtime0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfBuildtime0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfBuildtime0Reg = crate::RegValueT<FtdfBuildtime0Reg_SPEC>;

impl FtdfBuildtime0Reg {
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfBuildtime0Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfBuildtime0Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfBuildtime0Reg {
    #[inline(always)]
    fn default() -> FtdfBuildtime0Reg {
        <crate::RegValueT<FtdfBuildtime0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfBuildtime1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfBuildtime1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfBuildtime1Reg = crate::RegValueT<FtdfBuildtime1Reg_SPEC>;

impl FtdfBuildtime1Reg {
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfBuildtime1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfBuildtime1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfBuildtime1Reg {
    #[inline(always)]
    fn default() -> FtdfBuildtime1Reg {
        <crate::RegValueT<FtdfBuildtime1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfBuildtime2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfBuildtime2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfBuildtime2Reg = crate::RegValueT<FtdfBuildtime2Reg_SPEC>;

impl FtdfBuildtime2Reg {
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfBuildtime2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfBuildtime2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfBuildtime2Reg {
    #[inline(always)]
    fn default() -> FtdfBuildtime2Reg {
        <crate::RegValueT<FtdfBuildtime2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfBuildtime3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfBuildtime3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfBuildtime3Reg = crate::RegValueT<FtdfBuildtime3Reg_SPEC>;

impl FtdfBuildtime3Reg {
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfBuildtime3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfBuildtime3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfBuildtime3Reg {
    #[inline(always)]
    fn default() -> FtdfBuildtime3Reg {
        <crate::RegValueT<FtdfBuildtime3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfDebugcontrolReg_SPEC;
impl crate::sealed::RegSpec for FtdfDebugcontrolReg_SPEC {
    type DataType = u32;
}

pub type FtdfDebugcontrolReg = crate::RegValueT<FtdfDebugcontrolReg_SPEC>;

impl FtdfDebugcontrolReg {
    #[inline(always)]
    pub fn dbg_rx_input(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, FtdfDebugcontrolReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,FtdfDebugcontrolReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfDebugcontrolReg {
    #[inline(always)]
    fn default() -> FtdfDebugcontrolReg {
        <crate::RegValueT<FtdfDebugcontrolReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfEventcurrvalReg_SPEC;
impl crate::sealed::RegSpec for FtdfEventcurrvalReg_SPEC {
    type DataType = u32;
}

pub type FtdfEventcurrvalReg = crate::RegValueT<FtdfEventcurrvalReg_SPEC>;

impl FtdfEventcurrvalReg {
    #[inline(always)]
    pub fn eventcurrval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ffffff,
        1,
        0,
        u32,
        u32,
        FtdfEventcurrvalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1ffffff,
            1,
            0,
            u32,
            u32,
            FtdfEventcurrvalReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfEventcurrvalReg {
    #[inline(always)]
    fn default() -> FtdfEventcurrvalReg {
        <crate::RegValueT<FtdfEventcurrvalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfFtdfCeReg_SPEC;
impl crate::sealed::RegSpec for FtdfFtdfCeReg_SPEC {
    type DataType = u32;
}

pub type FtdfFtdfCeReg = crate::RegValueT<FtdfFtdfCeReg_SPEC>;

impl FtdfFtdfCeReg {
    #[inline(always)]
    pub fn ftdf_ce(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, FtdfFtdfCeReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,FtdfFtdfCeReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfFtdfCeReg {
    #[inline(always)]
    fn default() -> FtdfFtdfCeReg {
        <crate::RegValueT<FtdfFtdfCeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfFtdfCmReg_SPEC;
impl crate::sealed::RegSpec for FtdfFtdfCmReg_SPEC {
    type DataType = u32;
}

pub type FtdfFtdfCmReg = crate::RegValueT<FtdfFtdfCmReg_SPEC>;

impl FtdfFtdfCmReg {
    #[inline(always)]
    pub fn ftdf_cm(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, FtdfFtdfCmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,FtdfFtdfCmReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfFtdfCmReg {
    #[inline(always)]
    fn default() -> FtdfFtdfCmReg {
        <crate::RegValueT<FtdfFtdfCmReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfGlobControl0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfGlobControl0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfGlobControl0Reg = crate::RegValueT<FtdfGlobControl0Reg_SPEC>;

impl FtdfGlobControl0Reg {
    #[inline(always)]
    pub fn mactschenabled(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<18,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macleenabled(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<17,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macsimpleaddress(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfGlobControl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfGlobControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tx_dma_req(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_dma_req(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispancoordinator(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfGlobControl0Reg {
    #[inline(always)]
    fn default() -> FtdfGlobControl0Reg {
        <crate::RegValueT<FtdfGlobControl0Reg_SPEC> as RegisterValue<_>>::new(65280)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfGlobControl1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfGlobControl1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfGlobControl1Reg = crate::RegValueT<FtdfGlobControl1Reg_SPEC>;

impl FtdfGlobControl1Reg {
    #[inline(always)]
    pub fn macshortaddress(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfGlobControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfGlobControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macpanid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfGlobControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfGlobControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfGlobControl1Reg {
    #[inline(always)]
    fn default() -> FtdfGlobControl1Reg {
        <crate::RegValueT<FtdfGlobControl1Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfGlobControl2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfGlobControl2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfGlobControl2Reg = crate::RegValueT<FtdfGlobControl2Reg_SPEC>;

impl FtdfGlobControl2Reg {
    #[inline(always)]
    pub fn aextendedaddress_l(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfGlobControl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfGlobControl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfGlobControl2Reg {
    #[inline(always)]
    fn default() -> FtdfGlobControl2Reg {
        <crate::RegValueT<FtdfGlobControl2Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfGlobControl3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfGlobControl3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfGlobControl3Reg = crate::RegValueT<FtdfGlobControl3Reg_SPEC>;

impl FtdfGlobControl3Reg {
    #[inline(always)]
    pub fn aextendedaddress_h(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfGlobControl3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfGlobControl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfGlobControl3Reg {
    #[inline(always)]
    fn default() -> FtdfGlobControl3Reg {
        <crate::RegValueT<FtdfGlobControl3Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacresetReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacresetReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacresetReg = crate::RegValueT<FtdfLmacresetReg_SPEC>;

impl FtdfLmacresetReg {
    #[inline(always)]
    pub fn lmacglobreset_count(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_timctrl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_count(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_sec(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_tstim(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_oreg(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_ahb(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_rx(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacreset_control(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacresetReg {
    #[inline(always)]
    fn default() -> FtdfLmacresetReg {
        <crate::RegValueT<FtdfLmacresetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl0Reg = crate::RegValueT<FtdfLmacControl0Reg_SPEC>;

impl FtdfLmacControl0Reg {
    #[inline(always)]
    pub fn keep_phy_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pti_rx(
        self,
    ) -> crate::common::RegisterField<
        27,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxalwayson(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, FtdfLmacControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<25,1,0,FtdfLmacControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxonduration(
        self,
    ) -> crate::common::RegisterField<
        1,
        0xffffff,
        1,
        0,
        u32,
        u32,
        FtdfLmacControl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0xffffff,
            1,
            0,
            u32,
            u32,
            FtdfLmacControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl0Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl0Reg {
        <crate::RegValueT<FtdfLmacControl0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl10Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl10Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl10Reg = crate::RegValueT<FtdfLmacControl10Reg_SPEC>;

impl FtdfLmacControl10Reg {
    #[inline(always)]
    pub fn macrzzeroval(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn maccslmarginrz(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macwurzcorrection(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl10Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl10Reg {
        <crate::RegValueT<FtdfLmacControl10Reg_SPEC> as RegisterValue<_>>::new(536870912)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl11Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl11Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl11Reg = crate::RegValueT<FtdfLmacControl11Reg_SPEC>;

impl FtdfLmacControl11Reg {
    #[inline(always)]
    pub fn csma_ca_bo_threshold(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csma_ca_nb_val(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macdiscarxofftorz(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, FtdfLmacControl11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<16,1,0,FtdfLmacControl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macrxtotalcycletime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfLmacControl11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfLmacControl11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl11Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl11Reg {
        <crate::RegValueT<FtdfLmacControl11Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl1Reg = crate::RegValueT<FtdfLmacControl1Reg_SPEC>;

impl FtdfLmacControl1Reg {
    #[inline(always)]
    pub fn phyrxattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, FtdfLmacControl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,FtdfLmacControl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phyrxattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyrxattr_calcap(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyrxattr_cn(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyrxattr_dem_lqi(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyrxattr_dem_cca(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl1Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl1Reg {
        <crate::RegValueT<FtdfLmacControl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl2Reg = crate::RegValueT<FtdfLmacControl2Reg_SPEC>;

impl FtdfLmacControl2Reg {
    #[inline(always)]
    pub fn edscanduration(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xffffff,
        1,
        0,
        u32,
        u32,
        FtdfLmacControl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xffffff,
            1,
            0,
            u32,
            u32,
            FtdfLmacControl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn edscanenable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacControl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacControl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacControl2Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl2Reg {
        <crate::RegValueT<FtdfLmacControl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl3Reg = crate::RegValueT<FtdfLmacControl3Reg_SPEC>;

impl FtdfLmacControl3Reg {
    #[inline(always)]
    pub fn ftdf_lpdp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, FtdfLmacControl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<27,1,0,FtdfLmacControl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fp_force_value(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfLmacControl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfLmacControl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fp_override(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, FtdfLmacControl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<25,1,0,FtdfLmacControl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn addr_tab_match_fp_value(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, FtdfLmacControl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,FtdfLmacControl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ccaidlewait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macmaxframetotalwaittime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfLmacControl3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfLmacControl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl3Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl3Reg {
        <crate::RegValueT<FtdfLmacControl3Reg_SPEC> as RegisterValue<_>>::new(117441732)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl4Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl4Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl4Reg = crate::RegValueT<FtdfLmacControl4Reg_SPEC>;

impl FtdfLmacControl4Reg {
    #[inline(always)]
    pub fn phyackattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phyackattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyackattr_calcap(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyackattr_cn(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyackattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxpipepropdelay(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn physleepwait(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl4Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl4Reg {
        <crate::RegValueT<FtdfLmacControl4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl5Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl5Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl5Reg = crate::RegValueT<FtdfLmacControl5Reg_SPEC>;

impl FtdfLmacControl5Reg {
    #[inline(always)]
    pub fn phycsmacaattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phycsmacaattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phycsmacaattr_calcap(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phycsmacaattr_cn(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phycsmacaattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ccastatwait(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ack_response_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl5Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl5Reg {
        <crate::RegValueT<FtdfLmacControl5Reg_SPEC> as RegisterValue<_>>::new(2240)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl6Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl6Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl6Reg = crate::RegValueT<FtdfLmacControl6Reg_SPEC>;

impl FtdfLmacControl6Reg {
    #[inline(always)]
    pub fn wuifsperiod(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sifsperiod(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lifsperiod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControl6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControl6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl6Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl6Reg {
        <crate::RegValueT<FtdfLmacControl6Reg_SPEC> as RegisterValue<_>>::new(789544)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl7Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl7Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl7Reg = crate::RegValueT<FtdfLmacControl7Reg_SPEC>;

impl FtdfLmacControl7Reg {
    #[inline(always)]
    pub fn maccslsampleperiod(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfLmacControl7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfLmacControl7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macwuperiod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfLmacControl7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfLmacControl7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl7Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl7Reg {
        <crate::RegValueT<FtdfLmacControl7Reg_SPEC> as RegisterValue<_>>::new(4325376)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl8Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl8Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl8Reg = crate::RegValueT<FtdfLmacControl8Reg_SPEC>;

impl FtdfLmacControl8Reg {
    #[inline(always)]
    pub fn maccslstartsampletime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfLmacControl8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfLmacControl8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl8Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl8Reg {
        <crate::RegValueT<FtdfLmacControl8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl9Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl9Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControl9Reg = crate::RegValueT<FtdfLmacControl9Reg_SPEC>;

impl FtdfLmacControl9Reg {
    #[inline(always)]
    pub fn maccslframependingwaitt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfLmacControl9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfLmacControl9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn maccsldataperiod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfLmacControl9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfLmacControl9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl9Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl9Reg {
        <crate::RegValueT<FtdfLmacControl9Reg_SPEC> as RegisterValue<_>>::new(66)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControlDeltaReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControlDeltaReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControlDeltaReg = crate::RegValueT<FtdfLmacControlDeltaReg_SPEC>;

impl FtdfLmacControlDeltaReg {
    #[inline(always)]
    pub fn wakeuptimerenablestatus_d(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn getgeneratorval_e(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn symboltime2thr_e(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn symboltimethr_e(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn synctimestamp_e(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacready4sleep_d(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacControlDeltaReg {
    #[inline(always)]
    fn default() -> FtdfLmacControlDeltaReg {
        <crate::RegValueT<FtdfLmacControlDeltaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControlMaskReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControlMaskReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControlMaskReg = crate::RegValueT<FtdfLmacControlMaskReg_SPEC>;

impl FtdfLmacControlMaskReg {
    #[inline(always)]
    pub fn wakeuptimerenablestatus_m(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn getgeneratorval_m(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn symboltime2thr_m(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn symboltimethr_m(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn synctimestamp_m(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacready4sleep_m(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacControlMaskReg {
    #[inline(always)]
    fn default() -> FtdfLmacControlMaskReg {
        <crate::RegValueT<FtdfLmacControlMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControlOsReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControlOsReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControlOsReg = crate::RegValueT<FtdfLmacControlOsReg_SPEC>;

impl FtdfLmacControlOsReg {
    #[inline(always)]
    pub fn csma_ca_resume_clear(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn csma_ca_resume_set(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn singlecca(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxenable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn getgeneratorval(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacControlOsReg {
    #[inline(always)]
    fn default() -> FtdfLmacControlOsReg {
        <crate::RegValueT<FtdfLmacControlOsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControlStatusReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControlStatusReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacControlStatusReg = crate::RegValueT<FtdfLmacControlStatusReg_SPEC>;

impl FtdfLmacControlStatusReg {
    #[inline(always)]
    pub fn csma_ca_bo_stat(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControlStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControlStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csma_ca_resume_stat(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, FtdfLmacControlStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<19,1,0,FtdfLmacControlStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn csma_ca_nb_stat(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfLmacControlStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfLmacControlStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn edscanvalue(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacControlStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacControlStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wakeuptimerenablestatus(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacControlStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacControlStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ccastat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmacready4sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacControlStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacControlStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacControlStatusReg {
    #[inline(always)]
    fn default() -> FtdfLmacControlStatusReg {
        <crate::RegValueT<FtdfLmacControlStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacEventReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacEventReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacEventReg = crate::RegValueT<FtdfLmacEventReg_SPEC>;

impl FtdfLmacEventReg {
    #[inline(always)]
    pub fn csma_ca_bo_thr_e(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxtimerexpired_e(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ccastat_e(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edscanready_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacEventReg {
    #[inline(always)]
    fn default() -> FtdfLmacEventReg {
        <crate::RegValueT<FtdfLmacEventReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacManual1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacManual1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacManual1Reg = crate::RegValueT<FtdfLmacManual1Reg_SPEC>;

impl FtdfLmacManual1Reg {
    #[inline(always)]
    pub fn lmac_manual_phy_attr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmac_manual_phy_attr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfLmacManual1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfLmacManual1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmac_manual_phy_attr_calcap(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacManual1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacManual1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmac_manual_phy_attr_cn(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacManual1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacManual1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmac_manual_phy_attr_dem_pti(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacManual1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacManual1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmac_manual_pti(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfLmacManual1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfLmacManual1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmac_manual_tx_frm_nr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        u8,
        u8,
        FtdfLmacManual1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            u8,
            u8,
            FtdfLmacManual1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmac_manual_ed_request(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmac_manual_rx_pipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmac_manual_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmac_manual_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmac_manual_phy_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lmac_manual_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacManual1Reg {
    #[inline(always)]
    fn default() -> FtdfLmacManual1Reg {
        <crate::RegValueT<FtdfLmacManual1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacManualOsReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacManualOsReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacManualOsReg = crate::RegValueT<FtdfLmacManualOsReg_SPEC>;

impl FtdfLmacManualOsReg {
    #[inline(always)]
    pub fn lmac_manual_tx_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacManualOsReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacManualOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacManualOsReg {
    #[inline(always)]
    fn default() -> FtdfLmacManualOsReg {
        <crate::RegValueT<FtdfLmacManualOsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacManualStatusReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacManualStatusReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacManualStatusReg = crate::RegValueT<FtdfLmacManualStatusReg_SPEC>;

impl FtdfLmacManualStatusReg {
    #[inline(always)]
    pub fn lmac_manual_ed_stat(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfLmacManualStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfLmacManualStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmac_manual_cca_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacManualStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacManualStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacManualStatusReg {
    #[inline(always)]
    fn default() -> FtdfLmacManualStatusReg {
        <crate::RegValueT<FtdfLmacManualStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacMaskReg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacMaskReg_SPEC {
    type DataType = u32;
}

pub type FtdfLmacMaskReg = crate::RegValueT<FtdfLmacMaskReg_SPEC>;

impl FtdfLmacMaskReg {
    #[inline(always)]
    pub fn csma_ca_bo_thr_m(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxtimerexpired_m(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ccastat_m(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edscanready_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfLmacMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfLmacMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfLmacMaskReg {
    #[inline(always)]
    fn default() -> FtdfLmacMaskReg {
        <crate::RegValueT<FtdfLmacMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLongAddr00Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLongAddr00Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLongAddr00Reg = crate::RegValueT<FtdfLongAddr00Reg_SPEC>;

impl FtdfLongAddr00Reg {
    #[inline(always)]
    pub fn exp_sa_l(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfLongAddr00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfLongAddr00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLongAddr00Reg {
    #[inline(always)]
    fn default() -> FtdfLongAddr00Reg {
        <crate::RegValueT<FtdfLongAddr00Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLongAddr10Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLongAddr10Reg_SPEC {
    type DataType = u32;
}

pub type FtdfLongAddr10Reg = crate::RegValueT<FtdfLongAddr10Reg_SPEC>;

impl FtdfLongAddr10Reg {
    #[inline(always)]
    pub fn exp_sa_h(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfLongAddr10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfLongAddr10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLongAddr10Reg {
    #[inline(always)]
    fn default() -> FtdfLongAddr10Reg {
        <crate::RegValueT<FtdfLongAddr10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMacackwaitdurationReg_SPEC;
impl crate::sealed::RegSpec for FtdfMacackwaitdurationReg_SPEC {
    type DataType = u32;
}

pub type FtdfMacackwaitdurationReg = crate::RegValueT<FtdfMacackwaitdurationReg_SPEC>;

impl FtdfMacackwaitdurationReg {
    #[inline(always)]
    pub fn macackwaitduration(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfMacackwaitdurationReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfMacackwaitdurationReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMacackwaitdurationReg {
    #[inline(always)]
    fn default() -> FtdfMacackwaitdurationReg {
        <crate::RegValueT<FtdfMacackwaitdurationReg_SPEC> as RegisterValue<_>>::new(54)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMacenhackwaitdurationReg_SPEC;
impl crate::sealed::RegSpec for FtdfMacenhackwaitdurationReg_SPEC {
    type DataType = u32;
}

pub type FtdfMacenhackwaitdurationReg = crate::RegValueT<FtdfMacenhackwaitdurationReg_SPEC>;

impl FtdfMacenhackwaitdurationReg {
    #[inline(always)]
    pub fn macenhackwaitduration(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfMacenhackwaitdurationReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfMacenhackwaitdurationReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMacenhackwaitdurationReg {
    #[inline(always)]
    fn default() -> FtdfMacenhackwaitdurationReg {
        <crate::RegValueT<FtdfMacenhackwaitdurationReg_SPEC> as RegisterValue<_>>::new(864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMacfcserrorcountReg_SPEC;
impl crate::sealed::RegSpec for FtdfMacfcserrorcountReg_SPEC {
    type DataType = u32;
}

pub type FtdfMacfcserrorcountReg = crate::RegValueT<FtdfMacfcserrorcountReg_SPEC>;

impl FtdfMacfcserrorcountReg {
    #[inline(always)]
    pub fn macfcserrorcount(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfMacfcserrorcountReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfMacfcserrorcountReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMacfcserrorcountReg {
    #[inline(always)]
    fn default() -> FtdfMacfcserrorcountReg {
        <crate::RegValueT<FtdfMacfcserrorcountReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMacrxaddrfailfrmcntReg_SPEC;
impl crate::sealed::RegSpec for FtdfMacrxaddrfailfrmcntReg_SPEC {
    type DataType = u32;
}

pub type FtdfMacrxaddrfailfrmcntReg = crate::RegValueT<FtdfMacrxaddrfailfrmcntReg_SPEC>;

impl FtdfMacrxaddrfailfrmcntReg {
    #[inline(always)]
    pub fn macrxaddrfailfrmcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfMacrxaddrfailfrmcntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfMacrxaddrfailfrmcntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMacrxaddrfailfrmcntReg {
    #[inline(always)]
    fn default() -> FtdfMacrxaddrfailfrmcntReg {
        <crate::RegValueT<FtdfMacrxaddrfailfrmcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMacrxstdackfrmokcntReg_SPEC;
impl crate::sealed::RegSpec for FtdfMacrxstdackfrmokcntReg_SPEC {
    type DataType = u32;
}

pub type FtdfMacrxstdackfrmokcntReg = crate::RegValueT<FtdfMacrxstdackfrmokcntReg_SPEC>;

impl FtdfMacrxstdackfrmokcntReg {
    #[inline(always)]
    pub fn macrxstdackfrmokcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfMacrxstdackfrmokcntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfMacrxstdackfrmokcntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMacrxstdackfrmokcntReg {
    #[inline(always)]
    fn default() -> FtdfMacrxstdackfrmokcntReg {
        <crate::RegValueT<FtdfMacrxstdackfrmokcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMacrxunsupfrmcntReg_SPEC;
impl crate::sealed::RegSpec for FtdfMacrxunsupfrmcntReg_SPEC {
    type DataType = u32;
}

pub type FtdfMacrxunsupfrmcntReg = crate::RegValueT<FtdfMacrxunsupfrmcntReg_SPEC>;

impl FtdfMacrxunsupfrmcntReg {
    #[inline(always)]
    pub fn macrxunsupfrmcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfMacrxunsupfrmcntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfMacrxunsupfrmcntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMacrxunsupfrmcntReg {
    #[inline(always)]
    fn default() -> FtdfMacrxunsupfrmcntReg {
        <crate::RegValueT<FtdfMacrxunsupfrmcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMactstxackdelayvalReg_SPEC;
impl crate::sealed::RegSpec for FtdfMactstxackdelayvalReg_SPEC {
    type DataType = u32;
}

pub type FtdfMactstxackdelayvalReg = crate::RegValueT<FtdfMactstxackdelayvalReg_SPEC>;

impl FtdfMactstxackdelayvalReg {
    #[inline(always)]
    pub fn mactstxackdelayval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfMactstxackdelayvalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfMactstxackdelayvalReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMactstxackdelayvalReg {
    #[inline(always)]
    fn default() -> FtdfMactstxackdelayvalReg {
        <crate::RegValueT<FtdfMactstxackdelayvalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfMactxstdackfrmcntReg_SPEC;
impl crate::sealed::RegSpec for FtdfMactxstdackfrmcntReg_SPEC {
    type DataType = u32;
}

pub type FtdfMactxstdackfrmcntReg = crate::RegValueT<FtdfMactxstdackfrmcntReg_SPEC>;

impl FtdfMactxstdackfrmcntReg {
    #[inline(always)]
    pub fn mactxstdackfrmcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfMactxstdackfrmcntReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfMactxstdackfrmcntReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfMactxstdackfrmcntReg {
    #[inline(always)]
    fn default() -> FtdfMactxstdackfrmcntReg {
        <crate::RegValueT<FtdfMactxstdackfrmcntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfPhyParameters0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfPhyParameters0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfPhyParameters0Reg = crate::RegValueT<FtdfPhyParameters0Reg_SPEC>;

impl FtdfPhyParameters0Reg {
    #[inline(always)]
    pub fn rxbitpos_7(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxbitpos_6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxbitpos_5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxbitpos_4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxbitpos_3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxbitpos_2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxbitpos_1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxbitpos_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfPhyParameters0Reg {
    #[inline(always)]
    fn default() -> FtdfPhyParameters0Reg {
        <crate::RegValueT<FtdfPhyParameters0Reg_SPEC> as RegisterValue<_>>::new(1985229328)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfPhyParameters1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfPhyParameters1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfPhyParameters1Reg = crate::RegValueT<FtdfPhyParameters1Reg_SPEC>;

impl FtdfPhyParameters1Reg {
    #[inline(always)]
    pub fn txbitpos_7(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbitpos_6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbitpos_5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbitpos_4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbitpos_3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbitpos_2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbitpos_1(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn txbitpos_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfPhyParameters1Reg {
    #[inline(always)]
    fn default() -> FtdfPhyParameters1Reg {
        <crate::RegValueT<FtdfPhyParameters1Reg_SPEC> as RegisterValue<_>>::new(1985229328)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfPhyParameters2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfPhyParameters2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfPhyParameters2Reg = crate::RegValueT<FtdfPhyParameters2Reg_SPEC>;

impl FtdfPhyParameters2Reg {
    #[inline(always)]
    pub fn phytrxwait(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phytxfinish(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phytxlatency(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phytxstartup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfPhyParameters2Reg {
    #[inline(always)]
    fn default() -> FtdfPhyParameters2Reg {
        <crate::RegValueT<FtdfPhyParameters2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfPhyParameters3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfPhyParameters3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfPhyParameters3Reg = crate::RegValueT<FtdfPhyParameters3Reg_SPEC>;

impl FtdfPhyParameters3Reg {
    #[inline(always)]
    pub fn use_legacy_phy_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, FtdfPhyParameters3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,FtdfPhyParameters3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phyenable(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyrxlatency(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyrxstartup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfPhyParameters3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfPhyParameters3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfPhyParameters3Reg {
    #[inline(always)]
    fn default() -> FtdfPhyParameters3Reg {
        <crate::RegValueT<FtdfPhyParameters3Reg_SPEC> as RegisterValue<_>>::new(16777216)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRelName0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRelName0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRelName0Reg = crate::RegValueT<FtdfRelName0Reg_SPEC>;

impl FtdfRelName0Reg {
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRelName0Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRelName0Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRelName0Reg {
    #[inline(always)]
    fn default() -> FtdfRelName0Reg {
        <crate::RegValueT<FtdfRelName0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRelName1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRelName1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRelName1Reg = crate::RegValueT<FtdfRelName1Reg_SPEC>;

impl FtdfRelName1Reg {
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRelName1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRelName1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRelName1Reg {
    #[inline(always)]
    fn default() -> FtdfRelName1Reg {
        <crate::RegValueT<FtdfRelName1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRelName2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRelName2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRelName2Reg = crate::RegValueT<FtdfRelName2Reg_SPEC>;

impl FtdfRelName2Reg {
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRelName2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRelName2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRelName2Reg {
    #[inline(always)]
    fn default() -> FtdfRelName2Reg {
        <crate::RegValueT<FtdfRelName2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRelName3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRelName3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRelName3Reg = crate::RegValueT<FtdfRelName3Reg_SPEC>;

impl FtdfRelName3Reg {
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRelName3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRelName3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRelName3Reg {
    #[inline(always)]
    fn default() -> FtdfRelName3Reg {
        <crate::RegValueT<FtdfRelName3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxControl0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxControl0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxControl0Reg = crate::RegValueT<FtdfRxControl0Reg_SPEC>;

impl FtdfRxControl0Reg {
    #[inline(always)]
    pub fn disrxackreceivedca(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macimplicitbroadcast(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macpasswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macalwayspasswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macalwayspassfrmtype(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfRxControl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfRxControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macalwayspasstopancoordinator(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macalwayspassbeaconwrongpanid(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macalwayspasswrongdaddr(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macalwayspasswrongdpanid(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macalwayspassresframeversion(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disdatarequestca(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn macalwayspasscrcerror(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disrxackrequestca(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disrxfrmpendingca(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_read_buf_ptr(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, FtdfRxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfRxControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxcoordrealignonly(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxbeacononly(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbgrxtransparentmode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxControl0Reg {
    #[inline(always)]
    fn default() -> FtdfRxControl0Reg {
        <crate::RegValueT<FtdfRxControl0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxEventReg_SPEC;
impl crate::sealed::RegSpec for FtdfRxEventReg_SPEC {
    type DataType = u32;
}

pub type FtdfRxEventReg = crate::RegValueT<FtdfRxEventReg_SPEC>;

impl FtdfRxEventReg {
    #[inline(always)]
    pub fn rxbyte_e(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_buf_avail_e(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_overflow_e(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfRxEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfRxEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxsof_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxEventReg {
    #[inline(always)]
    fn default() -> FtdfRxEventReg {
        <crate::RegValueT<FtdfRxEventReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo00Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo00Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo00Reg = crate::RegValueT<FtdfRxFifo00Reg_SPEC>;

impl FtdfRxFifo00Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo00Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo00Reg {
        <crate::RegValueT<FtdfRxFifo00Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo10Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo10Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo10Reg = crate::RegValueT<FtdfRxFifo10Reg_SPEC>;

impl FtdfRxFifo10Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo10Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo10Reg {
        <crate::RegValueT<FtdfRxFifo10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo20Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo20Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo20Reg = crate::RegValueT<FtdfRxFifo20Reg_SPEC>;

impl FtdfRxFifo20Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo20Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo20Reg {
        <crate::RegValueT<FtdfRxFifo20Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo30Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo30Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo30Reg = crate::RegValueT<FtdfRxFifo30Reg_SPEC>;

impl FtdfRxFifo30Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo30Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo30Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo30Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo30Reg {
        <crate::RegValueT<FtdfRxFifo30Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo40Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo40Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo40Reg = crate::RegValueT<FtdfRxFifo40Reg_SPEC>;

impl FtdfRxFifo40Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo40Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo40Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo40Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo40Reg {
        <crate::RegValueT<FtdfRxFifo40Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo50Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo50Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo50Reg = crate::RegValueT<FtdfRxFifo50Reg_SPEC>;

impl FtdfRxFifo50Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo50Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo50Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo50Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo50Reg {
        <crate::RegValueT<FtdfRxFifo50Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo60Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo60Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo60Reg = crate::RegValueT<FtdfRxFifo60Reg_SPEC>;

impl FtdfRxFifo60Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo60Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo60Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo60Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo60Reg {
        <crate::RegValueT<FtdfRxFifo60Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxFifo70Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxFifo70Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxFifo70Reg = crate::RegValueT<FtdfRxFifo70Reg_SPEC>;

impl FtdfRxFifo70Reg {
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxFifo70Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxFifo70Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxFifo70Reg {
    #[inline(always)]
    fn default() -> FtdfRxFifo70Reg {
        <crate::RegValueT<FtdfRxFifo70Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMaskReg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMaskReg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMaskReg = crate::RegValueT<FtdfRxMaskReg_SPEC>;

impl FtdfRxMaskReg {
    #[inline(always)]
    pub fn rxbyte_m(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_buf_avail_m(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_overflow_m(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfRxMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfRxMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxsof_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMaskReg {
    #[inline(always)]
    fn default() -> FtdfRxMaskReg {
        <crate::RegValueT<FtdfRxMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta00Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta00Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta00Reg = crate::RegValueT<FtdfRxMeta00Reg_SPEC>;

impl FtdfRxMeta00Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta00Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta00Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta00Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta00Reg {
        <crate::RegValueT<FtdfRxMeta00Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta01Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta01Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta01Reg = crate::RegValueT<FtdfRxMeta01Reg_SPEC>;

impl FtdfRxMeta01Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta01Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta01Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta01Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta01Reg {
        <crate::RegValueT<FtdfRxMeta01Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta02Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta02Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta02Reg = crate::RegValueT<FtdfRxMeta02Reg_SPEC>;

impl FtdfRxMeta02Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta02Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta02Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta02Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta02Reg {
        <crate::RegValueT<FtdfRxMeta02Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta03Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta03Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta03Reg = crate::RegValueT<FtdfRxMeta03Reg_SPEC>;

impl FtdfRxMeta03Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta03Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta03Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta03Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta03Reg {
        <crate::RegValueT<FtdfRxMeta03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta04Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta04Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta04Reg = crate::RegValueT<FtdfRxMeta04Reg_SPEC>;

impl FtdfRxMeta04Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta04Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta04Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta04Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta04Reg {
        <crate::RegValueT<FtdfRxMeta04Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta05Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta05Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta05Reg = crate::RegValueT<FtdfRxMeta05Reg_SPEC>;

impl FtdfRxMeta05Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta05Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta05Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta05Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta05Reg {
        <crate::RegValueT<FtdfRxMeta05Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta06Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta06Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta06Reg = crate::RegValueT<FtdfRxMeta06Reg_SPEC>;

impl FtdfRxMeta06Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta06Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta06Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta06Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta06Reg {
        <crate::RegValueT<FtdfRxMeta06Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta07Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta07Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta07Reg = crate::RegValueT<FtdfRxMeta07Reg_SPEC>;

impl FtdfRxMeta07Reg {
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfRxMeta07Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfRxMeta07Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfRxMeta07Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta07Reg {
        <crate::RegValueT<FtdfRxMeta07Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta10Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta10Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta10Reg = crate::RegValueT<FtdfRxMeta10Reg_SPEC>;

impl FtdfRxMeta10Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta10Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta10Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta10Reg {
        <crate::RegValueT<FtdfRxMeta10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta11Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta11Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta11Reg = crate::RegValueT<FtdfRxMeta11Reg_SPEC>;

impl FtdfRxMeta11Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta11Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta11Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta11Reg {
        <crate::RegValueT<FtdfRxMeta11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta12Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta12Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta12Reg = crate::RegValueT<FtdfRxMeta12Reg_SPEC>;

impl FtdfRxMeta12Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta12Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta12Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta12Reg {
        <crate::RegValueT<FtdfRxMeta12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta13Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta13Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta13Reg = crate::RegValueT<FtdfRxMeta13Reg_SPEC>;

impl FtdfRxMeta13Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta13Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta13Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta13Reg {
        <crate::RegValueT<FtdfRxMeta13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta14Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta14Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta14Reg = crate::RegValueT<FtdfRxMeta14Reg_SPEC>;

impl FtdfRxMeta14Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta14Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta14Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta14Reg {
        <crate::RegValueT<FtdfRxMeta14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta15Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta15Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta15Reg = crate::RegValueT<FtdfRxMeta15Reg_SPEC>;

impl FtdfRxMeta15Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta15Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta15Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta15Reg {
        <crate::RegValueT<FtdfRxMeta15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta16Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta16Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta16Reg = crate::RegValueT<FtdfRxMeta16Reg_SPEC>;

impl FtdfRxMeta16Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta16Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta16Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta16Reg {
        <crate::RegValueT<FtdfRxMeta16Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxMeta17Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRxMeta17Reg_SPEC {
    type DataType = u32;
}

pub type FtdfRxMeta17Reg = crate::RegValueT<FtdfRxMeta17Reg_SPEC>;

impl FtdfRxMeta17Reg {
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfRxMeta17Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc16_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxMeta17Reg {
    #[inline(always)]
    fn default() -> FtdfRxMeta17Reg {
        <crate::RegValueT<FtdfRxMeta17Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxStatusDeltaReg_SPEC;
impl crate::sealed::RegSpec for FtdfRxStatusDeltaReg_SPEC {
    type DataType = u32;
}

pub type FtdfRxStatusDeltaReg = crate::RegValueT<FtdfRxStatusDeltaReg_SPEC>;

impl FtdfRxStatusDeltaReg {
    #[inline(always)]
    pub fn rx_buff_is_full_d(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxStatusDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxStatusDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxStatusDeltaReg {
    #[inline(always)]
    fn default() -> FtdfRxStatusDeltaReg {
        <crate::RegValueT<FtdfRxStatusDeltaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxStatusMaskReg_SPEC;
impl crate::sealed::RegSpec for FtdfRxStatusMaskReg_SPEC {
    type DataType = u32;
}

pub type FtdfRxStatusMaskReg = crate::RegValueT<FtdfRxStatusMaskReg_SPEC>;

impl FtdfRxStatusMaskReg {
    #[inline(always)]
    pub fn rx_buff_is_full_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxStatusMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxStatusMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxStatusMaskReg {
    #[inline(always)]
    fn default() -> FtdfRxStatusMaskReg {
        <crate::RegValueT<FtdfRxStatusMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRxStatusReg_SPEC;
impl crate::sealed::RegSpec for FtdfRxStatusReg_SPEC {
    type DataType = u32;
}

pub type FtdfRxStatusReg = crate::RegValueT<FtdfRxStatusReg_SPEC>;

impl FtdfRxStatusReg {
    #[inline(always)]
    pub fn rx_write_buf_ptr(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, FtdfRxStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,FtdfRxStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_buff_is_full(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfRxStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfRxStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfRxStatusReg {
    #[inline(always)]
    fn default() -> FtdfRxStatusReg {
        <crate::RegValueT<FtdfRxStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSeckey0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSeckey0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSeckey0Reg = crate::RegValueT<FtdfSeckey0Reg_SPEC>;

impl FtdfSeckey0Reg {
    #[inline(always)]
    pub fn seckey_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSeckey0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSeckey0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSeckey0Reg {
    #[inline(always)]
    fn default() -> FtdfSeckey0Reg {
        <crate::RegValueT<FtdfSeckey0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSeckey1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSeckey1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSeckey1Reg = crate::RegValueT<FtdfSeckey1Reg_SPEC>;

impl FtdfSeckey1Reg {
    #[inline(always)]
    pub fn seckey_1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSeckey1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSeckey1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSeckey1Reg {
    #[inline(always)]
    fn default() -> FtdfSeckey1Reg {
        <crate::RegValueT<FtdfSeckey1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSeckey2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSeckey2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSeckey2Reg = crate::RegValueT<FtdfSeckey2Reg_SPEC>;

impl FtdfSeckey2Reg {
    #[inline(always)]
    pub fn seckey_2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSeckey2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSeckey2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSeckey2Reg {
    #[inline(always)]
    fn default() -> FtdfSeckey2Reg {
        <crate::RegValueT<FtdfSeckey2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSeckey3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSeckey3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSeckey3Reg = crate::RegValueT<FtdfSeckey3Reg_SPEC>;

impl FtdfSeckey3Reg {
    #[inline(always)]
    pub fn seckey_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSeckey3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSeckey3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSeckey3Reg {
    #[inline(always)]
    fn default() -> FtdfSeckey3Reg {
        <crate::RegValueT<FtdfSeckey3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecnonce0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSecnonce0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSecnonce0Reg = crate::RegValueT<FtdfSecnonce0Reg_SPEC>;

impl FtdfSecnonce0Reg {
    #[inline(always)]
    pub fn secnonce_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSecnonce0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSecnonce0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSecnonce0Reg {
    #[inline(always)]
    fn default() -> FtdfSecnonce0Reg {
        <crate::RegValueT<FtdfSecnonce0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecnonce1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSecnonce1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSecnonce1Reg = crate::RegValueT<FtdfSecnonce1Reg_SPEC>;

impl FtdfSecnonce1Reg {
    #[inline(always)]
    pub fn secnonce_1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSecnonce1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSecnonce1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSecnonce1Reg {
    #[inline(always)]
    fn default() -> FtdfSecnonce1Reg {
        <crate::RegValueT<FtdfSecnonce1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecnonce2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSecnonce2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSecnonce2Reg = crate::RegValueT<FtdfSecnonce2Reg_SPEC>;

impl FtdfSecnonce2Reg {
    #[inline(always)]
    pub fn secnonce_2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSecnonce2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSecnonce2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSecnonce2Reg {
    #[inline(always)]
    fn default() -> FtdfSecnonce2Reg {
        <crate::RegValueT<FtdfSecnonce2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecnonce3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSecnonce3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSecnonce3Reg = crate::RegValueT<FtdfSecnonce3Reg_SPEC>;

impl FtdfSecnonce3Reg {
    #[inline(always)]
    pub fn secnonce_3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, FtdfSecnonce3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfSecnonce3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSecnonce3Reg {
    #[inline(always)]
    fn default() -> FtdfSecnonce3Reg {
        <crate::RegValueT<FtdfSecnonce3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecurity0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSecurity0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSecurity0Reg = crate::RegValueT<FtdfSecurity0Reg_SPEC>;

impl FtdfSecurity0Reg {
    #[inline(always)]
    pub fn secencdecn(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfSecurity0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,FtdfSecurity0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn secmlength(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7f,
        1,
        0,
        u8,
        u8,
        FtdfSecurity0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7f,
            1,
            0,
            u8,
            u8,
            FtdfSecurity0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secalength(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7f,
        1,
        0,
        u8,
        u8,
        FtdfSecurity0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7f,
            1,
            0,
            u8,
            u8,
            FtdfSecurity0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secentry(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, FtdfSecurity0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,FtdfSecurity0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sectxrxn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfSecurity0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfSecurity0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfSecurity0Reg {
    #[inline(always)]
    fn default() -> FtdfSecurity0Reg {
        <crate::RegValueT<FtdfSecurity0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecurity1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSecurity1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSecurity1Reg = crate::RegValueT<FtdfSecurity1Reg_SPEC>;

impl FtdfSecurity1Reg {
    #[inline(always)]
    pub fn secencrflags(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, FtdfSecurity1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfSecurity1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn secauthflags(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, FtdfSecurity1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfSecurity1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSecurity1Reg {
    #[inline(always)]
    fn default() -> FtdfSecurity1Reg {
        <crate::RegValueT<FtdfSecurity1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecurityEventmaskReg_SPEC;
impl crate::sealed::RegSpec for FtdfSecurityEventmaskReg_SPEC {
    type DataType = u32;
}

pub type FtdfSecurityEventmaskReg = crate::RegValueT<FtdfSecurityEventmaskReg_SPEC>;

impl FtdfSecurityEventmaskReg {
    #[inline(always)]
    pub fn secready_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfSecurityEventmaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfSecurityEventmaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfSecurityEventmaskReg {
    #[inline(always)]
    fn default() -> FtdfSecurityEventmaskReg {
        <crate::RegValueT<FtdfSecurityEventmaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecurityEventReg_SPEC;
impl crate::sealed::RegSpec for FtdfSecurityEventReg_SPEC {
    type DataType = u32;
}

pub type FtdfSecurityEventReg = crate::RegValueT<FtdfSecurityEventReg_SPEC>;

impl FtdfSecurityEventReg {
    #[inline(always)]
    pub fn secready_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfSecurityEventReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfSecurityEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfSecurityEventReg {
    #[inline(always)]
    fn default() -> FtdfSecurityEventReg {
        <crate::RegValueT<FtdfSecurityEventReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecurityOsReg_SPEC;
impl crate::sealed::RegSpec for FtdfSecurityOsReg_SPEC {
    type DataType = u32;
}

pub type FtdfSecurityOsReg = crate::RegValueT<FtdfSecurityOsReg_SPEC>;

impl FtdfSecurityOsReg {
    #[inline(always)]
    pub fn secstart(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfSecurityOsReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfSecurityOsReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn secabort(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfSecurityOsReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfSecurityOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfSecurityOsReg {
    #[inline(always)]
    fn default() -> FtdfSecurityOsReg {
        <crate::RegValueT<FtdfSecurityOsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSecurityStatusReg_SPEC;
impl crate::sealed::RegSpec for FtdfSecurityStatusReg_SPEC {
    type DataType = u32;
}

pub type FtdfSecurityStatusReg = crate::RegValueT<FtdfSecurityStatusReg_SPEC>;

impl FtdfSecurityStatusReg {
    #[inline(always)]
    pub fn secauthfail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfSecurityStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfSecurityStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn secbusy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfSecurityStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfSecurityStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfSecurityStatusReg {
    #[inline(always)]
    fn default() -> FtdfSecurityStatusReg {
        <crate::RegValueT<FtdfSecurityStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSizeAndVal0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfSizeAndVal0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfSizeAndVal0Reg = crate::RegValueT<FtdfSizeAndVal0Reg_SPEC>;

impl FtdfSizeAndVal0Reg {
    #[inline(always)]
    pub fn short_longnot(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfSizeAndVal0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfSizeAndVal0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn valid_sa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfSizeAndVal0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfSizeAndVal0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSizeAndVal0Reg {
    #[inline(always)]
    fn default() -> FtdfSizeAndVal0Reg {
        <crate::RegValueT<FtdfSizeAndVal0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSymboltime2ThrReg_SPEC;
impl crate::sealed::RegSpec for FtdfSymboltime2ThrReg_SPEC {
    type DataType = u32;
}

pub type FtdfSymboltime2ThrReg = crate::RegValueT<FtdfSymboltime2ThrReg_SPEC>;

impl FtdfSymboltime2ThrReg {
    #[inline(always)]
    pub fn symboltime2thr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSymboltime2ThrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSymboltime2ThrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSymboltime2ThrReg {
    #[inline(always)]
    fn default() -> FtdfSymboltime2ThrReg {
        <crate::RegValueT<FtdfSymboltime2ThrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSymboltimesnapshotvalReg_SPEC;
impl crate::sealed::RegSpec for FtdfSymboltimesnapshotvalReg_SPEC {
    type DataType = u32;
}

pub type FtdfSymboltimesnapshotvalReg = crate::RegValueT<FtdfSymboltimesnapshotvalReg_SPEC>;

impl FtdfSymboltimesnapshotvalReg {
    #[inline(always)]
    pub fn symboltimesnapshotval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSymboltimesnapshotvalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSymboltimesnapshotvalReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSymboltimesnapshotvalReg {
    #[inline(always)]
    fn default() -> FtdfSymboltimesnapshotvalReg {
        <crate::RegValueT<FtdfSymboltimesnapshotvalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSymboltimethrReg_SPEC;
impl crate::sealed::RegSpec for FtdfSymboltimethrReg_SPEC {
    type DataType = u32;
}

pub type FtdfSymboltimethrReg = crate::RegValueT<FtdfSymboltimethrReg_SPEC>;

impl FtdfSymboltimethrReg {
    #[inline(always)]
    pub fn symboltimethr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSymboltimethrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSymboltimethrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSymboltimethrReg {
    #[inline(always)]
    fn default() -> FtdfSymboltimethrReg {
        <crate::RegValueT<FtdfSymboltimethrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSynctimestampphasevalReg_SPEC;
impl crate::sealed::RegSpec for FtdfSynctimestampphasevalReg_SPEC {
    type DataType = u32;
}

pub type FtdfSynctimestampphasevalReg = crate::RegValueT<FtdfSynctimestampphasevalReg_SPEC>;

impl FtdfSynctimestampphasevalReg {
    #[inline(always)]
    pub fn synctimestampphaseval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfSynctimestampphasevalReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfSynctimestampphasevalReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSynctimestampphasevalReg {
    #[inline(always)]
    fn default() -> FtdfSynctimestampphasevalReg {
        <crate::RegValueT<FtdfSynctimestampphasevalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSynctimestampthrReg_SPEC;
impl crate::sealed::RegSpec for FtdfSynctimestampthrReg_SPEC {
    type DataType = u32;
}

pub type FtdfSynctimestampthrReg = crate::RegValueT<FtdfSynctimestampthrReg_SPEC>;

impl FtdfSynctimestampthrReg {
    #[inline(always)]
    pub fn synctimestampthr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ffffff,
        1,
        0,
        u32,
        u32,
        FtdfSynctimestampthrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ffffff,
            1,
            0,
            u32,
            u32,
            FtdfSynctimestampthrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSynctimestampthrReg {
    #[inline(always)]
    fn default() -> FtdfSynctimestampthrReg {
        <crate::RegValueT<FtdfSynctimestampthrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfSynctimestampvalReg_SPEC;
impl crate::sealed::RegSpec for FtdfSynctimestampvalReg_SPEC {
    type DataType = u32;
}

pub type FtdfSynctimestampvalReg = crate::RegValueT<FtdfSynctimestampvalReg_SPEC>;

impl FtdfSynctimestampvalReg {
    #[inline(always)]
    pub fn synctimestampval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfSynctimestampvalReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfSynctimestampvalReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfSynctimestampvalReg {
    #[inline(always)]
    fn default() -> FtdfSynctimestampvalReg {
        <crate::RegValueT<FtdfSynctimestampvalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTimerControl1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTimerControl1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTimerControl1Reg = crate::RegValueT<FtdfTimerControl1Reg_SPEC>;

impl FtdfTimerControl1Reg {
    #[inline(always)]
    pub fn synctimestampena(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTimerControl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTimerControl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTimerControl1Reg {
    #[inline(always)]
    fn default() -> FtdfTimerControl1Reg {
        <crate::RegValueT<FtdfTimerControl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTimestampcurrphasevalReg_SPEC;
impl crate::sealed::RegSpec for FtdfTimestampcurrphasevalReg_SPEC {
    type DataType = u32;
}

pub type FtdfTimestampcurrphasevalReg = crate::RegValueT<FtdfTimestampcurrphasevalReg_SPEC>;

impl FtdfTimestampcurrphasevalReg {
    #[inline(always)]
    pub fn timestampcurrphaseval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfTimestampcurrphasevalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfTimestampcurrphasevalReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTimestampcurrphasevalReg {
    #[inline(always)]
    fn default() -> FtdfTimestampcurrphasevalReg {
        <crate::RegValueT<FtdfTimestampcurrphasevalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTimestampcurrvalReg_SPEC;
impl crate::sealed::RegSpec for FtdfTimestampcurrvalReg_SPEC {
    type DataType = u32;
}

pub type FtdfTimestampcurrvalReg = crate::RegValueT<FtdfTimestampcurrvalReg_SPEC>;

impl FtdfTimestampcurrvalReg {
    #[inline(always)]
    pub fn timestampcurrval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTimestampcurrvalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTimestampcurrvalReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTimestampcurrvalReg {
    #[inline(always)]
    fn default() -> FtdfTimestampcurrvalReg {
        <crate::RegValueT<FtdfTimestampcurrvalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTschControl0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTschControl0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTschControl0Reg = crate::RegValueT<FtdfTschControl0Reg_SPEC>;

impl FtdfTschControl0Reg {
    #[inline(always)]
    pub fn mactsrxwait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfTschControl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfTschControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mactstxackdelay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfTschControl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfTschControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTschControl0Reg {
    #[inline(always)]
    fn default() -> FtdfTschControl0Reg {
        <crate::RegValueT<FtdfTschControl0Reg_SPEC> as RegisterValue<_>>::new(144180200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTschControl1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTschControl1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTschControl1Reg = crate::RegValueT<FtdfTschControl1Reg_SPEC>;

impl FtdfTschControl1Reg {
    #[inline(always)]
    pub fn mactsrxtx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfTschControl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfTschControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTschControl1Reg {
    #[inline(always)]
    fn default() -> FtdfTschControl1Reg {
        <crate::RegValueT<FtdfTschControl1Reg_SPEC> as RegisterValue<_>>::new(192)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTschControl2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTschControl2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTschControl2Reg = crate::RegValueT<FtdfTschControl2Reg_SPEC>;

impl FtdfTschControl2Reg {
    #[inline(always)]
    pub fn mactsackwait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfTschControl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfTschControl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mactsrxackdelay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        FtdfTschControl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            FtdfTschControl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTschControl2Reg {
    #[inline(always)]
    fn default() -> FtdfTschControl2Reg {
        <crate::RegValueT<FtdfTschControl2Reg_SPEC> as RegisterValue<_>>::new(26215200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxbyteEReg_SPEC;
impl crate::sealed::RegSpec for FtdfTxbyteEReg_SPEC {
    type DataType = u32;
}

pub type FtdfTxbyteEReg = crate::RegValueT<FtdfTxbyteEReg_SPEC>;

impl FtdfTxbyteEReg {
    #[inline(always)]
    pub fn tx_last_symbol_e(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxbyteEReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxbyteEReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txbyte_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxbyteEReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxbyteEReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxbyteEReg {
    #[inline(always)]
    fn default() -> FtdfTxbyteEReg {
        <crate::RegValueT<FtdfTxbyteEReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxbyteMReg_SPEC;
impl crate::sealed::RegSpec for FtdfTxbyteMReg_SPEC {
    type DataType = u32;
}

pub type FtdfTxbyteMReg = crate::RegValueT<FtdfTxbyteMReg_SPEC>;

impl FtdfTxbyteMReg {
    #[inline(always)]
    pub fn tx_last_symbol_m(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxbyteMReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxbyteMReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn txbyte_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxbyteMReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxbyteMReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxbyteMReg {
    #[inline(always)]
    fn default() -> FtdfTxbyteMReg {
        <crate::RegValueT<FtdfTxbyteMReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxpipepropdelayReg_SPEC;
impl crate::sealed::RegSpec for FtdfTxpipepropdelayReg_SPEC {
    type DataType = u32;
}

pub type FtdfTxpipepropdelayReg = crate::RegValueT<FtdfTxpipepropdelayReg_SPEC>;

impl FtdfTxpipepropdelayReg {
    #[inline(always)]
    pub fn txpipepropdelay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfTxpipepropdelayReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfTxpipepropdelayReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxpipepropdelayReg {
    #[inline(always)]
    fn default() -> FtdfTxpipepropdelayReg {
        <crate::RegValueT<FtdfTxpipepropdelayReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxClearOsReg_SPEC;
impl crate::sealed::RegSpec for FtdfTxClearOsReg_SPEC {
    type DataType = u32;
}

pub type FtdfTxClearOsReg = crate::RegValueT<FtdfTxClearOsReg_SPEC>;

impl FtdfTxClearOsReg {
    #[inline(always)]
    pub fn tx_flag_clear(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, FtdfTxClearOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,FtdfTxClearOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxClearOsReg {
    #[inline(always)]
    fn default() -> FtdfTxClearOsReg {
        <crate::RegValueT<FtdfTxClearOsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxControl0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxControl0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxControl0Reg = crate::RegValueT<FtdfTxControl0Reg_SPEC>;

impl FtdfTxControl0Reg {
    #[inline(always)]
    pub fn macmaxcsmabackoffs(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxControl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macminbe(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, FtdfTxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn macmaxbe(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, FtdfTxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dbgtxtransparentmode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxControl0Reg {
    #[inline(always)]
    fn default() -> FtdfTxControl0Reg {
        <crate::RegValueT<FtdfTxControl0Reg_SPEC> as RegisterValue<_>>::new(17232)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFifo00Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFifo00Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFifo00Reg = crate::RegValueT<FtdfTxFifo00Reg_SPEC>;

impl FtdfTxFifo00Reg {
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxFifo00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxFifo00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxFifo00Reg {
    #[inline(always)]
    fn default() -> FtdfTxFifo00Reg {
        <crate::RegValueT<FtdfTxFifo00Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFifo10Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFifo10Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFifo10Reg = crate::RegValueT<FtdfTxFifo10Reg_SPEC>;

impl FtdfTxFifo10Reg {
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxFifo10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxFifo10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxFifo10Reg {
    #[inline(always)]
    fn default() -> FtdfTxFifo10Reg {
        <crate::RegValueT<FtdfTxFifo10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFifo20Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFifo20Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFifo20Reg = crate::RegValueT<FtdfTxFifo20Reg_SPEC>;

impl FtdfTxFifo20Reg {
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxFifo20Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxFifo20Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxFifo20Reg {
    #[inline(always)]
    fn default() -> FtdfTxFifo20Reg {
        <crate::RegValueT<FtdfTxFifo20Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFifo30Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFifo30Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFifo30Reg = crate::RegValueT<FtdfTxFifo30Reg_SPEC>;

impl FtdfTxFifo30Reg {
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxFifo30Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxFifo30Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxFifo30Reg {
    #[inline(always)]
    fn default() -> FtdfTxFifo30Reg {
        <crate::RegValueT<FtdfTxFifo30Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearE0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearE0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearE0Reg = crate::RegValueT<FtdfTxFlagClearE0Reg_SPEC>;

impl FtdfTxFlagClearE0Reg {
    #[inline(always)]
    pub fn tx_flag_clear_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearE0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearE0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearE0Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearE0Reg {
        <crate::RegValueT<FtdfTxFlagClearE0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearE1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearE1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearE1Reg = crate::RegValueT<FtdfTxFlagClearE1Reg_SPEC>;

impl FtdfTxFlagClearE1Reg {
    #[inline(always)]
    pub fn tx_flag_clear_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearE1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearE1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearE1Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearE1Reg {
        <crate::RegValueT<FtdfTxFlagClearE1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearE2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearE2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearE2Reg = crate::RegValueT<FtdfTxFlagClearE2Reg_SPEC>;

impl FtdfTxFlagClearE2Reg {
    #[inline(always)]
    pub fn tx_flag_clear_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearE2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearE2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearE2Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearE2Reg {
        <crate::RegValueT<FtdfTxFlagClearE2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearE3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearE3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearE3Reg = crate::RegValueT<FtdfTxFlagClearE3Reg_SPEC>;

impl FtdfTxFlagClearE3Reg {
    #[inline(always)]
    pub fn tx_flag_clear_e(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearE3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearE3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearE3Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearE3Reg {
        <crate::RegValueT<FtdfTxFlagClearE3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearM0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearM0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearM0Reg = crate::RegValueT<FtdfTxFlagClearM0Reg_SPEC>;

impl FtdfTxFlagClearM0Reg {
    #[inline(always)]
    pub fn tx_flag_clear_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearM0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearM0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearM0Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearM0Reg {
        <crate::RegValueT<FtdfTxFlagClearM0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearM1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearM1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearM1Reg = crate::RegValueT<FtdfTxFlagClearM1Reg_SPEC>;

impl FtdfTxFlagClearM1Reg {
    #[inline(always)]
    pub fn tx_flag_clear_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearM1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearM1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearM1Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearM1Reg {
        <crate::RegValueT<FtdfTxFlagClearM1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearM2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearM2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearM2Reg = crate::RegValueT<FtdfTxFlagClearM2Reg_SPEC>;

impl FtdfTxFlagClearM2Reg {
    #[inline(always)]
    pub fn tx_flag_clear_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearM2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearM2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearM2Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearM2Reg {
        <crate::RegValueT<FtdfTxFlagClearM2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagClearM3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagClearM3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagClearM3Reg = crate::RegValueT<FtdfTxFlagClearM3Reg_SPEC>;

impl FtdfTxFlagClearM3Reg {
    #[inline(always)]
    pub fn tx_flag_clear_m(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagClearM3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagClearM3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagClearM3Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagClearM3Reg {
        <crate::RegValueT<FtdfTxFlagClearM3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagS0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagS0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagS0Reg = crate::RegValueT<FtdfTxFlagS0Reg_SPEC>;

impl FtdfTxFlagS0Reg {
    #[inline(always)]
    pub fn tx_flag_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagS0Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagS0Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagS0Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagS0Reg {
        <crate::RegValueT<FtdfTxFlagS0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagS1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagS1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagS1Reg = crate::RegValueT<FtdfTxFlagS1Reg_SPEC>;

impl FtdfTxFlagS1Reg {
    #[inline(always)]
    pub fn tx_flag_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagS1Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagS1Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagS1Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagS1Reg {
        <crate::RegValueT<FtdfTxFlagS1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagS2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagS2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagS2Reg = crate::RegValueT<FtdfTxFlagS2Reg_SPEC>;

impl FtdfTxFlagS2Reg {
    #[inline(always)]
    pub fn tx_flag_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagS2Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagS2Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagS2Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagS2Reg {
        <crate::RegValueT<FtdfTxFlagS2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxFlagS3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxFlagS3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxFlagS3Reg = crate::RegValueT<FtdfTxFlagS3Reg_SPEC>;

impl FtdfTxFlagS3Reg {
    #[inline(always)]
    pub fn tx_flag_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxFlagS3Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxFlagS3Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxFlagS3Reg {
    #[inline(always)]
    fn default() -> FtdfTxFlagS3Reg {
        <crate::RegValueT<FtdfTxFlagS3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData00Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData00Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData00Reg = crate::RegValueT<FtdfTxMetaData00Reg_SPEC>;

impl FtdfTxMetaData00Reg {
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<
        15,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<
        11,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<
        7,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData00Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData00Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData00Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData00Reg {
        <crate::RegValueT<FtdfTxMetaData00Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData01Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData01Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData01Reg = crate::RegValueT<FtdfTxMetaData01Reg_SPEC>;

impl FtdfTxMetaData01Reg {
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData01Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData01Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData01Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData01Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<
        15,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData01Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData01Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<
        11,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData01Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData01Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<
        7,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData01Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData01Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData01Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData01Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData01Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData01Reg {
        <crate::RegValueT<FtdfTxMetaData01Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData02Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData02Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData02Reg = crate::RegValueT<FtdfTxMetaData02Reg_SPEC>;

impl FtdfTxMetaData02Reg {
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData02Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData02Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData02Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData02Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<
        15,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData02Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData02Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<
        11,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData02Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData02Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<
        7,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData02Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData02Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData02Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData02Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData02Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData02Reg {
        <crate::RegValueT<FtdfTxMetaData02Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData03Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData03Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData03Reg = crate::RegValueT<FtdfTxMetaData03Reg_SPEC>;

impl FtdfTxMetaData03Reg {
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<
        15,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<
        11,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<
        7,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData03Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData03Reg {
        <crate::RegValueT<FtdfTxMetaData03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData10Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData10Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData10Reg = crate::RegValueT<FtdfTxMetaData10Reg_SPEC>;

impl FtdfTxMetaData10Reg {
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData10Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData10Reg {
        <crate::RegValueT<FtdfTxMetaData10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData11Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData11Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData11Reg = crate::RegValueT<FtdfTxMetaData11Reg_SPEC>;

impl FtdfTxMetaData11Reg {
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData11Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData11Reg {
        <crate::RegValueT<FtdfTxMetaData11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData12Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData12Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData12Reg = crate::RegValueT<FtdfTxMetaData12Reg_SPEC>;

impl FtdfTxMetaData12Reg {
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData12Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData12Reg {
        <crate::RegValueT<FtdfTxMetaData12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxMetaData13Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxMetaData13Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxMetaData13Reg = crate::RegValueT<FtdfTxMetaData13Reg_SPEC>;

impl FtdfTxMetaData13Reg {
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        FtdfTxMetaData13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            FtdfTxMetaData13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxMetaData13Reg {
    #[inline(always)]
    fn default() -> FtdfTxMetaData13Reg {
        <crate::RegValueT<FtdfTxMetaData13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxPriority0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxPriority0Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxPriority0Reg = crate::RegValueT<FtdfTxPriority0Reg_SPEC>;

impl FtdfTxPriority0Reg {
    #[inline(always)]
    pub fn pti_tx(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxPriority0Reg {
    #[inline(always)]
    fn default() -> FtdfTxPriority0Reg {
        <crate::RegValueT<FtdfTxPriority0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxPriority1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxPriority1Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxPriority1Reg = crate::RegValueT<FtdfTxPriority1Reg_SPEC>;

impl FtdfTxPriority1Reg {
    #[inline(always)]
    pub fn pti_tx(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxPriority1Reg {
    #[inline(always)]
    fn default() -> FtdfTxPriority1Reg {
        <crate::RegValueT<FtdfTxPriority1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxPriority2Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxPriority2Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxPriority2Reg = crate::RegValueT<FtdfTxPriority2Reg_SPEC>;

impl FtdfTxPriority2Reg {
    #[inline(always)]
    pub fn pti_tx(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxPriority2Reg {
    #[inline(always)]
    fn default() -> FtdfTxPriority2Reg {
        <crate::RegValueT<FtdfTxPriority2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxPriority3Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxPriority3Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxPriority3Reg = crate::RegValueT<FtdfTxPriority3Reg_SPEC>;

impl FtdfTxPriority3Reg {
    #[inline(always)]
    pub fn pti_tx(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        FtdfTxPriority3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            FtdfTxPriority3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxPriority3Reg {
    #[inline(always)]
    fn default() -> FtdfTxPriority3Reg {
        <crate::RegValueT<FtdfTxPriority3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus00Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus00Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus00Reg = crate::RegValueT<FtdfTxReturnStatus00Reg_SPEC>;

impl FtdfTxReturnStatus00Reg {
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxReturnStatus00Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxReturnStatus00Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus00Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus00Reg {
        <crate::RegValueT<FtdfTxReturnStatus00Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus01Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus01Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus01Reg = crate::RegValueT<FtdfTxReturnStatus01Reg_SPEC>;

impl FtdfTxReturnStatus01Reg {
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxReturnStatus01Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxReturnStatus01Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus01Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus01Reg {
        <crate::RegValueT<FtdfTxReturnStatus01Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus02Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus02Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus02Reg = crate::RegValueT<FtdfTxReturnStatus02Reg_SPEC>;

impl FtdfTxReturnStatus02Reg {
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxReturnStatus02Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxReturnStatus02Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus02Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus02Reg {
        <crate::RegValueT<FtdfTxReturnStatus02Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus03Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus03Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus03Reg = crate::RegValueT<FtdfTxReturnStatus03Reg_SPEC>;

impl FtdfTxReturnStatus03Reg {
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        FtdfTxReturnStatus03Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            FtdfTxReturnStatus03Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus03Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus03Reg {
        <crate::RegValueT<FtdfTxReturnStatus03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus10Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus10Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus10Reg = crate::RegValueT<FtdfTxReturnStatus10Reg_SPEC>;

impl FtdfTxReturnStatus10Reg {
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxReturnStatus10Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxReturnStatus10Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus10Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus10Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackfail(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxReturnStatus10Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxReturnStatus10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus10Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus10Reg {
        <crate::RegValueT<FtdfTxReturnStatus10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus11Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus11Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus11Reg = crate::RegValueT<FtdfTxReturnStatus11Reg_SPEC>;

impl FtdfTxReturnStatus11Reg {
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxReturnStatus11Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxReturnStatus11Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus11Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus11Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackfail(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxReturnStatus11Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxReturnStatus11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus11Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus11Reg {
        <crate::RegValueT<FtdfTxReturnStatus11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus12Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus12Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus12Reg = crate::RegValueT<FtdfTxReturnStatus12Reg_SPEC>;

impl FtdfTxReturnStatus12Reg {
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxReturnStatus12Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxReturnStatus12Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus12Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus12Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackfail(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxReturnStatus12Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxReturnStatus12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus12Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus12Reg {
        <crate::RegValueT<FtdfTxReturnStatus12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxReturnStatus13Reg_SPEC;
impl crate::sealed::RegSpec for FtdfTxReturnStatus13Reg_SPEC {
    type DataType = u32;
}

pub type FtdfTxReturnStatus13Reg = crate::RegValueT<FtdfTxReturnStatus13Reg_SPEC>;

impl FtdfTxReturnStatus13Reg {
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
        u8,
        u8,
        FtdfTxReturnStatus13Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x7,
            1,
            0,
            u8,
            u8,
            FtdfTxReturnStatus13Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus13Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus13Reg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ackfail(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfTxReturnStatus13Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfTxReturnStatus13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxReturnStatus13Reg {
    #[inline(always)]
    fn default() -> FtdfTxReturnStatus13Reg {
        <crate::RegValueT<FtdfTxReturnStatus13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfTxSetOsReg_SPEC;
impl crate::sealed::RegSpec for FtdfTxSetOsReg_SPEC {
    type DataType = u32;
}

pub type FtdfTxSetOsReg = crate::RegValueT<FtdfTxSetOsReg_SPEC>;

impl FtdfTxSetOsReg {
    #[inline(always)]
    pub fn tx_flag_set(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, FtdfTxSetOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,FtdfTxSetOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfTxSetOsReg {
    #[inline(always)]
    fn default() -> FtdfTxSetOsReg {
        <crate::RegValueT<FtdfTxSetOsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfWakeupControlOsReg_SPEC;
impl crate::sealed::RegSpec for FtdfWakeupControlOsReg_SPEC {
    type DataType = u32;
}

pub type FtdfWakeupControlOsReg = crate::RegValueT<FtdfWakeupControlOsReg_SPEC>;

impl FtdfWakeupControlOsReg {
    #[inline(always)]
    pub fn wakeuptimerenable_clear(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfWakeupControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfWakeupControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeuptimerenable_set(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfWakeupControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfWakeupControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfWakeupControlOsReg {
    #[inline(always)]
    fn default() -> FtdfWakeupControlOsReg {
        <crate::RegValueT<FtdfWakeupControlOsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfWakeupControlReg_SPEC;
impl crate::sealed::RegSpec for FtdfWakeupControlReg_SPEC {
    type DataType = u32;
}

pub type FtdfWakeupControlReg = crate::RegValueT<FtdfWakeupControlReg_SPEC>;

impl FtdfWakeupControlReg {
    #[inline(always)]
    pub fn wakeup_mode(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        u8,
        u8,
        FtdfWakeupControlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            u8,
            u8,
            FtdfWakeupControlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wakeupenable(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, FtdfWakeupControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<29,1,0,FtdfWakeupControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeupintthr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ffffff,
        1,
        0,
        u32,
        u32,
        FtdfWakeupControlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ffffff,
            1,
            0,
            u32,
            u32,
            FtdfWakeupControlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfWakeupControlReg {
    #[inline(always)]
    fn default() -> FtdfWakeupControlReg {
        <crate::RegValueT<FtdfWakeupControlReg_SPEC> as RegisterValue<_>>::new(1073741824)
    }
}
