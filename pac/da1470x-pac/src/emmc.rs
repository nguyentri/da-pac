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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"EMMC registers"]
unsafe impl ::core::marker::Send for super::Emmc {}
unsafe impl ::core::marker::Sync for super::Emmc {}
impl super::Emmc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn emmc_adma_err_stat_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcAdmaErrStatRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcAdmaErrStatRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_adma_sa_low_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcAdmaSaLowRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcAdmaSaLowRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_argument_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcArgumentRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcArgumentRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_at_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcAtCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1344usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_at_stat_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcAtStatRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcAtStatRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1348usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_auto_cmd_stat_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcAutoCmdStatRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcAutoCmdStatRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_bgap_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcBgapCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_blockcount_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcBlockcountRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcBlockcountRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_blocksize_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcBlocksizeRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcBlocksizeRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_boot_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcBootCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcBootCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1326usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_buf_data_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcBufDataRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcBufDataRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_capabilities1_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcCapabilities1RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcCapabilities1RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_capabilities2_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcCapabilities2RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcCapabilities2RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_clk_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcClkCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_cmd_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcCmdRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcCmdRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_cqcap_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcCqcapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcCqcapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_curr_capabilities1_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcCurrCapabilities1RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcCurrCapabilities1RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_curr_capabilities2_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcCurrCapabilities2RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcCurrCapabilities2RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_embedded_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcEmbeddedCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcEmbeddedCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3948usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_emmc_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcEmmcCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1324usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_error_int_signal_en_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_error_int_stat_en_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcErrorIntStatEnRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_error_int_stat_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcErrorIntStatRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcErrorIntStatRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_force_auto_cmd_stat_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcForceAutoCmdStatRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcForceAutoCmdStatRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_force_error_int_stat_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcForceErrorIntStatRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcForceErrorIntStatRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_host_cntrl_vers_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcHostCntrlVersRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcHostCntrlVersRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(254usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_host_ctrl1_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcHostCtrl1RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_host_ctrl2_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcHostCtrl2RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_mbiu_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcMbiuCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1296usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_mshc_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcMshcCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcMshcCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_mshc_ver_id_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcMshcVerIdRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcMshcVerIdRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_mshc_ver_type_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcMshcVerTypeRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcMshcVerTypeRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_normal_int_signal_en_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_normal_int_stat_en_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcNormalIntStatEnRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_normal_int_stat_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcNormalIntStatRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcNormalIntStatRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_ddr50_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetDdr50RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetDdr50RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(110usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_ds_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetDsRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetDsRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_hs_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetHsRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetHsRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_init_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetInitRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetInitRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_sdr104_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetSdr104RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetSdr104RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_sdr12_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetSdr12RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetSdr12RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_sdr25_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetSdr25RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetSdr25RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_sdr50_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetSdr50RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetSdr50RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_preset_uhs2_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPresetUhs2RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPresetUhs2RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_pstate_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPstateReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPstateReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_pwr_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPwrCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPwrCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(41usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_p_embedded_cntrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPEmbeddedCntrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPEmbeddedCntrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(230usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_p_vendor_specific_area_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPVendorSpecificAreaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPVendorSpecificAreaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_p_vndr2_specific_area_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcPVndr2SpecificAreaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcPVndr2SpecificAreaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(234usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_resp01_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcResp01RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcResp01RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_resp23_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcResp23RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcResp23RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_resp45_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcResp45RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcResp45RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_resp67_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcResp67RReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcResp67RReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_sdmasa_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcSdmasaRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcSdmasaRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_slot_intr_status_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcSlotIntrStatusRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcSlotIntrStatusRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_sw_rst_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcSwRstRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcSwRstRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(47usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_tout_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcToutCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcToutCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_wup_ctrl_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcWupCtrlRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcWupCtrlRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(43usize),
            )
        }
    }

    #[inline(always)]
    pub const fn emmc_xfer_mode_r_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EmmcXferModeRReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EmmcXferModeRReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcAdmaErrStatRReg_SPEC;
impl crate::sealed::RegSpec for EmmcAdmaErrStatRReg_SPEC {
    type DataType = u8;
}

pub type EmmcAdmaErrStatRReg = crate::RegValueT<EmmcAdmaErrStatRReg_SPEC>;

impl EmmcAdmaErrStatRReg {
    #[inline(always)]
    pub fn adma_len_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcAdmaErrStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcAdmaErrStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adma_err_states(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcAdmaErrStatRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcAdmaErrStatRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcAdmaErrStatRReg {
    #[inline(always)]
    fn default() -> EmmcAdmaErrStatRReg {
        <crate::RegValueT<EmmcAdmaErrStatRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcAdmaSaLowRReg_SPEC;
impl crate::sealed::RegSpec for EmmcAdmaSaLowRReg_SPEC {
    type DataType = u32;
}

pub type EmmcAdmaSaLowRReg = crate::RegValueT<EmmcAdmaSaLowRReg_SPEC>;

impl EmmcAdmaSaLowRReg {
    #[inline(always)]
    pub fn adma_sa_low(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcAdmaSaLowRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcAdmaSaLowRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcAdmaSaLowRReg {
    #[inline(always)]
    fn default() -> EmmcAdmaSaLowRReg {
        <crate::RegValueT<EmmcAdmaSaLowRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcArgumentRReg_SPEC;
impl crate::sealed::RegSpec for EmmcArgumentRReg_SPEC {
    type DataType = u32;
}

pub type EmmcArgumentRReg = crate::RegValueT<EmmcArgumentRReg_SPEC>;

impl EmmcArgumentRReg {
    #[inline(always)]
    pub fn argument(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcArgumentRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcArgumentRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcArgumentRReg {
    #[inline(always)]
    fn default() -> EmmcArgumentRReg {
        <crate::RegValueT<EmmcArgumentRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcAtCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcAtCtrlRReg_SPEC {
    type DataType = u32;
}

pub type EmmcAtCtrlRReg = crate::RegValueT<EmmcAtCtrlRReg_SPEC>;

impl EmmcAtCtrlRReg {
    #[inline(always)]
    pub fn swin_th_val(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, EmmcAtCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn post_change_dly(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, u8, EmmcAtCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x3,1,0,u8,u8,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pre_change_dly(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, EmmcAtCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tune_clk_stop_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_tune_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rpt_tune_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swin_th_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcAtCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcAtCtrlRReg {
        <crate::RegValueT<EmmcAtCtrlRReg_SPEC> as RegisterValue<_>>::new(16777220)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcAtStatRReg_SPEC;
impl crate::sealed::RegSpec for EmmcAtStatRReg_SPEC {
    type DataType = u32;
}

pub type EmmcAtStatRReg = crate::RegValueT<EmmcAtStatRReg_SPEC>;

impl EmmcAtStatRReg {
    #[inline(always)]
    pub fn l_edge_ph_code(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, EmmcAtStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,EmmcAtStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r_edge_ph_code(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EmmcAtStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EmmcAtStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn center_ph_code(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, EmmcAtStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,EmmcAtStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcAtStatRReg {
    #[inline(always)]
    fn default() -> EmmcAtStatRReg {
        <crate::RegValueT<EmmcAtStatRReg_SPEC> as RegisterValue<_>>::new(6)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcAutoCmdStatRReg_SPEC;
impl crate::sealed::RegSpec for EmmcAutoCmdStatRReg_SPEC {
    type DataType = u16;
}

pub type EmmcAutoCmdStatRReg = crate::RegValueT<EmmcAutoCmdStatRReg_SPEC>;

impl EmmcAutoCmdStatRReg {
    #[inline(always)]
    pub fn cmd_not_issued_auto_cmd12(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_ebit_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd12_not_exec(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcAutoCmdStatRReg {
    #[inline(always)]
    fn default() -> EmmcAutoCmdStatRReg {
        <crate::RegValueT<EmmcAutoCmdStatRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcBgapCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcBgapCtrlRReg_SPEC {
    type DataType = u8;
}

pub type EmmcBgapCtrlRReg = crate::RegValueT<EmmcBgapCtrlRReg_SPEC>;

impl EmmcBgapCtrlRReg {
    #[inline(always)]
    pub fn int_at_bgap(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcBgapCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rd_wait_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcBgapCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn continue_req(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcBgapCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stop_bg_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcBgapCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcBgapCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcBgapCtrlRReg {
        <crate::RegValueT<EmmcBgapCtrlRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcBlockcountRReg_SPEC;
impl crate::sealed::RegSpec for EmmcBlockcountRReg_SPEC {
    type DataType = u16;
}

pub type EmmcBlockcountRReg = crate::RegValueT<EmmcBlockcountRReg_SPEC>;

impl EmmcBlockcountRReg {
    #[inline(always)]
    pub fn block_cnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        EmmcBlockcountRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            EmmcBlockcountRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcBlockcountRReg {
    #[inline(always)]
    fn default() -> EmmcBlockcountRReg {
        <crate::RegValueT<EmmcBlockcountRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcBlocksizeRReg_SPEC;
impl crate::sealed::RegSpec for EmmcBlocksizeRReg_SPEC {
    type DataType = u16;
}

pub type EmmcBlocksizeRReg = crate::RegValueT<EmmcBlocksizeRReg_SPEC>;

impl EmmcBlocksizeRReg {
    #[inline(always)]
    pub fn sdma_buf_bdary(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        u8,
        u8,
        EmmcBlocksizeRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            u8,
            u8,
            EmmcBlocksizeRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn xfer_block_size(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        EmmcBlocksizeRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            EmmcBlocksizeRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcBlocksizeRReg {
    #[inline(always)]
    fn default() -> EmmcBlocksizeRReg {
        <crate::RegValueT<EmmcBlocksizeRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcBootCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcBootCtrlRReg_SPEC {
    type DataType = u16;
}

pub type EmmcBootCtrlRReg = crate::RegValueT<EmmcBootCtrlRReg_SPEC>;

impl EmmcBootCtrlRReg {
    #[inline(always)]
    pub fn boot_tout_cnt(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, EmmcBootCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            u8,
            u8,
            EmmcBootCtrlRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn boot_ack_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcBootCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcBootCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn validate_boot(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcBootCtrlRReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcBootCtrlRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn man_boot_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcBootCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcBootCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcBootCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcBootCtrlRReg {
        <crate::RegValueT<EmmcBootCtrlRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcBufDataRReg_SPEC;
impl crate::sealed::RegSpec for EmmcBufDataRReg_SPEC {
    type DataType = u32;
}

pub type EmmcBufDataRReg = crate::RegValueT<EmmcBufDataRReg_SPEC>;

impl EmmcBufDataRReg {
    #[inline(always)]
    pub fn buf_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcBufDataRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcBufDataRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcBufDataRReg {
    #[inline(always)]
    fn default() -> EmmcBufDataRReg {
        <crate::RegValueT<EmmcBufDataRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcCapabilities1RReg_SPEC;
impl crate::sealed::RegSpec for EmmcCapabilities1RReg_SPEC {
    type DataType = u32;
}

pub type EmmcCapabilities1RReg = crate::RegValueT<EmmcCapabilities1RReg_SPEC>;

impl EmmcCapabilities1RReg {
    #[inline(always)]
    pub fn slot_type_r(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcCapabilities1RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcCapabilities1RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn async_int_support(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<29,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_addr_64_v3(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<28,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_addr_64_v4(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<27,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn volt_18(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<26,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn volt_30(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<25,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn volt_33(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<24,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sus_res_support(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<23,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdma_support(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<22,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn high_speed_support(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<21,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adma2_support(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<19,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn embedded_8_bit(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<18,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn max_blk_len(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcCapabilities1RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcCapabilities1RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn base_clk_freq(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcCapabilities1RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcCapabilities1RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tout_clk_unit(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tout_clk_freq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        EmmcCapabilities1RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            EmmcCapabilities1RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcCapabilities1RReg {
    #[inline(always)]
    fn default() -> EmmcCapabilities1RReg {
        <crate::RegValueT<EmmcCapabilities1RReg_SPEC> as RegisterValue<_>>::new(544092320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcCapabilities2RReg_SPEC;
impl crate::sealed::RegSpec for EmmcCapabilities2RReg_SPEC {
    type DataType = u32;
}

pub type EmmcCapabilities2RReg = crate::RegValueT<EmmcCapabilities2RReg_SPEC>;

impl EmmcCapabilities2RReg {
    #[inline(always)]
    pub fn vdd2_18v_support(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<28,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adma3_support(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<27,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_mul(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcCapabilities2RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcCapabilities2RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn re_tuning_modes(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcCapabilities2RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcCapabilities2RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn use_tuning_sdr50(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn retune_cnt(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        EmmcCapabilities2RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            EmmcCapabilities2RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn drv_typed(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn drv_typec(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn drv_typea(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uhs2_support(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ddr50_support(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdr104_support(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdr50_support(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcCapabilities2RReg {
    #[inline(always)]
    fn default() -> EmmcCapabilities2RReg {
        <crate::RegValueT<EmmcCapabilities2RReg_SPEC> as RegisterValue<_>>::new(119)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcClkCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcClkCtrlRReg_SPEC {
    type DataType = u16;
}

pub type EmmcClkCtrlRReg = crate::RegValueT<EmmcClkCtrlRReg_SPEC>;

impl EmmcClkCtrlRReg {
    #[inline(always)]
    pub fn freq_sel(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EmmcClkCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn upper_freq_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, EmmcClkCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_gen_select(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sd_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn internal_clk_stable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcClkCtrlRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn internal_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcClkCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcClkCtrlRReg {
        <crate::RegValueT<EmmcClkCtrlRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcCmdRReg_SPEC;
impl crate::sealed::RegSpec for EmmcCmdRReg_SPEC {
    type DataType = u16;
}

pub type EmmcCmdRReg = crate::RegValueT<EmmcCmdRReg_SPEC>;

impl EmmcCmdRReg {
    #[inline(always)]
    pub fn cmd_index(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, EmmcCmdRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_type(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, EmmcCmdRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_present_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_idx_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_crc_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sub_cmd_flag(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_type_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, EmmcCmdRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcCmdRReg {
    #[inline(always)]
    fn default() -> EmmcCmdRReg {
        <crate::RegValueT<EmmcCmdRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcCqcapReg_SPEC;
impl crate::sealed::RegSpec for EmmcCqcapReg_SPEC {
    type DataType = u32;
}

pub type EmmcCqcapReg = crate::RegValueT<EmmcCqcapReg_SPEC>;

impl EmmcCqcapReg {
    #[inline(always)]
    pub fn crypto_support(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcCqcapReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,EmmcCqcapReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cqccap_rsvd2(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, EmmcCqcapReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,EmmcCqcapReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn itcfmul(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, EmmcCqcapReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,EmmcCqcapReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn itcfval(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, EmmcCqcapReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,EmmcCqcapReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcCqcapReg {
    #[inline(always)]
    fn default() -> EmmcCqcapReg {
        <crate::RegValueT<EmmcCqcapReg_SPEC> as RegisterValue<_>>::new(12488)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcCurrCapabilities1RReg_SPEC;
impl crate::sealed::RegSpec for EmmcCurrCapabilities1RReg_SPEC {
    type DataType = u32;
}

pub type EmmcCurrCapabilities1RReg = crate::RegValueT<EmmcCurrCapabilities1RReg_SPEC>;

impl EmmcCurrCapabilities1RReg {
    #[inline(always)]
    pub fn max_cur_18v(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcCurrCapabilities1RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcCurrCapabilities1RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn max_cur_30v(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcCurrCapabilities1RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcCurrCapabilities1RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn max_cur_33v(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcCurrCapabilities1RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcCurrCapabilities1RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcCurrCapabilities1RReg {
    #[inline(always)]
    fn default() -> EmmcCurrCapabilities1RReg {
        <crate::RegValueT<EmmcCurrCapabilities1RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcCurrCapabilities2RReg_SPEC;
impl crate::sealed::RegSpec for EmmcCurrCapabilities2RReg_SPEC {
    type DataType = u32;
}

pub type EmmcCurrCapabilities2RReg = crate::RegValueT<EmmcCurrCapabilities2RReg_SPEC>;

impl EmmcCurrCapabilities2RReg {
    #[inline(always)]
    pub fn max_cur_vdd2_18v(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcCurrCapabilities2RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcCurrCapabilities2RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcCurrCapabilities2RReg {
    #[inline(always)]
    fn default() -> EmmcCurrCapabilities2RReg {
        <crate::RegValueT<EmmcCurrCapabilities2RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcEmbeddedCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcEmbeddedCtrlRReg_SPEC {
    type DataType = u32;
}

pub type EmmcEmbeddedCtrlRReg = crate::RegValueT<EmmcEmbeddedCtrlRReg_SPEC>;

impl EmmcEmbeddedCtrlRReg {
    #[inline(always)]
    pub fn back_end_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7f,
        1,
        0,
        u8,
        u8,
        EmmcEmbeddedCtrlRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7f,
            1,
            0,
            u8,
            u8,
            EmmcEmbeddedCtrlRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn int_pin_sel(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
        u8,
        u8,
        EmmcEmbeddedCtrlRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x7,
            1,
            0,
            u8,
            u8,
            EmmcEmbeddedCtrlRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_pin_sel(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        EmmcEmbeddedCtrlRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            EmmcEmbeddedCtrlRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bus_width_preset(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7f,
        1,
        0,
        u8,
        u8,
        EmmcEmbeddedCtrlRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x7f,
            1,
            0,
            u8,
            u8,
            EmmcEmbeddedCtrlRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn num_int_pin(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcEmbeddedCtrlRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcEmbeddedCtrlRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn num_clk_pin(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
        u8,
        EmmcEmbeddedCtrlRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            EmmcEmbeddedCtrlRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcEmbeddedCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcEmbeddedCtrlRReg {
        <crate::RegValueT<EmmcEmbeddedCtrlRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcEmmcCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcEmmcCtrlRReg_SPEC {
    type DataType = u16;
}

pub type EmmcEmmcCtrlRReg = crate::RegValueT<EmmcEmmcCtrlRReg_SPEC>;

impl EmmcEmmcCtrlRReg {
    #[inline(always)]
    pub fn emmc_rst_n_oe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcEmmcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn emmc_rst_n(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcEmmcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disable_data_crc_chk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcEmmcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_is_emmc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcEmmcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcEmmcCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcEmmcCtrlRReg {
        <crate::RegValueT<EmmcEmmcCtrlRReg_SPEC> as RegisterValue<_>>::new(12)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcErrorIntSignalEnRReg_SPEC;
impl crate::sealed::RegSpec for EmmcErrorIntSignalEnRReg_SPEC {
    type DataType = u16;
}

pub type EmmcErrorIntSignalEnRReg = crate::RegValueT<EmmcErrorIntSignalEnRReg_SPEC>;

impl EmmcErrorIntSignalEnRReg {
    #[inline(always)]
    pub fn vendor_err_signal_en3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vendor_err_signal_en2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vendor_err_signal_en1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boot_ack_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tuning_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adma_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cur_lmt_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_end_bit_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_crc_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_tout_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_idx_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_end_bit_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_crc_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_tout_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcErrorIntSignalEnRReg {
    #[inline(always)]
    fn default() -> EmmcErrorIntSignalEnRReg {
        <crate::RegValueT<EmmcErrorIntSignalEnRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcErrorIntStatEnRReg_SPEC;
impl crate::sealed::RegSpec for EmmcErrorIntStatEnRReg_SPEC {
    type DataType = u16;
}

pub type EmmcErrorIntStatEnRReg = crate::RegValueT<EmmcErrorIntStatEnRReg_SPEC>;

impl EmmcErrorIntStatEnRReg {
    #[inline(always)]
    pub fn vendor_err_stat_en3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vendor_err_stat_en2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vendor_err_stat_en1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boot_ack_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tuning_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adma_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cur_lmt_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_end_bit_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_crc_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_tout_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_idx_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_end_bit_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_crc_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_tout_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcErrorIntStatEnRReg {
    #[inline(always)]
    fn default() -> EmmcErrorIntStatEnRReg {
        <crate::RegValueT<EmmcErrorIntStatEnRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcErrorIntStatRReg_SPEC;
impl crate::sealed::RegSpec for EmmcErrorIntStatRReg_SPEC {
    type DataType = u16;
}

pub type EmmcErrorIntStatRReg = crate::RegValueT<EmmcErrorIntStatRReg_SPEC>;

impl EmmcErrorIntStatRReg {
    #[inline(always)]
    pub fn boot_ack_err(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tuning_err(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adma_err(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_err(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cur_lmt_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcErrorIntStatRReg {
    #[inline(always)]
    fn default() -> EmmcErrorIntStatRReg {
        <crate::RegValueT<EmmcErrorIntStatRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcForceAutoCmdStatRReg_SPEC;
impl crate::sealed::RegSpec for EmmcForceAutoCmdStatRReg_SPEC {
    type DataType = u16;
}

pub type EmmcForceAutoCmdStatRReg = crate::RegValueT<EmmcForceAutoCmdStatRReg_SPEC>;

impl EmmcForceAutoCmdStatRReg {
    #[inline(always)]
    pub fn force_cmd_not_issued_auto_cmd12(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_auto_cmd_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_auto_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_auto_cmd_ebit_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_auto_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_auto_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_auto_cmd12_not_exec(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcForceAutoCmdStatRReg {
    #[inline(always)]
    fn default() -> EmmcForceAutoCmdStatRReg {
        <crate::RegValueT<EmmcForceAutoCmdStatRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcForceErrorIntStatRReg_SPEC;
impl crate::sealed::RegSpec for EmmcForceErrorIntStatRReg_SPEC {
    type DataType = u16;
}

pub type EmmcForceErrorIntStatRReg = crate::RegValueT<EmmcForceErrorIntStatRReg_SPEC>;

impl EmmcForceErrorIntStatRReg {
    #[inline(always)]
    pub fn force_vendor_err3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_vendor_err2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_vendor_err1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_boot_ack_err(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_tuning_err(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_adma_err(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_auto_cmd_err(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_cur_lmt_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_data_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_data_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_data_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_cmd_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcForceErrorIntStatRReg {
    #[inline(always)]
    fn default() -> EmmcForceErrorIntStatRReg {
        <crate::RegValueT<EmmcForceErrorIntStatRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcHostCntrlVersRReg_SPEC;
impl crate::sealed::RegSpec for EmmcHostCntrlVersRReg_SPEC {
    type DataType = u16;
}

pub type EmmcHostCntrlVersRReg = crate::RegValueT<EmmcHostCntrlVersRReg_SPEC>;

impl EmmcHostCntrlVersRReg {
    #[inline(always)]
    pub fn vendor_version_num(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcHostCntrlVersRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcHostCntrlVersRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spec_version_num(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcHostCntrlVersRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcHostCntrlVersRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcHostCntrlVersRReg {
    #[inline(always)]
    fn default() -> EmmcHostCntrlVersRReg {
        <crate::RegValueT<EmmcHostCntrlVersRReg_SPEC> as RegisterValue<_>>::new(5)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcHostCtrl1RReg_SPEC;
impl crate::sealed::RegSpec for EmmcHostCtrl1RReg_SPEC {
    type DataType = u8;
}

pub type EmmcHostCtrl1RReg = crate::RegValueT<EmmcHostCtrl1RReg_SPEC>;

impl EmmcHostCtrl1RReg {
    #[inline(always)]
    pub fn card_detect_sig_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_detect_test_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ext_dat_xfer(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_sel(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, EmmcHostCtrl1RReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcHostCtrl1RReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn high_speed_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dat_xfer_width(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn led_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcHostCtrl1RReg {
    #[inline(always)]
    fn default() -> EmmcHostCtrl1RReg {
        <crate::RegValueT<EmmcHostCtrl1RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcHostCtrl2RReg_SPEC;
impl crate::sealed::RegSpec for EmmcHostCtrl2RReg_SPEC {
    type DataType = u16;
}

pub type EmmcHostCtrl2RReg = crate::RegValueT<EmmcHostCtrl2RReg_SPEC>;

impl EmmcHostCtrl2RReg {
    #[inline(always)]
    pub fn preset_val_enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn async_int_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn addressing(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn host_ver4_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd23_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adma2_len_mode(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uhs2_if_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sample_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn exec_tuning(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn drv_strength_sel(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, EmmcHostCtrl2RReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcHostCtrl2RReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn signaling_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uhs_mode_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, EmmcHostCtrl2RReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            EmmcHostCtrl2RReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcHostCtrl2RReg {
    #[inline(always)]
    fn default() -> EmmcHostCtrl2RReg {
        <crate::RegValueT<EmmcHostCtrl2RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcMbiuCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcMbiuCtrlRReg_SPEC {
    type DataType = u8;
}

pub type EmmcMbiuCtrlRReg = crate::RegValueT<EmmcMbiuCtrlRReg_SPEC>;

impl EmmcMbiuCtrlRReg {
    #[inline(always)]
    pub fn burst_incr16_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcMbiuCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_incr8_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcMbiuCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn burst_incr4_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcMbiuCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn undefl_incr_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcMbiuCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcMbiuCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcMbiuCtrlRReg {
        <crate::RegValueT<EmmcMbiuCtrlRReg_SPEC> as RegisterValue<_>>::new(14)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcMshcCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcMshcCtrlRReg_SPEC {
    type DataType = u8;
}

pub type EmmcMshcCtrlRReg = crate::RegValueT<EmmcMshcCtrlRReg_SPEC>;

impl EmmcMshcCtrlRReg {
    #[inline(always)]
    pub fn sw_cg_dis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcMshcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcMshcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_conflict_check(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcMshcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcMshcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcMshcCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcMshcCtrlRReg {
        <crate::RegValueT<EmmcMshcCtrlRReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcMshcVerIdRReg_SPEC;
impl crate::sealed::RegSpec for EmmcMshcVerIdRReg_SPEC {
    type DataType = u32;
}

pub type EmmcMshcVerIdRReg = crate::RegValueT<EmmcMshcVerIdRReg_SPEC>;

impl EmmcMshcVerIdRReg {
    #[inline(always)]
    pub fn mshc_ver_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcMshcVerIdRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcMshcVerIdRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcMshcVerIdRReg {
    #[inline(always)]
    fn default() -> EmmcMshcVerIdRReg {
        <crate::RegValueT<EmmcMshcVerIdRReg_SPEC> as RegisterValue<_>>::new(825765930)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcMshcVerTypeRReg_SPEC;
impl crate::sealed::RegSpec for EmmcMshcVerTypeRReg_SPEC {
    type DataType = u32;
}

pub type EmmcMshcVerTypeRReg = crate::RegValueT<EmmcMshcVerTypeRReg_SPEC>;

impl EmmcMshcVerTypeRReg {
    #[inline(always)]
    pub fn mshc_ver_type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcMshcVerTypeRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcMshcVerTypeRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcMshcVerTypeRReg {
    #[inline(always)]
    fn default() -> EmmcMshcVerTypeRReg {
        <crate::RegValueT<EmmcMshcVerTypeRReg_SPEC> as RegisterValue<_>>::new(1734421034)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcNormalIntSignalEnRReg_SPEC;
impl crate::sealed::RegSpec for EmmcNormalIntSignalEnRReg_SPEC {
    type DataType = u16;
}

pub type EmmcNormalIntSignalEnRReg = crate::RegValueT<EmmcNormalIntSignalEnRReg_SPEC>;

impl EmmcNormalIntSignalEnRReg {
    #[inline(always)]
    pub fn cqe_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<
            14,
            1,
            0,
            EmmcNormalIntSignalEnRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fx_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<
            13,
            1,
            0,
            EmmcNormalIntSignalEnRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn re_tune_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<
            12,
            1,
            0,
            EmmcNormalIntSignalEnRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn int_c_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<
            11,
            1,
            0,
            EmmcNormalIntSignalEnRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn int_b_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<
            10,
            1,
            0,
            EmmcNormalIntSignalEnRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn int_a_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_interrupt_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_removal_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_insertion_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_rd_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_wr_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_interrupt_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgap_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xfer_complete_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_complete_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcNormalIntSignalEnRReg {
    #[inline(always)]
    fn default() -> EmmcNormalIntSignalEnRReg {
        <crate::RegValueT<EmmcNormalIntSignalEnRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcNormalIntStatEnRReg_SPEC;
impl crate::sealed::RegSpec for EmmcNormalIntStatEnRReg_SPEC {
    type DataType = u16;
}

pub type EmmcNormalIntStatEnRReg = crate::RegValueT<EmmcNormalIntStatEnRReg_SPEC>;

impl EmmcNormalIntStatEnRReg {
    #[inline(always)]
    pub fn cqe_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fx_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn re_tune_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn int_c_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn int_b_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn int_a_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_interrupt_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_removal_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_insertion_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_rd_ready_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_wr_ready_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_interrupt_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgap_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xfer_complete_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_complete_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcNormalIntStatEnRReg {
    #[inline(always)]
    fn default() -> EmmcNormalIntStatEnRReg {
        <crate::RegValueT<EmmcNormalIntStatEnRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcNormalIntStatRReg_SPEC;
impl crate::sealed::RegSpec for EmmcNormalIntStatRReg_SPEC {
    type DataType = u16;
}

pub type EmmcNormalIntStatRReg = crate::RegValueT<EmmcNormalIntStatRReg_SPEC>;

impl EmmcNormalIntStatRReg {
    #[inline(always)]
    pub fn err_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cqe_event(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fx_event(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn re_tune_event(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn int_c(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn int_b(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn int_a(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_removal_stat_r(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_insertion(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_rd_ready(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_wr_ready(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgap_event(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xfer_complete(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_complete(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcNormalIntStatRReg {
    #[inline(always)]
    fn default() -> EmmcNormalIntStatRReg {
        <crate::RegValueT<EmmcNormalIntStatRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetDdr50RReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetDdr50RReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetDdr50RReg = crate::RegValueT<EmmcPresetDdr50RReg_SPEC>;

impl EmmcPresetDdr50RReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcPresetDdr50RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcPresetDdr50RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetDdr50RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetDdr50RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetDdr50RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetDdr50RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetDdr50RReg {
    #[inline(always)]
    fn default() -> EmmcPresetDdr50RReg {
        <crate::RegValueT<EmmcPresetDdr50RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetDsRReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetDsRReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetDsRReg = crate::RegValueT<EmmcPresetDsRReg_SPEC>;

impl EmmcPresetDsRReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, EmmcPresetDsRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,EmmcPresetDsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetDsRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetDsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetDsRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetDsRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetDsRReg {
    #[inline(always)]
    fn default() -> EmmcPresetDsRReg {
        <crate::RegValueT<EmmcPresetDsRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetHsRReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetHsRReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetHsRReg = crate::RegValueT<EmmcPresetHsRReg_SPEC>;

impl EmmcPresetHsRReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, EmmcPresetHsRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,EmmcPresetHsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetHsRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetHsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetHsRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetHsRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetHsRReg {
    #[inline(always)]
    fn default() -> EmmcPresetHsRReg {
        <crate::RegValueT<EmmcPresetHsRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetInitRReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetInitRReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetInitRReg = crate::RegValueT<EmmcPresetInitRReg_SPEC>;

impl EmmcPresetInitRReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcPresetInitRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcPresetInitRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetInitRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetInitRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetInitRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetInitRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetInitRReg {
    #[inline(always)]
    fn default() -> EmmcPresetInitRReg {
        <crate::RegValueT<EmmcPresetInitRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetSdr104RReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetSdr104RReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetSdr104RReg = crate::RegValueT<EmmcPresetSdr104RReg_SPEC>;

impl EmmcPresetSdr104RReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcPresetSdr104RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcPresetSdr104RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr104RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr104RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetSdr104RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetSdr104RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetSdr104RReg {
    #[inline(always)]
    fn default() -> EmmcPresetSdr104RReg {
        <crate::RegValueT<EmmcPresetSdr104RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetSdr12RReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetSdr12RReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetSdr12RReg = crate::RegValueT<EmmcPresetSdr12RReg_SPEC>;

impl EmmcPresetSdr12RReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcPresetSdr12RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcPresetSdr12RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr12RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr12RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetSdr12RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetSdr12RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetSdr12RReg {
    #[inline(always)]
    fn default() -> EmmcPresetSdr12RReg {
        <crate::RegValueT<EmmcPresetSdr12RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetSdr25RReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetSdr25RReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetSdr25RReg = crate::RegValueT<EmmcPresetSdr25RReg_SPEC>;

impl EmmcPresetSdr25RReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcPresetSdr25RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcPresetSdr25RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr25RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr25RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetSdr25RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetSdr25RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetSdr25RReg {
    #[inline(always)]
    fn default() -> EmmcPresetSdr25RReg {
        <crate::RegValueT<EmmcPresetSdr25RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetSdr50RReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetSdr50RReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetSdr50RReg = crate::RegValueT<EmmcPresetSdr50RReg_SPEC>;

impl EmmcPresetSdr50RReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcPresetSdr50RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcPresetSdr50RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr50RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr50RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetSdr50RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetSdr50RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetSdr50RReg {
    #[inline(always)]
    fn default() -> EmmcPresetSdr50RReg {
        <crate::RegValueT<EmmcPresetSdr50RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPresetUhs2RReg_SPEC;
impl crate::sealed::RegSpec for EmmcPresetUhs2RReg_SPEC {
    type DataType = u16;
}

pub type EmmcPresetUhs2RReg = crate::RegValueT<EmmcPresetUhs2RReg_SPEC>;

impl EmmcPresetUhs2RReg {
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        EmmcPresetUhs2RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            EmmcPresetUhs2RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetUhs2RReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetUhs2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        EmmcPresetUhs2RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            EmmcPresetUhs2RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPresetUhs2RReg {
    #[inline(always)]
    fn default() -> EmmcPresetUhs2RReg {
        <crate::RegValueT<EmmcPresetUhs2RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPstateReg_SPEC;
impl crate::sealed::RegSpec for EmmcPstateReg_SPEC {
    type DataType = u32;
}

pub type EmmcPstateReg = crate::RegValueT<EmmcPstateReg_SPEC>;

impl EmmcPstateReg {
    #[inline(always)]
    pub fn uhs2_if_detect(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lane_sync(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn in_dormant_st(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sub_cmd_stat(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_issue_err(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn host_reg_vol(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_line_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dat_3_0(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, EmmcPstateReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wr_protect_sw_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_detect_pin_level(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_stable(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_inserted(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_rd_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buf_wr_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rd_xfer_active(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wr_xfer_active(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dat_7_4(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, EmmcPstateReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn re_tune_req(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dat_line_active(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_inhibit_dat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_inhibit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcPstateReg {
    #[inline(always)]
    fn default() -> EmmcPstateReg {
        <crate::RegValueT<EmmcPstateReg_SPEC> as RegisterValue<_>>::new(34340864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPwrCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcPwrCtrlRReg_SPEC {
    type DataType = u8;
}

pub type EmmcPwrCtrlRReg = crate::RegValueT<EmmcPwrCtrlRReg_SPEC>;

impl EmmcPwrCtrlRReg {
    #[inline(always)]
    pub fn sd_bus_vol_vdd2(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, EmmcPwrCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,EmmcPwrCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sd_bus_pwr_vdd2(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcPwrCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcPwrCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sd_bus_vol_vdd1(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, EmmcPwrCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,EmmcPwrCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sd_bus_pwr_vdd1(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcPwrCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcPwrCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcPwrCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcPwrCtrlRReg {
        <crate::RegValueT<EmmcPwrCtrlRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPEmbeddedCntrlReg_SPEC;
impl crate::sealed::RegSpec for EmmcPEmbeddedCntrlReg_SPEC {
    type DataType = u16;
}

pub type EmmcPEmbeddedCntrlReg = crate::RegValueT<EmmcPEmbeddedCntrlReg_SPEC>;

impl EmmcPEmbeddedCntrlReg {
    #[inline(always)]
    pub fn reg_offset_embedded_cntrl_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        EmmcPEmbeddedCntrlReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            EmmcPEmbeddedCntrlReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPEmbeddedCntrlReg {
    #[inline(always)]
    fn default() -> EmmcPEmbeddedCntrlReg {
        <crate::RegValueT<EmmcPEmbeddedCntrlReg_SPEC> as RegisterValue<_>>::new(3948)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPVendorSpecificAreaReg_SPEC;
impl crate::sealed::RegSpec for EmmcPVendorSpecificAreaReg_SPEC {
    type DataType = u16;
}

pub type EmmcPVendorSpecificAreaReg = crate::RegValueT<EmmcPVendorSpecificAreaReg_SPEC>;

impl EmmcPVendorSpecificAreaReg {
    #[inline(always)]
    pub fn reg_offset_addr_vendor(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        EmmcPVendorSpecificAreaReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            EmmcPVendorSpecificAreaReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPVendorSpecificAreaReg {
    #[inline(always)]
    fn default() -> EmmcPVendorSpecificAreaReg {
        <crate::RegValueT<EmmcPVendorSpecificAreaReg_SPEC> as RegisterValue<_>>::new(1280)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcPVndr2SpecificAreaReg_SPEC;
impl crate::sealed::RegSpec for EmmcPVndr2SpecificAreaReg_SPEC {
    type DataType = u16;
}

pub type EmmcPVndr2SpecificAreaReg = crate::RegValueT<EmmcPVndr2SpecificAreaReg_SPEC>;

impl EmmcPVndr2SpecificAreaReg {
    #[inline(always)]
    pub fn reg_offset_addr_vndr2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        EmmcPVndr2SpecificAreaReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            EmmcPVndr2SpecificAreaReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcPVndr2SpecificAreaReg {
    #[inline(always)]
    fn default() -> EmmcPVndr2SpecificAreaReg {
        <crate::RegValueT<EmmcPVndr2SpecificAreaReg_SPEC> as RegisterValue<_>>::new(384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcResp01RReg_SPEC;
impl crate::sealed::RegSpec for EmmcResp01RReg_SPEC {
    type DataType = u32;
}

pub type EmmcResp01RReg = crate::RegValueT<EmmcResp01RReg_SPEC>;

impl EmmcResp01RReg {
    #[inline(always)]
    pub fn resp01(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcResp01RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcResp01RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcResp01RReg {
    #[inline(always)]
    fn default() -> EmmcResp01RReg {
        <crate::RegValueT<EmmcResp01RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcResp23RReg_SPEC;
impl crate::sealed::RegSpec for EmmcResp23RReg_SPEC {
    type DataType = u32;
}

pub type EmmcResp23RReg = crate::RegValueT<EmmcResp23RReg_SPEC>;

impl EmmcResp23RReg {
    #[inline(always)]
    pub fn resp23(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcResp23RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcResp23RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcResp23RReg {
    #[inline(always)]
    fn default() -> EmmcResp23RReg {
        <crate::RegValueT<EmmcResp23RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcResp45RReg_SPEC;
impl crate::sealed::RegSpec for EmmcResp45RReg_SPEC {
    type DataType = u32;
}

pub type EmmcResp45RReg = crate::RegValueT<EmmcResp45RReg_SPEC>;

impl EmmcResp45RReg {
    #[inline(always)]
    pub fn resp45(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcResp45RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcResp45RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcResp45RReg {
    #[inline(always)]
    fn default() -> EmmcResp45RReg {
        <crate::RegValueT<EmmcResp45RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcResp67RReg_SPEC;
impl crate::sealed::RegSpec for EmmcResp67RReg_SPEC {
    type DataType = u32;
}

pub type EmmcResp67RReg = crate::RegValueT<EmmcResp67RReg_SPEC>;

impl EmmcResp67RReg {
    #[inline(always)]
    pub fn resp67(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcResp67RReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcResp67RReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcResp67RReg {
    #[inline(always)]
    fn default() -> EmmcResp67RReg {
        <crate::RegValueT<EmmcResp67RReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcSdmasaRReg_SPEC;
impl crate::sealed::RegSpec for EmmcSdmasaRReg_SPEC {
    type DataType = u32;
}

pub type EmmcSdmasaRReg = crate::RegValueT<EmmcSdmasaRReg_SPEC>;

impl EmmcSdmasaRReg {
    #[inline(always)]
    pub fn blockcnt_sdmasa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        EmmcSdmasaRReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            EmmcSdmasaRReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcSdmasaRReg {
    #[inline(always)]
    fn default() -> EmmcSdmasaRReg {
        <crate::RegValueT<EmmcSdmasaRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcSlotIntrStatusRReg_SPEC;
impl crate::sealed::RegSpec for EmmcSlotIntrStatusRReg_SPEC {
    type DataType = u16;
}

pub type EmmcSlotIntrStatusRReg = crate::RegValueT<EmmcSlotIntrStatusRReg_SPEC>;

impl EmmcSlotIntrStatusRReg {
    #[inline(always)]
    pub fn intr_slot(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        EmmcSlotIntrStatusRReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            EmmcSlotIntrStatusRReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for EmmcSlotIntrStatusRReg {
    #[inline(always)]
    fn default() -> EmmcSlotIntrStatusRReg {
        <crate::RegValueT<EmmcSlotIntrStatusRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcSwRstRReg_SPEC;
impl crate::sealed::RegSpec for EmmcSwRstRReg_SPEC {
    type DataType = u8;
}

pub type EmmcSwRstRReg = crate::RegValueT<EmmcSwRstRReg_SPEC>;

impl EmmcSwRstRReg {
    #[inline(always)]
    pub fn sw_rst_dat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcSwRstRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcSwRstRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_rst_cmd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcSwRstRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcSwRstRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_rst_all(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcSwRstRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcSwRstRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcSwRstRReg {
    #[inline(always)]
    fn default() -> EmmcSwRstRReg {
        <crate::RegValueT<EmmcSwRstRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcToutCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcToutCtrlRReg_SPEC {
    type DataType = u8;
}

pub type EmmcToutCtrlRReg = crate::RegValueT<EmmcToutCtrlRReg_SPEC>;

impl EmmcToutCtrlRReg {
    #[inline(always)]
    pub fn tout_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, EmmcToutCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,EmmcToutCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcToutCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcToutCtrlRReg {
        <crate::RegValueT<EmmcToutCtrlRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcWupCtrlRReg_SPEC;
impl crate::sealed::RegSpec for EmmcWupCtrlRReg_SPEC {
    type DataType = u8;
}

pub type EmmcWupCtrlRReg = crate::RegValueT<EmmcWupCtrlRReg_SPEC>;

impl EmmcWupCtrlRReg {
    #[inline(always)]
    pub fn card_removal(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcWupCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcWupCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_insert(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcWupCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcWupCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn card_int(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcWupCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcWupCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcWupCtrlRReg {
    #[inline(always)]
    fn default() -> EmmcWupCtrlRReg {
        <crate::RegValueT<EmmcWupCtrlRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcXferModeRReg_SPEC;
impl crate::sealed::RegSpec for EmmcXferModeRReg_SPEC {
    type DataType = u16;
}

pub type EmmcXferModeRReg = crate::RegValueT<EmmcXferModeRReg_SPEC>;

impl EmmcXferModeRReg {
    #[inline(always)]
    pub fn resp_int_disable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_err_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_type(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn multi_blk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn data_xfer_dir(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_cmd_enable(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, EmmcXferModeRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn block_count_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_en_emmc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmmcXferModeRReg {
    #[inline(always)]
    fn default() -> EmmcXferModeRReg {
        <crate::RegValueT<EmmcXferModeRReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
