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
#[doc = r"CRG_TOP registers"]
unsafe impl ::core::marker::Send for super::CrgTop {}
unsafe impl ::core::marker::Sync for super::CrgTop {}
impl super::CrgTop {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ana_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AnaStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AnaStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bandgap_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BandgapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BandgapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bias_vref_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BiasVrefSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BiasVrefSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_amba_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkAmbaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkAmbaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_cmac_switch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkCmacSwitchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkCmacSwitchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_radio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRadioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRadioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_rchs_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRchsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRchsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_rclp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRclpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRclpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_rcx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRcxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRcxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_rtcdiv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRtcdivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRtcdivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_snc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkSncCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkSncCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_switch2xtal_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkSwitch2XtalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkSwitch2XtalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_tmr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkTmrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkTmrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_xtal32k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkXtal32KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkXtal32KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn discharge_rail_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DischargeRailReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DischargeRailReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcd_ext_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdExtCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdExtCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0PadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_reset_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0ResetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0ResetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_set_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0SetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0SetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1PadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_reset_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1ResetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1ResetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_set_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1SetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1SetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p2_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2PadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p2_reset_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2ResetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2ResetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p2_set_pad_latch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2SetPadLatchReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2SetPadLatchReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pmu_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pmu_sleep_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuSleepReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuSleepReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pmu_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn por_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn por_pin_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorPinReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorPinReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn por_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PorTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PorTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn power_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PowerCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PowerCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn power_lvl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PowerLvlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PowerLvlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ram_pwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RamPwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RamPwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rst_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RstCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RstCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn secure_boot_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SecureBootReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SecureBootReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn slp_map_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SlpMapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SlpMapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sw_v18f_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SwV18FReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SwV18FReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sys_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SysCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SysCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sys_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SysStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SysStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vbus_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vbus_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn wakeup_hibern_reg(
        &self,
    ) -> &'static crate::common::Reg<self::WakeupHibernReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::WakeupHibernReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaStatusReg_SPEC;
impl crate::sealed::RegSpec for AnaStatusReg_SPEC {
    type DataType = u32;
}

pub type AnaStatusReg = crate::RegValueT<AnaStatusReg_SPEC>;

impl AnaStatusReg {
    #[inline(always)]
    pub fn flag_ldo_v30_combined_ok(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xor_dout_wakeup_pads(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbus_available(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn flag_adc_ldo_ok(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn flag_ibias_trim(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vin_nok(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bg_ok(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn boost_dcdc_vled_ok(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_high_temp(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbat_vsys_state(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, u8, AnaStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x3,1,0,u8,u8,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_head_lim(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_curr_lim(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_lim(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_mipi_ok(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_v30_ok(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn switch_v18f_ok(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buck_dcdc_v18p_ok(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buck_dcdc_v18_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buck_dcdc_v14_ok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn buck_dcdc_v12_ok(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbus_plugin(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vsys_near_vled(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbus_above_vsys(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbus_ok(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vsys_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_ok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AnaStatusReg {
    #[inline(always)]
    fn default() -> AnaStatusReg {
        <crate::RegValueT<AnaStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BandgapReg_SPEC;
impl crate::sealed::RegSpec for BandgapReg_SPEC {
    type DataType = u32;
}

pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[inline(always)]
    pub fn en_bgr_tccomp(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bandgap_enable_clamp(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgr_trim(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sysram_lpmx(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BandgapReg {
    #[inline(always)]
    fn default() -> BandgapReg {
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(32800)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BiasVrefSelReg_SPEC;
impl crate::sealed::RegSpec for BiasVrefSelReg_SPEC {
    type DataType = u32;
}

pub type BiasVrefSelReg = crate::RegValueT<BiasVrefSelReg_SPEC>;

impl BiasVrefSelReg {
    #[inline(always)]
    pub fn bias_vref_rf2_sel(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, BiasVrefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,BiasVrefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bias_vref_rf1_sel(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, BiasVrefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,BiasVrefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BiasVrefSelReg {
    #[inline(always)]
    fn default() -> BiasVrefSelReg {
        <crate::RegValueT<BiasVrefSelReg_SPEC> as RegisterValue<_>>::new(187)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodCtrlReg_SPEC;
impl crate::sealed::RegSpec for BodCtrlReg_SPEC {
    type DataType = u32;
}

pub type BodCtrlReg = crate::RegValueT<BodCtrlReg_SPEC>;

impl BodCtrlReg {
    #[inline(always)]
    pub fn bod_vbus_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vbat_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_mipi_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vsys_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18f_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18p_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v14_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v12_rst_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vbus_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vbat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_mipi_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vsys_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18f_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18p_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v14_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v12_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_status_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BodCtrlReg {
    #[inline(always)]
    fn default() -> BodCtrlReg {
        <crate::RegValueT<BodCtrlReg_SPEC> as RegisterValue<_>>::new(523266)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodStatusReg_SPEC;
impl crate::sealed::RegSpec for BodStatusReg_SPEC {
    type DataType = u32;
}

pub type BodStatusReg = crate::RegValueT<BodStatusReg_SPEC>;

impl BodStatusReg {
    #[inline(always)]
    pub fn bod_vbus(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vbat(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vmipi(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vsys(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18f(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v18(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v14(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v12(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,BodStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BodStatusReg {
    #[inline(always)]
    fn default() -> BodStatusReg {
        <crate::RegValueT<BodStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkAmbaReg_SPEC;
impl crate::sealed::RegSpec for ClkAmbaReg_SPEC {
    type DataType = u32;
}

pub type ClkAmbaReg = crate::RegValueT<ClkAmbaReg_SPEC>;

impl ClkAmbaReg {
    #[inline(always)]
    pub fn oqspi_pullup_enable(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn oqspi_gpio_mode(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic2_div(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspic_div(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn oqspif_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn oqspif_div(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aes_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slow_pclk_div(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pclk_div(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hclk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkAmbaReg {
    #[inline(always)]
    fn default() -> ClkAmbaReg {
        <crate::RegValueT<ClkAmbaReg_SPEC> as RegisterValue<_>>::new(82)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCmacSwitchReg_SPEC;
impl crate::sealed::RegSpec for ClkCmacSwitchReg_SPEC {
    type DataType = u32;
}

pub type ClkCmacSwitchReg = crate::RegValueT<ClkCmacSwitchReg_SPEC>;

impl ClkCmacSwitchReg {
    #[inline(always)]
    pub fn cmac_running_on_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCmacSwitchReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCmacSwitchReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_running_on_divn(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkCmacSwitchReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ClkCmacSwitchReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkCmacSwitchReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkCmacSwitchReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkCmacSwitchReg {
    #[inline(always)]
    fn default() -> ClkCmacSwitchReg {
        <crate::RegValueT<ClkCmacSwitchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCtrlReg_SPEC;
impl crate::sealed::RegSpec for ClkCtrlReg_SPEC {
    type DataType = u32;
}

pub type ClkCtrlReg = crate::RegValueT<ClkCtrlReg_SPEC>;

impl ClkCtrlReg {
    #[inline(always)]
    pub fn running_at_pll(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_xtal32m(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_rchs(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_rclp(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vad_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_clk_sel(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_clk_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkCtrlReg {
    #[inline(always)]
    fn default() -> ClkCtrlReg {
        <crate::RegValueT<ClkCtrlReg_SPEC> as RegisterValue<_>>::new(8193)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRadioReg_SPEC;
impl crate::sealed::RegSpec for ClkRadioReg_SPEC {
    type DataType = u32;
}

pub type ClkRadioReg = crate::RegValueT<ClkRadioReg_SPEC>;

impl ClkRadioReg {
    #[inline(always)]
    pub fn rad_reg_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfcu_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_synch_reset(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRadioReg {
    #[inline(always)]
    fn default() -> ClkRadioReg {
        <crate::RegValueT<ClkRadioReg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRchsReg_SPEC;
impl crate::sealed::RegSpec for ClkRchsReg_SPEC {
    type DataType = u32;
}

pub type ClkRchsReg = crate::RegValueT<ClkRchsReg_SPEC>;

impl ClkRchsReg {
    #[inline(always)]
    pub fn rchs_speed(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rchs_init_range(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rchs_init_del(
        self,
    ) -> crate::common::RegisterField<12, 0xff, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xff,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rchs_init_dtcf(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rchs_init_dtc(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0xf,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rchs_bias(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRchsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rchs_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRchsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRchsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRchsReg {
    #[inline(always)]
    fn default() -> ClkRchsReg {
        <crate::RegValueT<ClkRchsReg_SPEC> as RegisterValue<_>>::new(1574062)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRclpReg_SPEC;
impl crate::sealed::RegSpec for ClkRclpReg_SPEC {
    type DataType = u32;
}

pub type ClkRclpReg = crate::RegValueT<ClkRclpReg_SPEC>;

impl ClkRclpReg {
    #[inline(always)]
    pub fn rclp_low_speed_force(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkRclpReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkRclpReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rclp_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, ClkRclpReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,ClkRclpReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rclp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRclpReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRclpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRclpReg {
    #[inline(always)]
    fn default() -> ClkRclpReg {
        <crate::RegValueT<ClkRclpReg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRcxReg_SPEC;
impl crate::sealed::RegSpec for ClkRcxReg_SPEC {
    type DataType = u32;
}

pub type ClkRcxReg = crate::RegValueT<ClkRcxReg_SPEC>;

impl ClkRcxReg {
    #[inline(always)]
    pub fn rcx_bias(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_c0(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_cadjust(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, ClkRcxReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkRcxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkRcxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRcxReg {
    #[inline(always)]
    fn default() -> ClkRcxReg {
        <crate::RegValueT<ClkRcxReg_SPEC> as RegisterValue<_>>::new(2812)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRtcdivReg_SPEC;
impl crate::sealed::RegSpec for ClkRtcdivReg_SPEC {
    type DataType = u32;
}

pub type ClkRtcdivReg = crate::RegValueT<ClkRtcdivReg_SPEC>;

impl ClkRtcdivReg {
    #[inline(always)]
    pub fn rtc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rtc_div_enable(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rtc_div_denom(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rtc_div_int(
        self,
    ) -> crate::common::RegisterField<10, 0x1ff, 1, 0, u16, u16, ClkRtcdivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1ff,
            1,
            0,
            u16,
            u16,
            ClkRtcdivReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtc_div_frac(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, ClkRtcdivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRtcdivReg {
    #[inline(always)]
    fn default() -> ClkRtcdivReg {
        <crate::RegValueT<ClkRtcdivReg_SPEC> as RegisterValue<_>>::new(335528)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSncCtrlReg_SPEC;
impl crate::sealed::RegSpec for ClkSncCtrlReg_SPEC {
    type DataType = u32;
}

pub type ClkSncCtrlReg = crate::RegValueT<ClkSncCtrlReg_SPEC>;

impl ClkSncCtrlReg {
    #[inline(always)]
    pub fn snc_state_retained(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkSncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkSncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkSncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkSncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkSncCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkSncCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkSncCtrlReg {
    #[inline(always)]
    fn default() -> ClkSncCtrlReg {
        <crate::RegValueT<ClkSncCtrlReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSwitch2XtalReg_SPEC;
impl crate::sealed::RegSpec for ClkSwitch2XtalReg_SPEC {
    type DataType = u32;
}

pub type ClkSwitch2XtalReg = crate::RegValueT<ClkSwitch2XtalReg_SPEC>;

impl ClkSwitch2XtalReg {
    #[inline(always)]
    pub fn switch2xtal(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkSwitch2XtalReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,ClkSwitch2XtalReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkSwitch2XtalReg {
    #[inline(always)]
    fn default() -> ClkSwitch2XtalReg {
        <crate::RegValueT<ClkSwitch2XtalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTmrReg_SPEC;
impl crate::sealed::RegSpec for ClkTmrReg_SPEC {
    type DataType = u32;
}

pub type ClkTmrReg = crate::RegValueT<ClkTmrReg_SPEC>;

impl ClkTmrReg {
    #[inline(always)]
    pub fn tmr2_pwm_aon_mode(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr_pwm_aon_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeupct_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkTmrReg {
    #[inline(always)]
    fn default() -> ClkTmrReg {
        <crate::RegValueT<ClkTmrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkXtal32KReg_SPEC;
impl crate::sealed::RegSpec for ClkXtal32KReg_SPEC {
    type DataType = u32;
}

pub type ClkXtal32KReg = crate::RegValueT<ClkXtal32KReg_SPEC>;

impl ClkXtal32KReg {
    #[inline(always)]
    pub fn xtal32k_disable_output(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_disable_ampreg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_cur(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, ClkXtal32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_rbias(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, ClkXtal32KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkXtal32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkXtal32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkXtal32KReg {
    #[inline(always)]
    fn default() -> ClkXtal32KReg {
        <crate::RegValueT<ClkXtal32KReg_SPEC> as RegisterValue<_>>::new(46)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DischargeRailReg_SPEC;
impl crate::sealed::RegSpec for DischargeRailReg_SPEC {
    type DataType = u32;
}

pub type DischargeRailReg = crate::RegValueT<DischargeRailReg_SPEC>;

impl DischargeRailReg {
    #[inline(always)]
    pub fn reset_v30(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_vpod(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_v18p(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_v18(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_v14(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DischargeRailReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DischargeRailReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DischargeRailReg {
    #[inline(always)]
    fn default() -> DischargeRailReg {
        <crate::RegValueT<DischargeRailReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdExtCtrlReg_SPEC;
impl crate::sealed::RegSpec for LcdExtCtrlReg_SPEC {
    type DataType = u32;
}

pub type LcdExtCtrlReg = crate::RegValueT<LcdExtCtrlReg_SPEC>;

impl LcdExtCtrlReg {
    #[inline(always)]
    pub fn lcd_ext_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdExtCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,LcdExtCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcd_ext_cnt_reload(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, LcdExtCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            LcdExtCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdExtCtrlReg {
    #[inline(always)]
    fn default() -> LcdExtCtrlReg {
        <crate::RegValueT<LcdExtCtrlReg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PadLatchReg_SPEC;
impl crate::sealed::RegSpec for P0PadLatchReg_SPEC {
    type DataType = u32;
}

pub type P0PadLatchReg = crate::RegValueT<P0PadLatchReg_SPEC>;

impl P0PadLatchReg {
    #[inline(always)]
    pub fn p0_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0PadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0PadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0PadLatchReg {
    #[inline(always)]
    fn default() -> P0PadLatchReg {
        <crate::RegValueT<P0PadLatchReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0ResetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P0ResetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P0ResetPadLatchReg = crate::RegValueT<P0ResetPadLatchReg_SPEC>;

impl P0ResetPadLatchReg {
    #[inline(always)]
    pub fn p0_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0ResetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0ResetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0ResetPadLatchReg {
    #[inline(always)]
    fn default() -> P0ResetPadLatchReg {
        <crate::RegValueT<P0ResetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0SetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P0SetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P0SetPadLatchReg = crate::RegValueT<P0SetPadLatchReg_SPEC>;

impl P0SetPadLatchReg {
    #[inline(always)]
    pub fn p0_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0SetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0SetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0SetPadLatchReg {
    #[inline(always)]
    fn default() -> P0SetPadLatchReg {
        <crate::RegValueT<P0SetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PadLatchReg_SPEC;
impl crate::sealed::RegSpec for P1PadLatchReg_SPEC {
    type DataType = u32;
}

pub type P1PadLatchReg = crate::RegValueT<P1PadLatchReg_SPEC>;

impl P1PadLatchReg {
    #[inline(always)]
    pub fn p1_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P1PadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P1PadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1PadLatchReg {
    #[inline(always)]
    fn default() -> P1PadLatchReg {
        <crate::RegValueT<P1PadLatchReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1ResetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P1ResetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P1ResetPadLatchReg = crate::RegValueT<P1ResetPadLatchReg_SPEC>;

impl P1ResetPadLatchReg {
    #[inline(always)]
    pub fn p1_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P1ResetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P1ResetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1ResetPadLatchReg {
    #[inline(always)]
    fn default() -> P1ResetPadLatchReg {
        <crate::RegValueT<P1ResetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1SetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P1SetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P1SetPadLatchReg = crate::RegValueT<P1SetPadLatchReg_SPEC>;

impl P1SetPadLatchReg {
    #[inline(always)]
    pub fn p1_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P1SetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P1SetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1SetPadLatchReg {
    #[inline(always)]
    fn default() -> P1SetPadLatchReg {
        <crate::RegValueT<P1SetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PadLatchReg_SPEC;
impl crate::sealed::RegSpec for P2PadLatchReg_SPEC {
    type DataType = u32;
}

pub type P2PadLatchReg = crate::RegValueT<P2PadLatchReg_SPEC>;

impl P2PadLatchReg {
    #[inline(always)]
    pub fn p2_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        P2PadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            P2PadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2PadLatchReg {
    #[inline(always)]
    fn default() -> P2PadLatchReg {
        <crate::RegValueT<P2PadLatchReg_SPEC> as RegisterValue<_>>::new(32767)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2ResetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P2ResetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P2ResetPadLatchReg = crate::RegValueT<P2ResetPadLatchReg_SPEC>;

impl P2ResetPadLatchReg {
    #[inline(always)]
    pub fn p2_reset_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        P2ResetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            P2ResetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2ResetPadLatchReg {
    #[inline(always)]
    fn default() -> P2ResetPadLatchReg {
        <crate::RegValueT<P2ResetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2SetPadLatchReg_SPEC;
impl crate::sealed::RegSpec for P2SetPadLatchReg_SPEC {
    type DataType = u32;
}

pub type P2SetPadLatchReg = crate::RegValueT<P2SetPadLatchReg_SPEC>;

impl P2SetPadLatchReg {
    #[inline(always)]
    pub fn p2_set_latch_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fff,
        1,
        0,
        u16,
        u16,
        P2SetPadLatchReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fff,
            1,
            0,
            u16,
            u16,
            P2SetPadLatchReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2SetPadLatchReg {
    #[inline(always)]
    fn default() -> P2SetPadLatchReg {
        <crate::RegValueT<P2SetPadLatchReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuCtrlReg_SPEC;
impl crate::sealed::RegSpec for PmuCtrlReg_SPEC {
    type DataType = u32;
}

pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[inline(always)]
    pub fn retain_rgp_ram(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn retain_gpu_clut(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn retain_dcache(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpu_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctrl_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn enable_clkless(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn retain_cache(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_on_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aud_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuCtrlReg {
    #[inline(always)]
    fn default() -> PmuCtrlReg {
        <crate::RegValueT<PmuCtrlReg_SPEC> as RegisterValue<_>>::new(1551)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuSleepReg_SPEC;
impl crate::sealed::RegSpec for PmuSleepReg_SPEC {
    type DataType = u32;
}

pub type PmuSleepReg = crate::RegValueT<PmuSleepReg_SPEC>;

impl PmuSleepReg {
    #[inline(always)]
    pub fn ultra_fast_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn enable_fast_switch(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, PmuSleepReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vled_bypass_refresh_time(
        self,
    ) -> crate::common::RegisterField<23, 0x7f, 1, 0, u8, u8, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x7f,1,0,u8,u8,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_sleep_interval(
        self,
    ) -> crate::common::RegisterField<19, 0xf, 1, 0, u8, u8, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0xf,1,0,u8,u8,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rails_refresh_interval(
        self,
    ) -> crate::common::RegisterField<5, 0x3fff, 1, 0, u16, u16, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3fff,1,0,u16,u16,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn base_refresh_interval(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, PmuSleepReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,PmuSleepReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuSleepReg {
    #[inline(always)]
    fn default() -> PmuSleepReg {
        <crate::RegValueT<PmuSleepReg_SPEC> as RegisterValue<_>>::new(8920584)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuTrimReg_SPEC;
impl crate::sealed::RegSpec for PmuTrimReg_SPEC {
    type DataType = u32;
}

pub type PmuTrimReg = crate::RegValueT<PmuTrimReg_SPEC>;

impl PmuTrimReg {
    #[inline(always)]
    pub fn ibias_10n_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_ret_v30_trim(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_v30_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, PmuTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,PmuTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuTrimReg {
    #[inline(always)]
    fn default() -> PmuTrimReg {
        <crate::RegValueT<PmuTrimReg_SPEC> as RegisterValue<_>>::new(1152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorCtrlReg_SPEC;
impl crate::sealed::RegSpec for PorCtrlReg_SPEC {
    type DataType = u32;
}

pub type PorCtrlReg = crate::RegValueT<PorCtrlReg_SPEC>;

impl PorCtrlReg {
    #[inline(always)]
    pub fn por_vsys_sleep_cycle_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vsys_hyst_sel(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vsys_hyst_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vsys_force_on(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vsys_mask(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_vsys_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_sleep_cycle_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_hyst_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_hyst_disable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_force_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_v30_disable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PorCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PorCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorCtrlReg {
    #[inline(always)]
    fn default() -> PorCtrlReg {
        <crate::RegValueT<PorCtrlReg_SPEC> as RegisterValue<_>>::new(2080)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorPinReg_SPEC;
impl crate::sealed::RegSpec for PorPinReg_SPEC {
    type DataType = u32;
}

pub type PorPinReg = crate::RegValueT<PorPinReg_SPEC>;

impl PorPinReg {
    #[inline(always)]
    pub fn por_pin_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PorPinReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn por_pin_select(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, PorPinReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,PorPinReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorPinReg {
    #[inline(always)]
    fn default() -> PorPinReg {
        <crate::RegValueT<PorPinReg_SPEC> as RegisterValue<_>>::new(127)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorTimerReg_SPEC;
impl crate::sealed::RegSpec for PorTimerReg_SPEC {
    type DataType = u32;
}

pub type PorTimerReg = crate::RegValueT<PorTimerReg_SPEC>;

impl PorTimerReg {
    #[inline(always)]
    pub fn por_time(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, PorTimerReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,PorTimerReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PorTimerReg {
    #[inline(always)]
    fn default() -> PorTimerReg {
        <crate::RegValueT<PorTimerReg_SPEC> as RegisterValue<_>>::new(24)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowerCtrlReg_SPEC;
impl crate::sealed::RegSpec for PowerCtrlReg_SPEC {
    type DataType = u32;
}

pub type PowerCtrlReg = crate::RegValueT<PowerCtrlReg_SPEC>;

impl PowerCtrlReg {
    #[inline(always)]
    pub fn sw_v18f_sleep_on(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sw_v18f_on(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vled_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_vled_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18p_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v18_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v14_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v12_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcdc_v12_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clamp_v12_dis(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clamp_v30_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_mipi_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_ret_v30_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_ret_v30_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_v30_sleep_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_v30_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_start_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PowerCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PowerCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PowerCtrlReg {
    #[inline(always)]
    fn default() -> PowerCtrlReg {
        <crate::RegValueT<PowerCtrlReg_SPEC> as RegisterValue<_>>::new(836499)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowerLvlReg_SPEC;
impl crate::sealed::RegSpec for PowerLvlReg_SPEC {
    type DataType = u32;
}

pub type PowerLvlReg = crate::RegValueT<PowerLvlReg_SPEC>;

impl PowerLvlReg {
    #[inline(always)]
    pub fn vsys_level(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn v18_level(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, PowerLvlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn v14_level(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn v12_sleep_level(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn v12_level(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn v30_sleep_level(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn v30_level(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vmipi_level(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clamp_v12_level(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, PowerLvlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,PowerLvlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PowerLvlReg {
    #[inline(always)]
    fn default() -> PowerLvlReg {
        <crate::RegValueT<PowerLvlReg_SPEC> as RegisterValue<_>>::new(38960)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamPwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for RamPwrCtrlReg_SPEC {
    type DataType = u32;
}

pub type RamPwrCtrlReg = crate::RegValueT<RamPwrCtrlReg_SPEC>;

impl RamPwrCtrlReg {
    #[inline(always)]
    pub fn ram13_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram12_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram11_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram10_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram9_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram8_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram7_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram6_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram5_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram4_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram3_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram2_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram1_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram0_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, RamPwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,RamPwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RamPwrCtrlReg {
    #[inline(always)]
    fn default() -> RamPwrCtrlReg {
        <crate::RegValueT<RamPwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetStatReg_SPEC;
impl crate::sealed::RegSpec for ResetStatReg_SPEC {
    type DataType = u32;
}

pub type ResetStatReg = crate::RegValueT<ResetStatReg_SPEC>;

impl ResetStatReg {
    #[inline(always)]
    pub fn snc_wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmac_wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swd_hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wdogreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn swreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hwreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn poreset_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetStatReg {
    #[inline(always)]
    fn default() -> ResetStatReg {
        <crate::RegValueT<ResetStatReg_SPEC> as RegisterValue<_>>::new(127)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstCtrlReg_SPEC;
impl crate::sealed::RegSpec for RstCtrlReg_SPEC {
    type DataType = u32;
}

pub type RstCtrlReg = crate::RegValueT<RstCtrlReg_SPEC>;

impl RstCtrlReg {
    #[inline(always)]
    pub fn sys_cache_flush_with_sw_reset(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, RstCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,RstCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for RstCtrlReg {
    #[inline(always)]
    fn default() -> RstCtrlReg {
        <crate::RegValueT<RstCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecureBootReg_SPEC;
impl crate::sealed::RegSpec for SecureBootReg_SPEC {
    type DataType = u32;
}

pub type SecureBootReg = crate::RegValueT<SecureBootReg_SPEC>;

impl SecureBootReg {
    #[inline(always)]
    pub fn prot_otp_cs_write(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_snc_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_oqspif_key_read(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_oqspif_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_aes_key_read(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_aes_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prot_sig_key_write(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_cmac_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_debugger_off(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn secure_boot(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SecureBootReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SecureBootReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SecureBootReg {
    #[inline(always)]
    fn default() -> SecureBootReg {
        <crate::RegValueT<SecureBootReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlpMapReg_SPEC;
impl crate::sealed::RegSpec for SlpMapReg_SPEC {
    type DataType = u32;
}

pub type SlpMapReg = crate::RegValueT<SlpMapReg_SPEC>;

impl SlpMapReg {
    #[inline(always)]
    pub fn lcd_inv_ext_clk_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcd_ext_clk_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bandgap_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rclp_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr4_pwm_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr3_pwm_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr_pwm_slp_map(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SlpMapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SlpMapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SlpMapReg {
    #[inline(always)]
    fn default() -> SlpMapReg {
        <crate::RegValueT<SlpMapReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwV18FReg_SPEC;
impl crate::sealed::RegSpec for SwV18FReg_SPEC {
    type DataType = u32;
}

pub type SwV18FReg = crate::RegValueT<SwV18FReg_SPEC>;

impl SwV18FReg {
    #[inline(always)]
    pub fn delay_trim(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, SwV18FReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,SwV18FReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn skip_soft_start(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SwV18FReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SwV18FReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_sw_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SwV18FReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SwV18FReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SwV18FReg {
    #[inline(always)]
    fn default() -> SwV18FReg {
        <crate::RegValueT<SwV18FReg_SPEC> as RegisterValue<_>>::new(4)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrlReg_SPEC;
impl crate::sealed::RegSpec for SysCtrlReg_SPEC {
    type DataType = u32;
}

pub type SysCtrlReg = crate::RegValueT<SysCtrlReg_SPEC>;

impl SysCtrlReg {
    #[inline(always)]
    pub fn sw_reset(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SysCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,SysCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cacheram_mux(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timeout_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn debugger_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_debugger_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn remap_intvect(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn remap_adr0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SysCtrlReg {
    #[inline(always)]
    fn default() -> SysCtrlReg {
        <crate::RegValueT<SysCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysStatReg_SPEC;
impl crate::sealed::RegSpec for SysStatReg_SPEC {
    type DataType = u32;
}

pub type SysStatReg = crate::RegValueT<SysStatReg_SPEC>;

impl SysStatReg {
    #[inline(always)]
    pub fn gpu_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpu_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctrl_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ctrl_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn power_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbg_is_active(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn snc_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tim_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mem_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mem_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aud_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aud_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rad_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rad_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SysStatReg {
    #[inline(always)]
    fn default() -> SysStatReg {
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(91557)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqClearReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqClearReg_SPEC {
    type DataType = u32;
}

pub type VbusIrqClearReg = crate::RegValueT<VbusIrqClearReg_SPEC>;

impl VbusIrqClearReg {
    #[inline(always)]
    pub fn vbus_irq_clear(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        VbusIrqClearReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            VbusIrqClearReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for VbusIrqClearReg {
    #[inline(always)]
    fn default() -> VbusIrqClearReg {
        <crate::RegValueT<VbusIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqMaskReg_SPEC {
    type DataType = u32;
}

pub type VbusIrqMaskReg = crate::RegValueT<VbusIrqMaskReg_SPEC>;

impl VbusIrqMaskReg {
    #[inline(always)]
    pub fn vbus_wakeup_clkless(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbus_irq_en_rise(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbus_irq_en_fall(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VbusIrqMaskReg {
    #[inline(always)]
    fn default() -> VbusIrqMaskReg {
        <crate::RegValueT<VbusIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeupHibernReg_SPEC;
impl crate::sealed::RegSpec for WakeupHibernReg_SPEC {
    type DataType = u32;
}

pub type WakeupHibernReg = crate::RegValueT<WakeupHibernReg_SPEC>;

impl WakeupHibernReg {
    #[inline(always)]
    pub fn hibernation_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, WakeupHibernReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,WakeupHibernReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeup_pd_en(
        self,
    ) -> crate::common::RegisterField<6, 0xf, 1, 0, u8, u8, WakeupHibernReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0xf,1,0,u8,u8,WakeupHibernReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeup_en(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, WakeupHibernReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,WakeupHibernReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for WakeupHibernReg {
    #[inline(always)]
    fn default() -> WakeupHibernReg {
        <crate::RegValueT<WakeupHibernReg_SPEC> as RegisterValue<_>>::new(4095)
    }
}
