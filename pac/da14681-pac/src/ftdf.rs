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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:13 +0000

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
    #[doc = "Build time"]
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

    #[doc = "Build time"]
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

    #[doc = "Build time"]
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

    #[doc = "Build time"]
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

    #[doc = "Debug control register"]
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

    #[doc = "Value of event generator"]
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

    #[doc = "Selection register events"]
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

    #[doc = "Mask selection register events"]
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

    #[doc = "Global control register"]
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

    #[doc = "Global control register"]
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

    #[doc = "Global control register"]
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

    #[doc = "Global control register"]
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

    #[doc = "Lmax reset register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac delta control register"]
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

    #[doc = "Lmac mask control register"]
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

    #[doc = "Lmac control register"]
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

    #[doc = "Lmac status register"]
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

    #[doc = "Lmac event regsiter"]
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

    #[doc = "Lmax manual PHY register"]
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

    #[doc = "One shot register triggers transmission in manual mode"]
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

    #[doc = "Lmac status register in manual mode"]
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

    #[doc = "Lmac mask register"]
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

    #[doc = "Maximum time to wait for a ACK"]
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

    #[doc = "Maximum time to wait for an enhanced ACK frame"]
    #[inline(always)]
    pub const fn ftdf_macenhackwaitduration_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfMacenhackwaitdurationReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfMacenhackwaitdurationReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(65596usize))
        }
    }

    #[doc = "Lmac FCS error register"]
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

    #[doc = "Discarded frames register"]
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

    #[doc = "Received acknowledgment frames"]
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

    #[doc = "Unsupported frames register"]
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

    #[doc = "Time left until next ACK is sent (us)"]
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

    #[doc = "Transmitted acknowledgment frames"]
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

    #[doc = "Lmac PHY parameter register"]
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

    #[doc = "Lmac PHY parameter register"]
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

    #[doc = "Lmac PHY parameter register"]
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

    #[doc = "Lmac PHY parameter register"]
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

    #[doc = "Name of the release"]
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

    #[doc = "Name of the release"]
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

    #[doc = "Name of the release"]
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

    #[doc = "Name of the release"]
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

    #[doc = "Receive control register"]
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

    #[doc = "Receive event register"]
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

    #[doc = "Address receive fifo 0"]
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

    #[doc = "Address transmit fifo 1"]
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

    #[doc = "Address transmit fifo 2"]
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

    #[doc = "Address transmit fifo 3"]
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

    #[doc = "Address transmit fifo 4"]
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

    #[doc = "Address transmit fifo 5"]
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

    #[doc = "Address transmit fifo 6"]
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

    #[doc = "Address transmit fifo 7"]
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

    #[doc = "Receive event mask register"]
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

    #[doc = "Receive metadata register 0"]
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

    #[doc = "Receive metadata register 1"]
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

    #[doc = "Receive metadata register 2"]
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

    #[doc = "Receive metadata register 3"]
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

    #[doc = "Receive metadata register 4"]
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

    #[doc = "Receive metadata register 5"]
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

    #[doc = "Receive metadata register 6"]
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

    #[doc = "Receive metadata register 7"]
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

    #[doc = "Receive metadata register 0"]
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

    #[doc = "Receive metadata register 1"]
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

    #[doc = "Receive metadata register 2"]
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

    #[doc = "Receive metadata register 3"]
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

    #[doc = "Receive metadata register 4"]
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

    #[doc = "Receive metadata register 5"]
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

    #[doc = "Receive metadata register 6"]
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

    #[doc = "Receive metadata register 7"]
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

    #[doc = "Receive status delta register"]
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

    #[doc = "Receive status delta mask register"]
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

    #[doc = "Receive status register"]
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

    #[doc = "Seckey register"]
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

    #[doc = "Seckey register"]
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

    #[doc = "SecKey register"]
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

    #[doc = "Seckey register"]
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

    #[doc = "Nonce register used for encryption/decryption"]
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

    #[doc = "Nonce register used for encryption/decryption"]
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

    #[doc = "Nonce register used for encryption/decryption"]
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

    #[doc = "Nonce register used for encryption/decryption"]
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

    #[doc = "Security register"]
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

    #[doc = "Security register"]
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

    #[doc = "security event mask register"]
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

    #[doc = "security event register"]
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

    #[doc = "One shot register to start encryption/decryption"]
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

    #[doc = "Security status register"]
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

    #[doc = "Symboltime threshold register 2"]
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

    #[doc = "Value timestamp generator"]
    #[inline(always)]
    pub const fn ftdf_symboltimesnapshotval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSymboltimesnapshotvalReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfSymboltimesnapshotvalReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(66064usize))
        }
    }

    #[doc = "Symboltime threshold register 1"]
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

    #[doc = "Timestamp phase value regsiter"]
    #[inline(always)]
    pub const fn ftdf_synctimestampphaseval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfSynctimestampphasevalReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfSynctimestampphasevalReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(66336usize))
        }
    }

    #[doc = "Threshold timestamp generator"]
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

    #[doc = "Value timestamp generator"]
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

    #[doc = "Timer control register"]
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

    #[doc = "Value of timestamp generator phase within a symbol"]
    #[inline(always)]
    pub const fn ftdf_timestampcurrphaseval_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfTimestampcurrphasevalReg_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<self::FtdfTimestampcurrphasevalReg_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(65652usize))
        }
    }

    #[doc = "Value of timestamp generator"]
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

    #[doc = "Lmac tsch control register"]
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

    #[doc = "Lmac tsch control register"]
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

    #[doc = "Lmac tsch control register"]
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

    #[doc = "Transmit first byte register"]
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

    #[doc = "Transmit first byte mask register"]
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

    #[doc = "Prop delay transmit register"]
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

    #[doc = "One shot register to clear flag"]
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

    #[doc = "Transmit control register"]
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

    #[doc = "Address transmit fifo 0"]
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

    #[doc = "Address transmit fifo 1"]
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

    #[doc = "Address transmit fifo 2"]
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

    #[doc = "Address transmit fifo 3"]
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

    #[doc = "Clear flag register 0"]
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

    #[doc = "Clear flag register 1"]
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

    #[doc = "Clear flag register 2"]
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

    #[doc = "Clear flag register 3"]
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

    #[doc = "Mask flag register 0"]
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

    #[doc = "Mask flag register 1"]
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

    #[doc = "Clear flag register 2"]
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

    #[doc = "Clear flag register 3"]
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

    #[doc = "Transmit packet ready for transmission register 0"]
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

    #[doc = "Transmit packet ready for transmission register 1"]
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

    #[doc = "Transmit packet ready for transmission register 2"]
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

    #[doc = "Transmit packet ready for transmission register 3"]
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

    #[doc = "Transmit metadata register 0"]
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

    #[doc = "Transmit metadata register 1"]
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

    #[doc = "Transmit metadata register 2"]
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

    #[doc = "Transmit metadata register 3"]
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

    #[doc = "Transmit metadata register 0"]
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

    #[doc = "Transmit metadata register 1"]
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

    #[doc = "Transmit metadata register 2"]
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

    #[doc = "Transmit metadata register 3"]
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

    #[doc = "Transmit priority register 0"]
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

    #[doc = "Transmit priority register 1"]
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

    #[doc = "Transmit priority register 2"]
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

    #[doc = "Transmit priority register 3"]
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

    #[doc = "Transmit status register 0"]
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

    #[doc = "Transmit status register 1"]
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

    #[doc = "Transmit status register 2"]
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

    #[doc = "Transmit status register 3"]
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

    #[doc = "Transmit status register 0"]
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

    #[doc = "Transmit status register 1"]
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

    #[doc = "Transmit status register 2"]
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

    #[doc = "Transmit status register 3"]
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

    #[doc = "One shot register to set flag"]
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

    #[doc = "Treshold value Wakeup timer"]
    #[inline(always)]
    pub const fn ftdf_wakeupintthr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfWakeupintthrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfWakeupintthrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(69632usize),
            )
        }
    }

    #[doc = "Wakeup timer vcontrol register"]
    #[inline(always)]
    pub const fn ftdf_wakeup_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::FtdfWakeupControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FtdfWakeupControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(69636usize),
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
#[doc = "Build time"]
pub type FtdfBuildtime0Reg = crate::RegValueT<FtdfBuildtime0Reg_SPEC>;

