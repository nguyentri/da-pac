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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:10 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"GPREG registers"]
unsafe impl ::core::marker::Send for super::Gpreg {}
unsafe impl ::core::marker::Sync for super::Gpreg {}
impl super::Gpreg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "BLE FINECNT sampled value while in deep sleep state."]
    #[inline(always)]
    pub const fn ble_finecnt_samp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BleFinecntSampReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BleFinecntSampReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Various debug information register."]
    #[inline(always)]
    pub const fn debug_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DebugReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DebugReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Base address of the ECC Crypto memory register."]
    #[inline(always)]
    pub const fn ecc_base_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccBaseAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccBaseAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "General purpose system control register."]
    #[inline(always)]
    pub const fn gp_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "General purpose system status register."]
    #[inline(always)]
    pub const fn gp_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Controls muxing and enabling of the LEDs."]
    #[inline(always)]
    pub const fn led_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LedControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LedControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "System PLL control register 1."]
    #[inline(always)]
    pub const fn pll_sys_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "System PLL control register 2."]
    #[inline(always)]
    pub const fn pll_sys_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "System PLL control register 3."]
    #[inline(always)]
    pub const fn pll_sys_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "System PLL status register."]
    #[inline(always)]
    pub const fn pll_sys_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "System PLL test register."]
    #[inline(always)]
    pub const fn pll_sys_test_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PllSysTestReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PllSysTestReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Controls unfreezing of various timers/counters (incl. DMA and USB)."]
    #[inline(always)]
    pub const fn reset_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Controls freezing of various timers/counters (incl. DMA and USB)."]
    #[inline(always)]
    pub const fn set_freeze_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetFreezeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetFreezeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BleFinecntSampReg_SPEC;
impl crate::sealed::RegSpec for BleFinecntSampReg_SPEC {
    type DataType = u16;
}

#[doc = "BLE FINECNT sampled value while in deep sleep state."]
pub type BleFinecntSampReg = crate::RegValueT<BleFinecntSampReg_SPEC>;

