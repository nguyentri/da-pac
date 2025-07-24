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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:12 +0000

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
  0xe000e41cu64 => "
      NVIC.ipr7(),
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
  0x40003064u64 => "
      ADPLLDIG.adpll_acc_ctrl_reg(),
    ",
  0x40003060u64 => "
      ADPLLDIG.adpll_anatst_ctrl_reg(),
    ",
  0x40003094u64 => "
      ADPLLDIG.adpll_anatst_rd_reg(),
    ",
  0x40003034u64 => "
      ADPLLDIG.adpll_ana_ctrl_reg(),
    ",
  0x40003000u64 => "
      ADPLLDIG.adpll_attr_ctrl_reg(),
    ",
  0x40003004u64 => "
      ADPLLDIG.adpll_cn_ctrl_reg(),
    ",
  0x40003020u64 => "
      ADPLLDIG.adpll_dcoamp_cal_ctrl_reg(),
    ",
  0x40003080u64 => "
      ADPLLDIG.adpll_dco_rd_reg(),
    ",
  0x40003038u64 => "
      ADPLLDIG.adpll_div_ctrl_reg(),
    ",
  0x40003008u64 => "
      ADPLLDIG.adpll_fif_ctrl1_reg(),
    ",
  0x4000300cu64 => "
      ADPLLDIG.adpll_fif_ctrl2_reg(),
    ",
  0x4000307cu64 => "
      ADPLLDIG.adpll_freqmeas_rd_reg(),
    ",
  0x40003040u64 => "
      ADPLLDIG.adpll_fsm_ctrl_reg(),
    ",
  0x4000303cu64 => "
      ADPLLDIG.adpll_init_ctrl_reg(),
    ",
  0x40003010u64 => "
      ADPLLDIG.adpll_kdco_cal_ctrl1_reg(),
    ",
  0x40003014u64 => "
      ADPLLDIG.adpll_kdco_cal_ctrl2_reg(),
    ",
  0x40003084u64 => "
      ADPLLDIG.adpll_kdco_rd_reg(),
    ",
  0x40003018u64 => "
      ADPLLDIG.adpll_kdtctdc_cal_ctrl1_reg(),
    ",
  0x4000301cu64 => "
      ADPLLDIG.adpll_kdtctdc_cal_ctrl2_reg(),
    ",
  0x40003088u64 => "
      ADPLLDIG.adpll_kdtc_rd_reg(),
    ",
  0x4000302cu64 => "
      ADPLLDIG.adpll_lf_ctrl1_reg(),
    ",
  0x40003030u64 => "
      ADPLLDIG.adpll_lf_ctrl2_reg(),
    ",
  0x40003048u64 => "
      ADPLLDIG.adpll_misc_ctrl_reg(),
    ",
  0x40003044u64 => "
      ADPLLDIG.adpll_mon_ctrl_reg(),
    ",
  0x40003050u64 => "
      ADPLLDIG.adpll_overrule_ctrl1_reg(),
    ",
  0x40003054u64 => "
      ADPLLDIG.adpll_overrule_ctrl2_reg(),
    ",
  0x40003058u64 => "
      ADPLLDIG.adpll_overrule_ctrl3_reg(),
    ",
  0x40003090u64 => "
      ADPLLDIG.adpll_pllfcwdt_rd_reg(),
    ",
  0x4000305cu64 => "
      ADPLLDIG.adpll_rfpt_ctrl_reg(),
    ",
  0x40003028u64 => "
      ADPLLDIG.adpll_sdmod_ctrl_reg(),
    ",
  0x4000308cu64 => "
      ADPLLDIG.adpll_tunestate_rd_reg(),
    ",
  0x40003024u64 => "
      ADPLLDIG.adpll_txmod_ctrl_reg(),
    ",
  0x50001602u64 => "
      ANAMISC.clk_ref_cnt_reg(),
    ",
  0x50001600u64 => "
      ANAMISC.clk_ref_sel_reg(),
    ",
  0x50001606u64 => "
      ANAMISC.clk_ref_val_h_reg(),
    ",
  0x50001604u64 => "
      ANAMISC.clk_ref_val_l_reg(),
    ",
  0x400000a4u64 => "
      BLE.ble_actscanstat_reg(),
    ",
  0x40000090u64 => "
      BLE.ble_advchmap_reg(),
    ",
  0x400000a0u64 => "
      BLE.ble_advtim_reg(),
    ",
  0x400000c0u64 => "
      BLE.ble_aescntl_reg(),
    ",
  0x400000d0u64 => "
      BLE.ble_aeskey127_96_reg(),
    ",
  0x400000c4u64 => "
      BLE.ble_aeskey31_0_reg(),
    ",
  0x400000c8u64 => "
      BLE.ble_aeskey63_32_reg(),
    ",
  0x400000ccu64 => "
      BLE.ble_aeskey95_64_reg(),
    ",
  0x400000d4u64 => "
      BLE.ble_aesptr_reg(),
    ",
  0x40000044u64 => "
      BLE.ble_basetimecntcorr_reg(),
    ",
  0x4000001cu64 => "
      BLE.ble_basetimecnt_reg(),
    ",
  0x40000024u64 => "
      BLE.ble_bdaddrl_reg(),
    ",
  0x40000028u64 => "
      BLE.ble_bdaddru_reg(),
    ",
  0x40000108u64 => "
      BLE.ble_blemprio0_reg(),
    ",
  0x4000010cu64 => "
      BLE.ble_blemprio1_reg(),
    ",
  0x40000200u64 => "
      BLE.ble_cntl2_reg(),
    ",
  0x40000100u64 => "
      BLE.ble_coexifcntl0_reg(),
    ",
  0x40000104u64 => "
      BLE.ble_coexifcntl1_reg(),
    ",
  0x4000002cu64 => "
      BLE.ble_currentrxdescptr_reg(),
    ",
  0x40000058u64 => "
      BLE.ble_debugaddmax_reg(),
    ",
  0x4000005cu64 => "
      BLE.ble_debugaddmin_reg(),
    ",
  0x40000030u64 => "
      BLE.ble_deepslcntl_reg(),
    ",
  0x40000038u64 => "
      BLE.ble_deepslstat_reg(),
    ",
  0x40000034u64 => "
      BLE.ble_deepslwkup_reg(),
    ",
  0x4000020cu64 => "
      BLE.ble_diagcntl2_reg(),
    ",
  0x40000210u64 => "
      BLE.ble_diagcntl3_reg(),
    ",
  0x40000050u64 => "
      BLE.ble_diagcntl_reg(),
    ",
  0x40000054u64 => "
      BLE.ble_diagstat_reg(),
    ",
  0x40000208u64 => "
      BLE.ble_em_base_reg(),
    ",
  0x4000003cu64 => "
      BLE.ble_enbpreset_reg(),
    ",
  0x40000060u64 => "
      BLE.ble_errortypestat_reg(),
    ",
  0x40000040u64 => "
      BLE.ble_finecntcorr_reg(),
    ",
  0x40000020u64 => "
      BLE.ble_finetimecnt_reg(),
    ",
  0x400000f8u64 => "
      BLE.ble_finetimtgt_reg(),
    ",
  0x400000f4u64 => "
      BLE.ble_grosstimtgt_reg(),
    ",
  0x40000018u64 => "
      BLE.ble_intack_reg(),
    ",
  0x4000000cu64 => "
      BLE.ble_intcntl_reg(),
    ",
  0x40000014u64 => "
      BLE.ble_intrawstat_reg(),
    ",
  0x40000010u64 => "
      BLE.ble_intstat_reg(),
    ",
  0x40000070u64 => "
      BLE.ble_radiocntl0_reg(),
    ",
  0x40000074u64 => "
      BLE.ble_radiocntl1_reg(),
    ",
  0x40000078u64 => "
      BLE.ble_radiocntl2_reg(),
    ",
  0x4000007cu64 => "
      BLE.ble_radiocntl3_reg(),
    ",
  0x40000080u64 => "
      BLE.ble_radiopwrupdn_reg(),
    ",
  0x400000e0u64 => "
      BLE.ble_rftestcntl_reg(),
    ",
  0x400000e8u64 => "
      BLE.ble_rftestrxstat_reg(),
    ",
  0x400000e4u64 => "
      BLE.ble_rftesttxstat_reg(),
    ",
  0x40000000u64 => "
      BLE.ble_rwblecntl_reg(),
    ",
  0x40000008u64 => "
      BLE.ble_rwbleconf_reg(),
    ",
  0x400000dcu64 => "
      BLE.ble_rxmicval_reg(),
    ",
  0x400000fcu64 => "
      BLE.ble_sampleclk_reg(),
    ",
  0x40000064u64 => "
      BLE.ble_swprofiling_reg(),
    ",
  0x400000f0u64 => "
      BLE.ble_timgencntl_reg(),
    ",
  0x400000d8u64 => "
      BLE.ble_txmicval_reg(),
    ",
  0x40000004u64 => "
      BLE.ble_version_reg(),
    ",
  0x400000b8u64 => "
      BLE.ble_wlnbdev_reg(),
    ",
  0x400000b4u64 => "
      BLE.ble_wlprivaddptr_reg(),
    ",
  0x400000b0u64 => "
      BLE.ble_wlpubaddptr_reg(),
    ",
  0x50003200u64 => "
      CHIP_VERSION.chip_id1_reg(),
    ",
  0x50003204u64 => "
      CHIP_VERSION.chip_id2_reg(),
    ",
  0x50003208u64 => "
      CHIP_VERSION.chip_id3_reg(),
    ",
  0x5000320cu64 => "
      CHIP_VERSION.chip_id4_reg(),
    ",
  0x50003214u64 => "
      CHIP_VERSION.chip_revision_reg(),
    ",
  0x50003210u64 => "
      CHIP_VERSION.chip_swc_reg(),
    ",
  0x500032f8u64 => "
      CHIP_VERSION.chip_test1_reg(),
    ",
  0x500032fcu64 => "
      CHIP_VERSION.chip_test2_reg(),
    ",
  0x50000324u64 => "
      CRG_AON.gp_data_reg(),
    ",
  0x50000310u64 => "
      CRG_AON.hibern_ctrl_reg(),
    ",
  0x50000300u64 => "
      CRG_AON.hwr_ctrl_reg(),
    ",
  0x5000030cu64 => "
      CRG_AON.pad_latch_reg(),
    ",
  0x50000320u64 => "
      CRG_AON.power_aon_ctrl_reg(),
    ",
  0x50000308u64 => "
      CRG_AON.ram_lpmx_reg(),
    ",
  0x50000304u64 => "
      CRG_AON.reset_stat_reg(),
    ",
  0x500003f0u64 => "
      CRG_AON.test_vdd_reg(),
    ",
  0x5000424cu64 => "
      CRG_TIM.clk_rtcdiv_reg(),
    ",
  0x5000002au64 => "
      CRG_TOP.ana_status_reg(),
    ",
  0x50000028u64 => "
      CRG_TOP.bandgap_reg(),
    ",
  0x50000000u64 => "
      CRG_TOP.clk_amba_reg(),
    ",
  0x5000000au64 => "
      CRG_TOP.clk_ctrl_reg(),
    ",
  0x50000002u64 => "
      CRG_TOP.clk_freq_trim_reg(),
    ",
  0x50000004u64 => "
      CRG_TOP.clk_per_reg(),
    ",
  0x50000008u64 => "
      CRG_TOP.clk_radio_reg(),
    ",
  0x50000020u64 => "
      CRG_TOP.clk_rc32k_reg(),
    ",
  0x50000024u64 => "
      CRG_TOP.clk_rc32m_reg(),
    ",
  0x50000026u64 => "
      CRG_TOP.clk_rcx_reg(),
    ",
  0x50000022u64 => "
      CRG_TOP.clk_xtal32k_reg(),
    ",
  0x50000010u64 => "
      CRG_TOP.pmu_ctrl_reg(),
    ",
  0x50000050u64 => "
      CRG_TOP.pmu_sleep_reg(),
    ",
  0x50000040u64 => "
      CRG_TOP.por_pin_reg(),
    ",
  0x50000042u64 => "
      CRG_TOP.por_timer_reg(),
    ",
  0x50000052u64 => "
      CRG_TOP.power_ctrl_reg(),
    ",
  0x50000054u64 => "
      CRG_TOP.power_level_reg(),
    ",
  0x50000018u64 => "
      CRG_TOP.ram_pwr_ctrl_reg(),
    ",
  0x50000012u64 => "
      CRG_TOP.sys_ctrl_reg(),
    ",
  0x50000014u64 => "
      CRG_TOP.sys_stat_reg(),
    ",
  0x50000016u64 => "
      CRG_TOP.trim_ctrl_reg(),
    ",
  0x50000038u64 => "
      CRG_TOP.xtal32m_ctrl0_reg(),
    ",
  0x50000030u64 => "
      CRG_TOP.xtal32m_start_reg(),
    ",
  0x50000032u64 => "
      CRG_TOP.xtal32m_trstat_reg(),
    ",
  0x50000034u64 => "
      CRG_TOP.xtalrdy_ctrl_reg(),
    ",
  0x50000036u64 => "
      CRG_TOP.xtalrdy_stat_reg(),
    ",
  0x5000150eu64 => "
      GPADC.gp_adc_clear_int_reg(),
    ",
  0x50001502u64 => "
      GPADC.gp_adc_ctrl2_reg(),
    ",
  0x50001504u64 => "
      GPADC.gp_adc_ctrl3_reg(),
    ",
  0x50001500u64 => "
      GPADC.gp_adc_ctrl_reg(),
    ",
  0x5000150au64 => "
      GPADC.gp_adc_offn_reg(),
    ",
  0x50001508u64 => "
      GPADC.gp_adc_offp_reg(),
    ",
  0x5000151cu64 => "
      GPADC.gp_adc_param_dif_reg(),
    ",
  0x5000151eu64 => "
      GPADC.gp_adc_param_se_reg(),
    ",
  0x50001510u64 => "
      GPADC.gp_adc_result_reg(),
    ",
  0x5000150cu64 => "
      GPADC.gp_adc_trim_reg(),
    ",
  0x5000303cu64 => "
      GPIO.bist_ctrl_reg(),
    ",
  0x50003006u64 => "
      GPIO.p00_mode_reg(),
    ",
  0x5000301au64 => "
      GPIO.p010_mode_reg(),
    ",
  0x5000301cu64 => "
      GPIO.p011_mode_reg(),
    ",
  0x50003008u64 => "
      GPIO.p01_mode_reg(),
    ",
  0x5000300au64 => "
      GPIO.p02_mode_reg(),
    ",
  0x5000300cu64 => "
      GPIO.p03_mode_reg(),
    ",
  0x5000300eu64 => "
      GPIO.p04_mode_reg(),
    ",
  0x50003010u64 => "
      GPIO.p05_mode_reg(),
    ",
  0x50003012u64 => "
      GPIO.p06_mode_reg(),
    ",
  0x50003014u64 => "
      GPIO.p07_mode_reg(),
    ",
  0x50003016u64 => "
      GPIO.p08_mode_reg(),
    ",
  0x50003018u64 => "
      GPIO.p09_mode_reg(),
    ",
  0x50003000u64 => "
      GPIO.p0_data_reg(),
    ",
  0x50003004u64 => "
      GPIO.p0_reset_data_reg(),
    ",
  0x50003002u64 => "
      GPIO.p0_set_data_reg(),
    ",
  0x5000301eu64 => "
      GPIO.pad_weak_ctrl_reg(),
    ",
  0x50003040u64 => "
      GPIO.rombist_resulth_reg(),
    ",
  0x5000303eu64 => "
      GPIO.rombist_resultl_reg(),
    ",
  0x50003020u64 => "
      GPIO.scan_observe_reg(),
    ",
  0x50003032u64 => "
      GPIO.test_ctrl2_reg(),
    ",
  0x50003034u64 => "
      GPIO.test_ctrl3_reg(),
    ",
  0x50003036u64 => "
      GPIO.test_ctrl4_reg(),
    ",
  0x50003030u64 => "
      GPIO.test_ctrl_reg(),
    ",
  0x50003038u64 => "
      GPIO.xtal32m_testctrl0_reg(),
    ",
  0x5000303au64 => "
      GPIO.xtal32m_testctrl1_reg(),
    ",
  0x5000330au64 => "
      GPREG.ble_timer_reg(),
    ",
  0x50003304u64 => "
      GPREG.debug_reg(),
    ",
  0x50003308u64 => "
      GPREG.gp_control_reg(),
    ",
  0x50003306u64 => "
      GPREG.gp_status_reg(),
    ",
  0x5000330cu64 => "
      GPREG.mem_ctrl_reg(),
    ",
  0x50003302u64 => "
      GPREG.reset_freeze_reg(),
    ",
  0x50003300u64 => "
      GPREG.set_freeze_reg(),
    ",
  0x50001398u64 => "
      I_2_C.i2c_ack_general_call_reg(),
    ",
  0x5000135cu64 => "
      I_2_C.i2c_clr_activity_reg(),
    ",
  0x50001368u64 => "
      I_2_C.i2c_clr_gen_call_reg(),
    ",
  0x50001340u64 => "
      I_2_C.i2c_clr_intr_reg(),
    ",
  0x50001350u64 => "
      I_2_C.i2c_clr_rd_req_reg(),
    ",
  0x50001358u64 => "
      I_2_C.i2c_clr_rx_done_reg(),
    ",
  0x50001348u64 => "
      I_2_C.i2c_clr_rx_over_reg(),
    ",
  0x50001344u64 => "
      I_2_C.i2c_clr_rx_under_reg(),
    ",
  0x50001364u64 => "
      I_2_C.i2c_clr_start_det_reg(),
    ",
  0x50001360u64 => "
      I_2_C.i2c_clr_stop_det_reg(),
    ",
  0x50001354u64 => "
      I_2_C.i2c_clr_tx_abrt_reg(),
    ",
  0x5000134cu64 => "
      I_2_C.i2c_clr_tx_over_reg(),
    ",
  0x500013fau64 => "
      I_2_C.i2c_comp2_version(),
    ",
  0x500013f4u64 => "
      I_2_C.i2c_comp_param1_reg(),
    ",
  0x500013f6u64 => "
      I_2_C.i2c_comp_param2_reg(),
    ",
  0x500013feu64 => "
      I_2_C.i2c_comp_type2_reg(),
    ",
  0x500013fcu64 => "
      I_2_C.i2c_comp_type_reg(),
    ",
  0x500013f8u64 => "
      I_2_C.i2c_comp_version_reg(),
    ",
  0x50001300u64 => "
      I_2_C.i2c_con_reg(),
    ",
  0x50001310u64 => "
      I_2_C.i2c_data_cmd_reg(),
    ",
  0x50001388u64 => "
      I_2_C.i2c_dma_cr_reg(),
    ",
  0x50001390u64 => "
      I_2_C.i2c_dma_rdlr_reg(),
    ",
  0x5000138cu64 => "
      I_2_C.i2c_dma_tdlr_reg(),
    ",
  0x5000136cu64 => "
      I_2_C.i2c_enable_reg(),
    ",
  0x5000139cu64 => "
      I_2_C.i2c_enable_status_reg(),
    ",
  0x5000131cu64 => "
      I_2_C.i2c_fs_scl_hcnt_reg(),
    ",
  0x50001320u64 => "
      I_2_C.i2c_fs_scl_lcnt_reg(),
    ",
  0x500013a0u64 => "
      I_2_C.i2c_ic_fs_spklen_reg(),
    ",
  0x50001330u64 => "
      I_2_C.i2c_intr_mask_reg(),
    ",
  0x5000132cu64 => "
      I_2_C.i2c_intr_stat_reg(),
    ",
  0x50001334u64 => "
      I_2_C.i2c_raw_intr_stat_reg(),
    ",
  0x50001378u64 => "
      I_2_C.i2c_rxflr_reg(),
    ",
  0x50001338u64 => "
      I_2_C.i2c_rx_tl_reg(),
    ",
  0x50001308u64 => "
      I_2_C.i2c_sar_reg(),
    ",
  0x5000137cu64 => "
      I_2_C.i2c_sda_hold_reg(),
    ",
  0x50001394u64 => "
      I_2_C.i2c_sda_setup_reg(),
    ",
  0x50001314u64 => "
      I_2_C.i2c_ss_scl_hcnt_reg(),
    ",
  0x50001318u64 => "
      I_2_C.i2c_ss_scl_lcnt_reg(),
    ",
  0x50001370u64 => "
      I_2_C.i2c_status_reg(),
    ",
  0x50001304u64 => "
      I_2_C.i2c_tar_reg(),
    ",
  0x50001374u64 => "
      I_2_C.i2c_txflr_reg(),
    ",
  0x50001380u64 => "
      I_2_C.i2c_tx_abrt_source_reg(),
    ",
  0x5000133cu64 => "
      I_2_C.i2c_tx_tl_reg(),
    ",
  0x5000140cu64 => "
      KBRD.gpio_debounce_reg(),
    ",
  0x50001410u64 => "
      KBRD.gpio_int_level_ctrl_reg(),
    ",
  0x50001400u64 => "
      KBRD.gpio_irq0_in_sel_reg(),
    ",
  0x50001402u64 => "
      KBRD.gpio_irq1_in_sel_reg(),
    ",
  0x50001404u64 => "
      KBRD.gpio_irq2_in_sel_reg(),
    ",
  0x50001406u64 => "
      KBRD.gpio_irq3_in_sel_reg(),
    ",
  0x50001408u64 => "
      KBRD.gpio_irq4_in_sel_reg(),
    ",
  0x5000140eu64 => "
      KBRD.gpio_reset_irq_reg(),
    ",
  0x50001414u64 => "
      KBRD.kbrd_ctrl_reg(),
    ",
  0x50001412u64 => "
      KBRD.kbrd_irq_in_sel0_reg(),
    ",
  0x50003700u64 => "
      MBIST_SRAM_12.mbist_sram12_addr_reg(),
    ",
  0x50003706u64 => "
      MBIST_SRAM_12.mbist_sram12_rd_lsb_reg(),
    ",
  0x50003704u64 => "
      MBIST_SRAM_12.mbist_sram12_rd_msb_reg(),
    ",
  0x50003702u64 => "
      MBIST_SRAM_12.mbist_sram12_state_reg(),
    ",
  0x50003800u64 => "
      MBIST_SRAM_3.mbist_sram3_addr_reg(),
    ",
  0x50003806u64 => "
      MBIST_SRAM_3.mbist_sram3_rd_lsb_reg(),
    ",
  0x50003804u64 => "
      MBIST_SRAM_3.mbist_sram3_rd_msb_reg(),
    ",
  0x50003802u64 => "
      MBIST_SRAM_3.mbist_sram3_state_reg(),
    ",
  0x7f40018u64 => "
      OTPC.otpc_ahbadr_reg(),
    ",
  0x7f4001cu64 => "
      OTPC.otpc_celadr_reg(),
    ",
  0x7f40000u64 => "
      OTPC.otpc_mode_reg(),
    ",
  0x7f40020u64 => "
      OTPC.otpc_nwords_reg(),
    ",
  0x7f40008u64 => "
      OTPC.otpc_paddr_reg(),
    ",
  0x7f4000cu64 => "
      OTPC.otpc_pword_reg(),
    ",
  0x7f40004u64 => "
      OTPC.otpc_stat_reg(),
    ",
  0x7f40010u64 => "
      OTPC.otpc_tim1_reg(),
    ",
  0x7f40014u64 => "
      OTPC.otpc_tim2_reg(),
    ",
  0x40080020u64 => "
      PATCH.patch_addr0_reg(),
    ",
  0x40080070u64 => "
      PATCH.patch_addr10_reg(),
    ",
  0x40080078u64 => "
      PATCH.patch_addr11_reg(),
    ",
  0x40080080u64 => "
      PATCH.patch_addr12_reg(),
    ",
  0x40080088u64 => "
      PATCH.patch_addr13_reg(),
    ",
  0x40080090u64 => "
      PATCH.patch_addr14_reg(),
    ",
  0x40080098u64 => "
      PATCH.patch_addr15_reg(),
    ",
  0x400800a0u64 => "
      PATCH.patch_addr16_reg(),
    ",
  0x400800a8u64 => "
      PATCH.patch_addr17_reg(),
    ",
  0x400800b0u64 => "
      PATCH.patch_addr18_reg(),
    ",
  0x400800b8u64 => "
      PATCH.patch_addr19_reg(),
    ",
  0x40080028u64 => "
      PATCH.patch_addr1_reg(),
    ",
  0x400800c0u64 => "
      PATCH.patch_addr20_reg(),
    ",
  0x400800c8u64 => "
      PATCH.patch_addr21_reg(),
    ",
  0x40080030u64 => "
      PATCH.patch_addr2_reg(),
    ",
  0x40080038u64 => "
      PATCH.patch_addr3_reg(),
    ",
  0x40080040u64 => "
      PATCH.patch_addr4_reg(),
    ",
  0x40080048u64 => "
      PATCH.patch_addr5_reg(),
    ",
  0x40080050u64 => "
      PATCH.patch_addr6_reg(),
    ",
  0x40080058u64 => "
      PATCH.patch_addr7_reg(),
    ",
  0x40080060u64 => "
      PATCH.patch_addr8_reg(),
    ",
  0x40080068u64 => "
      PATCH.patch_addr9_reg(),
    ",
  0x400800c4u64 => "
      PATCH.patch_data20_reg(),
    ",
  0x400800ccu64 => "
      PATCH.patch_data21_reg(),
    ",
  0x40080000u64 => "
      PATCH.patch_valid_reg(),
    ",
  0x50000206u64 => "
      QUADEC.qdec_clockdiv_reg(),
    ",
  0x50000208u64 => "
      QUADEC.qdec_ctrl2_reg(),
    ",
  0x50000200u64 => "
      QUADEC.qdec_ctrl_reg(),
    ",
  0x5000020cu64 => "
      QUADEC.qdec_event_cnt_reg(),
    ",
  0x50000202u64 => "
      QUADEC.qdec_xcnt_reg(),
    ",
  0x50000204u64 => "
      QUADEC.qdec_ycnt_reg(),
    ",
  0x5000020au64 => "
      QUADEC.qdec_zcnt_reg(),
    ",
  0x40001028u64 => "
      RFCU.rf_adci_dc_offset_reg(),
    ",
  0x4000102cu64 => "
      RFCU.rf_adcq_dc_offset_reg(),
    ",
  0x40001040u64 => "
      RFCU.rf_adc_ctrl1_reg(),
    ",
  0x40001044u64 => "
      RFCU.rf_adc_ctrl2_reg(),
    ",
  0x40001048u64 => "
      RFCU.rf_adc_ctrl3_reg(),
    ",
  0x4000100cu64 => "
      RFCU.rf_adplldig_ctrl_reg(),
    ",
  0x400010a0u64 => "
      RFCU.rf_adplldig_rfmon_ctrl_reg(),
    ",
  0x40001010u64 => "
      RFCU.rf_agc_ext_lut_reg(),
    ",
  0x40001000u64 => "
      RFCU.rf_attr_reg(),
    ",
  0x40001014u64 => "
      RFCU.rf_calstate_reg(),
    ",
  0x40001020u64 => "
      RFCU.rf_cal_ctrl_reg(),
    ",
  0x400010b0u64 => "
      RFCU.rf_diagirq_ctrl_reg(),
    ",
  0x400010b4u64 => "
      RFCU.rf_diagirq_stat_reg(),
    ",
  0x4000103cu64 => "
      RFCU.rf_iff_ctrl_reg(),
    ",
  0x40001074u64 => "
      RFCU.rf_io_ctrl_reg(),
    ",
  0x40001024u64 => "
      RFCU.rf_irq_ctrl_reg(),
    ",
  0x400010b8u64 => "
      RFCU.rf_ldo_ctrl_reg(),
    ",
  0x40001008u64 => "
      RFCU.rf_ldo_status_reg(),
    ",
  0x40001058u64 => "
      RFCU.rf_ldo_vref_sel_reg(),
    ",
  0x40001078u64 => "
      RFCU.rf_lna_ctrl1_reg(),
    ",
  0x4000107cu64 => "
      RFCU.rf_lna_ctrl2_reg(),
    ",
  0x40001080u64 => "
      RFCU.rf_lna_ctrl3_reg(),
    ",
  0x40001064u64 => "
      RFCU.rf_mixer_ctrl1_reg(),
    ",
  0x40001068u64 => "
      RFCU.rf_mixer_ctrl2_reg(),
    ",
  0x400010acu64 => "
      RFCU.rf_overrule_reg(),
    ",
  0x4000104cu64 => "
      RFCU.rf_pa_ctrl_reg(),
    ",
  0x40001004u64 => "
      RFCU.rf_radio_init_reg(),
    ",
  0x400010a8u64 => "
      RFCU.rf_rfcu_ctrl_reg(),
    ",
  0x40001018u64 => "
      RFCU.rf_scan_feedback_reg(),
    ",
  0x40001030u64 => "
      RFCU.rf_spare_reg(),
    ",
  0x40001380u64 => "
      RFCU_POWER.rf_always_en1_reg(),
    ",
  0x40001384u64 => "
      RFCU_POWER.rf_always_en2_reg(),
    ",
  0x40001324u64 => "
      RFCU_POWER.rf_cntrl_timer_10_reg(),
    ",
  0x40001328u64 => "
      RFCU_POWER.rf_cntrl_timer_11_reg(),
    ",
  0x4000132cu64 => "
      RFCU_POWER.rf_cntrl_timer_12_reg(),
    ",
  0x40001330u64 => "
      RFCU_POWER.rf_cntrl_timer_13_reg(),
    ",
  0x40001334u64 => "
      RFCU_POWER.rf_cntrl_timer_14_reg(),
    ",
  0x40001338u64 => "
      RFCU_POWER.rf_cntrl_timer_15_reg(),
    ",
  0x4000133cu64 => "
      RFCU_POWER.rf_cntrl_timer_16_reg(),
    ",
  0x40001340u64 => "
      RFCU_POWER.rf_cntrl_timer_17_reg(),
    ",
  0x40001344u64 => "
      RFCU_POWER.rf_cntrl_timer_18_reg(),
    ",
  0x40001348u64 => "
      RFCU_POWER.rf_cntrl_timer_19_reg(),
    ",
  0x40001300u64 => "
      RFCU_POWER.rf_cntrl_timer_1_reg(),
    ",
  0x4000134cu64 => "
      RFCU_POWER.rf_cntrl_timer_20_reg(),
    ",
  0x40001350u64 => "
      RFCU_POWER.rf_cntrl_timer_21_reg(),
    ",
  0x40001354u64 => "
      RFCU_POWER.rf_cntrl_timer_22_reg(),
    ",
  0x40001358u64 => "
      RFCU_POWER.rf_cntrl_timer_23_reg(),
    ",
  0x4000135cu64 => "
      RFCU_POWER.rf_cntrl_timer_24_reg(),
    ",
  0x40001360u64 => "
      RFCU_POWER.rf_cntrl_timer_25_reg(),
    ",
  0x40001364u64 => "
      RFCU_POWER.rf_cntrl_timer_26_reg(),
    ",
  0x40001368u64 => "
      RFCU_POWER.rf_cntrl_timer_27_reg(),
    ",
  0x4000136cu64 => "
      RFCU_POWER.rf_cntrl_timer_28_reg(),
    ",
  0x40001370u64 => "
      RFCU_POWER.rf_cntrl_timer_29_reg(),
    ",
  0x40001304u64 => "
      RFCU_POWER.rf_cntrl_timer_2_reg(),
    ",
  0x40001374u64 => "
      RFCU_POWER.rf_cntrl_timer_30_reg(),
    ",
  0x40001378u64 => "
      RFCU_POWER.rf_cntrl_timer_31_reg(),
    ",
  0x40001308u64 => "
      RFCU_POWER.rf_cntrl_timer_3_reg(),
    ",
  0x4000130cu64 => "
      RFCU_POWER.rf_cntrl_timer_4_reg(),
    ",
  0x40001310u64 => "
      RFCU_POWER.rf_cntrl_timer_5_reg(),
    ",
  0x40001314u64 => "
      RFCU_POWER.rf_cntrl_timer_6_reg(),
    ",
  0x40001318u64 => "
      RFCU_POWER.rf_cntrl_timer_7_reg(),
    ",
  0x4000131cu64 => "
      RFCU_POWER.rf_cntrl_timer_8_reg(),
    ",
  0x40001320u64 => "
      RFCU_POWER.rf_cntrl_timer_9_reg(),
    ",
  0x40001200u64 => "
      RFCU_POWER.rf_enable_config0_reg(),
    ",
  0x40001228u64 => "
      RFCU_POWER.rf_enable_config10_reg(),
    ",
  0x4000122cu64 => "
      RFCU_POWER.rf_enable_config11_reg(),
    ",
  0x40001230u64 => "
      RFCU_POWER.rf_enable_config12_reg(),
    ",
  0x40001234u64 => "
      RFCU_POWER.rf_enable_config13_reg(),
    ",
  0x40001238u64 => "
      RFCU_POWER.rf_enable_config14_reg(),
    ",
  0x4000123cu64 => "
      RFCU_POWER.rf_enable_config15_reg(),
    ",
  0x40001240u64 => "
      RFCU_POWER.rf_enable_config16_reg(),
    ",
  0x40001244u64 => "
      RFCU_POWER.rf_enable_config17_reg(),
    ",
  0x40001248u64 => "
      RFCU_POWER.rf_enable_config18_reg(),
    ",
  0x4000124cu64 => "
      RFCU_POWER.rf_enable_config19_reg(),
    ",
  0x40001204u64 => "
      RFCU_POWER.rf_enable_config1_reg(),
    ",
  0x40001250u64 => "
      RFCU_POWER.rf_enable_config20_reg(),
    ",
  0x40001254u64 => "
      RFCU_POWER.rf_enable_config21_reg(),
    ",
  0x40001258u64 => "
      RFCU_POWER.rf_enable_config22_reg(),
    ",
  0x4000125cu64 => "
      RFCU_POWER.rf_enable_config23_reg(),
    ",
  0x40001260u64 => "
      RFCU_POWER.rf_enable_config24_reg(),
    ",
  0x40001264u64 => "
      RFCU_POWER.rf_enable_config25_reg(),
    ",
  0x40001268u64 => "
      RFCU_POWER.rf_enable_config26_reg(),
    ",
  0x4000126cu64 => "
      RFCU_POWER.rf_enable_config27_reg(),
    ",
  0x40001270u64 => "
      RFCU_POWER.rf_enable_config28_reg(),
    ",
  0x40001274u64 => "
      RFCU_POWER.rf_enable_config29_reg(),
    ",
  0x40001208u64 => "
      RFCU_POWER.rf_enable_config2_reg(),
    ",
  0x40001278u64 => "
      RFCU_POWER.rf_enable_config30_reg(),
    ",
  0x4000127cu64 => "
      RFCU_POWER.rf_enable_config31_reg(),
    ",
  0x40001280u64 => "
      RFCU_POWER.rf_enable_config32_reg(),
    ",
  0x40001284u64 => "
      RFCU_POWER.rf_enable_config33_reg(),
    ",
  0x40001288u64 => "
      RFCU_POWER.rf_enable_config34_reg(),
    ",
  0x4000128cu64 => "
      RFCU_POWER.rf_enable_config35_reg(),
    ",
  0x40001290u64 => "
      RFCU_POWER.rf_enable_config36_reg(),
    ",
  0x40001294u64 => "
      RFCU_POWER.rf_enable_config37_reg(),
    ",
  0x40001298u64 => "
      RFCU_POWER.rf_enable_config38_reg(),
    ",
  0x4000129cu64 => "
      RFCU_POWER.rf_enable_config39_reg(),
    ",
  0x4000120cu64 => "
      RFCU_POWER.rf_enable_config3_reg(),
    ",
  0x400012a0u64 => "
      RFCU_POWER.rf_enable_config40_reg(),
    ",
  0x400012a4u64 => "
      RFCU_POWER.rf_enable_config41_reg(),
    ",
  0x400012a8u64 => "
      RFCU_POWER.rf_enable_config42_reg(),
    ",
  0x400012acu64 => "
      RFCU_POWER.rf_enable_config43_reg(),
    ",
  0x400012b0u64 => "
      RFCU_POWER.rf_enable_config44_reg(),
    ",
  0x400012b4u64 => "
      RFCU_POWER.rf_enable_config45_reg(),
    ",
  0x400012b8u64 => "
      RFCU_POWER.rf_enable_config46_reg(),
    ",
  0x40001210u64 => "
      RFCU_POWER.rf_enable_config4_reg(),
    ",
  0x40001214u64 => "
      RFCU_POWER.rf_enable_config5_reg(),
    ",
  0x40001218u64 => "
      RFCU_POWER.rf_enable_config6_reg(),
    ",
  0x4000121cu64 => "
      RFCU_POWER.rf_enable_config7_reg(),
    ",
  0x40001220u64 => "
      RFCU_POWER.rf_enable_config8_reg(),
    ",
  0x40001224u64 => "
      RFCU_POWER.rf_enable_config9_reg(),
    ",
  0x40001388u64 => "
      RFCU_POWER.rf_port_en_reg(),
    ",
  0x4000138cu64 => "
      RFCU_POWER.rf_port_pol_reg(),
    ",
  0x50003504u64 => "
      RFMON.rfmon_addr_reg(),
    ",
  0x50003510u64 => "
      RFMON.rfmon_crv_addr_reg(),
    ",
  0x50003514u64 => "
      RFMON.rfmon_crv_len_reg(),
    ",
  0x50003500u64 => "
      RFMON.rfmon_ctrl_reg(),
    ",
  0x50003508u64 => "
      RFMON.rfmon_len_reg(),
    ",
  0x5000350cu64 => "
      RFMON.rfmon_stat_reg(),
    ",
  0x50004118u64 => "
      RTC.rtc_alarm_enable_reg(),
    ",
  0x50004114u64 => "
      RTC.rtc_calendar_alarm_reg(),
    ",
  0x5000410cu64 => "
      RTC.rtc_calendar_reg(),
    ",
  0x50004100u64 => "
      RTC.rtc_control_reg(),
    ",
  0x5000411cu64 => "
      RTC.rtc_event_flags_reg(),
    ",
  0x50004104u64 => "
      RTC.rtc_hour_mode_reg(),
    ",
  0x50004124u64 => "
      RTC.rtc_interrupt_disable_reg(),
    ",
  0x50004120u64 => "
      RTC.rtc_interrupt_enable_reg(),
    ",
  0x50004128u64 => "
      RTC.rtc_interrupt_mask_reg(),
    ",
  0x50004130u64 => "
      RTC.rtc_keep_rtc_reg(),
    ",
  0x5000412cu64 => "
      RTC.rtc_status_reg(),
    ",
  0x50004110u64 => "
      RTC.rtc_time_alarm_reg(),
    ",
  0x50004108u64 => "
      RTC.rtc_time_reg(),
    ",
  0x50001208u64 => "
      SPI.spi_clock_reg(),
    ",
  0x50001204u64 => "
      SPI.spi_config_reg(),
    ",
  0x50001224u64 => "
      SPI.spi_cs_config_reg(),
    ",
  0x50001200u64 => "
      SPI.spi_ctrl_reg(),
    ",
  0x5000120cu64 => "
      SPI.spi_fifo_config_reg(),
    ",
  0x50001228u64 => "
      SPI.spi_fifo_high_reg(),
    ",
  0x5000121cu64 => "
      SPI.spi_fifo_read_reg(),
    ",
  0x50001218u64 => "
      SPI.spi_fifo_status_reg(),
    ",
  0x50001220u64 => "
      SPI.spi_fifo_write_reg(),
    ",
  0x50001210u64 => "
      SPI.spi_irq_mask_reg(),
    ",
  0x50001214u64 => "
      SPI.spi_status_reg(),
    ",
  0x50001230u64 => "
      SPI.spi_txbuffer_force_h_reg(),
    ",
  0x5000122cu64 => "
      SPI.spi_txbuffer_force_l_reg(),
    ",
  0x50003102u64 => "
      SYS_WDOG.watchdog_ctrl_reg(),
    ",
  0x50003100u64 => "
      SYS_WDOG.watchdog_reg(),
    ",
  0x50003416u64 => "
      TIMER_0.pwm2_end_cycle(),
    ",
  0x5000340au64 => "
      TIMER_0.pwm2_start_cycle(),
    ",
  0x50003418u64 => "
      TIMER_0.pwm3_end_cycle(),
    ",
  0x5000340cu64 => "
      TIMER_0.pwm3_start_cycle(),
    ",
  0x5000341au64 => "
      TIMER_0.pwm4_end_cycle(),
    ",
  0x5000340eu64 => "
      TIMER_0.pwm4_start_cycle(),
    ",
  0x5000341cu64 => "
      TIMER_0.pwm5_end_cycle(),
    ",
  0x50003410u64 => "
      TIMER_0.pwm5_start_cycle(),
    ",
  0x5000341eu64 => "
      TIMER_0.pwm6_end_cycle(),
    ",
  0x50003412u64 => "
      TIMER_0.pwm6_start_cycle(),
    ",
  0x50003420u64 => "
      TIMER_0.pwm7_end_cycle(),
    ",
  0x50003414u64 => "
      TIMER_0.pwm7_start_cycle(),
    ",
  0x50003400u64 => "
      TIMER_0.timer0_ctrl_reg(),
    ",
  0x50003402u64 => "
      TIMER_0.timer0_on_reg(),
    ",
  0x50003404u64 => "
      TIMER_0.timer0_reload_m_reg(),
    ",
  0x50003406u64 => "
      TIMER_0.timer0_reload_n_reg(),
    ",
  0x50003422u64 => "
      TIMER_0.triple_pwm_ctrl_reg(),
    ",
  0x50003408u64 => "
      TIMER_0.triple_pwm_frequency(),
    ",
  0x5000400cu64 => "
      TIMER_1.timer1_capcnt1_value_reg(),
    ",
  0x50004010u64 => "
      TIMER_1.timer1_capcnt2_value_reg(),
    ",
  0x50004004u64 => "
      TIMER_1.timer1_capture_reg(),
    ",
  0x50004014u64 => "
      TIMER_1.timer1_clr_event_reg(),
    ",
  0x50004000u64 => "
      TIMER_1.timer1_ctrl_reg(),
    ",
  0x50004008u64 => "
      TIMER_1.timer1_status_reg(),
    ",
  0x500010feu64 => "
      UART.uart_ctr_high_reg(),
    ",
  0x500010fcu64 => "
      UART.uart_ctr_reg(),
    ",
  0x500010c0u64 => "
      UART.uart_dlf_reg(),
    ",
  0x500010a8u64 => "
      UART.uart_dmasa_reg(),
    ",
  0x50001070u64 => "
      UART.uart_far_reg(),
    ",
  0x500010a4u64 => "
      UART.uart_htx_reg(),
    ",
  0x50001004u64 => "
      UART.uart_ier_dlh_reg(),
    ",
  0x50001008u64 => "
      UART.uart_iir_fcr_reg(),
    ",
  0x5000100cu64 => "
      UART.uart_lcr_reg(),
    ",
  0x50001014u64 => "
      UART.uart_lsr_reg(),
    ",
  0x50001010u64 => "
      UART.uart_mcr_reg(),
    ",
  0x50001018u64 => "
      UART.uart_msr_reg(),
    ",
  0x50001000u64 => "
      UART.uart_rbr_thr_dll_reg(),
    ",
  0x50001084u64 => "
      UART.uart_rfl_reg(),
    ",
  0x50001090u64 => "
      UART.uart_sbcr_reg(),
    ",
  0x5000101cu64 => "
      UART.uart_scr_reg(),
    ",
  0x50001094u64 => "
      UART.uart_sdmam_reg(),
    ",
  0x50001098u64 => "
      UART.uart_sfe_reg(),
    ",
  0x50001030u64 => "
      UART.uart_srbr_sthr0_reg(),
    ",
  0x50001058u64 => "
      UART.uart_srbr_sthr10_reg(),
    ",
  0x5000105cu64 => "
      UART.uart_srbr_sthr11_reg(),
    ",
  0x50001060u64 => "
      UART.uart_srbr_sthr12_reg(),
    ",
  0x50001064u64 => "
      UART.uart_srbr_sthr13_reg(),
    ",
  0x50001068u64 => "
      UART.uart_srbr_sthr14_reg(),
    ",
  0x5000106cu64 => "
      UART.uart_srbr_sthr15_reg(),
    ",
  0x50001034u64 => "
      UART.uart_srbr_sthr1_reg(),
    ",
  0x50001038u64 => "
      UART.uart_srbr_sthr2_reg(),
    ",
  0x5000103cu64 => "
      UART.uart_srbr_sthr3_reg(),
    ",
  0x50001040u64 => "
      UART.uart_srbr_sthr4_reg(),
    ",
  0x50001044u64 => "
      UART.uart_srbr_sthr5_reg(),
    ",
  0x50001048u64 => "
      UART.uart_srbr_sthr6_reg(),
    ",
  0x5000104cu64 => "
      UART.uart_srbr_sthr7_reg(),
    ",
  0x50001050u64 => "
      UART.uart_srbr_sthr8_reg(),
    ",
  0x50001054u64 => "
      UART.uart_srbr_sthr9_reg(),
    ",
  0x50001088u64 => "
      UART.uart_srr_reg(),
    ",
  0x5000108cu64 => "
      UART.uart_srts_reg(),
    ",
  0x5000109cu64 => "
      UART.uart_srt_reg(),
    ",
  0x500010a0u64 => "
      UART.uart_stet_reg(),
    ",
  0x50001080u64 => "
      UART.uart_tfl_reg(),
    ",
  0x500010fau64 => "
      UART.uart_ucv_high_reg(),
    ",
  0x500010f8u64 => "
      UART.uart_ucv_reg(),
    ",
  0x5000107cu64 => "
      UART.uart_usr_reg(),
    ",
  0x500011feu64 => "
      UART_2.uart2_ctr_high_reg(),
    ",
  0x500011fcu64 => "
      UART_2.uart2_ctr_reg(),
    ",
  0x500011c0u64 => "
      UART_2.uart2_dlf_reg(),
    ",
  0x500011a8u64 => "
      UART_2.uart2_dmasa_reg(),
    ",
  0x50001170u64 => "
      UART_2.uart2_far_reg(),
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
  0x50001114u64 => "
      UART_2.uart2_lsr_reg(),
    ",
  0x50001110u64 => "
      UART_2.uart2_mcr_reg(),
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
  0x5000119cu64 => "
      UART_2.uart2_srt_reg(),
    ",
  0x500011a0u64 => "
      UART_2.uart2_stet_reg(),
    ",
  0x50001180u64 => "
      UART_2.uart2_tfl_reg(),
    ",
  0x500011fau64 => "
      UART_2.uart2_ucv_high_reg(),
    ",
  0x500011f8u64 => "
      UART_2.uart2_ucv_reg(),
    ",
  0x5000117cu64 => "
      UART_2.uart2_usr_reg(),
    ",
  0x5000010eu64 => "
      WKUP.wkup2_pol_gpio_reg(),
    ",
  0x5000010au64 => "
      WKUP.wkup2_select_gpio_reg(),
    ",
  0x50000102u64 => "
      WKUP.wkup_compare_reg(),
    ",
  0x50000106u64 => "
      WKUP.wkup_counter_reg(),
    ",
  0x50000100u64 => "
      WKUP.wkup_ctrl_reg(),
    ",
  0x50000104u64 => "
      WKUP.wkup_irq_status_reg(),
    ",
  0x5000010cu64 => "
      WKUP.wkup_pol_gpio_reg(),
    ",
  0x50000108u64 => "
      WKUP.wkup_select_gpio_reg(),
    ",
};