impl FtdfBuildtime0Reg {
    #[doc = "Build time of device"]
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Build time"]
pub type FtdfBuildtime1Reg = crate::RegValueT<FtdfBuildtime1Reg_SPEC>;

impl FtdfBuildtime1Reg {
    #[doc = "Build time of device"]
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Build time"]
pub type FtdfBuildtime2Reg = crate::RegValueT<FtdfBuildtime2Reg_SPEC>;

impl FtdfBuildtime2Reg {
    #[doc = "Build time of device"]
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Build time"]
pub type FtdfBuildtime3Reg = crate::RegValueT<FtdfBuildtime3Reg_SPEC>;

impl FtdfBuildtime3Reg {
    #[doc = "Build time of device"]
    #[inline(always)]
    pub fn buildtime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Debug control register"]
pub type FtdfDebugcontrolReg = crate::RegValueT<FtdfDebugcontrolReg_SPEC>;

impl FtdfDebugcontrolReg {
    #[doc = "If set, the Rx debug interface will be selected as input for the Rx pipeline."]
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
#[doc = "Value of event generator"]
pub type FtdfEventcurrvalReg = crate::RegValueT<FtdfEventcurrvalReg_SPEC>;

impl FtdfEventcurrvalReg {
    #[doc = "Value of captured Event generator"]
    #[inline(always)]
    pub fn eventcurrval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        FtdfEventcurrvalReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
#[doc = "Selection register events"]
pub type FtdfFtdfCeReg = crate::RegValueT<FtdfFtdfCeReg_SPEC>;

impl FtdfFtdfCeReg {
    #[doc = "Composite serveice request from ftdf macro (see FR0400 in v40.100.2.41.pdf)\nBit 0 = unused\nBit 1 = rx interrupts\nBit 2 = unused\nBit 3 = miscelaneous interrupts\nBit 4 = tx interrupts\nBit 5 = Reserved"]
    #[inline(always)]
    pub fn ftdf_ce(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, FtdfFtdfCeReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, FtdfFtdfCeReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Mask selection register events"]
pub type FtdfFtdfCmReg = crate::RegValueT<FtdfFtdfCmReg_SPEC>;

impl FtdfFtdfCmReg {
    #[doc = "mask bits for ftf_ce"]
    #[inline(always)]
    pub fn ftdf_cm(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, FtdfFtdfCmReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, FtdfFtdfCmReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Global control register"]
pub type FtdfGlobControl0Reg = crate::RegValueT<FtdfGlobControl0Reg_SPEC>;

impl FtdfGlobControl0Reg {
    #[doc = "If set, TSCH mode is enabled"]
    #[inline(always)]
    pub fn mactschenabled(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<18,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, Low Energy mode is enabled"]
    #[inline(always)]
    pub fn macleenabled(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<17,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Simple address of the PAN coordinator"]
    #[inline(always)]
    pub fn macsimpleaddress(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Source of the TX_DMA_REQ output of this block."]
    #[inline(always)]
    pub fn tx_dma_req(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Source of the RX_DMA_REQ output of this block."]
    #[inline(always)]
    pub fn rx_dma_req(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfGlobControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfGlobControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable/disable receiver check on address\nfields (0=enabled, 1=disabled)"]
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
#[doc = "Global control register"]
pub type FtdfGlobControl1Reg = crate::RegValueT<FtdfGlobControl1Reg_SPEC>;

impl FtdfGlobControl1Reg {
    #[doc = "The values 0xFFFF and 0xFFFE indicate\nthat no IEEE Short Address is available.\nThe latter one is used if the device i"]
    #[inline(always)]
    pub fn macshortaddress(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            FtdfGlobControl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "The values 0xFFFF indicates that the device\nis not associated"]
    #[inline(always)]
    pub fn macpanid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Global control register"]
pub type FtdfGlobControl2Reg = crate::RegValueT<FtdfGlobControl2Reg_SPEC>;

impl FtdfGlobControl2Reg {
    #[doc = "Unique device address, lower 32 bit"]
    #[inline(always)]
    pub fn aextendedaddress_l(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Global control register"]
pub type FtdfGlobControl3Reg = crate::RegValueT<FtdfGlobControl3Reg_SPEC>;

impl FtdfGlobControl3Reg {
    #[doc = "Unique device address, higher 16 bit"]
    #[inline(always)]
    pub fn aextendedaddress_h(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Lmax reset register"]
pub type FtdfLmacresetReg = crate::RegValueT<FtdfLmacresetReg_SPEC>;

impl FtdfLmacresetReg {
    #[doc = "If set, the LMAC performance and traffic counters will be reset.\nUse this register for functionally reset these counters."]
    #[inline(always)]
    pub fn lmacglobreset_count(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_count: A \'1\' Resets LMAC timing control block (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_timctrl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_count: A \'1\' Resets LMAC mac counters (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_count(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_sec: A \'1\' Resets LMAC security (for debug and MLME-reset)\n\n#$LmacReset_wutim@on_off_regmap\n#LmacReset_wutim: A \'1\' Resets LMAC wake-up timer (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_sec(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_tstim: A \'1\' Resets LMAC timestamp timer (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_tstim(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_oreg: A \'1\' Resets LMAC on_off regmap (for debug and MLME-reset)\n\n#$LmacReset_areg@on_off_regmap\n#LmacReset_areg: A \'1\' Resets LMAC always_on regmap (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_oreg(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_ahb: A \'1\' Resets LMAC ahb interface (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_ahb(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_tx: A \'1\' Resets LMAC tx pipeline (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_rx: A \'1\' Resets LMAC rx pipeline (for debug and MLME-reset)"]
    #[inline(always)]
    pub fn lmacreset_rx(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacresetReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacresetReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LmacReset_control: A \'1\' Resets LMAC Controller (for debug and MLME-reset)"]
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl0Reg = crate::RegValueT<FtdfLmacControl0Reg_SPEC>;

impl FtdfLmacControl0Reg {
    #[doc = "When the transmit or receive action is ready (LmacReady4Sleep will is set), the phy_en signal is cleared unless the control register keep_phy_en is set.\nWhen the control register keep_phy_en is set, the signal phy_en shall remain being set until the keep_phy_en is cleared."]
    #[inline(always)]
    pub fn keep_phy_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Info to arbiter if phy_en is set"]
    #[inline(always)]
    pub fn pti(
        self,
    ) -> crate::common::RegisterField<27, 0xf, 1, 0, u8, FtdfLmacControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0xf,1,0,u8, FtdfLmacControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the receiver shall be always on if\nRxEnable is set"]
    #[inline(always)]
    pub fn rxalwayson(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, FtdfLmacControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<25,1,0,FtdfLmacControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time the Rx must be on"]
    #[inline(always)]
    pub fn rxonduration(
        self,
    ) -> crate::common::RegisterField<
        1,
        0xffffff,
        1,
        0,
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl10Reg = crate::RegValueT<FtdfLmacControl10Reg_SPEC>;

impl FtdfLmacControl10Reg {
    #[doc = "If the current RZtime is less or Equal to macRZzeroVal an RZtime with value zero is inserted in the wakeup frame"]
    #[inline(always)]
    pub fn macrzzeroval(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, FtdfLmacControl10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            u8,
            FtdfLmacControl10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "The UMAC can set the margin for the expected frame by control register macCSLmarginRZ (in 10 sym).\nSo the LMAC will make sure the receiver is ready to receive data this amount of time earlier than to be expected by the received RZ time"]
    #[inline(always)]
    pub fn maccslmarginrz(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, FtdfLmacControl10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            FtdfLmacControl10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This register shall be used if the Wake-up frame to be transmitted is larger than 15 octets.\nIt shall indicate the amount of extra data in a Wake-up frame after the RZ position in the frame (in 10 sym)."]
    #[inline(always)]
    pub fn macwurzcorrection(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfLmacControl10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl11Reg = crate::RegValueT<FtdfLmacControl11Reg_SPEC>;

impl FtdfLmacControl11Reg {
    #[doc = "This switching off and on of the PHY Rx can be disabled whith the control register macDisCaRxOfftoRZ.\n0 : Disabled\n1 : Enabled"]
    #[inline(always)]
    pub fn macdiscarxofftorz(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, FtdfLmacControl11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<16,1,0,FtdfLmacControl11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "In order to make it easier to calculate if it is efficient to disable and enable the PHY Rx until the RZ time is reached, a control register indicates the time needed to disable and enable the PHY Rx: macRxTotalCycleTime (resolution in 10 sym)"]
    #[inline(always)]
    pub fn macrxtotalcycletime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
            FtdfLmacControl11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl11Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl11Reg {
        <crate::RegValueT<FtdfLmacControl11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl1Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl1Reg_SPEC {
    type DataType = u32;
}
#[doc = "Lmac control register"]
pub type FtdfLmacControl1Reg = crate::RegValueT<FtdfLmacControl1Reg_SPEC>;

impl FtdfLmacControl1Reg {
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn phyrxattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, FtdfLmacControl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,FtdfLmacControl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn phyrxattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, FtdfLmacControl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8, FtdfLmacControl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn phyrxattr_calcap(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, FtdfLmacControl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, FtdfLmacControl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn phyrxattr_cn(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, FtdfLmacControl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8, FtdfLmacControl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn phyrxattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, FtdfLmacControl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, FtdfLmacControl1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl2Reg = crate::RegValueT<FtdfLmacControl2Reg_SPEC>;

impl FtdfLmacControl2Reg {
    #[doc = "Length of ED scan"]
    #[inline(always)]
    pub fn edscanduration(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xffffff,
        1,
        0,
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
            FtdfLmacControl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "if set, Energy Detect scan will be done"]
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl3Reg = crate::RegValueT<FtdfLmacControl3Reg_SPEC>;

impl FtdfLmacControl3Reg {
    #[doc = "Time to wait after CCA returned &quot;medium idle&quot; before starting TX-ON (in us).\nNote: not applicable in TSCH mode since there macTSRxTx shall be used."]
    #[inline(always)]
    pub fn ccaidlewait(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, FtdfLmacControl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            FtdfLmacControl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Max time to wait for a requested Data Frame or an announced broadcast frame"]
    #[inline(always)]
    pub fn macmaxframetotalwaittime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
            FtdfLmacControl3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfLmacControl3Reg {
    #[inline(always)]
    fn default() -> FtdfLmacControl3Reg {
        <crate::RegValueT<FtdfLmacControl3Reg_SPEC> as RegisterValue<_>>::new(1220)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfLmacControl4Reg_SPEC;
impl crate::sealed::RegSpec for FtdfLmacControl4Reg_SPEC {
    type DataType = u32;
}
#[doc = "Lmac control register"]
pub type FtdfLmacControl4Reg = crate::RegValueT<FtdfLmacControl4Reg_SPEC>;

impl FtdfLmacControl4Reg {
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn phyackattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn phyackattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,u8, FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn phyackattr_calcap(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8, FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn phyackattr_cn(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8, FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn phyackattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The control register RxPipePropDelay indicates the propagation delay in ~s of the Rx pipeline between the last symbol being captured at the DPHY interface and the &quot;data valid&quot; indication to the LMAC controller."]
    #[inline(always)]
    pub fn rxpipepropdelay(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time between negate and assert PHY_EN\nWhen the signal phy_en is deasserted, it will not be asserted within the time phySleepWait.\nThis time is indicated by the control register phySleepWait (resolution: ~s)."]
    #[inline(always)]
    pub fn physleepwait(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfLmacControl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfLmacControl4Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl5Reg = crate::RegValueT<FtdfLmacControl5Reg_SPEC>;

impl FtdfLmacControl5Reg {
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn phycsmacaattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn phycsmacaattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,u8, FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn phycsmacaattr_calcap(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8, FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn phycsmacaattr_cn(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8, FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn phycsmacaattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The output CCASTAT is valid after 8 symbols + phyRxStartup.\nThe 8 symbols are programmable by control registerCcaStatWait\\[4\\] in symbol timesl.\nDefault value is 8d."]
    #[inline(always)]
    pub fn ccastatwait(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "In order to have some flexibility the control register Ack_Response_Delay indicates the Acknowledge response time in ~s.\nThe default value shall is 192 ~s (12 symbols)."]
    #[inline(always)]
    pub fn ack_response_delay(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfLmacControl5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfLmacControl5Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl6Reg = crate::RegValueT<FtdfLmacControl6Reg_SPEC>;

impl FtdfLmacControl6Reg {
    #[doc = "The WakeUp IFS period is programmable by WUifsPeriod (in symbols).\nThe default is 12 symbols (192 ~s)."]
    #[inline(always)]
    pub fn wuifsperiod(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, FtdfLmacControl6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            FtdfLmacControl6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "The SIFS period is programmable by SifsPeriod (in symbols).\nThe default is 12 symbols (192 ~s)."]
    #[inline(always)]
    pub fn sifsperiod(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfLmacControl6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfLmacControl6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The LIFS period is programmable by LifsPeriod (in symbols).\nThe default is 40 symbols (640 ~s),"]
    #[inline(always)]
    pub fn lifsperiod(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfLmacControl6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfLmacControl6Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl7Reg = crate::RegValueT<FtdfLmacControl7Reg_SPEC>;

impl FtdfLmacControl7Reg {
    #[doc = "When performing a idle listening, the receiver is enabled for at least macCSLsamplePeriod (in symbols)."]
    #[inline(always)]
    pub fn maccslsampleperiod(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            FtdfLmacControl7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Wake-up duration in symbols."]
    #[inline(always)]
    pub fn macwuperiod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl8Reg = crate::RegValueT<FtdfLmacControl8Reg_SPEC>;

impl FtdfLmacControl8Reg {
    #[doc = "The control register macCSLstartSampleTime indicates the TimeStamp generator time (in symbols) when to start listening (called &quot;idle listening&quot;)."]
    #[inline(always)]
    pub fn maccslstartsampletime(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Lmac control register"]
pub type FtdfLmacControl9Reg = crate::RegValueT<FtdfLmacControl9Reg_SPEC>;

impl FtdfLmacControl9Reg {
    #[doc = "If a non Wake-up frame with Frame Pending bit = \'1\' is received, the receiver is enabled for at least an extra period of macCSLFramePendingWaitT (in symbols) after the end of the received frame.\nThe time the Enhanced ACK transmission lasts (if applicable) is included in this time."]
    #[inline(always)]
    pub fn maccslframependingwaitt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            FtdfLmacControl9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "After the wake-up sequence a frame is expected, the receiver will be enabled for at least a period of macCSLdataPeriod (in symbols)."]
    #[inline(always)]
    pub fn maccsldataperiod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Lmac delta control register"]
pub type FtdfLmacControlDeltaReg = crate::RegValueT<FtdfLmacControlDeltaReg_SPEC>;

impl FtdfLmacControlDeltaReg {
    #[doc = "Delta which indicates that WakeupTimerEnableStatus has changed"]
    #[inline(always)]
    pub fn wakeuptimerenablestatus_d(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event which indicates the getGeneratorVal request is completed"]
    #[inline(always)]
    pub fn getgeneratorval_e(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event that symboltime counter matched SymbolTime2Thr"]
    #[inline(always)]
    pub fn symboltime2thr_e(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event that symboltime counter matched SymbolTimeThr"]
    #[inline(always)]
    pub fn symboltimethr_e(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The SyncTimeStamp_e event is set when the TimeStampgenerator is loaded with SyncTimeStampVal.\nThis occurs at the rising edge of lp_clk when SyncTimeStampEna is set and the value of the Event generator is equal to the value SyncTimestampThr."]
    #[inline(always)]
    pub fn synctimestamp_e(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlDeltaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlDeltaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Delta bit for register &quot;LmacReady4sleep&quot;"]
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
#[doc = "Lmac mask control register"]
pub type FtdfLmacControlMaskReg = crate::RegValueT<FtdfLmacControlMaskReg_SPEC>;

impl FtdfLmacControlMaskReg {
    #[doc = "Mask for WakeupTimerEnableStatus_d"]
    #[inline(always)]
    pub fn wakeuptimerenablestatus_m(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask for getGeneratorVal_e"]
    #[inline(always)]
    pub fn getgeneratorval_m(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask for SymbolTime2Thr_e"]
    #[inline(always)]
    pub fn symboltime2thr_m(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask for SymbolTimeThr_e"]
    #[inline(always)]
    pub fn symboltimethr_m(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for event register SyncTimeStamp_e."]
    #[inline(always)]
    pub fn synctimestamp_m(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlMaskReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for delta bit &quot;LmacReady4sleep_d&quot;"]
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
#[doc = "Lmac control register"]
pub type FtdfLmacControlOsReg = crate::RegValueT<FtdfLmacControlOsReg_SPEC>;

impl FtdfLmacControlOsReg {
    #[doc = "If set, a single CCA will be performed."]
    #[inline(always)]
    pub fn singlecca(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "If set, receiving data may be done"]
    #[inline(always)]
    pub fn rxenable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacControlOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacControlOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "If set, the current values of WU gen and TS gen will be captured."]
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
#[doc = "Lmac status register"]
pub type FtdfLmacControlStatusReg = crate::RegValueT<FtdfLmacControlStatusReg_SPEC>;

impl FtdfLmacControlStatusReg {
    #[doc = "Result of ED scan."]
    #[inline(always)]
    pub fn edscanvalue(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
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
            FtdfLmacControlStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Status of WakeupTimerEnable after being clocked by LP_CLK (showing it\'s effective value)."]
    #[inline(always)]
    pub fn wakeuptimerenablestatus(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfLmacControlStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,FtdfLmacControlStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Value single CCA when CCAstat_e is set."]
    #[inline(always)]
    pub fn ccastat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacControlStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacControlStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that the LMAC is ready to go to sleep."]
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
#[doc = "Lmac event regsiter"]
pub type FtdfLmacEventReg = crate::RegValueT<FtdfLmacEventReg_SPEC>;

impl FtdfLmacEventReg {
    #[doc = "Set if one of the timers enabling the RX-ON mode expires without having received any valid frame"]
    #[inline(always)]
    pub fn rxtimerexpired_e(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, the single CCA is ready"]
    #[inline(always)]
    pub fn ccastat_e(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The event EdScanReady_e is set to notify that the ED scan is ready."]
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
#[doc = "Lmax manual PHY register"]
pub type FtdfLmacManual1Reg = crate::RegValueT<FtdfLmacManual1Reg_SPEC>;

impl FtdfLmacManual1Reg {
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn lmac_manual_phy_attr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn lmac_manual_phy_attr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,u8, FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn lmac_manual_phy_attr_calcap(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8, FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn lmac_manual_phy_attr_cn(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8, FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn lmac_manual_phy_attr_dem_pti(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "lmac_manual_pti controls the PTI interface signal when lmac_manual_mode is set"]
    #[inline(always)]
    pub fn lmac_manual_pti(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "lmac_manual_tx_frm_nr controls the entry in the tx buffer to be transmitted."]
    #[inline(always)]
    pub fn lmac_manual_tx_frm_nr(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, FtdfLmacManual1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8, FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "lmac_manual_ed_request controls the ED_REQUEST interface signal when lmac_manual_mode is set"]
    #[inline(always)]
    pub fn lmac_manual_ed_request(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "lmac_manual_rx_pipe_en controls the rx_enable signal towards the rx pipeline when lmac_manual_mode is set"]
    #[inline(always)]
    pub fn lmac_manual_rx_pipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "lmac_manual_rx_en controls the RX_EN interface signal when lmac_manual_mode is set"]
    #[inline(always)]
    pub fn lmac_manual_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "lmac_manual_tx_en controls the TX_EN interface signal when lmac_manual_mode is set"]
    #[inline(always)]
    pub fn lmac_manual_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "lmac_manual_phy_en controls the PHY_EN interface signal when lmac_manual_mode is set"]
    #[inline(always)]
    pub fn lmac_manual_phy_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacManual1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacManual1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If the control register lmac_manual_mode is set, the LMAC controller control\nsignals should be controlled by the lmac_manual_control registers"]
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
#[doc = "One shot register triggers transmission in manual mode"]
pub type FtdfLmacManualOsReg = crate::RegValueT<FtdfLmacManualOsReg_SPEC>;

impl FtdfLmacManualOsReg {
    #[doc = "One shot register which triggers the transmission of a frame from the tx buffer in lmac_manual_mode"]
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
#[doc = "Lmac status register in manual mode"]
pub type FtdfLmacManualStatusReg = crate::RegValueT<FtdfLmacManualStatusReg_SPEC>;

impl FtdfLmacManualStatusReg {
    #[doc = "lmac_manual_ed_stat shows the status of the ED_STAT interface signal."]
    #[inline(always)]
    pub fn lmac_manual_ed_stat(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
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
            FtdfLmacManualStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "lmac_manual_cca_stat shows the status of the CCA_STAT"]
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
#[doc = "Lmac mask register"]
pub type FtdfLmacMaskReg = crate::RegValueT<FtdfLmacMaskReg_SPEC>;

impl FtdfLmacMaskReg {
    #[doc = "Mask bit for event &quot;RxTimerExpired_e&quot;"]
    #[inline(always)]
    pub fn rxtimerexpired_m(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfLmacMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfLmacMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for event &quot;CCAstat_e&quot;"]
    #[inline(always)]
    pub fn ccastat_m(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfLmacMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfLmacMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for event &quot;EdScanReady_e&quot;"]
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
pub struct FtdfMacackwaitdurationReg_SPEC;
impl crate::sealed::RegSpec for FtdfMacackwaitdurationReg_SPEC {
    type DataType = u32;
}
#[doc = "Maximum time to wait for a ACK"]
pub type FtdfMacackwaitdurationReg = crate::RegValueT<FtdfMacackwaitdurationReg_SPEC>;

impl FtdfMacackwaitdurationReg {
    #[doc = "Max time to wait for a (normal) ACK"]
    #[inline(always)]
    pub fn macackwaitduration(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
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
#[doc = "Maximum time to wait for an enhanced ACK frame"]
pub type FtdfMacenhackwaitdurationReg = crate::RegValueT<FtdfMacenhackwaitdurationReg_SPEC>;

impl FtdfMacenhackwaitdurationReg {
    #[doc = "The maximum time (in s) to wait for an enhanced acknowledgement frame"]
    #[inline(always)]
    pub fn macenhackwaitduration(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Lmac FCS error register"]
pub type FtdfMacfcserrorcountReg = crate::RegValueT<FtdfMacfcserrorcountReg_SPEC>;

impl FtdfMacfcserrorcountReg {
    #[doc = "The number of received frames that were discarded due to an incorrect FCS."]
    #[inline(always)]
    pub fn macfcserrorcount(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Discarded frames register"]
pub type FtdfMacrxaddrfailfrmcntReg = crate::RegValueT<FtdfMacrxaddrfailfrmcntReg_SPEC>;

impl FtdfMacrxaddrfailfrmcntReg {
    #[doc = "Frames discarded due to incorrect address or PAN Id"]
    #[inline(always)]
    pub fn macrxaddrfailfrmcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Received acknowledgment frames"]
pub type FtdfMacrxstdackfrmokcntReg = crate::RegValueT<FtdfMacrxstdackfrmokcntReg_SPEC>;

impl FtdfMacrxstdackfrmokcntReg {
    #[doc = "Standard Acknowledgment frames received"]
    #[inline(always)]
    pub fn macrxstdackfrmokcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Unsupported frames register"]
pub type FtdfMacrxunsupfrmcntReg = crate::RegValueT<FtdfMacrxunsupfrmcntReg_SPEC>;

impl FtdfMacrxunsupfrmcntReg {
    #[doc = "Frames which do pass the checks but are not supported"]
    #[inline(always)]
    pub fn macrxunsupfrmcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Time left until next ACK is sent (us)"]
pub type FtdfMactstxackdelayvalReg = crate::RegValueT<FtdfMactstxackdelayvalReg_SPEC>;

impl FtdfMactstxackdelayvalReg {
    #[doc = "The time in us left until the ack frame is sent by the lmac"]
    #[inline(always)]
    pub fn mactstxackdelayval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Transmitted acknowledgment frames"]
pub type FtdfMactxstdackfrmcntReg = crate::RegValueT<FtdfMactxstdackfrmcntReg_SPEC>;

impl FtdfMactxstdackfrmcntReg {
    #[doc = "Standard Acknowledgment frames transmitted"]
    #[inline(always)]
    pub fn mactxstdackfrmcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Lmac PHY parameter register"]
pub type FtdfPhyParameters0Reg = crate::RegValueT<FtdfPhyParameters0Reg_SPEC>;

impl FtdfPhyParameters0Reg {
    #[doc = "See rxBitPos_0"]
    #[inline(always)]
    pub fn rxbitpos_7(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
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
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See rxBitPos_0"]
    #[inline(always)]
    pub fn rxbitpos_6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
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
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See rxBitPos_0"]
    #[inline(always)]
    pub fn rxbitpos_5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
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
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See rxBitPos_0"]
    #[inline(always)]
    pub fn rxbitpos_4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
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
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See rxBitPos_0"]
    #[inline(always)]
    pub fn rxbitpos_3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
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
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See rxBitPos_0"]
    #[inline(always)]
    pub fn rxbitpos_2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, FtdfPhyParameters0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See rxBitPos_0"]
    #[inline(always)]
    pub fn rxbitpos_1(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, FtdfPhyParameters0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            u8,
            FtdfPhyParameters0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Control rxBitPos(8)(3) controls the position that a bit should have at the DPHY interface.\nSo the default values are rxBitPos_0 = 0, rxBitPos_1 = 1, rxBitPos_2 = 2, etc.\n\nNote1 that this is a conversion from rx DPHY interface to the internal data byte\nSo\nfor(n=7;n>=0;n--)\nrx_data(n) = dphy_bit(tx_BitPos(n))\nendfor\n\nNote2 that rxBitPos and txBitPos must have inverse functions."]
    #[inline(always)]
    pub fn rxbitpos_0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, FtdfPhyParameters0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
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
#[doc = "Lmac PHY parameter register"]
pub type FtdfPhyParameters1Reg = crate::RegValueT<FtdfPhyParameters1Reg_SPEC>;

impl FtdfPhyParameters1Reg {
    #[doc = "See txBitPos_0"]
    #[inline(always)]
    pub fn txbitpos_7(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x7,
        1,
        0,
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
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See txBitPos_0"]
    #[inline(always)]
    pub fn txbitpos_6(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
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
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See txBitPos_0"]
    #[inline(always)]
    pub fn txbitpos_5(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
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
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See txBitPos_0"]
    #[inline(always)]
    pub fn txbitpos_4(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
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
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See txBitPos_0"]
    #[inline(always)]
    pub fn txbitpos_3(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
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
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See txBitPos_0"]
    #[inline(always)]
    pub fn txbitpos_2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, FtdfPhyParameters1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "See txBitPos_0"]
    #[inline(always)]
    pub fn txbitpos_1(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, FtdfPhyParameters1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            u8,
            FtdfPhyParameters1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Control txBitPos(8)(3) controls the position that a bit should have at the DPHY interface.\nSo the default values are txBitPos_0 = 0, txBitPos_1 = 1, txBitPos_2 = 2, etc.\n\nNote1 that this is a conversion from internal data byte to the DPHY interface.\nSo\nfor(n=7;n>=0;n--)\ntx_dphy_bit(n) = tx_data(tx_BitPos(n))\nendfor\n\nNote2 that txBitPos and rxBitPos must have inverse functions."]
    #[inline(always)]
    pub fn txbitpos_0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, FtdfPhyParameters1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
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
#[doc = "Lmac PHY parameter register"]
pub type FtdfPhyParameters2Reg = crate::RegValueT<FtdfPhyParameters2Reg_SPEC>;

impl FtdfPhyParameters2Reg {
    #[doc = "Phy wait time between TX_EN/RX_EN"]
    #[inline(always)]
    pub fn phytrxwait(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
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
            FtdfPhyParameters2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Phy wait time before deasserting TX_EN"]
    #[inline(always)]
    pub fn phytxfinish(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
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
            FtdfPhyParameters2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Phy delay between DPHY i/f and air"]
    #[inline(always)]
    pub fn phytxlatency(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
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
            FtdfPhyParameters2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Phy wait time before transmission"]
    #[inline(always)]
    pub fn phytxstartup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
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
#[doc = "Lmac PHY parameter register"]
pub type FtdfPhyParameters3Reg = crate::RegValueT<FtdfPhyParameters3Reg_SPEC>;

impl FtdfPhyParameters3Reg {
    #[doc = "Asserting the DPHY interface signals TX_EN or RX_EN does not take place within the time phyEnable after asserting the signal phy_en.\n(resolution: ~s)."]
    #[inline(always)]
    pub fn phyenable(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
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
            FtdfPhyParameters3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Phy delay between air and DPHY i/f"]
    #[inline(always)]
    pub fn phyrxlatency(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
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
            FtdfPhyParameters3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Phy wait time before receiving"]
    #[inline(always)]
    pub fn phyrxstartup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
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
            FtdfPhyParameters3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfPhyParameters3Reg {
    #[inline(always)]
    fn default() -> FtdfPhyParameters3Reg {
        <crate::RegValueT<FtdfPhyParameters3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfRelName0Reg_SPEC;
impl crate::sealed::RegSpec for FtdfRelName0Reg_SPEC {
    type DataType = u32;
}
#[doc = "Name of the release"]
pub type FtdfRelName0Reg = crate::RegValueT<FtdfRelName0Reg_SPEC>;

impl FtdfRelName0Reg {
    #[doc = "Name of the release"]
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Name of the release"]
pub type FtdfRelName1Reg = crate::RegValueT<FtdfRelName1Reg_SPEC>;

impl FtdfRelName1Reg {
    #[doc = "Name of the release"]
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Name of the release"]
pub type FtdfRelName2Reg = crate::RegValueT<FtdfRelName2Reg_SPEC>;

impl FtdfRelName2Reg {
    #[doc = "Name of the release"]
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Name of the release"]
pub type FtdfRelName3Reg = crate::RegValueT<FtdfRelName3Reg_SPEC>;

impl FtdfRelName3Reg {
    #[doc = "Name of the release"]
    #[inline(always)]
    pub fn rel_name(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive control register"]
pub type FtdfRxControl0Reg = crate::RegValueT<FtdfRxControl0Reg_SPEC>;

impl FtdfRxControl0Reg {
    #[doc = "If set, the LMAC controller shall ignore all consequent actions upon a set AR bit in the transmitted frame"]
    #[inline(always)]
    pub fn disrxackreceivedca(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, Frame Version 2 frames without Daddr or DPANId shall be accepted."]
    #[inline(always)]
    pub fn macimplicitbroadcast(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, WakeUp frames will not be reported but will be put into the Rx buffer."]
    #[inline(always)]
    pub fn macpasswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If the control register macAlwaysPassWakeUp is set, received Wake- up frames for this device are put into the Rx packet buffer without notifying the LMAC Controller."]
    #[inline(always)]
    pub fn macalwayspasswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The control registers macAlwaysPassFrmType\\[7:0\\], shall control if this Frame Type shall be dropped.\nIf a bit is set, the Frame Type corresponding with the bit position is not dropped, even in case of a CRC error.\nExample: if bit 1 is set, Frame Type 1 shall not be dropped.\nThe error shall be reported in the Rx meta data"]
    #[inline(always)]
    pub fn macalwayspassfrmtype(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, FtdfRxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When the control register macAlwaysPassToPanCoordinator is set, the frame is not dropped due to a span_coord_error.\nHowever, in case of an FCS error, the packet is dropped."]
    #[inline(always)]
    pub fn macalwayspasstopancoordinator(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If the control register macAlwaysPassBeaconWrongPANId is set, the frame is not dropped in case of a mismatch in PAN-ID, irrespective of the setting of RxBeaconOnly."]
    #[inline(always)]
    pub fn macalwayspassbeaconwrongpanid(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, a packet with a wrong DAddr is not dropped"]
    #[inline(always)]
    pub fn macalwayspasswrongdaddr(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If register macAlwaysPassWrongDPANId is set, packet with a wrong Destiantion PanID will not be dropped.\nHowever, in case of an FCS error, the packet is dropped."]
    #[inline(always)]
    pub fn macalwayspasswrongdpanid(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, a packet with a reserved FrameVersion shall not be dropped"]
    #[inline(always)]
    pub fn macalwayspassresframeversion(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When the control register DisDataRequestCa is set, the notification of the received Data Request is disabled."]
    #[inline(always)]
    pub fn disdatarequestca(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, a FCS error will not drop the frame"]
    #[inline(always)]
    pub fn macalwayspasscrcerror(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When the control register DisRxAckRequestca is set all consequent actions for a received Acknowledge Request bit are disabled."]
    #[inline(always)]
    pub fn disrxackrequestca(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Whan the control register DisRxFrmPendingCa is set, the notification of the received FP bit to the LMAC Controller is disabled."]
    #[inline(always)]
    pub fn disrxfrmpendingca(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indication where new data will be read\nAll four bits shall be used when using these pointer values (0d - 15d).\nHowever, the Receive Packet buffer has a size of 8 entries.\nSo reading the Receive Packet buffer entries shall use the mod8 of the pointer values."]
    #[inline(always)]
    pub fn rx_read_buf_ptr(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, FtdfRxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8, FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, only Coordinator Realignment frames are accepted"]
    #[inline(always)]
    pub fn rxcoordrealignonly(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, only Beacons frames are accepted"]
    #[inline(always)]
    pub fn rxbeacononly(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfRxControl0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfRxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If set, Rx pipe is fully set in transparent mode (for debug purpose)."]
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
#[doc = "Receive event register"]
pub type FtdfRxEventReg = crate::RegValueT<FtdfRxEventReg_SPEC>;

impl FtdfRxEventReg {
    #[doc = "Indicates the first byte of a new packet is received"]
    #[inline(always)]
    pub fn rxbyte_e(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a new packet is received"]
    #[inline(always)]
    pub fn rx_buf_avail_e(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that the Rx packet buffer has an\noverflowl"]
    #[inline(always)]
    pub fn rx_overflow_e(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfRxEventReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfRxEventReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Set when RX_SOF has been detected."]
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
#[doc = "Address receive fifo 0"]
pub type FtdfRxFifo00Reg = crate::RegValueT<FtdfRxFifo00Reg_SPEC>;

impl FtdfRxFifo00Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 1"]
pub type FtdfRxFifo10Reg = crate::RegValueT<FtdfRxFifo10Reg_SPEC>;

impl FtdfRxFifo10Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 2"]
pub type FtdfRxFifo20Reg = crate::RegValueT<FtdfRxFifo20Reg_SPEC>;

impl FtdfRxFifo20Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 3"]
pub type FtdfRxFifo30Reg = crate::RegValueT<FtdfRxFifo30Reg_SPEC>;

impl FtdfRxFifo30Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 4"]
pub type FtdfRxFifo40Reg = crate::RegValueT<FtdfRxFifo40Reg_SPEC>;

impl FtdfRxFifo40Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 5"]
pub type FtdfRxFifo50Reg = crate::RegValueT<FtdfRxFifo50Reg_SPEC>;

impl FtdfRxFifo50Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 6"]
pub type FtdfRxFifo60Reg = crate::RegValueT<FtdfRxFifo60Reg_SPEC>;

impl FtdfRxFifo60Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 7"]
pub type FtdfRxFifo70Reg = crate::RegValueT<FtdfRxFifo70Reg_SPEC>;

impl FtdfRxFifo70Reg {
    #[doc = "Receive fifo ram, contains 32 addresses per entry (32b x 32a = 128B). There are 8 entries supported."]
    #[inline(always)]
    pub fn rx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive event mask register"]
pub type FtdfRxMaskReg = crate::RegValueT<FtdfRxMaskReg_SPEC>;

impl FtdfRxMaskReg {
    #[doc = "Mask bit for event &quot;rxbyte_e&quot;."]
    #[inline(always)]
    pub fn rxbyte_m(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for event &quot;rx_buf_avail_e&quot;."]
    #[inline(always)]
    pub fn rx_buf_avail_m(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for event &quot; rx_overflow_e&quot;."]
    #[inline(always)]
    pub fn rx_overflow_m(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfRxMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfRxMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for event &quot;RxSof_e&quot;."]
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
#[doc = "Receive metadata register 0"]
pub type FtdfRxMeta00Reg = crate::RegValueT<FtdfRxMeta00Reg_SPEC>;

impl FtdfRxMeta00Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 1"]
pub type FtdfRxMeta01Reg = crate::RegValueT<FtdfRxMeta01Reg_SPEC>;

impl FtdfRxMeta01Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 2"]
pub type FtdfRxMeta02Reg = crate::RegValueT<FtdfRxMeta02Reg_SPEC>;

impl FtdfRxMeta02Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 3"]
pub type FtdfRxMeta03Reg = crate::RegValueT<FtdfRxMeta03Reg_SPEC>;

impl FtdfRxMeta03Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 4"]
pub type FtdfRxMeta04Reg = crate::RegValueT<FtdfRxMeta04Reg_SPEC>;

impl FtdfRxMeta04Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 5"]
pub type FtdfRxMeta05Reg = crate::RegValueT<FtdfRxMeta05Reg_SPEC>;

impl FtdfRxMeta05Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 6"]
pub type FtdfRxMeta06Reg = crate::RegValueT<FtdfRxMeta06Reg_SPEC>;

impl FtdfRxMeta06Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 7"]
pub type FtdfRxMeta07Reg = crate::RegValueT<FtdfRxMeta07Reg_SPEC>;

impl FtdfRxMeta07Reg {
    #[doc = "Timestamp taken when frame was received"]
    #[inline(always)]
    pub fn rx_timestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Receive metadata register 0"]
pub type FtdfRxMeta10Reg = crate::RegValueT<FtdfRxMeta10Reg_SPEC>;

impl FtdfRxMeta10Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta10Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta10Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive metadata register 1"]
pub type FtdfRxMeta11Reg = crate::RegValueT<FtdfRxMeta11Reg_SPEC>;

impl FtdfRxMeta11Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta11Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta11Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive metadata register 2"]
pub type FtdfRxMeta12Reg = crate::RegValueT<FtdfRxMeta12Reg_SPEC>;

impl FtdfRxMeta12Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta12Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta12Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive metadata register 3"]
pub type FtdfRxMeta13Reg = crate::RegValueT<FtdfRxMeta13Reg_SPEC>;

impl FtdfRxMeta13Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta13Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta13Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive metadata register 4"]
pub type FtdfRxMeta14Reg = crate::RegValueT<FtdfRxMeta14Reg_SPEC>;

impl FtdfRxMeta14Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta14Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta14Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta14Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive metadata register 5"]
pub type FtdfRxMeta15Reg = crate::RegValueT<FtdfRxMeta15Reg_SPEC>;

impl FtdfRxMeta15Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta15Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta15Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta15Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive metadata register 6"]
pub type FtdfRxMeta16Reg = crate::RegValueT<FtdfRxMeta16Reg_SPEC>;

impl FtdfRxMeta16Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta16Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta16Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta16Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive metadata register 7"]
pub type FtdfRxMeta17Reg = crate::RegValueT<FtdfRxMeta17Reg_SPEC>;

impl FtdfRxMeta17Reg {
    #[doc = "Link Quality Indication\n\n# $software_scratch@retention_ram\n# TX ram not used by hardware, can be used by software as scratch ram with retention."]
    #[inline(always)]
    pub fn quality_indicator(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfRxMeta17Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received frame not for PAN coordinator, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn ispanid_coord_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn spanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D Address error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn daddr_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "D PAN ID error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn dpanid_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame version error, applicable when frame is not discarded."]
    #[inline(always)]
    pub fn res_frm_version_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not supported frame type error, applicable when frame is not discarded"]
    #[inline(always)]
    pub fn res_frm_type_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, FtdfRxMeta17Reg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,FtdfRxMeta17Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error, applicable for transparent mode only"]
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
#[doc = "Receive status delta register"]
pub type FtdfRxStatusDeltaReg = crate::RegValueT<FtdfRxStatusDeltaReg_SPEC>;

impl FtdfRxStatusDeltaReg {
    #[doc = "Delta bit of status &quot;rx_buff_is_full&quot;"]
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
#[doc = "Receive status delta mask register"]
pub type FtdfRxStatusMaskReg = crate::RegValueT<FtdfRxStatusMaskReg_SPEC>;

impl FtdfRxStatusMaskReg {
    #[doc = "Mask bit of status &quot;rx_buff_is_full&quot;"]
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
#[doc = "Receive status register"]
pub type FtdfRxStatusReg = crate::RegValueT<FtdfRxStatusReg_SPEC>;

impl FtdfRxStatusReg {
    #[doc = "Indication where new data will be written.\nAll four bits shall be used when using these pointer values (0d - 15d).\nHowever, the Receive Packet buffer has a size of 8 entries.\nSo reading the Receive Packet buffer entries shall use the mod8 of the pointer values."]
    #[inline(always)]
    pub fn rx_write_buf_ptr(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, FtdfRxStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8, FtdfRxStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that the Rx packet buffer is full"]
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
#[doc = "Seckey register"]
pub type FtdfSeckey0Reg = crate::RegValueT<FtdfSeckey0Reg_SPEC>;

impl FtdfSeckey0Reg {
    #[doc = "Registers secKey\\[0..3\\] contain the key to be used."]
    #[inline(always)]
    pub fn seckey_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Seckey register"]
pub type FtdfSeckey1Reg = crate::RegValueT<FtdfSeckey1Reg_SPEC>;

impl FtdfSeckey1Reg {
    #[doc = "See register &quot;secKey_0&quot;"]
    #[inline(always)]
    pub fn seckey_1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "SecKey register"]
pub type FtdfSeckey2Reg = crate::RegValueT<FtdfSeckey2Reg_SPEC>;

impl FtdfSeckey2Reg {
    #[doc = "See register &quot;secKey_0&quot;"]
    #[inline(always)]
    pub fn seckey_2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Seckey register"]
pub type FtdfSeckey3Reg = crate::RegValueT<FtdfSeckey3Reg_SPEC>;

impl FtdfSeckey3Reg {
    #[doc = "See register &quot;secKey_0&quot;"]
    #[inline(always)]
    pub fn seckey_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Nonce register used for encryption/decryption"]
pub type FtdfSecnonce0Reg = crate::RegValueT<FtdfSecnonce0Reg_SPEC>;

impl FtdfSecnonce0Reg {
    #[doc = "Register secNonce\\[0..3\\] contains the Nonce to be used for encryption/decryption."]
    #[inline(always)]
    pub fn secnonce_0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Nonce register used for encryption/decryption"]
pub type FtdfSecnonce1Reg = crate::RegValueT<FtdfSecnonce1Reg_SPEC>;

impl FtdfSecnonce1Reg {
    #[doc = "See register &quot;Nonce_0&quot;"]
    #[inline(always)]
    pub fn secnonce_1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Nonce register used for encryption/decryption"]
pub type FtdfSecnonce2Reg = crate::RegValueT<FtdfSecnonce2Reg_SPEC>;

impl FtdfSecnonce2Reg {
    #[doc = "See register &quot;Nonce_0&quot;"]
    #[inline(always)]
    pub fn secnonce_2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Nonce register used for encryption/decryption"]
pub type FtdfSecnonce3Reg = crate::RegValueT<FtdfSecnonce3Reg_SPEC>;

impl FtdfSecnonce3Reg {
    #[doc = "See register &quot;Nonce_0&quot;"]
    #[inline(always)]
    pub fn secnonce_3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfSecnonce3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfSecnonce3Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Security register"]
pub type FtdfSecurity0Reg = crate::RegValueT<FtdfSecurity0Reg_SPEC>;

impl FtdfSecurity0Reg {
    #[doc = "The control register secEncDecn indicates whether to encrypt (\'1\') or decrypt (\'0\') the data."]
    #[inline(always)]
    pub fn secencdecn(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FtdfSecurity0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,FtdfSecurity0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The length of the m_data is indicated by control register secMlength."]
    #[inline(always)]
    pub fn secmlength(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, FtdfSecurity0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7f,1,0,u8, FtdfSecurity0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The length of the a_data is indicated by control register secAlength.\nThe end of the a_data is the start point of the m_data. (So secAlength must also be set if security level==4)"]
    #[inline(always)]
    pub fn secalength(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, FtdfSecurity0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8, FtdfSecurity0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The UMAC shall indicate by control registers secEntry and secTxRxn which entry to use and if it\'s from the Tx or Rx buffer (\'1\' resp. \'0\')."]
    #[inline(always)]
    pub fn secentry(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, FtdfSecurity0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, FtdfSecurity0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See register &quot;secEntry&quot;"]
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
#[doc = "Security register"]
pub type FtdfSecurity1Reg = crate::RegValueT<FtdfSecurity1Reg_SPEC>;

impl FtdfSecurity1Reg {
    #[doc = "Register secEncrFlags contain the encryption flags field.\nBits \\[2:0\\] are the 3-bit encoding flags of a_data, the other bits msut be set to \'0\'."]
    #[inline(always)]
    pub fn secencrflags(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FtdfSecurity1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, FtdfSecurity1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register secAuthFlags contains the authentication flags fields.\nbit\\[7\\] is \'0\'\nbit\\[6\\] is &quot;A_data present&quot;\nbit\\[5:3\\]: 3-bit security level of m_data\nbit\\[2:0\\]: 3-bit security level of a_data."]
    #[inline(always)]
    pub fn secauthflags(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfSecurity1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfSecurity1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "security event mask register"]
pub type FtdfSecurityEventmaskReg = crate::RegValueT<FtdfSecurityEventmaskReg_SPEC>;

impl FtdfSecurityEventmaskReg {
    #[doc = "Mask bit for event &quot;secReady_e&quot;."]
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
#[doc = "security event register"]
pub type FtdfSecurityEventReg = crate::RegValueT<FtdfSecurityEventReg_SPEC>;

impl FtdfSecurityEventReg {
    #[doc = "The Event bit secReady_e is set when the authentication process is ready (i.e. secBusy is cleared).\nThis Event shall contribute to the gen_irq."]
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
#[doc = "One shot register to start encryption/decryption"]
pub type FtdfSecurityOsReg = crate::RegValueT<FtdfSecurityOsReg_SPEC>;

impl FtdfSecurityOsReg {
    #[doc = "One_shot register to start the encryption, decryption and authentication support task."]
    #[inline(always)]
    pub fn secstart(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfSecurityOsReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfSecurityOsReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "See register &quot;Nonce_0&quot;"]
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
#[doc = "Security status register"]
pub type FtdfSecurityStatusReg = crate::RegValueT<FtdfSecurityStatusReg_SPEC>;

impl FtdfSecurityStatusReg {
    #[doc = "In case of decryption, the status bit secAuthFail will be set when the authentication has failed."]
    #[inline(always)]
    pub fn secauthfail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfSecurityStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfSecurityStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Register &quot;secBusy&quot; indicates if the encryption/decryption process is still running."]
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
pub struct FtdfSymboltime2ThrReg_SPEC;
impl crate::sealed::RegSpec for FtdfSymboltime2ThrReg_SPEC {
    type DataType = u32;
}
#[doc = "Symboltime threshold register 2"]
pub type FtdfSymboltime2ThrReg = crate::RegValueT<FtdfSymboltime2ThrReg_SPEC>;

impl FtdfSymboltime2ThrReg {
    #[doc = "Symboltime 2 Threshold to generate a general interrupt"]
    #[inline(always)]
    pub fn symboltime2thr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Value timestamp generator"]
pub type FtdfSymboltimesnapshotvalReg = crate::RegValueT<FtdfSymboltimesnapshotvalReg_SPEC>;

impl FtdfSymboltimesnapshotvalReg {
    #[doc = "The Status register SymbolTimeSnapshotVal indicates the actual value of the TimeStamp generator."]
    #[inline(always)]
    pub fn symboltimesnapshotval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Symboltime threshold register 1"]
pub type FtdfSymboltimethrReg = crate::RegValueT<FtdfSymboltimethrReg_SPEC>;

impl FtdfSymboltimethrReg {
    #[doc = "Symboltime Threshold to generate a general interrupt"]
    #[inline(always)]
    pub fn symboltimethr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Timestamp phase value regsiter"]
pub type FtdfSynctimestampphasevalReg = crate::RegValueT<FtdfSynctimestampphasevalReg_SPEC>;

impl FtdfSynctimestampphasevalReg {
    #[doc = "Value to sync TS gen phase within a symbol with.\nPlease note the +1 correction needed for most accurate result (+0.5 is than the average error, resulting is a just too fast clock)."]
    #[inline(always)]
    pub fn synctimestampphaseval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
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
#[doc = "Threshold timestamp generator"]
pub type FtdfSynctimestampthrReg = crate::RegValueT<FtdfSynctimestampthrReg_SPEC>;

impl FtdfSynctimestampthrReg {
    #[doc = "Threshold for synchronize TS gen.\nNote that due to implementation this register may only be written once per two LP_CLK periods."]
    #[inline(always)]
    pub fn synctimestampthr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        FtdfSynctimestampthrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
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
#[doc = "Value timestamp generator"]
pub type FtdfSynctimestampvalReg = crate::RegValueT<FtdfSynctimestampvalReg_SPEC>;

impl FtdfSynctimestampvalReg {
    #[doc = "Value to sync TS gen with."]
    #[inline(always)]
    pub fn synctimestampval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Timer control register"]
pub type FtdfTimerControl1Reg = crate::RegValueT<FtdfTimerControl1Reg_SPEC>;

impl FtdfTimerControl1Reg {
    #[doc = "If set, the TimeStampThr is enabled to generate a sync of the TS gen."]
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
#[doc = "Value of timestamp generator phase within a symbol"]
pub type FtdfTimestampcurrphasevalReg = crate::RegValueT<FtdfTimestampcurrphasevalReg_SPEC>;

impl FtdfTimestampcurrphasevalReg {
    #[doc = "Value of captured TS gen phase within a symbol"]
    #[inline(always)]
    pub fn timestampcurrphaseval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
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
#[doc = "Value of timestamp generator"]
pub type FtdfTimestampcurrvalReg = crate::RegValueT<FtdfTimestampcurrvalReg_SPEC>;

impl FtdfTimestampcurrvalReg {
    #[doc = "Value of captured TS gen"]
    #[inline(always)]
    pub fn timestampcurrval(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Lmac tsch control register"]
pub type FtdfTschControl0Reg = crate::RegValueT<FtdfTschControl0Reg_SPEC>;

impl FtdfTschControl0Reg {
    #[doc = "The times to wait for start of frame"]
    #[inline(always)]
    pub fn mactsrxwait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            FtdfTschControl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "End of Rx frame to start of Ack"]
    #[inline(always)]
    pub fn mactstxackdelay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Lmac tsch control register"]
pub type FtdfTschControl1Reg = crate::RegValueT<FtdfTschControl1Reg_SPEC>;

impl FtdfTschControl1Reg {
    #[doc = "The time between the CCA and the TX of a frame"]
    #[inline(always)]
    pub fn mactsrxtx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Lmac tsch control register"]
pub type FtdfTschControl2Reg = crate::RegValueT<FtdfTschControl2Reg_SPEC>;

impl FtdfTschControl2Reg {
    #[doc = "The minimum time to wait for start of an Acknowledgement"]
    #[inline(always)]
    pub fn mactsackwait(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            FtdfTschControl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "End of frame to when the transmitter shall listen for Acknowledgement"]
    #[inline(always)]
    pub fn mactsrxackdelay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Transmit first byte register"]
pub type FtdfTxbyteEReg = crate::RegValueT<FtdfTxbyteEReg_SPEC>;

impl FtdfTxbyteEReg {
    #[doc = "Indicates the last symbol of a frame is transmitted"]
    #[inline(always)]
    pub fn tx_last_symbol_e(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxbyteEReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxbyteEReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates the first byte of a frame is transmitted"]
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
#[doc = "Transmit first byte mask register"]
pub type FtdfTxbyteMReg = crate::RegValueT<FtdfTxbyteMReg_SPEC>;

impl FtdfTxbyteMReg {
    #[doc = "Mask bit for event &quot;tx_last_symbol_e&quot;."]
    #[inline(always)]
    pub fn tx_last_symbol_m(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxbyteMReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxbyteMReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask bit for event &quot;txbyte_e&quot;."]
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
#[doc = "Prop delay transmit register"]
pub type FtdfTxpipepropdelayReg = crate::RegValueT<FtdfTxpipepropdelayReg_SPEC>;

impl FtdfTxpipepropdelayReg {
    #[doc = "Prop delay of tx pipe, start to DPHY"]
    #[inline(always)]
    pub fn txpipepropdelay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
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
#[doc = "One shot register to clear flag"]
pub type FtdfTxClearOsReg = crate::RegValueT<FtdfTxClearOsReg_SPEC>;

impl FtdfTxClearOsReg {
    #[doc = "To clear tx_flag_stat"]
    #[inline(always)]
    pub fn tx_flag_clear(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, FtdfTxClearOsReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, FtdfTxClearOsReg_SPEC,crate::common::W>::from_register(self,0)
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
#[doc = "Transmit control register"]
pub type FtdfTxControl0Reg = crate::RegValueT<FtdfTxControl0Reg_SPEC>;

impl FtdfTxControl0Reg {
    #[doc = "Maximum number of CSMA-CA backoffs\n(range 0-5)"]
    #[inline(always)]
    pub fn macmaxcsmabackoffs(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, FtdfTxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8, FtdfTxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum Backoff Exponent (range 0-macMaxBE)"]
    #[inline(always)]
    pub fn macminbe(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, FtdfTxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8, FtdfTxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Backoff Exponent (range 3-8)"]
    #[inline(always)]
    pub fn macmaxbe(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, FtdfTxControl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8, FtdfTxControl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "If 1, the MPDU octets pass transparently through the MAC in the transmit direction (for debug purpose)."]
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
#[doc = "Address transmit fifo 0"]
pub type FtdfTxFifo00Reg = crate::RegValueT<FtdfTxFifo00Reg_SPEC>;

impl FtdfTxFifo00Reg {
    #[doc = "Transmit fifo buffer, contains 32 addresses per entry (32b x 32a = 128B). There are 4 entries supported."]
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 1"]
pub type FtdfTxFifo10Reg = crate::RegValueT<FtdfTxFifo10Reg_SPEC>;

impl FtdfTxFifo10Reg {
    #[doc = "Transmit fifo buffer, contains 32 addresses per entry (32b x 32a = 128B). There are 4 entries supported."]
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 2"]
pub type FtdfTxFifo20Reg = crate::RegValueT<FtdfTxFifo20Reg_SPEC>;

impl FtdfTxFifo20Reg {
    #[doc = "Transmit fifo buffer, contains 32 addresses per entry (32b x 32a = 128B). There are 4 entries supported."]
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Address transmit fifo 3"]
pub type FtdfTxFifo30Reg = crate::RegValueT<FtdfTxFifo30Reg_SPEC>;

impl FtdfTxFifo30Reg {
    #[doc = "Transmit fifo buffer, contains 32 addresses per entry (32b x 32a = 128B). There are 4 entries supported."]
    #[inline(always)]
    pub fn tx_fifo(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Clear flag register 0"]
pub type FtdfTxFlagClearE0Reg = crate::RegValueT<FtdfTxFlagClearE0Reg_SPEC>;

impl FtdfTxFlagClearE0Reg {
    #[doc = "When the LMAC clears the tx_flag_stat status event bit tx_flag_clear_e is set."]
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
#[doc = "Clear flag register 1"]
pub type FtdfTxFlagClearE1Reg = crate::RegValueT<FtdfTxFlagClearE1Reg_SPEC>;

impl FtdfTxFlagClearE1Reg {
    #[doc = "When the LMAC clears the tx_flag_stat status event bit tx_flag_clear_e is set."]
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
#[doc = "Clear flag register 2"]
pub type FtdfTxFlagClearE2Reg = crate::RegValueT<FtdfTxFlagClearE2Reg_SPEC>;

impl FtdfTxFlagClearE2Reg {
    #[doc = "When the LMAC clears the tx_flag_stat status event bit tx_flag_clear_e is set."]
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
#[doc = "Clear flag register 3"]
pub type FtdfTxFlagClearE3Reg = crate::RegValueT<FtdfTxFlagClearE3Reg_SPEC>;

impl FtdfTxFlagClearE3Reg {
    #[doc = "When the LMAC clears the tx_flag_stat status event bit tx_flag_clear_e is set."]
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
#[doc = "Mask flag register 0"]
pub type FtdfTxFlagClearM0Reg = crate::RegValueT<FtdfTxFlagClearM0Reg_SPEC>;

impl FtdfTxFlagClearM0Reg {
    #[doc = "Mask bit for event &quot;tx_flag_clear_e&quot;."]
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
#[doc = "Mask flag register 1"]
pub type FtdfTxFlagClearM1Reg = crate::RegValueT<FtdfTxFlagClearM1Reg_SPEC>;

impl FtdfTxFlagClearM1Reg {
    #[doc = "Mask bit for event &quot;tx_flag_clear_e&quot;."]
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
#[doc = "Clear flag register 2"]
pub type FtdfTxFlagClearM2Reg = crate::RegValueT<FtdfTxFlagClearM2Reg_SPEC>;

impl FtdfTxFlagClearM2Reg {
    #[doc = "Mask bit for event &quot;tx_flag_clear_e&quot;."]
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
#[doc = "Clear flag register 3"]
pub type FtdfTxFlagClearM3Reg = crate::RegValueT<FtdfTxFlagClearM3Reg_SPEC>;

impl FtdfTxFlagClearM3Reg {
    #[doc = "Mask bit for event &quot;tx_flag_clear_e&quot;."]
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
#[doc = "Transmit packet ready for transmission register 0"]
pub type FtdfTxFlagS0Reg = crate::RegValueT<FtdfTxFlagS0Reg_SPEC>;

impl FtdfTxFlagS0Reg {
    #[doc = "Packet is ready for transmission"]
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
#[doc = "Transmit packet ready for transmission register 1"]
pub type FtdfTxFlagS1Reg = crate::RegValueT<FtdfTxFlagS1Reg_SPEC>;

impl FtdfTxFlagS1Reg {
    #[doc = "Packet is ready for transmission"]
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
#[doc = "Transmit packet ready for transmission register 2"]
pub type FtdfTxFlagS2Reg = crate::RegValueT<FtdfTxFlagS2Reg_SPEC>;

impl FtdfTxFlagS2Reg {
    #[doc = "Packet is ready for transmission"]
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
#[doc = "Transmit packet ready for transmission register 3"]
pub type FtdfTxFlagS3Reg = crate::RegValueT<FtdfTxFlagS3Reg_SPEC>;

impl FtdfTxFlagS3Reg {
    #[doc = "Packet is ready for transmission"]
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
#[doc = "Transmit metadata register 0"]
pub type FtdfTxMetaData00Reg = crate::RegValueT<FtdfTxMetaData00Reg_SPEC>;

impl FtdfTxMetaData00Reg {
    #[doc = "Indicates whether CRC16 insertion must be enabled or not.\n0 : No hardware inserted CRC16\n1 : Hardware inserts CRC16"]
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether an acknowledge is expected from the recipient of this packet."]
    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether a CSMA-CA is required for the transmission of this packet."]
    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data/Cmd/Ack etc. Also indicate wakeup frame"]
    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<23, 0x7, 1, 0, u8, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x7,1,0,u8, FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x7,1,0,u8, FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<15, 0xf, 1, 0, u8, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0xf,1,0,u8, FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0xf,1,0,u8, FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xf,1,0,u8, FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Frame length"]
    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, FtdfTxMetaData00Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8, FtdfTxMetaData00Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit metadata register 1"]
pub type FtdfTxMetaData01Reg = crate::RegValueT<FtdfTxMetaData01Reg_SPEC>;

impl FtdfTxMetaData01Reg {
    #[doc = "Indicates whether CRC16 insertion must be enabled or not.\n0 : No hardware inserted CRC16\n1 : Hardware inserts CRC16"]
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether an acknowledge is expected from the recipient of this packet."]
    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether a CSMA-CA is required for the transmission of this packet."]
    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data/Cmd/Ack etc. Also indicate wakeup frame"]
    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<23, 0x7, 1, 0, u8, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x7,1,0,u8, FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x7,1,0,u8, FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<15, 0xf, 1, 0, u8, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0xf,1,0,u8, FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0xf,1,0,u8, FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xf,1,0,u8, FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Frame length"]
    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, FtdfTxMetaData01Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8, FtdfTxMetaData01Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit metadata register 2"]
pub type FtdfTxMetaData02Reg = crate::RegValueT<FtdfTxMetaData02Reg_SPEC>;

impl FtdfTxMetaData02Reg {
    #[doc = "Indicates whether CRC16 insertion must be enabled or not.\n0 : No hardware inserted CRC16\n1 : Hardware inserts CRC16"]
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether an acknowledge is expected from the recipient of this packet."]
    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether a CSMA-CA is required for the transmission of this packet."]
    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data/Cmd/Ack etc. Also indicate wakeup frame"]
    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<23, 0x7, 1, 0, u8, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x7,1,0,u8, FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x7,1,0,u8, FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<15, 0xf, 1, 0, u8, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0xf,1,0,u8, FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0xf,1,0,u8, FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xf,1,0,u8, FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Frame length"]
    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, FtdfTxMetaData02Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8, FtdfTxMetaData02Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit metadata register 3"]
pub type FtdfTxMetaData03Reg = crate::RegValueT<FtdfTxMetaData03Reg_SPEC>;

impl FtdfTxMetaData03Reg {
    #[doc = "Indicates whether CRC16 insertion must be enabled or not.\n0 : No hardware inserted CRC16\n1 : Hardware inserts CRC16"]
    #[inline(always)]
    pub fn crc16_ena(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether an acknowledge is expected from the recipient of this packet."]
    #[inline(always)]
    pub fn ackrequest(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates whether a CSMA-CA is required for the transmission of this packet."]
    #[inline(always)]
    pub fn csmaca_ena(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data/Cmd/Ack etc. Also indicate wakeup frame"]
    #[inline(always)]
    pub fn frametype(
        self,
    ) -> crate::common::RegisterField<23, 0x7, 1, 0, u8, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x7,1,0,u8, FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HighSide injection."]
    #[inline(always)]
    pub fn phyattr_hsi(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot-basis signals mapped on GPIO via PPA."]
    #[inline(always)]
    pub fn phyattr_rf_gpio_pins(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x7,1,0,u8, FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CalCap value."]
    #[inline(always)]
    pub fn phyattr_calcap(
        self,
    ) -> crate::common::RegisterField<15, 0xf, 1, 0, u8, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0xf,1,0,u8, FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Number."]
    #[inline(always)]
    pub fn phyattr_cn(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0xf,1,0,u8, FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DEM packet information."]
    #[inline(always)]
    pub fn phyattr_dem_pti(
        self,
    ) -> crate::common::RegisterField<7, 0xf, 1, 0, u8, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xf,1,0,u8, FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Frame length"]
    #[inline(always)]
    pub fn frame_length(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, FtdfTxMetaData03Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8, FtdfTxMetaData03Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit metadata register 0"]
pub type FtdfTxMetaData10Reg = crate::RegValueT<FtdfTxMetaData10Reg_SPEC>;

impl FtdfTxMetaData10Reg {
    #[doc = "Sequence Number of this packet."]
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfTxMetaData10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfTxMetaData10Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit metadata register 1"]
pub type FtdfTxMetaData11Reg = crate::RegValueT<FtdfTxMetaData11Reg_SPEC>;

impl FtdfTxMetaData11Reg {
    #[doc = "Sequence Number of this packet."]
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfTxMetaData11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfTxMetaData11Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit metadata register 2"]
pub type FtdfTxMetaData12Reg = crate::RegValueT<FtdfTxMetaData12Reg_SPEC>;

impl FtdfTxMetaData12Reg {
    #[doc = "Sequence Number of this packet."]
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfTxMetaData12Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfTxMetaData12Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit metadata register 3"]
pub type FtdfTxMetaData13Reg = crate::RegValueT<FtdfTxMetaData13Reg_SPEC>;

impl FtdfTxMetaData13Reg {
    #[doc = "Sequence Number of this packet."]
    #[inline(always)]
    pub fn macsn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, FtdfTxMetaData13Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, FtdfTxMetaData13Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit priority register 0"]
pub type FtdfTxPriority0Reg = crate::RegValueT<FtdfTxPriority0Reg_SPEC>;

impl FtdfTxPriority0Reg {
    #[doc = "A basic wake-up frame can be generated by the UMAC in the Tx buffer.\nThe meta data control bit IsWakeUp must be set to indicate that this is a Wake-up frame."]
    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority of packet"]
    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, FtdfTxPriority0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, FtdfTxPriority0Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit priority register 1"]
pub type FtdfTxPriority1Reg = crate::RegValueT<FtdfTxPriority1Reg_SPEC>;

impl FtdfTxPriority1Reg {
    #[doc = "A basic wake-up frame can be generated by the UMAC in the Tx buffer.\nThe meta data control bit IsWakeUp must be set to indicate that this is a Wake-up frame."]
    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority of packet"]
    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, FtdfTxPriority1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, FtdfTxPriority1Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit priority register 2"]
pub type FtdfTxPriority2Reg = crate::RegValueT<FtdfTxPriority2Reg_SPEC>;

impl FtdfTxPriority2Reg {
    #[doc = "A basic wake-up frame can be generated by the UMAC in the Tx buffer.\nThe meta data control bit IsWakeUp must be set to indicate that this is a Wake-up frame."]
    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority of packet"]
    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, FtdfTxPriority2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, FtdfTxPriority2Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit priority register 3"]
pub type FtdfTxPriority3Reg = crate::RegValueT<FtdfTxPriority3Reg_SPEC>;

impl FtdfTxPriority3Reg {
    #[doc = "A basic wake-up frame can be generated by the UMAC in the Tx buffer.\nThe meta data control bit IsWakeUp must be set to indicate that this is a Wake-up frame."]
    #[inline(always)]
    pub fn iswakeup(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, FtdfTxPriority3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,FtdfTxPriority3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority of packet"]
    #[inline(always)]
    pub fn tx_priority(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, FtdfTxPriority3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, FtdfTxPriority3Reg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Transmit status register 0"]
pub type FtdfTxReturnStatus00Reg = crate::RegValueT<FtdfTxReturnStatus00Reg_SPEC>;

impl FtdfTxReturnStatus00Reg {
    #[doc = "Transmit Timestamp\nThe TimeStamp of the transmitted packet."]
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Transmit status register 1"]
pub type FtdfTxReturnStatus01Reg = crate::RegValueT<FtdfTxReturnStatus01Reg_SPEC>;

impl FtdfTxReturnStatus01Reg {
    #[doc = "Transmit Timestamp\nThe TimeStamp of the transmitted packet."]
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Transmit status register 2"]
pub type FtdfTxReturnStatus02Reg = crate::RegValueT<FtdfTxReturnStatus02Reg_SPEC>;

impl FtdfTxReturnStatus02Reg {
    #[doc = "Transmit Timestamp\nThe TimeStamp of the transmitted packet."]
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Transmit status register 3"]
pub type FtdfTxReturnStatus03Reg = crate::RegValueT<FtdfTxReturnStatus03Reg_SPEC>;

impl FtdfTxReturnStatus03Reg {
    #[doc = "Transmit Timestamp\nThe TimeStamp of the transmitted packet."]
    #[inline(always)]
    pub fn txtimestamp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Transmit status register 0"]
pub type FtdfTxReturnStatus10Reg = crate::RegValueT<FtdfTxReturnStatus10Reg_SPEC>;

impl FtdfTxReturnStatus10Reg {
    #[doc = "Number of CSMA-CA retries"]
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
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
            FtdfTxReturnStatus10Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CSMA-CA status\n0 : SUCCESS\n1 : FAIL"]
    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus10Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus10Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Acknowledgement status\n0 : SUCCESS\n1 : FAIL"]
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
#[doc = "Transmit status register 1"]
pub type FtdfTxReturnStatus11Reg = crate::RegValueT<FtdfTxReturnStatus11Reg_SPEC>;

impl FtdfTxReturnStatus11Reg {
    #[doc = "Number of CSMA-CA retries"]
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
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
            FtdfTxReturnStatus11Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CSMA-CA status\n0 : SUCCESS\n1 : FAIL"]
    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus11Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus11Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Acknowledgement status\n0 : SUCCESS\n1 : FAIL"]
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
#[doc = "Transmit status register 2"]
pub type FtdfTxReturnStatus12Reg = crate::RegValueT<FtdfTxReturnStatus12Reg_SPEC>;

impl FtdfTxReturnStatus12Reg {
    #[doc = "Number of CSMA-CA retries"]
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
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
            FtdfTxReturnStatus12Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CSMA-CA status\n0 : SUCCESS\n1 : FAIL"]
    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus12Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus12Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Acknowledgement status\n0 : SUCCESS\n1 : FAIL"]
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
#[doc = "Transmit status register 3"]
pub type FtdfTxReturnStatus13Reg = crate::RegValueT<FtdfTxReturnStatus13Reg_SPEC>;

impl FtdfTxReturnStatus13Reg {
    #[doc = "Number of CSMA-CA retries"]
    #[inline(always)]
    pub fn csmacanrretries(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x7,
        1,
        0,
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
            FtdfTxReturnStatus13Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CSMA-CA status\n0 : SUCCESS\n1 : FAIL"]
    #[inline(always)]
    pub fn csmacafail(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfTxReturnStatus13Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfTxReturnStatus13Reg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Acknowledgement status\n0 : SUCCESS\n1 : FAIL"]
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
#[doc = "One shot register to set flag"]
pub type FtdfTxSetOsReg = crate::RegValueT<FtdfTxSetOsReg_SPEC>;

impl FtdfTxSetOsReg {
    #[doc = "To set tx_flag_stat"]
    #[inline(always)]
    pub fn tx_flag_set(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, FtdfTxSetOsReg_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xf,1,0,u8, FtdfTxSetOsReg_SPEC,crate::common::W>::from_register(self,0)
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
pub struct FtdfWakeupintthrReg_SPEC;
impl crate::sealed::RegSpec for FtdfWakeupintthrReg_SPEC {
    type DataType = u32;
}
#[doc = "Treshold value Wakeup timer"]
pub type FtdfWakeupintthrReg = crate::RegValueT<FtdfWakeupintthrReg_SPEC>;

impl FtdfWakeupintthrReg {
    #[doc = "Threshold for wake-up interrupt."]
    #[inline(always)]
    pub fn wakeupintthr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        FtdfWakeupintthrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            FtdfWakeupintthrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for FtdfWakeupintthrReg {
    #[inline(always)]
    fn default() -> FtdfWakeupintthrReg {
        <crate::RegValueT<FtdfWakeupintthrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FtdfWakeupControlReg_SPEC;
impl crate::sealed::RegSpec for FtdfWakeupControlReg_SPEC {
    type DataType = u32;
}
#[doc = "Wakeup timer vcontrol register"]
pub type FtdfWakeupControlReg = crate::RegValueT<FtdfWakeupControlReg_SPEC>;

impl FtdfWakeupControlReg {
    #[doc = "If set, the WakeUpIntThr is enabled to generate an interrupt."]
    #[inline(always)]
    pub fn wakeupenable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, FtdfWakeupControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,FtdfWakeupControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "A \'1\' Enables the wakeup timer.\nNote that in on_off_regmap, the register WakeupTimerEnableStatus shows the status of this register after being clocked by LP_CLK.\nChecking this register can be used as indication for software that this bit is effective in the desing."]
    #[inline(always)]
    pub fn wakeuptimerenable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, FtdfWakeupControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,FtdfWakeupControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FtdfWakeupControlReg {
    #[inline(always)]
    fn default() -> FtdfWakeupControlReg {
        <crate::RegValueT<FtdfWakeupControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
