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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:38 +0000

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
  0xe000edd0u64 => "
      SAU.ctrl(),
    ",
  0xe000edd4u64 => "
      SAU.r#type(),
    ",
  0xe000edd8u64 => "
      SAU.rnr(),
    ",
  0xe000eddcu64 => "
      SAU.rbar(),
    ",
  0xe000ede0u64 => "
      SAU.rlar(),
    ",
  0x30040018u64 => "
      AES_HASH.crypto_clrirq_reg(),
    ",
  0x30040000u64 => "
      AES_HASH.crypto_ctrl_reg(),
    ",
  0x30040010u64 => "
      AES_HASH.crypto_dest_addr_reg(),
    ",
  0x30040008u64 => "
      AES_HASH.crypto_fetch_addr_reg(),
    ",
  0x30040100u64 => "
      AES_HASH.crypto_keys_start(),
    ",
  0x3004000cu64 => "
      AES_HASH.crypto_len_reg(),
    ",
  0x3004001cu64 => "
      AES_HASH.crypto_mreg0_reg(),
    ",
  0x30040020u64 => "
      AES_HASH.crypto_mreg1_reg(),
    ",
  0x30040024u64 => "
      AES_HASH.crypto_mreg2_reg(),
    ",
  0x30040028u64 => "
      AES_HASH.crypto_mreg3_reg(),
    ",
  0x30040004u64 => "
      AES_HASH.crypto_start_reg(),
    ",
  0x30040014u64 => "
      AES_HASH.crypto_status_reg(),
    ",
  0x50030b14u64 => "
      ANAMISC.clk_ref_cnt_reg(),
    ",
  0x50030b10u64 => "
      ANAMISC.clk_ref_sel_reg(),
    ",
  0x50030b18u64 => "
      ANAMISC.clk_ref_val_reg(),
    ",
  0x5003061cu64 => "
      APU.apu_mux_reg(),
    ",
  0x50030634u64 => "
      APU.coef0a_set1_reg(),
    ",
  0x50030620u64 => "
      APU.coef10_set1_reg(),
    ",
  0x50030624u64 => "
      APU.coef32_set1_reg(),
    ",
  0x50030628u64 => "
      APU.coef54_set1_reg(),
    ",
  0x5003062cu64 => "
      APU.coef76_set1_reg(),
    ",
  0x50030630u64 => "
      APU.coef98_set1_reg(),
    ",
  0x50030700u64 => "
      APU.pcm1_ctrl_reg(),
    ",
  0x50030704u64 => "
      APU.pcm1_in1_reg(),
    ",
  0x50030708u64 => "
      APU.pcm1_in2_reg(),
    ",
  0x5003070cu64 => "
      APU.pcm1_out1_reg(),
    ",
  0x50030710u64 => "
      APU.pcm1_out2_reg(),
    ",
  0x50030600u64 => "
      APU.src1_ctrl_reg(),
    ",
  0x5003060cu64 => "
      APU.src1_in1_reg(),
    ",
  0x50030610u64 => "
      APU.src1_in2_reg(),
    ",
  0x50030604u64 => "
      APU.src1_in_fs_reg(),
    ",
  0x50030614u64 => "
      APU.src1_out1_reg(),
    ",
  0x50030618u64 => "
      APU.src1_out2_reg(),
    ",
  0x50030608u64 => "
      APU.src1_out_fs_reg(),
    ",
  0x100c0008u64 => "
      CACHE.cache_assoccfg_reg(),
    ",
  0x100c0000u64 => "
      CACHE.cache_ctrl1_reg(),
    ",
  0x100c0020u64 => "
      CACHE.cache_ctrl2_reg(),
    ",
  0x100c0040u64 => "
      CACHE.cache_flash_reg(),
    ",
  0x100c0004u64 => "
      CACHE.cache_lnsizecfg_reg(),
    ",
  0x100c0030u64 => "
      CACHE.cache_mrm_ctrl_reg(),
    ",
  0x100c0028u64 => "
      CACHE.cache_mrm_hits_reg(),
    ",
  0x100c003cu64 => "
      CACHE.cache_mrm_hits_thres_reg(),
    ",
  0x100c002cu64 => "
      CACHE.cache_mrm_misses_reg(),
    ",
  0x100c0038u64 => "
      CACHE.cache_mrm_misses_thres_reg(),
    ",
  0x100c0034u64 => "
      CACHE.cache_mrm_tint_reg(),
    ",
  0x100c0050u64 => "
      CACHE.swd_reset_reg(),
    ",
  0x5004041cu64 => "
      CHARGER.charger_cc_charge_timer_reg(),
    ",
  0x50040400u64 => "
      CHARGER.charger_ctrl_reg(),
    ",
  0x50040410u64 => "
      CHARGER.charger_current_param_reg(),
    ",
  0x50040420u64 => "
      CHARGER.charger_cv_charge_timer_reg(),
    ",
  0x5004046cu64 => "
      CHARGER.charger_error_irq_clr_reg(),
    ",
  0x5004045cu64 => "
      CHARGER.charger_error_irq_mask_reg(),
    ",
  0x50040464u64 => "
      CHARGER.charger_error_irq_status_reg(),
    ",
  0x50040438u64 => "
      CHARGER.charger_jeita_current_reg(),
    ",
  0x50040428u64 => "
      CHARGER.charger_jeita_v_charge_reg(),
    ",
  0x50040434u64 => "
      CHARGER.charger_jeita_v_ovp_reg(),
    ",
  0x5004042cu64 => "
      CHARGER.charger_jeita_v_precharge_reg(),
    ",
  0x50040430u64 => "
      CHARGER.charger_jeita_v_replenish_reg(),
    ",
  0x50040418u64 => "
      CHARGER.charger_pre_charge_timer_reg(),
    ",
  0x50040454u64 => "
      CHARGER.charger_pwr_up_timer_reg(),
    ",
  0x50040468u64 => "
      CHARGER.charger_state_irq_clr_reg(),
    ",
  0x50040458u64 => "
      CHARGER.charger_state_irq_mask_reg(),
    ",
  0x50040460u64 => "
      CHARGER.charger_state_irq_status_reg(),
    ",
  0x50040408u64 => "
      CHARGER.charger_status_reg(),
    ",
  0x5004044cu64 => "
      CHARGER.charger_tbat_comp_timer_reg(),
    ",
  0x50040448u64 => "
      CHARGER.charger_tbat_mon_timer_reg(),
    ",
  0x50040444u64 => "
      CHARGER.charger_tdie_comp_timer_reg(),
    ",
  0x50040414u64 => "
      CHARGER.charger_tempset_param_reg(),
    ",
  0x50040450u64 => "
      CHARGER.charger_thot_comp_timer_reg(),
    ",
  0x50040424u64 => "
      CHARGER.charger_total_charge_timer_reg(),
    ",
  0x5004043cu64 => "
      CHARGER.charger_vbat_comp_timer_reg(),
    ",
  0x5004040cu64 => "
      CHARGER.charger_voltage_param_reg(),
    ",
  0x50040440u64 => "
      CHARGER.charger_vovp_comp_timer_reg(),
    ",
  0x50040200u64 => "
      CHIP_VERSION.chip_id1_reg(),
    ",
  0x50040204u64 => "
      CHIP_VERSION.chip_id2_reg(),
    ",
  0x50040208u64 => "
      CHIP_VERSION.chip_id3_reg(),
    ",
  0x5004020cu64 => "
      CHIP_VERSION.chip_id4_reg(),
    ",
  0x50040214u64 => "
      CHIP_VERSION.chip_revision_reg(),
    ",
  0x50040210u64 => "
      CHIP_VERSION.chip_swc_reg(),
    ",
  0x500402f8u64 => "
      CHIP_VERSION.chip_test1_reg(),
    ",
  0x500402fcu64 => "
      CHIP_VERSION.chip_test2_reg(),
    ",
  0x40002000u64 => "
      CMAC.cm_ctrl_sys_reg(),
    ",
  0x40002104u64 => "
      CMAC.cm_diag_irq1_edge_reg(),
    ",
  0x4000210cu64 => "
      CMAC.cm_diag_irq1_mask_reg(),
    ",
  0x40002108u64 => "
      CMAC.cm_diag_irq1_stat_reg(),
    ",
  0x40002100u64 => "
      CMAC.cm_diag_irq1_word_reg(),
    ",
  0x40002004u64 => "
      CMAC.cm_wdog_reg(),
    ",
  0x50010404u64 => "
      CMAC_TIMER_SLP.cm_slp_ctrl2_reg(),
    ",
  0x50010400u64 => "
      CMAC_TIMER_SLP.cm_slp_ctrl_reg(),
    ",
  0x50010408u64 => "
      CMAC_TIMER_SLP.cm_slp_timer_reg(),
    ",
  0x50020904u64 => "
      CRG_COM.clk_com_reg(),
    ",
  0x5002090cu64 => "
      CRG_COM.reset_clk_com_reg(),
    ",
  0x50020908u64 => "
      CRG_COM.set_clk_com_reg(),
    ",
  0x50030c04u64 => "
      CRG_PER.clk_per_reg(),
    ",
  0x50030c40u64 => "
      CRG_PER.pcm_div_reg(),
    ",
  0x50030c44u64 => "
      CRG_PER.pcm_fdiv_reg(),
    ",
  0x50030c48u64 => "
      CRG_PER.pdm_div_reg(),
    ",
  0x50030c0cu64 => "
      CRG_PER.reset_clk_per_reg(),
    ",
  0x50030c08u64 => "
      CRG_PER.set_clk_per_reg(),
    ",
  0x50030c4cu64 => "
      CRG_PER.src_div_reg(),
    ",
  0x50040504u64 => "
      CRG_SYS.batcheck_reg(),
    ",
  0x50040500u64 => "
      CRG_SYS.clk_sys_reg(),
    ",
  0x500000ecu64 => "
      CRG_TOP.ana_status_reg(),
    ",
  0x50000050u64 => "
      CRG_TOP.bandgap_reg(),
    ",
  0x50000064u64 => "
      CRG_TOP.bod_lvl_ctrl0_reg(),
    ",
  0x50000068u64 => "
      CRG_TOP.bod_lvl_ctrl1_reg(),
    ",
  0x5000006cu64 => "
      CRG_TOP.bod_lvl_ctrl2_reg(),
    ",
  0x50000090u64 => "
      CRG_TOP.bod_status_reg(),
    ",
  0x50000000u64 => "
      CRG_TOP.clk_amba_reg(),
    ",
  0x50000014u64 => "
      CRG_TOP.clk_ctrl_reg(),
    ",
  0x50000010u64 => "
      CRG_TOP.clk_radio_reg(),
    ",
  0x5000003cu64 => "
      CRG_TOP.clk_rc32k_reg(),
    ",
  0x50000048u64 => "
      CRG_TOP.clk_rcx_reg(),
    ",
  0x5000001cu64 => "
      CRG_TOP.clk_switch2xtal_reg(),
    ",
  0x50000018u64 => "
      CRG_TOP.clk_tmr_reg(),
    ",
  0x500000d4u64 => "
      CRG_TOP.discharge_rail_reg(),
    ",
  0x500000a0u64 => "
      CRG_TOP.ldo_vddd_high_ctrl_reg(),
    ",
  0x50000070u64 => "
      CRG_TOP.p0_pad_latch_reg(),
    ",
  0x50000078u64 => "
      CRG_TOP.p0_reset_pad_latch_reg(),
    ",
  0x50000074u64 => "
      CRG_TOP.p0_set_pad_latch_reg(),
    ",
  0x5000007cu64 => "
      CRG_TOP.p1_pad_latch_reg(),
    ",
  0x50000084u64 => "
      CRG_TOP.p1_reset_pad_latch_reg(),
    ",
  0x50000080u64 => "
      CRG_TOP.p1_set_pad_latch_reg(),
    ",
  0x50000020u64 => "
      CRG_TOP.pmu_ctrl_reg(),
    ",
  0x500000f4u64 => "
      CRG_TOP.pmu_sleep_reg(),
    ",
  0x500000f8u64 => "
      CRG_TOP.pmu_trim_reg(),
    ",
  0x50000098u64 => "
      CRG_TOP.por_pin_reg(),
    ",
  0x5000009cu64 => "
      CRG_TOP.por_timer_reg(),
    ",
  0x50000094u64 => "
      CRG_TOP.por_vbat_ctrl_reg(),
    ",
  0x500000c0u64 => "
      CRG_TOP.ram_pwr_ctrl_reg(),
    ",
  0x500000bcu64 => "
      CRG_TOP.reset_stat_reg(),
    ",
  0x500000ccu64 => "
      CRG_TOP.secure_boot_reg(),
    ",
  0x50000028u64 => "
      CRG_TOP.sys_stat_reg(),
    ",
  0x50000058u64 => "
      CRG_TOP.vbus_irq_clear_reg(),
    ",
  0x50010000u64 => "
      CRG_XTAL.clk_freq_trim_reg(),
    ",
  0x50010070u64 => "
      CRG_XTAL.pll_sys_status_reg(),
    ",
  0x50010010u64 => "
      CRG_XTAL.trim_ctrl_reg(),
    ",
  0x50010018u64 => "
      CRG_XTAL.xtalrdy_ctrl_reg(),
    ",
  0x50000304u64 => "
      DCDC.dcdc_ctrl1_reg(),
    ",
  0x50000308u64 => "
      DCDC.dcdc_ctrl2_reg(),
    ",
  0x50000334u64 => "
      DCDC.dcdc_irq_clear_reg(),
    ",
  0x50000338u64 => "
      DCDC.dcdc_irq_mask_reg(),
    ",
  0x50000330u64 => "
      DCDC.dcdc_irq_status_reg(),
    ",
  0x50000320u64 => "
      DCDC.dcdc_status1_reg(),
    ",
  0x50000324u64 => "
      DCDC.dcdc_status2_reg(),
    ",
  0x50000328u64 => "
      DCDC.dcdc_status3_reg(),
    ",
  0x5000032cu64 => "
      DCDC.dcdc_status4_reg(),
    ",
  0x5000031cu64 => "
      DCDC.dcdc_test_reg(),
    ",
  0x5000030cu64 => "
      DCDC.dcdc_v14_reg(),
    ",
  0x50000318u64 => "
      DCDC.dcdc_v18p_reg(),
    ",
  0x50000314u64 => "
      DCDC.dcdc_v18_reg(),
    ",
  0x50000310u64 => "
      DCDC.dcdc_vdd_reg(),
    ",
  0x50040800u64 => "
      DMA.dma0_a_start_reg(),
    ",
  0x50040804u64 => "
      DMA.dma0_b_start_reg(),
    ",
  0x50040810u64 => "
      DMA.dma0_ctrl_reg(),
    ",
  0x50040814u64 => "
      DMA.dma0_idx_reg(),
    ",
  0x50040808u64 => "
      DMA.dma0_int_reg(),
    ",
  0x5004080cu64 => "
      DMA.dma0_len_reg(),
    ",
  0x50040820u64 => "
      DMA.dma1_a_start_reg(),
    ",
  0x50040824u64 => "
      DMA.dma1_b_start_reg(),
    ",
  0x50040830u64 => "
      DMA.dma1_ctrl_reg(),
    ",
  0x50040834u64 => "
      DMA.dma1_idx_reg(),
    ",
  0x50040828u64 => "
      DMA.dma1_int_reg(),
    ",
  0x5004082cu64 => "
      DMA.dma1_len_reg(),
    ",
  0x50040840u64 => "
      DMA.dma2_a_start_reg(),
    ",
  0x50040844u64 => "
      DMA.dma2_b_start_reg(),
    ",
  0x50040850u64 => "
      DMA.dma2_ctrl_reg(),
    ",
  0x50040854u64 => "
      DMA.dma2_idx_reg(),
    ",
  0x50040848u64 => "
      DMA.dma2_int_reg(),
    ",
  0x5004084cu64 => "
      DMA.dma2_len_reg(),
    ",
  0x50040860u64 => "
      DMA.dma3_a_start_reg(),
    ",
  0x50040864u64 => "
      DMA.dma3_b_start_reg(),
    ",
  0x50040870u64 => "
      DMA.dma3_ctrl_reg(),
    ",
  0x50040874u64 => "
      DMA.dma3_idx_reg(),
    ",
  0x50040868u64 => "
      DMA.dma3_int_reg(),
    ",
  0x5004086cu64 => "
      DMA.dma3_len_reg(),
    ",
  0x50040880u64 => "
      DMA.dma4_a_start_reg(),
    ",
  0x50040884u64 => "
      DMA.dma4_b_start_reg(),
    ",
  0x50040890u64 => "
      DMA.dma4_ctrl_reg(),
    ",
  0x50040894u64 => "
      DMA.dma4_idx_reg(),
    ",
  0x50040888u64 => "
      DMA.dma4_int_reg(),
    ",
  0x5004088cu64 => "
      DMA.dma4_len_reg(),
    ",
  0x500408a0u64 => "
      DMA.dma5_a_start_reg(),
    ",
  0x500408a4u64 => "
      DMA.dma5_b_start_reg(),
    ",
  0x500408b0u64 => "
      DMA.dma5_ctrl_reg(),
    ",
  0x500408b4u64 => "
      DMA.dma5_idx_reg(),
    ",
  0x500408a8u64 => "
      DMA.dma5_int_reg(),
    ",
  0x500408acu64 => "
      DMA.dma5_len_reg(),
    ",
  0x500408c0u64 => "
      DMA.dma6_a_start_reg(),
    ",
  0x500408c4u64 => "
      DMA.dma6_b_start_reg(),
    ",
  0x500408d0u64 => "
      DMA.dma6_ctrl_reg(),
    ",
  0x500408d4u64 => "
      DMA.dma6_idx_reg(),
    ",
  0x500408c8u64 => "
      DMA.dma6_int_reg(),
    ",
  0x500408ccu64 => "
      DMA.dma6_len_reg(),
    ",
  0x500408e0u64 => "
      DMA.dma7_a_start_reg(),
    ",
  0x500408e4u64 => "
      DMA.dma7_b_start_reg(),
    ",
  0x500408f0u64 => "
      DMA.dma7_ctrl_reg(),
    ",
  0x500408f4u64 => "
      DMA.dma7_idx_reg(),
    ",
  0x500408e8u64 => "
      DMA.dma7_int_reg(),
    ",
  0x500408ecu64 => "
      DMA.dma7_len_reg(),
    ",
  0x50040908u64 => "
      DMA.dma_clear_int_reg(),
    ",
  0x5004090cu64 => "
      DMA.dma_int_mask_reg(),
    ",
  0x50040904u64 => "
      DMA.dma_int_status_reg(),
    ",
  0x50040900u64 => "
      DMA.dma_req_mux_reg(),
    ",
  0x30020054u64 => "
      DW.ahb_dma_cclm1_reg(),
    ",
  0x30020058u64 => "
      DW.ahb_dma_cclm2_reg(),
    ",
  0x3002005cu64 => "
      DW.ahb_dma_cclm3_reg(),
    ",
  0x30020060u64 => "
      DW.ahb_dma_cclm4_reg(),
    ",
  0x30020048u64 => "
      DW.ahb_dma_dflt_master_reg(),
    ",
  0x30020000u64 => "
      DW.ahb_dma_pl1_reg(),
    ",
  0x30020004u64 => "
      DW.ahb_dma_pl2_reg(),
    ",
  0x30020008u64 => "
      DW.ahb_dma_pl3_reg(),
    ",
  0x3002000cu64 => "
      DW.ahb_dma_pl4_reg(),
    ",
  0x30020050u64 => "
      DW.ahb_dma_tcl_reg(),
    ",
  0x30020090u64 => "
      DW.ahb_dma_version_reg(),
    ",
  0x3002004cu64 => "
      DW.ahb_dma_wten_reg(),
    ",
  0x50030914u64 => "
      GPADC.gp_adc_clear_int_reg(),
    ",
  0x50030904u64 => "
      GPADC.gp_adc_ctrl2_reg(),
    ",
  0x50030908u64 => "
      GPADC.gp_adc_ctrl3_reg(),
    ",
  0x50030900u64 => "
      GPADC.gp_adc_ctrl_reg(),
    ",
  0x50030910u64 => "
      GPADC.gp_adc_offn_reg(),
    ",
  0x5003090cu64 => "
      GPADC.gp_adc_offp_reg(),
    ",
  0x50030918u64 => "
      GPADC.gp_adc_result_reg(),
    ",
  0x50020afcu64 => "
      GPIO.gpio_clk_sel_reg(),
    ",
  0x50020a18u64 => "
      GPIO.p0_00_mode_reg(),
    ",
  0x50020a1cu64 => "
      GPIO.p0_01_mode_reg(),
    ",
  0x50020a20u64 => "
      GPIO.p0_02_mode_reg(),
    ",
  0x50020a24u64 => "
      GPIO.p0_03_mode_reg(),
    ",
  0x50020a28u64 => "
      GPIO.p0_04_mode_reg(),
    ",
  0x50020a2cu64 => "
      GPIO.p0_05_mode_reg(),
    ",
  0x50020a30u64 => "
      GPIO.p0_06_mode_reg(),
    ",
  0x50020a34u64 => "
      GPIO.p0_07_mode_reg(),
    ",
  0x50020a38u64 => "
      GPIO.p0_08_mode_reg(),
    ",
  0x50020a3cu64 => "
      GPIO.p0_09_mode_reg(),
    ",
  0x50020a40u64 => "
      GPIO.p0_10_mode_reg(),
    ",
  0x50020a44u64 => "
      GPIO.p0_11_mode_reg(),
    ",
  0x50020a48u64 => "
      GPIO.p0_12_mode_reg(),
    ",
  0x50020a4cu64 => "
      GPIO.p0_13_mode_reg(),
    ",
  0x50020a50u64 => "
      GPIO.p0_14_mode_reg(),
    ",
  0x50020a54u64 => "
      GPIO.p0_15_mode_reg(),
    ",
  0x50020a58u64 => "
      GPIO.p0_16_mode_reg(),
    ",
  0x50020a5cu64 => "
      GPIO.p0_17_mode_reg(),
    ",
  0x50020a60u64 => "
      GPIO.p0_18_mode_reg(),
    ",
  0x50020a64u64 => "
      GPIO.p0_19_mode_reg(),
    ",
  0x50020a68u64 => "
      GPIO.p0_20_mode_reg(),
    ",
  0x50020a6cu64 => "
      GPIO.p0_21_mode_reg(),
    ",
  0x50020a70u64 => "
      GPIO.p0_22_mode_reg(),
    ",
  0x50020a74u64 => "
      GPIO.p0_23_mode_reg(),
    ",
  0x50020a78u64 => "
      GPIO.p0_24_mode_reg(),
    ",
  0x50020a7cu64 => "
      GPIO.p0_25_mode_reg(),
    ",
  0x50020a80u64 => "
      GPIO.p0_26_mode_reg(),
    ",
  0x50020a84u64 => "
      GPIO.p0_27_mode_reg(),
    ",
  0x50020a88u64 => "
      GPIO.p0_28_mode_reg(),
    ",
  0x50020a8cu64 => "
      GPIO.p0_29_mode_reg(),
    ",
  0x50020a90u64 => "
      GPIO.p0_30_mode_reg(),
    ",
  0x50020a94u64 => "
      GPIO.p0_31_mode_reg(),
    ",
  0x50020a00u64 => "
      GPIO.p0_data_reg(),
    ",
  0x50020af4u64 => "
      GPIO.p0_padpwr_ctrl_reg(),
    ",
  0x50020a10u64 => "
      GPIO.p0_reset_data_reg(),
    ",
  0x50020a08u64 => "
      GPIO.p0_set_data_reg(),
    ",
  0x50020a98u64 => "
      GPIO.p1_00_mode_reg(),
    ",
  0x50020a9cu64 => "
      GPIO.p1_01_mode_reg(),
    ",
  0x50020aa0u64 => "
      GPIO.p1_02_mode_reg(),
    ",
  0x50020aa4u64 => "
      GPIO.p1_03_mode_reg(),
    ",
  0x50020aa8u64 => "
      GPIO.p1_04_mode_reg(),
    ",
  0x50020aacu64 => "
      GPIO.p1_05_mode_reg(),
    ",
  0x50020ab0u64 => "
      GPIO.p1_06_mode_reg(),
    ",
  0x50020ab4u64 => "
      GPIO.p1_07_mode_reg(),
    ",
  0x50020ab8u64 => "
      GPIO.p1_08_mode_reg(),
    ",
  0x50020abcu64 => "
      GPIO.p1_09_mode_reg(),
    ",
  0x50020ac0u64 => "
      GPIO.p1_10_mode_reg(),
    ",
  0x50020ac4u64 => "
      GPIO.p1_11_mode_reg(),
    ",
  0x50020ac8u64 => "
      GPIO.p1_12_mode_reg(),
    ",
  0x50020accu64 => "
      GPIO.p1_13_mode_reg(),
    ",
  0x50020ad0u64 => "
      GPIO.p1_14_mode_reg(),
    ",
  0x50020ad4u64 => "
      GPIO.p1_15_mode_reg(),
    ",
  0x50020ad8u64 => "
      GPIO.p1_16_mode_reg(),
    ",
  0x50020adcu64 => "
      GPIO.p1_17_mode_reg(),
    ",
  0x50020ae0u64 => "
      GPIO.p1_18_mode_reg(),
    ",
  0x50020ae4u64 => "
      GPIO.p1_19_mode_reg(),
    ",
  0x50020ae8u64 => "
      GPIO.p1_20_mode_reg(),
    ",
  0x50020aecu64 => "
      GPIO.p1_21_mode_reg(),
    ",
  0x50020af0u64 => "
      GPIO.p1_22_mode_reg(),
    ",
  0x50020a04u64 => "
      GPIO.p1_data_reg(),
    ",
  0x50020af8u64 => "
      GPIO.p1_padpwr_ctrl_reg(),
    ",
  0x50020a14u64 => "
      GPIO.p1_reset_data_reg(),
    ",
  0x50020a0cu64 => "
      GPIO.p1_set_data_reg(),
    ",
  0x50020b00u64 => "
      GPIO.pad_weak_ctrl_reg(),
    ",
  0x50040308u64 => "
      GPREG.debug_reg(),
    ",
  0x50040310u64 => "
      GPREG.gp_control_reg(),
    ",
  0x5004030cu64 => "
      GPREG.gp_status_reg(),
    ",
  0x50040304u64 => "
      GPREG.reset_freeze_reg(),
    ",
  0x50040300u64 => "
      GPREG.set_freeze_reg(),
    ",
  0x50040318u64 => "
      GPREG.usbpad_reg(),
    ",
  0x50020698u64 => "
      I_2_C.i2c_ack_general_call_reg(),
    ",
  0x5002065cu64 => "
      I_2_C.i2c_clr_activity_reg(),
    ",
  0x50020668u64 => "
      I_2_C.i2c_clr_gen_call_reg(),
    ",
  0x50020640u64 => "
      I_2_C.i2c_clr_intr_reg(),
    ",
  0x50020650u64 => "
      I_2_C.i2c_clr_rd_req_reg(),
    ",
  0x50020658u64 => "
      I_2_C.i2c_clr_rx_done_reg(),
    ",
  0x50020648u64 => "
      I_2_C.i2c_clr_rx_over_reg(),
    ",
  0x50020644u64 => "
      I_2_C.i2c_clr_rx_under_reg(),
    ",
  0x50020664u64 => "
      I_2_C.i2c_clr_start_det_reg(),
    ",
  0x50020660u64 => "
      I_2_C.i2c_clr_stop_det_reg(),
    ",
  0x50020654u64 => "
      I_2_C.i2c_clr_tx_abrt_reg(),
    ",
  0x5002064cu64 => "
      I_2_C.i2c_clr_tx_over_reg(),
    ",
  0x500206f4u64 => "
      I_2_C.i2c_comp_param1_reg(),
    ",
  0x500206fcu64 => "
      I_2_C.i2c_comp_type_reg(),
    ",
  0x500206f8u64 => "
      I_2_C.i2c_comp_version_reg(),
    ",
  0x50020600u64 => "
      I_2_C.i2c_con_reg(),
    ",
  0x50020610u64 => "
      I_2_C.i2c_data_cmd_reg(),
    ",
  0x50020688u64 => "
      I_2_C.i2c_dma_cr_reg(),
    ",
  0x50020690u64 => "
      I_2_C.i2c_dma_rdlr_reg(),
    ",
  0x5002068cu64 => "
      I_2_C.i2c_dma_tdlr_reg(),
    ",
  0x5002066cu64 => "
      I_2_C.i2c_enable_reg(),
    ",
  0x5002069cu64 => "
      I_2_C.i2c_enable_status_reg(),
    ",
  0x5002061cu64 => "
      I_2_C.i2c_fs_scl_hcnt_reg(),
    ",
  0x50020620u64 => "
      I_2_C.i2c_fs_scl_lcnt_reg(),
    ",
  0x5002060cu64 => "
      I_2_C.i2c_hs_maddr_reg(),
    ",
  0x50020624u64 => "
      I_2_C.i2c_hs_scl_hcnt_reg(),
    ",
  0x50020628u64 => "
      I_2_C.i2c_hs_scl_lcnt_reg(),
    ",
  0x500206a0u64 => "
      I_2_C.i2c_ic_fs_spklen_reg(),
    ",
  0x500206a4u64 => "
      I_2_C.i2c_ic_hs_spklen_reg(),
    ",
  0x50020630u64 => "
      I_2_C.i2c_intr_mask_reg(),
    ",
  0x5002062cu64 => "
      I_2_C.i2c_intr_stat_reg(),
    ",
  0x50020634u64 => "
      I_2_C.i2c_raw_intr_stat_reg(),
    ",
  0x50020678u64 => "
      I_2_C.i2c_rxflr_reg(),
    ",
  0x50020638u64 => "
      I_2_C.i2c_rx_tl_reg(),
    ",
  0x50020608u64 => "
      I_2_C.i2c_sar_reg(),
    ",
  0x5002067cu64 => "
      I_2_C.i2c_sda_hold_reg(),
    ",
  0x50020694u64 => "
      I_2_C.i2c_sda_setup_reg(),
    ",
  0x50020614u64 => "
      I_2_C.i2c_ss_scl_hcnt_reg(),
    ",
  0x50020618u64 => "
      I_2_C.i2c_ss_scl_lcnt_reg(),
    ",
  0x50020670u64 => "
      I_2_C.i2c_status_reg(),
    ",
  0x50020604u64 => "
      I_2_C.i2c_tar_reg(),
    ",
  0x50020674u64 => "
      I_2_C.i2c_txflr_reg(),
    ",
  0x50020680u64 => "
      I_2_C.i2c_tx_abrt_source_reg(),
    ",
  0x5002063cu64 => "
      I_2_C.i2c_tx_tl_reg(),
    ",
  0x50020798u64 => "
      I_2_C_2.i2c2_ack_general_call_reg(),
    ",
  0x5002075cu64 => "
      I_2_C_2.i2c2_clr_activity_reg(),
    ",
  0x50020768u64 => "
      I_2_C_2.i2c2_clr_gen_call_reg(),
    ",
  0x50020740u64 => "
      I_2_C_2.i2c2_clr_intr_reg(),
    ",
  0x50020750u64 => "
      I_2_C_2.i2c2_clr_rd_req_reg(),
    ",
  0x50020758u64 => "
      I_2_C_2.i2c2_clr_rx_done_reg(),
    ",
  0x50020748u64 => "
      I_2_C_2.i2c2_clr_rx_over_reg(),
    ",
  0x50020744u64 => "
      I_2_C_2.i2c2_clr_rx_under_reg(),
    ",
  0x50020764u64 => "
      I_2_C_2.i2c2_clr_start_det_reg(),
    ",
  0x50020760u64 => "
      I_2_C_2.i2c2_clr_stop_det_reg(),
    ",
  0x50020754u64 => "
      I_2_C_2.i2c2_clr_tx_abrt_reg(),
    ",
  0x5002074cu64 => "
      I_2_C_2.i2c2_clr_tx_over_reg(),
    ",
  0x500207f4u64 => "
      I_2_C_2.i2c2_comp_param1_reg(),
    ",
  0x500207fcu64 => "
      I_2_C_2.i2c2_comp_type_reg(),
    ",
  0x500207f8u64 => "
      I_2_C_2.i2c2_comp_version_reg(),
    ",
  0x50020700u64 => "
      I_2_C_2.i2c2_con_reg(),
    ",
  0x50020710u64 => "
      I_2_C_2.i2c2_data_cmd_reg(),
    ",
  0x50020788u64 => "
      I_2_C_2.i2c2_dma_cr_reg(),
    ",
  0x50020790u64 => "
      I_2_C_2.i2c2_dma_rdlr_reg(),
    ",
  0x5002078cu64 => "
      I_2_C_2.i2c2_dma_tdlr_reg(),
    ",
  0x5002076cu64 => "
      I_2_C_2.i2c2_enable_reg(),
    ",
  0x5002079cu64 => "
      I_2_C_2.i2c2_enable_status_reg(),
    ",
  0x5002071cu64 => "
      I_2_C_2.i2c2_fs_scl_hcnt_reg(),
    ",
  0x50020720u64 => "
      I_2_C_2.i2c2_fs_scl_lcnt_reg(),
    ",
  0x5002070cu64 => "
      I_2_C_2.i2c2_hs_maddr_reg(),
    ",
  0x50020724u64 => "
      I_2_C_2.i2c2_hs_scl_hcnt_reg(),
    ",
  0x50020728u64 => "
      I_2_C_2.i2c2_hs_scl_lcnt_reg(),
    ",
  0x500207a0u64 => "
      I_2_C_2.i2c2_ic_fs_spklen_reg(),
    ",
  0x500207a4u64 => "
      I_2_C_2.i2c2_ic_hs_spklen_reg(),
    ",
  0x50020730u64 => "
      I_2_C_2.i2c2_intr_mask_reg(),
    ",
  0x5002072cu64 => "
      I_2_C_2.i2c2_intr_stat_reg(),
    ",
  0x50020734u64 => "
      I_2_C_2.i2c2_raw_intr_stat_reg(),
    ",
  0x50020778u64 => "
      I_2_C_2.i2c2_rxflr_reg(),
    ",
  0x50020738u64 => "
      I_2_C_2.i2c2_rx_tl_reg(),
    ",
  0x50020708u64 => "
      I_2_C_2.i2c2_sar_reg(),
    ",
  0x5002077cu64 => "
      I_2_C_2.i2c2_sda_hold_reg(),
    ",
  0x50020794u64 => "
      I_2_C_2.i2c2_sda_setup_reg(),
    ",
  0x50020714u64 => "
      I_2_C_2.i2c2_ss_scl_hcnt_reg(),
    ",
  0x50020718u64 => "
      I_2_C_2.i2c2_ss_scl_lcnt_reg(),
    ",
  0x50020770u64 => "
      I_2_C_2.i2c2_status_reg(),
    ",
  0x50020704u64 => "
      I_2_C_2.i2c2_tar_reg(),
    ",
  0x50020774u64 => "
      I_2_C_2.i2c2_txflr_reg(),
    ",
  0x50020780u64 => "
      I_2_C_2.i2c2_tx_abrt_source_reg(),
    ",
  0x5002073cu64 => "
      I_2_C_2.i2c2_tx_tl_reg(),
    ",
  0x3003001cu64 => "
      LCDC.lcdc_backporchxy_reg(),
    ",
  0x30030008u64 => "
      LCDC.lcdc_bgcolor_reg(),
    ",
  0x30030018u64 => "
      LCDC.lcdc_blankingxy_reg(),
    ",
  0x30030004u64 => "
      LCDC.lcdc_clkctrl_reg(),
    ",
  0x30030184u64 => "
      LCDC.lcdc_crc_reg(),
    ",
  0x30030028u64 => "
      LCDC.lcdc_dbib_cfg_reg(),
    ",
  0x300300e8u64 => "
      LCDC.lcdc_dbib_cmd_reg(),
    ",
  0x30030014u64 => "
      LCDC.lcdc_frontporchxy_reg(),
    ",
  0x3003002cu64 => "
      LCDC.lcdc_gpio_reg(),
    ",
  0x300300f4u64 => "
      LCDC.lcdc_idreg_reg(),
    ",
  0x300300f8u64 => "
      LCDC.lcdc_interrupt_reg(),
    ",
  0x300300bcu64 => "
      LCDC.lcdc_jdi_enb_end_hline_reg(),
    ",
  0x300300c0u64 => "
      LCDC.lcdc_jdi_enb_start_clk_reg(),
    ",
  0x300300b8u64 => "
      LCDC.lcdc_jdi_enb_start_hline_reg(),
    ",
  0x300300c4u64 => "
      LCDC.lcdc_jdi_enb_width_clk_reg(),
    ",
  0x30030094u64 => "
      LCDC.lcdc_jdi_fbx_blanking_reg(),
    ",
  0x30030098u64 => "
      LCDC.lcdc_jdi_fby_blanking_reg(),
    ",
  0x3003009cu64 => "
      LCDC.lcdc_jdi_hck_width_reg(),
    ",
  0x300300b0u64 => "
      LCDC.lcdc_jdi_hst_delay_reg(),
    ",
  0x300300b4u64 => "
      LCDC.lcdc_jdi_hst_width_reg(),
    ",
  0x30030090u64 => "
      LCDC.lcdc_jdi_resxy_reg(),
    ",
  0x300300acu64 => "
      LCDC.lcdc_jdi_vck_delay_reg(),
    ",
  0x300300a4u64 => "
      LCDC.lcdc_jdi_vst_delay_reg(),
    ",
  0x300300a8u64 => "
      LCDC.lcdc_jdi_vst_width_reg(),
    ",
  0x300300a0u64 => "
      LCDC.lcdc_jdi_xrst_width_reg(),
    ",
  0x3003003cu64 => "
      LCDC.lcdc_layer0_baseaddr_reg(),
    ",
  0x30030030u64 => "
      LCDC.lcdc_layer0_mode_reg(),
    ",
  0x30030188u64 => "
      LCDC.lcdc_layer0_offsetx_reg(),
    ",
  0x30030044u64 => "
      LCDC.lcdc_layer0_resxy_reg(),
    ",
  0x30030038u64 => "
      LCDC.lcdc_layer0_sizexy_reg(),
    ",
  0x30030034u64 => "
      LCDC.lcdc_layer0_startxy_reg(),
    ",
  0x30030040u64 => "
      LCDC.lcdc_layer0_stride_reg(),
    ",
  0x30030000u64 => "
      LCDC.lcdc_mode_reg(),
    ",
  0x3003000cu64 => "
      LCDC.lcdc_resxy_reg(),
    ",
  0x300300fcu64 => "
      LCDC.lcdc_status_reg(),
    ",
  0x50030a44u64 => "
      LRA.lra_adc_ctrl1_reg(),
    ",
  0x50030a50u64 => "
      LRA.lra_adc_result_reg(),
    ",
  0x50030a3cu64 => "
      LRA.lra_brd_hs_reg(),
    ",
  0x50030a38u64 => "
      LRA.lra_brd_ls_reg(),
    ",
  0x50030a40u64 => "
      LRA.lra_brd_stat_reg(),
    ",
  0x50030a00u64 => "
      LRA.lra_ctrl1_reg(),
    ",
  0x50030a04u64 => "
      LRA.lra_ctrl2_reg(),
    ",
  0x50030a58u64 => "
      LRA.lra_dft_reg(),
    ",
  0x50030a2cu64 => "
      LRA.lra_flt_coef1_reg(),
    ",
  0x50030a30u64 => "
      LRA.lra_flt_coef2_reg(),
    ",
  0x50030a34u64 => "
      LRA.lra_flt_coef3_reg(),
    ",
  0x50030a0cu64 => "
      LRA.lra_flt_smp1_reg(),
    ",
  0x50030a10u64 => "
      LRA.lra_flt_smp2_reg(),
    ",
  0x50030a14u64 => "
      LRA.lra_flt_smp3_reg(),
    ",
  0x50030a18u64 => "
      LRA.lra_flt_smp4_reg(),
    ",
  0x50030a1cu64 => "
      LRA.lra_flt_smp5_reg(),
    ",
  0x50030a20u64 => "
      LRA.lra_flt_smp6_reg(),
    ",
  0x50030a24u64 => "
      LRA.lra_flt_smp7_reg(),
    ",
  0x50030a28u64 => "
      LRA.lra_flt_smp8_reg(),
    ",
  0x50030a54u64 => "
      LRA.lra_ldo_reg(),
    ",
  0x50050078u64 => "
      MEMCTRL.busy_reset_reg(),
    ",
  0x50050074u64 => "
      MEMCTRL.busy_set_reg(),
    ",
  0x5005007cu64 => "
      MEMCTRL.busy_stat_reg(),
    ",
  0x50050020u64 => "
      MEMCTRL.cmi_code_base_reg(),
    ",
  0x50050024u64 => "
      MEMCTRL.cmi_data_base_reg(),
    ",
  0x5005002cu64 => "
      MEMCTRL.cmi_end_reg(),
    ",
  0x50050028u64 => "
      MEMCTRL.cmi_shared_base_reg(),
    ",
  0x50050004u64 => "
      MEMCTRL.mem_prio_reg(),
    ",
  0x50050008u64 => "
      MEMCTRL.mem_stall_reg(),
    ",
  0x50050010u64 => "
      MEMCTRL.mem_status2_reg(),
    ",
  0x5005000cu64 => "
      MEMCTRL.mem_status_reg(),
    ",
  0x50050030u64 => "
      MEMCTRL.snc_base_reg(),
    ",
  0x30070000u64 => "
      OTPC.otpc_mode_reg(),
    ",
  0x30070008u64 => "
      OTPC.otpc_paddr_reg(),
    ",
  0x3007000cu64 => "
      OTPC.otpc_pword_reg(),
    ",
  0x30070004u64 => "
      OTPC.otpc_stat_reg(),
    ",
  0x30070010u64 => "
      OTPC.otpc_tim1_reg(),
    ",
  0x30070014u64 => "
      OTPC.otpc_tim2_reg(),
    ",
  0x50000280u64 => "
      PDC.pdc_acknowledge_reg(),
    ",
  0x50000200u64 => "
      PDC.pdc_ctrl0_reg(),
    ",
  0x50000228u64 => "
      PDC.pdc_ctrl10_reg(),
    ",
  0x5000022cu64 => "
      PDC.pdc_ctrl11_reg(),
    ",
  0x50000230u64 => "
      PDC.pdc_ctrl12_reg(),
    ",
  0x50000234u64 => "
      PDC.pdc_ctrl13_reg(),
    ",
  0x50000238u64 => "
      PDC.pdc_ctrl14_reg(),
    ",
  0x5000023cu64 => "
      PDC.pdc_ctrl15_reg(),
    ",
  0x50000204u64 => "
      PDC.pdc_ctrl1_reg(),
    ",
  0x50000208u64 => "
      PDC.pdc_ctrl2_reg(),
    ",
  0x5000020cu64 => "
      PDC.pdc_ctrl3_reg(),
    ",
  0x50000210u64 => "
      PDC.pdc_ctrl4_reg(),
    ",
  0x50000214u64 => "
      PDC.pdc_ctrl5_reg(),
    ",
  0x50000218u64 => "
      PDC.pdc_ctrl6_reg(),
    ",
  0x5000021cu64 => "
      PDC.pdc_ctrl7_reg(),
    ",
  0x50000220u64 => "
      PDC.pdc_ctrl8_reg(),
    ",
  0x50000224u64 => "
      PDC.pdc_ctrl9_reg(),
    ",
  0x5000028cu64 => "
      PDC.pdc_pending_cm33_reg(),
    ",
  0x50000290u64 => "
      PDC.pdc_pending_cmac_reg(),
    ",
  0x50000284u64 => "
      PDC.pdc_pending_reg(),
    ",
  0x50000288u64 => "
      PDC.pdc_pending_snc_reg(),
    ",
  0x50000294u64 => "
      PDC.pdc_set_pending_reg(),
    ",
  0x5003050cu64 => "
      PWMLED.pwmled_ctrl_reg(),
    ",
  0x50030500u64 => "
      PWMLED.pwmled_duty_cycle_led1_reg(),
    ",
  0x50030504u64 => "
      PWMLED.pwmled_duty_cycle_led2_reg(),
    ",
  0x50030508u64 => "
      PWMLED.pwmled_frequency_reg(),
    ",
  0x38000030u64 => "
      QSPIC.qspic_burstbrk_reg(),
    ",
  0x3800000cu64 => "
      QSPIC.qspic_burstcmda_reg(),
    ",
  0x38000010u64 => "
      QSPIC.qspic_burstcmdb_reg(),
    ",
  0x38000038u64 => "
      QSPIC.qspic_chckerase_reg(),
    ",
  0x38000000u64 => "
      QSPIC.qspic_ctrlbus_reg(),
    ",
  0x38000004u64 => "
      QSPIC.qspic_ctrlmode_reg(),
    ",
  0x38000080u64 => "
      QSPIC.qspic_ctr_ctrl_reg(),
    ",
  0x38000088u64 => "
      QSPIC.qspic_ctr_eaddr_reg(),
    ",
  0x38000094u64 => "
      QSPIC.qspic_ctr_key_0_3_reg(),
    ",
  0x380000a0u64 => "
      QSPIC.qspic_ctr_key_12_15_reg(),
    ",
  0x380000a4u64 => "
      QSPIC.qspic_ctr_key_16_19_reg(),
    ",
  0x380000a8u64 => "
      QSPIC.qspic_ctr_key_20_23_reg(),
    ",
  0x380000acu64 => "
      QSPIC.qspic_ctr_key_24_27_reg(),
    ",
  0x380000b0u64 => "
      QSPIC.qspic_ctr_key_28_31_reg(),
    ",
  0x38000098u64 => "
      QSPIC.qspic_ctr_key_4_7_reg(),
    ",
  0x3800009cu64 => "
      QSPIC.qspic_ctr_key_8_11_reg(),
    ",
  0x3800008cu64 => "
      QSPIC.qspic_ctr_nonce_0_3_reg(),
    ",
  0x38000090u64 => "
      QSPIC.qspic_ctr_nonce_4_7_reg(),
    ",
  0x38000084u64 => "
      QSPIC.qspic_ctr_saddr_reg(),
    ",
  0x38000020u64 => "
      QSPIC.qspic_dummydata_reg(),
    ",
  0x38000028u64 => "
      QSPIC.qspic_erasecmda_reg(),
    ",
  0x3800002cu64 => "
      QSPIC.qspic_erasecmdb_reg(),
    ",
  0x38000024u64 => "
      QSPIC.qspic_erasectrl_reg(),
    ",
  0x3800003cu64 => "
      QSPIC.qspic_gp_reg(),
    ",
  0x3800001cu64 => "
      QSPIC.qspic_readdata_reg(),
    ",
  0x38000008u64 => "
      QSPIC.qspic_recvdata_reg(),
    ",
  0x38000034u64 => "
      QSPIC.qspic_statuscmd_reg(),
    ",
  0x38000014u64 => "
      QSPIC.qspic_status_reg(),
    ",
  0x38000040u64 => "
      QSPIC.qspic_ucode_start(),
    ",
  0x38000018u64 => "
      QSPIC.qspic_writedata_reg(),
    ",
  0x34000040u64 => "
      QSPIC_2.qspic2_awritecmd_reg(),
    ",
  0x34000030u64 => "
      QSPIC_2.qspic2_burstbrk_reg(),
    ",
  0x3400000cu64 => "
      QSPIC_2.qspic2_burstcmda_reg(),
    ",
  0x34000010u64 => "
      QSPIC_2.qspic2_burstcmdb_reg(),
    ",
  0x34000038u64 => "
      QSPIC_2.qspic2_chckerase_reg(),
    ",
  0x34000000u64 => "
      QSPIC_2.qspic2_ctrlbus_reg(),
    ",
  0x34000004u64 => "
      QSPIC_2.qspic2_ctrlmode_reg(),
    ",
  0x34000020u64 => "
      QSPIC_2.qspic2_dummydata_reg(),
    ",
  0x34000028u64 => "
      QSPIC_2.qspic2_erasecmda_reg(),
    ",
  0x3400002cu64 => "
      QSPIC_2.qspic2_erasecmdb_reg(),
    ",
  0x34000024u64 => "
      QSPIC_2.qspic2_erasectrl_reg(),
    ",
  0x3400003cu64 => "
      QSPIC_2.qspic2_gp_reg(),
    ",
  0x34000044u64 => "
      QSPIC_2.qspic2_memblen_reg(),
    ",
  0x3400001cu64 => "
      QSPIC_2.qspic2_readdata_reg(),
    ",
  0x34000008u64 => "
      QSPIC_2.qspic2_recvdata_reg(),
    ",
  0x34000034u64 => "
      QSPIC_2.qspic2_statuscmd_reg(),
    ",
  0x34000014u64 => "
      QSPIC_2.qspic2_status_reg(),
    ",
  0x34000018u64 => "
      QSPIC_2.qspic2_writedata_reg(),
    ",
  0x50040604u64 => "
      RFMON.rfmon_addr_reg(),
    ",
  0x50040610u64 => "
      RFMON.rfmon_crv_addr_reg(),
    ",
  0x50040614u64 => "
      RFMON.rfmon_crv_len_reg(),
    ",
  0x50040600u64 => "
      RFMON.rfmon_ctrl_reg(),
    ",
  0x50040608u64 => "
      RFMON.rfmon_len_reg(),
    ",
  0x5004060cu64 => "
      RFMON.rfmon_stat_reg(),
    ",
  0x50000418u64 => "
      RTC.rtc_alarm_enable_reg(),
    ",
  0x50000414u64 => "
      RTC.rtc_calendar_alarm_reg(),
    ",
  0x5000040cu64 => "
      RTC.rtc_calendar_reg(),
    ",
  0x50000400u64 => "
      RTC.rtc_control_reg(),
    ",
  0x50000480u64 => "
      RTC.rtc_event_ctrl_reg(),
    ",
  0x5000041cu64 => "
      RTC.rtc_event_flags_reg(),
    ",
  0x50000404u64 => "
      RTC.rtc_hour_mode_reg(),
    ",
  0x50000424u64 => "
      RTC.rtc_interrupt_disable_reg(),
    ",
  0x50000420u64 => "
      RTC.rtc_interrupt_enable_reg(),
    ",
  0x50000428u64 => "
      RTC.rtc_interrupt_mask_reg(),
    ",
  0x50000430u64 => "
      RTC.rtc_keep_rtc_reg(),
    ",
  0x50000490u64 => "
      RTC.rtc_motor_event_cnt_reg(),
    ",
  0x50000484u64 => "
      RTC.rtc_motor_event_period_reg(),
    ",
  0x5000048cu64 => "
      RTC.rtc_pdc_event_clear_reg(),
    ",
  0x50000494u64 => "
      RTC.rtc_pdc_event_cnt_reg(),
    ",
  0x50000488u64 => "
      RTC.rtc_pdc_event_period_reg(),
    ",
  0x5000042cu64 => "
      RTC.rtc_status_reg(),
    ",
  0x50000410u64 => "
      RTC.rtc_time_alarm_reg(),
    ",
  0x50000408u64 => "
      RTC.rtc_time_reg(),
    ",
  0x50020814u64 => "
      SDADC.sdadc_clear_int_reg(),
    ",
  0x50020800u64 => "
      SDADC.sdadc_ctrl_reg(),
    ",
  0x5002080cu64 => "
      SDADC.sdadc_gain_corr_reg(),
    ",
  0x50020810u64 => "
      SDADC.sdadc_offs_corr_reg(),
    ",
  0x50020818u64 => "
      SDADC.sdadc_result_reg(),
    ",
  0x50030e80u64 => "
      SMOTOR.cmd_table_base(),
    ",
  0x50030e04u64 => "
      SMOTOR.pg0_ctrl_reg(),
    ",
  0x50030e08u64 => "
      SMOTOR.pg1_ctrl_reg(),
    ",
  0x50030e0cu64 => "
      SMOTOR.pg2_ctrl_reg(),
    ",
  0x50030e10u64 => "
      SMOTOR.pg3_ctrl_reg(),
    ",
  0x50030e14u64 => "
      SMOTOR.pg4_ctrl_reg(),
    ",
  0x50030e20u64 => "
      SMOTOR.smotor_cmd_fifo_reg(),
    ",
  0x50030e24u64 => "
      SMOTOR.smotor_cmd_read_ptr_reg(),
    ",
  0x50030e28u64 => "
      SMOTOR.smotor_cmd_write_ptr_reg(),
    ",
  0x50030e00u64 => "
      SMOTOR.smotor_ctrl_reg(),
    ",
  0x50030e30u64 => "
      SMOTOR.smotor_irq_clear_reg(),
    ",
  0x50030e18u64 => "
      SMOTOR.smotor_trigger_reg(),
    ",
  0x50030e40u64 => "
      SMOTOR.wavetable_base(),
    ",
  0x50020c00u64 => "
      SNC.snc_ctrl_reg(),
    ",
  0x50020c08u64 => "
      SNC.snc_lp_timer_reg(),
    ",
  0x50020c0cu64 => "
      SNC.snc_pc_reg(),
    ",
  0x50020c10u64 => "
      SNC.snc_r1_reg(),
    ",
  0x50020c14u64 => "
      SNC.snc_r2_reg(),
    ",
  0x50020c04u64 => "
      SNC.snc_status_reg(),
    ",
  0x50020c18u64 => "
      SNC.snc_tmp1_reg(),
    ",
  0x50020c1cu64 => "
      SNC.snc_tmp2_reg(),
    ",
  0x50020308u64 => "
      SPI.spi_clear_int_reg(),
    ",
  0x50020300u64 => "
      SPI.spi_ctrl_reg(),
    ",
  0x50020304u64 => "
      SPI.spi_rx_tx_reg(),
    ",
  0x50020408u64 => "
      SPI_2.spi2_clear_int_reg(),
    ",
  0x50020400u64 => "
      SPI_2.spi2_ctrl_reg(),
    ",
  0x50020404u64 => "
      SPI_2.spi2_rx_tx_reg(),
    ",
  0x50000704u64 => "
      SYS_WDOG.watchdog_ctrl_reg(),
    ",
  0x50000700u64 => "
      SYS_WDOG.watchdog_reg(),
    ",
  0x50010220u64 => "
      TIMER.timer_capture_gpio1_reg(),
    ",
  0x50010224u64 => "
      TIMER.timer_capture_gpio2_reg(),
    ",
  0x5001023cu64 => "
      TIMER.timer_capture_gpio3_reg(),
    ",
  0x50010240u64 => "
      TIMER.timer_capture_gpio4_reg(),
    ",
  0x50010244u64 => "
      TIMER.timer_clear_gpio_event_reg(),
    ",
  0x50010248u64 => "
      TIMER.timer_clear_irq_reg(),
    ",
  0x50010200u64 => "
      TIMER.timer_ctrl_reg(),
    ",
  0x5001020cu64 => "
      TIMER.timer_gpio1_conf_reg(),
    ",
  0x50010210u64 => "
      TIMER.timer_gpio2_conf_reg(),
    ",
  0x50010234u64 => "
      TIMER.timer_gpio3_conf_reg(),
    ",
  0x50010238u64 => "
      TIMER.timer_gpio4_conf_reg(),
    ",
  0x5001021cu64 => "
      TIMER.timer_prescaler_reg(),
    ",
  0x50010228u64 => "
      TIMER.timer_prescaler_val_reg(),
    ",
  0x50010230u64 => "
      TIMER.timer_pwm_dc_reg(),
    ",
  0x5001022cu64 => "
      TIMER.timer_pwm_freq_reg(),
    ",
  0x50010214u64 => "
      TIMER.timer_reload_reg(),
    ",
  0x50010218u64 => "
      TIMER.timer_shotwidth_reg(),
    ",
  0x50010208u64 => "
      TIMER.timer_status_reg(),
    ",
  0x50010204u64 => "
      TIMER.timer_timer_val_reg(),
    ",
  0x50010320u64 => "
      TIMER_2.timer2_capture_gpio1_reg(),
    ",
  0x50010324u64 => "
      TIMER_2.timer2_capture_gpio2_reg(),
    ",
  0x50010334u64 => "
      TIMER_2.timer2_clear_irq_reg(),
    ",
  0x50010300u64 => "
      TIMER_2.timer2_ctrl_reg(),
    ",
  0x5001030cu64 => "
      TIMER_2.timer2_gpio1_conf_reg(),
    ",
  0x50010310u64 => "
      TIMER_2.timer2_gpio2_conf_reg(),
    ",
  0x5001031cu64 => "
      TIMER_2.timer2_prescaler_reg(),
    ",
  0x50010328u64 => "
      TIMER_2.timer2_prescaler_val_reg(),
    ",
  0x50010330u64 => "
      TIMER_2.timer2_pwm_dc_reg(),
    ",
  0x5001032cu64 => "
      TIMER_2.timer2_pwm_freq_reg(),
    ",
  0x50010314u64 => "
      TIMER_2.timer2_reload_reg(),
    ",
  0x50010318u64 => "
      TIMER_2.timer2_shotwidth_reg(),
    ",
  0x50010308u64 => "
      TIMER_2.timer2_status_reg(),
    ",
  0x50010304u64 => "
      TIMER_2.timer2_timer_val_reg(),
    ",
  0x50040a20u64 => "
      TIMER_3.timer3_capture_gpio1_reg(),
    ",
  0x50040a24u64 => "
      TIMER_3.timer3_capture_gpio2_reg(),
    ",
  0x50040a34u64 => "
      TIMER_3.timer3_clear_irq_reg(),
    ",
  0x50040a00u64 => "
      TIMER_3.timer3_ctrl_reg(),
    ",
  0x50040a0cu64 => "
      TIMER_3.timer3_gpio1_conf_reg(),
    ",
  0x50040a10u64 => "
      TIMER_3.timer3_gpio2_conf_reg(),
    ",
  0x50040a1cu64 => "
      TIMER_3.timer3_prescaler_reg(),
    ",
  0x50040a28u64 => "
      TIMER_3.timer3_prescaler_val_reg(),
    ",
  0x50040a30u64 => "
      TIMER_3.timer3_pwm_dc_reg(),
    ",
  0x50040a2cu64 => "
      TIMER_3.timer3_pwm_freq_reg(),
    ",
  0x50040a14u64 => "
      TIMER_3.timer3_reload_reg(),
    ",
  0x50040a08u64 => "
      TIMER_3.timer3_status_reg(),
    ",
  0x50040a04u64 => "
      TIMER_3.timer3_timer_val_reg(),
    ",
  0x50040b20u64 => "
      TIMER_4.timer4_capture_gpio1_reg(),
    ",
  0x50040b24u64 => "
      TIMER_4.timer4_capture_gpio2_reg(),
    ",
  0x50040b34u64 => "
      TIMER_4.timer4_clear_irq_reg(),
    ",
  0x50040b00u64 => "
      TIMER_4.timer4_ctrl_reg(),
    ",
  0x50040b0cu64 => "
      TIMER_4.timer4_gpio1_conf_reg(),
    ",
  0x50040b10u64 => "
      TIMER_4.timer4_gpio2_conf_reg(),
    ",
  0x50040b1cu64 => "
      TIMER_4.timer4_prescaler_reg(),
    ",
  0x50040b28u64 => "
      TIMER_4.timer4_prescaler_val_reg(),
    ",
  0x50040b30u64 => "
      TIMER_4.timer4_pwm_dc_reg(),
    ",
  0x50040b2cu64 => "
      TIMER_4.timer4_pwm_freq_reg(),
    ",
  0x50040b14u64 => "
      TIMER_4.timer4_reload_reg(),
    ",
  0x50040b08u64 => "
      TIMER_4.timer4_status_reg(),
    ",
  0x50040b04u64 => "
      TIMER_4.timer4_timer_val_reg(),
    ",
  0x500200fcu64 => "
      UART.uart_ctr_reg(),
    ",
  0x500200c0u64 => "
      UART.uart_dlf_reg(),
    ",
  0x500200a8u64 => "
      UART.uart_dmasa_reg(),
    ",
  0x500200a4u64 => "
      UART.uart_htx_reg(),
    ",
  0x50020004u64 => "
      UART.uart_ier_dlh_reg(),
    ",
  0x50020008u64 => "
      UART.uart_iir_fcr_reg(),
    ",
  0x5002000cu64 => "
      UART.uart_lcr_reg(),
    ",
  0x50020014u64 => "
      UART.uart_lsr_reg(),
    ",
  0x50020010u64 => "
      UART.uart_mcr_reg(),
    ",
  0x50020000u64 => "
      UART.uart_rbr_thr_dll_reg(),
    ",
  0x50020084u64 => "
      UART.uart_rfl_reg(),
    ",
  0x50020090u64 => "
      UART.uart_sbcr_reg(),
    ",
  0x5002001cu64 => "
      UART.uart_scr_reg(),
    ",
  0x50020094u64 => "
      UART.uart_sdmam_reg(),
    ",
  0x50020098u64 => "
      UART.uart_sfe_reg(),
    ",
  0x50020030u64 => "
      UART.uart_srbr_sthr0_reg(),
    ",
  0x50020058u64 => "
      UART.uart_srbr_sthr10_reg(),
    ",
  0x5002005cu64 => "
      UART.uart_srbr_sthr11_reg(),
    ",
  0x50020060u64 => "
      UART.uart_srbr_sthr12_reg(),
    ",
  0x50020064u64 => "
      UART.uart_srbr_sthr13_reg(),
    ",
  0x50020068u64 => "
      UART.uart_srbr_sthr14_reg(),
    ",
  0x5002006cu64 => "
      UART.uart_srbr_sthr15_reg(),
    ",
  0x50020034u64 => "
      UART.uart_srbr_sthr1_reg(),
    ",
  0x50020038u64 => "
      UART.uart_srbr_sthr2_reg(),
    ",
  0x5002003cu64 => "
      UART.uart_srbr_sthr3_reg(),
    ",
  0x50020040u64 => "
      UART.uart_srbr_sthr4_reg(),
    ",
  0x50020044u64 => "
      UART.uart_srbr_sthr5_reg(),
    ",
  0x50020048u64 => "
      UART.uart_srbr_sthr6_reg(),
    ",
  0x5002004cu64 => "
      UART.uart_srbr_sthr7_reg(),
    ",
  0x50020050u64 => "
      UART.uart_srbr_sthr8_reg(),
    ",
  0x50020054u64 => "
      UART.uart_srbr_sthr9_reg(),
    ",
  0x50020088u64 => "
      UART.uart_srr_reg(),
    ",
  0x5002009cu64 => "
      UART.uart_srt_reg(),
    ",
  0x500200a0u64 => "
      UART.uart_stet_reg(),
    ",
  0x50020080u64 => "
      UART.uart_tfl_reg(),
    ",
  0x500200f8u64 => "
      UART.uart_ucv_reg(),
    ",
  0x5002007cu64 => "
      UART.uart_usr_reg(),
    ",
  0x500201fcu64 => "
      UART_2.uart2_ctr_reg(),
    ",
  0x500201c0u64 => "
      UART_2.uart2_dlf_reg(),
    ",
  0x500201a8u64 => "
      UART_2.uart2_dmasa_reg(),
    ",
  0x500201a4u64 => "
      UART_2.uart2_htx_reg(),
    ",
  0x50020104u64 => "
      UART_2.uart2_ier_dlh_reg(),
    ",
  0x50020108u64 => "
      UART_2.uart2_iir_fcr_reg(),
    ",
  0x500201ccu64 => "
      UART_2.uart2_lcr_ext(),
    ",
  0x5002010cu64 => "
      UART_2.uart2_lcr_reg(),
    ",
  0x50020114u64 => "
      UART_2.uart2_lsr_reg(),
    ",
  0x50020110u64 => "
      UART_2.uart2_mcr_reg(),
    ",
  0x50020118u64 => "
      UART_2.uart2_msr_reg(),
    ",
  0x500201c4u64 => "
      UART_2.uart2_rar_reg(),
    ",
  0x50020100u64 => "
      UART_2.uart2_rbr_thr_dll_reg(),
    ",
  0x50020184u64 => "
      UART_2.uart2_rfl_reg(),
    ",
  0x50020190u64 => "
      UART_2.uart2_sbcr_reg(),
    ",
  0x5002011cu64 => "
      UART_2.uart2_scr_reg(),
    ",
  0x50020194u64 => "
      UART_2.uart2_sdmam_reg(),
    ",
  0x50020198u64 => "
      UART_2.uart2_sfe_reg(),
    ",
  0x50020130u64 => "
      UART_2.uart2_srbr_sthr0_reg(),
    ",
  0x50020158u64 => "
      UART_2.uart2_srbr_sthr10_reg(),
    ",
  0x5002015cu64 => "
      UART_2.uart2_srbr_sthr11_reg(),
    ",
  0x50020160u64 => "
      UART_2.uart2_srbr_sthr12_reg(),
    ",
  0x50020164u64 => "
      UART_2.uart2_srbr_sthr13_reg(),
    ",
  0x50020168u64 => "
      UART_2.uart2_srbr_sthr14_reg(),
    ",
  0x5002016cu64 => "
      UART_2.uart2_srbr_sthr15_reg(),
    ",
  0x50020134u64 => "
      UART_2.uart2_srbr_sthr1_reg(),
    ",
  0x50020138u64 => "
      UART_2.uart2_srbr_sthr2_reg(),
    ",
  0x5002013cu64 => "
      UART_2.uart2_srbr_sthr3_reg(),
    ",
  0x50020140u64 => "
      UART_2.uart2_srbr_sthr4_reg(),
    ",
  0x50020144u64 => "
      UART_2.uart2_srbr_sthr5_reg(),
    ",
  0x50020148u64 => "
      UART_2.uart2_srbr_sthr6_reg(),
    ",
  0x5002014cu64 => "
      UART_2.uart2_srbr_sthr7_reg(),
    ",
  0x50020150u64 => "
      UART_2.uart2_srbr_sthr8_reg(),
    ",
  0x50020154u64 => "
      UART_2.uart2_srbr_sthr9_reg(),
    ",
  0x50020188u64 => "
      UART_2.uart2_srr_reg(),
    ",
  0x5002018cu64 => "
      UART_2.uart2_srts_reg(),
    ",
  0x5002019cu64 => "
      UART_2.uart2_srt_reg(),
    ",
  0x500201a0u64 => "
      UART_2.uart2_stet_reg(),
    ",
  0x500201c8u64 => "
      UART_2.uart2_tar_reg(),
    ",
  0x50020180u64 => "
      UART_2.uart2_tfl_reg(),
    ",
  0x500201f8u64 => "
      UART_2.uart2_ucv_reg(),
    ",
  0x5002017cu64 => "
      UART_2.uart2_usr_reg(),
    ",
  0x5002021cu64 => "
      UART_3.uart3_config_reg(),
    ",
  0x500202e0u64 => "
      UART_3.uart3_ctrl_reg(),
    ",
  0x500202fcu64 => "
      UART_3.uart3_ctr_reg(),
    ",
  0x500202c0u64 => "
      UART_3.uart3_dlf_reg(),
    ",
  0x500202a8u64 => "
      UART_3.uart3_dmasa_reg(),
    ",
  0x500202e8u64 => "
      UART_3.uart3_err_ctrl_reg(),
    ",
  0x500202a4u64 => "
      UART_3.uart3_htx_reg(),
    ",
  0x50020204u64 => "
      UART_3.uart3_ier_dlh_reg(),
    ",
  0x50020208u64 => "
      UART_3.uart3_iir_fcr_reg(),
    ",
  0x500202ecu64 => "
      UART_3.uart3_irq_status_reg(),
    ",
  0x500202ccu64 => "
      UART_3.uart3_lcr_ext(),
    ",
  0x5002020cu64 => "
      UART_3.uart3_lcr_reg(),
    ",
  0x50020214u64 => "
      UART_3.uart3_lsr_reg(),
    ",
  0x50020210u64 => "
      UART_3.uart3_mcr_reg(),
    ",
  0x50020218u64 => "
      UART_3.uart3_msr_reg(),
    ",
  0x500202c4u64 => "
      UART_3.uart3_rar_reg(),
    ",
  0x50020200u64 => "
      UART_3.uart3_rbr_thr_dll_reg(),
    ",
  0x50020284u64 => "
      UART_3.uart3_rfl_reg(),
    ",
  0x50020290u64 => "
      UART_3.uart3_sbcr_reg(),
    ",
  0x50020294u64 => "
      UART_3.uart3_sdmam_reg(),
    ",
  0x50020298u64 => "
      UART_3.uart3_sfe_reg(),
    ",
  0x50020230u64 => "
      UART_3.uart3_srbr_sthr0_reg(),
    ",
  0x50020258u64 => "
      UART_3.uart3_srbr_sthr10_reg(),
    ",
  0x5002025cu64 => "
      UART_3.uart3_srbr_sthr11_reg(),
    ",
  0x50020260u64 => "
      UART_3.uart3_srbr_sthr12_reg(),
    ",
  0x50020264u64 => "
      UART_3.uart3_srbr_sthr13_reg(),
    ",
  0x50020268u64 => "
      UART_3.uart3_srbr_sthr14_reg(),
    ",
  0x5002026cu64 => "
      UART_3.uart3_srbr_sthr15_reg(),
    ",
  0x50020234u64 => "
      UART_3.uart3_srbr_sthr1_reg(),
    ",
  0x50020238u64 => "
      UART_3.uart3_srbr_sthr2_reg(),
    ",
  0x5002023cu64 => "
      UART_3.uart3_srbr_sthr3_reg(),
    ",
  0x50020240u64 => "
      UART_3.uart3_srbr_sthr4_reg(),
    ",
  0x50020244u64 => "
      UART_3.uart3_srbr_sthr5_reg(),
    ",
  0x50020248u64 => "
      UART_3.uart3_srbr_sthr6_reg(),
    ",
  0x5002024cu64 => "
      UART_3.uart3_srbr_sthr7_reg(),
    ",
  0x50020250u64 => "
      UART_3.uart3_srbr_sthr8_reg(),
    ",
  0x50020254u64 => "
      UART_3.uart3_srbr_sthr9_reg(),
    ",
  0x50020288u64 => "
      UART_3.uart3_srr_reg(),
    ",
  0x5002028cu64 => "
      UART_3.uart3_srts_reg(),
    ",
  0x5002029cu64 => "
      UART_3.uart3_srt_reg(),
    ",
  0x500202a0u64 => "
      UART_3.uart3_stet_reg(),
    ",
  0x500202c8u64 => "
      UART_3.uart3_tar_reg(),
    ",
  0x50020280u64 => "
      UART_3.uart3_tfl_reg(),
    ",
  0x500202e4u64 => "
      UART_3.uart3_timer_reg(),
    ",
  0x500202f8u64 => "
      UART_3.uart3_ucv_reg(),
    ",
  0x5002027cu64 => "
      UART_3.uart3_usr_reg(),
    ",
  0x50040020u64 => "
      USB.usb_altev_reg(),
    ",
  0x50040024u64 => "
      USB.usb_altmsk_reg(),
    ",
  0x500401a8u64 => "
      USB.usb_charger_ctrl_reg(),
    ",
  0x500401acu64 => "
      USB.usb_charger_stat_reg(),
    ",
  0x500401a0u64 => "
      USB.usb_dma_ctrl_reg(),
    ",
  0x50040090u64 => "
      USB.usb_ep0_nak_reg(),
    ",
  0x50040080u64 => "
      USB.usb_epc0_reg(),
    ",
  0x500400a0u64 => "
      USB.usb_epc1_reg(),
    ",
  0x500400b0u64 => "
      USB.usb_epc2_reg(),
    ",
  0x500400c0u64 => "
      USB.usb_epc3_reg(),
    ",
  0x500400d0u64 => "
      USB.usb_epc4_reg(),
    ",
  0x500400e0u64 => "
      USB.usb_epc5_reg(),
    ",
  0x500400f0u64 => "
      USB.usb_epc6_reg(),
    ",
  0x50040010u64 => "
      USB.usb_far_reg(),
    ",
  0x50040048u64 => "
      USB.usb_fnh_reg(),
    ",
  0x5004004cu64 => "
      USB.usb_fnl_reg(),
    ",
  0x50040040u64 => "
      USB.usb_fwev_reg(),
    ",
  0x50040044u64 => "
      USB.usb_fwmsk_reg(),
    ",
  0x50040018u64 => "
      USB.usb_maev_reg(),
    ",
  0x5004001cu64 => "
      USB.usb_mamsk_reg(),
    ",
  0x50040000u64 => "
      USB.usb_mctrl_reg(),
    ",
  0x50040038u64 => "
      USB.usb_nakev_reg(),
    ",
  0x5004003cu64 => "
      USB.usb_nakmsk_reg(),
    ",
  0x50040014u64 => "
      USB.usb_nfsr_reg(),
    ",
  0x5004009cu64 => "
      USB.usb_rxc0_reg(),
    ",
  0x500400bcu64 => "
      USB.usb_rxc1_reg(),
    ",
  0x500400dcu64 => "
      USB.usb_rxc2_reg(),
    ",
  0x500400fcu64 => "
      USB.usb_rxc3_reg(),
    ",
  0x50040094u64 => "
      USB.usb_rxd0_reg(),
    ",
  0x500400b4u64 => "
      USB.usb_rxd1_reg(),
    ",
  0x500400d4u64 => "
      USB.usb_rxd2_reg(),
    ",
  0x500400f4u64 => "
      USB.usb_rxd3_reg(),
    ",
  0x50040030u64 => "
      USB.usb_rxev_reg(),
    ",
  0x50040034u64 => "
      USB.usb_rxmsk_reg(),
    ",
  0x50040098u64 => "
      USB.usb_rxs0_reg(),
    ",
  0x500400b8u64 => "
      USB.usb_rxs1_reg(),
    ",
  0x500400d8u64 => "
      USB.usb_rxs2_reg(),
    ",
  0x500400f8u64 => "
      USB.usb_rxs3_reg(),
    ",
  0x50040008u64 => "
      USB.usb_tcr_reg(),
    ",
  0x5004008cu64 => "
      USB.usb_txc0_reg(),
    ",
  0x500400acu64 => "
      USB.usb_txc1_reg(),
    ",
  0x500400ccu64 => "
      USB.usb_txc2_reg(),
    ",
  0x500400ecu64 => "
      USB.usb_txc3_reg(),
    ",
  0x50040084u64 => "
      USB.usb_txd0_reg(),
    ",
  0x500400a4u64 => "
      USB.usb_txd1_reg(),
    ",
  0x500400c4u64 => "
      USB.usb_txd2_reg(),
    ",
  0x500400e4u64 => "
      USB.usb_txd3_reg(),
    ",
  0x50040028u64 => "
      USB.usb_txev_reg(),
    ",
  0x5004002cu64 => "
      USB.usb_txmsk_reg(),
    ",
  0x50040088u64 => "
      USB.usb_txs0_reg(),
    ",
  0x500400a8u64 => "
      USB.usb_txs1_reg(),
    ",
  0x500400c8u64 => "
      USB.usb_txs2_reg(),
    ",
  0x500400e8u64 => "
      USB.usb_txs3_reg(),
    ",
  0x5004000cu64 => "
      USB.usb_utr_reg(),
    ",
  0x5004007cu64 => "
      USB.usb_ux20cdr_reg(),
    ",
  0x50040004u64 => "
      USB.usb_xcvdiag_reg(),
    ",
  0x50000148u64 => "
      WAKEUP.wkup_clear_p0_reg(),
    ",
  0x5000014cu64 => "
      WAKEUP.wkup_clear_p1_reg(),
    ",
  0x50000100u64 => "
      WAKEUP.wkup_ctrl_reg(),
    ",
  0x50000128u64 => "
      WAKEUP.wkup_pol_p0_reg(),
    ",
  0x5000012cu64 => "
      WAKEUP.wkup_pol_p1_reg(),
    ",
  0x50000108u64 => "
      WAKEUP.wkup_reset_irq_reg(),
    ",
  0x50000114u64 => "
      WAKEUP.wkup_select_p0_reg(),
    ",
  0x50000118u64 => "
      WAKEUP.wkup_select_p1_reg(),
    ",
  0x50000154u64 => "
      WAKEUP.wkup_sel_gpio_p0_reg(),
    ",
  0x50000158u64 => "
      WAKEUP.wkup_sel_gpio_p1_reg(),
    ",
  0x5000013cu64 => "
      WAKEUP.wkup_status_p0_reg(),
    ",
  0x50000140u64 => "
      WAKEUP.wkup_status_p1_reg(),
    ",
};
