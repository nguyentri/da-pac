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
#[doc = r"EMMC registers"]
unsafe impl ::core::marker::Send for super::Emmc {}
unsafe impl ::core::marker::Sync for super::Emmc {}
impl super::Emmc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "This register stores the ADMA state during an ADMA error. This register is applicable for an SD/eMMC/UHS-II mode."]
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

    #[doc = "This register holds the lower 32-bit system address for DMA transfer. This register is applicable for an SD/eMMC/UHS-II mode."]
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

    #[doc = "This register is used to configure the SD/eMMC command argument."]
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

    #[doc = "This register controls some aspects of tuning and auto-tuning features. Do not program this register when HOST_CTRL2_R.SAMPLE_C"]
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

    #[doc = "Register to read the Center, Left and Right codes used by tuning and auto-tuning engines. Center code field is also used for so"]
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

    #[doc = "This register is used to indicate the CMD12 response error of Auto CMD12, and the CMD23 response error of Auto CMD23. The Host"]
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

    #[doc = "This register is used by the host driver to control any operation related to Block Gap. This register is applicable for an SD/e"]
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

    #[doc = "This register is used to configure the number of data blocks. This register is applicable for both SD and eMMC modes."]
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

    #[doc = "This register is used to configure an SDMA buffer boundary and the number of bytes in a data block. This register is applicable"]
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

    #[doc = "This register is used to control the eMMC Boot operation."]
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

    #[doc = "This register is used to access the packet buffer. This register is applicable for an SD/eMMC/UHS-II mode."]
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

    #[doc = "This register provides the Host Driver with information specific to the Host Controller implementation. The host controller may"]
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

    #[doc = "This register provides the Host Driver with information specific to the Host Controller implementation. The host controller may"]
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

    #[doc = "This register controls SDCLK (card clock) in an SD/eMMC mode and RCLK in the UHS-II mode. This register is applicable for an SD"]
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

    #[doc = "This register is used to provide the information related to a command and a response packet. This register is applicable for an"]
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

    #[doc = "This register indicates the capabilities of the command queuing engine."]
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

    #[doc = "This register indicate the maximum current capability for each voltage, for VDD1. The value is meaningful if the Voltage Suppor"]
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

    #[doc = "This register indicates the maximum current capability for each voltage (for VDD2). The value is meaningful if Voltage Support"]
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

    #[doc = "This register controls the embedded device. When the Host Controller is connected to a removable device, this register is not u"]
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

    #[doc = "This register is used to control the eMMC operation."]
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

    #[doc = "This register is used to select the interrupt status that is notified to the Host System as an interrupt. All these status bits"]
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

    #[doc = "This register sets the Interrupt Status for Error Interrupt Status register (ERROR_INT_STAT_R), when ERROR_INT_STAT_EN_R is set"]
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

    #[doc = "This register enables an interrupt when the Error Interrupt Status Enable is enabled and at least one of the statuses is set to"]
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

    #[doc = "The register is not a physically implemented but is an address at which the Auto CMD Error Status register can be written.This"]
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

    #[doc = "This register is not physically implemented but is an address at which the Error Interrupt Status register can be written. The"]
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

    #[doc = "This register is used to indicate the Host Controller Version number."]
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

    #[doc = "This register is used to control the operation of the Host Controller. This register is applicable for an SD/eMMC/UHS-II mode."]
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

    #[doc = "This register is used to control how the Host Controller operates. This register is applicable for an SD/eMMC/UHS-II mode."]
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

    #[doc = "This register is used to select the valid burst types that the AHB Master bus interface can generate. \n\nWhen more than one bit"]
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

    #[doc = "This register is used to control the operation of MSHC Host Controller."]
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

    #[doc = "This register reflects the current release number of DWC_mshc/DWC_mshc_lite."]
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

    #[doc = "This register reflects the current release type of DWC_mshc/DWC_mshc_lite."]
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

    #[doc = "This register is used to select the interrupt status that is indicated to the Host System as the interrupt. All these status bi"]
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

    #[doc = "This register enables the Interrupt Status for Normal Interrupt Status register (NORMAL_INT_STAT_R) when NORMAL_INT_STAT_R is s"]
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

    #[doc = "This register reflects the status of the Normal Interrupt. This register is applicable for an SD/eMMC/UHS-II mode."]
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

    #[doc = "This register defines the Preset Value for DDR50 and High Speed DDR speed modes in the SD and eMMC modes, respectively."]
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

    #[doc = "This register defines Preset Value for Default Speed mode in SD mode."]
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

    #[doc = "This register defines Preset Value for High Speed mode in SD mode."]
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

    #[doc = "This register defines Preset Value for Initialization in SD/eMMC mode."]
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

    #[doc = "This register defines Preset Value for SDR104 and HS200 speed modes in the SD and eMMC modes, respectively."]
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

    #[doc = "This register defines Preset Value for SDR12 and Legacy speed mode in SD and eMMC mode respectively."]
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

    #[doc = "This register defines Preset Value for SDR25 and High Speed SDR speed mode in SD and eMMC mode respectively."]
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

    #[doc = "This register defines Preset Value for SDR50 speed mode in SD mode."]
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

    #[doc = "This register is used to hold the preset value for UHS-II and HS400 speed modes in the SD and eMMC modes, respectively."]
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

    #[doc = "This register indicates the present status of the Host Controller. This register is applicable for an SD/eMMC/UHS-II mode."]
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

    #[doc = "This register is used to control the bus power for the Card. This register is applicable for an SD, eMMC, and UHS-II modes."]
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

    #[doc = "This register points to the location of UHS-II embedded control registers."]
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

    #[doc = "This register used as a pointer for the Vendor Specific Area 1."]
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

    #[doc = "This register is used as a pointer for the Vendor Specific Area 2."]
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

    #[doc = "This register stores 39-08 bits of the Response Field for an SD/eMMC mode.  The response for an SD/eMMC command can be a maximu"]
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

    #[doc = "This register stores 71-40 bits of the Response Field for an SD/eMMC mode. This register is used to store the response from the"]
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

    #[doc = "This register stores 103-72 bits of the Response Field for an SD/eMMC mode.  The response for SD/eMMC command can be a maximum"]
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

    #[doc = "This register stores 135-104 bits of the Response Field for an SD/eMMC mode.    The SD/eMMC response can be a maximum of 128 bi"]
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

    #[doc = "This register is used to configure a 32-bit Block Count or an SDMA System Address based on the Host Version 4 Enable bit in the"]
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

    #[doc = "This register indicates the Interrupt status of each slot."]
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

    #[doc = "This register is used to generate a reset pulse by writing 1 to each bit of this register. After completing the reset, the Host"]
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

    #[doc = "This register is used to set the Data Timeout Counter value for an SD/eMMC mode according to the timer clock defined by the Cap"]
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

    #[doc = "This register is mandatory for the Host Controller, but the wakeup functionality depends on the Host Controller system hardware"]
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

    #[doc = "This register is used to control the operation of data transfers for an SD/eMMC mode. The Host driver sets this register before"]
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

#[doc = "This register stores the ADMA state during an ADMA error. This register is applicable for an SD/eMMC/UHS-II mode."]
pub type EmmcAdmaErrStatRReg = crate::RegValueT<EmmcAdmaErrStatRReg_SPEC>;

impl EmmcAdmaErrStatRReg {
    #[doc = "ADMA Length Mismatch Error States\nThis error occurs in the following instances:\n- While the Block Count Enable is being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length\n- When the total data length cannot be divided by the block length\nValues:\n0x0 (NO_ERR): No Error\n0x1 (ERROR): Error"]
    #[inline(always)]
    pub fn adma_len_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcAdmaErrStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcAdmaErrStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "ADMA Error States\nThese bits indicate the state of ADMA when an error occurs during ADMA data transfer.\nValues:\n0x0 (ST_STOP): Stop DMA - SYS_ADR register points to a location next to the error descriptor\n0x1 (ST_FDS): Fetch Descriptor - SYS_ADR register points to the error descriptor\n0x2 (UNUSED): Never set this state\n0x3 (ST_TFR): Transfer Data - SYS_ADR register points to a location next to the error descriptor"]
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

#[doc = "This register holds the lower 32-bit system address for DMA transfer. This register is applicable for an SD/eMMC/UHS-II mode."]
pub type EmmcAdmaSaLowRReg = crate::RegValueT<EmmcAdmaSaLowRReg_SPEC>;

