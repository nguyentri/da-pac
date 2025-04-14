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
#[doc = r"DSI2 registers"]
unsafe impl ::core::marker::Send for super::Dsi2 {}
unsafe impl ::core::marker::Sync for super::Dsi2 {}
impl super::Dsi2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

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

pub type Dsi2CfgAutoinsertEotpReg = crate::RegValueT<Dsi2CfgAutoinsertEotpReg_SPEC>;

impl Dsi2CfgAutoinsertEotpReg {
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

pub type Dsi2CfgBtaHToCountReg = crate::RegValueT<Dsi2CfgBtaHToCountReg_SPEC>;

impl Dsi2CfgBtaHToCountReg {
    #[inline(always)]
    pub fn bta_h_to_count(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
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

pub type Dsi2CfgClkLaneEnReg = crate::RegValueT<Dsi2CfgClkLaneEnReg_SPEC>;

impl Dsi2CfgClkLaneEnReg {
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

pub type Dsi2CfgContinuousHsClkReg = crate::RegValueT<Dsi2CfgContinuousHsClkReg_SPEC>;

impl Dsi2CfgContinuousHsClkReg {
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

pub type Dsi2CfgDataLaneEnReg = crate::RegValueT<Dsi2CfgDataLaneEnReg_SPEC>;

impl Dsi2CfgDataLaneEnReg {
    #[inline(always)]
    pub fn data_lane_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        Dsi2CfgDataLaneEnReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            Dsi2CfgDataLaneEnReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgDisableBurstReg = crate::RegValueT<Dsi2CfgDisableBurstReg_SPEC>;

impl Dsi2CfgDisableBurstReg {
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

pub type Dsi2CfgDisblRxCrcCheckReg = crate::RegValueT<Dsi2CfgDisblRxCrcCheckReg_SPEC>;

impl Dsi2CfgDisblRxCrcCheckReg {
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

pub type Dsi2CfgExtCmdsAftEotpReg = crate::RegValueT<Dsi2CfgExtCmdsAftEotpReg_SPEC>;

impl Dsi2CfgExtCmdsAftEotpReg {
    #[inline(always)]
    pub fn extra_cmds_after_eotp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
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

pub type Dsi2CfgHtxToCountReg = crate::RegValueT<Dsi2CfgHtxToCountReg_SPEC>;

impl Dsi2CfgHtxToCountReg {
    #[inline(always)]
    pub fn htx_to_count(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
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

pub type Dsi2CfgIrqMask = crate::RegValueT<Dsi2CfgIrqMask_SPEC>;

impl Dsi2CfgIrqMask {
    #[inline(always)]
    pub fn irq_hs_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_lp_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_host_bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_map_directory(
        self,
    ) -> crate::common::RegisterField<
        9,
        0xfffff,
        1,
        0,
        u32,
        u32,
        Dsi2CfgIrqMask_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0xfffff,
            1,
            0,
            u32,
            u32,
            Dsi2CfgIrqMask_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq_rx_packet_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_rx_header_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_rx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_rx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_tx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_tx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_dphy_direction(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqMask_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqMask_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Dsi2CfgIrqMask2 = crate::RegValueT<Dsi2CfgIrqMask2_SPEC>;

impl Dsi2CfgIrqMask2 {
    #[inline(always)]
    pub fn irq_crc_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqMask2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqMask2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_multi_bit_ecc_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqMask2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqMask2_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Dsi2CfgIrqStatus = crate::RegValueT<Dsi2CfgIrqStatus_SPEC>;

impl Dsi2CfgIrqStatus {
    #[inline(always)]
    pub fn irq_hs_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_lp_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_host_bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_map_directory(
        self,
    ) -> crate::common::RegisterField<
        9,
        0xfffff,
        1,
        0,
        u32,
        u32,
        Dsi2CfgIrqStatus_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0xfffff,
            1,
            0,
            u32,
            u32,
            Dsi2CfgIrqStatus_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn irq_rx_packet_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_rx_header_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_rx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_rx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_tx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_tx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_dphy_direction(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqStatus_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type Dsi2CfgIrqStatus2 = crate::RegValueT<Dsi2CfgIrqStatus2_SPEC>;

impl Dsi2CfgIrqStatus2 {
    #[inline(always)]
    pub fn irq_crc_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgIrqStatus2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgIrqStatus2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_multi_bit_ecc_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgIrqStatus2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgIrqStatus2_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type Dsi2CfgLrxHToCountReg = crate::RegValueT<Dsi2CfgLrxHToCountReg_SPEC>;

impl Dsi2CfgLrxHToCountReg {
    #[inline(always)]
    pub fn lrx_h_to_count(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffff,
        1,
        0,
        u32,
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

pub type Dsi2CfgNumLanesReg = crate::RegValueT<Dsi2CfgNumLanesReg_SPEC>;

impl Dsi2CfgNumLanesReg {
    #[inline(always)]
    pub fn num_lanes(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        Dsi2CfgNumLanesReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            Dsi2CfgNumLanesReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgPacketControl = crate::RegValueT<Dsi2CfgPacketControl_SPEC>;

impl Dsi2CfgPacketControl {
    #[inline(always)]
    pub fn pkt_bta_only(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Dsi2CfgPacketControl_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<26,1,0,Dsi2CfgPacketControl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_bta_after_sent(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Dsi2CfgPacketControl_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<25,1,0,Dsi2CfgPacketControl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_lp_or_hs(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Dsi2CfgPacketControl_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,Dsi2CfgPacketControl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_header_dtype(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
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
            u8,
            Dsi2CfgPacketControl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pkt_vc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        u8,
        u8,
        Dsi2CfgPacketControl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            u8,
            u8,
            Dsi2CfgPacketControl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pkt_wc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
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

pub type Dsi2CfgPktRdLevel = crate::RegValueT<Dsi2CfgPktRdLevel_SPEC>;

impl Dsi2CfgPktRdLevel {
    #[inline(always)]
    pub fn pkt_fifo_rd_level(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgPktRdLevel_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dsi2CfgPktRdLevel_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type Dsi2CfgPktRxHeader = crate::RegValueT<Dsi2CfgPktRxHeader_SPEC>;

impl Dsi2CfgPktRxHeader {
    #[inline(always)]
    pub fn pkt_rx_header_dtype(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3f,
        1,
        0,
        u8,
        u8,
        Dsi2CfgPktRxHeader_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            Dsi2CfgPktRxHeader_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pkt_rx_vc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        u8,
        u8,
        Dsi2CfgPktRxHeader_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            u8,
            u8,
            Dsi2CfgPktRxHeader_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pkt_rx_wc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgPktRxHeader_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
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

pub type Dsi2CfgPktRxPayload = crate::RegValueT<Dsi2CfgPktRxPayload_SPEC>;

impl Dsi2CfgPktRxPayload {
    #[inline(always)]
    pub fn pkt_pkt_rx_payload(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type Dsi2CfgPktStatus = crate::RegValueT<Dsi2CfgPktStatus_SPEC>;

impl Dsi2CfgPktStatus {
    #[inline(always)]
    pub fn pkt_rx_packet_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_rx_header_rcvd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_rx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_rx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_tx_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_tx_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_dphy_direction(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pkt_tx_done(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgPktStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgPktStatus_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type Dsi2CfgPktWrLevel = crate::RegValueT<Dsi2CfgPktWrLevel_SPEC>;

impl Dsi2CfgPktWrLevel {
    #[inline(always)]
    pub fn pkt_fifo_wr_level(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgPktWrLevel_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dsi2CfgPktWrLevel_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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

pub type Dsi2CfgRxErrorStatusReg = crate::RegValueT<Dsi2CfgRxErrorStatusReg_SPEC>;

impl Dsi2CfgRxErrorStatusReg {
    #[inline(always)]
    pub fn bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rev_lp_data_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hs_fwd_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_single_bit_err_pos(
        self,
    ) -> crate::common::RegisterField<
        3,
        0xf,
        1,
        0,
        u8,
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
            u8,
            Dsi2CfgRxErrorStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ecc_multi_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgRxErrorStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgRxErrorStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type Dsi2CfgSendPacket = crate::RegValueT<Dsi2CfgSendPacket_SPEC>;

impl Dsi2CfgSendPacket {
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

pub type Dsi2CfgStatusOutReg = crate::RegValueT<Dsi2CfgStatusOutReg_SPEC>;

impl Dsi2CfgStatusOutReg {
    #[inline(always)]
    pub fn last_rcvd_trigger(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        Dsi2CfgStatusOutReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            Dsi2CfgStatusOutReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dsi2_prot_violation(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<15,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn invalid_trans_length(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<13,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dsi2_vc_id_invalid(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<12,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dsi2_dt_not_recognized(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<11,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_checksum_error(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_multi_bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_1bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn contention_detected(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn false_ctrl_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn periph_timeout_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_trans_sync_error(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn esc_mode_entry_cmd_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eot_sync_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sot_sync_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2CfgStatusOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2CfgStatusOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type Dsi2CfgTwakeupReg = crate::RegValueT<Dsi2CfgTwakeupReg_SPEC>;

impl Dsi2CfgTwakeupReg {
    #[inline(always)]
    pub fn twakeup(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ffff,
        1,
        0,
        u32,
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

pub type Dsi2CfgTxGapReg = crate::RegValueT<Dsi2CfgTxGapReg_SPEC>;

impl Dsi2CfgTxGapReg {
    #[inline(always)]
    pub fn tx_gap(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dsi2CfgTxGapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dsi2CfgTxGapReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Dsi2CfgTxPayload = crate::RegValueT<Dsi2CfgTxPayload_SPEC>;

impl Dsi2CfgTxPayload {
    #[inline(always)]
    pub fn cfg_tx_payload(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
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

pub type Dsi2CfgTPostReg = crate::RegValueT<Dsi2CfgTPostReg_SPEC>;

impl Dsi2CfgTPostReg {
    #[inline(always)]
    pub fn t_post(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dsi2CfgTPostReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dsi2CfgTPostReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Dsi2CfgTPreReg = crate::RegValueT<Dsi2CfgTPreReg_SPEC>;

impl Dsi2CfgTPreReg {
    #[inline(always)]
    pub fn t_pre(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dsi2CfgTPreReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dsi2CfgTPreReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Dsi2CfgVidBllpMode = crate::RegValueT<Dsi2CfgVidBllpMode_SPEC>;

impl Dsi2CfgVidBllpMode {
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

pub type Dsi2CfgVidEnable = crate::RegValueT<Dsi2CfgVidEnable_SPEC>;

impl Dsi2CfgVidEnable {
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

pub type Dsi2CfgVidHbp = crate::RegValueT<Dsi2CfgVidHbp_SPEC>;

impl Dsi2CfgVidHbp {
    #[inline(always)]
    pub fn vid_hbp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgVidHbp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dsi2CfgVidHbp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgVidHfp = crate::RegValueT<Dsi2CfgVidHfp_SPEC>;

impl Dsi2CfgVidHfp {
    #[inline(always)]
    pub fn vid_hfp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgVidHfp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dsi2CfgVidHfp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgVidHsa = crate::RegValueT<Dsi2CfgVidHsa_SPEC>;

impl Dsi2CfgVidHsa {
    #[inline(always)]
    pub fn vid_hsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgVidHsa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dsi2CfgVidHsa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgVidHsyncPolarity = crate::RegValueT<Dsi2CfgVidHsyncPolarity_SPEC>;

impl Dsi2CfgVidHsyncPolarity {
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

pub type Dsi2CfgVidOverride = crate::RegValueT<Dsi2CfgVidOverride_SPEC>;

impl Dsi2CfgVidOverride {
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

pub type Dsi2CfgVidPacketsPLine = crate::RegValueT<Dsi2CfgVidPacketsPLine_SPEC>;

impl Dsi2CfgVidPacketsPLine {
    #[inline(always)]
    pub fn vid_packets_p_line(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
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

pub type Dsi2CfgVidPixAlignment = crate::RegValueT<Dsi2CfgVidPixAlignment_SPEC>;

impl Dsi2CfgVidPixAlignment {
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

pub type Dsi2CfgVidPixFormat = crate::RegValueT<Dsi2CfgVidPixFormat_SPEC>;

impl Dsi2CfgVidPixFormat {
    #[inline(always)]
    pub fn vid_pix_format(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        Dsi2CfgVidPixFormat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            Dsi2CfgVidPixFormat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgVidPixPayloadSize = crate::RegValueT<Dsi2CfgVidPixPayloadSize_SPEC>;

impl Dsi2CfgVidPixPayloadSize {
    #[inline(always)]
    pub fn vid_pix_payload_size(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
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

pub type Dsi2CfgVidPixPPacket = crate::RegValueT<Dsi2CfgVidPixPPacket_SPEC>;

impl Dsi2CfgVidPixPPacket {
    #[inline(always)]
    pub fn vid_pix_p_packet(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
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

pub type Dsi2CfgVidStartDelay = crate::RegValueT<Dsi2CfgVidStartDelay_SPEC>;

impl Dsi2CfgVidStartDelay {
    #[inline(always)]
    pub fn vid_start_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
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

pub type Dsi2CfgVidUseNullPktBllp = crate::RegValueT<Dsi2CfgVidUseNullPktBllp_SPEC>;

impl Dsi2CfgVidUseNullPktBllp {
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

pub type Dsi2CfgVidVactive = crate::RegValueT<Dsi2CfgVidVactive_SPEC>;

impl Dsi2CfgVidVactive {
    #[inline(always)]
    pub fn vid_vactive(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgVidVactive_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
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

pub type Dsi2CfgVidVbp = crate::RegValueT<Dsi2CfgVidVbp_SPEC>;

impl Dsi2CfgVidVbp {
    #[inline(always)]
    pub fn vid_vbp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgVidVbp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dsi2CfgVidVbp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgVidVc = crate::RegValueT<Dsi2CfgVidVc_SPEC>;

impl Dsi2CfgVidVc {
    #[inline(always)]
    pub fn vid_vc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Dsi2CfgVidVc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Dsi2CfgVidVc_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Dsi2CfgVidVfp = crate::RegValueT<Dsi2CfgVidVfp_SPEC>;

impl Dsi2CfgVidVfp {
    #[inline(always)]
    pub fn vid_vfp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Dsi2CfgVidVfp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Dsi2CfgVidVfp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgVidVideoMode = crate::RegValueT<Dsi2CfgVidVideoMode_SPEC>;

impl Dsi2CfgVidVideoMode {
    #[inline(always)]
    pub fn vid_vid_video_mode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        Dsi2CfgVidVideoMode_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            Dsi2CfgVidVideoMode_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Dsi2CfgVidVsyncPolarity = crate::RegValueT<Dsi2CfgVidVsyncPolarity_SPEC>;

impl Dsi2CfgVidVsyncPolarity {
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