impl BleFinecntSampReg {
    #[doc = "This register is located at the Always On Power Domain and it holds the automatically sampled value of the BLE FINECNT timer\nThe HW automatically samples the value into this register during the sequence of \"BLE Sleep On\" and restores automatically the value during the BLE Wake up sequence.\nThe Software may read and modify the value while the BLE is in Sleep state. While the BLE is awake, the value of the register has no meaning, while changing the value by writing another one will have no effect in the operation of the BLE core."]
    #[inline(always)]
    pub fn ble_finecnt_samp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        BleFinecntSampReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            BleFinecntSampReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for BleFinecntSampReg {
    #[inline(always)]
    fn default() -> BleFinecntSampReg {
        <crate::RegValueT<BleFinecntSampReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugReg_SPEC;
impl crate::sealed::RegSpec for DebugReg_SPEC {
    type DataType = u16;
}

#[doc = "Various debug information register."]
pub type DebugReg = crate::RegValueT<DebugReg_SPEC>;

impl DebugReg {
    #[doc = "Default \'1\', freezing of the on-chip timers is enabled when the Cortex-M0 is halted in DEBUG State.\nIf \'0\', freezing of the on-chip timers is depending on FREEZE_REG when the Cortex-M0 is halted in DEBUG State except the watchdog timer. The watchdog timer is always frozen when the Cortex-M0 is halted in DEBUG State.\nNote: This bit is retained."]
    #[inline(always)]
    pub fn debugs_freeze_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DebugReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, DebugReg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for DebugReg {
    #[inline(always)]
    fn default() -> DebugReg {
        <crate::RegValueT<DebugReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccBaseAddrReg_SPEC;
impl crate::sealed::RegSpec for EccBaseAddrReg_SPEC {
    type DataType = u16;
}

#[doc = "Base address of the ECC Crypto memory register."]
pub type EccBaseAddrReg = crate::RegValueT<EccBaseAddrReg_SPEC>;

impl EccBaseAddrReg {
    #[doc = "Contains the base address of the ECC Crypto memory.\nMemory allocation is in pages of 1KB and up to 127KB.\nSince the ECC has an address range of 2KB and the total addressable memory range is 128KB, the maximum value of 0x7F (127KB offset) will result in 1KB at the top of the memory range and the other 1KB at the bottom of the memory range."]
    #[inline(always)]
    pub fn ecc_base_addr(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, EccBaseAddrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,EccBaseAddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccBaseAddrReg {
    #[inline(always)]
    fn default() -> EccBaseAddrReg {
        <crate::RegValueT<EccBaseAddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpControlReg_SPEC;
impl crate::sealed::RegSpec for GpControlReg_SPEC {
    type DataType = u16;
}

#[doc = "General purpose system control register."]
pub type GpControlReg = crate::RegValueT<GpControlReg_SPEC>;

impl GpControlReg {
    #[doc = "The current value of the BLE_WAKEUP_LP_IRQ interrupt request."]
    #[inline(always)]
    pub fn ble_wakeup_lp_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, GpControlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,GpControlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "If \'1\', the AHB-to-AHB bridge is bypassed, needed to access the BLE Register file, only when the system clock source is the XTAL and both hclk and ble_hclk are running at 16MHz, i.e. at the XTAL clock rate."]
    #[inline(always)]
    pub fn ble_h2h_bridge_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the BLE wakes up. Must be kept high at least for 1 low power clock period. \nIf the BLE is in deep sleep state, then by setting this bit it will cause the wakeup LP IRQ to be asserted with a delay of 3 to 4 low power cycles."]
    #[inline(always)]
    pub fn ble_wakeup_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpControlReg {
    #[inline(always)]
    fn default() -> GpControlReg {
        <crate::RegValueT<GpControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpStatusReg_SPEC;
impl crate::sealed::RegSpec for GpStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "General purpose system status register."]
pub type GpStatusReg = crate::RegValueT<GpStatusReg_SPEC>;

impl GpStatusReg {
    #[doc = "If \'1\', it designates that the chip is in Calibration Phase i.e. the OTP has been initially programmed but no Calibration has occured."]
    #[inline(always)]
    pub fn cal_phase(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpStatusReg {
    #[inline(always)]
    fn default() -> GpStatusReg {
        <crate::RegValueT<GpStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LedControlReg_SPEC;
impl crate::sealed::RegSpec for LedControlReg_SPEC {
    type DataType = u16;
}

#[doc = "Controls muxing and enabling of the LEDs."]
pub type LedControlReg = crate::RegValueT<LedControlReg_SPEC>;

impl LedControlReg {
    #[doc = "LED current trimming bits."]
    #[inline(always)]
    pub fn led_trim(
        self,
    ) -> crate::common::RegisterField<6, 0xf, 1, 0, u8, u8, LedControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0xf,1,0,u8,u8,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: LED3 disabled,\n1: LED3 enabled."]
    #[inline(always)]
    pub fn led3_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: LED2 disabled,\n1: LED2 enabled."]
    #[inline(always)]
    pub fn led2_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: LED1 disabled,\n1: LED1 enabled."]
    #[inline(always)]
    pub fn led1_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: LED3 = PWM4,\n1: LED3 = Breathing Timer."]
    #[inline(always)]
    pub fn led3_src_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: LED2 = PWM3,\n1: LED2 = Breathing Timer."]
    #[inline(always)]
    pub fn led2_src_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: LED1 = PWM2,\n1: LED1 = Breathing Timer.\nNote: The PWM2/3/4 can also be routed to GPIOs using PID 25/26/27 respectively."]
    #[inline(always)]
    pub fn led1_src_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LedControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LedControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LedControlReg {
    #[inline(always)]
    fn default() -> LedControlReg {
        <crate::RegValueT<LedControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl1Reg_SPEC {
    type DataType = u16;
}

#[doc = "System PLL control register 1."]
pub type PllSysCtrl1Reg = crate::RegValueT<PllSysCtrl1Reg_SPEC>;

impl PllSysCtrl1Reg {
    #[doc = "PLL Output dvider R (x means divide by x, 0 means divide by 1)"]
    #[inline(always)]
    pub fn pll_r_div(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, PllSysCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: indicates that the reference input is tracked,\n1: indicates that the reference input is sampled."]
    #[inline(always)]
    pub fn ldo_pll_vref_hold(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: LDO PLL off,\n1: LDO PLL on."]
    #[inline(always)]
    pub fn ldo_pll_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: Power down\n1: PLL on"]
    #[inline(always)]
    pub fn pll_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllSysCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PllSysCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl1Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl1Reg {
        <crate::RegValueT<PllSysCtrl1Reg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl2Reg_SPEC {
    type DataType = u16;
}

#[doc = "System PLL control register 2."]
pub type PllSysCtrl2Reg = crate::RegValueT<PllSysCtrl2Reg_SPEC>;

impl PllSysCtrl2Reg {
    #[doc = "0: VCO current read from min_current <5:0>,\n1: VCO current is internally determined with a calibration algoritm."]
    #[inline(always)]
    pub fn pll_sel_min_cur_int(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PllSysCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PllSysCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PLL manual delay value for Phase Frequency Detector.\n0: 0.493\n1: 0.814\n2: 1.13 ns <- default\n3: 1.44 ns"]
    #[inline(always)]
    pub fn pll_del_sel(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, PllSysCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,PllSysCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PLL Loop divider N (x means divide by x, 0 means divide by 1)"]
    #[inline(always)]
    pub fn pll_n_div(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, PllSysCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,PllSysCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl2Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl2Reg {
        <crate::RegValueT<PllSysCtrl2Reg_SPEC> as RegisterValue<_>>::new(8198)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for PllSysCtrl3Reg_SPEC {
    type DataType = u16;
}

#[doc = "System PLL control register 3."]
pub type PllSysCtrl3Reg = crate::RegValueT<PllSysCtrl3Reg_SPEC>;

impl PllSysCtrl3Reg {
    #[doc = "Recalibrate"]
    #[inline(always)]
    pub fn pll_recalib(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PllSysCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PllSysCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Programmable delay time for the loop filter voltage preset value. After PLL_EN is set, the loopfilter precharge resistors are disabled after this delay time. One LSB is 48 ns"]
    #[inline(always)]
    pub fn pll_start_del(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, PllSysCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,PllSysCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PLL charge pump current select\nOne LSB is 5uA."]
    #[inline(always)]
    pub fn pll_icp_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, PllSysCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,PllSysCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysCtrl3Reg {
    #[inline(always)]
    fn default() -> PllSysCtrl3Reg {
        <crate::RegValueT<PllSysCtrl3Reg_SPEC> as RegisterValue<_>>::new(15369)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysStatusReg_SPEC;
impl crate::sealed::RegSpec for PllSysStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "System PLL status register."]
pub type PllSysStatusReg = crate::RegValueT<PllSysStatusReg_SPEC>;

impl PllSysStatusReg {
    #[doc = "Indicates that calibration has finished."]
    #[inline(always)]
    pub fn pll_calibr_end(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Calibrated VCO frequency band."]
    #[inline(always)]
    pub fn pll_pll_best_min_cur(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, u8, PllSysStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3f,1,0,u8,u8,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: Indicates that LDO PLL is in regulation."]
    #[inline(always)]
    pub fn ldo_pll_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "1: PLL locked"]
    #[inline(always)]
    pub fn pll_lock_fine(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllSysStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,PllSysStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysStatusReg {
    #[inline(always)]
    fn default() -> PllSysStatusReg {
        <crate::RegValueT<PllSysStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSysTestReg_SPEC;
impl crate::sealed::RegSpec for PllSysTestReg_SPEC {
    type DataType = u16;
}

#[doc = "System PLL test register."]
pub type PllSysTestReg = crate::RegValueT<PllSysTestReg_SPEC>;

impl PllSysTestReg {
    #[doc = "Lock measurement time in <tbd> clock cycle of xx usec. After this period PLL_LOCK_FINE is calculated based on the difference of the M and N counted pulses in that period. If PLL_LOCK_FINE is still 0, the lock state machine restarts until PLL_LOCK_FINE gets 1\n0: <tbd> usec\n7: <tbd> usec"]
    #[inline(always)]
    pub fn pll_lock_det_res_cnt(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, PllSysTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select test mode for output divider R\nMaps PLL_R_DIV input on pins <tbd> and divider output on pin <tbd>"]
    #[inline(always)]
    pub fn pll_sel_r_div_test(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select test mode for loop divider N.\nMaps PLL_N_DIV input on pins <tbd> and divider output on pin <tbd>"]
    #[inline(always)]
    pub fn pll_sel_n_div_test(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0: normal value\n1: reverse charge pump up/down signals"]
    #[inline(always)]
    pub fn pll_change(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: set to open loop to termine max frequency"]
    #[inline(always)]
    pub fn pll_open_loop(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: map loopfilter voltage on external pin <tbd>"]
    #[inline(always)]
    pub fn pll_test_vctr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VCO current trimming."]
    #[inline(always)]
    pub fn pll_min_current(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, u8, PllSysTestReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3f,1,0,u8,u8,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "1: disable PLL internal loop filter"]
    #[inline(always)]
    pub fn pll_dis_loopfilt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PllSysTestReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PllSysTestReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PllSysTestReg {
    #[inline(always)]
    fn default() -> PllSysTestReg {
        <crate::RegValueT<PllSysTestReg_SPEC> as RegisterValue<_>>::new(112)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetFreezeReg_SPEC;
impl crate::sealed::RegSpec for ResetFreezeReg_SPEC {
    type DataType = u16;
}

#[doc = "Controls unfreezing of various timers/counters (incl. DMA and USB)."]
pub type ResetFreezeReg = crate::RegValueT<ResetFreezeReg_SPEC>;

impl ResetFreezeReg {
    #[doc = "If \'1\', the SW Timer (TIMER2) continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim2(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the SW Timer (TIMER1) continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim1(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the DMA continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the USB continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_usb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the watchdog timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the BLE master clock continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the SW Timer (TIMER0) continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim0(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the Wake Up Timer continues, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetFreezeReg {
    #[inline(always)]
    fn default() -> ResetFreezeReg {
        <crate::RegValueT<ResetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetFreezeReg_SPEC;
impl crate::sealed::RegSpec for SetFreezeReg_SPEC {
    type DataType = u16;
}

#[doc = "Controls freezing of various timers/counters (incl. DMA and USB)."]
pub type SetFreezeReg = crate::RegValueT<SetFreezeReg_SPEC>;

impl SetFreezeReg {
    #[doc = "If \'1\', the SW Timer (TIMER2) is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim2(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the SW Timer (TIMER1) is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim1(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the DMA is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the USB is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_usb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the watchdog timer is frozen, \'0\' is discarded. WATCHDOG_CTRL_REG\\[NMI_RST\\] must be \'0\' to allow the freeze function."]
    #[inline(always)]
    pub fn frz_wdog(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the BLE master clock is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_bletim(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the SW Timer (TIMER0) is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_swtim0(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "If \'1\', the Wake Up Timer is frozen, \'0\' is discarded."]
    #[inline(always)]
    pub fn frz_wkuptim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetFreezeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetFreezeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetFreezeReg {
    #[inline(always)]
    fn default() -> SetFreezeReg {
        <crate::RegValueT<SetFreezeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
