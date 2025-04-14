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
  0x5005061cu64 => "
      ANAMISC_BIF.clk_cal_irq_reg(),
    ",
  0x50050614u64 => "
      ANAMISC_BIF.clk_ref_cnt_reg(),
    ",
  0x50050610u64 => "
      ANAMISC_BIF.clk_ref_sel_reg(),
    ",
  0x50050618u64 => "
      ANAMISC_BIF.clk_ref_val_reg(),
    ",
  0x100c0020u64 => "
      CACHE.cache_ctrl2_reg(),
    ",
  0x100c0040u64 => "
      CACHE.cache_flash_reg(),
    ",
  0x100c0030u64 => "
      CACHE.cache_mrm_ctrl_reg(),
    ",
  0x100c0048u64 => "
      CACHE.cache_mrm_hits1ws_reg(),
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
  0x51000628u64 => "
      CHARGER.charger_cc_charge_timer_reg(),
    ",
  0x51000600u64 => "
      CHARGER.charger_ctrl_reg(),
    ",
  0x51000610u64 => "
      CHARGER.charger_current_param_reg(),
    ",
  0x51000618u64 => "
      CHARGER.charger_current_status_reg(),
    ",
  0x5100062cu64 => "
      CHARGER.charger_cv_charge_timer_reg(),
    ",
  0x5100067cu64 => "
      CHARGER.charger_error_irq_clr_reg(),
    ",
  0x5100066cu64 => "
      CHARGER.charger_error_irq_mask_reg(),
    ",
  0x51000674u64 => "
      CHARGER.charger_error_irq_status_reg(),
    ",
  0x51000648u64 => "
      CHARGER.charger_jeita_current2_reg(),
    ",
  0x51000644u64 => "
      CHARGER.charger_jeita_current_reg(),
    ",
  0x51000634u64 => "
      CHARGER.charger_jeita_v_charge_reg(),
    ",
  0x51000640u64 => "
      CHARGER.charger_jeita_v_ovp_reg(),
    ",
  0x51000638u64 => "
      CHARGER.charger_jeita_v_precharge_reg(),
    ",
  0x5100063cu64 => "
      CHARGER.charger_jeita_v_replenish_reg(),
    ",
  0x51000680u64 => "
      CHARGER.charger_lock_reg(),
    ",
  0x51000624u64 => "
      CHARGER.charger_pre_charge_timer_reg(),
    ",
  0x51000664u64 => "
      CHARGER.charger_pwr_up_timer_reg(),
    ",
  0x51000678u64 => "
      CHARGER.charger_state_irq_clr_reg(),
    ",
  0x51000668u64 => "
      CHARGER.charger_state_irq_mask_reg(),
    ",
  0x51000670u64 => "
      CHARGER.charger_state_irq_status_reg(),
    ",
  0x51000608u64 => "
      CHARGER.charger_status_reg(),
    ",
  0x51000684u64 => "
      CHARGER.charger_swlock_reg(),
    ",
  0x5100065cu64 => "
      CHARGER.charger_tbat_comp_timer_reg(),
    ",
  0x51000658u64 => "
      CHARGER.charger_tbat_mon_timer_reg(),
    ",
  0x51000654u64 => "
      CHARGER.charger_tdie_comp_timer_reg(),
    ",
  0x51000620u64 => "
      CHARGER.charger_tempset2_param_reg(),
    ",
  0x5100061cu64 => "
      CHARGER.charger_tempset_param_reg(),
    ",
  0x51000660u64 => "
      CHARGER.charger_thot_comp_timer_reg(),
    ",
  0x51000630u64 => "
      CHARGER.charger_total_charge_timer_reg(),
    ",
  0x5100064cu64 => "
      CHARGER.charger_vbat_comp_timer_reg(),
    ",
  0x5100060cu64 => "
      CHARGER.charger_voltage_param_reg(),
    ",
  0x51000614u64 => "
      CHARGER.charger_voltage_status_reg(),
    ",
  0x51000650u64 => "
      CHARGER.charger_vovp_comp_timer_reg(),
    ",
  0x50040310u64 => "
      CHG_DET.chg_det_adc_ctrl_reg(),
    ",
  0x50040318u64 => "
      CHG_DET.chg_det_dcd_timer_reg(),
    ",
  0x50040300u64 => "
      CHG_DET.chg_det_fsm_ctrl_reg(),
    ",
  0x50040304u64 => "
      CHG_DET.chg_det_fsm_status_reg(),
    ",
  0x50040324u64 => "
      CHG_DET.chg_det_irq_clear_reg(),
    ",
  0x5004031cu64 => "
      CHG_DET.chg_det_irq_mask_reg(),
    ",
  0x50040320u64 => "
      CHG_DET.chg_det_irq_status_reg(),
    ",
  0x5004030cu64 => "
      CHG_DET.chg_det_status_reg(),
    ",
  0x50040308u64 => "
      CHG_DET.chg_det_sw_ctrl_reg(),
    ",
  0x50040314u64 => "
      CHG_DET.chg_det_timer_reg(),
    ",
  0x50030040u64 => "
      CRG_AUD.pcm_div_reg(),
    ",
  0x50030044u64 => "
      CRG_AUD.pcm_fdiv_reg(),
    ",
  0x50030048u64 => "
      CRG_AUD.pdm_div_reg(),
    ",
  0x5003004cu64 => "
      CRG_AUD.src_div_reg(),
    ",
  0x50060004u64 => "
      CRG_CTRL.clk_pdctrl_reg(),
    ",
  0x51001004u64 => "
      CRG_GPU.clk_gpu_reg(),
    ",
  0x50020904u64 => "
      CRG_SNC.clk_snc_reg(),
    ",
  0x5002090cu64 => "
      CRG_SNC.reset_clk_snc_reg(),
    ",
  0x50020908u64 => "
      CRG_SNC.set_clk_snc_reg(),
    ",
  0x50040410u64 => "
      CRG_SYS.batcheck_reg(),
    ",
  0x50040400u64 => "
      CRG_SYS.clk_sys_reg(),
    ",
  0x5004040cu64 => "
      CRG_SYS.reset_clk_sys_reg(),
    ",
  0x50040408u64 => "
      CRG_SYS.set_clk_sys_reg(),
    ",
  0x500000ecu64 => "
      CRG_TOP.ana_status_reg(),
    ",
  0x50000050u64 => "
      CRG_TOP.bandgap_reg(),
    ",
  0x500000e8u64 => "
      CRG_TOP.bias_vref_sel_reg(),
    ",
  0x50000060u64 => "
      CRG_TOP.bod_ctrl_reg(),
    ",
  0x50000064u64 => "
      CRG_TOP.bod_status_reg(),
    ",
  0x50000000u64 => "
      CRG_TOP.clk_amba_reg(),
    ",
  0x50000004u64 => "
      CRG_TOP.clk_cmac_switch_reg(),
    ",
  0x50000014u64 => "
      CRG_TOP.clk_ctrl_reg(),
    ",
  0x50000010u64 => "
      CRG_TOP.clk_radio_reg(),
    ",
  0x50000044u64 => "
      CRG_TOP.clk_rchs_reg(),
    ",
  0x5000003cu64 => "
      CRG_TOP.clk_rclp_reg(),
    ",
  0x50000048u64 => "
      CRG_TOP.clk_rcx_reg(),
    ",
  0x5000004cu64 => "
      CRG_TOP.clk_rtcdiv_reg(),
    ",
  0x5000002cu64 => "
      CRG_TOP.clk_snc_ctrl_reg(),
    ",
  0x5000001cu64 => "
      CRG_TOP.clk_switch2xtal_reg(),
    ",
  0x50000018u64 => "
      CRG_TOP.clk_tmr_reg(),
    ",
  0x50000040u64 => "
      CRG_TOP.clk_xtal32k_reg(),
    ",
  0x500000d4u64 => "
      CRG_TOP.discharge_rail_reg(),
    ",
  0x50000034u64 => "
      CRG_TOP.lcd_ext_ctrl_reg(),
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
  0x50000088u64 => "
      CRG_TOP.p2_pad_latch_reg(),
    ",
  0x50000090u64 => "
      CRG_TOP.p2_reset_pad_latch_reg(),
    ",
  0x5000008cu64 => "
      CRG_TOP.p2_set_pad_latch_reg(),
    ",
  0x50000020u64 => "
      CRG_TOP.pmu_ctrl_reg(),
    ",
  0x500000f4u64 => "
      CRG_TOP.pmu_sleep_reg(),
    ",
  0x500000d0u64 => "
      CRG_TOP.pmu_trim_reg(),
    ",
  0x50000094u64 => "
      CRG_TOP.por_ctrl_reg(),
    ",
  0x50000098u64 => "
      CRG_TOP.por_pin_reg(),
    ",
  0x5000009cu64 => "
      CRG_TOP.por_timer_reg(),
    ",
  0x500000f0u64 => "
      CRG_TOP.power_ctrl_reg(),
    ",
  0x500000f8u64 => "
      CRG_TOP.power_lvl_reg(),
    ",
  0x500000c0u64 => "
      CRG_TOP.ram_pwr_ctrl_reg(),
    ",
  0x500000bcu64 => "
      CRG_TOP.reset_stat_reg(),
    ",
  0x5000000cu64 => "
      CRG_TOP.rst_ctrl_reg(),
    ",
  0x500000ccu64 => "
      CRG_TOP.secure_boot_reg(),
    ",
  0x50000030u64 => "
      CRG_TOP.slp_map_reg(),
    ",
  0x500000e4u64 => "
      CRG_TOP.sw_v18f_reg(),
    ",
  0x50000024u64 => "
      CRG_TOP.sys_ctrl_reg(),
    ",
  0x50000028u64 => "
      CRG_TOP.sys_stat_reg(),
    ",
  0x50000058u64 => "
      CRG_TOP.vbus_irq_clear_reg(),
    ",
  0x50000054u64 => "
      CRG_TOP.vbus_irq_mask_reg(),
    ",
  0x500000e0u64 => "
      CRG_TOP.wakeup_hibern_reg(),
    ",
  0x50000b00u64 => "
      CRG_VSYS.vsys_gen_ctrl_reg(),
    ",
  0x50000b08u64 => "
      CRG_VSYS.vsys_gen_irq_clear_reg(),
    ",
  0x50000b0cu64 => "
      CRG_VSYS.vsys_gen_irq_mask_reg(),
    ",
  0x50000b04u64 => "
      CRG_VSYS.vsys_gen_irq_status_reg(),
    ",
  0x50050460u64 => "
      CRG_XTAL.pll_sys_ctrl1_reg(),
    ",
  0x50050464u64 => "
      CRG_XTAL.pll_sys_ctrl2_reg(),
    ",
  0x50050468u64 => "
      CRG_XTAL.pll_sys_ctrl3_reg(),
    ",
  0x50050470u64 => "
      CRG_XTAL.pll_sys_status_reg(),
    ",
  0x50050474u64 => "
      CRG_XTAL.pll_usb_ctrl1_reg(),
    ",
  0x50050478u64 => "
      CRG_XTAL.pll_usb_ctrl2_reg(),
    ",
  0x5005047cu64 => "
      CRG_XTAL.pll_usb_ctrl3_reg(),
    ",
  0x50050480u64 => "
      CRG_XTAL.pll_usb_status_reg(),
    ",
  0x50050498u64 => "
      CRG_XTAL.reset_sys_irq_ctrl_reg(),
    ",
  0x50050494u64 => "
      CRG_XTAL.set_sys_irq_ctrl_reg(),
    ",
  0x50050490u64 => "
      CRG_XTAL.sys_irq_ctrl_reg(),
    ",
  0x5005040cu64 => "
      CRG_XTAL.xtal32m_cap_meas_reg(),
    ",
  0x50050414u64 => "
      CRG_XTAL.xtal32m_ctrl_reg(),
    ",
  0x50050410u64 => "
      CRG_XTAL.xtal32m_fsm_reg(),
    ",
  0x50050418u64 => "
      CRG_XTAL.xtal32m_irq_ctrl_reg(),
    ",
  0x50050428u64 => "
      CRG_XTAL.xtal32m_irq_stat_reg(),
    ",
  0x50050404u64 => "
      CRG_XTAL.xtal32m_settle_reg(),
    ",
  0x50050400u64 => "
      CRG_XTAL.xtal32m_start_reg(),
    ",
  0x50050424u64 => "
      CRG_XTAL.xtal32m_stat0_reg(),
    ",
  0x50050408u64 => "
      CRG_XTAL.xtal32m_trim_reg(),
    ",
  0x30100004u64 => "
      DCACHE.dcache_base_addr_reg(),
    ",
  0x30100000u64 => "
      DCACHE.dcache_ctrl_reg(),
    ",
  0x30100014u64 => "
      DCACHE.dcache_mrm_ctrl_reg(),
    ",
  0x30100010u64 => "
      DCACHE.dcache_mrm_evicts_reg(),
    ",
  0x30100024u64 => "
      DCACHE.dcache_mrm_evicts_thres_reg(),
    ",
  0x30100008u64 => "
      DCACHE.dcache_mrm_hits_reg(),
    ",
  0x30100020u64 => "
      DCACHE.dcache_mrm_hits_thres_reg(),
    ",
  0x3010000cu64 => "
      DCACHE.dcache_mrm_misses_reg(),
    ",
  0x3010001cu64 => "
      DCACHE.dcache_mrm_misses_thres_reg(),
    ",
  0x30100018u64 => "
      DCACHE.dcache_mrm_tint_reg(),
    ",
  0x50000300u64 => "
      DCDC.buck_ctrl_reg(),
    ",
  0x50000508u64 => "
      DCDC_BOOST.boost_ctrl_reg0(),
    ",
  0x5000050cu64 => "
      DCDC_BOOST.boost_ctrl_reg1(),
    ",
  0x50000514u64 => "
      DCDC_BOOST.boost_status_reg(),
    ",
  0x50000510u64 => "
      DCDC_BOOST.boost_test_ctrl_reg(),
    ",
  0x50000500u64 => "
      DCDC_BOOST.vled_pwr_ctrl_reg(),
    ",
  0x50000504u64 => "
      DCDC_BOOST.vled_pwr_status_reg(),
    ",
  0x51000400u64 => "
      DMA.dma0_a_start_reg(),
    ",
  0x51000404u64 => "
      DMA.dma0_b_start_reg(),
    ",
  0x51000410u64 => "
      DMA.dma0_ctrl_reg(),
    ",
  0x51000414u64 => "
      DMA.dma0_idx_reg(),
    ",
  0x51000408u64 => "
      DMA.dma0_int_reg(),
    ",
  0x5100040cu64 => "
      DMA.dma0_len_reg(),
    ",
  0x51000420u64 => "
      DMA.dma1_a_start_reg(),
    ",
  0x51000424u64 => "
      DMA.dma1_b_start_reg(),
    ",
  0x51000430u64 => "
      DMA.dma1_ctrl_reg(),
    ",
  0x51000434u64 => "
      DMA.dma1_idx_reg(),
    ",
  0x51000428u64 => "
      DMA.dma1_int_reg(),
    ",
  0x5100042cu64 => "
      DMA.dma1_len_reg(),
    ",
  0x51000440u64 => "
      DMA.dma2_a_start_reg(),
    ",
  0x51000444u64 => "
      DMA.dma2_b_start_reg(),
    ",
  0x51000450u64 => "
      DMA.dma2_ctrl_reg(),
    ",
  0x51000454u64 => "
      DMA.dma2_idx_reg(),
    ",
  0x51000448u64 => "
      DMA.dma2_int_reg(),
    ",
  0x5100044cu64 => "
      DMA.dma2_len_reg(),
    ",
  0x51000460u64 => "
      DMA.dma3_a_start_reg(),
    ",
  0x51000464u64 => "
      DMA.dma3_b_start_reg(),
    ",
  0x51000470u64 => "
      DMA.dma3_ctrl_reg(),
    ",
  0x51000474u64 => "
      DMA.dma3_idx_reg(),
    ",
  0x51000468u64 => "
      DMA.dma3_int_reg(),
    ",
  0x5100046cu64 => "
      DMA.dma3_len_reg(),
    ",
  0x51000480u64 => "
      DMA.dma4_a_start_reg(),
    ",
  0x51000484u64 => "
      DMA.dma4_b_start_reg(),
    ",
  0x51000490u64 => "
      DMA.dma4_ctrl_reg(),
    ",
  0x51000494u64 => "
      DMA.dma4_idx_reg(),
    ",
  0x51000488u64 => "
      DMA.dma4_int_reg(),
    ",
  0x5100048cu64 => "
      DMA.dma4_len_reg(),
    ",
  0x510004a0u64 => "
      DMA.dma5_a_start_reg(),
    ",
  0x510004a4u64 => "
      DMA.dma5_b_start_reg(),
    ",
  0x510004b0u64 => "
      DMA.dma5_ctrl_reg(),
    ",
  0x510004b4u64 => "
      DMA.dma5_idx_reg(),
    ",
  0x510004a8u64 => "
      DMA.dma5_int_reg(),
    ",
  0x510004acu64 => "
      DMA.dma5_len_reg(),
    ",
  0x510004c0u64 => "
      DMA.dma6_a_start_reg(),
    ",
  0x510004c4u64 => "
      DMA.dma6_b_start_reg(),
    ",
  0x510004d0u64 => "
      DMA.dma6_ctrl_reg(),
    ",
  0x510004d4u64 => "
      DMA.dma6_idx_reg(),
    ",
  0x510004c8u64 => "
      DMA.dma6_int_reg(),
    ",
  0x510004ccu64 => "
      DMA.dma6_len_reg(),
    ",
  0x510004e0u64 => "
      DMA.dma7_a_start_reg(),
    ",
  0x510004e4u64 => "
      DMA.dma7_b_start_reg(),
    ",
  0x510004f0u64 => "
      DMA.dma7_ctrl_reg(),
    ",
  0x510004f4u64 => "
      DMA.dma7_idx_reg(),
    ",
  0x510004e8u64 => "
      DMA.dma7_int_reg(),
    ",
  0x510004ecu64 => "
      DMA.dma7_len_reg(),
    ",
  0x51000508u64 => "
      DMA.dma_clear_int_reg(),
    ",
  0x5100050cu64 => "
      DMA.dma_int_mask_reg(),
    ",
  0x51000504u64 => "
      DMA.dma_int_status_reg(),
    ",
  0x51000500u64 => "
      DMA.dma_req_mux_reg(),
    ",
  0x51000514u64 => "
      DMA.dma_reset_int_mask_reg(),
    ",
  0x51000510u64 => "
      DMA.dma_set_int_mask_reg(),
    ",
  0x51001314u64 => "
      DSI_2.dsi2_cfg_autoinsert_eotp_reg(),
    ",
  0x51001328u64 => "
      DSI_2.dsi2_cfg_bta_h_to_count_reg(),
    ",
  0x51001360u64 => "
      DSI_2.dsi2_cfg_clk_lane_en_reg(),
    ",
  0x51001304u64 => "
      DSI_2.dsi2_cfg_continuous_hs_clk_reg(),
    ",
  0x51001364u64 => "
      DSI_2.dsi2_cfg_data_lane_en_reg(),
    ",
  0x51001330u64 => "
      DSI_2.dsi2_cfg_disable_burst_reg(),
    ",
  0x5100131cu64 => "
      DSI_2.dsi2_cfg_disbl_rx_crc_check_reg(),
    ",
  0x51001318u64 => "
      DSI_2.dsi2_cfg_ext_cmds_aft_eotp_reg(),
    ",
  0x51001320u64 => "
      DSI_2.dsi2_cfg_htx_to_count_reg(),
    ",
  0x510014a8u64 => "
      DSI_2.dsi2_cfg_irq_mask(),
    ",
  0x510014acu64 => "
      DSI_2.dsi2_cfg_irq_mask2(),
    ",
  0x510014a0u64 => "
      DSI_2.dsi2_cfg_irq_status(),
    ",
  0x510014a4u64 => "
      DSI_2.dsi2_cfg_irq_status2(),
    ",
  0x51001324u64 => "
      DSI_2.dsi2_cfg_lrx_h_to_count_reg(),
    ",
  0x51001300u64 => "
      DSI_2.dsi2_cfg_num_lanes_reg(),
    ",
  0x51001484u64 => "
      DSI_2.dsi2_cfg_packet_control(),
    ",
  0x51001494u64 => "
      DSI_2.dsi2_cfg_pkt_rd_level(),
    ",
  0x5100149cu64 => "
      DSI_2.dsi2_cfg_pkt_rx_header(),
    ",
  0x51001498u64 => "
      DSI_2.dsi2_cfg_pkt_rx_payload(),
    ",
  0x5100148cu64 => "
      DSI_2.dsi2_cfg_pkt_status(),
    ",
  0x51001490u64 => "
      DSI_2.dsi2_cfg_pkt_wr_level(),
    ",
  0x51001344u64 => "
      DSI_2.dsi2_cfg_rx_error_status_reg(),
    ",
  0x51001488u64 => "
      DSI_2.dsi2_cfg_send_packet(),
    ",
  0x51001340u64 => "
      DSI_2.dsi2_cfg_status_out_reg(),
    ",
  0x5100132cu64 => "
      DSI_2.dsi2_cfg_twakeup_reg(),
    ",
  0x51001310u64 => "
      DSI_2.dsi2_cfg_tx_gap_reg(),
    ",
  0x51001480u64 => "
      DSI_2.dsi2_cfg_tx_payload(),
    ",
  0x5100130cu64 => "
      DSI_2.dsi2_cfg_t_post_reg(),
    ",
  0x51001308u64 => "
      DSI_2.dsi2_cfg_t_pre_reg(),
    ",
  0x51001440u64 => "
      DSI_2.dsi2_cfg_vid_bllp_mode(),
    ",
  0x51001400u64 => "
      DSI_2.dsi2_cfg_vid_enable(),
    ",
  0x5100142cu64 => "
      DSI_2.dsi2_cfg_vid_hbp(),
    ",
  0x51001428u64 => "
      DSI_2.dsi2_cfg_vid_hfp(),
    ",
  0x51001430u64 => "
      DSI_2.dsi2_cfg_vid_hsa(),
    ",
  0x51001418u64 => "
      DSI_2.dsi2_cfg_vid_hsync_polarity(),
    ",
  0x51001420u64 => "
      DSI_2.dsi2_cfg_vid_override(),
    ",
  0x51001434u64 => "
      DSI_2.dsi2_cfg_vid_packets_p_line(),
    ",
  0x5100140cu64 => "
      DSI_2.dsi2_cfg_vid_pix_alignment(),
    ",
  0x51001410u64 => "
      DSI_2.dsi2_cfg_vid_pix_format(),
    ",
  0x51001408u64 => "
      DSI_2.dsi2_cfg_vid_pix_payload_size(),
    ",
  0x51001404u64 => "
      DSI_2.dsi2_cfg_vid_pix_p_packet(),
    ",
  0x51001424u64 => "
      DSI_2.dsi2_cfg_vid_start_delay(),
    ",
  0x51001444u64 => "
      DSI_2.dsi2_cfg_vid_use_null_pkt_bllp(),
    ",
  0x51001448u64 => "
      DSI_2.dsi2_cfg_vid_vactive(),
    ",
  0x51001438u64 => "
      DSI_2.dsi2_cfg_vid_vbp(),
    ",
  0x5100144cu64 => "
      DSI_2.dsi2_cfg_vid_vc(),
    ",
  0x5100143cu64 => "
      DSI_2.dsi2_cfg_vid_vfp(),
    ",
  0x5100141cu64 => "
      DSI_2.dsi2_cfg_vid_video_mode(),
    ",
  0x51001414u64 => "
      DSI_2.dsi2_cfg_vid_vsync_polarity(),
    ",
  0x51001518u64 => "
      DSIDPHY_REG.dphy_bist_dc_out_reg(),
    ",
  0x51001510u64 => "
      DSIDPHY_REG.dphy_bist_enbl_reg(),
    ",
  0x51001514u64 => "
      DSIDPHY_REG.dphy_bist_pattern_reg(),
    ",
  0x5100150cu64 => "
      DSIDPHY_REG.dphy_clk_data_lane_prog_reg(),
    ",
  0x51001500u64 => "
      DSIDPHY_REG.dphy_global_ctrl_reg(),
    ",
  0x51001504u64 => "
      DSIDPHY_REG.dphy_pll_control_reg(),
    ",
  0x51001508u64 => "
      DSIDPHY_REG.dphy_status_reg(),
    ",
  0x5100151cu64 => "
      DSIDPHY_REG.dphy_tx_rcal_reg(),
    ",
  0x51001520u64 => "
      DSIDPHY_REG.dsi2_dphy_clk_rst_n_ctrl_reg(),
    ",
  0x51001524u64 => "
      DSIDPHY_REG.dsi2_error_status_reg(),
    ",
  0x51001528u64 => "
      DSIDPHY_REG.dsi2_interrupt_en_reg(),
    ",
  0x51001534u64 => "
      DSIDPHY_REG.dsi2_memctrl_reg(),
    ",
  0x51001530u64 => "
      DSIDPHY_REG.dsi2_trigger_reg(),
    ",
  0x5100152cu64 => "
      DSIDPHY_REG.dsi2_ulps_cfg_reg(),
    ",
  0x30058054u64 => "
      EMMC.emmc_adma_err_stat_r_reg(),
    ",
  0x30058058u64 => "
      EMMC.emmc_adma_sa_low_r_reg(),
    ",
  0x30058008u64 => "
      EMMC.emmc_argument_r_reg(),
    ",
  0x30058540u64 => "
      EMMC.emmc_at_ctrl_r_reg(),
    ",
  0x30058544u64 => "
      EMMC.emmc_at_stat_r_reg(),
    ",
  0x3005803cu64 => "
      EMMC.emmc_auto_cmd_stat_r_reg(),
    ",
  0x3005802au64 => "
      EMMC.emmc_bgap_ctrl_r_reg(),
    ",
  0x30058006u64 => "
      EMMC.emmc_blockcount_r_reg(),
    ",
  0x30058004u64 => "
      EMMC.emmc_blocksize_r_reg(),
    ",
  0x3005852eu64 => "
      EMMC.emmc_boot_ctrl_r_reg(),
    ",
  0x30058020u64 => "
      EMMC.emmc_buf_data_r_reg(),
    ",
  0x30058040u64 => "
      EMMC.emmc_capabilities1_r_reg(),
    ",
  0x30058044u64 => "
      EMMC.emmc_capabilities2_r_reg(),
    ",
  0x3005802cu64 => "
      EMMC.emmc_clk_ctrl_r_reg(),
    ",
  0x3005800eu64 => "
      EMMC.emmc_cmd_r_reg(),
    ",
  0x30058184u64 => "
      EMMC.emmc_cqcap_reg(),
    ",
  0x30058048u64 => "
      EMMC.emmc_curr_capabilities1_r_reg(),
    ",
  0x3005804cu64 => "
      EMMC.emmc_curr_capabilities2_r_reg(),
    ",
  0x30058f6cu64 => "
      EMMC.emmc_embedded_ctrl_r_reg(),
    ",
  0x3005852cu64 => "
      EMMC.emmc_emmc_ctrl_r_reg(),
    ",
  0x3005803au64 => "
      EMMC.emmc_error_int_signal_en_r_reg(),
    ",
  0x30058036u64 => "
      EMMC.emmc_error_int_stat_en_r_reg(),
    ",
  0x30058032u64 => "
      EMMC.emmc_error_int_stat_r_reg(),
    ",
  0x30058050u64 => "
      EMMC.emmc_force_auto_cmd_stat_r_reg(),
    ",
  0x30058052u64 => "
      EMMC.emmc_force_error_int_stat_r_reg(),
    ",
  0x300580feu64 => "
      EMMC.emmc_host_cntrl_vers_r_reg(),
    ",
  0x30058028u64 => "
      EMMC.emmc_host_ctrl1_r_reg(),
    ",
  0x3005803eu64 => "
      EMMC.emmc_host_ctrl2_r_reg(),
    ",
  0x30058510u64 => "
      EMMC.emmc_mbiu_ctrl_r_reg(),
    ",
  0x30058508u64 => "
      EMMC.emmc_mshc_ctrl_r_reg(),
    ",
  0x30058500u64 => "
      EMMC.emmc_mshc_ver_id_r_reg(),
    ",
  0x30058504u64 => "
      EMMC.emmc_mshc_ver_type_r_reg(),
    ",
  0x30058038u64 => "
      EMMC.emmc_normal_int_signal_en_r_reg(),
    ",
  0x30058034u64 => "
      EMMC.emmc_normal_int_stat_en_r_reg(),
    ",
  0x30058030u64 => "
      EMMC.emmc_normal_int_stat_r_reg(),
    ",
  0x3005806eu64 => "
      EMMC.emmc_preset_ddr50_r_reg(),
    ",
  0x30058062u64 => "
      EMMC.emmc_preset_ds_r_reg(),
    ",
  0x30058064u64 => "
      EMMC.emmc_preset_hs_r_reg(),
    ",
  0x30058060u64 => "
      EMMC.emmc_preset_init_r_reg(),
    ",
  0x3005806cu64 => "
      EMMC.emmc_preset_sdr104_r_reg(),
    ",
  0x30058066u64 => "
      EMMC.emmc_preset_sdr12_r_reg(),
    ",
  0x30058068u64 => "
      EMMC.emmc_preset_sdr25_r_reg(),
    ",
  0x3005806au64 => "
      EMMC.emmc_preset_sdr50_r_reg(),
    ",
  0x30058074u64 => "
      EMMC.emmc_preset_uhs2_r_reg(),
    ",
  0x30058024u64 => "
      EMMC.emmc_pstate_reg(),
    ",
  0x30058029u64 => "
      EMMC.emmc_pwr_ctrl_r_reg(),
    ",
  0x300580e6u64 => "
      EMMC.emmc_p_embedded_cntrl_reg(),
    ",
  0x300580e8u64 => "
      EMMC.emmc_p_vendor_specific_area_reg(),
    ",
  0x300580eau64 => "
      EMMC.emmc_p_vndr2_specific_area_reg(),
    ",
  0x30058010u64 => "
      EMMC.emmc_resp01_r_reg(),
    ",
  0x30058014u64 => "
      EMMC.emmc_resp23_r_reg(),
    ",
  0x30058018u64 => "
      EMMC.emmc_resp45_r_reg(),
    ",
  0x3005801cu64 => "
      EMMC.emmc_resp67_r_reg(),
    ",
  0x30058000u64 => "
      EMMC.emmc_sdmasa_r_reg(),
    ",
  0x300580fcu64 => "
      EMMC.emmc_slot_intr_status_r_reg(),
    ",
  0x3005802fu64 => "
      EMMC.emmc_sw_rst_r_reg(),
    ",
  0x3005802eu64 => "
      EMMC.emmc_tout_ctrl_r_reg(),
    ",
  0x3005802bu64 => "
      EMMC.emmc_wup_ctrl_r_reg(),
    ",
  0x3005800cu64 => "
      EMMC.emmc_xfer_mode_r_reg(),
    ",
  0x5002081cu64 => "
      GPADC.gp_adc_clear_int_reg(),
    ",
  0x50020804u64 => "
      GPADC.gp_adc_ctrl2_reg(),
    ",
  0x50020808u64 => "
      GPADC.gp_adc_ctrl3_reg(),
    ",
  0x50020800u64 => "
      GPADC.gp_adc_ctrl_reg(),
    ",
  0x50020814u64 => "
      GPADC.gp_adc_offn_reg(),
    ",
  0x50020810u64 => "
      GPADC.gp_adc_offp_reg(),
    ",
  0x50020820u64 => "
      GPADC.gp_adc_result_reg(),
    ",
  0x5002080cu64 => "
      GPADC.gp_adc_sel_reg(),
    ",
  0x50020818u64 => "
      GPADC.gp_adc_trim_reg(),
    ",
  0x5005026cu64 => "
      GPIO.gpio_clk_sel_reg(),
    ",
  0x5005027cu64 => "
      GPIO.lcdc_map_ctrl_reg(),
    ",
  0x50050124u64 => "
      GPIO.p0_00_mode_reg(),
    ",
  0x50050128u64 => "
      GPIO.p0_01_mode_reg(),
    ",
  0x5005012cu64 => "
      GPIO.p0_02_mode_reg(),
    ",
  0x50050130u64 => "
      GPIO.p0_03_mode_reg(),
    ",
  0x50050134u64 => "
      GPIO.p0_04_mode_reg(),
    ",
  0x50050138u64 => "
      GPIO.p0_05_mode_reg(),
    ",
  0x5005013cu64 => "
      GPIO.p0_06_mode_reg(),
    ",
  0x50050140u64 => "
      GPIO.p0_07_mode_reg(),
    ",
  0x50050144u64 => "
      GPIO.p0_08_mode_reg(),
    ",
  0x50050148u64 => "
      GPIO.p0_09_mode_reg(),
    ",
  0x5005014cu64 => "
      GPIO.p0_10_mode_reg(),
    ",
  0x50050150u64 => "
      GPIO.p0_11_mode_reg(),
    ",
  0x50050154u64 => "
      GPIO.p0_12_mode_reg(),
    ",
  0x50050158u64 => "
      GPIO.p0_13_mode_reg(),
    ",
  0x5005015cu64 => "
      GPIO.p0_14_mode_reg(),
    ",
  0x50050160u64 => "
      GPIO.p0_15_mode_reg(),
    ",
  0x50050164u64 => "
      GPIO.p0_16_mode_reg(),
    ",
  0x50050168u64 => "
      GPIO.p0_17_mode_reg(),
    ",
  0x5005016cu64 => "
      GPIO.p0_18_mode_reg(),
    ",
  0x50050170u64 => "
      GPIO.p0_19_mode_reg(),
    ",
  0x50050174u64 => "
      GPIO.p0_20_mode_reg(),
    ",
  0x50050178u64 => "
      GPIO.p0_21_mode_reg(),
    ",
  0x5005017cu64 => "
      GPIO.p0_22_mode_reg(),
    ",
  0x50050180u64 => "
      GPIO.p0_23_mode_reg(),
    ",
  0x50050184u64 => "
      GPIO.p0_24_mode_reg(),
    ",
  0x50050188u64 => "
      GPIO.p0_25_mode_reg(),
    ",
  0x5005018cu64 => "
      GPIO.p0_26_mode_reg(),
    ",
  0x50050190u64 => "
      GPIO.p0_27_mode_reg(),
    ",
  0x50050194u64 => "
      GPIO.p0_28_mode_reg(),
    ",
  0x50050198u64 => "
      GPIO.p0_29_mode_reg(),
    ",
  0x5005019cu64 => "
      GPIO.p0_30_mode_reg(),
    ",
  0x500501a0u64 => "
      GPIO.p0_31_mode_reg(),
    ",
  0x50050100u64 => "
      GPIO.p0_data_reg(),
    ",
  0x50050260u64 => "
      GPIO.p0_padpwr_ctrl_reg(),
    ",
  0x50050118u64 => "
      GPIO.p0_reset_data_reg(),
    ",
  0x5005010cu64 => "
      GPIO.p0_set_data_reg(),
    ",
  0x50050270u64 => "
      GPIO.p0_weak_ctrl_reg(),
    ",
  0x500501a4u64 => "
      GPIO.p1_00_mode_reg(),
    ",
  0x500501a8u64 => "
      GPIO.p1_01_mode_reg(),
    ",
  0x500501acu64 => "
      GPIO.p1_02_mode_reg(),
    ",
  0x500501b0u64 => "
      GPIO.p1_03_mode_reg(),
    ",
  0x500501b4u64 => "
      GPIO.p1_04_mode_reg(),
    ",
  0x500501b8u64 => "
      GPIO.p1_05_mode_reg(),
    ",
  0x500501bcu64 => "
      GPIO.p1_06_mode_reg(),
    ",
  0x500501c0u64 => "
      GPIO.p1_07_mode_reg(),
    ",
  0x500501c4u64 => "
      GPIO.p1_08_mode_reg(),
    ",
  0x500501c8u64 => "
      GPIO.p1_09_mode_reg(),
    ",
  0x500501ccu64 => "
      GPIO.p1_10_mode_reg(),
    ",
  0x500501d0u64 => "
      GPIO.p1_11_mode_reg(),
    ",
  0x500501d4u64 => "
      GPIO.p1_12_mode_reg(),
    ",
  0x500501d8u64 => "
      GPIO.p1_13_mode_reg(),
    ",
  0x500501dcu64 => "
      GPIO.p1_14_mode_reg(),
    ",
  0x500501e0u64 => "
      GPIO.p1_15_mode_reg(),
    ",
  0x500501e4u64 => "
      GPIO.p1_16_mode_reg(),
    ",
  0x500501e8u64 => "
      GPIO.p1_17_mode_reg(),
    ",
  0x500501ecu64 => "
      GPIO.p1_18_mode_reg(),
    ",
  0x500501f0u64 => "
      GPIO.p1_19_mode_reg(),
    ",
  0x500501f4u64 => "
      GPIO.p1_20_mode_reg(),
    ",
  0x500501f8u64 => "
      GPIO.p1_21_mode_reg(),
    ",
  0x500501fcu64 => "
      GPIO.p1_22_mode_reg(),
    ",
  0x50050200u64 => "
      GPIO.p1_23_mode_reg(),
    ",
  0x50050204u64 => "
      GPIO.p1_24_mode_reg(),
    ",
  0x50050208u64 => "
      GPIO.p1_25_mode_reg(),
    ",
  0x5005020cu64 => "
      GPIO.p1_26_mode_reg(),
    ",
  0x50050210u64 => "
      GPIO.p1_27_mode_reg(),
    ",
  0x50050214u64 => "
      GPIO.p1_28_mode_reg(),
    ",
  0x50050218u64 => "
      GPIO.p1_29_mode_reg(),
    ",
  0x5005021cu64 => "
      GPIO.p1_30_mode_reg(),
    ",
  0x50050220u64 => "
      GPIO.p1_31_mode_reg(),
    ",
  0x50050104u64 => "
      GPIO.p1_data_reg(),
    ",
  0x50050264u64 => "
      GPIO.p1_padpwr_ctrl_reg(),
    ",
  0x5005011cu64 => "
      GPIO.p1_reset_data_reg(),
    ",
  0x50050110u64 => "
      GPIO.p1_set_data_reg(),
    ",
  0x50050274u64 => "
      GPIO.p1_weak_ctrl_reg(),
    ",
  0x50050224u64 => "
      GPIO.p2_00_mode_reg(),
    ",
  0x50050228u64 => "
      GPIO.p2_01_mode_reg(),
    ",
  0x5005022cu64 => "
      GPIO.p2_02_mode_reg(),
    ",
  0x50050230u64 => "
      GPIO.p2_03_mode_reg(),
    ",
  0x50050234u64 => "
      GPIO.p2_04_mode_reg(),
    ",
  0x50050238u64 => "
      GPIO.p2_05_mode_reg(),
    ",
  0x5005023cu64 => "
      GPIO.p2_06_mode_reg(),
    ",
  0x50050240u64 => "
      GPIO.p2_07_mode_reg(),
    ",
  0x50050244u64 => "
      GPIO.p2_08_mode_reg(),
    ",
  0x50050248u64 => "
      GPIO.p2_09_mode_reg(),
    ",
  0x5005024cu64 => "
      GPIO.p2_10_mode_reg(),
    ",
  0x50050250u64 => "
      GPIO.p2_11_mode_reg(),
    ",
  0x50050254u64 => "
      GPIO.p2_12_mode_reg(),
    ",
  0x50050258u64 => "
      GPIO.p2_13_mode_reg(),
    ",
  0x5005025cu64 => "
      GPIO.p2_14_mode_reg(),
    ",
  0x50050108u64 => "
      GPIO.p2_data_reg(),
    ",
  0x50050268u64 => "
      GPIO.p2_padpwr_ctrl_reg(),
    ",
  0x50050120u64 => "
      GPIO.p2_reset_data_reg(),
    ",
  0x50050114u64 => "
      GPIO.p2_set_data_reg(),
    ",
  0x50050278u64 => "
      GPIO.p2_weak_ctrl_reg(),
    ",
  0x50050280u64 => "
      GPIO.pad_drive_ctrl_reg(),
    ",
  0x50040108u64 => "
      GPREG.debug_reg(),
    ",
  0x5004010cu64 => "
      GPREG.gp_status_reg(),
    ",
  0x50040104u64 => "
      GPREG.reset_freeze_reg(),
    ",
  0x50040100u64 => "
      GPREG.set_freeze_reg(),
    ",
  0x50040118u64 => "
      GPREG.usbpad_reg(),
    ",
  0x510012c4u64 => "
      GPU_CORE.d2_cachectl(),
    ",
  0x510012e8u64 => "
      GPU_CORE.d2_colkey(),
    ",
  0x51001264u64 => "
      GPU_CORE.d2_color1(),
    ",
  0x51001268u64 => "
      GPU_CORE.d2_color2(),
    ",
  0x51001200u64 => "
      GPU_CORE.d2_control(),
    ",
  0x51001204u64 => "
      GPU_CORE.d2_control2(),
    ",
  0x51001208u64 => "
      GPU_CORE.d2_control3(),
    ",
  0x510012c8u64 => "
      GPU_CORE.d2_dliststart(),
    ",
  0x510012f0u64 => "
      GPU_CORE.d2_hwrevision(),
    ",
  0x510012c0u64 => "
      GPU_CORE.d2_irqctl(),
    ",
  0x51001258u64 => "
      GPU_CORE.d2_l1band(),
    ",
  0x51001210u64 => "
      GPU_CORE.d2_l1start(),
    ",
  0x51001228u64 => "
      GPU_CORE.d2_l1xadd(),
    ",
  0x51001240u64 => "
      GPU_CORE.d2_l1yadd(),
    ",
  0x5100125cu64 => "
      GPU_CORE.d2_l2band(),
    ",
  0x51001214u64 => "
      GPU_CORE.d2_l2start(),
    ",
  0x5100122cu64 => "
      GPU_CORE.d2_l2xadd(),
    ",
  0x51001244u64 => "
      GPU_CORE.d2_l2yadd(),
    ",
  0x51001218u64 => "
      GPU_CORE.d2_l3start(),
    ",
  0x51001230u64 => "
      GPU_CORE.d2_l3xadd(),
    ",
  0x51001248u64 => "
      GPU_CORE.d2_l3yadd(),
    ",
  0x5100121cu64 => "
      GPU_CORE.d2_l4start(),
    ",
  0x51001234u64 => "
      GPU_CORE.d2_l4xadd(),
    ",
  0x5100124cu64 => "
      GPU_CORE.d2_l4yadd(),
    ",
  0x51001220u64 => "
      GPU_CORE.d2_l5start(),
    ",
  0x51001238u64 => "
      GPU_CORE.d2_l5xadd(),
    ",
  0x51001250u64 => "
      GPU_CORE.d2_l5yadd(),
    ",
  0x51001224u64 => "
      GPU_CORE.d2_l6start(),
    ",
  0x5100123cu64 => "
      GPU_CORE.d2_l6xadd(),
    ",
  0x51001254u64 => "
      GPU_CORE.d2_l6yadd(),
    ",
  0x51001290u64 => "
      GPU_CORE.d2_lustart(),
    ",
  0x51001294u64 => "
      GPU_CORE.d2_luxadd(),
    ",
  0x51001298u64 => "
      GPU_CORE.d2_luyadd(),
    ",
  0x510012a0u64 => "
      GPU_CORE.d2_lvstartf(),
    ",
  0x5100129cu64 => "
      GPU_CORE.d2_lvstarti(),
    ",
  0x510012a4u64 => "
      GPU_CORE.d2_lvxaddi(),
    ",
  0x510012a8u64 => "
      GPU_CORE.d2_lvyaddi(),
    ",
  0x510012acu64 => "
      GPU_CORE.d2_lvyxaddf(),
    ",
  0x51001280u64 => "
      GPU_CORE.d2_origin(),
    ",
  0x51001274u64 => "
      GPU_CORE.d2_pattern(),
    ",
  0x510012ccu64 => "
      GPU_CORE.d2_perfcount1(),
    ",
  0x510012d0u64 => "
      GPU_CORE.d2_perfcount2(),
    ",
  0x510012d4u64 => "
      GPU_CORE.d2_perftrigger(),
    ",
  0x5100127cu64 => "
      GPU_CORE.d2_pitch(),
    ",
  0x51001278u64 => "
      GPU_CORE.d2_size(),
    ",
  0x510012f4u64 => "
      GPU_CORE.d2_status(),
    ",
  0x510012d8u64 => "
      GPU_CORE.d2_texclut(),
    ",
  0x510012dcu64 => "
      GPU_CORE.d2_texclut_addr(),
    ",
  0x510012e0u64 => "
      GPU_CORE.d2_texclut_data(),
    ",
  0x510012e4u64 => "
      GPU_CORE.d2_texclut_offset(),
    ",
  0x510012b8u64 => "
      GPU_CORE.d2_texmask(),
    ",
  0x510012bcu64 => "
      GPU_CORE.d2_texorigin(),
    ",
  0x510012b4u64 => "
      GPU_CORE.d2_texpitch(),
    ",
  0x51001100u64 => "
      GPU_REG.gpu_ctrl_reg(),
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
  0x50020598u64 => "
      I_2_C_3.i2c3_ack_general_call_reg(),
    ",
  0x5002055cu64 => "
      I_2_C_3.i2c3_clr_activity_reg(),
    ",
  0x50020568u64 => "
      I_2_C_3.i2c3_clr_gen_call_reg(),
    ",
  0x50020540u64 => "
      I_2_C_3.i2c3_clr_intr_reg(),
    ",
  0x50020550u64 => "
      I_2_C_3.i2c3_clr_rd_req_reg(),
    ",
  0x50020558u64 => "
      I_2_C_3.i2c3_clr_rx_done_reg(),
    ",
  0x50020548u64 => "
      I_2_C_3.i2c3_clr_rx_over_reg(),
    ",
  0x50020544u64 => "
      I_2_C_3.i2c3_clr_rx_under_reg(),
    ",
  0x50020564u64 => "
      I_2_C_3.i2c3_clr_start_det_reg(),
    ",
  0x50020560u64 => "
      I_2_C_3.i2c3_clr_stop_det_reg(),
    ",
  0x50020554u64 => "
      I_2_C_3.i2c3_clr_tx_abrt_reg(),
    ",
  0x5002054cu64 => "
      I_2_C_3.i2c3_clr_tx_over_reg(),
    ",
  0x50020500u64 => "
      I_2_C_3.i2c3_con_reg(),
    ",
  0x50020510u64 => "
      I_2_C_3.i2c3_data_cmd_reg(),
    ",
  0x50020588u64 => "
      I_2_C_3.i2c3_dma_cr_reg(),
    ",
  0x50020590u64 => "
      I_2_C_3.i2c3_dma_rdlr_reg(),
    ",
  0x5002058cu64 => "
      I_2_C_3.i2c3_dma_tdlr_reg(),
    ",
  0x5002056cu64 => "
      I_2_C_3.i2c3_enable_reg(),
    ",
  0x5002059cu64 => "
      I_2_C_3.i2c3_enable_status_reg(),
    ",
  0x5002051cu64 => "
      I_2_C_3.i2c3_fs_scl_hcnt_reg(),
    ",
  0x50020520u64 => "
      I_2_C_3.i2c3_fs_scl_lcnt_reg(),
    ",
  0x5002050cu64 => "
      I_2_C_3.i2c3_hs_maddr_reg(),
    ",
  0x50020524u64 => "
      I_2_C_3.i2c3_hs_scl_hcnt_reg(),
    ",
  0x50020528u64 => "
      I_2_C_3.i2c3_hs_scl_lcnt_reg(),
    ",
  0x500205a0u64 => "
      I_2_C_3.i2c3_ic_fs_spklen_reg(),
    ",
  0x500205a4u64 => "
      I_2_C_3.i2c3_ic_hs_spklen_reg(),
    ",
  0x50020530u64 => "
      I_2_C_3.i2c3_intr_mask_reg(),
    ",
  0x5002052cu64 => "
      I_2_C_3.i2c3_intr_stat_reg(),
    ",
  0x50020534u64 => "
      I_2_C_3.i2c3_raw_intr_stat_reg(),
    ",
  0x50020578u64 => "
      I_2_C_3.i2c3_rxflr_reg(),
    ",
  0x50020538u64 => "
      I_2_C_3.i2c3_rx_tl_reg(),
    ",
  0x50020508u64 => "
      I_2_C_3.i2c3_sar_reg(),
    ",
  0x5002057cu64 => "
      I_2_C_3.i2c3_sda_hold_reg(),
    ",
  0x50020594u64 => "
      I_2_C_3.i2c3_sda_setup_reg(),
    ",
  0x50020514u64 => "
      I_2_C_3.i2c3_ss_scl_hcnt_reg(),
    ",
  0x50020518u64 => "
      I_2_C_3.i2c3_ss_scl_lcnt_reg(),
    ",
  0x50020570u64 => "
      I_2_C_3.i2c3_status_reg(),
    ",
  0x50020504u64 => "
      I_2_C_3.i2c3_tar_reg(),
    ",
  0x50020574u64 => "
      I_2_C_3.i2c3_txflr_reg(),
    ",
  0x50020580u64 => "
      I_2_C_3.i2c3_tx_abrt_source_reg(),
    ",
  0x5002053cu64 => "
      I_2_C_3.i2c3_tx_tl_reg(),
    ",
  0x50020cd4u64 => "
      I_3_C.i3c_bus_free_avail_timing_reg(),
    ",
  0x50020c0cu64 => "
      I_3_C.i3c_command_queue_port_reg(),
    ",
  0x50020c50u64 => "
      I_3_C.i3c_data_buffer_stat_level_reg(),
    ",
  0x50020c20u64 => "
      I_3_C.i3c_data_buffer_thld_ctrl_reg(),
    ",
  0x50020c04u64 => "
      I_3_C.i3c_device_addr_reg(),
    ",
  0x50020c5cu64 => "
      I_3_C.i3c_device_addr_table_ptr_reg(),
    ",
  0x50020cb0u64 => "
      I_3_C.i3c_device_ctrl_extended_reg(),
    ",
  0x50020c00u64 => "
      I_3_C.i3c_device_ctrl_reg(),
    ",
  0x50020e80u64 => "
      I_3_C.i3c_dev_addr_table_loc1_reg(),
    ",
  0x50020e84u64 => "
      I_3_C.i3c_dev_addr_table_loc2_reg(),
    ",
  0x50020e88u64 => "
      I_3_C.i3c_dev_addr_table_loc3_reg(),
    ",
  0x50020e8cu64 => "
      I_3_C.i3c_dev_addr_table_loc4_reg(),
    ",
  0x50020e90u64 => "
      I_3_C.i3c_dev_addr_table_loc5_reg(),
    ",
  0x50020e94u64 => "
      I_3_C.i3c_dev_addr_table_loc6_reg(),
    ",
  0x50020e98u64 => "
      I_3_C.i3c_dev_addr_table_loc7_reg(),
    ",
  0x50020e9cu64 => "
      I_3_C.i3c_dev_addr_table_loc8_reg(),
    ",
  0x50020e00u64 => "
      I_3_C.i3c_dev_char_table1_loc1_reg(),
    ",
  0x50020e04u64 => "
      I_3_C.i3c_dev_char_table1_loc2_reg(),
    ",
  0x50020e08u64 => "
      I_3_C.i3c_dev_char_table1_loc3_reg(),
    ",
  0x50020e0cu64 => "
      I_3_C.i3c_dev_char_table1_loc4_reg(),
    ",
  0x50020e10u64 => "
      I_3_C.i3c_dev_char_table2_loc1_reg(),
    ",
  0x50020e14u64 => "
      I_3_C.i3c_dev_char_table2_loc2_reg(),
    ",
  0x50020e18u64 => "
      I_3_C.i3c_dev_char_table2_loc3_reg(),
    ",
  0x50020e1cu64 => "
      I_3_C.i3c_dev_char_table2_loc4_reg(),
    ",
  0x50020e20u64 => "
      I_3_C.i3c_dev_char_table3_loc1_reg(),
    ",
  0x50020e24u64 => "
      I_3_C.i3c_dev_char_table3_loc2_reg(),
    ",
  0x50020e28u64 => "
      I_3_C.i3c_dev_char_table3_loc3_reg(),
    ",
  0x50020e2cu64 => "
      I_3_C.i3c_dev_char_table3_loc4_reg(),
    ",
  0x50020e30u64 => "
      I_3_C.i3c_dev_char_table4_loc1_reg(),
    ",
  0x50020e34u64 => "
      I_3_C.i3c_dev_char_table4_loc2_reg(),
    ",
  0x50020e38u64 => "
      I_3_C.i3c_dev_char_table4_loc3_reg(),
    ",
  0x50020e3cu64 => "
      I_3_C.i3c_dev_char_table4_loc4_reg(),
    ",
  0x50020e40u64 => "
      I_3_C.i3c_dev_char_table5_loc1_reg(),
    ",
  0x50020e44u64 => "
      I_3_C.i3c_dev_char_table5_loc2_reg(),
    ",
  0x50020e48u64 => "
      I_3_C.i3c_dev_char_table5_loc3_reg(),
    ",
  0x50020e4cu64 => "
      I_3_C.i3c_dev_char_table5_loc4_reg(),
    ",
  0x50020e50u64 => "
      I_3_C.i3c_dev_char_table6_loc1_reg(),
    ",
  0x50020e54u64 => "
      I_3_C.i3c_dev_char_table6_loc2_reg(),
    ",
  0x50020e58u64 => "
      I_3_C.i3c_dev_char_table6_loc3_reg(),
    ",
  0x50020e5cu64 => "
      I_3_C.i3c_dev_char_table6_loc4_reg(),
    ",
  0x50020e60u64 => "
      I_3_C.i3c_dev_char_table7_loc1_reg(),
    ",
  0x50020e64u64 => "
      I_3_C.i3c_dev_char_table7_loc2_reg(),
    ",
  0x50020e68u64 => "
      I_3_C.i3c_dev_char_table7_loc3_reg(),
    ",
  0x50020e6cu64 => "
      I_3_C.i3c_dev_char_table7_loc4_reg(),
    ",
  0x50020e70u64 => "
      I_3_C.i3c_dev_char_table8_loc1_reg(),
    ",
  0x50020e74u64 => "
      I_3_C.i3c_dev_char_table8_loc2_reg(),
    ",
  0x50020e78u64 => "
      I_3_C.i3c_dev_char_table8_loc3_reg(),
    ",
  0x50020e7cu64 => "
      I_3_C.i3c_dev_char_table8_loc4_reg(),
    ",
  0x50020c60u64 => "
      I_3_C.i3c_dev_char_table_pointer_reg(),
    ",
  0x50020c08u64 => "
      I_3_C.i3c_hw_capability_reg(),
    ",
  0x50020c24u64 => "
      I_3_C.i3c_ibi_queue_ctrl_reg(),
    ",
  0x50020c18u64 => "
      I_3_C.i3c_ibi_queue_status_data_reg(),
    ",
  0x50020c30u64 => "
      I_3_C.i3c_ibi_sir_req_reject_reg(),
    ",
  0x50020c48u64 => "
      I_3_C.i3c_intr_force_reg(),
    ",
  0x50020c44u64 => "
      I_3_C.i3c_intr_signal_en_reg(),
    ",
  0x50020c40u64 => "
      I_3_C.i3c_intr_status_en_reg(),
    ",
  0x50020c3cu64 => "
      I_3_C.i3c_intr_status_reg(),
    ",
  0x50020c54u64 => "
      I_3_C.i3c_present_state_reg(),
    ",
  0x50020ce8u64 => "
      I_3_C.i3c_queue_size_capability_reg(),
    ",
  0x50020c4cu64 => "
      I_3_C.i3c_queue_status_level_reg(),
    ",
  0x50020c1cu64 => "
      I_3_C.i3c_queue_thld_ctrl_reg(),
    ",
  0x50020c34u64 => "
      I_3_C.i3c_reset_ctrl_reg(),
    ",
  0x50020c10u64 => "
      I_3_C.i3c_response_queue_port_reg(),
    ",
  0x50020c14u64 => "
      I_3_C.i3c_rx_tx_data_port_reg(),
    ",
  0x50020cc8u64 => "
      I_3_C.i3c_scl_ext_lcnt_timing_reg(),
    ",
  0x50020cccu64 => "
      I_3_C.i3c_scl_ext_termn_lcnt_time_reg(),
    ",
  0x50020cc0u64 => "
      I_3_C.i3c_scl_i2c_fmp_timing_reg(),
    ",
  0x50020cbcu64 => "
      I_3_C.i3c_scl_i2c_fm_timing_reg(),
    ",
  0x50020cb4u64 => "
      I_3_C.i3c_scl_i3c_od_timing_reg(),
    ",
  0x50020cb8u64 => "
      I_3_C.i3c_scl_i3c_pp_timing_reg(),
    ",
  0x50020cd0u64 => "
      I_3_C.i3c_sda_hold_dly_timing_reg(),
    ",
  0x50020c38u64 => "
      I_3_C.i3c_slv_event_status_reg(),
    ",
  0x50020c6cu64 => "
      I_3_C.i3c_vendor_specific_reg_ptr_reg(),
    ",
  0x50020ce0u64 => "
      I_3_C.i3c_ver_id_reg(),
    ",
  0x50020ce4u64 => "
      I_3_C.i3c_ver_type_reg(),
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
  0x300301a8u64 => "
      LCDC.lcdc_clkctrl_cg_reg(),
    ",
  0x30030004u64 => "
      LCDC.lcdc_clkctrl_reg(),
    ",
  0x30030100u64 => "
      LCDC.lcdc_colmod(),
    ",
  0x300300f0u64 => "
      LCDC.lcdc_conf_reg(),
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
  0x300300ecu64 => "
      LCDC.lcdc_dbib_rdat(),
    ",
  0x300301a4u64 => "
      LCDC.lcdc_fmtctrl_2_reg(),
    ",
  0x300301a0u64 => "
      LCDC.lcdc_fmtctrl_reg(),
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
  0x3003003cu64 => "
      LCDC.lcdc_layer0_baseaddr_reg(),
    ",
  0x30030030u64 => "
      LCDC.lcdc_layer0_mode_reg(),
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
  0x3003005cu64 => "
      LCDC.lcdc_layer1_baseaddr_reg(),
    ",
  0x30030050u64 => "
      LCDC.lcdc_layer1_mode_reg(),
    ",
  0x30030064u64 => "
      LCDC.lcdc_layer1_resxy_reg(),
    ",
  0x30030058u64 => "
      LCDC.lcdc_layer1_sizexy_reg(),
    ",
  0x30030054u64 => "
      LCDC.lcdc_layer1_startxy_reg(),
    ",
  0x30030060u64 => "
      LCDC.lcdc_layer1_stride_reg(),
    ",
  0x30030000u64 => "
      LCDC.lcdc_mode_reg(),
    ",
  0x300307fcu64 => "
      LCDC.lcdc_palette_255(),
    ",
  0x30030400u64 => "
      LCDC.lcdc_palette_base(),
    ",
  0x3003000cu64 => "
      LCDC.lcdc_resxy_reg(),
    ",
  0x30030024u64 => "
      LCDC.lcdc_startxy_reg(),
    ",
  0x300300fcu64 => "
      LCDC.lcdc_status_reg(),
    ",
  0x50050078u64 => "
      MEMCTRL.busy_reset_reg(),
    ",
  0x5005007cu64 => "
      MEMCTRL.busy_reset_reg2(),
    ",
  0x50050070u64 => "
      MEMCTRL.busy_set_reg(),
    ",
  0x50050074u64 => "
      MEMCTRL.busy_set_reg2(),
    ",
  0x50050080u64 => "
      MEMCTRL.busy_stat_reg(),
    ",
  0x50050084u64 => "
      MEMCTRL.busy_stat_reg2(),
    ",
  0x50050030u64 => "
      MEMCTRL.cmac_status_reg(),
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
      MEMCTRL.mem_prio_arb1_4_reg(),
    ",
  0x50050008u64 => "
      MEMCTRL.mem_prio_arb5_8_reg(),
    ",
  0x50050010u64 => "
      MEMCTRL.mem_stall_reg(),
    ",
  0x50050018u64 => "
      MEMCTRL.mem_status2_reg(),
    ",
  0x50050014u64 => "
      MEMCTRL.mem_status_reg(),
    ",
  0x36000034u64 => "
      OQSPIF.oqspif_burstbrk_reg(),
    ",
  0x3600000cu64 => "
      OQSPIF.oqspif_burstcmda_reg(),
    ",
  0x36000010u64 => "
      OQSPIF.oqspif_burstcmdb_reg(),
    ",
  0x3600003cu64 => "
      OQSPIF.oqspif_chckerase_reg(),
    ",
  0x36000000u64 => "
      OQSPIF.oqspif_ctrlbus_reg(),
    ",
  0x36000004u64 => "
      OQSPIF.oqspif_ctrlmode_reg(),
    ",
  0x36000100u64 => "
      OQSPIF.oqspif_ctr_ctrl_reg(),
    ",
  0x36000108u64 => "
      OQSPIF.oqspif_ctr_eaddr_reg(),
    ",
  0x36000114u64 => "
      OQSPIF.oqspif_ctr_key_0_3_reg(),
    ",
  0x36000120u64 => "
      OQSPIF.oqspif_ctr_key_12_15_reg(),
    ",
  0x36000124u64 => "
      OQSPIF.oqspif_ctr_key_16_19_reg(),
    ",
  0x36000128u64 => "
      OQSPIF.oqspif_ctr_key_20_23_reg(),
    ",
  0x3600012cu64 => "
      OQSPIF.oqspif_ctr_key_24_27_reg(),
    ",
  0x36000130u64 => "
      OQSPIF.oqspif_ctr_key_28_31_reg(),
    ",
  0x36000118u64 => "
      OQSPIF.oqspif_ctr_key_4_7_reg(),
    ",
  0x3600011cu64 => "
      OQSPIF.oqspif_ctr_key_8_11_reg(),
    ",
  0x3600010cu64 => "
      OQSPIF.oqspif_ctr_nonce_0_3_reg(),
    ",
  0x36000110u64 => "
      OQSPIF.oqspif_ctr_nonce_4_7_reg(),
    ",
  0x36000104u64 => "
      OQSPIF.oqspif_ctr_saddr_reg(),
    ",
  0x36000020u64 => "
      OQSPIF.oqspif_dummydata_reg(),
    ",
  0x36000028u64 => "
      OQSPIF.oqspif_erasecmda_reg(),
    ",
  0x3600002cu64 => "
      OQSPIF.oqspif_erasecmdb_reg(),
    ",
  0x36000030u64 => "
      OQSPIF.oqspif_erasecmdc_reg(),
    ",
  0x36000024u64 => "
      OQSPIF.oqspif_erasectrl_reg(),
    ",
  0x36000040u64 => "
      OQSPIF.oqspif_gp_reg(),
    ",
  0x3600001cu64 => "
      OQSPIF.oqspif_readdata_reg(),
    ",
  0x36000008u64 => "
      OQSPIF.oqspif_recvdata_reg(),
    ",
  0x36000038u64 => "
      OQSPIF.oqspif_statuscmd_reg(),
    ",
  0x36000014u64 => "
      OQSPIF.oqspif_status_reg(),
    ",
  0x36000018u64 => "
      OQSPIF.oqspif_writedata_reg(),
    ",
};
