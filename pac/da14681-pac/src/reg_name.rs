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
  0x40020018u64 => "
      AES_HASH.crypto_clrirq_reg(),
    ",
  0x40020000u64 => "
      AES_HASH.crypto_ctrl_reg(),
    ",
  0x40020010u64 => "
      AES_HASH.crypto_dest_addr_reg(),
    ",
  0x40020008u64 => "
      AES_HASH.crypto_fetch_addr_reg(),
    ",
  0x40020100u64 => "
      AES_HASH.crypto_keys_start(),
    ",
  0x4002000cu64 => "
      AES_HASH.crypto_len_reg(),
    ",
  0x4002001cu64 => "
      AES_HASH.crypto_mreg0_reg(),
    ",
  0x40020020u64 => "
      AES_HASH.crypto_mreg1_reg(),
    ",
  0x40020024u64 => "
      AES_HASH.crypto_mreg2_reg(),
    ",
  0x40020028u64 => "
      AES_HASH.crypto_mreg3_reg(),
    ",
  0x40020004u64 => "
      AES_HASH.crypto_start_reg(),
    ",
  0x40020014u64 => "
      AES_HASH.crypto_status_reg(),
    ",
  0x50001b08u64 => "
      ANAMISC.charger_ctrl1_reg(),
    ",
  0x50001b0au64 => "
      ANAMISC.charger_ctrl2_reg(),
    ",
  0x50001b0cu64 => "
      ANAMISC.charger_status_reg(),
    ",
  0x50001b62u64 => "
      ANAMISC.clk_ref_cnt_reg(),
    ",
  0x50001b60u64 => "
      ANAMISC.clk_ref_sel_reg(),
    ",
  0x50001b66u64 => "
      ANAMISC.clk_ref_val_h_reg(),
    ",
  0x50001b64u64 => "
      ANAMISC.clk_ref_val_l_reg(),
    ",
  0x50001b46u64 => "
      ANAMISC.soc_add2ch_reg(),
    ",
  0x50001b50u64 => "
      ANAMISC.soc_charge_avg_reg(),
    ",
  0x50001b48u64 => "
      ANAMISC.soc_charge_cntr1_reg(),
    ",
  0x50001b4au64 => "
      ANAMISC.soc_charge_cntr2_reg(),
    ",
  0x50001b4cu64 => "
      ANAMISC.soc_charge_cntr3_reg(),
    ",
  0x50001b40u64 => "
      ANAMISC.soc_ctrl1_reg(),
    ",
  0x50001b42u64 => "
      ANAMISC.soc_ctrl2_reg(),
    ",
  0x50001b44u64 => "
      ANAMISC.soc_ctrl3_reg(),
    ",
  0x50001b54u64 => "
      ANAMISC.soc_ext_in_reg(),
    ",
  0x50001b56u64 => "
      ANAMISC.soc_ext_out_reg(),
    ",
  0x50001b52u64 => "
      ANAMISC.soc_status_reg(),
    ",
  0x5000401cu64 => "
      APU.apu_mux_reg(),
    ",
  0x50004034u64 => "
      APU.coef0a_set1_reg(),
    ",
  0x50004020u64 => "
      APU.coef10_set1_reg(),
    ",
  0x50004024u64 => "
      APU.coef32_set1_reg(),
    ",
  0x50004028u64 => "
      APU.coef54_set1_reg(),
    ",
  0x5000402cu64 => "
      APU.coef76_set1_reg(),
    ",
  0x50004030u64 => "
      APU.coef98_set1_reg(),
    ",
  0x50004100u64 => "
      APU.pcm1_ctrl_reg(),
    ",
  0x50004104u64 => "
      APU.pcm1_in1_reg(),
    ",
  0x50004108u64 => "
      APU.pcm1_in2_reg(),
    ",
  0x5000410cu64 => "
      APU.pcm1_out1_reg(),
    ",
  0x50004110u64 => "
      APU.pcm1_out2_reg(),
    ",
  0x50004000u64 => "
      APU.src1_ctrl_reg(),
    ",
  0x5000400cu64 => "
      APU.src1_in1_reg(),
    ",
  0x50004010u64 => "
      APU.src1_in2_reg(),
    ",
  0x50004004u64 => "
      APU.src1_in_fs_reg(),
    ",
  0x50004014u64 => "
      APU.src1_out1_reg(),
    ",
  0x50004018u64 => "
      APU.src1_out2_reg(),
    ",
  0x50004008u64 => "
      APU.src1_out_fs_reg(),
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
  0x40000110u64 => "
      BLE.ble_bleprioscharb_reg(),
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
  0x400c3008u64 => "
      CACHE.cache_assoccfg_reg(),
    ",
  0x400c3000u64 => "
      CACHE.cache_ctrl1_reg(),
    ",
  0x400c3020u64 => "
      CACHE.cache_ctrl2_reg(),
    ",
  0x400c3004u64 => "
      CACHE.cache_lnsizecfg_reg(),
    ",
  0x400c3030u64 => "
      CACHE.cache_mrm_ctrl_reg(),
    ",
  0x400c3028u64 => "
      CACHE.cache_mrm_hits_reg(),
    ",
  0x400c302cu64 => "
      CACHE.cache_mrm_misses_reg(),
    ",
  0x400c3038u64 => "
      CACHE.cache_mrm_thres_reg(),
    ",
  0x400c3034u64 => "
      CACHE.cache_mrm_tint_reg(),
    ",
  0x400c3050u64 => "
      CACHE.swd_reset_reg(),
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
  0x50002f08u64 => "
      COEX.coex_ble_pti_reg(),
    ",
  0x50002f00u64 => "
      COEX.coex_ctrl_reg(),
    ",
  0x50002f0cu64 => "
      COEX.coex_diag_reg(),
    ",
  0x50002f0au64 => "
      COEX.coex_ftdf_pti_reg(),
    ",
  0x50002f04u64 => "
      COEX.coex_int_mask_reg(),
    ",
  0x50002f06u64 => "
      COEX.coex_int_stat_reg(),
    ",
  0x50002f24u64 => "
      COEX.coex_pri10_reg(),
    ",
  0x50002f26u64 => "
      COEX.coex_pri11_reg(),
    ",
  0x50002f28u64 => "
      COEX.coex_pri12_reg(),
    ",
  0x50002f2au64 => "
      COEX.coex_pri13_reg(),
    ",
  0x50002f2cu64 => "
      COEX.coex_pri14_reg(),
    ",
  0x50002f2eu64 => "
      COEX.coex_pri15_reg(),
    ",
  0x50002f30u64 => "
      COEX.coex_pri16_reg(),
    ",
  0x50002f32u64 => "
      COEX.coex_pri17_reg(),
    ",
  0x50002f12u64 => "
      COEX.coex_pri1_reg(),
    ",
  0x50002f14u64 => "
      COEX.coex_pri2_reg(),
    ",
  0x50002f16u64 => "
      COEX.coex_pri3_reg(),
    ",
  0x50002f18u64 => "
      COEX.coex_pri4_reg(),
    ",
  0x50002f1au64 => "
      COEX.coex_pri5_reg(),
    ",
  0x50002f1cu64 => "
      COEX.coex_pri6_reg(),
    ",
  0x50002f1eu64 => "
      COEX.coex_pri7_reg(),
    ",
  0x50002f20u64 => "
      COEX.coex_pri8_reg(),
    ",
  0x50002f22u64 => "
      COEX.coex_pri9_reg(),
    ",
  0x50002f02u64 => "
      COEX.coex_stat_reg(),
    ",
  0x50001c04u64 => "
      CRG_PER.clk_per_reg(),
    ",
  0x50001c40u64 => "
      CRG_PER.pcm_div_reg(),
    ",
  0x50001c42u64 => "
      CRG_PER.pcm_fdiv_reg(),
    ",
  0x50001c44u64 => "
      CRG_PER.pdm_div_reg(),
    ",
  0x50001c46u64 => "
      CRG_PER.src_div_reg(),
    ",
  0x50001c4au64 => "
      CRG_PER.usbpad_reg(),
    ",
  0x5000002au64 => "
      CRG_TOP.ana_status_reg(),
    ",
  0x50000028u64 => "
      CRG_TOP.bandgap_reg(),
    ",
  0x50000036u64 => "
      CRG_TOP.bod_ctrl2_reg(),
    ",
  0x50000034u64 => "
      CRG_TOP.bod_ctrl_reg(),
    ",
  0x50000038u64 => "
      CRG_TOP.bod_status_reg(),
    ",
  0x50000022u64 => "
      CRG_TOP.clk_16m_reg(),
    ",
  0x50000020u64 => "
      CRG_TOP.clk_32k_reg(),
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
  0x50000008u64 => "
      CRG_TOP.clk_radio_reg(),
    ",
  0x50000024u64 => "
      CRG_TOP.clk_rcx20k_reg(),
    ",
  0x5000000cu64 => "
      CRG_TOP.clk_tmr_reg(),
    ",
  0x5000003au64 => "
      CRG_TOP.ldo_ctrl1_reg(),
    ",
  0x5000003cu64 => "
      CRG_TOP.ldo_ctrl2_reg(),
    ",
  0x50000010u64 => "
      CRG_TOP.pmu_ctrl_reg(),
    ",
  0x5000003eu64 => "
      CRG_TOP.sleep_timer_reg(),
    ",
  0x50000012u64 => "
      CRG_TOP.sys_ctrl_reg(),
    ",
  0x50000014u64 => "
      CRG_TOP.sys_stat_reg(),
    ",
  0x50000032u64 => "
      CRG_TOP.vbus_irq_clear_reg(),
    ",
  0x50000030u64 => "
      CRG_TOP.vbus_irq_mask_reg(),
    ",
  0x50000050u64 => "
      CRG_TOP.xtalrdy_ctrl_reg(),
    ",
  0x50000082u64 => "
      DCDC.dcdc_ctrl_0_reg(),
    ",
  0x50000084u64 => "
      DCDC.dcdc_ctrl_1_reg(),
    ",
  0x50000086u64 => "
      DCDC.dcdc_ctrl_2_reg(),
    ",
  0x500000b6u64 => "
      DCDC.dcdc_irq_clear_reg(),
    ",
  0x500000b8u64 => "
      DCDC.dcdc_irq_mask_reg(),
    ",
  0x500000b4u64 => "
      DCDC.dcdc_irq_status_reg(),
    ",
  0x50000098u64 => "
      DCDC.dcdc_ret_0_reg(),
    ",
  0x5000009au64 => "
      DCDC.dcdc_ret_1_reg(),
    ",
  0x500000a2u64 => "
      DCDC.dcdc_status_0_reg(),
    ",
  0x500000a4u64 => "
      DCDC.dcdc_status_1_reg(),
    ",
  0x500000a6u64 => "
      DCDC.dcdc_status_2_reg(),
    ",
  0x500000a8u64 => "
      DCDC.dcdc_status_3_reg(),
    ",
  0x500000aau64 => "
      DCDC.dcdc_status_4_reg(),
    ",
  0x5000009eu64 => "
      DCDC.dcdc_test_0_reg(),
    ",
  0x500000a0u64 => "
      DCDC.dcdc_test_1_reg(),
    ",
  0x500000acu64 => "
      DCDC.dcdc_trim_0_reg(),
    ",
  0x500000aeu64 => "
      DCDC.dcdc_trim_1_reg(),
    ",
  0x500000b0u64 => "
      DCDC.dcdc_trim_2_reg(),
    ",
  0x500000b2u64 => "
      DCDC.dcdc_trim_3_reg(),
    ",
  0x5000009cu64 => "
      DCDC.dcdc_trim_reg(),
    ",
  0x50000088u64 => "
      DCDC.dcdc_v14_0_reg(),
    ",
  0x5000008au64 => "
      DCDC.dcdc_v14_1_reg(),
    ",
  0x50000094u64 => "
      DCDC.dcdc_v18p_0_reg(),
    ",
  0x50000096u64 => "
      DCDC.dcdc_v18p_1_reg(),
    ",
  0x5000008cu64 => "
      DCDC.dcdc_v18_0_reg(),
    ",
  0x5000008eu64 => "
      DCDC.dcdc_v18_1_reg(),
    ",
  0x50000090u64 => "
      DCDC.dcdc_vdd_0_reg(),
    ",
  0x50000092u64 => "
      DCDC.dcdc_vdd_1_reg(),
    ",
  0x50003502u64 => "
      DMA.dma0_a_starth_reg(),
    ",
  0x50003500u64 => "
      DMA.dma0_a_startl_reg(),
    ",
  0x50003506u64 => "
      DMA.dma0_b_starth_reg(),
    ",
  0x50003504u64 => "
      DMA.dma0_b_startl_reg(),
    ",
  0x5000350cu64 => "
      DMA.dma0_ctrl_reg(),
    ",
  0x5000350eu64 => "
      DMA.dma0_idx_reg(),
    ",
  0x50003508u64 => "
      DMA.dma0_int_reg(),
    ",
  0x5000350au64 => "
      DMA.dma0_len_reg(),
    ",
  0x50003512u64 => "
      DMA.dma1_a_starth_reg(),
    ",
  0x50003510u64 => "
      DMA.dma1_a_startl_reg(),
    ",
  0x50003516u64 => "
      DMA.dma1_b_starth_reg(),
    ",
  0x50003514u64 => "
      DMA.dma1_b_startl_reg(),
    ",
  0x5000351cu64 => "
      DMA.dma1_ctrl_reg(),
    ",
  0x5000351eu64 => "
      DMA.dma1_idx_reg(),
    ",
  0x50003518u64 => "
      DMA.dma1_int_reg(),
    ",
  0x5000351au64 => "
      DMA.dma1_len_reg(),
    ",
  0x50003522u64 => "
      DMA.dma2_a_starth_reg(),
    ",
  0x50003520u64 => "
      DMA.dma2_a_startl_reg(),
    ",
  0x50003526u64 => "
      DMA.dma2_b_starth_reg(),
    ",
  0x50003524u64 => "
      DMA.dma2_b_startl_reg(),
    ",
  0x5000352cu64 => "
      DMA.dma2_ctrl_reg(),
    ",
  0x5000352eu64 => "
      DMA.dma2_idx_reg(),
    ",
  0x50003528u64 => "
      DMA.dma2_int_reg(),
    ",
  0x5000352au64 => "
      DMA.dma2_len_reg(),
    ",
  0x50003532u64 => "
      DMA.dma3_a_starth_reg(),
    ",
  0x50003530u64 => "
      DMA.dma3_a_startl_reg(),
    ",
  0x50003536u64 => "
      DMA.dma3_b_starth_reg(),
    ",
  0x50003534u64 => "
      DMA.dma3_b_startl_reg(),
    ",
  0x5000353cu64 => "
      DMA.dma3_ctrl_reg(),
    ",
  0x5000353eu64 => "
      DMA.dma3_idx_reg(),
    ",
  0x50003538u64 => "
      DMA.dma3_int_reg(),
    ",
  0x5000353au64 => "
      DMA.dma3_len_reg(),
    ",
  0x50003542u64 => "
      DMA.dma4_a_starth_reg(),
    ",
  0x50003540u64 => "
      DMA.dma4_a_startl_reg(),
    ",
  0x50003546u64 => "
      DMA.dma4_b_starth_reg(),
    ",
  0x50003544u64 => "
      DMA.dma4_b_startl_reg(),
    ",
  0x5000354cu64 => "
      DMA.dma4_ctrl_reg(),
    ",
  0x5000354eu64 => "
      DMA.dma4_idx_reg(),
    ",
  0x50003548u64 => "
      DMA.dma4_int_reg(),
    ",
  0x5000354au64 => "
      DMA.dma4_len_reg(),
    ",
  0x50003552u64 => "
      DMA.dma5_a_starth_reg(),
    ",
  0x50003550u64 => "
      DMA.dma5_a_startl_reg(),
    ",
  0x50003556u64 => "
      DMA.dma5_b_starth_reg(),
    ",
  0x50003554u64 => "
      DMA.dma5_b_startl_reg(),
    ",
  0x5000355cu64 => "
      DMA.dma5_ctrl_reg(),
    ",
  0x5000355eu64 => "
      DMA.dma5_idx_reg(),
    ",
  0x50003558u64 => "
      DMA.dma5_int_reg(),
    ",
  0x5000355au64 => "
      DMA.dma5_len_reg(),
    ",
  0x50003562u64 => "
      DMA.dma6_a_starth_reg(),
    ",
  0x50003560u64 => "
      DMA.dma6_a_startl_reg(),
    ",
  0x50003566u64 => "
      DMA.dma6_b_starth_reg(),
    ",
  0x50003564u64 => "
      DMA.dma6_b_startl_reg(),
    ",
  0x5000356cu64 => "
      DMA.dma6_ctrl_reg(),
    ",
  0x5000356eu64 => "
      DMA.dma6_idx_reg(),
    ",
  0x50003568u64 => "
      DMA.dma6_int_reg(),
    ",
  0x5000356au64 => "
      DMA.dma6_len_reg(),
    ",
  0x50003572u64 => "
      DMA.dma7_a_starth_reg(),
    ",
  0x50003570u64 => "
      DMA.dma7_a_startl_reg(),
    ",
  0x50003576u64 => "
      DMA.dma7_b_starth_reg(),
    ",
  0x50003574u64 => "
      DMA.dma7_b_startl_reg(),
    ",
  0x5000357cu64 => "
      DMA.dma7_ctrl_reg(),
    ",
  0x5000357eu64 => "
      DMA.dma7_idx_reg(),
    ",
  0x50003578u64 => "
      DMA.dma7_int_reg(),
    ",
  0x5000357au64 => "
      DMA.dma7_len_reg(),
    ",
  0x50003584u64 => "
      DMA.dma_clear_int_reg(),
    ",
  0x50003582u64 => "
      DMA.dma_int_status_reg(),
    ",
  0x50003580u64 => "
      DMA.dma_req_mux_reg(),
    ",
  0x50006004u64 => "
      ECC.ecc_command_reg(),
    ",
  0x50006000u64 => "
      ECC.ecc_config_reg(),
    ",
  0x50006008u64 => "
      ECC.ecc_control_reg(),
    ",
  0x5000600cu64 => "
      ECC.ecc_status_reg(),
    ",
  0x50006010u64 => "
      ECC.ecc_version_reg(),
    ",
  0x40090010u64 => "
      FTDF.ftdf_buildtime_0_reg(),
    ",
  0x40090014u64 => "
      FTDF.ftdf_buildtime_1_reg(),
    ",
  0x40090018u64 => "
      FTDF.ftdf_buildtime_2_reg(),
    ",
  0x4009001cu64 => "
      FTDF.ftdf_buildtime_3_reg(),
    ",
  0x40090390u64 => "
      FTDF.ftdf_debugcontrol_reg(),
    ",
  0x40090058u64 => "
      FTDF.ftdf_eventcurrval_reg(),
    ",
  0x40090250u64 => "
      FTDF.ftdf_ftdf_ce_reg(),
    ",
  0x40090254u64 => "
      FTDF.ftdf_ftdf_cm_reg(),
    ",
  0x40090020u64 => "
      FTDF.ftdf_glob_control_0_reg(),
    ",
  0x40090024u64 => "
      FTDF.ftdf_glob_control_1_reg(),
    ",
  0x40090028u64 => "
      FTDF.ftdf_glob_control_2_reg(),
    ",
  0x4009002cu64 => "
      FTDF.ftdf_glob_control_3_reg(),
    ",
  0x40090360u64 => "
      FTDF.ftdf_lmacreset_reg(),
    ",
  0x40090030u64 => "
      FTDF.ftdf_lmac_control_0_reg(),
    ",
  0x4009010cu64 => "
      FTDF.ftdf_lmac_control_10_reg(),
    ",
  0x4009006cu64 => "
      FTDF.ftdf_lmac_control_11_reg(),
    ",
  0x40090040u64 => "
      FTDF.ftdf_lmac_control_1_reg(),
    ",
  0x40090044u64 => "
      FTDF.ftdf_lmac_control_2_reg(),
    ",
  0x40090048u64 => "
      FTDF.ftdf_lmac_control_3_reg(),
    ",
  0x40090060u64 => "
      FTDF.ftdf_lmac_control_4_reg(),
    ",
  0x40090064u64 => "
      FTDF.ftdf_lmac_control_5_reg(),
    ",
  0x40090068u64 => "
      FTDF.ftdf_lmac_control_6_reg(),
    ",
  0x40090100u64 => "
      FTDF.ftdf_lmac_control_7_reg(),
    ",
  0x40090104u64 => "
      FTDF.ftdf_lmac_control_8_reg(),
    ",
  0x40090108u64 => "
      FTDF.ftdf_lmac_control_9_reg(),
    ",
  0x40090070u64 => "
      FTDF.ftdf_lmac_control_delta_reg(),
    ",
  0x40090080u64 => "
      FTDF.ftdf_lmac_control_mask_reg(),
    ",
  0x40090050u64 => "
      FTDF.ftdf_lmac_control_os_reg(),
    ",
  0x40090054u64 => "
      FTDF.ftdf_lmac_control_status_reg(),
    ",
  0x40090090u64 => "
      FTDF.ftdf_lmac_event_reg(),
    ",
  0x400900a0u64 => "
      FTDF.ftdf_lmac_manual_1_reg(),
    ",
  0x400900a4u64 => "
      FTDF.ftdf_lmac_manual_os_reg(),
    ",
  0x400900a8u64 => "
      FTDF.ftdf_lmac_manual_status_reg(),
    ",
  0x40090094u64 => "
      FTDF.ftdf_lmac_mask_reg(),
    ",
  0x40090038u64 => "
      FTDF.ftdf_macackwaitduration_reg(),
    ",
  0x4009003cu64 => "
      FTDF.ftdf_macenhackwaitduration_reg(),
    ",
  0x40090340u64 => "
      FTDF.ftdf_macfcserrorcount_reg(),
    ",
  0x40090318u64 => "
      FTDF.ftdf_macrxaddrfailfrmcnt_reg(),
    ",
  0x40090314u64 => "
      FTDF.ftdf_macrxstdackfrmokcnt_reg(),
    ",
  0x4009031cu64 => "
      FTDF.ftdf_macrxunsupfrmcnt_reg(),
    ",
  0x40090078u64 => "
      FTDF.ftdf_mactstxackdelayval_reg(),
    ",
  0x40090310u64 => "
      FTDF.ftdf_mactxstdackfrmcnt_reg(),
    ",
  0x40090180u64 => "
      FTDF.ftdf_phy_parameters_0_reg(),
    ",
  0x40090184u64 => "
      FTDF.ftdf_phy_parameters_1_reg(),
    ",
  0x40090188u64 => "
      FTDF.ftdf_phy_parameters_2_reg(),
    ",
  0x4009018cu64 => "
      FTDF.ftdf_phy_parameters_3_reg(),
    ",
  0x40090000u64 => "
      FTDF.ftdf_rel_name_0_reg(),
    ",
  0x40090004u64 => "
      FTDF.ftdf_rel_name_1_reg(),
    ",
  0x40090008u64 => "
      FTDF.ftdf_rel_name_2_reg(),
    ",
  0x4009000cu64 => "
      FTDF.ftdf_rel_name_3_reg(),
    ",
  0x40090200u64 => "
      FTDF.ftdf_rx_control_0_reg(),
    ",
  0x40090204u64 => "
      FTDF.ftdf_rx_event_reg(),
    ",
  0x40088000u64 => "
      FTDF.ftdf_rx_fifo_0_0_reg(),
    ",
  0x40088080u64 => "
      FTDF.ftdf_rx_fifo_1_0_reg(),
    ",
  0x40088100u64 => "
      FTDF.ftdf_rx_fifo_2_0_reg(),
    ",
  0x40088180u64 => "
      FTDF.ftdf_rx_fifo_3_0_reg(),
    ",
  0x40088200u64 => "
      FTDF.ftdf_rx_fifo_4_0_reg(),
    ",
  0x40088280u64 => "
      FTDF.ftdf_rx_fifo_5_0_reg(),
    ",
  0x40088300u64 => "
      FTDF.ftdf_rx_fifo_6_0_reg(),
    ",
  0x40088380u64 => "
      FTDF.ftdf_rx_fifo_7_0_reg(),
    ",
  0x40090208u64 => "
      FTDF.ftdf_rx_mask_reg(),
    ",
  0x40080280u64 => "
      FTDF.ftdf_rx_meta_0_0_reg(),
    ",
  0x40080290u64 => "
      FTDF.ftdf_rx_meta_0_1_reg(),
    ",
  0x400802a0u64 => "
      FTDF.ftdf_rx_meta_0_2_reg(),
    ",
  0x400802b0u64 => "
      FTDF.ftdf_rx_meta_0_3_reg(),
    ",
  0x400802c0u64 => "
      FTDF.ftdf_rx_meta_0_4_reg(),
    ",
  0x400802d0u64 => "
      FTDF.ftdf_rx_meta_0_5_reg(),
    ",
  0x400802e0u64 => "
      FTDF.ftdf_rx_meta_0_6_reg(),
    ",
  0x400802f0u64 => "
      FTDF.ftdf_rx_meta_0_7_reg(),
    ",
  0x40080284u64 => "
      FTDF.ftdf_rx_meta_1_0_reg(),
    ",
  0x40080294u64 => "
      FTDF.ftdf_rx_meta_1_1_reg(),
    ",
  0x400802a4u64 => "
      FTDF.ftdf_rx_meta_1_2_reg(),
    ",
  0x400802b4u64 => "
      FTDF.ftdf_rx_meta_1_3_reg(),
    ",
  0x400802c4u64 => "
      FTDF.ftdf_rx_meta_1_4_reg(),
    ",
  0x400802d4u64 => "
      FTDF.ftdf_rx_meta_1_5_reg(),
    ",
  0x400802e4u64 => "
      FTDF.ftdf_rx_meta_1_6_reg(),
    ",
  0x400802f4u64 => "
      FTDF.ftdf_rx_meta_1_7_reg(),
    ",
  0x40090220u64 => "
      FTDF.ftdf_rx_status_delta_reg(),
    ",
  0x40090224u64 => "
      FTDF.ftdf_rx_status_mask_reg(),
    ",
  0x4009020cu64 => "
      FTDF.ftdf_rx_status_reg(),
    ",
  0x40090118u64 => "
      FTDF.ftdf_seckey_0_reg(),
    ",
  0x4009011cu64 => "
      FTDF.ftdf_seckey_1_reg(),
    ",
  0x40090120u64 => "
      FTDF.ftdf_seckey_2_reg(),
    ",
  0x40090124u64 => "
      FTDF.ftdf_seckey_3_reg(),
    ",
  0x40090128u64 => "
      FTDF.ftdf_secnonce_0_reg(),
    ",
  0x4009012cu64 => "
      FTDF.ftdf_secnonce_1_reg(),
    ",
  0x40090130u64 => "
      FTDF.ftdf_secnonce_2_reg(),
    ",
  0x40090134u64 => "
      FTDF.ftdf_secnonce_3_reg(),
    ",
  0x40090110u64 => "
      FTDF.ftdf_security_0_reg(),
    ",
  0x40090114u64 => "
      FTDF.ftdf_security_1_reg(),
    ",
  0x40090154u64 => "
      FTDF.ftdf_security_eventmask_reg(),
    ",
  0x40090150u64 => "
      FTDF.ftdf_security_event_reg(),
    ",
  0x40090138u64 => "
      FTDF.ftdf_security_os_reg(),
    ",
  0x40090140u64 => "
      FTDF.ftdf_security_status_reg(),
    ",
  0x40090384u64 => "
      FTDF.ftdf_symboltime2thr_reg(),
    ",
  0x40090210u64 => "
      FTDF.ftdf_symboltimesnapshotval_reg(),
    ",
  0x40090380u64 => "
      FTDF.ftdf_symboltimethr_reg(),
    ",
  0x40090320u64 => "
      FTDF.ftdf_synctimestampphaseval_reg(),
    ",
  0x40090304u64 => "
      FTDF.ftdf_synctimestampthr_reg(),
    ",
  0x40090308u64 => "
      FTDF.ftdf_synctimestampval_reg(),
    ",
  0x4009030cu64 => "
      FTDF.ftdf_timer_control_1_reg(),
    ",
  0x40090074u64 => "
      FTDF.ftdf_timestampcurrphaseval_reg(),
    ",
  0x4009005cu64 => "
      FTDF.ftdf_timestampcurrval_reg(),
    ",
  0x40090160u64 => "
      FTDF.ftdf_tsch_control_0_reg(),
    ",
  0x40090164u64 => "
      FTDF.ftdf_tsch_control_1_reg(),
    ",
  0x40090168u64 => "
      FTDF.ftdf_tsch_control_2_reg(),
    ",
  0x40090394u64 => "
      FTDF.ftdf_txbyte_e_reg(),
    ",
  0x40090398u64 => "
      FTDF.ftdf_txbyte_m_reg(),
    ",
  0x40090034u64 => "
      FTDF.ftdf_txpipepropdelay_reg(),
    ",
  0x40090484u64 => "
      FTDF.ftdf_tx_clear_os_reg(),
    ",
  0x40090240u64 => "
      FTDF.ftdf_tx_control_0_reg(),
    ",
  0x40080000u64 => "
      FTDF.ftdf_tx_fifo_0_0_reg(),
    ",
  0x40080080u64 => "
      FTDF.ftdf_tx_fifo_1_0_reg(),
    ",
  0x40080100u64 => "
      FTDF.ftdf_tx_fifo_2_0_reg(),
    ",
  0x40080180u64 => "
      FTDF.ftdf_tx_fifo_3_0_reg(),
    ",
  0x40090404u64 => "
      FTDF.ftdf_tx_flag_clear_e_0_reg(),
    ",
  0x40090424u64 => "
      FTDF.ftdf_tx_flag_clear_e_1_reg(),
    ",
  0x40090444u64 => "
      FTDF.ftdf_tx_flag_clear_e_2_reg(),
    ",
  0x40090464u64 => "
      FTDF.ftdf_tx_flag_clear_e_3_reg(),
    ",
  0x40090408u64 => "
      FTDF.ftdf_tx_flag_clear_m_0_reg(),
    ",
  0x40090428u64 => "
      FTDF.ftdf_tx_flag_clear_m_1_reg(),
    ",
  0x40090448u64 => "
      FTDF.ftdf_tx_flag_clear_m_2_reg(),
    ",
  0x40090468u64 => "
      FTDF.ftdf_tx_flag_clear_m_3_reg(),
    ",
  0x40090400u64 => "
      FTDF.ftdf_tx_flag_s_0_reg(),
    ",
  0x40090420u64 => "
      FTDF.ftdf_tx_flag_s_1_reg(),
    ",
  0x40090440u64 => "
      FTDF.ftdf_tx_flag_s_2_reg(),
    ",
  0x40090460u64 => "
      FTDF.ftdf_tx_flag_s_3_reg(),
    ",
  0x40080200u64 => "
      FTDF.ftdf_tx_meta_data_0_0_reg(),
    ",
  0x40080210u64 => "
      FTDF.ftdf_tx_meta_data_0_1_reg(),
    ",
  0x40080220u64 => "
      FTDF.ftdf_tx_meta_data_0_2_reg(),
    ",
  0x40080230u64 => "
      FTDF.ftdf_tx_meta_data_0_3_reg(),
    ",
  0x40080204u64 => "
      FTDF.ftdf_tx_meta_data_1_0_reg(),
    ",
  0x40080214u64 => "
      FTDF.ftdf_tx_meta_data_1_1_reg(),
    ",
  0x40080224u64 => "
      FTDF.ftdf_tx_meta_data_1_2_reg(),
    ",
  0x40080234u64 => "
      FTDF.ftdf_tx_meta_data_1_3_reg(),
    ",
  0x40090410u64 => "
      FTDF.ftdf_tx_priority_0_reg(),
    ",
  0x40090430u64 => "
      FTDF.ftdf_tx_priority_1_reg(),
    ",
  0x40090450u64 => "
      FTDF.ftdf_tx_priority_2_reg(),
    ",
  0x40090470u64 => "
      FTDF.ftdf_tx_priority_3_reg(),
    ",
  0x40080240u64 => "
      FTDF.ftdf_tx_return_status_0_0_reg(),
    ",
  0x40080250u64 => "
      FTDF.ftdf_tx_return_status_0_1_reg(),
    ",
  0x40080260u64 => "
      FTDF.ftdf_tx_return_status_0_2_reg(),
    ",
  0x40080270u64 => "
      FTDF.ftdf_tx_return_status_0_3_reg(),
    ",
  0x40080244u64 => "
      FTDF.ftdf_tx_return_status_1_0_reg(),
    ",
  0x40080254u64 => "
      FTDF.ftdf_tx_return_status_1_1_reg(),
    ",
  0x40080264u64 => "
      FTDF.ftdf_tx_return_status_1_2_reg(),
    ",
  0x40080274u64 => "
      FTDF.ftdf_tx_return_status_1_3_reg(),
    ",
  0x40090480u64 => "
      FTDF.ftdf_tx_set_os_reg(),
    ",
  0x40091000u64 => "
      FTDF.ftdf_wakeupintthr_reg(),
    ",
  0x40091004u64 => "
      FTDF.ftdf_wakeup_control_reg(),
    ",
  0x50003418u64 => "
      GP_TIMERS.breath_cfg_reg(),
    ",
  0x5000341eu64 => "
      GP_TIMERS.breath_ctrl_reg(),
    ",
  0x5000341au64 => "
      GP_TIMERS.breath_duty_max_reg(),
    ",
  0x5000341cu64 => "
      GP_TIMERS.breath_duty_min_reg(),
    ",
  0x5000340eu64 => "
      GP_TIMERS.pwm2_end_cycle(),
    ",
  0x50003408u64 => "
      GP_TIMERS.pwm2_start_cycle(),
    ",
  0x50003410u64 => "
      GP_TIMERS.pwm3_end_cycle(),
    ",
  0x5000340au64 => "
      GP_TIMERS.pwm3_start_cycle(),
    ",
  0x50003412u64 => "
      GP_TIMERS.pwm4_end_cycle(),
    ",
  0x5000340cu64 => "
      GP_TIMERS.pwm4_start_cycle(),
    ",
  0x50003400u64 => "
      GP_TIMERS.timer0_ctrl_reg(),
    ",
  0x50003402u64 => "
      GP_TIMERS.timer0_on_reg(),
    ",
  0x50003404u64 => "
      GP_TIMERS.timer0_reload_m_reg(),
    ",
  0x50003406u64 => "
      GP_TIMERS.timer0_reload_n_reg(),
    ",
  0x50003416u64 => "
      GP_TIMERS.triple_pwm_ctrl_reg(),
    ",
  0x50003414u64 => "
      GP_TIMERS.triple_pwm_frequency(),
    ",
  0x5000190au64 => "
      GPADC.gp_adc_clear_int_reg(),
    ",
  0x50001902u64 => "
      GPADC.gp_adc_ctrl2_reg(),
    ",
  0x50001904u64 => "
      GPADC.gp_adc_ctrl3_reg(),
    ",
  0x50001900u64 => "
      GPADC.gp_adc_ctrl_reg(),
    ",
  0x50001908u64 => "
      GPADC.gp_adc_offn_reg(),
    ",
  0x50001906u64 => "
      GPADC.gp_adc_offp_reg(),
    ",
  0x5000190cu64 => "
      GPADC.gp_adc_result_reg(),
    ",
  0x500030d0u64 => "
      GPIO.gpio_clk_sel(),
    ",
  0x5000301eu64 => "
      GPIO.p00_mode_reg(),
    ",
  0x50003020u64 => "
      GPIO.p01_mode_reg(),
    ",
  0x50003022u64 => "
      GPIO.p02_mode_reg(),
    ",
  0x50003024u64 => "
      GPIO.p03_mode_reg(),
    ",
  0x50003026u64 => "
      GPIO.p04_mode_reg(),
    ",
  0x50003028u64 => "
      GPIO.p05_mode_reg(),
    ",
  0x5000302au64 => "
      GPIO.p06_mode_reg(),
    ",
  0x5000302cu64 => "
      GPIO.p07_mode_reg(),
    ",
  0x50003000u64 => "
      GPIO.p0_data_reg(),
    ",
  0x500030c0u64 => "
      GPIO.p0_padpwr_ctrl_reg(),
    ",
  0x50003014u64 => "
      GPIO.p0_reset_data_reg(),
    ",
  0x5000300au64 => "
      GPIO.p0_set_data_reg(),
    ",
  0x5000302eu64 => "
      GPIO.p10_mode_reg(),
    ",
  0x50003030u64 => "
      GPIO.p11_mode_reg(),
    ",
  0x50003032u64 => "
      GPIO.p12_mode_reg(),
    ",
  0x50003034u64 => "
      GPIO.p13_mode_reg(),
    ",
  0x50003036u64 => "
      GPIO.p14_mode_reg(),
    ",
  0x50003038u64 => "
      GPIO.p15_mode_reg(),
    ",
  0x5000303au64 => "
      GPIO.p16_mode_reg(),
    ",
  0x5000303cu64 => "
      GPIO.p17_mode_reg(),
    ",
  0x50003002u64 => "
      GPIO.p1_data_reg(),
    ",
  0x500030c2u64 => "
      GPIO.p1_padpwr_ctrl_reg(),
    ",
  0x50003016u64 => "
      GPIO.p1_reset_data_reg(),
    ",
  0x5000300cu64 => "
      GPIO.p1_set_data_reg(),
    ",
  0x5000303eu64 => "
      GPIO.p20_mode_reg(),
    ",
  0x50003040u64 => "
      GPIO.p21_mode_reg(),
    ",
  0x50003042u64 => "
      GPIO.p22_mode_reg(),
    ",
  0x50003044u64 => "
      GPIO.p23_mode_reg(),
    ",
  0x50003046u64 => "
      GPIO.p24_mode_reg(),
    ",
  0x50003004u64 => "
      GPIO.p2_data_reg(),
    ",
  0x500030c4u64 => "
      GPIO.p2_padpwr_ctrl_reg(),
    ",
  0x50003018u64 => "
      GPIO.p2_reset_data_reg(),
    ",
  0x5000300eu64 => "
      GPIO.p2_set_data_reg(),
    ",
  0x5000304eu64 => "
      GPIO.p30_mode_reg(),
    ",
  0x50003050u64 => "
      GPIO.p31_mode_reg(),
    ",
  0x50003052u64 => "
      GPIO.p32_mode_reg(),
    ",
  0x50003054u64 => "
      GPIO.p33_mode_reg(),
    ",
  0x50003056u64 => "
      GPIO.p34_mode_reg(),
    ",
  0x50003058u64 => "
      GPIO.p35_mode_reg(),
    ",
  0x5000305au64 => "
      GPIO.p36_mode_reg(),
    ",
  0x5000305cu64 => "
      GPIO.p37_mode_reg(),
    ",
  0x50003006u64 => "
      GPIO.p3_data_reg(),
    ",
  0x500030c6u64 => "
      GPIO.p3_padpwr_ctrl_reg(),
    ",
  0x5000301au64 => "
      GPIO.p3_reset_data_reg(),
    ",
  0x50003010u64 => "
      GPIO.p3_set_data_reg(),
    ",
  0x5000305eu64 => "
      GPIO.p40_mode_reg(),
    ",
  0x50003060u64 => "
      GPIO.p41_mode_reg(),
    ",
  0x50003062u64 => "
      GPIO.p42_mode_reg(),
    ",
  0x50003064u64 => "
      GPIO.p43_mode_reg(),
    ",
  0x50003066u64 => "
      GPIO.p44_mode_reg(),
    ",
  0x50003068u64 => "
      GPIO.p45_mode_reg(),
    ",
  0x5000306au64 => "
      GPIO.p46_mode_reg(),
    ",
  0x5000306cu64 => "
      GPIO.p47_mode_reg(),
    ",
  0x50003008u64 => "
      GPIO.p4_data_reg(),
    ",
  0x500030c8u64 => "
      GPIO.p4_padpwr_ctrl_reg(),
    ",
  0x5000301cu64 => "
      GPIO.p4_reset_data_reg(),
    ",
  0x50003012u64 => "
      GPIO.p4_set_data_reg(),
    ",
  0x50003304u64 => "
      GPREG.debug_reg(),
    ",
  0x5000330au64 => "
      GPREG.ecc_base_addr_reg(),
    ",
  0x50003308u64 => "
      GPREG.gp_control_reg(),
    ",
  0x50003306u64 => "
      GPREG.gp_status_reg(),
    ",
  0x5000330cu64 => "
      GPREG.led_control_reg(),
    ",
  0x50003310u64 => "
      GPREG.pll_sys_ctrl1_reg(),
    ",
  0x50003312u64 => "
      GPREG.pll_sys_ctrl2_reg(),
    ",
  0x50003314u64 => "
      GPREG.pll_sys_ctrl3_reg(),
    ",
  0x50003316u64 => "
      GPREG.pll_sys_status_reg(),
    ",
  0x50003318u64 => "
      GPREG.pll_sys_test_reg(),
    ",
  0x50003302u64 => "
      GPREG.reset_freeze_reg(),
    ",
  0x50003300u64 => "
      GPREG.set_freeze_reg(),
    ",
  0x50001498u64 => "
      I_2_C.i2c_ack_general_call_reg(),
    ",
  0x5000145cu64 => "
      I_2_C.i2c_clr_activity_reg(),
    ",
  0x50001468u64 => "
      I_2_C.i2c_clr_gen_call_reg(),
    ",
  0x50001440u64 => "
      I_2_C.i2c_clr_intr_reg(),
    ",
  0x50001450u64 => "
      I_2_C.i2c_clr_rd_req_reg(),
    ",
  0x50001458u64 => "
      I_2_C.i2c_clr_rx_done_reg(),
    ",
  0x50001448u64 => "
      I_2_C.i2c_clr_rx_over_reg(),
    ",
  0x50001444u64 => "
      I_2_C.i2c_clr_rx_under_reg(),
    ",
  0x50001464u64 => "
      I_2_C.i2c_clr_start_det_reg(),
    ",
  0x50001460u64 => "
      I_2_C.i2c_clr_stop_det_reg(),
    ",
  0x50001454u64 => "
      I_2_C.i2c_clr_tx_abrt_reg(),
    ",
  0x5000144cu64 => "
      I_2_C.i2c_clr_tx_over_reg(),
    ",
  0x500014fau64 => "
      I_2_C.i2c_comp2_version(),
    ",
  0x500014f4u64 => "
      I_2_C.i2c_comp_param1_reg(),
    ",
  0x500014f6u64 => "
      I_2_C.i2c_comp_param2_reg(),
    ",
  0x500014feu64 => "
      I_2_C.i2c_comp_type2_reg(),
    ",
  0x500014fcu64 => "
      I_2_C.i2c_comp_type_reg(),
    ",
  0x500014f8u64 => "
      I_2_C.i2c_comp_version_reg(),
    ",
  0x50001400u64 => "
      I_2_C.i2c_con_reg(),
    ",
  0x50001410u64 => "
      I_2_C.i2c_data_cmd_reg(),
    ",
  0x50001488u64 => "
      I_2_C.i2c_dma_cr_reg(),
    ",
  0x50001490u64 => "
      I_2_C.i2c_dma_rdlr_reg(),
    ",
  0x5000148cu64 => "
      I_2_C.i2c_dma_tdlr_reg(),
    ",
  0x5000146cu64 => "
      I_2_C.i2c_enable_reg(),
    ",
  0x5000149cu64 => "
      I_2_C.i2c_enable_status_reg(),
    ",
  0x5000141cu64 => "
      I_2_C.i2c_fs_scl_hcnt_reg(),
    ",
  0x50001420u64 => "
      I_2_C.i2c_fs_scl_lcnt_reg(),
    ",
  0x5000140cu64 => "
      I_2_C.i2c_hs_maddr_reg(),
    ",
  0x500014a0u64 => "
      I_2_C.i2c_ic_fs_spklen_reg(),
    ",
  0x50001430u64 => "
      I_2_C.i2c_intr_mask_reg(),
    ",
  0x5000142cu64 => "
      I_2_C.i2c_intr_stat_reg(),
    ",
  0x50001434u64 => "
      I_2_C.i2c_raw_intr_stat_reg(),
    ",
  0x50001478u64 => "
      I_2_C.i2c_rxflr_reg(),
    ",
  0x50001438u64 => "
      I_2_C.i2c_rx_tl_reg(),
    ",
  0x50001408u64 => "
      I_2_C.i2c_sar_reg(),
    ",
  0x5000147cu64 => "
      I_2_C.i2c_sda_hold_reg(),
    ",
  0x50001494u64 => "
      I_2_C.i2c_sda_setup_reg(),
    ",
  0x50001414u64 => "
      I_2_C.i2c_ss_scl_hcnt_reg(),
    ",
  0x50001418u64 => "
      I_2_C.i2c_ss_scl_lcnt_reg(),
    ",
  0x50001470u64 => "
      I_2_C.i2c_status_reg(),
    ",
  0x50001404u64 => "
      I_2_C.i2c_tar_reg(),
    ",
  0x50001474u64 => "
      I_2_C.i2c_txflr_reg(),
    ",
  0x50001480u64 => "
      I_2_C.i2c_tx_abrt_source_reg(),
    ",
  0x5000143cu64 => "
      I_2_C.i2c_tx_tl_reg(),
    ",
  0x50001598u64 => "
      I_2_C_2.i2c2_ack_general_call_reg(),
    ",
  0x5000155cu64 => "
      I_2_C_2.i2c2_clr_activity_reg(),
    ",
  0x50001568u64 => "
      I_2_C_2.i2c2_clr_gen_call_reg(),
    ",
  0x50001540u64 => "
      I_2_C_2.i2c2_clr_intr_reg(),
    ",
  0x50001550u64 => "
      I_2_C_2.i2c2_clr_rd_req_reg(),
    ",
  0x50001558u64 => "
      I_2_C_2.i2c2_clr_rx_done_reg(),
    ",
  0x50001548u64 => "
      I_2_C_2.i2c2_clr_rx_over_reg(),
    ",
  0x50001544u64 => "
      I_2_C_2.i2c2_clr_rx_under_reg(),
    ",
  0x50001564u64 => "
      I_2_C_2.i2c2_clr_start_det_reg(),
    ",
  0x50001560u64 => "
      I_2_C_2.i2c2_clr_stop_det_reg(),
    ",
  0x50001554u64 => "
      I_2_C_2.i2c2_clr_tx_abrt_reg(),
    ",
  0x5000154cu64 => "
      I_2_C_2.i2c2_clr_tx_over_reg(),
    ",
  0x500015fau64 => "
      I_2_C_2.i2c2_comp2_version(),
    ",
  0x500015f4u64 => "
      I_2_C_2.i2c2_comp_param1_reg(),
    ",
  0x500015f6u64 => "
      I_2_C_2.i2c2_comp_param2_reg(),
    ",
  0x500015feu64 => "
      I_2_C_2.i2c2_comp_type2_reg(),
    ",
  0x500015fcu64 => "
      I_2_C_2.i2c2_comp_type_reg(),
    ",
  0x500015f8u64 => "
      I_2_C_2.i2c2_comp_version_reg(),
    ",
  0x50001500u64 => "
      I_2_C_2.i2c2_con_reg(),
    ",
  0x50001510u64 => "
      I_2_C_2.i2c2_data_cmd_reg(),
    ",
  0x50001588u64 => "
      I_2_C_2.i2c2_dma_cr_reg(),
    ",
  0x50001590u64 => "
      I_2_C_2.i2c2_dma_rdlr_reg(),
    ",
  0x5000158cu64 => "
      I_2_C_2.i2c2_dma_tdlr_reg(),
    ",
  0x5000156cu64 => "
      I_2_C_2.i2c2_enable_reg(),
    ",
  0x5000159cu64 => "
      I_2_C_2.i2c2_enable_status_reg(),
    ",
  0x5000151cu64 => "
      I_2_C_2.i2c2_fs_scl_hcnt_reg(),
    ",
  0x50001520u64 => "
      I_2_C_2.i2c2_fs_scl_lcnt_reg(),
    ",
  0x5000150cu64 => "
      I_2_C_2.i2c2_hs_maddr_reg(),
    ",
  0x500015a0u64 => "
      I_2_C_2.i2c2_ic_fs_spklen_reg(),
    ",
  0x50001530u64 => "
      I_2_C_2.i2c2_intr_mask_reg(),
    ",
  0x5000152cu64 => "
      I_2_C_2.i2c2_intr_stat_reg(),
    ",
  0x50001534u64 => "
      I_2_C_2.i2c2_raw_intr_stat_reg(),
    ",
  0x50001578u64 => "
      I_2_C_2.i2c2_rxflr_reg(),
    ",
  0x50001538u64 => "
      I_2_C_2.i2c2_rx_tl_reg(),
    ",
  0x50001508u64 => "
      I_2_C_2.i2c2_sar_reg(),
    ",
  0x5000157cu64 => "
      I_2_C_2.i2c2_sda_hold_reg(),
    ",
  0x50001594u64 => "
      I_2_C_2.i2c2_sda_setup_reg(),
    ",
  0x50001514u64 => "
      I_2_C_2.i2c2_ss_scl_hcnt_reg(),
    ",
  0x50001518u64 => "
      I_2_C_2.i2c2_ss_scl_lcnt_reg(),
    ",
  0x50001570u64 => "
      I_2_C_2.i2c2_status_reg(),
    ",
  0x50001504u64 => "
      I_2_C_2.i2c2_tar_reg(),
    ",
  0x50001574u64 => "
      I_2_C_2.i2c2_txflr_reg(),
    ",
  0x50001580u64 => "
      I_2_C_2.i2c2_tx_abrt_source_reg(),
    ",
  0x5000153cu64 => "
      I_2_C_2.i2c2_tx_tl_reg(),
    ",
  0x50001708u64 => "
      IR.ir_ctrl_reg(),
    ",
  0x50001702u64 => "
      IR.ir_freq_carrier_off_reg(),
    ",
  0x50001700u64 => "
      IR.ir_freq_carrier_on_reg(),
    ",
  0x50001712u64 => "
      IR.ir_irq_status_reg(),
    ",
  0x50001704u64 => "
      IR.ir_logic_one_time_reg(),
    ",
  0x50001706u64 => "
      IR.ir_logic_zero_time_reg(),
    ",
  0x5000170eu64 => "
      IR.ir_main_fifo_reg(),
    ",
  0x50001710u64 => "
      IR.ir_repeat_fifo_reg(),
    ",
  0x5000170cu64 => "
      IR.ir_repeat_time_reg(),
    ",
  0x5000170au64 => "
      IR.ir_status_reg(),
    ",
  0x50001602u64 => "
      KBSCAN.kbscn_ctrl2_reg(),
    ",
  0x50001600u64 => "
      KBSCAN.kbscn_ctrl_reg(),
    ",
  0x50001606u64 => "
      KBSCAN.kbscn_debounce_reg(),
    ",
  0x50001604u64 => "
      KBSCAN.kbscn_matrix_size_reg(),
    ",
  0x5000160au64 => "
      KBSCAN.kbscn_message_key_reg(),
    ",
  0x5000160cu64 => "
      KBSCAN.kbscn_p00_mode_reg(),
    ",
  0x5000160eu64 => "
      KBSCAN.kbscn_p01_mode_reg(),
    ",
  0x50001610u64 => "
      KBSCAN.kbscn_p02_mode_reg(),
    ",
  0x50001612u64 => "
      KBSCAN.kbscn_p03_mode_reg(),
    ",
  0x50001614u64 => "
      KBSCAN.kbscn_p04_mode_reg(),
    ",
  0x50001616u64 => "
      KBSCAN.kbscn_p05_mode_reg(),
    ",
  0x50001618u64 => "
      KBSCAN.kbscn_p06_mode_reg(),
    ",
  0x5000161au64 => "
      KBSCAN.kbscn_p07_mode_reg(),
    ",
  0x5000161cu64 => "
      KBSCAN.kbscn_p10_mode_reg(),
    ",
  0x5000161eu64 => "
      KBSCAN.kbscn_p11_mode_reg(),
    ",
  0x50001620u64 => "
      KBSCAN.kbscn_p12_mode_reg(),
    ",
  0x50001622u64 => "
      KBSCAN.kbscn_p13_mode_reg(),
    ",
  0x50001624u64 => "
      KBSCAN.kbscn_p14_mode_reg(),
    ",
  0x50001626u64 => "
      KBSCAN.kbscn_p15_mode_reg(),
    ",
  0x50001628u64 => "
      KBSCAN.kbscn_p16_mode_reg(),
    ",
  0x5000162au64 => "
      KBSCAN.kbscn_p17_mode_reg(),
    ",
  0x5000162cu64 => "
      KBSCAN.kbscn_p20_mode_reg(),
    ",
  0x5000162eu64 => "
      KBSCAN.kbscn_p21_mode_reg(),
    ",
  0x50001630u64 => "
      KBSCAN.kbscn_p22_mode_reg(),
    ",
  0x50001632u64 => "
      KBSCAN.kbscn_p23_mode_reg(),
    ",
  0x50001634u64 => "
      KBSCAN.kbscn_p24_mode_reg(),
    ",
  0x5000163cu64 => "
      KBSCAN.kbscn_p30_mode_reg(),
    ",
  0x5000163eu64 => "
      KBSCAN.kbscn_p31_mode_reg(),
    ",
  0x50001640u64 => "
      KBSCAN.kbscn_p32_mode_reg(),
    ",
  0x50001642u64 => "
      KBSCAN.kbscn_p33_mode_reg(),
    ",
  0x50001644u64 => "
      KBSCAN.kbscn_p34_mode_reg(),
    ",
  0x50001646u64 => "
      KBSCAN.kbscn_p35_mode_reg(),
    ",
  0x50001648u64 => "
      KBSCAN.kbscn_p36_mode_reg(),
    ",
  0x5000164au64 => "
      KBSCAN.kbscn_p37_mode_reg(),
    ",
  0x5000164cu64 => "
      KBSCAN.kbscn_p40_mode_reg(),
    ",
  0x5000164eu64 => "
      KBSCAN.kbscn_p41_mode_reg(),
    ",
  0x50001650u64 => "
      KBSCAN.kbscn_p42_mode_reg(),
    ",
  0x50001652u64 => "
      KBSCAN.kbscn_p43_mode_reg(),
    ",
  0x50001654u64 => "
      KBSCAN.kbscn_p44_mode_reg(),
    ",
  0x50001656u64 => "
      KBSCAN.kbscn_p45_mode_reg(),
    ",
  0x50001658u64 => "
      KBSCAN.kbscn_p46_mode_reg(),
    ",
  0x5000165au64 => "
      KBSCAN.kbscn_p47_mode_reg(),
    ",
  0x50001608u64 => "
      KBSCAN.kbscn_status_reg(),
    ",
  0x7f4000cu64 => "
      OTPC.otpc_ahbadr_reg(),
    ",
  0x7f40010u64 => "
      OTPC.otpc_celadr_reg(),
    ",
  0x7f40018u64 => "
      OTPC.otpc_ffprt_reg(),
    ",
  0x7f4001cu64 => "
      OTPC.otpc_ffrd_reg(),
    ",
  0x7f40000u64 => "
      OTPC.otpc_mode_reg(),
    ",
  0x7f40014u64 => "
      OTPC.otpc_nwords_reg(),
    ",
  0x7f40004u64 => "
      OTPC.otpc_pctrl_reg(),
    ",
  0x7f40024u64 => "
      OTPC.otpc_pwordh_reg(),
    ",
  0x7f40020u64 => "
      OTPC.otpc_pwordl_reg(),
    ",
  0x7f40008u64 => "
      OTPC.otpc_stat_reg(),
    ",
  0x7f40028u64 => "
      OTPC.otpc_tim1_reg(),
    ",
  0x7f4002cu64 => "
      OTPC.otpc_tim2_reg(),
    ",
  0xc000030u64 => "
      QSPIC.qspic_burstbrk_reg(),
    ",
  0xc00000cu64 => "
      QSPIC.qspic_burstcmda_reg(),
    ",
  0xc000010u64 => "
      QSPIC.qspic_burstcmdb_reg(),
    ",
  0xc000038u64 => "
      QSPIC.qspic_chckerase_reg(),
    ",
  0xc000000u64 => "
      QSPIC.qspic_ctrlbus_reg(),
    ",
  0xc000004u64 => "
      QSPIC.qspic_ctrlmode_reg(),
    ",
  0xc000020u64 => "
      QSPIC.qspic_dummydata_reg(),
    ",
  0xc000028u64 => "
      QSPIC.qspic_erasecmda_reg(),
    ",
  0xc00002cu64 => "
      QSPIC.qspic_erasecmdb_reg(),
    ",
  0xc000024u64 => "
      QSPIC.qspic_erasectrl_reg(),
    ",
  0xc00003cu64 => "
      QSPIC.qspic_gp_reg(),
    ",
  0xc00001cu64 => "
      QSPIC.qspic_readdata_reg(),
    ",
  0xc000008u64 => "
      QSPIC.qspic_recvdata_reg(),
    ",
  0xc000034u64 => "
      QSPIC.qspic_statuscmd_reg(),
    ",
  0xc000014u64 => "
      QSPIC.qspic_status_reg(),
    ",
  0xc000040u64 => "
      QSPIC.qspic_ucode_start(),
    ",
  0xc000018u64 => "
      QSPIC.qspic_writedata_reg(),
    ",
  0x50001a08u64 => "
      QUAD.qdec_clockdiv_reg(),
    ",
  0x50001a00u64 => "
      QUAD.qdec_ctrl_reg(),
    ",
  0x50001a02u64 => "
      QUAD.qdec_xcnt_reg(),
    ",
  0x50001a04u64 => "
      QUAD.qdec_ycnt_reg(),
    ",
  0x50001a06u64 => "
      QUAD.qdec_zcnt_reg(),
    ",
  0x50001206u64 => "
      SPI.spi_clear_int_reg(),
    ",
  0x50001200u64 => "
      SPI.spi_ctrl_reg(),
    ",
  0x50001208u64 => "
      SPI.spi_ctrl_reg1(),
    ",
  0x50001202u64 => "
      SPI.spi_rx_tx_reg0(),
    ",
  0x50001204u64 => "
      SPI.spi_rx_tx_reg1(),
    ",
  0x50001306u64 => "
      SPI_2.spi2_clear_int_reg(),
    ",
  0x50001300u64 => "
      SPI_2.spi2_ctrl_reg(),
    ",
  0x50001308u64 => "
      SPI_2.spi2_ctrl_reg1(),
    ",
  0x50001302u64 => "
      SPI_2.spi2_rx_tx_reg0(),
    ",
  0x50001304u64 => "
      SPI_2.spi2_rx_tx_reg1(),
    ",
  0x50000210u64 => "
      TIMER_1.captim_capture_gpio1_reg(),
    ",
  0x50000212u64 => "
      TIMER_1.captim_capture_gpio2_reg(),
    ",
  0x50000200u64 => "
      TIMER_1.captim_ctrl_reg(),
    ",
  0x50000206u64 => "
      TIMER_1.captim_gpio1_conf_reg(),
    ",
  0x50000208u64 => "
      TIMER_1.captim_gpio2_conf_reg(),
    ",
  0x5000020eu64 => "
      TIMER_1.captim_prescaler_reg(),
    ",
  0x50000214u64 => "
      TIMER_1.captim_prescaler_val_reg(),
    ",
  0x50000218u64 => "
      TIMER_1.captim_pwm_dc_reg(),
    ",
  0x50000216u64 => "
      TIMER_1.captim_pwm_freq_reg(),
    ",
  0x5000020au64 => "
      TIMER_1.captim_reload_reg(),
    ",
  0x5000020cu64 => "
      TIMER_1.captim_shotwidth_reg(),
    ",
  0x50000204u64 => "
      TIMER_1.captim_status_reg(),
    ",
  0x50000202u64 => "
      TIMER_1.captim_timer_val_reg(),
    ",
  0x50005000u64 => "
      TRNG.trng_ctrl_reg(),
    ",
  0x50005004u64 => "
      TRNG.trng_fifolvl_reg(),
    ",
  0x50005008u64 => "
      TRNG.trng_ver_reg(),
    ",
  0x500010f4u64 => "
      UART.uart_cpr_reg(),
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
  0x50001000u64 => "
      UART.uart_rbr_thr_dll_reg(),
    ",
  0x50001090u64 => "
      UART.uart_sbcr_reg(),
    ",
  0x5000101cu64 => "
      UART.uart_scr_reg(),
    ",
  0x50001088u64 => "
      UART.uart_srr_reg(),
    ",
  0x500010f8u64 => "
      UART.uart_ucv_reg(),
    ",
  0x5000107cu64 => "
      UART.uart_usr_reg(),
    ",
  0x500011f4u64 => "
      UART_2.uart2_cpr_reg(),
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
  0x50001810u64 => "
      USB.usb_altev_reg(),
    ",
  0x50001812u64 => "
      USB.usb_altmsk_reg(),
    ",
  0x500018d4u64 => "
      USB.usb_charger_ctrl_reg(),
    ",
  0x500018d6u64 => "
      USB.usb_charger_stat_reg(),
    ",
  0x500018d0u64 => "
      USB.usb_dma_ctrl_reg(),
    ",
  0x50001848u64 => "
      USB.usb_ep0_nak_reg(),
    ",
  0x50001840u64 => "
      USB.usb_epc0_reg(),
    ",
  0x50001850u64 => "
      USB.usb_epc1_reg(),
    ",
  0x50001858u64 => "
      USB.usb_epc2_reg(),
    ",
  0x50001860u64 => "
      USB.usb_epc3_reg(),
    ",
  0x50001868u64 => "
      USB.usb_epc4_reg(),
    ",
  0x50001870u64 => "
      USB.usb_epc5_reg(),
    ",
  0x50001878u64 => "
      USB.usb_epc6_reg(),
    ",
  0x50001808u64 => "
      USB.usb_far_reg(),
    ",
  0x50001824u64 => "
      USB.usb_fnh_reg(),
    ",
  0x50001826u64 => "
      USB.usb_fnl_reg(),
    ",
  0x50001820u64 => "
      USB.usb_fwev_reg(),
    ",
  0x50001822u64 => "
      USB.usb_fwmsk_reg(),
    ",
  0x5000180cu64 => "
      USB.usb_maev_reg(),
    ",
  0x5000180eu64 => "
      USB.usb_mamsk_reg(),
    ",
  0x50001800u64 => "
      USB.usb_mctrl_reg(),
    ",
  0x5000181cu64 => "
      USB.usb_nakev_reg(),
    ",
  0x5000181eu64 => "
      USB.usb_nakmsk_reg(),
    ",
  0x5000180au64 => "
      USB.usb_nfsr_reg(),
    ",
  0x5000184eu64 => "
      USB.usb_rxc0_reg(),
    ",
  0x5000185eu64 => "
      USB.usb_rxc1_reg(),
    ",
  0x5000186eu64 => "
      USB.usb_rxc2_reg(),
    ",
  0x5000187eu64 => "
      USB.usb_rxc3_reg(),
    ",
  0x5000184au64 => "
      USB.usb_rxd0_reg(),
    ",
  0x5000185au64 => "
      USB.usb_rxd1_reg(),
    ",
  0x5000186au64 => "
      USB.usb_rxd2_reg(),
    ",
  0x5000187au64 => "
      USB.usb_rxd3_reg(),
    ",
  0x50001818u64 => "
      USB.usb_rxev_reg(),
    ",
  0x5000181au64 => "
      USB.usb_rxmsk_reg(),
    ",
  0x5000184cu64 => "
      USB.usb_rxs0_reg(),
    ",
  0x5000185cu64 => "
      USB.usb_rxs1_reg(),
    ",
  0x5000186cu64 => "
      USB.usb_rxs2_reg(),
    ",
  0x5000187cu64 => "
      USB.usb_rxs3_reg(),
    ",
  0x50001804u64 => "
      USB.usb_tcr_reg(),
    ",
  0x50001846u64 => "
      USB.usb_txc0_reg(),
    ",
  0x50001856u64 => "
      USB.usb_txc1_reg(),
    ",
  0x50001866u64 => "
      USB.usb_txc2_reg(),
    ",
  0x50001876u64 => "
      USB.usb_txc3_reg(),
    ",
  0x50001842u64 => "
      USB.usb_txd0_reg(),
    ",
  0x50001852u64 => "
      USB.usb_txd1_reg(),
    ",
  0x50001862u64 => "
      USB.usb_txd2_reg(),
    ",
  0x50001872u64 => "
      USB.usb_txd3_reg(),
    ",
  0x50001814u64 => "
      USB.usb_txev_reg(),
    ",
  0x50001816u64 => "
      USB.usb_txmsk_reg(),
    ",
  0x50001844u64 => "
      USB.usb_txs0_reg(),
    ",
  0x50001854u64 => "
      USB.usb_txs1_reg(),
    ",
  0x50001864u64 => "
      USB.usb_txs2_reg(),
    ",
  0x50001874u64 => "
      USB.usb_txs3_reg(),
    ",
  0x50001806u64 => "
      USB.usb_utr_reg(),
    ",
  0x5000183eu64 => "
      USB.usb_ux20cdr_reg(),
    ",
  0x50001802u64 => "
      USB.usb_xcvdiag_reg(),
    ",
  0x50000102u64 => "
      WAKEUP.wkup_compare_reg(),
    ",
  0x50000106u64 => "
      WAKEUP.wkup_counter_reg(),
    ",
  0x50000100u64 => "
      WAKEUP.wkup_ctrl_reg(),
    ",
  0x50000114u64 => "
      WAKEUP.wkup_pol_p0_reg(),
    ",
  0x50000116u64 => "
      WAKEUP.wkup_pol_p1_reg(),
    ",
  0x50000118u64 => "
      WAKEUP.wkup_pol_p2_reg(),
    ",
  0x5000011au64 => "
      WAKEUP.wkup_pol_p3_reg(),
    ",
  0x5000011cu64 => "
      WAKEUP.wkup_pol_p4_reg(),
    ",
  0x50000108u64 => "
      WAKEUP.wkup_reset_cntr_reg(),
    ",
  0x50000104u64 => "
      WAKEUP.wkup_reset_irq_reg(),
    ",
  0x5000010au64 => "
      WAKEUP.wkup_select_p0_reg(),
    ",
  0x5000010cu64 => "
      WAKEUP.wkup_select_p1_reg(),
    ",
  0x5000010eu64 => "
      WAKEUP.wkup_select_p2_reg(),
    ",
  0x50000110u64 => "
      WAKEUP.wkup_select_p3_reg(),
    ",
  0x50000112u64 => "
      WAKEUP.wkup_select_p4_reg(),
    ",
  0x50003102u64 => "
      WDOG.watchdog_ctrl_reg(),
    ",
  0x50003100u64 => "
      WDOG.watchdog_reg(),
    ",
};
