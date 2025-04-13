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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"DSI2 registers"]
unsafe impl ::core::marker::Send for super::Dsi2 {}
unsafe impl ::core::marker::Sync for super::Dsi2 {}
impl super::Dsi2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_autoinsert_eotp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgAutoinsertEotpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgAutoinsertEotpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_bta_h_to_count_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgBtaHToCountReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgBtaHToCountReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_clk_lane_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgClkLaneEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgClkLaneEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_continuous_hs_clk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgContinuousHsClkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgContinuousHsClkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_data_lane_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgDataLaneEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgDataLaneEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_disable_burst_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgDisableBurstReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgDisableBurstReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_disbl_rx_crc_check_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgDisblRxCrcCheckReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgDisblRxCrcCheckReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_ext_cmds_aft_eotp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgExtCmdsAftEotpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgExtCmdsAftEotpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_htx_to_count_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgHtxToCountReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgHtxToCountReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Status of APB to packet interface, reading will clear IRQ status 1 and 2"]
    #[inline(always)]
    pub const fn dsi2_cfg_irq_mask(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgIrqMask_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(424usize),
            )
        }
    }

    #[doc = "Status of APB to packet interface part 2, read part 2 first then dsi2 host irq 2"]
    #[inline(always)]
    pub const fn dsi2_cfg_irq_mask2(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgIrqMask2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgIrqMask2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(428usize),
            )
        }
    }

    #[doc = "Status of APB to packet interface, reading will clear IRQ status 1 and 2"]
    #[inline(always)]
    pub const fn dsi2_cfg_irq_status(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgIrqStatus_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgIrqStatus_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[doc = "Status of APB to packet interface part 2, read part 2 first then dsi2 host irq 2"]
    #[inline(always)]
    pub const fn dsi2_cfg_irq_status2(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgIrqStatus2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgIrqStatus2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(420usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_lrx_h_to_count_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgLrxHToCountReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgLrxHToCountReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "configure numer of active lanes"]
    #[inline(always)]
    pub const fn dsi2_cfg_num_lanes_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgNumLanesReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgNumLanesReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Tx packet control register"]
    #[inline(always)]
    pub const fn dsi2_cfg_packet_control(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgPacketControl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgPacketControl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[doc = "Read level of APB to pkt interface fifo"]
    #[inline(always)]
    pub const fn dsi2_cfg_pkt_rd_level(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgPktRdLevel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgPktRdLevel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(404usize),
            )
        }
    }

    #[doc = "Packet 2 APB RX header"]
    #[inline(always)]
    pub const fn dsi2_cfg_pkt_rx_header(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgPktRxHeader_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgPktRxHeader_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(412usize),
            )
        }
    }

    #[doc = "Packet 2 APB RX payload"]
    #[inline(always)]
    pub const fn dsi2_cfg_pkt_rx_payload(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgPktRxPayload_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgPktRxPayload_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(408usize),
            )
        }
    }

    #[doc = "Status of APB to packet interface"]
    #[inline(always)]
    pub const fn dsi2_cfg_pkt_status(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgPktStatus_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgPktStatus_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(396usize),
            )
        }
    }

    #[doc = "Write level of APB to pkt interface fifo"]
    #[inline(always)]
    pub const fn dsi2_cfg_pkt_wr_level(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgPktWrLevel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgPktWrLevel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(400usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_rx_error_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgRxErrorStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgRxErrorStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Tx send packet"]
    #[inline(always)]
    pub const fn dsi2_cfg_send_packet(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgSendPacket_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgSendPacket_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
            )
        }
    }

    #[doc = "Contains the status of the status register"]
    #[inline(always)]
    pub const fn dsi2_cfg_status_out_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgStatusOutReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgStatusOutReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_twakeup_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgTwakeupReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgTwakeupReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_tx_gap_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgTxGapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgTxGapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "TX Payload data write register"]
    #[inline(always)]
    pub const fn dsi2_cfg_tx_payload(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgTxPayload_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgTxPayload_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_t_post_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgTPostReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgTPostReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_t_pre_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgTPreReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgTPreReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_bllp_mode(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidBllpMode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidBllpMode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_enable(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidEnable_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidEnable_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_hbp(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidHbp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidHbp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_hfp(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidHfp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidHfp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_hsa(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidHsa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidHsa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_hsync_polarity(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidHsyncPolarity_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidHsyncPolarity_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_override(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidOverride_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidOverride_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_packets_p_line(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidPacketsPLine_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidPacketsPLine_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(308usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_pix_alignment(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidPixAlignment_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidPixAlignment_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_pix_format(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidPixFormat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidPixFormat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_pix_payload_size(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidPixPayloadSize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidPixPayloadSize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_pix_p_packet(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidPixPPacket_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidPixPPacket_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_start_delay(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidStartDelay_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidStartDelay_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_use_null_pkt_bllp(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidUseNullPktBllp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidUseNullPktBllp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(324usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_vactive(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidVactive_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidVactive_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(328usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_vbp(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidVbp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidVbp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(312usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_vc(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidVc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidVc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(332usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_vfp(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidVfp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidVfp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(316usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_video_mode(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidVideoMode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidVideoMode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn dsi2_cfg_vid_vsync_polarity(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2CfgVidVsyncPolarity_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2CfgVidVsyncPolarity_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgAutoinsertEotpReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgAutoinsertEotpReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgAutoinsertEotpReg = crate::RegValueT<Dsi2CfgAutoinsertEotpReg_SPEC>;

impl Dsi2CfgAutoinsertEotpReg {
    #[doc = "Enables the Host Controller to automatically insert an EoTp short packet when switching from HS to LP mode.\n1b0 eotp is not automatically inserted\n1b1 eotp is automatically inserted"]
    #[inline(always)]
    pub fn autoinsert_eotp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgAutoinsertEotpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgAutoinsertEotpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgAutoinsertEotpReg {
    #[inline(always)]
    fn default() -> Dsi2CfgAutoinsertEotpReg {
        <crate::RegValueT<Dsi2CfgAutoinsertEotpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgBtaHToCountReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgBtaHToCountReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgBtaHToCountReg = crate::RegValueT<Dsi2CfgBtaHToCountReg_SPEC>;

impl Dsi2CfgBtaHToCountReg {
    #[doc = "Host Bust Turn Around (BTA) Timout. Sets the value of the DSI-2 Host Bus Turn Around timeout in clk clock periods that once reached will initiate a timeout error. A value of 0x000000 disables the timeout."]
    #[inline(always)]
    pub fn bta_h_to_count(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        Dsi2CfgBtaHToCountReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            Dsi2CfgBtaHToCountReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgBtaHToCountReg {
    #[inline(always)]
    fn default() -> Dsi2CfgBtaHToCountReg {
        <crate::RegValueT<Dsi2CfgBtaHToCountReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgClkLaneEnReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgClkLaneEnReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgClkLaneEnReg = crate::RegValueT<Dsi2CfgClkLaneEnReg_SPEC>;

impl Dsi2CfgClkLaneEnReg {
    #[doc = "Forces PHY Enable n signals to 1\'b1 when register is set to 1. See the DSI-2\nController User Guide description of the input port cfg clk lane en for additional information."]
    #[inline(always)]
    pub fn clk_lane_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgClkLaneEnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgClkLaneEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgClkLaneEnReg {
    #[inline(always)]
    fn default() -> Dsi2CfgClkLaneEnReg {
        <crate::RegValueT<Dsi2CfgClkLaneEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgContinuousHsClkReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgContinuousHsClkReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgContinuousHsClkReg = crate::RegValueT<Dsi2CfgContinuousHsClkReg_SPEC>;

impl Dsi2CfgContinuousHsClkReg {
    #[doc = "Sets the Host Controller into non-continuous MIPI clock mode. When in\nnon-continuous clock mode, the high speed clock will transistion into low power mode between\ntransmissions. 1\'b0 - Non-Continuous high speed clock 1\'b1 - Continuous high speed clock"]
    #[inline(always)]
    pub fn continuous_hs_clk(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgContinuousHsClkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgContinuousHsClkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgContinuousHsClkReg {
    #[inline(always)]
    fn default() -> Dsi2CfgContinuousHsClkReg {
        <crate::RegValueT<Dsi2CfgContinuousHsClkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgDataLaneEnReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgDataLaneEnReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgDataLaneEnReg = crate::RegValueT<Dsi2CfgDataLaneEnReg_SPEC>;

impl Dsi2CfgDataLaneEnReg {
    #[doc = "Forces PHY Enable n signals to 1\'b1 when register is set to 1. See the DSI-2\nController User Guide description of the input port cfg data lane en for additional informa-\ntion."]
    #[inline(always)]
    pub fn data_lane_en(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsi2CfgDataLaneEnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsi2CfgDataLaneEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgDataLaneEnReg {
    #[inline(always)]
    fn default() -> Dsi2CfgDataLaneEnReg {
        <crate::RegValueT<Dsi2CfgDataLaneEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgDisableBurstReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgDisableBurstReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgDisableBurstReg = crate::RegValueT<Dsi2CfgDisableBurstReg_SPEC>;

impl Dsi2CfgDisableBurstReg {
    #[doc = "Disables packets combined into a burst. Normal DSI-2 operation is to combine packets and send as a burst without returning to LP mode between each packet.\n1b0 packets are combined and sent as a burst. (Default DSI-2 behavior)\n1b1 Controller returns to LP mode between each packet."]
    #[inline(always)]
    pub fn disable_burst(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgDisableBurstReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgDisableBurstReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgDisableBurstReg {
    #[inline(always)]
    fn default() -> Dsi2CfgDisableBurstReg {
        <crate::RegValueT<Dsi2CfgDisableBurstReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgDisblRxCrcCheckReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgDisblRxCrcCheckReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgDisblRxCrcCheckReg = crate::RegValueT<Dsi2CfgDisblRxCrcCheckReg_SPEC>;

impl Dsi2CfgDisblRxCrcCheckReg {
    #[doc = "Prevents the Host from checking the payload CRC in long packets sent from the Peripheral. This input should be set if the peripheral does not support CRC generation."]
    #[inline(always)]
    pub fn disable_rx_crc_check(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgDisblRxCrcCheckReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgDisblRxCrcCheckReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgDisblRxCrcCheckReg {
    #[inline(always)]
    fn default() -> Dsi2CfgDisblRxCrcCheckReg {
        <crate::RegValueT<Dsi2CfgDisblRxCrcCheckReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgExtCmdsAftEotpReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgExtCmdsAftEotpReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgExtCmdsAftEotpReg = crate::RegValueT<Dsi2CfgExtCmdsAftEotpReg_SPEC>;

impl Dsi2CfgExtCmdsAftEotpReg {
    #[doc = "Configures the DSI-2 Host Controller to send extra End Of Transmission Packets after the end of a packet. The value of cfg_extra_cmd_after_eotp is the number of extra EOTP packets sent."]
    #[inline(always)]
    pub fn extra_cmds_after_eotp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        Dsi2CfgExtCmdsAftEotpReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            Dsi2CfgExtCmdsAftEotpReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgExtCmdsAftEotpReg {
    #[inline(always)]
    fn default() -> Dsi2CfgExtCmdsAftEotpReg {
        <crate::RegValueT<Dsi2CfgExtCmdsAftEotpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgHtxToCountReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgHtxToCountReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgHtxToCountReg = crate::RegValueT<Dsi2CfgHtxToCountReg_SPEC>;

impl Dsi2CfgHtxToCountReg {
    #[doc = "Host HS TX Timeout count, HS TX Timeout. Sets the value of the DSI-2 host High Speed TX timeout count in clk clock periods that once reached will initiate a timeout error and follow the recovery procedure documented in the DSI-2 specification. A value of 0x000000 disables the timeout."]
    #[inline(always)]
    pub fn htx_to_count(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        Dsi2CfgHtxToCountReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            Dsi2CfgHtxToCountReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgHtxToCountReg {
    #[inline(always)]
    fn default() -> Dsi2CfgHtxToCountReg {
        <crate::RegValueT<Dsi2CfgHtxToCountReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgIrqMask_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgIrqMask_SPEC {
    type DataType = u32;
}
#[doc = "Status of APB to packet interface, reading will clear IRQ status 1 and 2"]
pub type Dsi2CfgIrqMask = crate::RegValueT<Dsi2CfgIrqMask_SPEC>;

impl Dsi2CfgIrqMask {
    #[doc = "host bta timeout, host controller host bta timeout port"]
    #[inline(always)]
    pub fn irq_hs_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "low power rx timeout, host controller lp rx timeout port"]
    #[inline(always)]
    pub fn irq_lp_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "high speed tx timeout, host controller hs tx timeout port"]
    #[inline(always)]
    pub fn irq_host_bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "map directory to dsi host controller status out port bit descriptions"]
    #[inline(always)]
    pub fn irq_map_directory(
        self,
    ) -> crate::common::RegisterField<9, 0xfffff, 1, 0, u32, Dsi2CfgIrqMask_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0xfffff,1,0,u32, Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "All RX packet payload data has been received"]
    #[inline(always)]
    pub fn irq_rx_packet_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX packet header has been received"]
    #[inline(always)]
    pub fn irq_rx_header_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX fifo underflow"]
    #[inline(always)]
    pub fn irq_rx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX fifo overflow"]
    #[inline(always)]
    pub fn irq_rx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX fifo underflow"]
    #[inline(always)]
    pub fn irq_tx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX fifo overflow"]
    #[inline(always)]
    pub fn irq_tx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DPHY direction\n0 - TX had control\n1 - RX has control"]
    #[inline(always)]
    pub fn irq_dphy_direction(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX packet done"]
    #[inline(always)]
    pub fn irq_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State machine not idle"]
    #[inline(always)]
    pub fn irq_state_not_idle(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgIrqMask {
    #[inline(always)]
    fn default() -> Dsi2CfgIrqMask {
        <crate::RegValueT<Dsi2CfgIrqMask_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgIrqMask2_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgIrqMask2_SPEC {
    type DataType = u32;
}
#[doc = "Status of APB to packet interface part 2, read part 2 first then dsi2 host irq 2"]
pub type Dsi2CfgIrqMask2 = crate::RegValueT<Dsi2CfgIrqMask2_SPEC>;

impl Dsi2CfgIrqMask2 {
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn irq_crc_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqMask2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqMask2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX packet done"]
    #[inline(always)]
    pub fn irq_multi_bit_ecc_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqMask2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqMask2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Single bit ecc error"]
    #[inline(always)]
    pub fn irq_single_bit_ecc_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgIrqMask2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgIrqMask2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgIrqMask2 {
    #[inline(always)]
    fn default() -> Dsi2CfgIrqMask2 {
        <crate::RegValueT<Dsi2CfgIrqMask2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgIrqStatus_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgIrqStatus_SPEC {
    type DataType = u32;
}
#[doc = "Status of APB to packet interface, reading will clear IRQ status 1 and 2"]
pub type Dsi2CfgIrqStatus = crate::RegValueT<Dsi2CfgIrqStatus_SPEC>;

impl Dsi2CfgIrqStatus {
    #[doc = "host bta timeout, host controller host bta timeout port"]
    #[inline(always)]
    pub fn irq_hs_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "low power rx timeout, host controller lp rx timeout port"]
    #[inline(always)]
    pub fn irq_lp_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "high speed tx timeout, host controller hs tx timeout port"]
    #[inline(always)]
    pub fn irq_host_bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "map directory to dsi host controller status out port bit descriptions"]
    #[inline(always)]
    pub fn irq_map_directory(
        self,
    ) -> crate::common::RegisterField<9, 0xfffff, 1, 0, u32, Dsi2CfgIrqStatus_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0xfffff,1,0,u32, Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "All RX packet payload data has been received"]
    #[inline(always)]
    pub fn irq_rx_packet_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX packet header has been received"]
    #[inline(always)]
    pub fn irq_rx_header_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX fifo underflow"]
    #[inline(always)]
    pub fn irq_rx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX fifo overflow"]
    #[inline(always)]
    pub fn irq_rx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX fifo underflow"]
    #[inline(always)]
    pub fn irq_tx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX fifo overflow"]
    #[inline(always)]
    pub fn irq_tx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DPHY direction\n0 - TX had control\n1 - RX has control"]
    #[inline(always)]
    pub fn irq_dphy_direction(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX packet done"]
    #[inline(always)]
    pub fn irq_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "State machine not idle"]
    #[inline(always)]
    pub fn irq_state_not_idle(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgIrqStatus {
    #[inline(always)]
    fn default() -> Dsi2CfgIrqStatus {
        <crate::RegValueT<Dsi2CfgIrqStatus_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgIrqStatus2_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgIrqStatus2_SPEC {
    type DataType = u32;
}
#[doc = "Status of APB to packet interface part 2, read part 2 first then dsi2 host irq 2"]
pub type Dsi2CfgIrqStatus2 = crate::RegValueT<Dsi2CfgIrqStatus2_SPEC>;

impl Dsi2CfgIrqStatus2 {
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn irq_crc_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqStatus2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqStatus2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX packet done"]
    #[inline(always)]
    pub fn irq_multi_bit_ecc_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqStatus2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqStatus2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Single bit ecc error"]
    #[inline(always)]
    pub fn irq_single_bit_ecc_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgIrqStatus2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgIrqStatus2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgIrqStatus2 {
    #[inline(always)]
    fn default() -> Dsi2CfgIrqStatus2 {
        <crate::RegValueT<Dsi2CfgIrqStatus2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgLrxHToCountReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgLrxHToCountReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgLrxHToCountReg = crate::RegValueT<Dsi2CfgLrxHToCountReg_SPEC>;

impl Dsi2CfgLrxHToCountReg {
    #[doc = "Host Low Power RX Timeout, LP_RX-H Timeout. Sets the value of the DSI-2 Low Power RX timeout count in clk clock periods that once reached will initiate a timeout error and follow the recovery procedure documented in the DSI-2 specification. A value of 0x000000 disables the timeout."]
    #[inline(always)]
    pub fn lrx_h_to_count(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
        Dsi2CfgLrxHToCountReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffff,
            1,
            0,
            u32,
            Dsi2CfgLrxHToCountReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgLrxHToCountReg {
    #[inline(always)]
    fn default() -> Dsi2CfgLrxHToCountReg {
        <crate::RegValueT<Dsi2CfgLrxHToCountReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgNumLanesReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgNumLanesReg_SPEC {
    type DataType = u32;
}
#[doc = "configure numer of active lanes"]
pub type Dsi2CfgNumLanesReg = crate::RegValueT<Dsi2CfgNumLanesReg_SPEC>;

impl Dsi2CfgNumLanesReg {
    #[doc = "Sets the number of active lanes that are to be used for transmitting data. 4\'b0000\n- No active lanes (reset default)\n4\'b0001 - 1 Active Lanes\n4\'b0010 - 2 Active Lanes\n4\'b0011 - 3 Active Lanes (not supported)\n4\'b0100 - 4 Active Lanes (not supported)"]
    #[inline(always)]
    pub fn num_lanes(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Dsi2CfgNumLanesReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8, Dsi2CfgNumLanesReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgNumLanesReg {
    #[inline(always)]
    fn default() -> Dsi2CfgNumLanesReg {
        <crate::RegValueT<Dsi2CfgNumLanesReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgPacketControl_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgPacketControl_SPEC {
    type DataType = u32;
}
#[doc = "Tx packet control register"]
pub type Dsi2CfgPacketControl = crate::RegValueT<Dsi2CfgPacketControl_SPEC>;

impl Dsi2CfgPacketControl {
    #[doc = "Perform BTA only, no packet tx"]
    #[inline(always)]
    pub fn pkt_bta_only(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Dsi2CfgPacketControl_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,Dsi2CfgPacketControl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Perform BTA after packet is sent"]
    #[inline(always)]
    pub fn pkt_bta_after_sent(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Dsi2CfgPacketControl_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<25,1,0,Dsi2CfgPacketControl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "low power or high speed"]
    #[inline(always)]
    pub fn pkt_lp_or_hs(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Dsi2CfgPacketControl_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,Dsi2CfgPacketControl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Packet header DSI Data Type"]
    #[inline(always)]
    pub fn pkt_header_dtype(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        Dsi2CfgPacketControl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            Dsi2CfgPacketControl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Virtual channel"]
    #[inline(always)]
    pub fn pkt_vc(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Dsi2CfgPacketControl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            u8,
            Dsi2CfgPacketControl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Packet word count"]
    #[inline(always)]
    pub fn pkt_wc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        Dsi2CfgPacketControl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            Dsi2CfgPacketControl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgPacketControl {
    #[inline(always)]
    fn default() -> Dsi2CfgPacketControl {
        <crate::RegValueT<Dsi2CfgPacketControl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgPktRdLevel_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgPktRdLevel_SPEC {
    type DataType = u32;
}
#[doc = "Read level of APB to pkt interface fifo"]
pub type Dsi2CfgPktRdLevel = crate::RegValueT<Dsi2CfgPktRdLevel_SPEC>;

impl Dsi2CfgPktRdLevel {
    #[doc = "Read level of APB to pkt interface fifo"]
    #[inline(always)]
    pub fn pkt_fifo_rd_level(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgPktRdLevel_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dsi2CfgPktRdLevel_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgPktRdLevel {
    #[inline(always)]
    fn default() -> Dsi2CfgPktRdLevel {
        <crate::RegValueT<Dsi2CfgPktRdLevel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgPktRxHeader_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgPktRxHeader_SPEC {
    type DataType = u32;
}
#[doc = "Packet 2 APB RX header"]
pub type Dsi2CfgPktRxHeader = crate::RegValueT<Dsi2CfgPktRxHeader_SPEC>;

impl Dsi2CfgPktRxHeader {
    #[doc = "Packet header DSI Data Type"]
    #[inline(always)]
    pub fn pkt_rx_header_dtype(
        self,
    ) -> crate::common::RegisterField<18, 0x3f, 1, 0, u8, Dsi2CfgPktRxHeader_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x3f,1,0,u8, Dsi2CfgPktRxHeader_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Virtual channel"]
    #[inline(always)]
    pub fn pkt_rx_vc(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Dsi2CfgPktRxHeader_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8, Dsi2CfgPktRxHeader_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Packet word count"]
    #[inline(always)]
    pub fn pkt_rx_wc(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgPktRxHeader_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            Dsi2CfgPktRxHeader_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgPktRxHeader {
    #[inline(always)]
    fn default() -> Dsi2CfgPktRxHeader {
        <crate::RegValueT<Dsi2CfgPktRxHeader_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgPktRxPayload_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgPktRxPayload_SPEC {
    type DataType = u32;
}
#[doc = "Packet 2 APB RX payload"]
pub type Dsi2CfgPktRxPayload = crate::RegValueT<Dsi2CfgPktRxPayload_SPEC>;

impl Dsi2CfgPktRxPayload {
    #[doc = "APB to pkt interface RX payload read"]
    #[inline(always)]
    pub fn pkt_pkt_rx_payload(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        Dsi2CfgPktRxPayload_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Dsi2CfgPktRxPayload_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgPktRxPayload {
    #[inline(always)]
    fn default() -> Dsi2CfgPktRxPayload {
        <crate::RegValueT<Dsi2CfgPktRxPayload_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgPktStatus_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgPktStatus_SPEC {
    type DataType = u32;
}
#[doc = "Status of APB to packet interface"]
pub type Dsi2CfgPktStatus = crate::RegValueT<Dsi2CfgPktStatus_SPEC>;

impl Dsi2CfgPktStatus {
    #[doc = "All RX packet payload data has been received"]
    #[inline(always)]
    pub fn pkt_rx_packet_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX packet header has been received"]
    #[inline(always)]
    pub fn pkt_rx_header_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX fifo underflow"]
    #[inline(always)]
    pub fn pkt_rx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RX fifo overflow"]
    #[inline(always)]
    pub fn pkt_rx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX fifo underflow"]
    #[inline(always)]
    pub fn pkt_tx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX fifo overflow"]
    #[inline(always)]
    pub fn pkt_tx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DPHY direction\n0 - TX had control\n1 - RX has control"]
    #[inline(always)]
    pub fn pkt_dphy_direction(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX packet done"]
    #[inline(always)]
    pub fn pkt_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "State machine not idle"]
    #[inline(always)]
    pub fn pkt_state_not_idle(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgPktStatus {
    #[inline(always)]
    fn default() -> Dsi2CfgPktStatus {
        <crate::RegValueT<Dsi2CfgPktStatus_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgPktWrLevel_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgPktWrLevel_SPEC {
    type DataType = u32;
}
#[doc = "Write level of APB to pkt interface fifo"]
pub type Dsi2CfgPktWrLevel = crate::RegValueT<Dsi2CfgPktWrLevel_SPEC>;

impl Dsi2CfgPktWrLevel {
    #[doc = "Write level of APB to pkt interface fifo"]
    #[inline(always)]
    pub fn pkt_fifo_wr_level(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgPktWrLevel_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dsi2CfgPktWrLevel_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgPktWrLevel {
    #[inline(always)]
    fn default() -> Dsi2CfgPktWrLevel {
        <crate::RegValueT<Dsi2CfgPktWrLevel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgRxErrorStatusReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgRxErrorStatusReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgRxErrorStatusReg = crate::RegValueT<Dsi2CfgRxErrorStatusReg_SPEC>;

impl Dsi2CfgRxErrorStatusReg {
    #[doc = "BTA timeout"]
    #[inline(always)]
    pub fn bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reverse Low Power Data receive timeout"]
    #[inline(always)]
    pub fn rev_lp_data_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "High Speed forward TX timeout"]
    #[inline(always)]
    pub fn hs_fwd_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC error"]
    #[inline(always)]
    pub fn crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bit position for ECC single bit error"]
    #[inline(always)]
    pub fn ecc_single_bit_err_pos(
        self,
    ) -> crate::common::RegisterField<
        3,
        0xf,
        1,
        0,
        u8,
        Dsi2CfgRxErrorStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0xf,
            1,
            0,
            u8,
            Dsi2CfgRxErrorStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ECC multi bit error"]
    #[inline(always)]
    pub fn ecc_multi_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ECC single bit error"]
    #[inline(always)]
    pub fn ecc_single_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgRxErrorStatusReg {
    #[inline(always)]
    fn default() -> Dsi2CfgRxErrorStatusReg {
        <crate::RegValueT<Dsi2CfgRxErrorStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgSendPacket_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgSendPacket_SPEC {
    type DataType = u32;
}
#[doc = "Tx send packet"]
pub type Dsi2CfgSendPacket = crate::RegValueT<Dsi2CfgSendPacket_SPEC>;

impl Dsi2CfgSendPacket {
    #[doc = "Tx send packet, writing to this register causes the packet described in dsi2 host\npkt control to be sent."]
    #[inline(always)]
    pub fn cfg_send_packet(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgSendPacket_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgSendPacket_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgSendPacket {
    #[inline(always)]
    fn default() -> Dsi2CfgSendPacket {
        <crate::RegValueT<Dsi2CfgSendPacket_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgStatusOutReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgStatusOutReg_SPEC {
    type DataType = u32;
}
#[doc = "Contains the status of the status register"]
pub type Dsi2CfgStatusOutReg = crate::RegValueT<Dsi2CfgStatusOutReg_SPEC>;

impl Dsi2CfgStatusOutReg {
    #[doc = "Last received Trigger. Current status of the RxTriggerEsc\\[3:0\\] from the TX DPHY. Each bit represents one of the possible 4 trigger values was received but unfortunately the MIPI DPHY spec does not define with RxTriggerEsc bit represents which trigger value so the value will be DPHY provider dependent. For Mixel DPHYs the values are as follows:\n4b0001 reset trigger ( 01100010 )\n4b0010 unknown trigger 3 ( 01011101 )\n4b0100 unknown trigger 4 ( 00100001 )\n4b1000 unknown trigger 5 ( 10100000 )"]
    #[inline(always)]
    pub fn last_rcvd_trigger(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Protocol violation error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn dsi2_prot_violation(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission length error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn invalid_trans_length(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Invalid VC from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn dsi2_vc_id_invalid(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Invalid data type from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn dsi2_dt_not_recognized(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Checksum error (long packet only) from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn lp_checksum_error(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ECC multi-bit error from peripheral error report (not corrected), cleared upon read"]
    #[inline(always)]
    pub fn ecc_multi_bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ECC single bit error from peripheral error report (and corrected), cleared upon read"]
    #[inline(always)]
    pub fn ecc_1bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Contention Detection from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn contention_detected(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "False Control Error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn false_ctrl_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Peripheral Timeout error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn periph_timeout_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Low Power Transmit Sync error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn lp_trans_sync_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Escape Mode Entry Command Error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn esc_mode_entry_cmd_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "End of Transmission Sync Error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn eot_sync_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Start of Transmission Sync Error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn sot_sync_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Start of Transmission Error from peripheral error report, cleared upon read"]
    #[inline(always)]
    pub fn sot_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgStatusOutReg {
    #[inline(always)]
    fn default() -> Dsi2CfgStatusOutReg {
        <crate::RegValueT<Dsi2CfgStatusOutReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgTwakeupReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgTwakeupReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgTwakeupReg = crate::RegValueT<Dsi2CfgTwakeupReg_SPEC>;

impl Dsi2CfgTwakeupReg {
    #[doc = "PHY Twakeup timing parameter. Sets the number of clk_esc clock periods to keep a clock or data lane in Mark-1 state after exiting ULPS. The MIPI PHY spec (D-PHY and C-PHY) requires a minimum of 1ms in Mark-1 state after leaving ULPS.\n\nBecause each lane requires a wakeup timer, the default hardware configuration ignores the lower 8 bits to reduce logic and area. This gives a timer resolution of 256 esc clocks. If higher resolution is required, the controller can be delivered with full 19 bit support at the expense of extra logic and area."]
    #[inline(always)]
    pub fn twakeup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffff,
        1,
        0,
        u32,
        Dsi2CfgTwakeupReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ffff,
            1,
            0,
            u32,
            Dsi2CfgTwakeupReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgTwakeupReg {
    #[inline(always)]
    fn default() -> Dsi2CfgTwakeupReg {
        <crate::RegValueT<Dsi2CfgTwakeupReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgTxGapReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgTxGapReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgTxGapReg = crate::RegValueT<Dsi2CfgTxGapReg_SPEC>;

impl Dsi2CfgTxGapReg {
    #[doc = "Sets the number of byte clock periods (clk input) that the controller will wait after the clock lane has been put into LP mode before enabling the clock lane for HS mode again."]
    #[inline(always)]
    pub fn tx_gap(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Dsi2CfgTxGapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Dsi2CfgTxGapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgTxGapReg {
    #[inline(always)]
    fn default() -> Dsi2CfgTxGapReg {
        <crate::RegValueT<Dsi2CfgTxGapReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgTxPayload_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgTxPayload_SPEC {
    type DataType = u32;
}
#[doc = "TX Payload data write register"]
pub type Dsi2CfgTxPayload = crate::RegValueT<Dsi2CfgTxPayload_SPEC>;

impl Dsi2CfgTxPayload {
    #[doc = "Tx Payload data write register. Writes to this registers load the payload fo with\n32 bit values."]
    #[inline(always)]
    pub fn cfg_tx_payload(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        Dsi2CfgTxPayload_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Dsi2CfgTxPayload_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgTxPayload {
    #[inline(always)]
    fn default() -> Dsi2CfgTxPayload {
        <crate::RegValueT<Dsi2CfgTxPayload_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgTPostReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgTPostReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgTPostReg = crate::RegValueT<Dsi2CfgTPostReg_SPEC>;

impl Dsi2CfgTPostReg {
    #[doc = "Sets the number of byte clock periods (clk input) to wait before putting the clock lane into LP mode after the data lanes have been put into LP mode."]
    #[inline(always)]
    pub fn t_post(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Dsi2CfgTPostReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Dsi2CfgTPostReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgTPostReg {
    #[inline(always)]
    fn default() -> Dsi2CfgTPostReg {
        <crate::RegValueT<Dsi2CfgTPostReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgTPreReg_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgTPreReg_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgTPreReg = crate::RegValueT<Dsi2CfgTPreReg_SPEC>;

impl Dsi2CfgTPreReg {
    #[doc = "Sets the number of byte clock periods (clk input) that the controller will wait after enabling the clock lane for HS operation before enabling the data lanes for HS operation."]
    #[inline(always)]
    pub fn t_pre(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Dsi2CfgTPreReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Dsi2CfgTPreReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgTPreReg {
    #[inline(always)]
    fn default() -> Dsi2CfgTPreReg {
        <crate::RegValueT<Dsi2CfgTPreReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidBllpMode_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidBllpMode_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidBllpMode = crate::RegValueT<Dsi2CfgVidBllpMode_SPEC>;

impl Dsi2CfgVidBllpMode {
    #[doc = "Optimize bllp periods to Low Power mode when possible\n0 blanking packets are sent during BLLP periods\n1 LP mode is used for BLLP periods"]
    #[inline(always)]
    pub fn vid_bllp_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgVidBllpMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgVidBllpMode_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidBllpMode {
    #[inline(always)]
    fn default() -> Dsi2CfgVidBllpMode {
        <crate::RegValueT<Dsi2CfgVidBllpMode_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidEnable_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidEnable_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidEnable = crate::RegValueT<Dsi2CfgVidEnable_SPEC>;

impl Dsi2CfgVidEnable {
    #[doc = "Enables the video interface\n0 = video interface off; all packets go through rx packet interface\n1 = video interface on; video packets routed out video interface"]
    #[inline(always)]
    pub fn vid_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgVidEnable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgVidEnable_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidEnable {
    #[inline(always)]
    fn default() -> Dsi2CfgVidEnable {
        <crate::RegValueT<Dsi2CfgVidEnable_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidHbp_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidHbp_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidHbp = crate::RegValueT<Dsi2CfgVidHbp_SPEC>;

impl Dsi2CfgVidHbp {
    #[doc = "Sets the DSI-2 packet payload size, in bytes, of the horizontal back porch blanking packet. This input is ignored if cfg_vid_override = 1b0."]
    #[inline(always)]
    pub fn vid_hbp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgVidHbp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dsi2CfgVidHbp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidHbp {
    #[inline(always)]
    fn default() -> Dsi2CfgVidHbp {
        <crate::RegValueT<Dsi2CfgVidHbp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidHfp_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidHfp_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidHfp = crate::RegValueT<Dsi2CfgVidHfp_SPEC>;

impl Dsi2CfgVidHfp {
    #[doc = "Sets the DSI-2 packet payload size, in bytes, of the horizontal front porch blanking packet. This input is ignored if cfg_vid_override = 1b0."]
    #[inline(always)]
    pub fn vid_hfp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgVidHfp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dsi2CfgVidHfp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidHfp {
    #[inline(always)]
    fn default() -> Dsi2CfgVidHfp {
        <crate::RegValueT<Dsi2CfgVidHfp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidHsa_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidHsa_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidHsa = crate::RegValueT<Dsi2CfgVidHsa_SPEC>;

impl Dsi2CfgVidHsa {
    #[doc = "Sets the DSI-2 packet payload size, in bytes, of the horizontal sync width filler blanking packet. This input is ignored if cfg_vid_override = 1b0."]
    #[inline(always)]
    pub fn vid_hsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgVidHsa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dsi2CfgVidHsa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidHsa {
    #[inline(always)]
    fn default() -> Dsi2CfgVidHsa {
        <crate::RegValueT<Dsi2CfgVidHsa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidHsyncPolarity_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidHsyncPolarity_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidHsyncPolarity = crate::RegValueT<Dsi2CfgVidHsyncPolarity_SPEC>;

impl Dsi2CfgVidHsyncPolarity {
    #[doc = "Sets Polarity of vid_hsync input, 0 active low, 1 active high"]
    #[inline(always)]
    pub fn vid_hsync_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgVidHsyncPolarity_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgVidHsyncPolarity_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidHsyncPolarity {
    #[inline(always)]
    fn default() -> Dsi2CfgVidHsyncPolarity {
        <crate::RegValueT<Dsi2CfgVidHsyncPolarity_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidOverride_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidOverride_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidOverride = crate::RegValueT<Dsi2CfgVidOverride_SPEC>;

impl Dsi2CfgVidOverride {
    #[doc = "Overrides internal counters and uses values on configuration inputs.\n1b0 Sync timing parameters are calculated off video input.\n1b1 Sync timing parameters are set by cfg_vid_* configuration signals."]
    #[inline(always)]
    pub fn vid_override(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgVidOverride_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgVidOverride_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidOverride {
    #[inline(always)]
    fn default() -> Dsi2CfgVidOverride {
        <crate::RegValueT<Dsi2CfgVidOverride_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidPacketsPLine_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidPacketsPLine_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidPacketsPLine = crate::RegValueT<Dsi2CfgVidPacketsPLine_SPEC>;

impl Dsi2CfgVidPacketsPLine {
    #[doc = "Sets the number of packets that will be sent for a video line. Default reset value is 3d1. Currently, only the default is supported:\n3b001 Video Line is sent in a single packet"]
    #[inline(always)]
    pub fn vid_packets_p_line(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
        Dsi2CfgVidPacketsPLine_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            Dsi2CfgVidPacketsPLine_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgVidPacketsPLine {
    #[inline(always)]
    fn default() -> Dsi2CfgVidPacketsPLine {
        <crate::RegValueT<Dsi2CfgVidPacketsPLine_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidPixAlignment_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidPixAlignment_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidPixAlignment = crate::RegValueT<Dsi2CfgVidPixAlignment_SPEC>;

impl Dsi2CfgVidPixAlignment {
    #[doc = "Some RGB modes can be aligned either MSB or LSB onto the video_pX\\[35:0\\] inputs. See section DSI2 spec 4.9.5 for assignments.\n0 LSB alignment\n1 MSB alignment"]
    #[inline(always)]
    pub fn vid_pix_alignment(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgVidPixAlignment_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgVidPixAlignment_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidPixAlignment {
    #[inline(always)]
    fn default() -> Dsi2CfgVidPixAlignment {
        <crate::RegValueT<Dsi2CfgVidPixAlignment_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidPixFormat_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidPixFormat_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidPixFormat = crate::RegValueT<Dsi2CfgVidPixFormat_SPEC>;

impl Dsi2CfgVidPixFormat {
    #[doc = "Sets the DSI-2 packet type of the pixels. Value is the actual data type sent across the MIPI interface.\n0x0B Compressed Pixel Stream, 32-bit or 64-bit\n0x0C - Loosely Packed Pixel Stream, 20-bit YCbCr, 4:2:2\n0x1C - Packed Pixel Stream, 24-bit YCbCr, 4:2:2 format\n0x2C - Packed Pixel Stream, 16-bit YCbCr, 4:2:2 format\n0x0D - Packed Pixel Stream, 30-bit RGB, 10-10-10 format\n0x1D - Packed Pixel Stream, 36-bit RGB, 12-12-12 format\n0x3D - Packed Pixel Stream, 12-bit YCbCr, 4:2:0 format\n0x0E - Packed Pixel Stream, 16-bit RGB, 5-6-5 format\n0x1E - Packed Pixel Stream, 18-bit RGB, 6-6-6\n0x2E - Loosely Packed Pixel Stream, 18-bit RGB, 6-6-6\n0x3E - Packed Pixel Stream, 24-bit RGB, 8-8-8"]
    #[inline(always)]
    pub fn vid_pix_format(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Dsi2CfgVidPixFormat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Dsi2CfgVidPixFormat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidPixFormat {
    #[inline(always)]
    fn default() -> Dsi2CfgVidPixFormat {
        <crate::RegValueT<Dsi2CfgVidPixFormat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidPixPayloadSize_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidPixPayloadSize_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidPixPayloadSize = crate::RegValueT<Dsi2CfgVidPixPayloadSize_SPEC>;

impl Dsi2CfgVidPixPayloadSize {
    #[doc = "The number of bytes in a video payload packet. Normally this can be set to zero and let the payload size to be calculated based on cfg_vid_pixels_packet. However, in cases like DSC operation where the number of bytes may not be a multiple of 32 or 64, cfg_vid_pixel_payload_size should be set to reflect the total number of bytes to be sent."]
    #[inline(always)]
    pub fn vid_pix_payload_size(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        Dsi2CfgVidPixPayloadSize_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            Dsi2CfgVidPixPayloadSize_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgVidPixPayloadSize {
    #[inline(always)]
    fn default() -> Dsi2CfgVidPixPayloadSize {
        <crate::RegValueT<Dsi2CfgVidPixPayloadSize_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidPixPPacket_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidPixPPacket_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidPixPPacket = crate::RegValueT<Dsi2CfgVidPixPPacket_SPEC>;

impl Dsi2CfgVidPixPPacket {
    #[doc = "Sets the number of pixels that are sent in a packet for each video line. If cfg_vid_packets_per_line is set to 3d1, cfg_vid_pixels_per_packet would be set to the total pixels for each video line. If cfg_vid_packets_per_line is set to 2d2, then this should be set to half the number of pixels on a video line."]
    #[inline(always)]
    pub fn vid_pix_p_packet(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        Dsi2CfgVidPixPPacket_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            Dsi2CfgVidPixPPacket_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgVidPixPPacket {
    #[inline(always)]
    fn default() -> Dsi2CfgVidPixPPacket {
        <crate::RegValueT<Dsi2CfgVidPixPPacket_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidStartDelay_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidStartDelay_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidStartDelay = crate::RegValueT<Dsi2CfgVidStartDelay_SPEC>;

impl Dsi2CfgVidStartDelay {
    #[doc = "In order to optimize DSI-2 utility, the video interface buffers a certain number of pixels before initiating a DSI-2 packet. This configuration port controls the number of clk clock cycles to wait before requesting the DSI-2 Host Controller to start sending data."]
    #[inline(always)]
    pub fn vid_start_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        Dsi2CfgVidStartDelay_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            Dsi2CfgVidStartDelay_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgVidStartDelay {
    #[inline(always)]
    fn default() -> Dsi2CfgVidStartDelay {
        <crate::RegValueT<Dsi2CfgVidStartDelay_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidUseNullPktBllp_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidUseNullPktBllp_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidUseNullPktBllp = crate::RegValueT<Dsi2CfgVidUseNullPktBllp_SPEC>;

impl Dsi2CfgVidUseNullPktBllp {
    #[doc = "Selects type of blanking packet to be sent during bllp region\n0 - Blanking packet used in bllp region\n1 - Null packet used in bllp region"]
    #[inline(always)]
    pub fn vid_use_null_pkt_bllp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgVidUseNullPktBllp_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgVidUseNullPktBllp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidUseNullPktBllp {
    #[inline(always)]
    fn default() -> Dsi2CfgVidUseNullPktBllp {
        <crate::RegValueT<Dsi2CfgVidUseNullPktBllp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidVactive_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidVactive_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidVactive = crate::RegValueT<Dsi2CfgVidVactive_SPEC>;

impl Dsi2CfgVidVactive {
    #[doc = "Sets the number of lines in the vertical active area. This input is ignored if cfg_vid_override = 1b0."]
    #[inline(always)]
    pub fn vid_vactive(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgVidVactive_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            Dsi2CfgVidVactive_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dsi2CfgVidVactive {
    #[inline(always)]
    fn default() -> Dsi2CfgVidVactive {
        <crate::RegValueT<Dsi2CfgVidVactive_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidVbp_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidVbp_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidVbp = crate::RegValueT<Dsi2CfgVidVbp_SPEC>;

impl Dsi2CfgVidVbp {
    #[doc = "Sets the number of lines in the vertical back porch. This input is ignored if cfg_vid_override = 1b0."]
    #[inline(always)]
    pub fn vid_vbp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgVidVbp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dsi2CfgVidVbp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidVbp {
    #[inline(always)]
    fn default() -> Dsi2CfgVidVbp {
        <crate::RegValueT<Dsi2CfgVidVbp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidVc_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidVc_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidVc = crate::RegValueT<Dsi2CfgVidVc_SPEC>;

impl Dsi2CfgVidVc {
    #[doc = "Sets the Virtual Channel (VC) of packets that will be sent to the receive packet interface. Unless cfg_disable_vc_check is set, packets with VC not equal to this value are discarded and the \"DSI-2 VC ID Invalid\" bit (bit 12) in the DSI-2 error report is set."]
    #[inline(always)]
    pub fn vid_vc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsi2CfgVidVc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsi2CfgVidVc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidVc {
    #[inline(always)]
    fn default() -> Dsi2CfgVidVc {
        <crate::RegValueT<Dsi2CfgVidVc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidVfp_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidVfp_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidVfp = crate::RegValueT<Dsi2CfgVidVfp_SPEC>;

impl Dsi2CfgVidVfp {
    #[doc = "Sets the number of lines in the vertical front porch. This input is ignored if cfg_vid_override = 1b0."]
    #[inline(always)]
    pub fn vid_vfp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dsi2CfgVidVfp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dsi2CfgVidVfp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidVfp {
    #[inline(always)]
    fn default() -> Dsi2CfgVidVfp {
        <crate::RegValueT<Dsi2CfgVidVfp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidVideoMode_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidVideoMode_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidVideoMode = crate::RegValueT<Dsi2CfgVidVideoMode_SPEC>;

impl Dsi2CfgVidVideoMode {
    #[doc = "Select DSI-2 video mode that the host VID module should generate packets for.\n2b00 Non-Burst mode with Sync Pulses\n2b01 Non-Burst mode with Sync Events\n2b10 Burst Mode\n2b11 Reserved, not valid"]
    #[inline(always)]
    pub fn vid_vid_video_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsi2CfgVidVideoMode_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsi2CfgVidVideoMode_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidVideoMode {
    #[inline(always)]
    fn default() -> Dsi2CfgVidVideoMode {
        <crate::RegValueT<Dsi2CfgVidVideoMode_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2CfgVidVsyncPolarity_SPEC;
impl crate::sealed::RegSpec for Dsi2CfgVidVsyncPolarity_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Dsi2CfgVidVsyncPolarity = crate::RegValueT<Dsi2CfgVidVsyncPolarity_SPEC>;

impl Dsi2CfgVidVsyncPolarity {
    #[doc = "Sets polarity of vid_vsync input, 0 active low, 1 active high"]
    #[inline(always)]
    pub fn vid_vsync_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2CfgVidVsyncPolarity_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2CfgVidVsyncPolarity_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2CfgVidVsyncPolarity {
    #[inline(always)]
    fn default() -> Dsi2CfgVidVsyncPolarity {
        <crate::RegValueT<Dsi2CfgVidVsyncPolarity_SPEC> as RegisterValue<_>>::new(0)
    }
}
