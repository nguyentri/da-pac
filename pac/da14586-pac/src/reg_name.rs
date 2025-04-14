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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:45 +0000

//! Contains perfect hash function that maps form raw addresses to
//! a string containing the names of all registers that point to an address.
//!
//! When using tracing feature to record accesses to registers, the exact
//! API path, though which a specific address was accessed gets lost.
//! This poses a problem when recorded register accesses contain accesses
//! to unexpected registers. [`reg_name_from_addr`] can be used to make
//! logs of raw register accesses more readable to humans by providing a list
//! of names of registers that alias a specific physical address.
//!
use phf::phf_map;

/// Get a &str name of a register given it's address.
pub fn reg_name_from_addr(addr: u64) -> Option<&'static &'static str> {
    REGISTER_NAMES.get(&addr)
}

static REGISTER_NAMES: phf::Map<u64, &'static str> = phf_map! {
  0xe000e100u64 => "
      NVIC.iser(),
    ",
  0xe000e180u64 => "
      NVIC.icer(),
    ",
  0xe000e200u64 => "
      NVIC.ispr(),
    ",
  0xe000e280u64 => "
      NVIC.icpr(),
    ",
  0xe000e400u64 => "
      NVIC.ipr0(),
    ",
  0xe000e404u64 => "
      NVIC.ipr1(),
    ",
  0xe000e408u64 => "
      NVIC.ipr2(),
    ",
  0xe000e40cu64 => "
      NVIC.ipr3(),
    ",
  0xe000e410u64 => "
      NVIC.ipr4(),
    ",
  0xe000e414u64 => "
      NVIC.ipr5(),
    ",
  0xe000e418u64 => "
      NVIC.ipr6(),
    ",
  0xe000ed00u64 => "
      SCB.cpuid(),
    ",
  0xe000ed04u64 => "
      SCB.icsr(),
    ",
  0xe000ed0cu64 => "
      SCB.aircr(),
    ",
  0xe000ed10u64 => "
      SCB.scr(),
    ",
  0xe000ed14u64 => "
      SCB.ccr(),
    ",
  0xe000ed1cu64 => "
      SCB.shpr2(),
    ",
  0xe000ed20u64 => "
      SCB.shpr3(),
    ",
  0xe000e010u64 => "
      SYS_TICK.ctrl(),
    ",
  0xe000e014u64 => "
      SYS_TICK.load(),
    ",
  0xe000e018u64 => "
      SYS_TICK.val(),
    ",
  0xe000e01cu64 => "
      SYS_TICK.calib(),
    ",
  0x50001508u64 => "
      ADC_580_BIF_NL_01.gp_adc_clear_int_reg(),
    ",
  0x50001502u64 => "
      ADC_580_BIF_NL_01.gp_adc_ctrl2_reg(),
    ",
  0x50001500u64 => "
      ADC_580_BIF_NL_01.gp_adc_ctrl_reg(),
    ",
  0x5000150eu64 => "
      ADC_580_BIF_NL_01.gp_adc_delay2_reg(),
    ",
  0x5000150cu64 => "
      ADC_580_BIF_NL_01.gp_adc_delay_reg(),
    ",
  0x50001506u64 => "
      ADC_580_BIF_NL_01.gp_adc_offn_reg(),
    ",
  0x50001504u64 => "
      ADC_580_BIF_NL_01.gp_adc_offp_reg(),
    ",
  0x5000150au64 => "
      ADC_580_BIF_NL_01.gp_adc_result_reg(),
    ",
  0x40008410u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr0_reg(),
    ",
  0x40008418u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr1_reg(),
    ",
  0x40008420u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr2_reg(),
    ",
  0x40008428u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr3_reg(),
    ",
  0x40008430u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr4_reg(),
    ",
  0x40008438u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr5_reg(),
    ",
  0x40008440u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr6_reg(),
    ",
  0x40008448u64 => "
      AMBACORE_580_PATCH_GR_00.patch_addr7_reg(),
    ",
  0x40008414u64 => "
      AMBACORE_580_PATCH_GR_00.patch_data0_reg(),
    ",
  0x4000841cu64 => "
      AMBACORE_580_PATCH_GR_00.patch_data1_reg(),
    ",
  0x40008424u64 => "
      AMBACORE_580_PATCH_GR_00.patch_data2_reg(),
    ",
  0x4000842cu64 => "
      AMBACORE_580_PATCH_GR_00.patch_data3_reg(),
    ",
  0x40008434u64 => "
      AMBACORE_580_PATCH_GR_00.patch_data4_reg(),
    ",
  0x4000843cu64 => "
      AMBACORE_580_PATCH_GR_00.patch_data5_reg(),
    ",
  0x40008444u64 => "
      AMBACORE_580_PATCH_GR_00.patch_data6_reg(),
    ",
  0x4000844cu64 => "
      AMBACORE_580_PATCH_GR_00.patch_data7_reg(),
    ",
  0x40008400u64 => "
      AMBACORE_580_PATCH_GR_00.patch_valid_reg(),
    ",
  0x40008408u64 => "
      AMBACORE_580_PATCH_GR_00.patch_valid_reset_reg(),
    ",
  0x40008404u64 => "
      AMBACORE_580_PATCH_GR_00.patch_valid_set_reg(),
    ",
  0x50001602u64 => "
      ANAMISC_580_NL_01.clk_ref_cnt_reg(),
    ",
  0x50001600u64 => "
      ANAMISC_580_NL_01.clk_ref_sel_reg(),
    ",
  0x50001606u64 => "
      ANAMISC_580_NL_01.clk_ref_val_h_reg(),
    ",
  0x50001604u64 => "
      ANAMISC_580_NL_01.clk_ref_val_l_reg(),
    ",
  0x400000a4u64 => "
      BLE_580_GR_01.ble_actscanstat_reg(),
    ",
  0x40000090u64 => "
      BLE_580_GR_01.ble_advchmap_reg(),
    ",
  0x400000a0u64 => "
      BLE_580_GR_01.ble_advtim_reg(),
    ",
  0x400000c0u64 => "
      BLE_580_GR_01.ble_aescntl_reg(),
    ",
  0x400000d0u64 => "
      BLE_580_GR_01.ble_aeskey127_96_reg(),
    ",
  0x400000c4u64 => "
      BLE_580_GR_01.ble_aeskey31_0_reg(),
    ",
  0x400000c8u64 => "
      BLE_580_GR_01.ble_aeskey63_32_reg(),
    ",
  0x400000ccu64 => "
      BLE_580_GR_01.ble_aeskey95_64_reg(),
    ",
  0x400000d4u64 => "
      BLE_580_GR_01.ble_aesptr_reg(),
    ",
  0x40000044u64 => "
      BLE_580_GR_01.ble_basetimecntcorr_reg(),
    ",
  0x4000001cu64 => "
      BLE_580_GR_01.ble_basetimecnt_reg(),
    ",
  0x40000024u64 => "
      BLE_580_GR_01.ble_bdaddrl_reg(),
    ",
  0x40000028u64 => "
      BLE_580_GR_01.ble_bdaddru_reg(),
    ",
  0x40000200u64 => "
      BLE_580_GR_01.ble_cntl2_reg(),
    ",
  0x4000002cu64 => "
      BLE_580_GR_01.ble_currentrxdescptr_reg(),
    ",
  0x40000058u64 => "
      BLE_580_GR_01.ble_debugaddmax_reg(),
    ",
  0x4000005cu64 => "
      BLE_580_GR_01.ble_debugaddmin_reg(),
    ",
  0x40000030u64 => "
      BLE_580_GR_01.ble_deepslcntl_reg(),
    ",
  0x40000038u64 => "
      BLE_580_GR_01.ble_deepslstat_reg(),
    ",
  0x40000034u64 => "
      BLE_580_GR_01.ble_deepslwkup_reg(),
    ",
  0x40000050u64 => "
      BLE_580_GR_01.ble_diagcntl_reg(),
    ",
  0x40000054u64 => "
      BLE_580_GR_01.ble_diagstat_reg(),
    ",
  0x4000003cu64 => "
      BLE_580_GR_01.ble_enbpreset_reg(),
    ",
  0x40000060u64 => "
      BLE_580_GR_01.ble_errortypestat_reg(),
    ",
  0x40000040u64 => "
      BLE_580_GR_01.ble_finecntcorr_reg(),
    ",
  0x40000020u64 => "
      BLE_580_GR_01.ble_finetimecnt_reg(),
    ",
  0x400000f8u64 => "
      BLE_580_GR_01.ble_finetimtgt_reg(),
    ",
  0x400000f4u64 => "
      BLE_580_GR_01.ble_grosstimtgt_reg(),
    ",
  0x40000018u64 => "
      BLE_580_GR_01.ble_intack_reg(),
    ",
  0x4000000cu64 => "
      BLE_580_GR_01.ble_intcntl_reg(),
    ",
  0x40000014u64 => "
      BLE_580_GR_01.ble_intrawstat_reg(),
    ",
  0x40000010u64 => "
      BLE_580_GR_01.ble_intstat_reg(),
    ",
  0x40000070u64 => "
      BLE_580_GR_01.ble_radiocntl0_reg(),
    ",
  0x40000074u64 => "
      BLE_580_GR_01.ble_radiocntl1_reg(),
    ",
  0x40000080u64 => "
      BLE_580_GR_01.ble_radiopwrupdn_reg(),
    ",
  0x400000e0u64 => "
      BLE_580_GR_01.ble_rftestcntl_reg(),
    ",
  0x40000204u64 => "
      BLE_580_GR_01.ble_rf_diagirq_reg(),
    ",
  0x40000000u64 => "
      BLE_580_GR_01.ble_rwbtlecntl_reg(),
    ",
  0x40000008u64 => "
      BLE_580_GR_01.ble_rwbtleconf_reg(),
    ",
  0x400000dcu64 => "
      BLE_580_GR_01.ble_rxmicval_reg(),
    ",
  0x400000fcu64 => "
      BLE_580_GR_01.ble_sampleclk_reg(),
    ",
  0x40000064u64 => "
      BLE_580_GR_01.ble_swprofiling_reg(),
    ",
  0x400000f0u64 => "
      BLE_580_GR_01.ble_timgencntl_reg(),
    ",
  0x400000d8u64 => "
      BLE_580_GR_01.ble_txmicval_reg(),
    ",
  0x40000004u64 => "
      BLE_580_GR_01.ble_version_reg(),
    ",
  0x400000b8u64 => "
      BLE_580_GR_01.ble_wlnbdev_reg(),
    ",
  0x400000b4u64 => "
      BLE_580_GR_01.ble_wlprivaddptr_reg(),
    ",
  0x400000b0u64 => "
      BLE_580_GR_01.ble_wlpubaddptr_reg(),
    ",
  0x50003205u64 => "
      CHIP_VERSION.chip_config1_reg(),
    ",
  0x50003206u64 => "
      CHIP_VERSION.chip_config2_reg(),
    ",
  0x50003207u64 => "
      CHIP_VERSION.chip_config3_reg(),
    ",
  0x50003200u64 => "
      CHIP_VERSION.chip_id1_reg(),
    ",
  0x50003201u64 => "
      CHIP_VERSION.chip_id2_reg(),
    ",
  0x50003202u64 => "
      CHIP_VERSION.chip_id3_reg(),
    ",
  0x50003204u64 => "
      CHIP_VERSION.chip_revision_reg(),
    ",
  0x50003203u64 => "
      CHIP_VERSION.chip_swc_reg(),
    ",
  0x5000320au64 => "
      CHIP_VERSION.chip_test1_reg(),
    ",
  0x5000320bu64 => "
      CHIP_VERSION.chip_test2_reg(),
    ",
  0x50000086u64 => "
      CRG_580_DCDC_NL_01.dcdc_cal1_reg(),
    ",
  0x50000088u64 => "
      CRG_580_DCDC_NL_01.dcdc_cal2_reg(),
    ",
  0x5000008au64 => "
      CRG_580_DCDC_NL_01.dcdc_cal3_reg(),
    ",
  0x50000082u64 => "
      CRG_580_DCDC_NL_01.dcdc_ctrl2_reg(),
    ",
  0x50000084u64 => "
      CRG_580_DCDC_NL_01.dcdc_ctrl3_reg(),
    ",
  0x50000080u64 => "
      CRG_580_DCDC_NL_01.dcdc_ctrl_reg(),
    ",
  0x5000002au64 => "
      CRG_580_NL_01.ana_status_reg(),
    ",
  0x50000028u64 => "
      CRG_580_NL_01.bandgap_reg(),
    ",
  0x50000022u64 => "
      CRG_580_NL_01.clk_16m_reg(),
    ",
  0x50000020u64 => "
      CRG_580_NL_01.clk_32k_reg(),
    ",
  0x50000000u64 => "
      CRG_580_NL_01.clk_amba_reg(),
    ",
  0x5000000au64 => "
      CRG_580_NL_01.clk_ctrl_reg(),
    ",
  0x50000002u64 => "
      CRG_580_NL_01.clk_freq_trim_reg(),
    ",
  0x50000004u64 => "
      CRG_580_NL_01.clk_per_reg(),
    ",
  0x50000008u64 => "
      CRG_580_NL_01.clk_radio_reg(),
    ",
  0x50000024u64 => "
      CRG_580_NL_01.clk_rcx20k_reg(),
    ",
  0x50000010u64 => "
      CRG_580_NL_01.pmu_ctrl_reg(),
    ",
  0x50000030u64 => "
      CRG_580_NL_01.rf_io_ctrl1_reg(),
    ",
  0x50000032u64 => "
      CRG_580_NL_01.rf_lna_ctrl1_reg(),
    ",
  0x50000034u64 => "
      CRG_580_NL_01.rf_lna_ctrl2_reg(),
    ",
  0x50000036u64 => "
      CRG_580_NL_01.rf_lna_ctrl3_reg(),
    ",
  0x50000038u64 => "
      CRG_580_NL_01.rf_rssi_comp_ctrl_reg(),
    ",
  0x5000003au64 => "
      CRG_580_NL_01.rf_vco_ctrl_reg(),
    ",
  0x5000003eu64 => "
      CRG_580_NL_01.spotp_test_reg(),
    ",
  0x50000012u64 => "
      CRG_580_NL_01.sys_ctrl_reg(),
    ",
  0x50000014u64 => "
      CRG_580_NL_01.sys_stat_reg(),
    ",
  0x50000016u64 => "
      CRG_580_NL_01.trim_ctrl_reg(),
    ",
  0x500030fau64 => "
      GPIO_580_PORTS_NL_01.bist_ctrl_reg(),
    ",
  0x50003006u64 => "
      GPIO_580_PORTS_NL_01.p00_mode_reg(),
    ",
  0x50003008u64 => "
      GPIO_580_PORTS_NL_01.p01_mode_reg(),
    ",
  0x50003070u64 => "
      GPIO_580_PORTS_NL_01.p01_padpwr_ctrl_reg(),
    ",
  0x5000300au64 => "
      GPIO_580_PORTS_NL_01.p02_mode_reg(),
    ",
  0x5000300cu64 => "
      GPIO_580_PORTS_NL_01.p03_mode_reg(),
    ",
  0x5000300eu64 => "
      GPIO_580_PORTS_NL_01.p04_mode_reg(),
    ",
  0x50003010u64 => "
      GPIO_580_PORTS_NL_01.p05_mode_reg(),
    ",
  0x50003012u64 => "
      GPIO_580_PORTS_NL_01.p06_mode_reg(),
    ",
  0x50003014u64 => "
      GPIO_580_PORTS_NL_01.p07_mode_reg(),
    ",
  0x50003000u64 => "
      GPIO_580_PORTS_NL_01.p0_data_reg(),
    ",
  0x50003004u64 => "
      GPIO_580_PORTS_NL_01.p0_reset_data_reg(),
    ",
  0x50003002u64 => "
      GPIO_580_PORTS_NL_01.p0_set_data_reg(),
    ",
  0x50003026u64 => "
      GPIO_580_PORTS_NL_01.p10_mode_reg(),
    ",
  0x50003028u64 => "
      GPIO_580_PORTS_NL_01.p11_mode_reg(),
    ",
  0x5000302au64 => "
      GPIO_580_PORTS_NL_01.p12_mode_reg(),
    ",
  0x5000302cu64 => "
      GPIO_580_PORTS_NL_01.p13_mode_reg(),
    ",
  0x5000302eu64 => "
      GPIO_580_PORTS_NL_01.p14_mode_reg(),
    ",
  0x50003030u64 => "
      GPIO_580_PORTS_NL_01.p15_mode_reg(),
    ",
  0x50003020u64 => "
      GPIO_580_PORTS_NL_01.p1_data_reg(),
    ",
  0x50003024u64 => "
      GPIO_580_PORTS_NL_01.p1_reset_data_reg(),
    ",
  0x50003022u64 => "
      GPIO_580_PORTS_NL_01.p1_set_data_reg(),
    ",
  0x50003046u64 => "
      GPIO_580_PORTS_NL_01.p20_mode_reg(),
    ",
  0x50003048u64 => "
      GPIO_580_PORTS_NL_01.p21_mode_reg(),
    ",
  0x5000304au64 => "
      GPIO_580_PORTS_NL_01.p22_mode_reg(),
    ",
  0x5000304cu64 => "
      GPIO_580_PORTS_NL_01.p23_mode_reg(),
    ",
  0x5000304eu64 => "
      GPIO_580_PORTS_NL_01.p24_mode_reg(),
    ",
  0x50003050u64 => "
      GPIO_580_PORTS_NL_01.p25_mode_reg(),
    ",
  0x50003052u64 => "
      GPIO_580_PORTS_NL_01.p26_mode_reg(),
    ",
  0x50003054u64 => "
      GPIO_580_PORTS_NL_01.p27_mode_reg(),
    ",
  0x50003056u64 => "
      GPIO_580_PORTS_NL_01.p28_mode_reg(),
    ",
  0x50003058u64 => "
      GPIO_580_PORTS_NL_01.p29_mode_reg(),
    ",
  0x50003040u64 => "
      GPIO_580_PORTS_NL_01.p2_data_reg(),
    ",
  0x50003072u64 => "
      GPIO_580_PORTS_NL_01.p2_padpwr_ctrl_reg(),
    ",
  0x50003044u64 => "
      GPIO_580_PORTS_NL_01.p2_reset_data_reg(),
    ",
  0x50003042u64 => "
      GPIO_580_PORTS_NL_01.p2_set_data_reg(),
    ",
  0x50003086u64 => "
      GPIO_580_PORTS_NL_01.p30_mode_reg(),
    ",
  0x50003088u64 => "
      GPIO_580_PORTS_NL_01.p31_mode_reg(),
    ",
  0x5000308au64 => "
      GPIO_580_PORTS_NL_01.p32_mode_reg(),
    ",
  0x5000308cu64 => "
      GPIO_580_PORTS_NL_01.p33_mode_reg(),
    ",
  0x5000308eu64 => "
      GPIO_580_PORTS_NL_01.p34_mode_reg(),
    ",
  0x50003090u64 => "
      GPIO_580_PORTS_NL_01.p35_mode_reg(),
    ",
  0x50003092u64 => "
      GPIO_580_PORTS_NL_01.p36_mode_reg(),
    ",
  0x50003094u64 => "
      GPIO_580_PORTS_NL_01.p37_mode_reg(),
    ",
  0x50003080u64 => "
      GPIO_580_PORTS_NL_01.p3_data_reg(),
    ",
  0x50003074u64 => "
      GPIO_580_PORTS_NL_01.p3_padpwr_ctrl_reg(),
    ",
  0x50003084u64 => "
      GPIO_580_PORTS_NL_01.p3_reset_data_reg(),
    ",
  0x50003082u64 => "
      GPIO_580_PORTS_NL_01.p3_set_data_reg(),
    ",
  0x500030feu64 => "
      GPIO_580_PORTS_NL_01.rombist_resulth_reg(),
    ",
  0x500030fcu64 => "
      GPIO_580_PORTS_NL_01.rombist_resultl_reg(),
    ",
  0x500030f2u64 => "
      GPIO_580_PORTS_NL_01.test_ctrl2_reg(),
    ",
  0x500030f4u64 => "
      GPIO_580_PORTS_NL_01.test_ctrl3_reg(),
    ",
  0x500030f6u64 => "
      GPIO_580_PORTS_NL_01.test_ctrl4_reg(),
    ",
  0x500030f8u64 => "
      GPIO_580_PORTS_NL_01.test_ctrl5_reg(),
    ",
  0x500030f0u64 => "
      GPIO_580_PORTS_NL_01.test_ctrl_reg(),
    ",
  0x50001398u64 => "
      I_2_C_580_NL_00.i2c_ack_general_call_reg(),
    ",
  0x5000135cu64 => "
      I_2_C_580_NL_00.i2c_clr_activity_reg(),
    ",
  0x50001368u64 => "
      I_2_C_580_NL_00.i2c_clr_gen_call_reg(),
    ",
  0x50001340u64 => "
      I_2_C_580_NL_00.i2c_clr_intr_reg(),
    ",
  0x50001350u64 => "
      I_2_C_580_NL_00.i2c_clr_rd_req_reg(),
    ",
  0x50001358u64 => "
      I_2_C_580_NL_00.i2c_clr_rx_done_reg(),
    ",
  0x50001348u64 => "
      I_2_C_580_NL_00.i2c_clr_rx_over_reg(),
    ",
  0x50001344u64 => "
      I_2_C_580_NL_00.i2c_clr_rx_under_reg(),
    ",
  0x50001364u64 => "
      I_2_C_580_NL_00.i2c_clr_start_det_reg(),
    ",
  0x50001360u64 => "
      I_2_C_580_NL_00.i2c_clr_stop_det_reg(),
    ",
  0x50001354u64 => "
      I_2_C_580_NL_00.i2c_clr_tx_abrt_reg(),
    ",
  0x5000134cu64 => "
      I_2_C_580_NL_00.i2c_clr_tx_over_reg(),
    ",
  0x50001300u64 => "
      I_2_C_580_NL_00.i2c_con_reg(),
    ",
  0x50001310u64 => "
      I_2_C_580_NL_00.i2c_data_cmd_reg(),
    ",
  0x5000136cu64 => "
      I_2_C_580_NL_00.i2c_enable_reg(),
    ",
  0x5000139cu64 => "
      I_2_C_580_NL_00.i2c_enable_status_reg(),
    ",
  0x5000131cu64 => "
      I_2_C_580_NL_00.i2c_fs_scl_hcnt_reg(),
    ",
  0x50001320u64 => "
      I_2_C_580_NL_00.i2c_fs_scl_lcnt_reg(),
    ",
  0x500013a0u64 => "
      I_2_C_580_NL_00.i2c_ic_fs_spklen_reg(),
    ",
  0x50001330u64 => "
      I_2_C_580_NL_00.i2c_intr_mask_reg(),
    ",
  0x5000132cu64 => "
      I_2_C_580_NL_00.i2c_intr_stat_reg(),
    ",
  0x50001334u64 => "
      I_2_C_580_NL_00.i2c_raw_intr_stat_reg(),
    ",
  0x50001378u64 => "
      I_2_C_580_NL_00.i2c_rxflr_reg(),
    ",
  0x50001338u64 => "
      I_2_C_580_NL_00.i2c_rx_tl_reg(),
    ",
  0x50001308u64 => "
      I_2_C_580_NL_00.i2c_sar_reg(),
    ",
  0x5000137cu64 => "
      I_2_C_580_NL_00.i2c_sda_hold_reg(),
    ",
  0x50001394u64 => "
      I_2_C_580_NL_00.i2c_sda_setup_reg(),
    ",
  0x50001314u64 => "
      I_2_C_580_NL_00.i2c_ss_scl_hcnt_reg(),
    ",
  0x50001318u64 => "
      I_2_C_580_NL_00.i2c_ss_scl_lcnt_reg(),
    ",
  0x50001370u64 => "
      I_2_C_580_NL_00.i2c_status_reg(),
    ",
  0x50001304u64 => "
      I_2_C_580_NL_00.i2c_tar_reg(),
    ",
  0x50001374u64 => "
      I_2_C_580_NL_00.i2c_txflr_reg(),
    ",
  0x50001380u64 => "
      I_2_C_580_NL_00.i2c_tx_abrt_source_reg(),
    ",
  0x5000133cu64 => "
      I_2_C_580_NL_00.i2c_tx_tl_reg(),
    ",
  0x5000140cu64 => "
      KBRD_580_NL_01.gpio_debounce_reg(),
    ",
  0x50001410u64 => "
      KBRD_580_NL_01.gpio_int_level_ctrl_reg(),
    ",
  0x50001400u64 => "
      KBRD_580_NL_01.gpio_irq0_in_sel_reg(),
    ",
  0x50001402u64 => "
      KBRD_580_NL_01.gpio_irq1_in_sel_reg(),
    ",
  0x50001404u64 => "
      KBRD_580_NL_01.gpio_irq2_in_sel_reg(),
    ",
  0x50001406u64 => "
      KBRD_580_NL_01.gpio_irq3_in_sel_reg(),
    ",
  0x50001408u64 => "
      KBRD_580_NL_01.gpio_irq4_in_sel_reg(),
    ",
  0x5000140eu64 => "
      KBRD_580_NL_01.gpio_reset_irq_reg(),
    ",
  0x50001412u64 => "
      KBRD_580_NL_01.kbrd_irq_in_sel0_reg(),
    ",
  0x50001414u64 => "
      KBRD_580_NL_01.kbrd_irq_in_sel1_reg(),
    ",
  0x50001416u64 => "
      KBRD_580_NL_01.kbrd_irq_in_sel2_reg(),
    ",
  0x4000800cu64 => "
      OTPC_580_GR_01.otpc_ahbadr_reg(),
    ",
  0x40008010u64 => "
      OTPC_580_GR_01.otpc_celadr_reg(),
    ",
  0x40008018u64 => "
      OTPC_580_GR_01.otpc_ffprt_reg(),
    ",
  0x4000801cu64 => "
      OTPC_580_GR_01.otpc_ffrd_reg(),
    ",
  0x40008000u64 => "
      OTPC_580_GR_01.otpc_mode_reg(),
    ",
  0x40008014u64 => "
      OTPC_580_GR_01.otpc_nwords_reg(),
    ",
  0x40008004u64 => "
      OTPC_580_GR_01.otpc_pctrl_reg(),
    ",
  0x40008008u64 => "
      OTPC_580_GR_01.otpc_stat_reg(),
    ",
  0x50000206u64 => "
      QUADEC_580_GR_01.qdec_clockdiv_reg(),
    ",
  0x50000208u64 => "
      QUADEC_580_GR_01.qdec_ctrl2_reg(),
    ",
  0x50000200u64 => "
      QUADEC_580_GR_01.qdec_ctrl_reg(),
    ",
  0x50000202u64 => "
      QUADEC_580_GR_01.qdec_xcnt_reg(),
    ",
  0x50000204u64 => "
      QUADEC_580_GR_01.qdec_ycnt_reg(),
    ",
  0x5000020au64 => "
      QUADEC_580_GR_01.qdec_zcnt_reg(),
    ",
  0x50002600u64 => "
      R_RFCU_580_NL_01.bias_ctrl1_reg(),
    ",
  0x50002310u64 => "
      R_RFCU_580_NL_01.rf_adci_dc_offset_reg(),
    ",
  0x50002312u64 => "
      R_RFCU_580_NL_01.rf_adcq_dc_offset_reg(),
    ",
  0x50002830u64 => "
      R_RFCU_580_NL_01.rf_adc_ctrl1_reg(),
    ",
  0x50002832u64 => "
      R_RFCU_580_NL_01.rf_adc_ctrl2_reg(),
    ",
  0x50002834u64 => "
      R_RFCU_580_NL_01.rf_adc_ctrl3_reg(),
    ",
  0x50002864u64 => "
      R_RFCU_580_NL_01.rf_afc_ctrl_reg(),
    ",
  0x50002860u64 => "
      R_RFCU_580_NL_01.rf_agc_ctrl1_reg(),
    ",
  0x50002862u64 => "
      R_RFCU_580_NL_01.rf_agc_ctrl2_reg(),
    ",
  0x50002850u64 => "
      R_RFCU_580_NL_01.rf_agc_lut_01_reg(),
    ",
  0x50002852u64 => "
      R_RFCU_580_NL_01.rf_agc_lut_23_reg(),
    ",
  0x50002854u64 => "
      R_RFCU_580_NL_01.rf_agc_lut_45_reg(),
    ",
  0x50002856u64 => "
      R_RFCU_580_NL_01.rf_agc_lut_67_reg(),
    ",
  0x50002858u64 => "
      R_RFCU_580_NL_01.rf_agc_lut_89_reg(),
    ",
  0x50002900u64 => "
      R_RFCU_580_NL_01.rf_agc_result_reg(),
    ",
  0x50002000u64 => "
      R_RFCU_580_NL_01.rf_bmcw_reg(),
    ",
  0x50002060u64 => "
      R_RFCU_580_NL_01.rf_calcap1_reg(),
    ",
  0x50002062u64 => "
      R_RFCU_580_NL_01.rf_calcap2_reg(),
    ",
  0x50002040u64 => "
      R_RFCU_580_NL_01.rf_calstate_reg(),
    ",
  0x50002200u64 => "
      R_RFCU_580_NL_01.rf_cal_ctrl_reg(),
    ",
  0x50002512u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_10_reg(),
    ",
  0x50002514u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_11_reg(),
    ",
  0x50002516u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_12_reg(),
    ",
  0x50002518u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_13_reg(),
    ",
  0x5000251au64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_14_reg(),
    ",
  0x50002500u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_1_reg(),
    ",
  0x50002502u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_2_reg(),
    ",
  0x50002504u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_3_reg(),
    ",
  0x50002506u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_4_reg(),
    ",
  0x50002508u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_5_reg(),
    ",
  0x5000250au64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_6_reg(),
    ",
  0x5000250cu64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_7_reg(),
    ",
  0x5000250eu64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_8_reg(),
    ",
  0x50002510u64 => "
      R_RFCU_580_NL_01.rf_cntrl_timer_9_reg(),
    ",
  0x50002c50u64 => "
      R_RFCU_580_NL_01.rf_cp_ctrl_reg(),
    ",
  0x50002866u64 => "
      R_RFCU_580_NL_01.rf_dc_offset_ctrl1_reg(),
    ",
  0x50002868u64 => "
      R_RFCU_580_NL_01.rf_dc_offset_ctrl2_reg(),
    ",
  0x5000286au64 => "
      R_RFCU_580_NL_01.rf_dc_offset_ctrl3_reg(),
    ",
  0x5000286cu64 => "
      R_RFCU_580_NL_01.rf_dc_offset_ctrl4_reg(),
    ",
  0x50002314u64 => "
      R_RFCU_580_NL_01.rf_dc_offset_result_reg(),
    ",
  0x50002840u64 => "
      R_RFCU_580_NL_01.rf_dem_ctrl_reg(),
    ",
  0x50002412u64 => "
      R_RFCU_580_NL_01.rf_enable_config10_reg(),
    ",
  0x50002414u64 => "
      R_RFCU_580_NL_01.rf_enable_config11_reg(),
    ",
  0x50002416u64 => "
      R_RFCU_580_NL_01.rf_enable_config12_reg(),
    ",
  0x50002418u64 => "
      R_RFCU_580_NL_01.rf_enable_config13_reg(),
    ",
  0x5000241au64 => "
      R_RFCU_580_NL_01.rf_enable_config14_reg(),
    ",
  0x5000241cu64 => "
      R_RFCU_580_NL_01.rf_enable_config15_reg(),
    ",
  0x5000241eu64 => "
      R_RFCU_580_NL_01.rf_enable_config16_reg(),
    ",
  0x50002420u64 => "
      R_RFCU_580_NL_01.rf_enable_config17_reg(),
    ",
  0x50002422u64 => "
      R_RFCU_580_NL_01.rf_enable_config18_reg(),
    ",
  0x50002424u64 => "
      R_RFCU_580_NL_01.rf_enable_config19_reg(),
    ",
  0x50002400u64 => "
      R_RFCU_580_NL_01.rf_enable_config1_reg(),
    ",
  0x50002426u64 => "
      R_RFCU_580_NL_01.rf_enable_config20_reg(),
    ",
  0x50002428u64 => "
      R_RFCU_580_NL_01.rf_enable_config21_reg(),
    ",
  0x5000242au64 => "
      R_RFCU_580_NL_01.rf_enable_config22_reg(),
    ",
  0x5000242cu64 => "
      R_RFCU_580_NL_01.rf_enable_config23_reg(),
    ",
  0x50002402u64 => "
      R_RFCU_580_NL_01.rf_enable_config2_reg(),
    ",
  0x50002404u64 => "
      R_RFCU_580_NL_01.rf_enable_config3_reg(),
    ",
  0x50002406u64 => "
      R_RFCU_580_NL_01.rf_enable_config4_reg(),
    ",
  0x50002408u64 => "
      R_RFCU_580_NL_01.rf_enable_config5_reg(),
    ",
  0x5000240au64 => "
      R_RFCU_580_NL_01.rf_enable_config6_reg(),
    ",
  0x5000240cu64 => "
      R_RFCU_580_NL_01.rf_enable_config7_reg(),
    ",
  0x5000240eu64 => "
      R_RFCU_580_NL_01.rf_enable_config8_reg(),
    ",
  0x50002410u64 => "
      R_RFCU_580_NL_01.rf_enable_config9_reg(),
    ",
  0x50002820u64 => "
      R_RFCU_580_NL_01.rf_iff_ctrl1_reg(),
    ",
  0x50002300u64 => "
      R_RFCU_580_NL_01.rf_iff_result_reg(),
    ",
  0x50002204u64 => "
      R_RFCU_580_NL_01.rf_irq_ctrl_reg(),
    ",
  0x50002c60u64 => "
      R_RFCU_580_NL_01.rf_lf_ctrl_reg(),
    ",
  0x50002c52u64 => "
      R_RFCU_580_NL_01.rf_lf_res_ctrl_reg(),
    ",
  0x50002c0au64 => "
      R_RFCU_580_NL_01.rf_mgain_ctrl2_reg(),
    ",
  0x50002c08u64 => "
      R_RFCU_580_NL_01.rf_mgain_ctrl_reg(),
    ",
  0x50002c10u64 => "
      R_RFCU_580_NL_01.rf_mgc_ctrl_reg(),
    ",
  0x50002810u64 => "
      R_RFCU_580_NL_01.rf_mixer_ctrl1_reg(),
    ",
  0x50002812u64 => "
      R_RFCU_580_NL_01.rf_mixer_ctrl2_reg(),
    ",
  0x50002020u64 => "
      R_RFCU_580_NL_01.rf_overrule_reg(),
    ",
  0x50002a00u64 => "
      R_RFCU_580_NL_01.rf_pa_ctrl_reg(),
    ",
  0x50002c40u64 => "
      R_RFCU_580_NL_01.rf_pfd_ctrl_reg(),
    ",
  0x50002870u64 => "
      R_RFCU_580_NL_01.rf_radig_spare_reg(),
    ",
  0x50002202u64 => "
      R_RFCU_580_NL_01.rf_ref_osc_reg(),
    ",
  0x50002902u64 => "
      R_RFCU_580_NL_01.rf_rssi_result_reg(),
    ",
  0x50002080u64 => "
      R_RFCU_580_NL_01.rf_scan_feedback_reg(),
    ",
  0x50002602u64 => "
      R_RFCU_580_NL_01.rf_spare1_reg(),
    ",
  0x50002c00u64 => "
      R_RFCU_580_NL_01.rf_synth_ctrl1_reg(),
    ",
  0x50002c02u64 => "
      R_RFCU_580_NL_01.rf_synth_ctrl2_reg(),
    ",
  0x50002c04u64 => "
      R_RFCU_580_NL_01.rf_synth_ctrl3_reg(),
    ",
  0x50002318u64 => "
      R_RFCU_580_NL_01.rf_synth_result2_reg(),
    ",
  0x5000231au64 => "
      R_RFCU_580_NL_01.rf_synth_result3_reg(),
    ",
  0x50002316u64 => "
      R_RFCU_580_NL_01.rf_synth_result_reg(),
    ",
  0x50002c70u64 => "
      R_RFCU_580_NL_01.rf_tdc_ctrl_reg(),
    ",
  0x50002c06u64 => "
      R_RFCU_580_NL_01.rf_vcocal_ctrl_reg(),
    ",
  0x50002c20u64 => "
      R_RFCU_580_NL_01.rf_vcovar_ctrl_reg(),
    ",
  0x50002c22u64 => "
      R_RFCU_580_NL_01.rf_vco_calcap_bit14_reg(),
    ",
  0x50002c24u64 => "
      R_RFCU_580_NL_01.rf_vco_calcap_bit15_reg(),
    ",
  0x50003502u64 => "
      RFPT_580_GR_01.rfpt_addr_reg(),
    ",
  0x50003500u64 => "
      RFPT_580_GR_01.rfpt_ctrl_reg(),
    ",
  0x50003504u64 => "
      RFPT_580_GR_01.rfpt_len_reg(),
    ",
  0x50003506u64 => "
      RFPT_580_GR_01.rfpt_stat_reg(),
    ",
  0x50003304u64 => "
      RISCUTIL_580_GPREG_NL_01.debug_reg(),
    ",
  0x50003308u64 => "
      RISCUTIL_580_GPREG_NL_01.gp_control_reg(),
    ",
  0x50003306u64 => "
      RISCUTIL_580_GPREG_NL_01.gp_status_reg(),
    ",
  0x50003302u64 => "
      RISCUTIL_580_GPREG_NL_01.reset_freeze_reg(),
    ",
  0x50003300u64 => "
      RISCUTIL_580_GPREG_NL_01.set_freeze_reg(),
    ",
  0x50003102u64 => "
      RISCUTIL_580_WDOG_NL_00.watchdog_ctrl_reg(),
    ",
  0x50003100u64 => "
      RISCUTIL_580_WDOG_NL_00.watchdog_reg(),
    ",
  0x50001206u64 => "
      SPI_443_NL_00.spi_clear_int_reg(),
    ",
  0x50001200u64 => "
      SPI_443_NL_00.spi_ctrl_reg(),
    ",
  0x50001208u64 => "
      SPI_443_NL_00.spi_ctrl_reg1(),
    ",
  0x50001202u64 => "
      SPI_443_NL_00.spi_rx_tx_reg0(),
    ",
  0x50001204u64 => "
      SPI_443_NL_00.spi_rx_tx_reg1(),
    ",
  0x50003408u64 => "
      TMR_580_NL_01.pwm2_duty_cycle(),
    ",
  0x5000340au64 => "
      TMR_580_NL_01.pwm3_duty_cycle(),
    ",
  0x5000340cu64 => "
      TMR_580_NL_01.pwm4_duty_cycle(),
    ",
  0x50003400u64 => "
      TMR_580_NL_01.timer0_ctrl_reg(),
    ",
  0x50003402u64 => "
      TMR_580_NL_01.timer0_on_reg(),
    ",
  0x50003404u64 => "
      TMR_580_NL_01.timer0_reload_m_reg(),
    ",
  0x50003406u64 => "
      TMR_580_NL_01.timer0_reload_n_reg(),
    ",
  0x50003410u64 => "
      TMR_580_NL_01.triple_pwm_ctrl_reg(),
    ",
  0x5000340eu64 => "
      TMR_580_NL_01.triple_pwm_frequency(),
    ",
  0x500010f4u64 => "
      UART_1.uart_cpr_reg(),
    ",
  0x500010fcu64 => "
      UART_1.uart_ctr_reg(),
    ",
  0x500010a4u64 => "
      UART_1.uart_htx_reg(),
    ",
  0x50001004u64 => "
      UART_1.uart_ier_dlh_reg(),
    ",
  0x50001008u64 => "
      UART_1.uart_iir_fcr_reg(),
    ",
  0x5000100cu64 => "
      UART_1.uart_lcr_reg(),
    ",
  0x50001024u64 => "
      UART_1.uart_lpdlh_reg(),
    ",
  0x50001020u64 => "
      UART_1.uart_lpdll_reg(),
    ",
  0x50001014u64 => "
      UART_1.uart_lsr_reg(),
    ",
  0x50001010u64 => "
      UART_1.uart_mcr_reg(),
    ",
  0x50001018u64 => "
      UART_1.uart_msr_reg(),
    ",
  0x50001000u64 => "
      UART_1.uart_rbr_thr_dll_reg(),
    ",
  0x50001084u64 => "
      UART_1.uart_rfl_reg(),
    ",
  0x50001090u64 => "
      UART_1.uart_sbcr_reg(),
    ",
  0x5000101cu64 => "
      UART_1.uart_scr_reg(),
    ",
  0x50001094u64 => "
      UART_1.uart_sdmam_reg(),
    ",
  0x50001098u64 => "
      UART_1.uart_sfe_reg(),
    ",
  0x50001030u64 => "
      UART_1.uart_srbr_sthr0_reg(),
    ",
  0x50001058u64 => "
      UART_1.uart_srbr_sthr10_reg(),
    ",
  0x5000105cu64 => "
      UART_1.uart_srbr_sthr11_reg(),
    ",
  0x50001060u64 => "
      UART_1.uart_srbr_sthr12_reg(),
    ",
  0x50001064u64 => "
      UART_1.uart_srbr_sthr13_reg(),
    ",
  0x50001068u64 => "
      UART_1.uart_srbr_sthr14_reg(),
    ",
  0x5000106cu64 => "
      UART_1.uart_srbr_sthr15_reg(),
    ",
  0x50001034u64 => "
      UART_1.uart_srbr_sthr1_reg(),
    ",
  0x50001038u64 => "
      UART_1.uart_srbr_sthr2_reg(),
    ",
  0x5000103cu64 => "
      UART_1.uart_srbr_sthr3_reg(),
    ",
  0x50001040u64 => "
      UART_1.uart_srbr_sthr4_reg(),
    ",
  0x50001044u64 => "
      UART_1.uart_srbr_sthr5_reg(),
    ",
  0x50001048u64 => "
      UART_1.uart_srbr_sthr6_reg(),
    ",
  0x5000104cu64 => "
      UART_1.uart_srbr_sthr7_reg(),
    ",
  0x50001050u64 => "
      UART_1.uart_srbr_sthr8_reg(),
    ",
  0x50001054u64 => "
      UART_1.uart_srbr_sthr9_reg(),
    ",
  0x50001088u64 => "
      UART_1.uart_srr_reg(),
    ",
  0x5000108cu64 => "
      UART_1.uart_srts_reg(),
    ",
  0x5000109cu64 => "
      UART_1.uart_srt_reg(),
    ",
  0x500010a0u64 => "
      UART_1.uart_stet_reg(),
    ",
  0x50001080u64 => "
      UART_1.uart_tfl_reg(),
    ",
  0x500010f8u64 => "
      UART_1.uart_ucv_reg(),
    ",
  0x5000107cu64 => "
      UART_1.uart_usr_reg(),
    ",
  0x500011f4u64 => "
      UART_2.uart2_cpr_reg(),
    ",
  0x500011fcu64 => "
      UART_2.uart2_ctr_reg(),
    ",
  0x500011a4u64 => "
      UART_2.uart2_htx_reg(),
    ",
  0x50001104u64 => "
      UART_2.uart2_ier_dlh_reg(),
    ",
  0x50001108u64 => "
      UART_2.uart2_iir_fcr_reg(),
    ",
  0x5000110cu64 => "
      UART_2.uart2_lcr_reg(),
    ",
  0x50001124u64 => "
      UART_2.uart2_lpdlh_reg(),
    ",
  0x50001120u64 => "
      UART_2.uart2_lpdll_reg(),
    ",
  0x50001114u64 => "
      UART_2.uart2_lsr_reg(),
    ",
  0x50001110u64 => "
      UART_2.uart2_mcr_reg(),
    ",
  0x50001118u64 => "
      UART_2.uart2_msr_reg(),
    ",
  0x50001100u64 => "
      UART_2.uart2_rbr_thr_dll_reg(),
    ",
  0x50001184u64 => "
      UART_2.uart2_rfl_reg(),
    ",
  0x50001190u64 => "
      UART_2.uart2_sbcr_reg(),
    ",
  0x5000111cu64 => "
      UART_2.uart2_scr_reg(),
    ",
  0x50001194u64 => "
      UART_2.uart2_sdmam_reg(),
    ",
  0x50001198u64 => "
      UART_2.uart2_sfe_reg(),
    ",
  0x50001130u64 => "
      UART_2.uart2_srbr_sthr0_reg(),
    ",
  0x50001158u64 => "
      UART_2.uart2_srbr_sthr10_reg(),
    ",
  0x5000115cu64 => "
      UART_2.uart2_srbr_sthr11_reg(),
    ",
  0x50001160u64 => "
      UART_2.uart2_srbr_sthr12_reg(),
    ",
  0x50001164u64 => "
      UART_2.uart2_srbr_sthr13_reg(),
    ",
  0x50001168u64 => "
      UART_2.uart2_srbr_sthr14_reg(),
    ",
  0x5000116cu64 => "
      UART_2.uart2_srbr_sthr15_reg(),
    ",
  0x50001134u64 => "
      UART_2.uart2_srbr_sthr1_reg(),
    ",
  0x50001138u64 => "
      UART_2.uart2_srbr_sthr2_reg(),
    ",
  0x5000113cu64 => "
      UART_2.uart2_srbr_sthr3_reg(),
    ",
  0x50001140u64 => "
      UART_2.uart2_srbr_sthr4_reg(),
    ",
  0x50001144u64 => "
      UART_2.uart2_srbr_sthr5_reg(),
    ",
  0x50001148u64 => "
      UART_2.uart2_srbr_sthr6_reg(),
    ",
  0x5000114cu64 => "
      UART_2.uart2_srbr_sthr7_reg(),
    ",
  0x50001150u64 => "
      UART_2.uart2_srbr_sthr8_reg(),
    ",
  0x50001154u64 => "
      UART_2.uart2_srbr_sthr9_reg(),
    ",
  0x50001188u64 => "
      UART_2.uart2_srr_reg(),
    ",
  0x5000118cu64 => "
      UART_2.uart2_srts_reg(),
    ",
  0x5000119cu64 => "
      UART_2.uart2_srt_reg(),
    ",
  0x500011a0u64 => "
      UART_2.uart2_stet_reg(),
    ",
  0x50001180u64 => "
      UART_2.uart2_tfl_reg(),
    ",
  0x500011f8u64 => "
      UART_2.uart2_ucv_reg(),
    ",
  0x5000117cu64 => "
      UART_2.uart2_usr_reg(),
    ",
  0x50000102u64 => "
      WKUP_580_NL_01.wkup_compare_reg(),
    ",
  0x50000106u64 => "
      WKUP_580_NL_01.wkup_counter_reg(),
    ",
  0x50000100u64 => "
      WKUP_580_NL_01.wkup_ctrl_reg(),
    ",
  0x50000112u64 => "
      WKUP_580_NL_01.wkup_pol_p0_reg(),
    ",
  0x50000114u64 => "
      WKUP_580_NL_01.wkup_pol_p1_reg(),
    ",
  0x50000116u64 => "
      WKUP_580_NL_01.wkup_pol_p2_reg(),
    ",
  0x50000118u64 => "
      WKUP_580_NL_01.wkup_pol_p3_reg(),
    ",
  0x50000108u64 => "
      WKUP_580_NL_01.wkup_reset_cntr_reg(),
    ",
  0x50000104u64 => "
      WKUP_580_NL_01.wkup_reset_irq_reg(),
    ",
  0x5000010au64 => "
      WKUP_580_NL_01.wkup_select_p0_reg(),
    ",
  0x5000010cu64 => "
      WKUP_580_NL_01.wkup_select_p1_reg(),
    ",
  0x5000010eu64 => "
      WKUP_580_NL_01.wkup_select_p2_reg(),
    ",
  0x50000110u64 => "
      WKUP_580_NL_01.wkup_select_p3_reg(),
    ",
};