impl EmmcAdmaSaLowRReg {
    #[doc = "ADMA System Address\n\nThese bits indicate the lower 32 bits of the ADMA system address.\n- SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location\n- ADMA2: This register stores the byte address of the executing command of the descriptor table\n- ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
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

#[doc = "This register is used to configure the SD/eMMC command argument."]
pub type EmmcArgumentRReg = crate::RegValueT<EmmcArgumentRReg_SPEC>;

impl EmmcArgumentRReg {
    #[doc = "Command Argument.\nThese bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
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

#[doc = "This register controls some aspects of tuning and auto-tuning features. Do not program this register when HOST_CTRL2_R.SAMPLE_C"]
pub type EmmcAtCtrlRReg = crate::RegValueT<EmmcAtCtrlRReg_SPEC>;

impl EmmcAtCtrlRReg {
    #[doc = "Sampling window threshold value setting\nThe maximum value that can be set here depends on the length of delayline used for tuning. A delayLine with 128 taps can use values from 0x0 to 0x7F.\nThis field is valid only when SWIN_TH_EN is \'1\'. Should be programmed only when SAMPLE_CLK_SEL is \'0\'\n     - 0x0 : Threshold values is 0x1, windows of length 1 tap and above can be selected as sampling window.\n     - 0x1 : Threshold values is 0x2, windows of length 2 taps and above can be selected as sampling window.\n     - 0x2 : Threshold values is 0x1, windows of length 3 taps and above can be selected as sampling window.\n     - ........\n     - 0x7F : Threshold values is 0x1, windows of length 127 taps and above can be selected as sampling window."]
    #[inline(always)]
    pub fn swin_th_val(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, u8, EmmcAtCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7,1,0,u8,u8,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Time taken for phase switching and stable clock output.\nSpecifies the maximum time (in terms of cclk cycles) that the delay line can take to switch its output phase after a change in tuning_cclk_sel or autotuning_cclk_sel.\nValues:\n0x0 (LATENCY_LT_1): Less than 1-cycle latency\n0x1 (LATENCY_LT_2): Less than 2-cycle latency\n0x2 (LATENCY_LT_3): Less than 3-cycle latency\n0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    #[inline(always)]
    pub fn post_change_dly(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, u8, EmmcAtCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x3,1,0,u8,u8,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maximum Latency specification between cclk_tx and cclk_rx.\nValues:\n0x0 (LATENCY_LT_1): Less than 1-cycle latency\n0x1 (LATENCY_LT_2): Less than 2-cycle latency\n0x2 (LATENCY_LT_3): Less than 3-cycle latency\n0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    #[inline(always)]
    pub fn pre_change_dly(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, EmmcAtCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock stopping control for Tuning and auto-tuning circuit.\nWhen enabled, clock gate control output of DWC_mshc (clk2card_on) is pulled low before changing phase select codes on tuning_cclk_sel and autotuning_cclk_sel. This effectively stops the Device/Card\nclock, cclk_rx and also drift_cclk_rx. Changing phase code when clocks are stopped ensures glitch free phase switching. Set this bit to 0 if the PHY or delayline can guarantee glitch free switching.\nValues:\n0x1 (ENABLE_CLK_STOPPING): Clocks stopped during phase code change\n0x0 (DISABLE_CLK_STOPPING): Clocks not stopped. PHY ensures glitch free phase switching."]
    #[inline(always)]
    pub fn tune_clk_stop_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This fields enables software-managed tuning flow.\nValues:\n0x1 (SW_TUNING_ENABLE): Software-managed tuning enabled. AT_STAT_R.CENTER_PH_CODE Field is now writable.\n0x0 (SW_TUNING_DISABLE): Software-managed tuning disabled."]
    #[inline(always)]
    pub fn sw_tune_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Framing errors are not generated when executing tuning. This debug bit allows users to report these errors.\nValues:\n0x1 (DEBUG_ERRORS): Debug mode for reporting framing errors\n0x0 (ERRORS_DISABLED): Default mode where as per SD-HCI no errors are reported."]
    #[inline(always)]
    pub fn rpt_tune_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcAtCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcAtCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sampling window Threshold enable\nSelects the tuning mode\nField should be programmed only when SAMPLE_CLK_SEL is \'0\'\nValues:\n0x1 (THRESHOLD_MODE): Tuning engine selects the first complete sampling window that meets the threshold set by SWIN_TH_VAL field\n0x0 (LARGEST_WIN_MODE): Tuning engine sweeps all taps and settles at the largest window"]
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

#[doc = "Register to read the Center, Left and Right codes used by tuning and auto-tuning engines. Center code field is also used for so"]
pub type EmmcAtStatRReg = crate::RegValueT<EmmcAtStatRReg_SPEC>;

impl EmmcAtStatRReg {
    #[doc = "Left Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Left edge of sampling window."]
    #[inline(always)]
    pub fn l_edge_ph_code(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, EmmcAtStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,EmmcAtStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Right Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Right edge of sampling window."]
    #[inline(always)]
    pub fn r_edge_ph_code(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EmmcAtStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EmmcAtStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Centered Phase code. Reading this field returns the current value on tuning_cclk_sel output. Setting AT_CTRL_R.SW_TUNE_EN enables software to write to this field and its contents are reflected on tuning_cclk_sel."]
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

#[doc = "This register is used to indicate the CMD12 response error of Auto CMD12, and the CMD23 response error of Auto CMD23. The Host"]
pub type EmmcAutoCmdStatRReg = crate::RegValueT<EmmcAutoCmdStatRReg_SPEC>;

impl EmmcAutoCmdStatRReg {
    #[doc = "Command Not Issued By Auto CMD12 Error\nIf this bit is set to 1, CMD_wo_DAT is not executed due to an Auto CMD12 Error (D04-D01) in this register.\nThis bit is set to 0 when Auto CMD Error is generated by Auto CMD23.\nValues:\n0x1 (TRUE): Not Issued\n0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn cmd_not_issued_auto_cmd12(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Auto CMD Response Error\nThis bit is set when Response Error Check Enable in the Transfer Mode register is set to 1 and an error is detected in R1 response of either Auto CMD12 or CMD13. This status is ignored if any bit between D00 to D04 is set to 1.\nValues:\n0x1 (TRUE): Error\n0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Auto CMD Index Error\nThis bit is set if the command index error occurs in response to a command.\nValues:\n0x1 (TRUE): Error\n0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Auto CMD End Bit Error\nThis bit is set when detecting that the end bit of command response is 0.\nValues:\n0x1 (TRUE): End Bit Error Generated\n0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_ebit_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Auto CMD CRC Error\nThis bit is set when detecting a CRC error in the command response.\nValues:\n0x1 (TRUE): CRC Error Generated\n0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Auto CMD Timeout Error\nThis bit is set if no response is returned with 64 SDCLK cycles from the end bit of the command.\nIf this bit is set to 1, error status bits (D04-D01) are meaningless.\nValues:\n0x1 (TRUE): Time out\n0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcAutoCmdStatRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcAutoCmdStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Auto CMD12 Not Executed\nIf multiple memory block data transfer is not started due to a command error, this bit is not set because it is not necessary to issue an Auto CMD12. Setting this bit to 1 means that the Host Controller cannot issue Auto CMD12 to stop multiple memory block data transfer, due to some error. If this bit is set to 1, error status bits (D04-D01) is meaningless.\nThis bit is set to 0 when Auto CMD Error is generated by Auto CMD23.\nValues:\n0x1 (TRUE): Not Executed\n0x0 (FALSE): Executed"]
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

#[doc = "This register is used by the host driver to control any operation related to Block Gap. This register is applicable for an SD/e"]
pub type EmmcBgapCtrlRReg = crate::RegValueT<EmmcBgapCtrlRReg_SPEC>;

impl EmmcBgapCtrlRReg {
    #[doc = "Interrupt At Block Gap.\nThis bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer.\n\nValues:\n\n0x0 (DISABLE): Disabled\n0x1 (ENABLE): Enabled"]
    #[inline(always)]
    pub fn int_at_bgap(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcBgapCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Read Wait Control.\nThis bit is used to enable the read wait protocol to stop read data using DAT\\[2\\] line if the card supports read wait. Otherwise, the Host Controller has to stop the card clock to hold the read data. \n\nValues:\n0x0 (DISABLE): Disable Read Wait Control\n0x1 (ENABLE): Enable Read Wait Control"]
    #[inline(always)]
    pub fn rd_wait_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcBgapCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Continue Request.\nThis bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored.\n\nValues:\n0x0 (NO_AFFECT): No Affect\n0x1 (RESTART): Restart"]
    #[inline(always)]
    pub fn continue_req(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcBgapCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcBgapCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Stop At Block Gap Request.\nThis bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers.\n\nValues:\n0x0 (XFER): Transfer\n0x1 (STOP): Stop"]
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

#[doc = "This register is used to configure the number of data blocks. This register is applicable for both SD and eMMC modes."]
pub type EmmcBlockcountRReg = crate::RegValueT<EmmcBlockcountRReg_SPEC>;

impl EmmcBlockcountRReg {
    #[doc = "16-bit Block Count.\n- If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected.\n- If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected.\nFollowing are the values for BLOCK_CNT:\n- 0x0: Stop Count\n- 0x1: 1 Block\n- 0x2: 2 Blocks\n- ... - ...\n- 0xFFFF: 65535 Blocks\nNote: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
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

#[doc = "This register is used to configure an SDMA buffer boundary and the number of bytes in a data block. This register is applicable"]
pub type EmmcBlocksizeRReg = crate::RegValueT<EmmcBlocksizeRReg_SPEC>;

impl EmmcBlocksizeRReg {
    #[doc = "SDMA Buffer Boundary.\nThese bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register.\n\nValues:\n0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary\n0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary\n0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary\n0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary\n0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary\n0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary\n0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary\n0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
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

    #[doc = "Transfer Block Size.\nThese bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return\nan invalid value, and write operations are ignored.\nFollowing are the values for XFER_BLOCK_SIZE:\n- 0x1: 1 byte\n- 0x2: 2 bytes\n- 0x3: 3 bytes\n- ...... \n- 0x1FF: 511 byte\n- 0x200: 512 bytes\n- ......   \n- 0x800: 2048 bytes\nNote: This register must be programmed with a non-zero value for data transfer."]
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

#[doc = "This register is used to control the eMMC Boot operation."]
pub type EmmcBootCtrlRReg = crate::RegValueT<EmmcBootCtrlRReg_SPEC>;

impl EmmcBootCtrlRReg {
    #[doc = "Boot Ack Timeout Counter Value.\nThis value determines the interval by which boot ack timeout (50 ms) is detected when boot ack is expected during boot operation.\n- 0xF : Reserved\n- 0xE : TMCLK x 2^27\n- ..   - ............\n- 0x1 : TMCLK x 2^14\n- 0x0 : TMCLK x 2^13"]
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

    #[doc = "Boot Acknowledge Enable\nWhen this bit set, DWC_mshc checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode.\nValues:\n0x1 (TRUE): Boot Ack enable\n0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcBootCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcBootCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Validate Mandatory Boot Enable bit\nThis bit is used to validate the MAN_BOOT_EN bit.\nValues:\n0x1 (TRUE): Validate Mandatory boot enable bit\n0x0 (FALSE): Ignore Mandatory boot Enable bit"]
    #[inline(always)]
    pub fn validate_boot(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcBootCtrlRReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcBootCtrlRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Mandatory Boot Enable\nThis bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The DWC_mshc clears this bit after the boot transfer is completed or terminated.\nValues:\n0x1 (MAN_BOOT_EN): Mandatory boot enable\n0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
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

#[doc = "This register is used to access the packet buffer. This register is applicable for an SD/eMMC/UHS-II mode."]
pub type EmmcBufDataRReg = crate::RegValueT<EmmcBufDataRReg_SPEC>;

impl EmmcBufDataRReg {
    #[doc = "Buffer Data.\nThese bits enable access to the Host Controller packet buffer."]
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

#[doc = "This register provides the Host Driver with information specific to the Host Controller implementation. The host controller may"]
pub type EmmcCapabilities1RReg = crate::RegValueT<EmmcCapabilities1RReg_SPEC>;

impl EmmcCapabilities1RReg {
    #[doc = "Slot Type\nThese bits indicate usage of a slot by a specific Host System.\nValues:\n0x0 (REMOVABLE_SLOT): Removable Card Slot\n0x1 (EMBEDDED_SLOT): Embedded Slot for one Device\n0x2 (SHARED_SLOT): Shared Bus Slot (SD mode)\n0x3 (UHS2_EMBEDDED_SLOT): UHS-II MultipleEmbedded Devices"]
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

    #[doc = "Asynchronous Interrupt Support (SD Mode only)\nValues:\n0x0 (FALSE): Asynchronous Interrupt Not Supported\n0x1 (TRUE): Asynchronous Interrupt Supported"]
    #[inline(always)]
    pub fn async_int_support(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<29,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "64-bit System Address Support for V3\nThis bit sets the Host controller to support 64-bit System Addressing of V3 mode.\nSDMA cannot be used in 64-bit Addressing in Version 3 Mode.\nIf this bit is set to 1, 64-bit ADMA2 with using 96-bit Descriptor can be enabled by setting Host Version 4 Enable (HOST_VER4_ENABLE = 0) and DMA select (DMA_SEL = 11b).\nValues:\n0x0 (FALSE): 64-bit System Address for V3 is Not Supported\n0x1 (TRUE): 64-bit System Address for V3 is Supported"]
    #[inline(always)]
    pub fn sys_addr_64_v3(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<28,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "64-bit System Address Support for V4\nThis bit sets the Host Controller to support 64-bit System Addressing of V4 mode. When this bit is set to 1, full or part of 64-bit address must be used to decode the Host Controller Registers so that Host Controller Registers can be placed above system memory area. 64-bit address decode of Host Controller registers is effective regardless of setting to 64-bit Addressing in Host Control 2.\nIf this bit is set to 1, 64-bit DMA Addressing for version 4 is enabled by setting Host Version 4 Enable\n(HOST_VER4_ENABLE = 1) and by setting 64-bit Addressing (ADDRESSING =1) in the Host Control 2 register. SDMA can be used and ADMA2 uses 128-bit Descriptor.\nValues:\n0x0 (FALSE): 64-bit System Address for V4 is Not Supported\n0x1 (TRUE): 64-bit System Address for V4 is Supported"]
    #[inline(always)]
    pub fn sys_addr_64_v4(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<27,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Voltage Support for 1.8V\nValues:\n0x0 (FALSE): 1.8V Not Supported\n0x1 (TRUE): 1.8V Supported"]
    #[inline(always)]
    pub fn volt_18(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<26,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Voltage Support for SD 3.0V or Embedded 1.2V\nValues:\n0x0 (FALSE): SD 3.0V or Embedded 1.2V Not Supported\n0x1 (TRUE): SD 3.0V or Embedded Supported"]
    #[inline(always)]
    pub fn volt_30(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<25,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Voltage Support for 3.3V\nValues:\n0x0 (FALSE): 3.3V Not Supported\n0x1 (TRUE): 3.3V Supported"]
    #[inline(always)]
    pub fn volt_33(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<24,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Suspense/Resume Support\nThis bit indicates whether the Host Controller supports Suspend/Resume functionality. If this bit is 0, the Host Driver does not issue either Suspend or Resume commands because the Suspend and Resume mechanism is not supported.\nValues:\n0x0 (FALSE): Not Supported\n0x1 (TRUE): Supported"]
    #[inline(always)]
    pub fn sus_res_support(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<23,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDMA Support\nThis bit indicates whether the Host Controller is capable of using SDMA to transfer data between the system memory and the Host Controller directly.\nValues:\n0x0 (FALSE): SDMA not Supported\n0x1 (TRUE): SDMA Supported"]
    #[inline(always)]
    pub fn sdma_support(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<22,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "High Speed Support\nThis bit indicates whether the Host Controller and the Host System supports High Speed mode and they can supply the SD Clock frequency from 25 MHz to 50 MHz.\nValues:\n0x0 (FALSE): High Speed not Supported\n0x1 (TRUE): High Speed Supported"]
    #[inline(always)]
    pub fn high_speed_support(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<21,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "ADMA2 Support\nThis bit indicates whether the Host Controller is capable of using ADMA2.\nValues:\n0x0 (FALSE): ADMA2 not Supported\n0x1 (TRUE): ADMA2 Supported"]
    #[inline(always)]
    pub fn adma2_support(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<19,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "8-bit Support for Embedded Device\nThis bit indicates whether the Host Controller is capable of using an 8-bit bus width mode. This bit is not effective when the Slot Type is set to 10b.\nValues:\n0x0 (FALSE): 8-bit Bus Width not Supported\n0x1 (TRUE): 8-bit Bus Width Supported"]
    #[inline(always)]
    pub fn embedded_8_bit(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<18,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Maximum Block Length\nThis bit indicates the maximum block size that the Host driver can read and write to the buffer in the Host Controller. The buffer transfers this block size without wait cycles. The transfer block length is always 512 bytes for the SD Memory irrespective of this bit\nValues:\n0x0 (ZERO): 512 Byte\n0x1 (ONE): 1024 Byte\n0x2 (TWO): 2048 Byte\n0x3 (THREE): Reserved"]
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

    #[doc = "Base Clock Frequency for SD clock\nThese bits indicate the base (maximum) clock frequency for the SD Clock. The definition of these bits depend on the Host Controller Version.\n- 6-Bit Base Clock Frequency: This mode is supported by the Host Controller version 1.00 and 2.00. The upper 2 bits are not effective and are always 0. The unit values are 1 MHz. The supported clock range is 10 MHz to 63 MHz.\n-- 0x00 : Get information through another method\n-- 0x01 : 1 MHz\n-- 0x02 : 2 MHz\n-- .............\n-- 0x3F : 63 MHz\n-- 0x40-0xFF : Not Supported\n- 8-Bit Base Clock Frequency: This mode is supported by the Host Controller version 3.00. The unit values are 1 MHz. The supported clock range is 10 MHz to 255 MHz.\n-- 0x00 : Get information through another method\n-- 0x01 : 1 MHz\n-- 0x02 : 2 MHz\n-- ............\n-- 0xFF : 255 MHz\nIf the frequency is 16.5 MHz, the larger value is set to 0001001b (17 MHz) because the Host Driver uses this value to calculate the clock divider value and it does not exceed the upper limit of the SD Clock frequency. If these bits are all 0, the Host system has to get information using a different method."]
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

    #[doc = "Timeout Clock Unit\nThis bit shows the unit of base clock frequency used to detect Data TImeout Error.\nValues:\n0x0 (KHZ): KHz\n0x1 (MHZ): MHz"]
    #[inline(always)]
    pub fn tout_clk_unit(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcCapabilities1RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcCapabilities1RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Timeout Clock Frequency\n\nThis bit shows the base clock frequency used to detect Data Timeout Error. The Timeout Clock unit defines the unit of timeout clock frequency. It can be KHz or MHz.\n- 0x00 : Get information through another method\n- 0x01 :  1KHz /  1MHz\n- 0x02 :  2KHz /  2MHz\n- 0x03 :  3KHz /  3MHz\n-  ...........\n- 0x3F : 63KHz / 63MHz"]
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

#[doc = "This register provides the Host Driver with information specific to the Host Controller implementation. The host controller may"]
pub type EmmcCapabilities2RReg = crate::RegValueT<EmmcCapabilities2RReg_SPEC>;

impl EmmcCapabilities2RReg {
    #[doc = "1.8V VDD2 Support\nThis bit indicates support of VDD2 for the Host System.\nValues:\n0x0 (FALSE): 1.8V VDD2 is not Supported\n0x1 (TRUE): 1.8V VDD2 is Supported"]
    #[inline(always)]
    pub fn vdd2_18v_support(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<28,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "ADMA3 Support\nThis bit indicates whether the Host Controller is capable of using ADMA3.\nValues:\n0x0 (FALSE): ADMA3 not Supported\n0x1 (TRUE): ADMA3 Supported"]
    #[inline(always)]
    pub fn adma3_support(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<27,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Clock Multiplier\nThese bits indicate the clock multiplier of the programmable clock generator. Setting these bits to 0 means that the Host Controller does not support a programmable clock generator.\n- 0x0: Clock Multiplier is not Supported\n- 0x1: Clock Multiplier M = 2\n- 0x2: Clock Multiplier M = 3\n-   .........\n- 0xFF: Clock Multiplier M = 256"]
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

    #[doc = "Re-Tuning Modes (UHS-I only)\nThese bits select the re-tuning method and limit the maximum data length.\nValues:\n0x0 (MODE1): Timer\n0x1 (MODE2): Timer and Re-Tuning Request (Not supported)\n0x2 (MODE3): Auto Re-Tuning (for transfer)\n0x3 (RSVD_MODE): Reserved"]
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

    #[doc = "Use Tuning for SDR50 (UHS-I only)"]
    #[inline(always)]
    pub fn use_tuning_sdr50(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Timer Count for Re-Tuning (UHS-I only)\n- 0x0: Re-Tuning Timer disabled\n- 0x1: 1 seconds\n- 0x2: 2 seconds\n- 0x3: 4 seconds\n-  ........\n- 0xB: 1024 seconds\n- 0xC: Reserved\n- 0xD: Reserved\n- 0xE: Reserved\n- 0xF: Get information from other source"]
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

    #[doc = "Driver Type D Support (UHS-I only)\nThis bit indicates support of Driver Type D for 1.8 Signaling.\nValues:\nDescription\n0x0 (FALSE): Driver Type D is not supported\n0x1 (TRUE): Driver Type D is supported"]
    #[inline(always)]
    pub fn drv_typed(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Driver Type C Support (UHS-I only)\nThis bit indicates support of Driver Type C for 1.8 Signaling.\nValues:\n0x0 (FALSE): Driver Type C is not supported\n0x1 (TRUE): Driver Type C is supported"]
    #[inline(always)]
    pub fn drv_typec(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Driver Type A Support (UHS-I only)\nThis bit indicates support of Driver Type A for 1.8 Signaling.\nValues:\n0x0 (FALSE): Driver Type A is not supported\n0x1 (TRUE): Driver Type A is supported"]
    #[inline(always)]
    pub fn drv_typea(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "UHS-II Support (UHS-II only)\nThis bit indicates whether Host Controller supports UHS-II.\nValues:\n0x0 (FALSE): UHS-II is not supported\n0x1 (TRUE): UHS-II is supported"]
    #[inline(always)]
    pub fn uhs2_support(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DDR50 Support (UHS-I only)\nValues:\n0x0 (FALSE): DDR50 is not supported\n0x1 (TRUE): DDR50 is supported"]
    #[inline(always)]
    pub fn ddr50_support(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDR104 Support (UHS-I only)\nThis bit mentions that SDR104 requires tuning.\nValues:\n0x0 (FALSE): SDR104 is not supported\n0x1 (TRUE): SDR104 is supported"]
    #[inline(always)]
    pub fn sdr104_support(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcCapabilities2RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcCapabilities2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDR50 Support (UHS-I only)\nThsi bit indicates that SDR50 is supported. The bit 13 (USE_TUNING_SDR50) indicates whether SDR50 requires tuning or not.\nValues:\n0x0 (FALSE): SDR50 is not supported\n0x1 (TRUE): SDR50 is supported"]
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

#[doc = "This register controls SDCLK (card clock) in an SD/eMMC mode and RCLK in the UHS-II mode. This register is applicable for an SD"]
pub type EmmcClkCtrlRReg = crate::RegValueT<EmmcClkCtrlRReg_SPEC>;

impl EmmcClkCtrlRReg {
    #[doc = "SDCLK/RCLK Frequency Select.\nThese bits are used to select the frequency of the SDCLK signal. These bits depend on setting of Preset Value Enable in the Host Control 2 register. If Preset Value Enable = 0, these bits are set by the Host Driver. If Preset Value Enable = 1, these bits are automatically set to a value specified in one of the Preset Value register. The value is reflected on the lower 8-bit of the card_clk_freq_sel signal. \n10-bit Divided Clock Mode:\n- 0x3FF : 1/2046 Divided clock\n- N    : 1/2N Divided Clock\n- 0x002  : 1/4 Divided Clock\n- 0x001 : 1/2 Divided Clock\n- 0x000 : Base clock (10MHz - 255 MHz)\nProgrammable Clock Mode : Enables the Host System to select a fine grain SD clock frequency:\n- 0x3FF : Base clock  * M /1024\n- N-1  : Base clock  * M /N\n- 0x002  : Base clock  * M /3\n- 0x001 : Base clock  * M /2\n- 0x000 : Base clock  * M"]
    #[inline(always)]
    pub fn freq_sel(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EmmcClkCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "These bits specify the upper 2 bits of 10-bit SDCLK/RCLK Frequency Select control. The value is reflected on the upper 2 bits of the card_clk_freq_sel signal."]
    #[inline(always)]
    pub fn upper_freq_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, EmmcClkCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock Generator Select.\nThis bit is used to select the clock generator mode in SDCLK/RCLK Frequency Select. If Preset Value Enable = 0, this bit is set by the Host Driver. If Preset Value Enable = 1, this bit is automatically set to a value specified in one of the Preset Value registers. The value is reflected on the card_clk_gen_sel signal.\n\nValues:\n0x0 (FALSE): Divided Clock Mode\n0x1 (TRUE): Programmable Clock Mode"]
    #[inline(always)]
    pub fn clk_gen_select(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PLL Enable.\nThis bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). When Host Version 4 Enable = 0, INTERNAL_CLK_EN bit may be used to activate PLL. The value is reflected on the card_clk_en signal.\nNote: If this bit is not used to to active the PLL when Host Version 4 Enable = 1, it is recommended to set this bit to \'1\' .\n\nValues:\n0x0 (FALSE): PLL is in low power mode\n0x1 (TRUE): PLL is enabled"]
    #[inline(always)]
    pub fn pll_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SD/eMMC Clock Enable.\nThis bit stops the SDCLK or RCLK when set to 0. The SDCLK/RCLK Frequency Select bit can be changed when this bit is set to 0.\nThe value is reflected on the clk2card_on pin.\n\nValues:\n0x0 (FALSE): Disable providing SDCLK/RCLK\n0x1 (TRUE): Enable providing SDCLK/RCLK"]
    #[inline(always)]
    pub fn sd_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcClkCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Internal Clock Stable\nThis bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the intclk_stable signal after the Internal Clock Enable bit is set to 1 and also reflects the synchronized value of the card_clk_stable signal after the PLL Enable bit is set to 1.\n\nValues:\n0x0 (FALSE): Not Ready\n0x1 (TRUE): Ready"]
    #[inline(always)]
    pub fn internal_clk_stable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcClkCtrlRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcClkCtrlRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Internal Clock Enable.\nThis bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. However, registers can still be read and written to. The value is reflected on the intclk_en signal.\nNote: If this bit is not used to control the internal clock (base clock and master clock), it is recommended to set this bit to \'1\'.\n\nValues:\n0x0 (FALSE): Stop\n0x1 (TRUE): Oscillate"]
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

#[doc = "This register is used to provide the information related to a command and a response packet. This register is applicable for an"]
pub type EmmcCmdRReg = crate::RegValueT<EmmcCmdRReg_SPEC>;

impl EmmcCmdRReg {
    #[doc = "Command Index.\nThese bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, EmmcCmdRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Type.\nThese bits indicate the command type.\nNote: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3.\n\nValues:\n0x3 (ABORT_CMD): Abort\n0x2 (RESUME_CMD): Resume\n0x1 (SUSPEND_CMD): Suspend\n0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    pub fn cmd_type(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, EmmcCmdRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Present Select.\nThis bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line.\nThis bit is set to 0 in the following instances:\n- Command using the CMD line\n- Command with no data transfer but using busy signal on the DAT\\[0\\] line\n- Resume Command\n\nValues:\n0x0 (NO_DATA): No Data Present\n0x1 (DATA): Data Present"]
    #[inline(always)]
    pub fn data_present_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Index Check Enable.\nThis bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error.\nNote:\n- Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response.\n- For the tuning command, this bit must always be set to enable the index check.\n\nValues:\n0x0 (DISABLED): Disable\n0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command CRC Check Enable.\nThis bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error.\nNote:\n- CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response.\n- For the tuning command, this bit must always be set to 1 to enable the CRC check.\n\nValues:\n0x0 (DISABLED): Disable\n0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sub Command Flag.\nThis bit distinguishes between a main command and a sub command.\n\nValues:\n0x0 (MAIN): Main Command\n0x1 (SUB): Sub Command"]
    #[inline(always)]
    pub fn sub_cmd_flag(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcCmdRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcCmdRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Type Select.\nThis bit indicates the type of response expected from the card.\n\nValues:\n0x0 (NO_RESP): No Response\n0x1 (RESP_LEN_136): Response Length 136\n0x2 (RESP_LEN_48): Response Length 48\n0x3 (RESP_LEN_48B): Response Length 48; Check\nBusy after response"]
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

#[doc = "This register indicates the capabilities of the command queuing engine."]
pub type EmmcCqcapReg = crate::RegValueT<EmmcCqcapReg_SPEC>;

impl EmmcCqcapReg {
    #[doc = "Crypto Support   \nThis bit indicates whether the Host Controller supports cryptographic operations.\nValues:\n0x0 (FALSE): Crypto not Supported\n0x1 (TRUE): Crypto Supported"]
    #[inline(always)]
    pub fn crypto_support(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcCqcapReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,EmmcCqcapReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "These bits \\[27:16\\] of the CQCAP register are reserved. They always return 0."]
    #[inline(always)]
    pub fn cqccap_rsvd2(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, EmmcCqcapReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,EmmcCqcapReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Internal Timer Clock Frequency Multiplier (ITCFMUL)\nThis field indicates the frequency of the clock used for interrupt coalescing timer and for\ndetermining the SQS polling period. See ITCFVAL definition for details. Values 0x5 to 0xF are reserved.\nValues:\n0x0 (CLK_1KHz): 1KHz clock\n0x1 (CLK_10KHz): 10KHz clock\n0x2 (CLK_100KHz): 100KHz clock\n0x3 (CLK_1MHz): 1MHz clock\n0x4 (CLK_10MHz): 10MHz clock"]
    #[inline(always)]
    pub fn itcfmul(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, EmmcCqcapReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,EmmcCqcapReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Internal Timer Clock Frequency Value (ITCFVAL)\nThis field scales the frequency of the timer clock provided by ITCFMUL. The Final clock frequency of actual timer clock is calculated as ITCFVAL* ITCFMUL."]
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

#[doc = "This register indicate the maximum current capability for each voltage, for VDD1. The value is meaningful if the Voltage Suppor"]
pub type EmmcCurrCapabilities1RReg = crate::RegValueT<EmmcCurrCapabilities1RReg_SPEC>;

impl EmmcCurrCapabilities1RReg {
    #[doc = "Maximum Current for 1.8V\nThis bit specifies the Maximum Current for 1.8V VDD1 power supply for the card.\n- 0: Get information through another method\n- 1: 4mA\n- 2: 8mA\n- 3: 13mA\n- .......\n- 255: 1020mA"]
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

    #[doc = "Maximum Current for 3.0V\nThis bit specifies the Maximum Current for 3.0V VDD1 power supply for the card.\n- 0: Get information through another method\n- 1: 4mA\n- 2: 8mA\n- 3: 13mA\n- .......\n- 255: 1020mA"]
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

    #[doc = "Maximum Current for 3.3V\nThis bit specifies the Maximum Current for 3.3V VDD1 power supply for the card.\n- 0: Get information through another method\n- 1: 4mA\n- 2: 8mA\n- 3: 13mA\n- .......\n- 255: 1020mA"]
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

#[doc = "This register indicates the maximum current capability for each voltage (for VDD2). The value is meaningful if Voltage Support"]
pub type EmmcCurrCapabilities2RReg = crate::RegValueT<EmmcCurrCapabilities2RReg_SPEC>;

impl EmmcCurrCapabilities2RReg {
    #[doc = "Maximum Current for 1.8V VDD2\nThis bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card.\n- 0: Get information through another method\n- 1: 4mA\n- 2: 8mA\n- 3: 13mA\n- .......\n- 255: 1020mA"]
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

#[doc = "This register controls the embedded device. When the Host Controller is connected to a removable device, this register is not u"]
pub type EmmcEmbeddedCtrlRReg = crate::RegValueT<EmmcEmbeddedCtrlRReg_SPEC>;

impl EmmcEmbeddedCtrlRReg {
    #[doc = "Back-End Power Control (SD Mode)\n\nEach bit of this field controls back-end power supply for an embedded device.\n- 0 : Back-End Power is off\n- 1 : Back-End Power is supplied\n\nD24 : Back-End Power for Device 1\n\nD25 : Back-End Power for Device 2\n\nD26 : Back-End Power for Device 3\n\nD27 : Back-End Power for Device 4\n\nD28 : Back-End Power for Device 5\n\nD29 : Back-End Power for Device 6\n\nD30 : Back-End Power for Device 7"]
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

    #[doc = "Interrupt Pin Select\n\nThese bits enable the interrupt pin inputs.\n- 000 : Interrupts (INT_A,INT_B,INT_C) are disabled\n- xx1 : INT_A is enabled\n- x1x : INT_B is enabled\n- 1xx : INT_C is enabled"]
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

    #[doc = "Clock Pin Select (SD Mode)\n\nThis bit is selected by one of clock pin outputs.\n- 0x0 : Clock pins are disabled\n- 0x1 : CLK\\[1\\] is selected\n- 0x2 : CLK\\[2\\] is selected\n-  .      . \n-  .      . \n-  .      . \n- 0x7 : CLK\\[7\\] is selected"]
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

    #[doc = "Bus Width Preset (SD Mode)\n\nEach bit of this field specifies the bus width for each embedded device. The shared bus supports mixing of 4-bit and 8-bit bus width devices.\n- D08 : Bus Width Preset for Device 1\n- D09 : Bus Width Preset for Device 2\n- D10 : Bus Width Preset for Device 3\n- D11 : Bus Width Preset for Device 4\n- D12 : Bus Width Preset for Device 5\n- D13 : Bus Width Preset for Device 6\n- D14 : Bus Width Preset for Device 7\n\nFunction of each bit is defined as follows:\n- 0 : 4-bit bus width mode\n- 1 : 8-bit bus width mode"]
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

    #[doc = "Number of Interrupt Input Pins\n\nThis field indicates support of interrupt input pins for an embedded system."]
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

    #[doc = "Number of Clock Pins (SD Mode)\n\nThis field indicates support of clock pins to select one of devices for shared bus system. Up to 7 clock pins can be supported.\n- 0x0 : Shared bus is not supported\n- 0x1 : 1 SDCLK is supported\n- 0x2 - 2 SDCLK is supported\n-  .    .\n-  .    .\n-  .    .\n- 0x7 : 7 SDCLK is supported"]
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

#[doc = "This register is used to control the eMMC operation."]
pub type EmmcEmmcCtrlRReg = crate::RegValueT<EmmcEmmcCtrlRReg_SPEC>;

impl EmmcEmmcCtrlRReg {
    #[doc = "Output Enable control for EMMC Device Reset signal PAD control.\nThis field drived sd_rst_n_oe output of DWC_mshc\nValues:\n0x1 (ENABLE): sd_rst_n_oe is 1\n0x0 (DISABLE): sd_rst_n_oe is 0"]
    #[inline(always)]
    pub fn emmc_rst_n_oe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcEmmcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "EMMC Device Reset signal control.\nThis register field controls the sd_rst_n output of DWC_mshc\nValues:\n0x1 (RST_DEASSERT): Reset to eMMC device is deasserted\n0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    pub fn emmc_rst_n(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcEmmcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Disable Data CRC Check\nThis bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing.\nValues:\n0x1 (DISABLE): DATA CRC check is disabled\n0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    pub fn disable_data_crc_chk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcEmmcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcEmmcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "eMMC Card present\nThis bit indicates the type of card connected. An application program this bit based on the card connected to MSHC.\nValues:\n0x1 (EMMC_CARD): Card connected to MSHC is an eMMC card\n0x0 (NON_EMMC_CARD): Card connected to MSHC is a non-eMMC card"]
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

#[doc = "This register is used to select the interrupt status that is notified to the Host System as an interrupt. All these status bits"]
pub type EmmcErrorIntSignalEnRReg = crate::RegValueT<EmmcErrorIntSignalEnRReg_SPEC>;

impl EmmcErrorIntSignalEnRReg {
    #[doc = "The 16th bit of Error Interrupt Signal Enable is reserved.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn vendor_err_signal_en3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The 15th bit of Error Interrupt Signal Enable is reserved.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn vendor_err_signal_en2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The 14th bit of Error Interrupt Signal Enable is reserved.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn vendor_err_signal_en1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Boot Acknowledgment Error (eMMC Mode only).\nSetting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Error Signal Enable (SD Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Tuning Error Signal Enable (UHS-I Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn tuning_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ADMA Error Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Auto CMD Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Current Limit Error Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data End Bit Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data CRC Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Timeout Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Index Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_idx_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command End Bit Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command CRC Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcErrorIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcErrorIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Timeout Error Signal Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

#[doc = "This register sets the Interrupt Status for Error Interrupt Status register (ERROR_INT_STAT_R), when ERROR_INT_STAT_EN_R is set"]
pub type EmmcErrorIntStatEnRReg = crate::RegValueT<EmmcErrorIntStatEnRReg_SPEC>;

impl EmmcErrorIntStatEnRReg {
    #[doc = "The 15th bit of Error Interrupt Status Enable register is reserved.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn vendor_err_stat_en3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The 14th bit of Error Interrupt Status Enable register is reserved.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn vendor_err_stat_en2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The 13th bit of Error Interrupt Status Enable register is reserved.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn vendor_err_stat_en1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Boot Acknowledgment Error (eMMC Mode only)\nSetting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R).\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Error Status Enable (SD Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Tuning Error Status Enable (UHS-I Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn tuning_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ADMA Error Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Auto CMD Error Status Enable (SD/eMMC Mode only).\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Current Limit Error Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data End Bit Error Status Enable (SD/eMMC Mode only).\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data CRC Error Status Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Timeout Error Status Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Index Error Status Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_idx_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command End Bit Error Status Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command CRC Error Status Enable (SD/eMMC Mode only)\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcErrorIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcErrorIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Timeout Error Status Enable (SD/eMMC Mode only).\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

#[doc = "This register enables an interrupt when the Error Interrupt Status Enable is enabled and at least one of the statuses is set to"]
pub type EmmcErrorIntStatRReg = crate::RegValueT<EmmcErrorIntStatRReg_SPEC>;

impl EmmcErrorIntStatRReg {
    #[doc = "Boot Acknowledgement Error.\nThis bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn boot_ack_err(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Error.\nHost Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Tuning Error.\nThis bit is set when an unrecoverable error is detected in a tuning circuit except during the tuning procedure (occurrence of an error during tuning procedure is indicated by Sampling Clock Select in the Host Control 2 register). By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning. To reset tuning circuit, Sampling Clock Select is set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Tuning Error, the Host Driver must discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from the tuning circuit error. This is applicable in SD/eMMC mode.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn tuning_err(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ADMA Error.\nThis bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons:\n- Error response received from System bus (Master I/F)\n- ADMA3,ADMA2 Descriptors invalid\n- CQE Task or Transfer descriptors invalid\nWhen the error occurs, the state of the ADMA is saved in the ADMA Error Status register.\nIn eMMC CQE mode:\nThe Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn adma_err(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Auto CMD Error.\nThis error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn auto_cmd_err(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Current Limit Error.\nBy setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can\nbe protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying\npower to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to\ndetect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Power Fail"]
    #[inline(always)]
    pub fn cur_lmt_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data End Bit Error.\nThis error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data CRC Error.\nThis error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Timeout Error.\nThis bit is set in SD/eMMC mode when detecting one of the following timeout conditions:\n- Busy timeout for R1b, R5b type\n- Busy timeout after Write CRC status\n- Write CRC Status timeout\n- Read Data timeout\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn data_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Index Error.\nThis bit is set if a Command Index error occurs in the command respons in SD/eMMC mode.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command End Bit Error.\nThis bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. \n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    pub fn cmd_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command CRC Error.\nCommand CRC Error is generated in SD/eMMC mode for following two cases.\n- If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response.\n- The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    pub fn cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcErrorIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcErrorIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Timeout Error.\nIn SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles.\n\nValues:\n0x0 (FALSE): No error\n0x1 (TRUE): Time out"]
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

#[doc = "The register is not a physically implemented but is an address at which the Auto CMD Error Status register can be written.This"]
pub type EmmcForceAutoCmdStatRReg = crate::RegValueT<EmmcForceAutoCmdStatRReg_SPEC>;

impl EmmcForceAutoCmdStatRReg {
    #[doc = "Force Event for Command Not Issued By Auto CMD12 Error\nValues:\n0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set\n0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_cmd_not_issued_auto_cmd12(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Auto CMD Response Error\nValues:\n0x1 (TRUE): Auto CMD Response Error Status is set\n0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Auto CMD Index Error\nValues:\n0x1 (TRUE): Auto CMD Index Error Status is set\n0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Auto CMD End Bit Error\nValues:\n0x1 (TRUE): Auto CMD End Bit Error Status is set\n0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_ebit_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Auto CMD CRC Error\nValues:\n0x1 (TRUE): Auto CMD CRC Error Status is set\n0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Auto CMD Timeout Error\nValues:\n0x1 (TRUE): Auto CMD Timeout Error Status is set\n0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcForceAutoCmdStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcForceAutoCmdStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Auto CMD12 Not Executed\nValues:\n0x1 (TRUE): Auto CMD12 Not Executed Status is set\n0x0 (FALSE): Not Affected"]
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

#[doc = "This register is not physically implemented but is an address at which the Error Interrupt Status register can be written. The"]
pub type EmmcForceErrorIntStatRReg = crate::RegValueT<EmmcForceErrorIntStatRReg_SPEC>;

impl EmmcForceErrorIntStatRReg {
    #[doc = "This bit (FORCE_VENDOR_ERR3) of the FORCE_ERROR_INT_STAT_R register is reserved. It always returns 0."]
    #[inline(always)]
    pub fn force_vendor_err3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "This bit (FORCE_VENDOR_ERR2) of the FORCE_ERROR_INT_STAT_R register is reserved. It always returns 0."]
    #[inline(always)]
    pub fn force_vendor_err2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "This bit (FORCE_VENDOR_ERR1) of the FORCE_ERROR_INT_STAT_R register is reserved. It always returns 0."]
    #[inline(always)]
    pub fn force_vendor_err1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Boot Ack error\nValues:\n\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    pub fn force_boot_ack_err(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Response Error (SD Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    pub fn force_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Tuning Error (UHS-I Mode only)"]
    #[inline(always)]
    pub fn force_tuning_err(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for ADMA Error\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    pub fn force_adma_err(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Auto CMD Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    pub fn force_auto_cmd_err(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Current Limit Error\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    pub fn force_cur_lmt_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Data End Bit Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_data_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Data CRC Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    pub fn force_data_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Data Timeout Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_data_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Command Index Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Command End Bit Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Command CRC Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcForceErrorIntStatRReg_SPEC, crate::common::W>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcForceErrorIntStatRReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Force Event for Command Timeout Error (SD/eMMC Mode only)\nValues:\n0x0 (FALSE): Not Affected\n0x1 (TRUE): Command Timeout Error Status is set"]
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

#[doc = "This register is used to indicate the Host Controller Version number."]
pub type EmmcHostCntrlVersRReg = crate::RegValueT<EmmcHostCntrlVersRReg_SPEC>;

impl EmmcHostCntrlVersRReg {
    #[doc = "Vendor Version Number             \n\nThis field is reserved for the vendor version number. Host Driver must not use this status."]
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

    #[doc = "Specification Version Number\nThese bits indicate the Host controller specification version. The upper and lower 4-bits indicate the version. Values 0x06-0xFF are reserved.\nValues:\n0x0 (VER_1_00): SD Host Controller Specification Version 1.00\n0x1 (VER_2_00): SD Host Controller Specification Version 2.00\n0x2 (VER_3_00): SD Host Controller Specification Version 3.00\n0x3 (VER_4_00): SD Host Controller Specification Version 4.00\n0x4 (VER_4_10): SD Host Controller Specification Version 4.10\n0x5 (VER_4_20): SD Host Controller Specification Version 4.20"]
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

#[doc = "This register is used to control the operation of the Host Controller. This register is applicable for an SD/eMMC/UHS-II mode."]
pub type EmmcHostCtrl1RReg = crate::RegValueT<EmmcHostCtrl1RReg_SPEC>;

impl EmmcHostCtrl1RReg {
    #[doc = "Card Detect Signal Selection.\nThis bit selects a source for card detection. When the source for the card detection is switched, the interrupt must be disabled during the switching period.\n\nValues:\n0x1 (CARD_DT_TEST_LEVEL): Card Detect Test Level is selected (for test purpose)\n0x0 (SDCD_PIN): SDCD# (card_detect_n signal) is selected (for normal use)"]
    #[inline(always)]
    pub fn card_detect_sig_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Detect Test Level.\nThis bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates whether a card inserted or not.\n\nValues:\n0x1 (CARD_INSERTED): Card Inserted\n0x0 (No_CARD): No Card"]
    #[inline(always)]
    pub fn card_detect_test_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Extended Data Transfer Width.\nThis bit controls 8-bit bus width mode of embedded device.\n\nValues:\n0x1 (EIGHT_BIT): 8-bit Bus Width\n0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    #[inline(always)]
    pub fn ext_dat_xfer(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA Select.\nThis field is used to select the DMA type.\nWhen Host Version 4 Enable is 1 in Host Control 2 register:\n- 0x0 : SDMA is selected\n- 0x1 : Reserved\n- 0x2 : ADMA2 is selected\n- 0x3 : ADMA2 or ADMA3 is selected\nWhen Host Version 4 Enable is 0 in Host Control 2 register:\n- 0x0 : SDMA is selected\n- 0x1 : Reserved\n- 0x2 : 32-bit Address ADMA2 is selected\n- 0x3 : 64-bit Address ADMA2 is selected\n\nValues:\n0x0 (SDMA): SDMA is selected\n0x1 (RSVD_BIT): Reserved\n0x2 (ADMA2): ADMA2 is selected\n0x3 (ADMA2_3): ADMA2 or ADMA3 is selected"]
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

    #[doc = "High Speed Enable (SD/eMMC Mode only).\nIn SD/eMMC mode, this bit is used to determine the selection of preset value for High Speed mode.\nBefore setting this bit, the Host Driver checks the High Speed Support in the Capabilities register.\nNote: DWC_MSHC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of cclk_tx clock irrespective of this bit.Please refer the section Connecting the Clock IO interface  in the Mobile Storage Host Controller user guide on clocking requirement for an SD/eMMC card.\n\nValues:\n0x1 (HIGH_SPEED): High Speed mode\n0x0 (NORMAL_SPEED): Normal Speed mode"]
    #[inline(always)]
    pub fn high_speed_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Transfer Width.\nFor SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card.\n\nValues:\n0x1 (FOUR_BIT): 4-bit mode\n0x0 (ONE_BIT): 1-bit mode"]
    #[inline(always)]
    pub fn dat_xfer_width(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcHostCtrl1RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcHostCtrl1RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LED Control.\nThis bit is used to caution the user not to remove the card while the SD card is being accessed. The value is reflected on the led_control signal.\n\nValues:\n0x0 (OFF): LED off\n0x1 (ON): LED on"]
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

#[doc = "This register is used to control how the Host Controller operates. This register is applicable for an SD/eMMC/UHS-II mode."]
pub type EmmcHostCtrl2RReg = crate::RegValueT<EmmcHostCtrl2RReg_SPEC>;

impl EmmcHostCtrl2RReg {
    #[doc = "Preset Value Enable\nThis bit enables automatic selection of SDCLK frequency and Driver strength Preset Value registers. When Preset Value Enable is set, SDCLK frequency generation (Frequency Select and Clock Generator\nSelect) and the driver strength selection are performed by the controller. These values are selected from set of Preset Value registers based on selected speed mode.\nNote: For more information, see the FAQ on Preset Register in the DWC_mshc Databook.\nValues:\n0x0 (FALSE): SDCLK and Driver Strength are controlled by Host Driver\n0x1 (TRUE): Automatic Selection by Preset Value are Enabled"]
    #[inline(always)]
    pub fn preset_val_enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Asynchronous Interrupt Enable\nThis bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register.\nValues:\n0x0 (FALSE): Disabled\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn async_int_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "64-bit Addressing\nThis bit is effective when Host Version 4 Enable is set to 1.\nValues:\n0x0 (FALSE): 32 bits addressing\n0x1 (TRUE): 64 bits addressing"]
    #[inline(always)]
    pub fn addressing(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Host Version 4 Enable\nThis bit selects either Version 3.00 compatible mode or Version 4 mode.\nFunctions of following fields are modified for Host Version 4 mode:\n- SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h)\n- ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register\n- 64-bit ADMA Descriptor Size: 128-bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1\n- Selection of 32-bit/64-bit System Addressing: Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register\n- 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register\nNote: It is recommended not to program ADMA3 Integrated Descriptor Address registers, UHS-II registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0).\nValues:\n0x0 (FALSE): Version 3.00 compatible mode\n0x1 (TRUE): Version 4 mode"]
    #[inline(always)]
    pub fn host_ver4_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "CMD23 Enable\nIf the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer.\nValues:\n0x0 (FALSE): Auto CMD23 is disabled\n0x1 (TRUE): Auto CMD23 is enabled"]
    #[inline(always)]
    pub fn cmd23_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "ADMA2 Length Mode\nThis bit selects ADMA2 Length mode to be either 16-bit or 26-bit.\nValues:\n0x0 (FALSE): 16-bit Data Length Mode\n0x1 (TRUE): 26-bit Data Length Mode"]
    #[inline(always)]
    pub fn adma2_len_mode(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "UHS-II Interface Enable\nThis bit is used to enable the UHS-II Interface. The value is reflected on the uhs2_if_en pin.\nValues:\n0x0 (FALSE): SD/eMMC Interface Enabled\n0x1 (TRUE): UHS-II Interface Enabled"]
    #[inline(always)]
    pub fn uhs2_if_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sampling Clock Select\nThis bit is used by the Host Controller to select the sampling clock in SD/eMMC mode to receive CMD and DAT. This bit is set by the tuning procedure and is valid after the completion of tuning (when Execute Tuning is cleared). Setting this bit to 1 means that tuning is completed successfully and setting this bit to 0 means that tuning has failed. The value is reflected on the sample_cclk_sel pin.\nValues:\n0x0 (FALSE): Fixed clock is used to sample data\n0x1 (TRUE): Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn sample_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Execute Tuning\nThis bit is set to 1 to start the tuning procedure in UHS-I/eMMC speed modes and this bit is automatically cleared when tuning procedure is completed.\nValues:\n0x0 (FALSE): Not Tuned or Tuning completed\n0x1 (TRUE): Execute Tuning"]
    #[inline(always)]
    pub fn exec_tuning(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Driver Strength Select\nThis bit is used to select the Host Controller output driver in 1.8V signaling UHS-I/eMMC speed modes. The bit depends on setting of Preset Value Enable. The value is reflected on the uhs1_drv_sth pin.\nValues:\n0x0 (TYPEB): Driver TYPEB is selected\n0x1 (TYPEA): Driver TYPEA is selected\n0x2 (TYPEC): Driver TYPEC is selected\n0x3 (TYPED): Driver TYPED is selected"]
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

    #[doc = "1.8V Signaling Enable\nThis bit controls voltage regulator for I/O cell in UHS-I/eMMC speed modes. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8 signaling\nfails. The value is reflected on the uhs1_swvolt_en pin.\nNote: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/SDR104/DDR50).\nValues:\n0x0 (V_3_3): 3.3V Signalling\n0x1 (V_1_8): 1.8V Signalling"]
    #[inline(always)]
    pub fn signaling_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcHostCtrl2RReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcHostCtrl2RReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "UHS Mode/eMMC Speed Mode Select\nThese bits are used to select UHS mode in the SD mode of operation. In eMMC mode, these bits are used to select eMMC Speed mode.\nUHS Mode (SD/UHS-II mode only):\n- 0x0: SDR12\n- 0x1: SDR25\n- 0x2: SDR50\n- 0x3: SDR104\n- 0x4: DDR50\n- 0x5: Reserved\n- 0x6: Reserved\n- 0x7: UHS-II\neMMC Speed Mode (eMMC mode only):\n- 0x0: Legacy\n- 0x1: High Speed SDR\n- 0x2: Reserved\n- 0x3: HS200\n- 0x4: High Speed DDR\n- 0x5: Reserved\n- 0x6: Reserved\n- 0x7: HS400\nValues:\n0x0 (SDR12): SDR12/Legacy\n0x1 (SDR25): SDR25/High Speed SDR\n0x2 (SDR50): SDR50\n0x3 (SDR104): SDR104/HS200\n0x4 (DDR50): DDR50/High Speed DDR\n0x5 (RSVD5): Reserved\n0x6 (RSVD6): Reserved\n0x7 (UHS2): UHS-II/HS400"]
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

#[doc = "This register is used to select the valid burst types that the AHB Master bus interface can generate. \n\nWhen more than one bit"]
pub type EmmcMbiuCtrlRReg = crate::RegValueT<EmmcMbiuCtrlRReg_SPEC>;

impl EmmcMbiuCtrlRReg {
    #[doc = "INCR16 Burst\nControls generation of INCR16 transfers on Master interface.\nValues:\n0x0 (FALSE): AHB INCR16 burst type is not generated on Master I/F\n0x1 (TRUE): AHB INCR16 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr16_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcMbiuCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "INCR8 Burst\nControls generation of INCR8 transfers on Master interface.\nValues:\n0x0 (FALSE): AHB INCR8 burst type is not generated on Master I/F\n0x1 (TRUE): AHB INCR8 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr8_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcMbiuCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "INCR4 Burst\nControls generation of INCR4 transfers on Master interface.\nValues:\n0x0 (FALSE): AHB INCR4 burst type is not generated on Master I/F\n0x1 (TRUE): AHB INCR4 burst type can be generated on Master I/F"]
    #[inline(always)]
    pub fn burst_incr4_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcMbiuCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcMbiuCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Undefined INCR Burst\nControls generation of undefined length INCR transfer on Master interface.\nValues:\n0x0 (FALSE): Undefined INCR type burst is the least preferred burst on AHB Master I/F\n0x1 (TRUE): Undefined INCR type burst is the most preferred burst on AHB Master I/F"]
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

#[doc = "This register is used to control the operation of MSHC Host Controller."]
pub type EmmcMshcCtrlRReg = crate::RegValueT<EmmcMshcCtrlRReg_SPEC>;

impl EmmcMshcCtrlRReg {
    #[doc = "Internal clock gating disable control\nThis bit must be used to disable IP\'s internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit.\nValues:\n0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally\n0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
    #[inline(always)]
    pub fn sw_cg_dis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcMshcCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcMshcCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command conflict check\nThis bit enables command conflict check.\nNote: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge\nof cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of\na flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value:\n(2*t_card_clk_period - t_setup)  \nValues:\n0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check\n0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
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

#[doc = "This register reflects the current release number of DWC_mshc/DWC_mshc_lite."]
pub type EmmcMshcVerIdRReg = crate::RegValueT<EmmcMshcVerIdRReg_SPEC>;

impl EmmcMshcVerIdRReg {
    #[doc = "Current release number\nThis field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release number that is read by an application.\nAn application reading this register in conjunction with the MSHC_VER_TYPE_R register, gathers details of the current release."]
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

#[doc = "This register reflects the current release type of DWC_mshc/DWC_mshc_lite."]
pub type EmmcMshcVerTypeRReg = crate::RegValueT<EmmcMshcVerTypeRReg_SPEC>;

impl EmmcMshcVerTypeRReg {
    #[doc = "Current release type\nThis field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release type that is read by an application.\nAn application reading this register in conjunction with the MSHC_VER_ID_R register, gathers details of the current release."]
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

#[doc = "This register is used to select the interrupt status that is indicated to the Host System as the interrupt. All these status bi"]
pub type EmmcNormalIntSignalEnRReg = crate::RegValueT<EmmcNormalIntSignalEnRReg_SPEC>;

impl EmmcNormalIntSignalEnRReg {
    #[doc = "Command Queuing Engine Event Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

    #[doc = "FX Event Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

    #[doc = "Re-Tuning Event (UHS-I only) Signal Enable.\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

    #[doc = "INT_C (Embedded) Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

    #[doc = "INT_B (Embedded) Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

    #[doc = "INT_A (Embedded) Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn int_a_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Interrupt Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Removal Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Insertion Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Buffer Read Ready Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Buffer Write Ready Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA Interrupt Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Block Gap Event Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Complete Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcNormalIntSignalEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcNormalIntSignalEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Complete Signal Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

#[doc = "This register enables the Interrupt Status for Normal Interrupt Status register (NORMAL_INT_STAT_R) when NORMAL_INT_STAT_R is s"]
pub type EmmcNormalIntStatEnRReg = crate::RegValueT<EmmcNormalIntStatEnRReg_SPEC>;

impl EmmcNormalIntStatEnRReg {
    #[doc = "CQE Event Status Enable\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FX Event Status Enable\nThis bit is added from Version 4.10.\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Re-Tuning Event (UHS-I only) Status Enable\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn re_tune_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "INT_C (Embedded) Status Enable\n\nIf this bit is set to 0, the Host Controller clears the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_C and may set this bit again after all interrupt requests to INT_C pin are cleared to prevent inadvertent interrupts.\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn int_c_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "INT_B (Embedded) Status Enable\n\nIf this bit is set to 0, the Host Controller clears the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_B and may set this bit again after all interrupt requests to INT_B pin are cleared to prevent inadvertent interrupts.\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn int_b_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "INT_A (Embedded) Status Enable\n\nIf this bit is set to 0, the Host Controller clears the interrupt request to the System. The Host Driver may clear this bit before servicing the INT_A and may set this bit again after all interrupt requests to INT_A pin are cleared to prevent inadvertent interrupts.\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn int_a_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Interrupt Status Enable\n\nIf this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts.\n\nBy setting this bit to 0, interrupt input must be masked by implementation so that the interrupt input is not affected by external signal in any state (for example, floating).\n\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Removal Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Insertion Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Buffer Read Ready Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Buffer Write Ready Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA Interrupt Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Block Gap Event Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Complete Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcNormalIntStatEnRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcNormalIntStatEnRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Complete Status Enable\nValues:\n\n0x0 (FALSE): Masked\n0x1 (TRUE): Enabled"]
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

#[doc = "This register reflects the status of the Normal Interrupt. This register is applicable for an SD/eMMC/UHS-II mode."]
pub type EmmcNormalIntStatRReg = crate::RegValueT<EmmcNormalIntStatRReg_SPEC>;

impl EmmcNormalIntStatRReg {
    #[doc = "Error Interrupt.\nIf any of the bits in the Error Interrupt Status register are set, then this bit is set.\n\nValues:\n0x0 (FALSE): No Error\n0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn err_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Queuing Event\nThis status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI\'s CQIS/CRNQIS register for more details.\n\nValues:\n0x0 (FALSE): No Event\n0x1 (TRUE): Command Queuing Event is detected"]
    #[inline(always)]
    pub fn cqe_event(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<14,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FX Event.\nThis status is set when R\\[14\\] of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function.\n\nValues:\n0x0 (FALSE): No Event\n0x1 (TRUE): FX Event is detected"]
    #[inline(always)]
    pub fn fx_event(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Re-tuning Event.\nThis bit is set if the Re-Tuning Request changes from 0 to 1. Re-Tuning request is not supported."]
    #[inline(always)]
    pub fn re_tune_event(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "INT_C (Embedded).\nThis bit is set if INT_C is enabled and if INT_C# pin is in low level. The INT_C# pin is not supported."]
    #[inline(always)]
    pub fn int_c(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "INT_B (Embedded).\nThis bit is set if INT_B is enabled and if INT_B# pin is in low level. The INT_B# pin is not supported."]
    #[inline(always)]
    pub fn int_b(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "INT_A (Embedded).\nThis bit is set if INT_A is enabled and if INT_A# pin is in low level. The INT_A# pin is not supported."]
    #[inline(always)]
    pub fn int_a(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<9,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Card Interrupt.\nThis bit reflects the synchronized value of:\n- DAT\\[1\\] Interrupt Input for SD Mode\n- DAT\\[2\\] Interrupt Input for UHS-II Mode\n\nValues:\n0x0 (FALSE): No Card Interrupt\n0x1 (TRUE): Generate Card Interrupt"]
    #[inline(always)]
    pub fn card_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<8,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Card Removal.\nThis bit is set if the Card Inserted in the Present State register changes from 1 to 0.\n\nValues:\n0x0 (FALSE): Card state stable or Debouncing\n0x1 (TRUE): Card Removed"]
    #[inline(always)]
    pub fn card_removal_stat_r(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Card Insertion\nThis bit is set if the Card Inserted in the Present State register changes from 0 to 1.\n\nValues:\n0x0 (FALSE): Card state stable or Debouncing\n0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    pub fn card_insertion(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Buffer Read Ready.\nThis bit is set if the Buffer Read Enable changes from 0 to 1.\n\nValues:\n0x0 (FALSE): Not ready to read buffer\n0x1 (TRUE): Ready to read buffer"]
    #[inline(always)]
    pub fn buf_rd_ready(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Buffer Write Ready.\nThis bit is set if the Buffer Write Enable changes from 0 to 1.\n\nValues:\n0x0 (FALSE): Not ready to write buffer\n0x1 (TRUE): Ready to write buffer"]
    #[inline(always)]
    pub fn buf_wr_ready(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA Interrupt.\nThis bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete.\n\nValues:\n0x0 (FALSE): No DMA Interrupt\n0x1 (TRUE): DMA Interrupt is generated"]
    #[inline(always)]
    pub fn dma_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Block Gap Event.\nThis bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request.\n\nValues:\n0x0 (FALSE): No Block Gap Event\n0x1 (TRUE): Transaction stopped at block gap"]
    #[inline(always)]
    pub fn bgap_event(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Complete.\nThis bit is set when a read/write transfer and a command with status busy is completed.\n\nValues:\n0x0 (FALSE): Not complete\n0x1 (TRUE): Command execution is completed"]
    #[inline(always)]
    pub fn xfer_complete(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcNormalIntStatRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,EmmcNormalIntStatRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Command Complete.\nIn an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23.\nThis interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1.\n\nValues:\n0x0 (FALSE): No command complete\n0x1 (TRUE): Command Complete"]
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

#[doc = "This register defines the Preset Value for DDR50 and High Speed DDR speed modes in the SD and eMMC modes, respectively."]
pub type EmmcPresetDdr50RReg = crate::RegValueT<EmmcPresetDdr50RReg_SPEC>;

impl EmmcPresetDdr50RReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate Driver strength value supported for DDR50 bus speed mode. These bits are meaningless for 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
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

    #[doc = "Clock Generator Select Value\nThis bit is effective when Host Controller supports programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetDdr50RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetDdr50RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\nThese bits specify a 10-bit preset value that must be set in the SDCLK/RCLK Frequency Select field of the Clock Control register, as described by a Host System."]
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

#[doc = "This register defines Preset Value for Default Speed mode in SD mode."]
pub type EmmcPresetDsRReg = crate::RegValueT<EmmcPresetDsRReg_SPEC>;

impl EmmcPresetDsRReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate the Driver strength value supported by 1.8V signaling bus speed modes. This field is meaningless for the Default speed mode as it uses 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, EmmcPresetDsRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,EmmcPresetDsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Clock Generator Select Value\nThis bit is effective when Host Controller supports programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetDsRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetDsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\n10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
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

#[doc = "This register defines Preset Value for High Speed mode in SD mode."]
pub type EmmcPresetHsRReg = crate::RegValueT<EmmcPresetHsRReg_SPEC>;

impl EmmcPresetHsRReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate the Driver strength value supported by 1.8V signaling bus speed modes. This field is meaningless for High speed mode as it uses 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, EmmcPresetHsRReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,EmmcPresetHsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Clock Generator Select Value\nThis bit is effective when Host Controller supports programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetHsRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetHsRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\n10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
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

#[doc = "This register defines Preset Value for Initialization in SD/eMMC mode."]
pub type EmmcPresetInitRReg = crate::RegValueT<EmmcPresetInitRReg_SPEC>;

impl EmmcPresetInitRReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate that the Driver strength is supported by 1.8V signaling bus speed modes. These bits are meaningless for 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
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

    #[doc = "Clock Generator Select Value\nThis bit is effective when the Host Controller supports a programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetInitRReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetInitRReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\n10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
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

#[doc = "This register defines Preset Value for SDR104 and HS200 speed modes in the SD and eMMC modes, respectively."]
pub type EmmcPresetSdr104RReg = crate::RegValueT<EmmcPresetSdr104RReg_SPEC>;

impl EmmcPresetSdr104RReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate Driver strength value supported for SDR104 bus speed mode. These bits are meaningless for 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
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

    #[doc = "Clock Generator Select Value\nThis bit is effective when Host Controller supports programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr104RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr104RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\nThese bits specify a 10-bit preset value that must be set in the SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
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

#[doc = "This register defines Preset Value for SDR12 and Legacy speed mode in SD and eMMC mode respectively."]
pub type EmmcPresetSdr12RReg = crate::RegValueT<EmmcPresetSdr12RReg_SPEC>;

impl EmmcPresetSdr12RReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate the Driver strength value supported for the SDR12 bus speed mode. These bits are meaningless for 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
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

    #[doc = "Clock Generator Select Value\nThis bit is effective when Host Controller supports programmable clock generator\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr12RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr12RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\n10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
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

#[doc = "This register defines Preset Value for SDR25 and High Speed SDR speed mode in SD and eMMC mode respectively."]
pub type EmmcPresetSdr25RReg = crate::RegValueT<EmmcPresetSdr25RReg_SPEC>;

impl EmmcPresetSdr25RReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate the Driver strength value supported for the SDR25 bus speed mode. These bits are meaningless for 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
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

    #[doc = "Clock Generator Select Value\nThis bit is effective when Host Controller supports programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr25RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr25RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\n10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
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

#[doc = "This register defines Preset Value for SDR50 speed mode in SD mode."]
pub type EmmcPresetSdr50RReg = crate::RegValueT<EmmcPresetSdr50RReg_SPEC>;

impl EmmcPresetSdr50RReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate Driver strength value supported for SDR50 bus speed mode. These bits are meaningless for 3.3V signaling.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
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

    #[doc = "Clock Generator Select Value\nThis bit is effective when Host Controller supports programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetSdr50RReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetSdr50RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\n10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
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

#[doc = "This register is used to hold the preset value for UHS-II and HS400 speed modes in the SD and eMMC modes, respectively."]
pub type EmmcPresetUhs2RReg = crate::RegValueT<EmmcPresetUhs2RReg_SPEC>;

impl EmmcPresetUhs2RReg {
    #[doc = "Driver Strength Select Value\nThese bits indicate the Driver strength value supported by 1.8V signaling bus speed modes in the SD mode. This field is meaningless for UHS-II mode. In eMMC mode, these bits can be used for selecting\nthe Drive strength value for HS400 mode.\nValues:\n0x0 (TYPEB): Driver Type B is selected\n0x1 (TYPEA): Driver Type A is selected\n0x2 (TYPEC): Driver Type C is selected\n0x3 (TYPED): Driver Type D is selected"]
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

    #[doc = "Clock Generator Select Value\nThis bit is effective when the Host Controller supports a programmable clock generator.\nValues:\n0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator\n0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPresetUhs2RReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPresetUhs2RReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "SDCLK/RCLK Frequency Select Value\nThese bits specify the 10-bit preset value that must be set in the SDCLK/RCLK Frequency Select field of the Clock Control register, as described by a Host System."]
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

#[doc = "This register indicates the present status of the Host Controller. This register is applicable for an SD/eMMC/UHS-II mode."]
pub type EmmcPstateReg = crate::RegValueT<EmmcPstateReg_SPEC>;

impl EmmcPstateReg {
    #[doc = "UHS-II Interface Detection.\nFor SD/eMMC mode, this bit always returns 0.\n\nValues:\n0x0 (FALSE): UHS-II interface is not detected\n0x1 (TRUE): UHS-II interface is detected"]
    #[inline(always)]
    pub fn uhs2_if_detect(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Lane Synchronization.\nFor SD/eMMC mode, this bit always returns 0.\n\nValues:\n0x0 (FALSE): UHS-II PHY is not initialized\n0x1 (TRUE): UHS-II PHY is initialized"]
    #[inline(always)]
    pub fn lane_sync(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "In Dormant Status\nFor SD/eMMC mode, this bit always returns 0.\n\nValues:\n0x0 (FALSE): Not in DORMANT state\n0x1 (TRUE): In DORMANT state"]
    #[inline(always)]
    pub fn in_dormant_st(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Sub Command Status.\nThis bit is used to distinguish between a main command and a sub command status.\n\nValues:\n0x0 (FALSE): Main Command Status\n0x1 (TRUE): Sub Command Status"]
    #[inline(always)]
    pub fn sub_cmd_stat(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Not Issued by Error.\nThis bit is set if a command cannot be issued after setting the command register due to an error except the Auto CMD12 error.\n\nValues:\n0x0 (FALSE): No error for issuing a command\n0x1 (TRUE): Command cannot be issued"]
    #[inline(always)]
    pub fn cmd_issue_err(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Host Regulator Voltage Stable.\nThis bit is used to check whether the host regulator voltage is stable for switching the voltage of UHS-I mode. This bit reflects the synchronized value of the host_reg_vol_stable signal.\n\nValues:\nDescription\n0x0 (FALSE): Host Regulator Voltage is not stable\n0x1 (TRUE): Host Regulator Voltage is stable"]
    #[inline(always)]
    pub fn host_reg_vol(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command-Line Signal Level.\nThis bit is used to check the CMD line level to recover from errors and for debugging. These bits reflect the value of the sd_cmd_in signal."]
    #[inline(always)]
    pub fn cmd_line_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DAT\\[3:0\\] Line Signal Level.\nThis bit is used to check the DAT line level to recover from errors and for debugging. These bits reflect the value of the sd_dat_in (lower nibble) signal."]
    #[inline(always)]
    pub fn dat_3_0(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, EmmcPstateReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Write Protect Switch Pin Level.\nThis bit is supported only for memory and combo cards. This bit reflects the synchronized value of the card_write_prot signal.\n\nValues:\n0x0 (FALSE): Write protected\n0x1 (TRUE): Write enabled"]
    #[inline(always)]
    pub fn wr_protect_sw_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Card Detect Pin Level.\nThis bit reflects the inverse synchronized value of the card_detect_n signal.\n\nValues:\n0x0 (FALSE): No card present\n0x1 (TRUE): Card Present"]
    #[inline(always)]
    pub fn card_detect_pin_level(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Card Stable.\nThis bit indicates the stability of the Card Detect Pin Level. A card is not detected if this bit is set to 1 and the value of the CARD_INSERTED bit is 0.\n\nValues:\n0x0 (FALSE): Reset or Debouncing\n0x1 (TRUE): No Card or Inserted"]
    #[inline(always)]
    pub fn card_stable(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Card Inserted.\nThis bit indicates whether a card has been inserted. The Host Controller debounces this signal so that Host Driver need not wait for it to stabilize.\n\nValues:\n0x0 (FALSE): Reset, Debouncing, or No card\n0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    pub fn card_inserted(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Buffer Read Enable.\nThis bit is used for non-DMA transfers. This bit is set if valid data exists in the Host buffer.\n\nValues:\n0x0 (DISABLED): Read disable\n0x1 (ENABLED): Read enable"]
    #[inline(always)]
    pub fn buf_rd_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Buffer Write Enable.\nThis bit is used for non-DMA transfers. This bit is set if space is available for writing data.\n\nValues:\n0x0 (DISABLED): Write disable\n0x1 (ENABLED): Write enable"]
    #[inline(always)]
    pub fn buf_wr_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Read Transfer Active.\nThis bit indicates whether a read transfer is active for SD/eMMC mode.\n\nValues:\n0x0 (INACTIVE): No valid data\n0x1 (ACTIVE): Transferring data"]
    #[inline(always)]
    pub fn rd_xfer_active(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Write Transfer Active.\nThis status indicates whether a write transfer is active  for SD/eMMC mode.\n\nValues:\n0x0 (INACTIVE): No valid data\n0x1 (ACTIVE): Transferring data"]
    #[inline(always)]
    pub fn wr_xfer_active(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DAT\\[7:4\\] Line Signal Level.\nThis bit is used to check the DAT line level to recover from errors and for debugging. These bits reflect the value of the sd_dat_in (upper nibble) signal."]
    #[inline(always)]
    pub fn dat_7_4(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, EmmcPstateReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Re-Tuning Request.\nDWC_mshc does not generate retuning request. The software must maintain the Retuning timer."]
    #[inline(always)]
    pub fn re_tune_req(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DAT Line Active (SD/eMMC Mode only).\nThis bit indicates whether one of the DAT lines on the SD/eMMC bus is in use.\nIn the case of read transactions, this bit indicates whether a read transfer is executing on the SD/eMMC bus.\nIn the case of write transactions, this bit indicates whether a write transfer is executing on the SD/eMMC bus.\nFor a command with busy, this status indicates whether the command executing busy is executing on an SD or eMMC bus.\n\nValues:\n0x0 (INACTIVE): DAT Line Inactive\n0x1 (ACTIVE): DAT Line Active"]
    #[inline(always)]
    pub fn dat_line_active(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Inhibit (DAT).\nThis bit is applicable for SD/eMMC mode and is generated if either DAT line active or Read transfer active is set to 1.\nIf this bit is set to 0, it indicates that the Host Controller can issue subsequent SD/eMMC commands.\n\nValues:\n0x0 (READY): Can issue command which used DAT line\n0x1 (NOT_READY): Cannot issue command which used\nDAT line"]
    #[inline(always)]
    pub fn cmd_inhibit_dat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcPstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcPstateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Command Inhibit (CMD).\nThis bit indicates the following :\n- SD/eMMC mode: If this bit is set to 0, it indicates that the CMD line is not in use and the Host controller can issue an SD/eMMC command using the CMD line. This bit is set when the command register is written. This bit is cleared when the command response is received. This bit is not cleared by the response of auto CMD12/23 but cleared by the response of read/write command.\n\nValues:\n0x0 (READY): Host Controller is ready to issue a command\n0x1 (NOT_READY): Host Controller is not ready to issue a command"]
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

#[doc = "This register is used to control the bus power for the Card. This register is applicable for an SD, eMMC, and UHS-II modes."]
pub type EmmcPwrCtrlRReg = crate::RegValueT<EmmcPwrCtrlRReg_SPEC>;

impl EmmcPwrCtrlRReg {
    #[doc = "SD Bus Voltage Select for VDD2.\nThis is irrelevant for SD/eMMC card.\n\nValues:\n0x7 (NOT_USED7): Not used\n0x6 (NOT_USED6): Not used\n0x5 (V_1_8): 1.8V\n0x4 (V_1_2): Reserved for 1.2V\n0x3 (RSVD3): Reserved\n0x2 (RSVD2): Reserved\n0x1 (RSVD1): Reserved\n0x0 (NO_VDD2): VDD2 Not Supported"]
    #[inline(always)]
    pub fn sd_bus_vol_vdd2(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, EmmcPwrCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,EmmcPwrCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SD Bus Power for VDD2.\nThis is irrelevant for SD/eMMC card.\n\nValues:\n0x0 (OFF): Power off\n0x1 (ON): Power on"]
    #[inline(always)]
    pub fn sd_bus_pwr_vdd2(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcPwrCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcPwrCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SD Bus Voltage Select for VDD1/eMMC Bus Voltage Select for VDD.\nThese bits enable the Host Driver to select the voltage level for an SD/eMMC card. Before setting this register, the Host Driver checks the Voltage Support bits in the Capabilities register.\nIf an unsupported voltage is selected, the Host System does not supply the SD Bus voltage. The value set in this field is available on the DWC_mshc output signal (sd_vdd1_sel), which is used by the voltage switching circuitry.\nSD Bus Voltage Select options:\n- 0x7 : 3.3V(Typical)\n- 0x6 : 3.0V(Typical)\n- 0x5 : 1.8V(Typical) for Embedded\n- 0x4 : 0x0   - Reserved\neMMC Bus Voltage Select options:\n- 0x7 : 3.3V(Typical)\n- 0x6 : 1.8V(Typical)\n- 0x5 : 1.2V(Typical)\n- 0x4 : 0x0   - Reserved\n\nValues:\n0x7 (V_3_3): 3.3V (Typ.)\n0x6 (V_3_0): 3.0V (Typ.)\n0x5 (V_1_8): 1.8V (Typ.) for Embedded\n0x4 (RSVD4): Reserved\n0x3 (RSVD3): Reserved\n0x2 (RSVD2): Reserved\n0x1 (RSVD1): Reserved\n0x0 (RSVD0): Reserved"]
    #[inline(always)]
    pub fn sd_bus_vol_vdd1(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, EmmcPwrCtrlRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,EmmcPwrCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SD Bus Power for VDD1.\nThis bit enables VDD1 power of the card. This setting is available on the sd_vdd1_on output of DWC_mshc so that it can be used to control the VDD1 power supply of the card. Before setting this bit, the SD Host Driver sets the SD Bus Voltage Select bit. If the Host Controller detects a No Card state, this bit is cleared.\nIn SD mode, if this bit is cleared, the Host Controller stops the SD Clock by clearing the SD_CLK_IN bit in the CLK_CTRL_R register.\n\nValues:\n0x0 (OFF): Power off\n0x1 (ON): Power on"]
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

#[doc = "This register points to the location of UHS-II embedded control registers."]
pub type EmmcPEmbeddedCntrlReg = crate::RegValueT<EmmcPEmbeddedCntrlReg_SPEC>;

impl EmmcPEmbeddedCntrlReg {
    #[doc = "Offset Address of Embedded Control register."]
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

#[doc = "This register used as a pointer for the Vendor Specific Area 1."]
pub type EmmcPVendorSpecificAreaReg = crate::RegValueT<EmmcPVendorSpecificAreaReg_SPEC>;

impl EmmcPVendorSpecificAreaReg {
    #[doc = "Base offset Address for Vendor-Specific registers."]
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

#[doc = "This register is used as a pointer for the Vendor Specific Area 2."]
pub type EmmcPVndr2SpecificAreaReg = crate::RegValueT<EmmcPVndr2SpecificAreaReg_SPEC>;

impl EmmcPVndr2SpecificAreaReg {
    #[doc = "Base offset Address for Command Queuing registers."]
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

#[doc = "This register stores 39-08 bits of the Response Field for an SD/eMMC mode.  The response for an SD/eMMC command can be a maximu"]
pub type EmmcResp01RReg = crate::RegValueT<EmmcResp01RReg_SPEC>;

impl EmmcResp01RReg {
    #[doc = "Command Response.\nThese bits reflect 39-8 bits of SD/eMMC Response Field.\nNote: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
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

#[doc = "This register stores 71-40 bits of the Response Field for an SD/eMMC mode. This register is used to store the response from the"]
pub type EmmcResp23RReg = crate::RegValueT<EmmcResp23RReg_SPEC>;

impl EmmcResp23RReg {
    #[doc = "Command Response.\nThese bits reflect 71-40 bits of the SD/eMMC Response Field."]
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

#[doc = "This register stores 103-72 bits of the Response Field for an SD/eMMC mode.  The response for SD/eMMC command can be a maximum"]
pub type EmmcResp45RReg = crate::RegValueT<EmmcResp45RReg_SPEC>;

impl EmmcResp45RReg {
    #[doc = "Command Response.\nThese bits reflect 103-72 bits of the Response Field."]
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

#[doc = "This register stores 135-104 bits of the Response Field for an SD/eMMC mode.    The SD/eMMC response can be a maximum of 128 bi"]
pub type EmmcResp67RReg = crate::RegValueT<EmmcResp67RReg_SPEC>;

impl EmmcResp67RReg {
    #[doc = "Command Response.\nThese bits reflect bits 135-104 of SD/EMMC Response Field.\nNote: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
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

#[doc = "This register is used to configure a 32-bit Block Count or an SDMA System Address based on the Host Version 4 Enable bit in the"]
pub type EmmcSdmasaRReg = crate::RegValueT<EmmcSdmasaRReg_SPEC>;

impl EmmcSdmasaRReg {
    #[doc = "32-bit Block Count (SDMA System Address).\n- SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may\nreturn an invalid value.\n- 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value.\nFollowing are the values for BLOCKCNT_SDMASA:\n- 0xFFFF_FFFF: 4G - 1 Block\n- ...  \n- 0x0000_0002: 2 Blocks\n- 0x0000_0001: 1 Block\n- 0x0000_0000: Stop Count\nNote:\n- For Host Version 4 Enable = 0, the Host driver does not program the system address in this register while operating in ADMA mode. The system address must be programmed in the ADMA System Address register.\n- For Host Version 4 Enable = 0, the Host driver programs a non-zero 32-bit block count value in this register when Auto CMD23 is enabled for non-DMA and ADMA modes. Auto CMD23 cannot be used with SDMA.\n- This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register."]
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

#[doc = "This register indicates the Interrupt status of each slot."]
pub type EmmcSlotIntrStatusRReg = crate::RegValueT<EmmcSlotIntrStatusRReg_SPEC>;

impl EmmcSlotIntrStatusRReg {
    #[doc = "Interrupt signal for each Slot     \n\nThese status bits indicate the logical OR of Interrupt signal and Wakeup signal for each slot. A maximum of 8 slots can be defined. If one interrupt signal is associated with multiple slots, the Host\nDriver can identify the interrupt that is generated by reading these bits. By a power on reset or by setting Software Reset For All bit, the interrupt signals are de-asserted and this status reads 00h.\n- Bit 00: Slot 1\n- Bit 01: Slot 2\n- Bit 02: Slot 3\n- ..........\n- ..........\n- Bit 07: Slot 8\n\nNote: MSHC Host Controller support single card slot. This register shall always return 0."]
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

#[doc = "This register is used to generate a reset pulse by writing 1 to each bit of this register. After completing the reset, the Host"]
pub type EmmcSwRstRReg = crate::RegValueT<EmmcSwRstRReg_SPEC>;

impl EmmcSwRstRReg {
    #[doc = "Software Reset For DAT line.\nThis bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset.\nThe following registers and bits are cleared by this bit:\n- Buffer Data Port register\n-- Buffer is cleared and initialized.  \n- Present state register\n-- Buffer Read Enable\n-- Buffer Write Enable\n-- Read Transfer Active\n-- Write Transfer Active\n-- DAT Line Active\n-- Command Inhibit (DAT)\n- Block Gap Control register\n-- Continue Request \n-- Stop At Block Gap Request\n- Normal Interrupt status register\n-- Buffer Read Ready\n-- Buffer Write Ready\n-- DMA Interrupt\n-- Block Gap Event\n-- Transfer Complete\n\nValues:\n0x0 (FALSE): Work\n0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_dat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcSwRstRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcSwRstRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Reset For CMD line.\nThis bit resets only a part of the command circuit to be able to issue a command.  This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors.\nThe following registers and bits are cleared by this bit:\n- Present State register : Command Inhibit (CMD) bit\n- Normal Interrupt Status register : Command Complete bit\n- Error Interrupt Status : Response error statuses related to Command Inhibit (CMD) bit\n\nValues:\n0x0 (FALSE): Work\n0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_cmd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcSwRstRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcSwRstRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Software Reset For All.\nThis reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card.\n\nValues:\n0x0 (FALSE): Work\n0x1 (TRUE): Reset"]
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

#[doc = "This register is used to set the Data Timeout Counter value for an SD/eMMC mode according to the timer clock defined by the Cap"]
pub type EmmcToutCtrlRReg = crate::RegValueT<EmmcToutCtrlRReg_SPEC>;

impl EmmcToutCtrlRReg {
    #[doc = "Data Timeout Counter Value.\nThis value determines the interval by which DAT line timeouts are detected. The Timeout clock frequency is generated by dividing the base clock TMCLK value by this value. When setting this\nregister, prevent inadvertent timeout events by clearing the Data Timeout Error Status Enable (in the Error Interrupt Status Enable register).\nThe values for these bits are:\n- 0xF :  Reserved\n- 0xE : TMCLK x 2^27\n- 0x1 : TMCLK x 2^14\n- 0x0 : TMCLK x 2^13\nNote: During a boot operating in an eMMC mode, an application must configure the boot data timeout value (approximately 1 sec) in this bit."]
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

#[doc = "This register is mandatory for the Host Controller, but the wakeup functionality depends on the Host Controller system hardware"]
pub type EmmcWupCtrlRReg = crate::RegValueT<EmmcWupCtrlRReg_SPEC>;

impl EmmcWupCtrlRReg {
    #[doc = "Wakeup Event Enable on SD Card Removal.\nThis bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit.\n\nValues:\n\n0x0 (DISABLED): Disable\n0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn card_removal(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, EmmcWupCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,EmmcWupCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wakeup Event Enable on SD Card Insertion.\nThis bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit.\n\nValues:\n0x0 (DISABLED): Disable\n0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn card_insert(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcWupCtrlRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcWupCtrlRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wakeup Event Enable on Card Interrupt.\nThis bit enables wakeup event through a Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1.\n\nValues:\n0x0 (DISABLED): Disable\n0x1 (ENABLED): Enable"]
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

#[doc = "This register is used to control the operation of data transfers for an SD/eMMC mode. The Host driver sets this register before"]
pub type EmmcXferModeRReg = crate::RegValueT<EmmcXferModeRReg_SPEC>;

impl EmmcXferModeRReg {
    #[doc = "Response Interrupt Disable.\nThe Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller.\nIf Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check\nthe response register.\nIf the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit\nregardless of the Command Complete Signal Enable.\nNote: During tuning (when the Execute Tuning bit in the Host Control2 register is set), the Command Complete Interrupt is not generated irrespective of the Response Interrupt Disable setting.\n\nValues:\n0x0 (ENABLED): Response Interrupt is enabled\n0x1 (DISABLED): Response Interrupt is disabled"]
    #[inline(always)]
    pub fn resp_int_disable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Error Check Enable.\nThe Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller.\nIf the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register.\nNote:\n- Response error check must not be enabled for any response type other than R1 and R5.\n- Response check must not be enabled for the tuning command.\n\nValues:\n0x0 (DISABLED): Response Error Check is disabled\n0x1 (ENABLED): Response Error Check is enabled"]
    #[inline(always)]
    pub fn resp_err_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Response Type R1/R5.\nThis bit selects either R1 or R5 as a response type when the Response Error Check is selected.\nError statuses checked in R1:\n- OUT_OF_RANGE\n- ADDRESS_ERROR\n- BLOCK_LEN_ERROR\n- WP_VIOLATION\n- CARD_IS_LOCKED\n- COM_CRC_ERROR\n- CARD_ECC_FAILED\n- CC_ERROR\n- ERROR\nResponse Flags checked in R5:\n- COM_CRC_ERROR\n- ERROR\n- FUNCTION_NUMBER\n- OUT_OF_RANGE\n\nValues:\n0x0 (RESP_R1): R1 (Memory)\n0x1 (RESP_R5): R5 (SDIO)"]
    #[inline(always)]
    pub fn resp_type(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Multi/Single Block Select.\nThis bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register.\n\nValues:\n0x0 (SINGLE): Single Block\n0x1 (MULTI): Multiple Block"]
    #[inline(always)]
    pub fn multi_blk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data Transfer Direction Select.\nThis bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data\nfrom the SD/eMMC card to the Host Controller and it is set to 0 for all other commands.\n\nValues:\n0x1 (READ): Read (Card to Host)\n0x0 (WRITE): Write (Host to Card)"]
    #[inline(always)]
    pub fn data_xfer_dir(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Auto Command Enable.\nThis field determines use of Auto Command functions.\nNote: In SDIO, this field must be set as 00b (Auto Command Disabled).\n\nValues:\n0x0 (AUTO_CMD_DISABLED): Auto Command Disabled\n0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable\n0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable\n0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Select"]
    #[inline(always)]
    pub fn auto_cmd_enable(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, EmmcXferModeRReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Block Count Enable.\nThis bit is used to enable the Block Count register, which is relevant for multiple block transfers.\nIf this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used.\n\nValues:\n0x1 (ENABLED): Enable\n0x0 (DISABLED): Disable"]
    #[inline(always)]
    pub fn block_count_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcXferModeRReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,EmmcXferModeRReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA Enable.\nThis bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register.\n\nValues:\n0x1 (ENABLED): DMA Data transfer\n0x0 (DISABLED): No data transfer or Non-DMA data\ntransfer"]
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
